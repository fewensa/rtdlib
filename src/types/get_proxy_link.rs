
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProxyLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getProxyLink
  /// Proxy identifier.
  proxy_id: Option<i32>,
  
}



impl Object for GetProxyLink {}
impl RObject for GetProxyLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getProxyLink" }
  fn td_type(&self) -> RTDType { RTDType::GetProxyLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetProxyLink {}


impl GetProxyLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getProxyLink".to_string(),
      proxy_id: None,
      
    }
  }
  
  pub fn proxy_id(&self) -> Option<i32> { self.proxy_id.clone() }
  #[doc(hidden)] pub fn _set_proxy_id(&mut self, proxy_id: i32) -> &mut Self { self.proxy_id = Some(proxy_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



