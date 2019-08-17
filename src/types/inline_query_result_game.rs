
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents information about a game. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultGame
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Game result.
  game: Option<Game>,
  
}



impl Object for InlineQueryResultGame {}
impl RObject for InlineQueryResultGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultGame" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultGame }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultGame {}


impl InlineQueryResultGame {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultGame".to_string(),
      id: None,
      game: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn game(&self) -> Option<Game> { self.game.clone() }
  #[doc(hidden)] pub fn _set_game(&mut self, game: Game) -> &mut Self { self.game = Some(game); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



