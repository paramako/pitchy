//! Symbolic note representation — e.g., "C#4" or "Bb2".
//! Includes spelling logic via [`NoteLetter`] and [`Accidental`].

mod symbol;
#[cfg(test)]
mod tests;

pub use symbol::{Accidental, NoteLetter};

use crate::{Pitch, PitchyError};

/// A musical note spelled with a letter, accidental, and octave.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    letter: NoteLetter,
    accidental: Accidental,
    octave: i8,
}

impl Note {
    /// Creates a new symbolic note.
    ///
    /// This does not check whether the resulting note is within the valid MIDI range.
    pub fn new(letter: NoteLetter, accidental: Accidental, octave: i8) -> Self {
        Self {
            letter,
            accidental,
            octave,
        }
    }

    /// Returns the octave number of the note, based on the MIDI mapping.
    /// For example, MIDI 69 (A4) returns 4, and MIDI 0 (C-1) returns -1.
    pub fn octave(&self) -> i8 {
        self.octave
    }

    /// Returns the [`NoteLetter`] of the note (e.g., C, D, E, etc.).
    pub fn letter(&self) -> NoteLetter {
        self.letter
    }

    /// Returns the [`Accidental`] of the note (e.g., ♯, ♭, 𝄪, etc.).
    pub fn accidental(&self) -> Accidental {
        self.accidental
    }

    /// Returns the name of the note (e.g., "A4", "C#3") if possible.
    /// Only available when the `std` feature is enabled.
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        format!("{}{}{}", self.letter, self.accidental, self.octave)
    }
}

impl TryFrom<Pitch> for Note {
    type Error = PitchyError;

    /// Attempts to convert a [`Pitch`] into a symbolic [`Note`] using standard sharp-based spelling.
    ///
    /// The conversion prefers natural and sharp spellings by default. Flat or double accidentals
    /// are only used when required to accurately represent the pitch semitone.
    ///
    /// # Errors
    /// Returns [`PitchyError::Unspelled`] if the pitch is outside the MIDI range or
    /// cannot be represented by a valid letter and accidental.
    ///
    /// # Examples
    /// ```
    /// use pitchy::{Pitch, Note};
    /// use std::str::FromStr;
    ///
    /// let pitch = Pitch::from_str("A4").unwrap();
    /// let note = Note::try_from(pitch).unwrap();
    /// #[cfg(feature = "std")]
    /// assert_eq!(note.name(), "A4");
    /// ```
    fn try_from(pitch: Pitch) -> Result<Self, Self::Error> {
        let midi = pitch.try_midi_number()? as i8;
        let octave = midi / 12 - 1;
        let semitone = midi % 12;

        // Use sharp-biased mapping: try natural & sharp-based letters first
        for accidental in [
            Accidental::Natural,
            Accidental::Sharp,
            Accidental::Flat,
            Accidental::DoubleSharp,
            Accidental::DoubleFlat,
        ] {
            for letter in NoteLetter::all() {
                let base = letter as i8;
                if base + accidental as i8 == semitone {
                    return Ok(Note::new(letter, accidental, octave));
                }
            }
        }

        Err(PitchyError::Unspelled)
    }
}
