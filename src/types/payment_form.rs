
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about an invoice payment form. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentForm {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // paymentForm
  /// Full information of the invoice.
  invoice: Option<Invoice>,
  /// Payment form URL.
  url: Option<String>,
  /// Contains information about the payment provider, if available, to support it natively without the need for opening the URL; may be null.
  payments_provider: Option<PaymentsProviderStripe>,
  /// Saved server-side order information; may be null.
  saved_order_info: Option<OrderInfo>,
  /// Contains information about saved card credentials; may be null.
  saved_credentials: Option<SavedCredentials>,
  /// True, if the user can choose to save credentials.
  can_save_credentials: Option<bool>,
  /// True, if the user will be able to save credentials protected by a password they set up.
  need_password: Option<bool>,
  
}



impl Object for PaymentForm {}
impl RObject for PaymentForm {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "paymentForm" }
  fn td_type(&self) -> RTDType { RTDType::PaymentForm }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PaymentForm {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "paymentForm".to_string(),
      invoice: None,
      url: None,
      payments_provider: None,
      saved_order_info: None,
      saved_credentials: None,
      can_save_credentials: None,
      need_password: None,
      
    }
  }
  
  pub fn invoice(&self) -> Option<Invoice> { self.invoice.clone() }
  #[doc(hidden)] pub fn _set_invoice(&mut self, invoice: Invoice) -> &mut Self { self.invoice = Some(invoice); self }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn payments_provider(&self) -> Option<PaymentsProviderStripe> { self.payments_provider.clone() }
  #[doc(hidden)] pub fn _set_payments_provider(&mut self, payments_provider: PaymentsProviderStripe) -> &mut Self { self.payments_provider = Some(payments_provider); self }
  
  pub fn saved_order_info(&self) -> Option<OrderInfo> { self.saved_order_info.clone() }
  #[doc(hidden)] pub fn _set_saved_order_info(&mut self, saved_order_info: OrderInfo) -> &mut Self { self.saved_order_info = Some(saved_order_info); self }
  
  pub fn saved_credentials(&self) -> Option<SavedCredentials> { self.saved_credentials.clone() }
  #[doc(hidden)] pub fn _set_saved_credentials(&mut self, saved_credentials: SavedCredentials) -> &mut Self { self.saved_credentials = Some(saved_credentials); self }
  
  pub fn can_save_credentials(&self) -> Option<bool> { self.can_save_credentials.clone() }
  #[doc(hidden)] pub fn _set_can_save_credentials(&mut self, can_save_credentials: bool) -> &mut Self { self.can_save_credentials = Some(can_save_credentials); self }
  
  pub fn need_password(&self) -> Option<bool> { self.need_password.clone() }
  #[doc(hidden)] pub fn _set_need_password(&mut self, need_password: bool) -> &mut Self { self.need_password = Some(need_password); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



