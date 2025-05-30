//! Provides the [`Pitch`] type for working with musical pitch using raw frequencies (Hz).
//!
//! Includes conversions from MIDI note numbers, transposition in semitones, and symbolic name parsing.
//!
//! Useful for audio engines, synthesizers, or any application that needs to translate between symbolic notes and actual sound.
//! Compatible with `no_std` environments.
#[cfg(test)]
mod tests;

pub use crate::error::PitchyError;

use core::str::FromStr;

use crate::{Note, math::*};

/// A musical pitch represented purely by its frequency in Hertz (Hz).
///
/// This type models raw sound frequency without symbolic context
/// (e.g. note letters or accidentals). For notation-aware handling, see [`Note`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pitch {
    /// The raw frequency in Hertz (Hz).
    frequency: f64,
}

impl Pitch {
    /// Creates a new pitch from the given frequency in Hertz (Hz).
    ///
    /// # Arguments
    ///
    /// * `frequency` – The frequency in Hz representing the pitch.
    pub fn new(frequency: f64) -> Self {
        Self { frequency }
    }

    /// Creates a pitch from a MIDI note number in the range 0–127.
    ///
    /// Returns an error if the MIDI number is out of range.
    pub fn try_from_midi_number(midi: u8) -> Result<Self, PitchyError> {
        if midi > 127 {
            return Err(PitchyError::OutOfMidiRange(midi));
        }
        let frequency = powf2((midi as f64 - 69.0) / 12.0) * 440.0;

        Ok(Self { frequency })
    }

    /// Returns the frequency of this pitch in Hertz (Hz).
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Transposes this pitch by a number of semitones.
    ///
    /// Positive values raise the pitch; negative values lower it.
    /// # Examples
    ///
    /// ```
    /// use pitchy::Pitch;
    /// use std::str::FromStr;
    ///
    /// let pitch = Pitch::from_str("C4").unwrap();
    /// let transposed = pitch.transpose(2.0);
    /// assert_eq!(transposed.try_midi_number().unwrap(), 62); // D4
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
    /// Returns `Ok(midi)` if the frequency corresponds to a valid MIDI note (0–127),
    /// otherwise returns `PitchyError::OutOfMidiRange(fallback)` where the fallback
    /// is the nearest clamped `u8` approximation.
    pub fn try_midi_number(&self) -> Result<u8, PitchyError> {
        let midi = 69.0 + 12.0 * log2(self.frequency / 440.0);
        let rounded = round(midi);

        if (0.0..=127.0).contains(&rounded) {
            Ok(rounded as u8)
        } else {
            let fallback = rounded.clamp(0.0, 127.0) as u8;
            Err(PitchyError::OutOfMidiRange(fallback))
        }
    }

    /// Returns the octave number for this pitch, based on the MIDI standard.
    ///
    /// MIDI 69 (A4) maps to octave 4. MIDI 0 (C-1) maps to octave -1.
    ///
    /// Returns `None` if the frequency is outside the MIDI range.
    pub fn octave(&self) -> Option<i8> {
        self.try_midi_number().ok().map(|midi| midi as i8 / 12 - 1)
    }
}

/// Parses a pitch from a note name string (e.g., "C4", "A#3", "Db5").
///
/// Accepts sharps (`#`) or flats (`b`) and supports octaves from -1 to 9.
///
/// Returns an error if the format is invalid or the note is out of range.
impl FromStr for Pitch {
    type Err = PitchyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() < 2 || s.len() > 4 {
            return Err(PitchyError::InvalidName);
        }

        let split_index = s
            .find(|c: char| c.is_ascii_digit() || c == '-')
            .ok_or(PitchyError::InvalidOctave)?;
        let (note_part, octave_str) = s.split_at(split_index);
        let octave: i8 = octave_str.parse().map_err(|_| PitchyError::InvalidOctave)?;

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
            _ => return Err(PitchyError::InvalidName),
        };

        let midi = (octave as i16)
            .checked_add(1)
            .and_then(|v| v.checked_mul(12))
            .and_then(|v| v.checked_add(semitone as i16))
            .ok_or(PitchyError::MidiOverflow)?;

        if !(0..=127).contains(&midi) {
            return Err(PitchyError::OutOfMidiRange(midi as u8));
        }

        let hz = powf2((midi as f64 - 69.0) / 12.0) * 440.0;
        Ok(Pitch::new(hz))
    }
}

/// Converts a symbolic [`Note`] into a [`Pitch`] using MIDI-based mapping.
impl TryFrom<Note> for Pitch {
    type Error = PitchyError;

    fn try_from(note: Note) -> Result<Pitch, PitchyError> {
        let semitone = (note.letter() as i8) + (note.accidental() as i8);
        let midi = ((note.octave() + 1) * 12 + semitone) as i16;

        if !(0..=127).contains(&midi) {
            return Err(PitchyError::OutOfMidiRange(midi as u8));
        }

        Pitch::try_from_midi_number(midi as u8)
    }
}
