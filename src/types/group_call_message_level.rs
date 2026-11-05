use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a level of features for a message sent in a live story group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallMessageLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The minimum number of Telegram Stars required to get features of the level
  min_star_count: i64,
  /// The amount of time the message of this level will be pinned, in seconds
  pin_duration: i64,
  /// The maximum allowed length of the message text
  max_text_length: i64,
  /// The maximum allowed number of custom emoji in the message text
  max_custom_emoji_count: i64,
  /// The first color used to show the message text in the RGB format
  first_color: i64,
  /// The second color used to show the message text in the RGB format
  second_color: i64,
  /// Background color for the message in the RGB format
  background_color: i64,
}

impl RObject for GroupCallMessageLevel {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "groupCallMessageLevel"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GroupCallMessageLevel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGroupCallMessageLevelBuilder {
    let inner = GroupCallMessageLevel {
      td_name: "groupCallMessageLevel".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGroupCallMessageLevelBuilder { inner }
  }

  pub fn min_star_count(&self) -> i64 {
    self.min_star_count
  }

  pub fn pin_duration(&self) -> i64 {
    self.pin_duration
  }

  pub fn max_text_length(&self) -> i64 {
    self.max_text_length
  }

  pub fn max_custom_emoji_count(&self) -> i64 {
    self.max_custom_emoji_count
  }

  pub fn first_color(&self) -> i64 {
    self.first_color
  }

  pub fn second_color(&self) -> i64 {
    self.second_color
  }

  pub fn background_color(&self) -> i64 {
    self.background_color
  }
}

#[doc(hidden)]
pub struct RTDGroupCallMessageLevelBuilder {
  inner: GroupCallMessageLevel,
}

impl RTDGroupCallMessageLevelBuilder {
  pub fn build(&self) -> GroupCallMessageLevel {
    self.inner.clone()
  }

  pub fn min_star_count(&mut self, min_star_count: i64) -> &mut Self {
    self.inner.min_star_count = min_star_count;
    self
  }

  pub fn pin_duration(&mut self, pin_duration: i64) -> &mut Self {
    self.inner.pin_duration = pin_duration;
    self
  }

  pub fn max_text_length(&mut self, max_text_length: i64) -> &mut Self {
    self.inner.max_text_length = max_text_length;
    self
  }

  pub fn max_custom_emoji_count(&mut self, max_custom_emoji_count: i64) -> &mut Self {
    self.inner.max_custom_emoji_count = max_custom_emoji_count;
    self
  }

  pub fn first_color(&mut self, first_color: i64) -> &mut Self {
    self.inner.first_color = first_color;
    self
  }

  pub fn second_color(&mut self, second_color: i64) -> &mut Self {
    self.inner.second_color = second_color;
    self
  }

  pub fn background_color(&mut self, background_color: i64) -> &mut Self {
    self.inner.background_color = background_color;
    self
  }
}

impl AsRef<GroupCallMessageLevel> for GroupCallMessageLevel {
  fn as_ref(&self) -> &GroupCallMessageLevel {
    self
  }
}

impl AsRef<GroupCallMessageLevel> for RTDGroupCallMessageLevelBuilder {
  fn as_ref(&self) -> &GroupCallMessageLevel {
    &self.inner
  }
}
