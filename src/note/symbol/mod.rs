//! Components for symbolic musical notes: note letters and accidentals.
//!
//! These types define the spelled representation of notes (e.g., `C#`, `Bb`, `F𝄪`).

use error::SymbolError;

mod error;

/// Represents the base letter of a musical note (C, D, E, F, G, A, B).
///
/// The `repr(u8)` maps each letter to its position in the chromatic scale,
/// which allows direct semitone indexing: C=0, D=2, E=4, F=5, G=7, A=9, B=11.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NoteLetter {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl NoteLetter {
    /// All 7 possible note letters
    pub const fn all() -> [Self; 7] {
        use NoteLetter::*;

        [C, D, E, F, G, A, B]
    }

    pub fn as_str(&self) -> &'static str {
        use NoteLetter::*;

        match self {
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            A => "A",
            B => "B",
        }
    }
}

impl core::fmt::Display for NoteLetter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Represents the accidental applied to a note (double flat, flat, natural, sharp, double sharp).
///
/// The `repr(i8)` lets us treat accidentals as signed semitone offsets:
/// DoubleFlat = -2, Flat = -1, Natural = 0, Sharp = 1, DoubleSharp = 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Accidental {
    DoubleFlat = -2,
    Flat = -1,
    Natural = 0,
    Sharp = 1,
    DoubleSharp = 2,
}

impl Accidental {
    pub fn as_str(&self) -> &'static str {
        use Accidental::*;

        match self {
            Natural => "",
            Sharp => "#",
            Flat => "b",
            DoubleSharp => "𝄪",
            DoubleFlat => "𝄫",
        }
    }
}

impl core::fmt::Display for Accidental {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<i8> for Accidental {
    type Error = SymbolError;

    fn try_from(v: i8) -> Result<Self, Self::Error> {
        match v {
            -2 => Ok(Accidental::DoubleFlat),
            -1 => Ok(Accidental::Flat),
            0 => Ok(Accidental::Natural),
            1 => Ok(Accidental::Sharp),
            2 => Ok(Accidental::DoubleSharp),
            _ => Err(SymbolError::InvalidAccidental(v)),
        }
    }
}
