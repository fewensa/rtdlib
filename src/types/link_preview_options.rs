use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Options to be used for generation of a link preview
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if link preview must be disabled
  is_disabled: bool,
  /// URL to use for link preview. If empty, then the first URL found in the message text will be used
  url: String,
  /// True, if shown media preview must be small; ignored in secret chats or if the URL isn't explicitly specified
  force_small_media: bool,
  /// True, if shown media preview must be large; ignored in secret chats or if the URL isn't explicitly specified
  force_large_media: bool,
  /// True, if link preview must be shown above message text; otherwise, the link preview will be shown below the message text; ignored in secret chats
  show_above_text: bool,
}

impl RObject for LinkPreviewOptions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewOptions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl LinkPreviewOptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewOptionsBuilder {
    let inner = LinkPreviewOptions {
      td_name: "linkPreviewOptions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewOptionsBuilder { inner }
  }

  pub fn is_disabled(&self) -> bool {
    self.is_disabled
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn force_small_media(&self) -> bool {
    self.force_small_media
  }

  pub fn force_large_media(&self) -> bool {
    self.force_large_media
  }

  pub fn show_above_text(&self) -> bool {
    self.show_above_text
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewOptionsBuilder {
  inner: LinkPreviewOptions,
}

impl RTDLinkPreviewOptionsBuilder {
  pub fn build(&self) -> LinkPreviewOptions {
    self.inner.clone()
  }

  pub fn is_disabled(&mut self, is_disabled: bool) -> &mut Self {
    self.inner.is_disabled = is_disabled;
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn force_small_media(&mut self, force_small_media: bool) -> &mut Self {
    self.inner.force_small_media = force_small_media;
    self
  }

  pub fn force_large_media(&mut self, force_large_media: bool) -> &mut Self {
    self.inner.force_large_media = force_large_media;
    self
  }

  pub fn show_above_text(&mut self, show_above_text: bool) -> &mut Self {
    self.inner.show_above_text = show_above_text;
    self
  }
}

impl AsRef<LinkPreviewOptions> for LinkPreviewOptions {
  fn as_ref(&self) -> &LinkPreviewOptions {
    self
  }
}

impl AsRef<LinkPreviewOptions> for RTDLinkPreviewOptionsBuilder {
  fn as_ref(&self) -> &LinkPreviewOptions {
    &self.inner
  }
}
