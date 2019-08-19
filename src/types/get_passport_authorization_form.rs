
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a Telegram Passport authorization form for sharing data with a service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPassportAuthorizationForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getPassportAuthorizationForm
  /// User identifier of the service's bot.
  bot_user_id: Option<i32>,
  /// Telegram Passport element types requested by the service.
  scope: Option<String>,
  /// Service's public_key.
  public_key: Option<String>,
  /// Authorization form nonce provided by the service.
  nonce: Option<String>,
  
}



impl Object for GetPassportAuthorizationForm {}
impl RObject for GetPassportAuthorizationForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPassportAuthorizationForm" }
  fn td_type(&self) -> RTDType { RTDType::GetPassportAuthorizationForm }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetPassportAuthorizationForm {}


impl GetPassportAuthorizationForm {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getPassportAuthorizationForm".to_string(),
      bot_user_id: None,
      scope: None,
      public_key: None,
      nonce: None,
      
    }
  }
  
  pub fn bot_user_id(&self) -> Option<i32> { self.bot_user_id.clone() }
  #[doc(hidden)] pub fn _set_bot_user_id(&mut self, bot_user_id: i32) -> &mut Self { self.bot_user_id = Some(bot_user_id); self }
  
  pub fn scope(&self) -> Option<String> { self.scope.clone() }
  #[doc(hidden)] pub fn _set_scope(&mut self, scope: String) -> &mut Self { self.scope = Some(scope); self }
  
  pub fn public_key(&self) -> Option<String> { self.public_key.clone() }
  #[doc(hidden)] pub fn _set_public_key(&mut self, public_key: String) -> &mut Self { self.public_key = Some(public_key); self }
  
  pub fn nonce(&self) -> Option<String> { self.nonce.clone() }
  #[doc(hidden)] pub fn _set_nonce(&mut self, nonce: String) -> &mut Self { self.nonce = Some(nonce); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



