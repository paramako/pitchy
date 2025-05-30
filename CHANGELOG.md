# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] – 2025-05-18

### ⚠️ Breaking Changes

- Introduced a new split between symbolic and frequency representations:
  - `Pitch`: frequency-based structure for audio computations
  - `Note`: symbolic structure (letter + accidental + octave)
- Parsing note strings like `"C#4"` now uses `Pitch::from_str` instead of `Note::from_str`
- Transposition functionality moved from `Note` to `Pitch` to better reflect their roles
- Unified error handling under `PitchyError` with new variants

### ✨ Changes

- Added `NoteLetter` and `Accidental` types for expressive symbolic note modeling
- Implemented `TryFrom<Pitch>` for `Note` and `TryFrom<Note>` for `Pitch` for converting between representations
- Richer module- and struct-level documentation

## [0.1.2] – 2025-05-30

### 🛠 Fixes

- Corrected broken code examples in the README

## [0.1.1] – 2025-05-17

🎉 Initial release of `pitchy`.

### ✨ Features

- Convert between MIDI note numbers, frequencies (Hz), and symbolic names (e.g. `"C#4"`)
- Support transposition in semitones (e.g. `.transpose(1.0)`)
- Construct notes from:
  - MIDI numbers: `Note::try_from_midi_number(...)`
  - Frequencies: `Note::new(...)`
  - Strings: `Note::from_str(...)`
- Query note properties:
  - `.frequency()`
  - `.midi_number()`
  - `.octave()`
  - `.note_parts()`
  - `.name()` *(requires `std`)*
- Full support for `no_std` via `--no-default-features --features libm`
- Friendly error reporting via `NoteError`

## [0.1.0] – YANKED