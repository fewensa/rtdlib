
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Part of the face, relative to which a mask should be placed. 
#[typetag::serde(tag = "@struct")]
pub trait MaskPoint: Object + RObject + Debug {}






impl MaskPoint {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<MaskPoint> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDMaskPointType {
  MaskPointChin,
  MaskPointEyes,
  MaskPointForehead,
  MaskPointMouth,
  
}
impl RTDMaskPointType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDMaskPointType)(text.as_ref()) }
}



