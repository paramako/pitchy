/// An error type representing failures when parsing or converting notes or pitches.
///
/// This error may occur when:
/// - The note name is invalid or unrecognized (e.g., `"H#4"`)
/// - The octave part cannot be parsed as a number
/// - The resulting pitch falls outside the valid MIDI range (0–127)
/// - The MIDI number calculation overflows
/// - A valid note spelling (letter + accidental) cannot be determined
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitchyError {
    InvalidName,
    InvalidOctave,
    OutOfMidiRange(u8),
    MidiOverflow,
    Unspelled,
}

impl core::fmt::Display for PitchyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PitchyError::InvalidName => write!(f, "The note name is invalid or unrecognized"),
            PitchyError::InvalidOctave => write!(f, "The octave portion could not be parsed"),
            PitchyError::OutOfMidiRange(midi) => {
                write!(
                    f,
                    "The computed MIDI note {midi} is outside the valid 0-127 range"
                )
            }
            PitchyError::MidiOverflow => {
                write!(
                    f,
                    "The MIDI note could not be computed due to numeric overflow"
                )
            }
            PitchyError::Unspelled => {
                write!(
                    f,
                    "The pitch could not be spelled as a standard letter and accidental"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PitchyError {}
