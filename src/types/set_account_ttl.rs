
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the period of inactivity after which the account of the current user will automatically be deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAccountTtl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setAccountTtl
  /// New account TTL.
  ttl: Option<AccountTtl>,
  
}



impl Object for SetAccountTtl {}
impl RObject for SetAccountTtl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setAccountTtl" }
  fn td_type(&self) -> RTDType { RTDType::SetAccountTtl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetAccountTtl {}


impl SetAccountTtl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setAccountTtl".to_string(),
      ttl: None,
      
    }
  }
  
  pub fn ttl(&self) -> Option<AccountTtl> { self.ttl.clone() }
  #[doc(hidden)] pub fn _set_ttl(&mut self, ttl: AccountTtl) -> &mut Self { self.ttl = Some(ttl); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



