use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an exception for built-in browser usage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebDomainException {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL for which the exception is done
  url: String,
  /// Domain of the URL. All URLs on the domain and subdomains of the domain are subject to the exception
  domain: String,
  /// Title of the website
  title: String,
  /// Identifier of the custom emoji with favicon of the website; may be 0 if unknown, in which case the first letter of the domain must be used
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  favicon_custom_emoji_id: isize,
}

impl RObject for WebDomainException {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "webDomainException"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WebDomainException {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWebDomainExceptionBuilder {
    let inner = WebDomainException {
      td_name: "webDomainException".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWebDomainExceptionBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn domain(&self) -> &String {
    &self.domain
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn favicon_custom_emoji_id(&self) -> isize {
    self.favicon_custom_emoji_id
  }
}

#[doc(hidden)]
pub struct RTDWebDomainExceptionBuilder {
  inner: WebDomainException,
}

impl RTDWebDomainExceptionBuilder {
  pub fn build(&self) -> WebDomainException {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn domain<T: AsRef<str>>(&mut self, domain: T) -> &mut Self {
    self.inner.domain = domain.as_ref().to_string();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn favicon_custom_emoji_id(&mut self, favicon_custom_emoji_id: isize) -> &mut Self {
    self.inner.favicon_custom_emoji_id = favicon_custom_emoji_id;
    self
  }
}

impl AsRef<WebDomainException> for WebDomainException {
  fn as_ref(&self) -> &WebDomainException {
    self
  }
}

impl AsRef<WebDomainException> for RTDWebDomainExceptionBuilder {
  fn as_ref(&self) -> &WebDomainException {
    &self.inner
  }
}
