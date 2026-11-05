use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of found affiliate programs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundAffiliatePrograms {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The total number of found affiliate programs
  total_count: i64,
  /// The list of affiliate programs
  programs: Vec<FoundAffiliateProgram>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for FoundAffiliatePrograms {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundAffiliatePrograms"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundAffiliatePrograms {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundAffiliateProgramsBuilder {
    let inner = FoundAffiliatePrograms {
      td_name: "foundAffiliatePrograms".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundAffiliateProgramsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn programs(&self) -> &Vec<FoundAffiliateProgram> {
    &self.programs
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDFoundAffiliateProgramsBuilder {
  inner: FoundAffiliatePrograms,
}

impl RTDFoundAffiliateProgramsBuilder {
  pub fn build(&self) -> FoundAffiliatePrograms {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn programs(&mut self, programs: Vec<FoundAffiliateProgram>) -> &mut Self {
    self.inner.programs = programs;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<FoundAffiliatePrograms> for FoundAffiliatePrograms {
  fn as_ref(&self) -> &FoundAffiliatePrograms {
    self
  }
}

impl AsRef<FoundAffiliatePrograms> for RTDFoundAffiliateProgramsBuilder {
  fn as_ref(&self) -> &FoundAffiliatePrograms {
    &self.inner
  }
}
