use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an additional payment option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Title for the payment option
  title: String,
  /// Payment form URL to be opened in a web view
  url: String,
}

impl RObject for PaymentOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PaymentOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentOptionBuilder {
    let inner = PaymentOption {
      td_name: "paymentOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentOptionBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDPaymentOptionBuilder {
  inner: PaymentOption,
}

impl RTDPaymentOptionBuilder {
  pub fn build(&self) -> PaymentOption {
    self.inner.clone()
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<PaymentOption> for PaymentOption {
  fn as_ref(&self) -> &PaymentOption {
    self
  }
}

impl AsRef<PaymentOption> for RTDPaymentOptionBuilder {
  fn as_ref(&self) -> &PaymentOption {
    &self.inner
  }
}
