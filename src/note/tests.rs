use core::str::FromStr;

use crate::Note;

/// (midi number, note, octave, frequency)
const NOTE_DATASETS: [(u8, &'static str, i8, f64); 6] = [
    (57, "A3", 3, 220.00),
    (69, "A4", 4, 440.0),
    (66, "F#4", 4, 369.99),
    (34, "A#1", 1, 58.27),
    (1, "C#-1", -1, 8.662),
    (127, "G9", 9, 12543.85),
];

#[test]
fn test_from_str() {
    for (midi, name, octave, hz) in NOTE_DATASETS {
        let note = Note::from_str(name).unwrap();
        assert_eq!(Some(midi), note.midi_number());
        assert!((note.frequency() - hz).abs() < 0.01);
        assert_eq!(Some(octave), note.octave());
        #[cfg(feature = "std")]
        assert_eq!(note.name().as_deref(), Some(name));
    }
}

#[test]
fn test_enharmonic() {
    let datasets = [("C#4", "Db4"), ("G#5", "Ab5"), ("F#6", "Gb6")];

    for (sharp_str, flat_str) in datasets {
        let sharp = Note::from_str(sharp_str).unwrap();
        let flat = Note::from_str(flat_str).unwrap();
        assert!((sharp.frequency() - flat.frequency()).abs() < 0.01);
        #[cfg(feature = "std")]
        assert_eq!(flat.name().as_deref(), Some(sharp_str));
    }
}

#[test]
fn test_transpose() {
    let datasets = [
        ("C4", 2.0, "D4", 62),
        ("A4", 1.0, "A#4", 70),
        ("G#3", 3.0, "B3", 59),
        ("F2", -2.0, "D#2", 39),
        ("D5", -12.0, "D4", 62),
        ("E3", 0.0, "E3", 52),
        ("C#5", -1.0, "C5", 72),
        ("B1", 13.0, "C3", 48),
    ];

    for (name, semitones, new_name, new_midi) in datasets {
        let note = Note::from_str(name).unwrap();
        let up = note.transpose(semitones);
        assert_eq!(up.midi_number(), Some(new_midi));
        #[cfg(feature = "std")]
        assert_eq!(up.name().as_deref(), Some(new_name));
        #[cfg(not(feature = "std"))]
        let _ = new_name; // prevent unused warning
    }
}

#[test]
fn test_note_letter() {
    let datasets = [
        ("C#4", "C#"),
        ("Ab5", "G#"), // G#5 = Ab5
        ("B4", "B"),
    ];

    for (note_str, letter) in datasets {
        let note = Note::from_str(note_str).unwrap();

        assert_eq!(note.note_letter(), Some(letter));
    }
}

#[test]
fn test_try_from_midi_number() {
    for (midi, name, octave, hz) in NOTE_DATASETS {
        let note = Note::try_from_midi_number(midi).unwrap();
        assert!((note.frequency() - hz).abs() < 0.01);
        assert_eq!(Some(octave), note.octave());
        #[cfg(feature = "std")]
        assert_eq!(note.name().as_deref(), Some(name));
        #[cfg(not(feature = "std"))]
        let _ = name; // prevent unused warning
    }
}

#[cfg(feature = "std")]
#[test]
fn test_name_roundtrip() {
    for midi in 0..=127 {
        let note = Note::try_from_midi_number(midi).unwrap();
        let name = note.name().unwrap();
        let parsed = Note::from_str(&name).unwrap();
        assert_eq!(note.midi_number(), parsed.midi_number());
        assert!((note.frequency() - parsed.frequency()).abs() < 0.01);
    }
}
