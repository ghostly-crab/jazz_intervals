use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

impl TryFrom<u8> for Direction {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Up),
            1 => Ok(Self::Down),
            _ => Err("'value' is outside of the allowable range!"),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Up => "Up",
            Self::Down => "Down",
        };

        write!(f, "{}", name)
    }
}
