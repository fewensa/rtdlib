use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a recommended chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendedChatFolder {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chat folder
  folder: ChatFolder,
  /// Describes a recommended chat folder
  description: String,
}

impl RObject for RecommendedChatFolder {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "recommendedChatFolder"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl RecommendedChatFolder {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRecommendedChatFolderBuilder {
    let inner = RecommendedChatFolder {
      td_name: "recommendedChatFolder".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRecommendedChatFolderBuilder { inner }
  }

  pub fn folder(&self) -> &ChatFolder {
    &self.folder
  }

  pub fn description(&self) -> &String {
    &self.description
  }
}

#[doc(hidden)]
pub struct RTDRecommendedChatFolderBuilder {
  inner: RecommendedChatFolder,
}

impl RTDRecommendedChatFolderBuilder {
  pub fn build(&self) -> RecommendedChatFolder {
    self.inner.clone()
  }

  pub fn folder<T: AsRef<ChatFolder>>(&mut self, folder: T) -> &mut Self {
    self.inner.folder = folder.as_ref().clone();
    self
  }

  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }
}

impl AsRef<RecommendedChatFolder> for RecommendedChatFolder {
  fn as_ref(&self) -> &RecommendedChatFolder {
    self
  }
}

impl AsRef<RecommendedChatFolder> for RTDRecommendedChatFolderBuilder {
  fn as_ref(&self) -> &RecommendedChatFolder {
    &self.inner
  }
}
