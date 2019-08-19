
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's address. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementAddress
  /// The address to be saved.
  address: Option<Address>,
  
}



impl Object for InputPassportElementAddress {}
impl RObject for InputPassportElementAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementAddress" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementAddress {}


impl InputPassportElementAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementAddress".to_string(),
      address: None,
      
    }
  }
  
  pub fn address(&self) -> Option<Address> { self.address.clone() }
  #[doc(hidden)] pub fn _set_address(&mut self, address: Address) -> &mut Self { self.address = Some(address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



