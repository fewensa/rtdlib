
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a payload fingerprint for interaction with tgcalls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallPayloadFingerprint {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Value of the field hash
  hash: String,
  /// Value of the field setup
  setup: String,
  /// Value of the field fingerprint
  fingerprint: String,
  
}

impl RObject for GroupCallPayloadFingerprint {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallPayloadFingerprint" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCallPayloadFingerprint {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallPayloadFingerprintBuilder {
    let mut inner = GroupCallPayloadFingerprint::default();
    inner.td_name = "groupCallPayloadFingerprint".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallPayloadFingerprintBuilder { inner }
  }

  pub fn hash(&self) -> &String { &self.hash }

  pub fn setup(&self) -> &String { &self.setup }

  pub fn fingerprint(&self) -> &String { &self.fingerprint }

}

#[doc(hidden)]
pub struct RTDGroupCallPayloadFingerprintBuilder {
  inner: GroupCallPayloadFingerprint
}

impl RTDGroupCallPayloadFingerprintBuilder {
  pub fn build(&self) -> GroupCallPayloadFingerprint { self.inner.clone() }

   
  pub fn hash<T: AsRef<str>>(&mut self, hash: T) -> &mut Self {
    self.inner.hash = hash.as_ref().to_string();
    self
  }

   
  pub fn setup<T: AsRef<str>>(&mut self, setup: T) -> &mut Self {
    self.inner.setup = setup.as_ref().to_string();
    self
  }

   
  pub fn fingerprint<T: AsRef<str>>(&mut self, fingerprint: T) -> &mut Self {
    self.inner.fingerprint = fingerprint.as_ref().to_string();
    self
  }

}

impl AsRef<GroupCallPayloadFingerprint> for GroupCallPayloadFingerprint {
  fn as_ref(&self) -> &GroupCallPayloadFingerprint { self }
}

impl AsRef<GroupCallPayloadFingerprint> for RTDGroupCallPayloadFingerprintBuilder {
  fn as_ref(&self) -> &GroupCallPayloadFingerprint { &self.inner }
}



