use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes settings for a business account start page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessStartPage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Title text of the start page
  title: String,
  /// Message text of the start page
  message: String,
  /// Greeting sticker of the start page; may be null if none
  sticker: Option<Sticker>,
}

impl RObject for BusinessStartPage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessStartPage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessStartPage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessStartPageBuilder {
    let inner = BusinessStartPage {
      td_name: "businessStartPage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessStartPageBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn message(&self) -> &String {
    &self.message
  }

  pub fn sticker(&self) -> &Option<Sticker> {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDBusinessStartPageBuilder {
  inner: BusinessStartPage,
}

impl RTDBusinessStartPageBuilder {
  pub fn build(&self) -> BusinessStartPage {
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

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }
}

impl AsRef<BusinessStartPage> for BusinessStartPage {
  fn as_ref(&self) -> &BusinessStartPage {
    self
  }
}

impl AsRef<BusinessStartPage> for RTDBusinessStartPageBuilder {
  fn as_ref(&self) -> &BusinessStartPage {
    &self.inner
  }
}
