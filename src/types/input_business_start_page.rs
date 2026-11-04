use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes settings for a business account start page to set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputBusinessStartPage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Title text of the start page; 0-getOption("business_start_page_title_length_max") characters
  title: String,
  /// Message text of the start page; 0-getOption("business_start_page_message_length_max") characters
  message: String,
  /// Greeting sticker of the start page; pass null if none. The sticker must belong to a sticker set and must not be a custom emoji
  sticker: InputFile,
}

impl RObject for InputBusinessStartPage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputBusinessStartPage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputBusinessStartPage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputBusinessStartPageBuilder {
    let inner = InputBusinessStartPage {
      td_name: "inputBusinessStartPage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputBusinessStartPageBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn message(&self) -> &String {
    &self.message
  }

  pub fn sticker(&self) -> &InputFile {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDInputBusinessStartPageBuilder {
  inner: InputBusinessStartPage,
}

impl RTDInputBusinessStartPageBuilder {
  pub fn build(&self) -> InputBusinessStartPage {
    self.inner.clone()
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn message<T: AsRef<str>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().to_string();
    self
  }

  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }
}

impl AsRef<InputBusinessStartPage> for InputBusinessStartPage {
  fn as_ref(&self) -> &InputBusinessStartPage {
    self
  }
}

impl AsRef<InputBusinessStartPage> for RTDInputBusinessStartPageBuilder {
  fn as_ref(&self) -> &InputBusinessStartPage {
    &self.inner
  }
}
