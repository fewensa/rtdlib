
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's address. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementAddress
  /// Address.
  address: Option<Address>,
  
}



impl Object for PassportElementAddress {}
impl RObject for PassportElementAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementAddress" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementAddress {}


impl PassportElementAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementAddress".to_string(),
      address: None,
      
    }
  }
  
  pub fn address(&self) -> Option<Address> { self.address.clone() }
  #[doc(hidden)] pub fn _set_address(&mut self, address: Address) -> &mut Self { self.address = Some(address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



