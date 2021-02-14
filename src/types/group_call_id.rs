
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains the group call identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Group call identifier
  id: i64,
  
}

impl RObject for GroupCallId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallId" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCallId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallIdBuilder {
    let mut inner = GroupCallId::default();
    inner.td_name = "groupCallId".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallIdBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

}

#[doc(hidden)]
pub struct RTDGroupCallIdBuilder {
  inner: GroupCallId
}

impl RTDGroupCallIdBuilder {
  pub fn build(&self) -> GroupCallId { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

}

impl AsRef<GroupCallId> for GroupCallId {
  fn as_ref(&self) -> &GroupCallId { self }
}

impl AsRef<GroupCallId> for RTDGroupCallIdBuilder {
  fn as_ref(&self) -> &GroupCallId { &self.inner }
}



