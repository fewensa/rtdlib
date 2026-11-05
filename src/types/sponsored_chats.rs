use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of sponsored chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SponsoredChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of sponsored chats
  chats: Vec<SponsoredChat>,
}

impl RObject for SponsoredChats {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sponsoredChats"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SponsoredChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSponsoredChatsBuilder {
    let inner = SponsoredChats {
      td_name: "sponsoredChats".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSponsoredChatsBuilder { inner }
  }

  pub fn chats(&self) -> &Vec<SponsoredChat> {
    &self.chats
  }
}

#[doc(hidden)]
pub struct RTDSponsoredChatsBuilder {
  inner: SponsoredChats,
}

impl RTDSponsoredChatsBuilder {
  pub fn build(&self) -> SponsoredChats {
    self.inner.clone()
  }

  pub fn chats(&mut self, chats: Vec<SponsoredChat>) -> &mut Self {
    self.inner.chats = chats;
    self
  }
}

impl AsRef<SponsoredChats> for SponsoredChats {
  fn as_ref(&self) -> &SponsoredChats {
    self
  }
}

impl AsRef<SponsoredChats> for RTDSponsoredChatsBuilder {
  fn as_ref(&self) -> &SponsoredChats {
    &self.inner
  }
}
