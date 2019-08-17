
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Points to a file. 
#[typetag::serde(tag = "@struct")]
pub trait InputFile: Object + RObject + Debug {}






impl InputFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<InputFile> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDInputFileType {
  InputFileGenerated,
  InputFileId,
  InputFileLocal,
  InputFileRemote,
  
}
impl RTDInputFileType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDInputFileType)(text.as_ref()) }
}



