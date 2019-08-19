
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns URL with the chat statistics. Currently this method can be used only for channels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatStatisticsUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getChatStatisticsUrl
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Parameters from "tg://statsrefresh?params=******" link.
  parameters: Option<String>,
  /// Pass true if a URL with the dark theme must be returned.
  is_dark: Option<bool>,
  
}



impl Object for GetChatStatisticsUrl {}
impl RObject for GetChatStatisticsUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getChatStatisticsUrl" }
  fn td_type(&self) -> RTDType { RTDType::GetChatStatisticsUrl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetChatStatisticsUrl {}


impl GetChatStatisticsUrl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getChatStatisticsUrl".to_string(),
      chat_id: None,
      parameters: None,
      is_dark: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn parameters(&self) -> Option<String> { self.parameters.clone() }
  #[doc(hidden)] pub fn _set_parameters(&mut self, parameters: String) -> &mut Self { self.parameters = Some(parameters); self }
  
  pub fn is_dark(&self) -> Option<bool> { self.is_dark.clone() }
  #[doc(hidden)] pub fn _set_is_dark(&mut self, is_dark: bool) -> &mut Self { self.is_dark = Some(is_dark); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



