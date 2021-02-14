
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a payload for interaction with tgcalls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallPayload {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Value of the field ufrag
  ufrag: String,
  /// Value of the field pwd
  pwd: String,
  /// The list of fingerprints
  fingerprints: Vec<GroupCallPayloadFingerprint>,
  
}

impl RObject for GroupCallPayload {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallPayload" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCallPayload {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallPayloadBuilder {
    let mut inner = GroupCallPayload::default();
    inner.td_name = "groupCallPayload".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallPayloadBuilder { inner }
  }

  pub fn ufrag(&self) -> &String { &self.ufrag }

  pub fn pwd(&self) -> &String { &self.pwd }

  pub fn fingerprints(&self) -> &Vec<GroupCallPayloadFingerprint> { &self.fingerprints }

}

#[doc(hidden)]
pub struct RTDGroupCallPayloadBuilder {
  inner: GroupCallPayload
}

impl RTDGroupCallPayloadBuilder {
  pub fn build(&self) -> GroupCallPayload { self.inner.clone() }

   
  pub fn ufrag<T: AsRef<str>>(&mut self, ufrag: T) -> &mut Self {
    self.inner.ufrag = ufrag.as_ref().to_string();
    self
  }

   
  pub fn pwd<T: AsRef<str>>(&mut self, pwd: T) -> &mut Self {
    self.inner.pwd = pwd.as_ref().to_string();
    self
  }

   
  pub fn fingerprints(&mut self, fingerprints: Vec<GroupCallPayloadFingerprint>) -> &mut Self {
    self.inner.fingerprints = fingerprints;
    self
  }

}

impl AsRef<GroupCallPayload> for GroupCallPayload {
  fn as_ref(&self) -> &GroupCallPayload { self }
}

impl AsRef<GroupCallPayload> for RTDGroupCallPayloadBuilder {
  fn as_ref(&self) -> &GroupCallPayload { &self.inner }
}



