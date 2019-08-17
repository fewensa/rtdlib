
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes a Vertical alignment of a table cell content. 
#[typetag::serde(tag = "@struct")]
pub trait PageBlockVerticalAlignment: Object + RObject + Debug {}






impl PageBlockVerticalAlignment {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<PageBlockVerticalAlignment> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDPageBlockVerticalAlignmentType {
  PageBlockVerticalAlignmentBottom,
  PageBlockVerticalAlignmentMiddle,
  PageBlockVerticalAlignmentTop,
  
}
impl RTDPageBlockVerticalAlignmentType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDPageBlockVerticalAlignmentType)(text.as_ref()) }
}



