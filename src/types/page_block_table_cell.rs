
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a cell of a table. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PageBlockTableCell {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockTableCell
  /// Cell text.
  text: Option<Box<RichText>>,
  /// True, if it is a header cell.
  is_header: Option<bool>,
  /// The number of columns the cell should span.
  colspan: Option<i32>,
  /// The number of rows the cell should span.
  rowspan: Option<i32>,
  /// Horizontal cell content alignment.
  align: Option<Box<PageBlockHorizontalAlignment>>,
  /// Vertical cell content alignment.
  valign: Option<Box<PageBlockVerticalAlignment>>,
  
}


impl Clone for PageBlockTableCell {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PageBlockTableCell {}
impl RObject for PageBlockTableCell {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockTableCell" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockTableCell }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PageBlockTableCell {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockTableCell".to_string(),
      text: None,
      is_header: None,
      colspan: None,
      rowspan: None,
      align: None,
      valign: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn is_header(&self) -> Option<bool> { self.is_header.clone() }
  #[doc(hidden)] pub fn _set_is_header(&mut self, is_header: bool) -> &mut Self { self.is_header = Some(is_header); self }
  
  pub fn colspan(&self) -> Option<i32> { self.colspan.clone() }
  #[doc(hidden)] pub fn _set_colspan(&mut self, colspan: i32) -> &mut Self { self.colspan = Some(colspan); self }
  
  pub fn rowspan(&self) -> Option<i32> { self.rowspan.clone() }
  #[doc(hidden)] pub fn _set_rowspan(&mut self, rowspan: i32) -> &mut Self { self.rowspan = Some(rowspan); self }
  
  pub fn align(&self) -> Option<Box<PageBlockHorizontalAlignment>> { self.align.clone() }
  #[doc(hidden)] pub fn _set_align(&mut self, align: Box<PageBlockHorizontalAlignment>) -> &mut Self { self.align = Some(align); self }
  
  pub fn valign(&self) -> Option<Box<PageBlockVerticalAlignment>> { self.valign.clone() }
  #[doc(hidden)] pub fn _set_valign(&mut self, valign: Box<PageBlockVerticalAlignment>) -> &mut Self { self.valign = Some(valign); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



