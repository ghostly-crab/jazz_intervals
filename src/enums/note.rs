// external
use std::fmt;

// internal
use super::{Direction, Interval};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Note {
    C,
    CSharpDFlat,
    D,
    DSharpEFlat,
    E,
    F,
    FSharpGFlat,
    G,
    GSharpAFlat,
    A,
    ASharpBFlat,
    B,
}

impl Note {
    pub fn from_interval(&self, direction: Direction, interval: Interval) -> Self {
        let new_note: u8 = if direction == Direction::Up { (*self as u8 + (interval as u8 + 1)) % 12 } else { (*self as u8 + 12 - (interval as u8 + 1)) % 12 };
        let new_note: Note = new_note.try_into().expect("% 12 should guarantee that a valid note is returned");

        new_note
    }
}

impl TryFrom<u8> for Note {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::C),
            1 => Ok(Self::CSharpDFlat),
            2 => Ok(Self::D),
            3 => Ok(Self::DSharpEFlat),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::FSharpGFlat),
            7 => Ok(Self::G),
            8 => Ok(Self::GSharpAFlat),
            9 => Ok(Self::A),
            10 => Ok(Self::ASharpBFlat),
            11 => Ok(Self::B),
            _ => Err("'value' is outside of allowable range!"),
        }
    }
}

impl TryFrom<&str> for Note {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C" => Ok(Self::C),
            "C#" | "Db" => Ok(Self::CSharpDFlat),
            "D" => Ok(Self::D),
            "D#" | "Eb" => Ok(Self::DSharpEFlat),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "F#" | "Gb" => Ok(Self::FSharpGFlat),
            "G" => Ok(Self::G),
            "G#" | "Ab" => Ok(Self::GSharpAFlat),
            "A" => Ok(Self::A),
            "A#" | "Bb" => Ok(Self::ASharpBFlat),
            "B" => Ok(Self::B),
            _ => Err("'value' is not a valid note!"),
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::C => "C",
            Self::CSharpDFlat => "C# | Db",
            Self::D => "D",
            Self::DSharpEFlat => "D# | Eb",
            Self::E => "E",
            Self::F => "F",
            Self::FSharpGFlat => "F# | Gb",
            Self::G => "G",
            Self::GSharpAFlat => "G# | Ab",
            Self::A => "A",
            Self::ASharpBFlat => "A# | Bb",
            Self::B => "B",
        };

        write!(f, "{}", name)
    }
}
