
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An MTProto proxy server. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyTypeMtproto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // proxyTypeMtproto
  /// The proxy's secret in hexadecimal encoding.
  secret: Option<String>,
  
}



impl Object for ProxyTypeMtproto {}
impl RObject for ProxyTypeMtproto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "proxyTypeMtproto" }
  fn td_type(&self) -> RTDType { RTDType::ProxyTypeMtproto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ProxyType for ProxyTypeMtproto {}


impl ProxyTypeMtproto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "proxyTypeMtproto".to_string(),
      secret: None,
      
    }
  }
  
  pub fn secret(&self) -> Option<String> { self.secret.clone() }
  #[doc(hidden)] pub fn _set_secret(&mut self, secret: String) -> &mut Self { self.secret = Some(secret); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



