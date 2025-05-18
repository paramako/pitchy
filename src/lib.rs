//! pitchy
//!
//! `pitchy` is a minimal, `no_std`-friendly Rust library for working with musical notes,
//! frequencies (Hz), MIDI numbers, and pitch operations like transposition and octave shifts.
//!
//! # Features
//!
//! - Convert frequencies to MIDI numbers and back
//! - Construct notes from symbols like `"C#4"` or MIDI numbers
//! - Transpose notes by semitones and change octaves
//! - Optional formatting of note names like "A4" when `std` is enabled
//! - Uses the `libm` math backend in `no_std` mode (enabled via the `libm` feature)
//!
//! # Example
//!
//! ```rust
//! use pitchy::Note;
//! use std::str::FromStr;
//!
//! let a4 = Note::from_str("A4").unwrap();
//! assert_eq!(a4.midi_number(), Some(69));
//!
//! let up = a4.transpose(12.0);
//! assert_eq!(up.midi_number(), Some(81)); // A5
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod math;
mod note;

pub use note::{Note, NoteError};
