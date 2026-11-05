use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of affiliate programs that were connected to an affiliate
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectedAffiliatePrograms {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The total number of affiliate programs that were connected to the affiliate
  total_count: i64,
  /// The list of connected affiliate programs
  programs: Vec<ConnectedAffiliateProgram>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for ConnectedAffiliatePrograms {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "connectedAffiliatePrograms"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ConnectedAffiliatePrograms {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDConnectedAffiliateProgramsBuilder {
    let inner = ConnectedAffiliatePrograms {
      td_name: "connectedAffiliatePrograms".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDConnectedAffiliateProgramsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn programs(&self) -> &Vec<ConnectedAffiliateProgram> {
    &self.programs
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDConnectedAffiliateProgramsBuilder {
  inner: ConnectedAffiliatePrograms,
}

impl RTDConnectedAffiliateProgramsBuilder {
  pub fn build(&self) -> ConnectedAffiliatePrograms {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn programs(&mut self, programs: Vec<ConnectedAffiliateProgram>) -> &mut Self {
    self.inner.programs = programs;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<ConnectedAffiliatePrograms> for ConnectedAffiliatePrograms {
  fn as_ref(&self) -> &ConnectedAffiliatePrograms {
    self
  }
}

impl AsRef<ConnectedAffiliatePrograms> for RTDConnectedAffiliateProgramsBuilder {
  fn as_ref(&self) -> &ConnectedAffiliatePrograms {
    &self.inner
  }
}
