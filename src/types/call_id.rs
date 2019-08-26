
use crate::types::*;
use crate::errors::*;




/// Contains the call identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Call identifier
  id: i32,
  
}

impl RObject for CallId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callId" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl CallId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallIdBuilder {
    let mut inner = CallId::default();
    inner.td_name = "callId".to_string();
    RTDCallIdBuilder { inner }
  }

  pub fn id(&self) -> i32 { self.id }

}

#[doc(hidden)]
pub struct RTDCallIdBuilder {
  inner: CallId
}

impl RTDCallIdBuilder {
  pub fn build(&self) -> CallId { self.inner.clone() }

   
  pub fn id(&mut self, id: i32) -> &mut Self {
    self.inner.id = id;
    self
  }

}

impl AsRef<CallId> for CallId {
  fn as_ref(&self) -> &CallId { self }
}

impl AsRef<CallId> for RTDCallIdBuilder {
  fn as_ref(&self) -> &CallId { &self.inner }
}



