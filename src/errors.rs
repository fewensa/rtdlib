
use std::{io, fmt, error};

#[derive(Debug)]
pub enum RTDError {
  Io(io::Error),
  SerdeJson(serde_json::Error),
  Custom(String),
}

pub type RTDResult<T> = Result<T, RTDError>;

impl RTDError {
  pub fn custom(msg: String) -> Self { RTDError::Custom(msg) }
}

impl fmt::Display for RTDError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RTDError::Io(ref err) => write!(f, "IO error: {}", err),
      RTDError::SerdeJson(ref err) => write!(f, "Serde json error: {}", err),
      RTDError::Custom(msg) => write!(f, "{}", msg),
    }
  }
}

impl error::Error for RTDError {
  fn cause(&self) -> Option<&dyn error::Error> {
    match *self {
      RTDError::Io(ref err) => Some(err),
      RTDError::SerdeJson(ref err) => Some(err),
      RTDError::Custom(_) => None
    }
  }
}

impl From<io::Error> for RTDError {
  fn from(err: io::Error) -> RTDError {
    RTDError::Io(err)
  }
}

impl From<serde_json::Error> for RTDError {
  fn from(err: serde_json::Error) -> RTDError {
    RTDError::SerdeJson(err)
  }
}


