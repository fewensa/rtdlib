use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a clickable rectangle area on a story media to be added
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryArea {
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
  type_: InputStoryAreaType,
}

impl RObject for InputStoryArea {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryArea"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputStoryArea {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreaBuilder {
    let inner = InputStoryArea {
      td_name: "inputStoryArea".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreaBuilder { inner }
  }

  pub fn position(&self) -> &AreaPosition {
    &self.position
  }

  pub fn type_(&self) -> &InputStoryAreaType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDInputStoryAreaBuilder {
  inner: InputStoryArea,
}

impl RTDInputStoryAreaBuilder {
  pub fn build(&self) -> InputStoryArea {
    self.inner.clone()
  }

  pub fn position<T: AsRef<AreaPosition>>(&mut self, position: T) -> &mut Self {
    self.inner.position = position.as_ref().clone();
    self
  }

  pub fn type_<T: AsRef<InputStoryAreaType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<InputStoryArea> for InputStoryArea {
  fn as_ref(&self) -> &InputStoryArea {
    self
  }
}

impl AsRef<InputStoryArea> for RTDInputStoryAreaBuilder {
  fn as_ref(&self) -> &InputStoryArea {
    &self.inner
  }
}
