use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about restrictions that must be applied to a chat or a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RestrictionInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A human-readable description of the reason why access to the content must be restricted. If empty, then the content can be accessed, but may be covered by hidden with 18+ spoiler anyway
  restriction_reason: String,
  /// True, if media content of the messages must be hidden with 18+ spoiler. Use value of the option "can_ignore_sensitive_content_restrictions" to check whether the current user can ignore the restriction. If age verification parameters were received in updateAgeVerificationParameters, then the user must complete age verification to ignore the restriction. Set the option "ignore_sensitive_content_restrictions" to true if the user passes age verification
  has_sensitive_content: bool,
}

impl RObject for RestrictionInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "restrictionInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl RestrictionInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRestrictionInfoBuilder {
    let inner = RestrictionInfo {
      td_name: "restrictionInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRestrictionInfoBuilder { inner }
  }

  pub fn restriction_reason(&self) -> &String {
    &self.restriction_reason
  }

  pub fn has_sensitive_content(&self) -> bool {
    self.has_sensitive_content
  }
}

#[doc(hidden)]
pub struct RTDRestrictionInfoBuilder {
  inner: RestrictionInfo,
}

impl RTDRestrictionInfoBuilder {
  pub fn build(&self) -> RestrictionInfo {
    self.inner.clone()
  }

  pub fn restriction_reason<T: AsRef<str>>(&mut self, restriction_reason: T) -> &mut Self {
    self.inner.restriction_reason = restriction_reason.as_ref().to_string();
    self
  }

  pub fn has_sensitive_content(&mut self, has_sensitive_content: bool) -> &mut Self {
    self.inner.has_sensitive_content = has_sensitive_content;
    self
  }
}

impl AsRef<RestrictionInfo> for RestrictionInfo {
  fn as_ref(&self) -> &RestrictionInfo {
    self
  }
}

impl AsRef<RestrictionInfo> for RTDRestrictionInfoBuilder {
  fn as_ref(&self) -> &RestrictionInfo {
    &self.inner
  }
}
