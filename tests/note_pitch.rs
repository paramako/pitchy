//! Integration tests for conversions between `Note` and `Pitch`.
//!
//! These tests ensure that symbolic note names and frequency-based pitches
//! convert reliably, with consistent MIDI numbers and stable roundtrips.

use core::convert::TryFrom;
use core::str::FromStr;
use pitchy::{Note, Pitch};

/// Validates that each MIDI note (0–127) can be converted to a `Pitch`,
/// then to a `Note`, and back to a `Pitch`, preserving frequency.
///
/// This ensures that all valid MIDI notes maintain accurate pitch-to-note
/// mapping with less than 0.01 Hz deviation after roundtrip.
#[test]
fn test_pitch_note_midi_roundtrip() {
    for midi in 0u8..=127 {
        let original_pitch = Pitch::try_from_midi_number(midi).unwrap();
        let note = Note::try_from(original_pitch).unwrap();
        let roundtrip_pitch = Pitch::try_from(note).unwrap();

        let delta = (original_pitch.frequency() - roundtrip_pitch.frequency()).abs();
        assert!(
            delta < 0.01,
            "Mismatch at MIDI {}: {:.3} Hz vs {:.3} Hz (Δ = {:.5})",
            midi,
            original_pitch.frequency(),
            roundtrip_pitch.frequency(),
            delta
        );
    }
}

/// Verifies that parsing a sharp note string into a `Pitch`,
/// converting it to a `Note`, and then back yields the original name
/// and MIDI number.
///
/// This tests the full roundtrip from symbolic name → `Pitch` → `Note`,
/// ensuring symbolic and semantic (MIDI) correctness.
#[test]
fn test_pitch_note_from_str_roundtrip() {
    let sharp_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let mut midi = 0u8;

    'outer: for octave in -1..=9 {
        for name in sharp_names {
            let original = format!("{name}{octave}");

            let pitch = Pitch::from_str(&original).unwrap();

            #[cfg(feature = "std")]
            {
                let note = Note::try_from(pitch).unwrap();
                assert_eq!(note.name(), original);
            }

            let midi_from_pitch = pitch.try_midi_number().unwrap();
            assert_eq!(
                midi_from_pitch, midi,
                "MIDI mismatch for {original}: expected {midi}, got {midi_from_pitch}"
            );

            if midi >= 127 {
                // G9 is the last note in MIDI tuning range
                assert_eq!(original, "G9");
                break 'outer;
            }
            midi += 1;
        }
    }

    assert_eq!(midi, 127, "Expected to test all 128 (0-127) MIDI notes");
}
