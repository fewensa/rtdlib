
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents the results of the inline query. Use 
#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQueryResults {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResults
  /// Unique identifier of the inline query.
  inline_query_id: Option<i64>,
  /// The offset for the next request. If empty, there are no more results.
  next_offset: Option<String>,
  /// Results of the query.
  results: Option<Vec<Box<InlineQueryResult>>>,
  /// If non-empty, this text should be shown on the button, which opens a private chat with the bot and sends the bot a start message with the switch_pm_parameter.
  switch_pm_text: Option<String>,
  /// Parameter for the bot start message.
  switch_pm_parameter: Option<String>,
  
}


impl Clone for InlineQueryResults {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InlineQueryResults {}
impl RObject for InlineQueryResults {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResults" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResults }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl InlineQueryResults {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResults".to_string(),
      inline_query_id: None,
      next_offset: None,
      results: None,
      switch_pm_text: None,
      switch_pm_parameter: None,
      
    }
  }
  
  pub fn inline_query_id(&self) -> Option<i64> { self.inline_query_id.clone() }
  #[doc(hidden)] pub fn _set_inline_query_id(&mut self, inline_query_id: i64) -> &mut Self { self.inline_query_id = Some(inline_query_id); self }
  
  pub fn next_offset(&self) -> Option<String> { self.next_offset.clone() }
  #[doc(hidden)] pub fn _set_next_offset(&mut self, next_offset: String) -> &mut Self { self.next_offset = Some(next_offset); self }
  
  pub fn results(&self) -> Option<Vec<Box<InlineQueryResult>>> { self.results.clone() }
  #[doc(hidden)] pub fn _set_results(&mut self, results: Vec<Box<InlineQueryResult>>) -> &mut Self { self.results = Some(results); self }
  
  pub fn switch_pm_text(&self) -> Option<String> { self.switch_pm_text.clone() }
  #[doc(hidden)] pub fn _set_switch_pm_text(&mut self, switch_pm_text: String) -> &mut Self { self.switch_pm_text = Some(switch_pm_text); self }
  
  pub fn switch_pm_parameter(&self) -> Option<String> { self.switch_pm_parameter.clone() }
  #[doc(hidden)] pub fn _set_switch_pm_parameter(&mut self, switch_pm_parameter: String) -> &mut Self { self.switch_pm_parameter = Some(switch_pm_parameter); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



