
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the result of an inline query; for bots only.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerInlineQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // answerInlineQuery
  /// Identifier of the inline query.
  inline_query_id: Option<i64>,
  /// True, if the result of the query can be cached for the specified user.
  is_personal: Option<bool>,
  /// The results of the query.
  results: Option<Vec<Box<InputInlineQueryResult>>>,
  /// Allowed time to cache the results of the query, in seconds.
  cache_time: Option<i32>,
  /// Offset for the next inline query; pass an empty string if there are no more results.
  next_offset: Option<String>,
  /// If non-empty, this text should be shown on the button that opens a private chat with the bot and sends a start message to the bot with the parameter switch_pm_parameter.
  switch_pm_text: Option<String>,
  /// The parameter for the bot start message.
  switch_pm_parameter: Option<String>,
  
}


impl Clone for AnswerInlineQuery {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for AnswerInlineQuery {}
impl RObject for AnswerInlineQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerInlineQuery" }
  fn td_type(&self) -> RTDType { RTDType::AnswerInlineQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AnswerInlineQuery {}


impl AnswerInlineQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "answerInlineQuery".to_string(),
      inline_query_id: None,
      is_personal: None,
      results: None,
      cache_time: None,
      next_offset: None,
      switch_pm_text: None,
      switch_pm_parameter: None,
      
    }
  }
  
  pub fn inline_query_id(&self) -> Option<i64> { self.inline_query_id.clone() }
  #[doc(hidden)] pub fn _set_inline_query_id(&mut self, inline_query_id: i64) -> &mut Self { self.inline_query_id = Some(inline_query_id); self }
  
  pub fn is_personal(&self) -> Option<bool> { self.is_personal.clone() }
  #[doc(hidden)] pub fn _set_is_personal(&mut self, is_personal: bool) -> &mut Self { self.is_personal = Some(is_personal); self }
  
  pub fn results(&self) -> Option<Vec<Box<InputInlineQueryResult>>> { self.results.clone() }
  #[doc(hidden)] pub fn _set_results(&mut self, results: Vec<Box<InputInlineQueryResult>>) -> &mut Self { self.results = Some(results); self }
  
  pub fn cache_time(&self) -> Option<i32> { self.cache_time.clone() }
  #[doc(hidden)] pub fn _set_cache_time(&mut self, cache_time: i32) -> &mut Self { self.cache_time = Some(cache_time); self }
  
  pub fn next_offset(&self) -> Option<String> { self.next_offset.clone() }
  #[doc(hidden)] pub fn _set_next_offset(&mut self, next_offset: String) -> &mut Self { self.next_offset = Some(next_offset); self }
  
  pub fn switch_pm_text(&self) -> Option<String> { self.switch_pm_text.clone() }
  #[doc(hidden)] pub fn _set_switch_pm_text(&mut self, switch_pm_text: String) -> &mut Self { self.switch_pm_text = Some(switch_pm_text); self }
  
  pub fn switch_pm_parameter(&self) -> Option<String> { self.switch_pm_parameter.clone() }
  #[doc(hidden)] pub fn _set_switch_pm_parameter(&mut self, switch_pm_parameter: String) -> &mut Self { self.switch_pm_parameter = Some(switch_pm_parameter); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



