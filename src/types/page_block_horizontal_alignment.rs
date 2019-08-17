
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes a horizontal alignment of a table cell content. 
#[typetag::serde(tag = "@struct")]
pub trait PageBlockHorizontalAlignment: Object + RObject + Debug {}






impl PageBlockHorizontalAlignment {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<PageBlockHorizontalAlignment> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDPageBlockHorizontalAlignmentType {
  PageBlockHorizontalAlignmentCenter,
  PageBlockHorizontalAlignmentLeft,
  PageBlockHorizontalAlignmentRight,
  
}
impl RTDPageBlockHorizontalAlignmentType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDPageBlockHorizontalAlignmentType)(text.as_ref()) }
}



