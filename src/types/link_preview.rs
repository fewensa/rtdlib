use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a link preview
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreview {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Original URL of the link
  url: String,
  /// URL to display
  display_url: String,
  /// Short name of the website (e.g., Google Docs, App Store)
  site_name: String,
  /// Title of the content
  title: String,
  /// Describes a link preview
  description: FormattedText,
  /// Author of the content
  author: String,
  /// Type of the link preview
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: LinkPreviewType,
  /// True, if size of media in the preview can be changed
  has_large_media: bool,
  /// True, if large media preview must be shown; otherwise, the media preview must be shown small and only the first frame must be shown for videos
  show_large_media: bool,
  /// True, if media must be shown above link preview description; otherwise, the media must be shown below the description
  show_media_above_description: bool,
  /// True, if there is no need to show an ordinary open URL confirmation, when opening the URL from the preview, because the URL is shown in the message text in clear
  skip_confirmation: bool,
  /// True, if the link preview must be shown above message text; otherwise, the link preview must be shown below the message text
  show_above_text: bool,
  /// Version of instant view (currently, can be 1 or 2) for the web page; 0 if none
  instant_view_version: i64,
}

impl RObject for LinkPreview {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreview"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl LinkPreview {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewBuilder {
    let inner = LinkPreview {
      td_name: "linkPreview".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn display_url(&self) -> &String {
    &self.display_url
  }

  pub fn site_name(&self) -> &String {
    &self.site_name
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn description(&self) -> &FormattedText {
    &self.description
  }

  pub fn author(&self) -> &String {
    &self.author
  }

  pub fn type_(&self) -> &LinkPreviewType {
    &self.type_
  }

  pub fn has_large_media(&self) -> bool {
    self.has_large_media
  }

  pub fn show_large_media(&self) -> bool {
    self.show_large_media
  }

  pub fn show_media_above_description(&self) -> bool {
    self.show_media_above_description
  }

  pub fn skip_confirmation(&self) -> bool {
    self.skip_confirmation
  }

  pub fn show_above_text(&self) -> bool {
    self.show_above_text
  }

  pub fn instant_view_version(&self) -> i64 {
    self.instant_view_version
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewBuilder {
  inner: LinkPreview,
}

impl RTDLinkPreviewBuilder {
  pub fn build(&self) -> LinkPreview {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn display_url<T: AsRef<str>>(&mut self, display_url: T) -> &mut Self {
    self.inner.display_url = display_url.as_ref().to_string();
    self
  }

  pub fn site_name<T: AsRef<str>>(&mut self, site_name: T) -> &mut Self {
    self.inner.site_name = site_name.as_ref().to_string();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn description<T: AsRef<FormattedText>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().clone();
    self
  }

  pub fn author<T: AsRef<str>>(&mut self, author: T) -> &mut Self {
    self.inner.author = author.as_ref().to_string();
    self
  }

  pub fn type_<T: AsRef<LinkPreviewType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn has_large_media(&mut self, has_large_media: bool) -> &mut Self {
    self.inner.has_large_media = has_large_media;
    self
  }

  pub fn show_large_media(&mut self, show_large_media: bool) -> &mut Self {
    self.inner.show_large_media = show_large_media;
    self
  }

  pub fn show_media_above_description(&mut self, show_media_above_description: bool) -> &mut Self {
    self.inner.show_media_above_description = show_media_above_description;
    self
  }

  pub fn skip_confirmation(&mut self, skip_confirmation: bool) -> &mut Self {
    self.inner.skip_confirmation = skip_confirmation;
    self
  }

  pub fn show_above_text(&mut self, show_above_text: bool) -> &mut Self {
    self.inner.show_above_text = show_above_text;
    self
  }

  pub fn instant_view_version(&mut self, instant_view_version: i64) -> &mut Self {
    self.inner.instant_view_version = instant_view_version;
    self
  }
}

impl AsRef<LinkPreview> for LinkPreview {
  fn as_ref(&self) -> &LinkPreview {
    self
  }
}

impl AsRef<LinkPreview> for RTDLinkPreviewBuilder {
  fn as_ref(&self) -> &LinkPreview {
    &self.inner
  }
}
