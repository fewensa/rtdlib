
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a server for relaying call data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallServer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Server identifier
  id: isize,
  /// Server IPv4 address
  ip_address: String,
  /// Server IPv6 address
  ipv6_address: String,
  /// Server port number
  port: i64,
  /// Server type
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: CallServerType,
  
}

impl RObject for CallServer {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callServer" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl CallServer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallServerBuilder {
    let mut inner = CallServer::default();
    inner.td_name = "callServer".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCallServerBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn ip_address(&self) -> &String { &self.ip_address }

  pub fn ipv6_address(&self) -> &String { &self.ipv6_address }

  pub fn port(&self) -> i64 { self.port }

  pub fn type_(&self) -> &CallServerType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDCallServerBuilder {
  inner: CallServer
}

impl RTDCallServerBuilder {
  pub fn build(&self) -> CallServer { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn ip_address<T: AsRef<str>>(&mut self, ip_address: T) -> &mut Self {
    self.inner.ip_address = ip_address.as_ref().to_string();
    self
  }

   
  pub fn ipv6_address<T: AsRef<str>>(&mut self, ipv6_address: T) -> &mut Self {
    self.inner.ipv6_address = ipv6_address.as_ref().to_string();
    self
  }

   
  pub fn port(&mut self, port: i64) -> &mut Self {
    self.inner.port = port;
    self
  }

   
  pub fn type_<T: AsRef<CallServerType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<CallServer> for CallServer {
  fn as_ref(&self) -> &CallServer { self }
}

impl AsRef<CallServer> for RTDCallServerBuilder {
  fn as_ref(&self) -> &CallServer { &self.inner }
}



