
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a game. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputInlineQueryResultGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputInlineQueryResultGame
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Short name of the game.
  game_short_name: Option<String>,
  /// Message reply markup. Must be of type replyMarkupInlineKeyboard or null.
  reply_markup: Option<Box<ReplyMarkup>>,
  
}


impl Clone for InputInlineQueryResultGame {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputInlineQueryResultGame {}
impl RObject for InputInlineQueryResultGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputInlineQueryResultGame" }
  fn td_type(&self) -> RTDType { RTDType::InputInlineQueryResultGame }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputInlineQueryResult for InputInlineQueryResultGame {}


impl InputInlineQueryResultGame {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputInlineQueryResultGame".to_string(),
      id: None,
      game_short_name: None,
      reply_markup: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn game_short_name(&self) -> Option<String> { self.game_short_name.clone() }
  #[doc(hidden)] pub fn _set_game_short_name(&mut self, game_short_name: String) -> &mut Self { self.game_short_name = Some(game_short_name); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



