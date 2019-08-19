
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains Telegram terms of service. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermsOfService {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // termsOfService
  /// Text of the terms of service.
  text: Option<FormattedText>,
  /// Mininum age of a user to be able to accept the terms; 0 if any.
  min_user_age: Option<i32>,
  /// True, if a blocking popup with terms of service must be shown to the user.
  show_popup: Option<bool>,
  
}



impl Object for TermsOfService {}
impl RObject for TermsOfService {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "termsOfService" }
  fn td_type(&self) -> RTDType { RTDType::TermsOfService }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TermsOfService {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "termsOfService".to_string(),
      text: None,
      min_user_age: None,
      show_popup: None,
      
    }
  }
  
  pub fn text(&self) -> Option<FormattedText> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: FormattedText) -> &mut Self { self.text = Some(text); self }
  
  pub fn min_user_age(&self) -> Option<i32> { self.min_user_age.clone() }
  #[doc(hidden)] pub fn _set_min_user_age(&mut self, min_user_age: i32) -> &mut Self { self.min_user_age = Some(min_user_age); self }
  
  pub fn show_popup(&self) -> Option<bool> { self.show_popup.clone() }
  #[doc(hidden)] pub fn _set_show_popup(&mut self, show_popup: bool) -> &mut Self { self.show_popup = Some(show_popup); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



