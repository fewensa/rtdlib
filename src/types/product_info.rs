use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a product that can be paid with invoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Product title
  title: String,
  /// Contains information about a product that can be paid with invoice
  description: FormattedText,
  /// Product photo; may be null
  photo: Option<Photo>,
}

impl RObject for ProductInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "productInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ProductInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProductInfoBuilder {
    let inner = ProductInfo {
      td_name: "productInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDProductInfoBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn description(&self) -> &FormattedText {
    &self.description
  }

  pub fn photo(&self) -> &Option<Photo> {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDProductInfoBuilder {
  inner: ProductInfo,
}

impl RTDProductInfoBuilder {
  pub fn build(&self) -> ProductInfo {
    self.inner.clone()
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn description<T: AsRef<FormattedText>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().clone();
    self
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }
}

impl AsRef<ProductInfo> for ProductInfo {
  fn as_ref(&self) -> &ProductInfo {
    self
  }
}

impl AsRef<ProductInfo> for RTDProductInfoBuilder {
  fn as_ref(&self) -> &ProductInfo {
    &self.inner
  }
}
