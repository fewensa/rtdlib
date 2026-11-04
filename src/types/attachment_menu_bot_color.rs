use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a color to highlight a bot added to attachment menu
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentMenuBotColor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Color in the RGB format for light themes
  light_color: i64,
  /// Color in the RGB format for dark themes
  dark_color: i64,
}

impl RObject for AttachmentMenuBotColor {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "attachmentMenuBotColor"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AttachmentMenuBotColor {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAttachmentMenuBotColorBuilder {
    let inner = AttachmentMenuBotColor {
      td_name: "attachmentMenuBotColor".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAttachmentMenuBotColorBuilder { inner }
  }

  pub fn light_color(&self) -> i64 {
    self.light_color
  }

  pub fn dark_color(&self) -> i64 {
    self.dark_color
  }
}

#[doc(hidden)]
pub struct RTDAttachmentMenuBotColorBuilder {
  inner: AttachmentMenuBotColor,
}

impl RTDAttachmentMenuBotColorBuilder {
  pub fn build(&self) -> AttachmentMenuBotColor {
    self.inner.clone()
  }

  pub fn light_color(&mut self, light_color: i64) -> &mut Self {
    self.inner.light_color = light_color;
    self
  }

  pub fn dark_color(&mut self, dark_color: i64) -> &mut Self {
    self.inner.dark_color = dark_color;
    self
  }
}

impl AsRef<AttachmentMenuBotColor> for AttachmentMenuBotColor {
  fn as_ref(&self) -> &AttachmentMenuBotColor {
    self
  }
}

impl AsRef<AttachmentMenuBotColor> for RTDAttachmentMenuBotColorBuilder {
  fn as_ref(&self) -> &AttachmentMenuBotColor {
    &self.inner
  }
}
