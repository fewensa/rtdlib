use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a sponsored chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SponsoredChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of this result
  unique_id: i64,
  /// Chat identifier
  chat_id: i64,
  /// Additional optional information about the sponsor to be shown along with the chat
  sponsor_info: String,
  /// If non-empty, additional information about the sponsored chat to be shown along with the chat
  additional_info: String,
}

impl RObject for SponsoredChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sponsoredChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SponsoredChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSponsoredChatBuilder {
    let inner = SponsoredChat {
      td_name: "sponsoredChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSponsoredChatBuilder { inner }
  }

  pub fn unique_id(&self) -> i64 {
    self.unique_id
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn sponsor_info(&self) -> &String {
    &self.sponsor_info
  }

  pub fn additional_info(&self) -> &String {
    &self.additional_info
  }
}

#[doc(hidden)]
pub struct RTDSponsoredChatBuilder {
  inner: SponsoredChat,
}

impl RTDSponsoredChatBuilder {
  pub fn build(&self) -> SponsoredChat {
    self.inner.clone()
  }

  pub fn unique_id(&mut self, unique_id: i64) -> &mut Self {
    self.inner.unique_id = unique_id;
    self
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn sponsor_info<T: AsRef<str>>(&mut self, sponsor_info: T) -> &mut Self {
    self.inner.sponsor_info = sponsor_info.as_ref().to_string();
    self
  }

  pub fn additional_info<T: AsRef<str>>(&mut self, additional_info: T) -> &mut Self {
    self.inner.additional_info = additional_info.as_ref().to_string();
    self
  }
}

impl AsRef<SponsoredChat> for SponsoredChat {
  fn as_ref(&self) -> &SponsoredChat {
    self
  }
}

impl AsRef<SponsoredChat> for RTDSponsoredChatBuilder {
  fn as_ref(&self) -> &SponsoredChat {
    &self.inner
  }
}
