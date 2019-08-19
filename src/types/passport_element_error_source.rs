
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains the description of an error in a Telegram Passport element. 
#[typetag::serde(tag = "@struct")]
pub trait PassportElementErrorSource: Object + RObject + Debug {}






impl PassportElementErrorSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<PassportElementErrorSource> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDPassportElementErrorSourceType {
  PassportElementErrorSourceDataField,
  PassportElementErrorSourceFile,
  PassportElementErrorSourceFiles,
  PassportElementErrorSourceFrontSide,
  PassportElementErrorSourceReverseSide,
  PassportElementErrorSourceSelfie,
  PassportElementErrorSourceTranslationFile,
  PassportElementErrorSourceTranslationFiles,
  PassportElementErrorSourceUnspecified,
  
}
impl RTDPassportElementErrorSourceType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDPassportElementErrorSourceType)(text.as_ref()) }
}



