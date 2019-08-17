
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with an invoice; can be used only by bots and only in private chats. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessageInvoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageInvoice
  /// Invoice.
  invoice: Option<Invoice>,
  /// Product title; 1-32 characters.
  title: Option<String>,
  /// Product description; 0-255 characters.
  description: Option<String>,
  /// Product photo URL; optional.
  photo_url: Option<String>,
  /// Product photo size.
  photo_size: Option<i32>,
  /// Product photo width.
  photo_width: Option<i32>,
  /// Product photo height.
  photo_height: Option<i32>,
  /// The invoice payload.
  payload: Option<String>,
  /// Payment provider token.
  provider_token: Option<String>,
  /// JSON-encoded data about the invoice, which will be shared with the payment provider.
  provider_data: Option<String>,
  /// Unique invoice bot start_parameter for the generation of this invoice.
  start_parameter: Option<String>,
  
}



impl Object for InputMessageInvoice {}
impl RObject for InputMessageInvoice {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageInvoice" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageInvoice }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageInvoice {}


impl InputMessageInvoice {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageInvoice".to_string(),
      invoice: None,
      title: None,
      description: None,
      photo_url: None,
      photo_size: None,
      photo_width: None,
      photo_height: None,
      payload: None,
      provider_token: None,
      provider_data: None,
      start_parameter: None,
      
    }
  }
  
  pub fn invoice(&self) -> Option<Invoice> { self.invoice.clone() }
  #[doc(hidden)] pub fn _set_invoice(&mut self, invoice: Invoice) -> &mut Self { self.invoice = Some(invoice); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn photo_url(&self) -> Option<String> { self.photo_url.clone() }
  #[doc(hidden)] pub fn _set_photo_url(&mut self, photo_url: String) -> &mut Self { self.photo_url = Some(photo_url); self }
  
  pub fn photo_size(&self) -> Option<i32> { self.photo_size.clone() }
  #[doc(hidden)] pub fn _set_photo_size(&mut self, photo_size: i32) -> &mut Self { self.photo_size = Some(photo_size); self }
  
  pub fn photo_width(&self) -> Option<i32> { self.photo_width.clone() }
  #[doc(hidden)] pub fn _set_photo_width(&mut self, photo_width: i32) -> &mut Self { self.photo_width = Some(photo_width); self }
  
  pub fn photo_height(&self) -> Option<i32> { self.photo_height.clone() }
  #[doc(hidden)] pub fn _set_photo_height(&mut self, photo_height: i32) -> &mut Self { self.photo_height = Some(photo_height); self }
  
  pub fn payload(&self) -> Option<String> { self.payload.clone() }
  #[doc(hidden)] pub fn _set_payload(&mut self, payload: String) -> &mut Self { self.payload = Some(payload); self }
  
  pub fn provider_token(&self) -> Option<String> { self.provider_token.clone() }
  #[doc(hidden)] pub fn _set_provider_token(&mut self, provider_token: String) -> &mut Self { self.provider_token = Some(provider_token); self }
  
  pub fn provider_data(&self) -> Option<String> { self.provider_data.clone() }
  #[doc(hidden)] pub fn _set_provider_data(&mut self, provider_data: String) -> &mut Self { self.provider_data = Some(provider_data); self }
  
  pub fn start_parameter(&self) -> Option<String> { self.start_parameter.clone() }
  #[doc(hidden)] pub fn _set_start_parameter(&mut self, start_parameter: String) -> &mut Self { self.start_parameter = Some(start_parameter); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



