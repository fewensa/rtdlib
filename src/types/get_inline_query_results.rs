
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInlineQueryResults {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getInlineQueryResults
  /// The identifier of the target bot.
  bot_user_id: Option<i32>,
  /// Identifier of the chat, where the query was sent.
  chat_id: Option<i64>,
  /// Location of the user, only if needed.
  user_location: Option<Location>,
  /// Text of the query.
  query: Option<String>,
  /// Offset of the first entry to return.
  offset: Option<String>,
  
}



impl Object for GetInlineQueryResults {}
impl RObject for GetInlineQueryResults {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getInlineQueryResults" }
  fn td_type(&self) -> RTDType { RTDType::GetInlineQueryResults }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetInlineQueryResults {}


impl GetInlineQueryResults {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getInlineQueryResults".to_string(),
      bot_user_id: None,
      chat_id: None,
      user_location: None,
      query: None,
      offset: None,
      
    }
  }
  
  pub fn bot_user_id(&self) -> Option<i32> { self.bot_user_id.clone() }
  #[doc(hidden)] pub fn _set_bot_user_id(&mut self, bot_user_id: i32) -> &mut Self { self.bot_user_id = Some(bot_user_id); self }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_location(&self) -> Option<Location> { self.user_location.clone() }
  #[doc(hidden)] pub fn _set_user_location(&mut self, user_location: Location) -> &mut Self { self.user_location = Some(user_location); self }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn offset(&self) -> Option<String> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: String) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



