
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A table. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockTable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockTable
  /// Table caption.
  caption: Option<Box<RichText>>,
  /// Table cells.
  cells: Option<Vec<Vec<PageBlockTableCell>>>,
  /// True, if the table is bordered.
  is_bordered: Option<bool>,
  /// True, if the table is striped.
  is_striped: Option<bool>,
  
}


impl Clone for PageBlockTable {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockTable {}
impl RObject for PageBlockTable {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockTable" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockTable }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockTable {}


impl PageBlockTable {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockTable".to_string(),
      caption: None,
      cells: None,
      is_bordered: None,
      is_striped: None,
      
    }
  }
  
  pub fn caption(&self) -> Option<Box<RichText>> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: Box<RichText>) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn cells(&self) -> Option<Vec<Vec<PageBlockTableCell>>> { self.cells.clone() }
  #[doc(hidden)] pub fn _set_cells(&mut self, cells: Vec<Vec<PageBlockTableCell>>) -> &mut Self { self.cells = Some(cells); self }
  
  pub fn is_bordered(&self) -> Option<bool> { self.is_bordered.clone() }
  #[doc(hidden)] pub fn _set_is_bordered(&mut self, is_bordered: bool) -> &mut Self { self.is_bordered = Some(is_bordered); self }
  
  pub fn is_striped(&self) -> Option<bool> { self.is_striped.clone() }
  #[doc(hidden)] pub fn _set_is_striped(&mut self, is_striped: bool) -> &mut Self { self.is_striped = Some(is_striped); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



