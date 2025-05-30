use core::str::FromStr;

use crate::Pitch;

/// (midi number, note, octave, frequency)
const NOTE_DATASETS: [(u8, &str, i8, f64); 6] = [
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
        let note = Pitch::from_str(name).unwrap();
        assert_eq!(midi, note.try_midi_number().unwrap());
        assert!((note.frequency() - hz).abs() < 0.01);
        assert_eq!(Some(octave), note.octave());
    }
}

#[test]
fn test_enharmonic() {
    let datasets = [("C#4", "Db4"), ("G#5", "Ab5"), ("F#6", "Gb6")];

    for (sharp_str, flat_str) in datasets {
        let sharp = Pitch::from_str(sharp_str).unwrap();
        let flat = Pitch::from_str(flat_str).unwrap();
        assert!((sharp.frequency() - flat.frequency()).abs() < 0.01);
    }
}

#[test]
fn test_transpose() {
    let datasets = [
        ("C4", 2.0, 62),
        ("A4", 1.0, 70),
        ("G#3", 3.0, 59),
        ("F2", -2.0, 39),
        ("D5", -12.0, 62),
        ("E3", 0.0, 52),
        ("C#5", -1.0, 72),
        ("B1", 13.0, 48),
    ];

    for (name, semitones, new_midi) in datasets {
        let note = Pitch::from_str(name).unwrap();
        let up = note.transpose(semitones);
        assert_eq!(up.try_midi_number().unwrap(), new_midi);
    }
}

#[test]
fn test_try_from_midi_number() {
    for (midi, _name, octave, hz) in NOTE_DATASETS {
        let pitch = Pitch::try_from_midi_number(midi).unwrap();
        assert!((pitch.frequency() - hz).abs() < 0.01);
        assert_eq!(Some(octave), pitch.octave());
        assert_eq!(pitch.try_midi_number().unwrap(), midi);
    }
}
