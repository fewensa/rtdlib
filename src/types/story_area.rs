use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a clickable rectangle area on a story media
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryArea {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Position of the area
  position: AreaPosition,
  /// Type of the area
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: StoryAreaType,
}

impl RObject for StoryArea {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyArea"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryArea {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAreaBuilder {
    let inner = StoryArea {
      td_name: "storyArea".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAreaBuilder { inner }
  }

  pub fn position(&self) -> &AreaPosition {
    &self.position
  }

  pub fn type_(&self) -> &StoryAreaType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDStoryAreaBuilder {
  inner: StoryArea,
}

impl RTDStoryAreaBuilder {
  pub fn build(&self) -> StoryArea {
    self.inner.clone()
  }

  pub fn position<T: AsRef<AreaPosition>>(&mut self, position: T) -> &mut Self {
    self.inner.position = position.as_ref().clone();
    self
  }

  pub fn type_<T: AsRef<StoryAreaType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<StoryArea> for StoryArea {
  fn as_ref(&self) -> &StoryArea {
    self
  }
}

impl AsRef<StoryArea> for RTDStoryAreaBuilder {
  fn as_ref(&self) -> &StoryArea {
    &self.inner
  }
}
