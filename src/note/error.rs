/// An error type representing possible failures when creating or parsing a [`crate::Note`].
///
/// This includes:
/// - Invalid or unrecognized note names (e.g., "H#4")
/// - Octaves that cannot be parsed into numbers
/// - Notes that fall outside the standard MIDI range of 0–127
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteError {
    OutOfMidiRange,
    InvalidOctave,
    InvalidName,
}

impl core::fmt::Display for NoteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            NoteError::InvalidName => "The note name is invalid or unrecognized",
            NoteError::InvalidOctave => "The octave portion could not be parsed",
            NoteError::OutOfMidiRange => "The computed MIDI note is outside the valid 0-127 range",
        };
        write!(f, "{}", msg)
    }
}
