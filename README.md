# pitchy

[![Crates.io](https://img.shields.io/crates/v/pitchy)](https://crates.io/crates/pitchy)
[![Docs.rs](https://docs.rs/pitchy/badge.svg)](https://docs.rs/pitchy)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Changelog](https://img.shields.io/badge/changelog-md-blue)](CHANGELOG.md)

**Minimal, `no_std`-friendly Rust library for working with musical pitches, frequencies, MIDI numbers, and symbolic note representations.**

---

## ✨ Features

- Convert between musical frequencies (Hz) and MIDI note numbers
- Transpose notes by semitones or octaves
- Convert ASCII note names like "C#4" or "Db3" into pitch (frequency-based) representations
- Reconstruct symbolic notes (e.g., "C#4") from pitch via default sharp-based spelling
- Extract musical components like note letter, accidental, and octave
- Lightweight and `no_std` compatible (via feature flag)

---

## 🚀 Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
pitchy = "0.2"
```

Or, to use in `no_std` mode:

```toml
[dependencies.pitchy]
version = "0.2"
default-features = false
features = ["libm"]
```

---

## 🔧 Example Usage

### Parse and Analyze a Note
```rust
use pitchy::Pitch;
use core::str::FromStr;

let pitch = Pitch::from_str("A4").unwrap();
assert_eq!(pitch.try_midi_number().unwrap(), 69);
assert_eq!(pitch.frequency(), 440.0);
```

### Transpose a Note
```rust
let pitch = Pitch::from_str("C4").unwrap();
let up = pitch.transpose(4.0);
assert_eq!(up.try_midi_number().unwrap(), 64); // E4
```

### Convert from MIDI Number
```rust
let pitch = Pitch::try_from_midi_number(60).unwrap();
let actual = pitch.frequency(); // 261.6255653005986
let expected = 261.625565; // C4
let epsilon = 1e-6; // 0.000001
assert!((actual - expected).abs() < epsilon);
```

### Convert Pitch to Note
```rust
use pitchy::{Pitch, Note};

let pitch = Pitch::new(277.183); // C#4
let note = Note::try_from(pitch).unwrap();

#[cfg(feature = "std")]
assert_eq!(note.name(), "C#4");

assert_eq!(note.letter(), pitchy::NoteLetter::C);
assert_eq!(note.accidental(), pitchy::Accidental::Sharp);
assert_eq!(note.octave(), 4);
```

---

## ⚙️ Optional Features

- `std` *(enabled by default)*: enables note name formatting
- `libm`: enables the `libm` math backend used in `no_std` mode

To build without `std`, use:

```bash
cargo build --no-default-features --features libm
```

---

## 📄 License

MIT © paramako
