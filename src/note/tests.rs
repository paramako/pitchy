#[test]
fn test_try_from_pitch_to_note() {
    use crate::{Accidental, Note, Pitch, note::symbol::NoteLetter};
    use core::convert::TryFrom;

    struct Case {
        midi: u8,
        expected_letter: NoteLetter,
        expected_accidental: Accidental,
        expected_octave: i8,
        #[cfg(feature = "std")]
        expected_name: &'static str,
    }

    let cases = [
        Case {
            midi: 0,
            expected_letter: NoteLetter::C,
            expected_accidental: Accidental::Natural,
            expected_octave: -1,
            #[cfg(feature = "std")]
            expected_name: "C-1",
        },
        Case {
            midi: 60,
            expected_letter: NoteLetter::C,
            expected_accidental: Accidental::Natural,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "C4",
        },
        Case {
            midi: 61,
            expected_letter: NoteLetter::C,
            expected_accidental: Accidental::Sharp,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "C#4",
        },
        Case {
            midi: 62,
            expected_letter: NoteLetter::D,
            expected_accidental: Accidental::Natural,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "D4",
        },
        Case {
            midi: 63,
            expected_letter: NoteLetter::D,
            expected_accidental: Accidental::Sharp,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "D#4",
        },
        Case {
            midi: 64,
            expected_letter: NoteLetter::E,
            expected_accidental: Accidental::Natural,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "E4",
        },
        Case {
            midi: 65,
            expected_letter: NoteLetter::F,
            expected_accidental: Accidental::Natural,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "F4",
        },
        Case {
            midi: 66,
            expected_letter: NoteLetter::F,
            expected_accidental: Accidental::Sharp,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "F#4",
        },
        Case {
            midi: 67,
            expected_letter: NoteLetter::G,
            expected_accidental: Accidental::Natural,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "G4",
        },
        Case {
            midi: 68,
            expected_letter: NoteLetter::G,
            expected_accidental: Accidental::Sharp,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "G#4",
        },
        Case {
            midi: 69,
            expected_letter: NoteLetter::A,
            expected_accidental: Accidental::Natural,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "A4",
        },
        Case {
            midi: 70,
            expected_letter: NoteLetter::A,
            expected_accidental: Accidental::Sharp,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "A#4",
        },
        Case {
            midi: 71,
            expected_letter: NoteLetter::B,
            expected_accidental: Accidental::Natural,
            expected_octave: 4,
            #[cfg(feature = "std")]
            expected_name: "B4",
        },
        Case {
            midi: 72,
            expected_letter: NoteLetter::C,
            expected_accidental: Accidental::Natural,
            expected_octave: 5,
            #[cfg(feature = "std")]
            expected_name: "C5",
        },
    ];

    for case in cases {
        let pitch = Pitch::try_from_midi_number(case.midi).unwrap();
        let note = Note::try_from(pitch).unwrap();

        assert_eq!(
            note.letter(),
            case.expected_letter,
            "Wrong letter for MIDI {}",
            case.midi
        );
        assert_eq!(
            note.accidental(),
            case.expected_accidental,
            "Wrong accidental for MIDI {}",
            case.midi
        );
        assert_eq!(
            note.octave(),
            case.expected_octave,
            "Wrong octave for MIDI {}",
            case.midi
        );

        #[cfg(feature = "std")]
        assert_eq!(
            note.name(),
            case.expected_name,
            "Wrong name for MIDI {}",
            case.midi
        );
    }
}
