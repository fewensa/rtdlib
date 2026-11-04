use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a successful payment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentReceipt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the product
  product_info: ProductInfo,
  /// Point in time (Unix timestamp) when the payment was made
  date: i64,
  /// User identifier of the seller bot
  seller_bot_user_id: i64,
  /// Type of the payment receipt
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: PaymentReceiptType,
}

impl RObject for PaymentReceipt {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "paymentReceipt"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PaymentReceipt {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPaymentReceiptBuilder {
    let inner = PaymentReceipt {
      td_name: "paymentReceipt".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPaymentReceiptBuilder { inner }
  }

  pub fn product_info(&self) -> &ProductInfo {
    &self.product_info
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn seller_bot_user_id(&self) -> i64 {
    self.seller_bot_user_id
  }

  pub fn type_(&self) -> &PaymentReceiptType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDPaymentReceiptBuilder {
  inner: PaymentReceipt,
}

impl RTDPaymentReceiptBuilder {
  pub fn build(&self) -> PaymentReceipt {
    self.inner.clone()
  }

  pub fn product_info<T: AsRef<ProductInfo>>(&mut self, product_info: T) -> &mut Self {
    self.inner.product_info = product_info.as_ref().clone();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn seller_bot_user_id(&mut self, seller_bot_user_id: i64) -> &mut Self {
    self.inner.seller_bot_user_id = seller_bot_user_id;
    self
  }

  pub fn type_<T: AsRef<PaymentReceiptType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<PaymentReceipt> for PaymentReceipt {
  fn as_ref(&self) -> &PaymentReceipt {
    self
  }
}

impl AsRef<PaymentReceipt> for RTDPaymentReceiptBuilder {
  fn as_ref(&self) -> &PaymentReceipt {
    &self.inner
  }
}
