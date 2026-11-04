use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains identifier of a community
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Community identifier
  id: i64,
}

impl RObject for CommunityId {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityId"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CommunityId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityIdBuilder {
    let inner = CommunityId {
      td_name: "communityId".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCommunityIdBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }
}

#[doc(hidden)]
pub struct RTDCommunityIdBuilder {
  inner: CommunityId,
}

impl RTDCommunityIdBuilder {
  pub fn build(&self) -> CommunityId {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }
}

impl AsRef<CommunityId> for CommunityId {
  fn as_ref(&self) -> &CommunityId {
    self
  }
}

impl AsRef<CommunityId> for RTDCommunityIdBuilder {
  fn as_ref(&self) -> &CommunityId {
    &self.inner
  }
}
