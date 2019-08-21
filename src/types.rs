use std::fmt;

pub use ggez_goodies::Point2;
pub use ggez_goodies::Vector2;

/// This is not actually used very many places,
/// but is still useful.
#[derive(Debug)]
pub enum Error {
    GgezError(ggez::GameError),
    DeserializeError(ron::de::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::GgezError(ref e) => write!(f, "ggez error: {}", e),
            Error::DeserializeError(ref e) => write!(f, "deserialize error: {}", e),
        }
    }
}
