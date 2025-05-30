//! # pitchy
//!
//! `pitchy` is a minimal, `no_std`-friendly Rust library for working with musical pitch,
//! represented as audio frequencies (Hz). It supports conversion to and from MIDI numbers,
//! pitch transposition, and symbolic note mapping.
//!
//! # Features
//!
//! - Convert frequencies to MIDI note numbers and back
//! - Transpose pitches by semitones with precise frequency calculations
//! - Query pitch octave and MIDI number mappings
//! - Parse standard note strings like `"C#4"` into [`Pitch`] values
//! - Optional formatting of symbolic note names like `"A4"` when `std` is enabled
//! - Uses the [`libm`](https://crates.io/crates/libm) math backend in `no_std` mode (via the `libm` feature)
//!
//! # Example
//!
//! ```rust
//! use pitchy::Pitch;
//! use std::str::FromStr;
//!
//! let a4 = Pitch::from_str("A4").unwrap();
//! assert_eq!(a4.try_midi_number().unwrap(), 69);
//!
//! let up = a4.transpose(12.0);
//! assert_eq!(up.try_midi_number().unwrap(), 81); // A5
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod error;
mod math;
mod note;
mod pitch;

pub use error::PitchyError;
pub use note::{Accidental, Note, NoteLetter};
pub use pitch::Pitch;
