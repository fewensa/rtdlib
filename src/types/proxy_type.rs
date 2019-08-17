
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes the type of the proxy server. 
#[typetag::serde(tag = "@struct")]
pub trait ProxyType: Object + RObject + Debug {}






impl ProxyType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<ProxyType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDProxyTypeType {
  ProxyTypeHttp,
  ProxyTypeMtproto,
  ProxyTypeSocks5,
  
}
impl RTDProxyTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDProxyTypeType)(text.as_ref()) }
}



