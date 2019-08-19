
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with an invoice from a bot. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentInvoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentInvoice
  /// Product price.
  price: Option<String>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentInvoice {}
impl RObject for PushMessageContentInvoice {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentInvoice" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentInvoice }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentInvoice {}


impl PushMessageContentInvoice {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentInvoice".to_string(),
      price: None,
      is_pinned: None,
      
    }
  }
  
  pub fn price(&self) -> Option<String> { self.price.clone() }
  #[doc(hidden)] pub fn _set_price(&mut self, price: String) -> &mut Self { self.price = Some(price); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



