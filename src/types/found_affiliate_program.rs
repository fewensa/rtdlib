use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a found affiliate program
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundAffiliateProgram {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the bot that created the program
  bot_user_id: i64,
  /// Information about the affiliate program
  info: AffiliateProgramInfo,
}

impl RObject for FoundAffiliateProgram {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundAffiliateProgram"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundAffiliateProgram {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundAffiliateProgramBuilder {
    let inner = FoundAffiliateProgram {
      td_name: "foundAffiliateProgram".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundAffiliateProgramBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn info(&self) -> &AffiliateProgramInfo {
    &self.info
  }
}

#[doc(hidden)]
pub struct RTDFoundAffiliateProgramBuilder {
  inner: FoundAffiliateProgram,
}

impl RTDFoundAffiliateProgramBuilder {
  pub fn build(&self) -> FoundAffiliateProgram {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn info<T: AsRef<AffiliateProgramInfo>>(&mut self, info: T) -> &mut Self {
    self.inner.info = info.as_ref().clone();
    self
  }
}

impl AsRef<FoundAffiliateProgram> for FoundAffiliateProgram {
  fn as_ref(&self) -> &FoundAffiliateProgram {
    self
  }
}

impl AsRef<FoundAffiliateProgram> for RTDFoundAffiliateProgramBuilder {
  fn as_ref(&self) -> &FoundAffiliateProgram {
    &self.inner
  }
}
