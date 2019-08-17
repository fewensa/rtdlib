
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents the type of a file. 
#[typetag::serde(tag = "@struct")]
pub trait FileType: Object + RObject + Debug {}






impl FileType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<FileType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDFileTypeType {
  FileTypeAnimation,
  FileTypeAudio,
  FileTypeDocument,
  FileTypeNone,
  FileTypePhoto,
  FileTypeProfilePhoto,
  FileTypeSecret,
  FileTypeSecretThumbnail,
  FileTypeSecure,
  FileTypeSticker,
  FileTypeThumbnail,
  FileTypeUnknown,
  FileTypeVideo,
  FileTypeVideoNote,
  FileTypeVoiceNote,
  FileTypeWallpaper,
  
}
impl RTDFileTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDFileTypeType)(text.as_ref()) }
}



