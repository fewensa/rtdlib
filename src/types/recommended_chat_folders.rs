use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of recommended chat folders
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendedChatFolders {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of recommended chat folders
  chat_folders: Vec<RecommendedChatFolder>,
}

impl RObject for RecommendedChatFolders {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "recommendedChatFolders"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl RecommendedChatFolders {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRecommendedChatFoldersBuilder {
    let inner = RecommendedChatFolders {
      td_name: "recommendedChatFolders".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRecommendedChatFoldersBuilder { inner }
  }

  pub fn chat_folders(&self) -> &Vec<RecommendedChatFolder> {
    &self.chat_folders
  }
}

#[doc(hidden)]
pub struct RTDRecommendedChatFoldersBuilder {
  inner: RecommendedChatFolders,
}

impl RTDRecommendedChatFoldersBuilder {
  pub fn build(&self) -> RecommendedChatFolders {
    self.inner.clone()
  }

  pub fn chat_folders(&mut self, chat_folders: Vec<RecommendedChatFolder>) -> &mut Self {
    self.inner.chat_folders = chat_folders;
    self
  }
}

impl AsRef<RecommendedChatFolders> for RecommendedChatFolders {
  fn as_ref(&self) -> &RecommendedChatFolders {
    self
  }
}

impl AsRef<RecommendedChatFolders> for RTDRecommendedChatFoldersBuilder {
  fn as_ref(&self) -> &RecommendedChatFolders {
    &self.inner
  }
}
