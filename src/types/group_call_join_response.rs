
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a join response for interaction with tgcalls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallJoinResponse {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Join response payload to pass to tgcalls
  payload: GroupCallPayload,
  /// Join response candidates to pass to tgcalls
  candidates: Vec<GroupCallJoinResponseCandidate>,
  
}

impl RObject for GroupCallJoinResponse {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallJoinResponse" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCallJoinResponse {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallJoinResponseBuilder {
    let mut inner = GroupCallJoinResponse::default();
    inner.td_name = "groupCallJoinResponse".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallJoinResponseBuilder { inner }
  }

  pub fn payload(&self) -> &GroupCallPayload { &self.payload }

  pub fn candidates(&self) -> &Vec<GroupCallJoinResponseCandidate> { &self.candidates }

}

#[doc(hidden)]
pub struct RTDGroupCallJoinResponseBuilder {
  inner: GroupCallJoinResponse
}

impl RTDGroupCallJoinResponseBuilder {
  pub fn build(&self) -> GroupCallJoinResponse { self.inner.clone() }

   
  pub fn payload<T: AsRef<GroupCallPayload>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().clone();
    self
  }

   
  pub fn candidates(&mut self, candidates: Vec<GroupCallJoinResponseCandidate>) -> &mut Self {
    self.inner.candidates = candidates;
    self
  }

}

impl AsRef<GroupCallJoinResponse> for GroupCallJoinResponse {
  fn as_ref(&self) -> &GroupCallJoinResponse { self }
}

impl AsRef<GroupCallJoinResponse> for RTDGroupCallJoinResponseBuilder {
  fn as_ref(&self) -> &GroupCallJoinResponse { &self.inner }
}



