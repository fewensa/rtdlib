use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an invoice payment form
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The payment form identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Type of the payment form
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: PaymentFormType,
  /// User identifier of the seller bot
  seller_bot_user_id: i64,
  /// Information about the product
  product_info: ProductInfo,
}

impl RObject for PaymentForm {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentForm"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PaymentForm {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentFormBuilder {
    let inner = PaymentForm {
      td_name: "paymentForm".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentFormBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn type_(&self) -> &PaymentFormType {
    &self.type_
  }

  pub fn seller_bot_user_id(&self) -> i64 {
    self.seller_bot_user_id
  }

  pub fn product_info(&self) -> &ProductInfo {
    &self.product_info
  }
}

#[doc(hidden)]
pub struct RTDPaymentFormBuilder {
  inner: PaymentForm,
}

impl RTDPaymentFormBuilder {
  pub fn build(&self) -> PaymentForm {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn type_<T: AsRef<PaymentFormType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn seller_bot_user_id(&mut self, seller_bot_user_id: i64) -> &mut Self {
    self.inner.seller_bot_user_id = seller_bot_user_id;
    self
  }

  pub fn product_info<T: AsRef<ProductInfo>>(&mut self, product_info: T) -> &mut Self {
    self.inner.product_info = product_info.as_ref().clone();
    self
  }
}

impl AsRef<PaymentForm> for PaymentForm {
  fn as_ref(&self) -> &PaymentForm {
    self
  }
}

impl AsRef<PaymentForm> for RTDPaymentFormBuilder {
  fn as_ref(&self) -> &PaymentForm {
    &self.inner
  }
}
