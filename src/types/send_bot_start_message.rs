
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBotStartMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // sendBotStartMessage
  /// Identifier of the bot.
  bot_user_id: Option<i32>,
  /// Identifier of the target chat.
  chat_id: Option<i64>,
  /// A hidden parameter sent to the bot for deep linking purposes (https://api.telegram.org/bots#deep-linking).
  parameter: Option<String>,
  
}



impl Object for SendBotStartMessage {}
impl RObject for SendBotStartMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendBotStartMessage" }
  fn td_type(&self) -> RTDType { RTDType::SendBotStartMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SendBotStartMessage {}


impl SendBotStartMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "sendBotStartMessage".to_string(),
      bot_user_id: None,
      chat_id: None,
      parameter: None,
      
    }
  }
  
  pub fn bot_user_id(&self) -> Option<i32> { self.bot_user_id.clone() }
  #[doc(hidden)] pub fn _set_bot_user_id(&mut self, bot_user_id: i32) -> &mut Self { self.bot_user_id = Some(bot_user_id); self }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn parameter(&self) -> Option<String> { self.parameter.clone() }
  #[doc(hidden)] pub fn _set_parameter(&mut self, parameter: String) -> &mut Self { self.parameter = Some(parameter); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



