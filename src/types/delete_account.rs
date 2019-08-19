
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteAccount
  /// The reason why the account was deleted; optional.
  reason: Option<String>,
  
}



impl Object for DeleteAccount {}
impl RObject for DeleteAccount {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteAccount" }
  fn td_type(&self) -> RTDType { RTDType::DeleteAccount }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteAccount {}


impl DeleteAccount {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteAccount".to_string(),
      reason: None,
      
    }
  }
  
  pub fn reason(&self) -> Option<String> { self.reason.clone() }
  #[doc(hidden)] pub fn _set_reason(&mut self, reason: String) -> &mut Self { self.reason = Some(reason); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



