
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a bot's answer to a callback query. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQueryAnswer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callbackQueryAnswer
  /// Text of the answer.
  text: Option<String>,
  /// True, if an alert should be shown to the user instead of a toast notification.
  show_alert: Option<bool>,
  /// URL to be opened.
  url: Option<String>,
  
}



impl Object for CallbackQueryAnswer {}
impl RObject for CallbackQueryAnswer {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callbackQueryAnswer" }
  fn td_type(&self) -> RTDType { RTDType::CallbackQueryAnswer }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl CallbackQueryAnswer {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callbackQueryAnswer".to_string(),
      text: None,
      show_alert: None,
      url: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn show_alert(&self) -> Option<bool> { self.show_alert.clone() }
  #[doc(hidden)] pub fn _set_show_alert(&mut self, show_alert: bool) -> &mut Self { self.show_alert = Some(show_alert); self }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



