
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Telegram Passport data has been received; for bots only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePassportDataReceived {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messagePassportDataReceived
  /// List of received Telegram Passport elements.
  elements: Option<Vec<EncryptedPassportElement>>,
  /// Encrypted data credentials.
  credentials: Option<EncryptedCredentials>,
  
}



impl Object for MessagePassportDataReceived {}
impl RObject for MessagePassportDataReceived {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePassportDataReceived" }
  fn td_type(&self) -> RTDType { RTDType::MessagePassportDataReceived }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessagePassportDataReceived {}


impl MessagePassportDataReceived {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messagePassportDataReceived".to_string(),
      elements: None,
      credentials: None,
      
    }
  }
  
  pub fn elements(&self) -> Option<Vec<EncryptedPassportElement>> { self.elements.clone() }
  #[doc(hidden)] pub fn _set_elements(&mut self, elements: Vec<EncryptedPassportElement>) -> &mut Self { self.elements = Some(elements); self }
  
  pub fn credentials(&self) -> Option<EncryptedCredentials> { self.credentials.clone() }
  #[doc(hidden)] pub fn _set_credentials(&mut self, credentials: EncryptedCredentials) -> &mut Self { self.credentials = Some(credentials); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



