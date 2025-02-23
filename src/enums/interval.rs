use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Interval {
    Half,
    Whole,
    MinorThird,
    MajorThird,
    Fourth,
    Tritone,
    Fifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    Octave,
}

impl TryFrom<u8> for Interval {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Half),
            1 => Ok(Self::Whole),
            2 => Ok(Self::MinorThird),
            3 => Ok(Self::MajorThird),
            4 => Ok(Self::Fourth),
            5 => Ok(Self::Tritone),
            6 => Ok(Self::Fifth),
            7 => Ok(Self::MinorSixth),
            8 => Ok(Self::MajorSixth),
            9 => Ok(Self::MinorSeventh),
            10 => Ok(Self::MajorSeventh),
            11 => Ok(Self::Octave),
            _ => Err("'value' is outside of allowable range!"),
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Half => "Half Step",
            Self::Whole => "Whole Step",
            Self::MinorThird => "Minor 3rd",
            Self::MajorThird => "Major 3rd",
            Self::Fourth => "Fourth",
            Self::Tritone => "Tritone",
            Self::Fifth => "Fifth",
            Self::MinorSixth => "Minor 6th",
            Self::MajorSixth => "Major 6th",
            Self::MinorSeventh => "Minor 7th",
            Self::MajorSeventh => "Major 7th",
            Self::Octave => "Octave",
        };

        write!(f, "{}", name)
    }
}
