# pitchy

[![Crates.io](https://img.shields.io/crates/v/pitchy)](https://crates.io/crates/pitchy)
[![Docs.rs](https://docs.rs/pitchy/badge.svg)](https://docs.rs/pitchy)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Changelog](https://img.shields.io/badge/changelog-md-blue)](CHANGELOG.md)

**Minimalistic Rust library for working with musical notes, frequencies (Hz), MIDI numbers, and pitch operations like transposition and octave shifts.**

---

## ✨ Features

- Parse note names like `"A4"`, `"C#3"`, `"Db5"`
- Convert to MIDI note number and frequency in Hz
- Transpose notes by semitones or octaves
- Lightweight and `no_std` compatible (via feature flag)

---

## 🚀 Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
pitchy = "0.1"
```

Or, to use in `no_std` mode:

```toml
[dependencies.pitchy]
version = "0.1"
default-features = false
features = ["libm"]
```

---

## 🔧 Example Usage

### Parse and Analyze a Note
```rust
use pitchy::Note;
use core::str::FromStr;

let note = Note::from_str("A4").unwrap();
assert_eq!(note.midi_number(), Some(69));
assert_eq!(note.frequency(), 440.0);
```

### Transpose a Note
```rust
let note = Note::from_str("C4").unwrap();
let up = note.transpose(4.0);
assert_eq!(up.midi_number(), Some(64)); // E4
```

### Convert from MIDI Number
```rust
let note = Note::try_from_midi_number(60).unwrap();
let actual = note.frequency(); // 261.6255653005986
let expected = 261.625565; // C4
let epsilon = 1e-6; // 0.000001
assert!((actual - expected).abs() < epsilon);
```

### Get Note Parts (no_std friendly)
```rust
let note = Note::from_str("F#3").unwrap();
let (semitone, octave) = note.note_parts().unwrap();
assert_eq!(semitone, 6); // F#
assert_eq!(octave, 3);
```

---

## ⚙️ Optional Features

- `std` *(enabled by default)*: enables note name formatting and `Display` impls
- `libm`: enables the `libm` math backend used in `no_std` mode

To build without `std`, use:

```bash
cargo build --no-default-features --features libm
```

---

## 📄 License

MIT © paramako
