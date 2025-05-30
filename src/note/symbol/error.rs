/// Errors that may occur when interpreting or representing symbolic note components,
/// such as accidentals or letter mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolError {
    /// The semitone offset does not correspond to a valid [`Accidental`].
    InvalidAccidental(i8),
    // More error variants may be added in the future
}

impl core::fmt::Display for SymbolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SymbolError::InvalidAccidental(i) => {
                write!(f, "invalid semitone offset for accidental: {}", i)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SymbolError {}
