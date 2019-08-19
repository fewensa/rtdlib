
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Answers a custom query; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerCustomQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // answerCustomQuery
  /// Identifier of a custom query.
  custom_query_id: Option<i64>,
  /// JSON-serialized answer to the query.
  data: Option<String>,
  
}



impl Object for AnswerCustomQuery {}
impl RObject for AnswerCustomQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "answerCustomQuery" }
  fn td_type(&self) -> RTDType { RTDType::AnswerCustomQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AnswerCustomQuery {}


impl AnswerCustomQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "answerCustomQuery".to_string(),
      custom_query_id: None,
      data: None,
      
    }
  }
  
  pub fn custom_query_id(&self) -> Option<i64> { self.custom_query_id.clone() }
  #[doc(hidden)] pub fn _set_custom_query_id(&mut self, custom_query_id: i64) -> &mut Self { self.custom_query_id = Some(custom_query_id); self }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



