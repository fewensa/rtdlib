
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInlineQueryResultMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendInlineQueryResultMessage
  /// Target chat.
  chat_id: Option<i64>,
  /// Identifier of a message to reply to or 0.
  reply_to_message_id: Option<i64>,
  /// Pass true to disable notification for the message. Not supported in secret chats.
  disable_notification: Option<bool>,
  /// Pass true if the message is sent from background.
  from_background: Option<bool>,
  /// Identifier of the inline query.
  query_id: Option<i64>,
  /// Identifier of the inline result.
  result_id: Option<String>,
  /// If true, there will be no mention of a bot, via which the message is sent. Can be used only for bots GetOption("animation_search_bot_username"), GetOption("photo_search_bot_username") and GetOption("venue_search_bot_username").
  hide_via_bot: Option<bool>,
  
}



impl Object for SendInlineQueryResultMessage {}
impl RObject for SendInlineQueryResultMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendInlineQueryResultMessage" }
  fn td_type(&self) -> RTDType { RTDType::SendInlineQueryResultMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendInlineQueryResultMessage {}


impl SendInlineQueryResultMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendInlineQueryResultMessage".to_string(),
      chat_id: None,
      reply_to_message_id: None,
      disable_notification: None,
      from_background: None,
      query_id: None,
      result_id: None,
      hide_via_bot: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn reply_to_message_id(&self) -> Option<i64> { self.reply_to_message_id.clone() }
  #[doc(hidden)] pub fn _set_reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self { self.reply_to_message_id = Some(reply_to_message_id); self }
  
  pub fn disable_notification(&self) -> Option<bool> { self.disable_notification.clone() }
  #[doc(hidden)] pub fn _set_disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  
  pub fn from_background(&self) -> Option<bool> { self.from_background.clone() }
  #[doc(hidden)] pub fn _set_from_background(&mut self, from_background: bool) -> &mut Self { self.from_background = Some(from_background); self }
  
  pub fn query_id(&self) -> Option<i64> { self.query_id.clone() }
  #[doc(hidden)] pub fn _set_query_id(&mut self, query_id: i64) -> &mut Self { self.query_id = Some(query_id); self }
  
  pub fn result_id(&self) -> Option<String> { self.result_id.clone() }
  #[doc(hidden)] pub fn _set_result_id(&mut self, result_id: String) -> &mut Self { self.result_id = Some(result_id); self }
  
  pub fn hide_via_bot(&self) -> Option<bool> { self.hide_via_bot.clone() }
  #[doc(hidden)] pub fn _set_hide_via_bot(&mut self, hide_via_bot: bool) -> &mut Self { self.hide_via_bot = Some(hide_via_bot); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



