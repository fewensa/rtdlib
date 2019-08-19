
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a game; not supported for channels or secret chats. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessageGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageGame
  /// User identifier of the bot that owns the game.
  bot_user_id: Option<i32>,
  /// Short name of the game.
  game_short_name: Option<String>,
  
}



impl Object for InputMessageGame {}
impl RObject for InputMessageGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageGame" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageGame }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageGame {}


impl InputMessageGame {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageGame".to_string(),
      bot_user_id: None,
      game_short_name: None,
      
    }
  }
  
  pub fn bot_user_id(&self) -> Option<i32> { self.bot_user_id.clone() }
  #[doc(hidden)] pub fn _set_bot_user_id(&mut self, bot_user_id: i32) -> &mut Self { self.bot_user_id = Some(bot_user_id); self }
  
  pub fn game_short_name(&self) -> Option<String> { self.game_short_name.clone() }
  #[doc(hidden)] pub fn _set_game_short_name(&mut self, game_short_name: String) -> &mut Self { self.game_short_name = Some(game_short_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



