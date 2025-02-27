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
        let value = value.to_lowercase();

        match value.as_str() {
            "c" => Ok(Self::C),
            "c#" | "cs" | "db" => Ok(Self::CSharpDFlat),
            "d" => Ok(Self::D),
            "d#" | "ds" | "eb" => Ok(Self::DSharpEFlat),
            "e" => Ok(Self::E),
            "f" => Ok(Self::F),
            "f#" | "fs" | "gb" => Ok(Self::FSharpGFlat),
            "g" => Ok(Self::G),
            "g#" | "gs" | "ab" => Ok(Self::GSharpAFlat),
            "a" => Ok(Self::A),
            "a#" | "as" | "bb" => Ok(Self::ASharpBFlat),
            "b" => Ok(Self::B),
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
