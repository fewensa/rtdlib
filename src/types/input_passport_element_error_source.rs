
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains the description of an error in a Telegram Passport element; for bots only. 
#[typetag::serde(tag = "@struct")]
pub trait InputPassportElementErrorSource: Object + RObject + Debug {}






impl InputPassportElementErrorSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<InputPassportElementErrorSource> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDInputPassportElementErrorSourceType {
  InputPassportElementErrorSourceDataField,
  InputPassportElementErrorSourceFile,
  InputPassportElementErrorSourceFiles,
  InputPassportElementErrorSourceFrontSide,
  InputPassportElementErrorSourceReverseSide,
  InputPassportElementErrorSourceSelfie,
  InputPassportElementErrorSourceTranslationFile,
  InputPassportElementErrorSourceTranslationFiles,
  InputPassportElementErrorSourceUnspecified,
  
}
impl RTDInputPassportElementErrorSourceType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDInputPassportElementErrorSourceType)(text.as_ref()) }
}



