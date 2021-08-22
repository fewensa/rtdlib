
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a group of video synchronization source identifiers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallVideoSourceGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The semantics of sources, one of "SIM" or "FID"
  semantics: String,
  /// The list of synchronization source identifiers
  source_ids: Vec<i64>,
  
}

impl RObject for GroupCallVideoSourceGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallVideoSourceGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCallVideoSourceGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallVideoSourceGroupBuilder {
    let mut inner = GroupCallVideoSourceGroup::default();
    inner.td_name = "groupCallVideoSourceGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallVideoSourceGroupBuilder { inner }
  }

  pub fn semantics(&self) -> &String { &self.semantics }

  pub fn source_ids(&self) -> &Vec<i64> { &self.source_ids }

}

#[doc(hidden)]
pub struct RTDGroupCallVideoSourceGroupBuilder {
  inner: GroupCallVideoSourceGroup
}

impl RTDGroupCallVideoSourceGroupBuilder {
  pub fn build(&self) -> GroupCallVideoSourceGroup { self.inner.clone() }

   
  pub fn semantics<T: AsRef<str>>(&mut self, semantics: T) -> &mut Self {
    self.inner.semantics = semantics.as_ref().to_string();
    self
  }

   
  pub fn source_ids(&mut self, source_ids: Vec<i64>) -> &mut Self {
    self.inner.source_ids = source_ids;
    self
  }

}

impl AsRef<GroupCallVideoSourceGroup> for GroupCallVideoSourceGroup {
  fn as_ref(&self) -> &GroupCallVideoSourceGroup { self }
}

impl AsRef<GroupCallVideoSourceGroup> for RTDGroupCallVideoSourceGroupBuilder {
  fn as_ref(&self) -> &GroupCallVideoSourceGroup { &self.inner }
}



