
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the result of a callback query; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerCallbackQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // answerCallbackQuery
  /// Identifier of the callback query.
  callback_query_id: Option<i64>,
  /// Text of the answer.
  text: Option<String>,
  /// If true, an alert should be shown to the user instead of a toast notification.
  show_alert: Option<bool>,
  /// URL to be opened.
  url: Option<String>,
  /// Time during which the result of the query can be cached, in seconds.
  cache_time: Option<i32>,
  
}



impl Object for AnswerCallbackQuery {}
impl RObject for AnswerCallbackQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerCallbackQuery" }
  fn td_type(&self) -> RTDType { RTDType::AnswerCallbackQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AnswerCallbackQuery {}


impl AnswerCallbackQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "answerCallbackQuery".to_string(),
      callback_query_id: None,
      text: None,
      show_alert: None,
      url: None,
      cache_time: None,
      
    }
  }
  
  pub fn callback_query_id(&self) -> Option<i64> { self.callback_query_id.clone() }
  #[doc(hidden)] pub fn _set_callback_query_id(&mut self, callback_query_id: i64) -> &mut Self { self.callback_query_id = Some(callback_query_id); self }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn show_alert(&self) -> Option<bool> { self.show_alert.clone() }
  #[doc(hidden)] pub fn _set_show_alert(&mut self, show_alert: bool) -> &mut Self { self.show_alert = Some(show_alert); self }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn cache_time(&self) -> Option<i32> { self.cache_time.clone() }
  #[doc(hidden)] pub fn _set_cache_time(&mut self, cache_time: i32) -> &mut Self { self.cache_time = Some(cache_time); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



