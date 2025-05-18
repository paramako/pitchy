mod error;
#[cfg(test)]
mod tests;

pub use error::NoteError;

use core::str::FromStr;

use crate::math::*;

const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// A musical note represented by its frequency in Hertz.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    /// Frequency of the note in Hertz.
    frequency: f64,
}

impl Note {
    /// Creates a new note from a given frequency in Hz.
    pub fn new(frequency: f64) -> Self {
        Self { frequency }
    }

    /// Creates a new note from a given MIDI number in the range 0–127.
    /// Returns an error if the MIDI value is outside the valid range.
    pub fn try_from_midi_number(midi: u8) -> Result<Self, NoteError> {
        if midi > 127 {
            return Err(NoteError::OutOfMidiRange);
        }
        let frequency = powf2((midi as f64 - 69.0) / 12.0) * 440.0;

        Ok(Self { frequency })
    }

    /// Returns the frequency of the note in Hz.
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Transposes the note by the given number of semitones.
    ///
    /// Positive values shift up, negative shift down.
    /// # Examples
    ///
    /// ```
    /// use pitchy::Note;
    /// use std::str::FromStr;
    ///
    /// let note = Note::from_str("C4").unwrap();
    /// let transposed = note.transpose(2.0);
    /// assert_eq!(transposed.midi_number(), Some(62)); // D4
    /// let expected_hz = 293.665;
    /// assert!((transposed.frequency() - expected_hz).abs() < 0.01);
    /// ```
    pub fn transpose(&self, semitones: f64) -> Self {
        Self {
            frequency: self.frequency * powf2(semitones / 12.0),
        }
    }

    /// Approximates the MIDI note number corresponding to this frequency.
    ///
    /// Returns `None` if the frequency falls outside the MIDI range (0–127).
    pub fn midi_number(&self) -> Option<u8> {
        let midi: f64 = 69.0 + 12.0 * log2(self.frequency / 440.0);
        let rounded = round(midi);
        if (0.0..=127.0).contains(&rounded) {
            Some(rounded as u8)
        } else {
            None
        }
    }

    /// Returns the octave number of the note, based on the MIDI mapping.
    /// For example, MIDI 69 (A4) returns 4, and MIDI 0 (C-1) returns -1.
    pub fn octave(&self) -> Option<i8> {
        self.midi_number().map(|midi| midi as i8 / 12 - 1)
    }

    /// Returns the note index (0 = C, 1 = C#, ..., 11 = B) and octave as a tuple.
    /// This is available in no_std contexts and provides symbolic note info.
    pub fn note_parts(&self) -> Option<(u8, i8)> {
        self.midi_number()
            .map(|midi| (midi % 12, midi as i8 / 12 - 1))
    }

    /// Returns the note letter name (e.g., "A", "C#") without octave, for no_std-friendly use.
    pub fn note_letter(&self) -> Option<&'static str> {
        self.midi_number()
            .map(|midi| NOTE_NAMES[(midi % 12) as usize])
    }

    /// Returns the name of the note (e.g., "A4", "C#3") if possible.
    /// Only available when the `std` feature is enabled.
    #[cfg(feature = "std")]
    pub fn name(&self) -> Option<String> {
        self.note_parts()
            .map(|(note, octave)| format!("{}{}", NOTE_NAMES[note as usize], octave))
    }
}

// #[cfg(feature = "std")]
impl FromStr for Note {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() < 2 || s.len() > 4 {
            return Err(NoteError::InvalidName);
        }

        let split_index = s
            .find(|c: char| c.is_ascii_digit() || c == '-')
            .ok_or(NoteError::InvalidOctave)?;
        let (note_part, octave_str) = s.split_at(split_index);
        let octave: i8 = octave_str.parse().map_err(|_| NoteError::InvalidOctave)?;

        let semitone = match note_part {
            n if n.eq_ignore_ascii_case("C") => 0,
            n if n.eq_ignore_ascii_case("C#") || n.eq_ignore_ascii_case("Db") => 1,
            n if n.eq_ignore_ascii_case("D") => 2,
            n if n.eq_ignore_ascii_case("D#") || n.eq_ignore_ascii_case("Eb") => 3,
            n if n.eq_ignore_ascii_case("E") => 4,
            n if n.eq_ignore_ascii_case("F") => 5,
            n if n.eq_ignore_ascii_case("F#") || n.eq_ignore_ascii_case("Gb") => 6,
            n if n.eq_ignore_ascii_case("G") => 7,
            n if n.eq_ignore_ascii_case("G#") || n.eq_ignore_ascii_case("Ab") => 8,
            n if n.eq_ignore_ascii_case("A") => 9,
            n if n.eq_ignore_ascii_case("A#") || n.eq_ignore_ascii_case("Bb") => 10,
            n if n.eq_ignore_ascii_case("B") => 11,
            _ => return Err(NoteError::InvalidName),
        };

        let midi = (octave as i16)
            .checked_add(1)
            .and_then(|v| v.checked_mul(12))
            .and_then(|v| v.checked_add(semitone as i16))
            .ok_or(NoteError::OutOfMidiRange)?;

        if !(0..=127).contains(&midi) {
            return Err(NoteError::OutOfMidiRange);
        }

        let hz = powf2((midi as f64 - 69.0) / 12.0) * 440.0;
        Ok(Note::new(hz))
    }
}
