
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Theme colors for a payment form
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentFormTheme {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A color of the payment form background in the RGB24 format
  background_color: i64,
  /// A color of text in the RGB24 format
  text_color: i64,
  /// A color of hints in the RGB24 format
  hint_color: i64,
  /// A color of links in the RGB24 format
  link_color: i64,
  /// A color of the buttons in the RGB24 format
  button_color: i64,
  /// A color of text on the buttons in the RGB24 format
  button_text_color: i64,
  
}

impl RObject for PaymentFormTheme {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "paymentFormTheme" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PaymentFormTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPaymentFormThemeBuilder {
    let mut inner = PaymentFormTheme::default();
    inner.td_name = "paymentFormTheme".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPaymentFormThemeBuilder { inner }
  }

  pub fn background_color(&self) -> i64 { self.background_color }

  pub fn text_color(&self) -> i64 { self.text_color }

  pub fn hint_color(&self) -> i64 { self.hint_color }

  pub fn link_color(&self) -> i64 { self.link_color }

  pub fn button_color(&self) -> i64 { self.button_color }

  pub fn button_text_color(&self) -> i64 { self.button_text_color }

}

#[doc(hidden)]
pub struct RTDPaymentFormThemeBuilder {
  inner: PaymentFormTheme
}

impl RTDPaymentFormThemeBuilder {
  pub fn build(&self) -> PaymentFormTheme { self.inner.clone() }

   
  pub fn background_color(&mut self, background_color: i64) -> &mut Self {
    self.inner.background_color = background_color;
    self
  }

   
  pub fn text_color(&mut self, text_color: i64) -> &mut Self {
    self.inner.text_color = text_color;
    self
  }

   
  pub fn hint_color(&mut self, hint_color: i64) -> &mut Self {
    self.inner.hint_color = hint_color;
    self
  }

   
  pub fn link_color(&mut self, link_color: i64) -> &mut Self {
    self.inner.link_color = link_color;
    self
  }

   
  pub fn button_color(&mut self, button_color: i64) -> &mut Self {
    self.inner.button_color = button_color;
    self
  }

   
  pub fn button_text_color(&mut self, button_text_color: i64) -> &mut Self {
    self.inner.button_text_color = button_text_color;
    self
  }

}

impl AsRef<PaymentFormTheme> for PaymentFormTheme {
  fn as_ref(&self) -> &PaymentFormTheme { self }
}

impl AsRef<PaymentFormTheme> for RTDPaymentFormThemeBuilder {
  fn as_ref(&self) -> &PaymentFormTheme { &self.inner }
}



