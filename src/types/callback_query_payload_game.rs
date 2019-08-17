
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The payload from a game callback button. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQueryPayloadGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callbackQueryPayloadGame
  /// A short name of the game that was attached to the callback button.
  game_short_name: Option<String>,
  
}



impl Object for CallbackQueryPayloadGame {}
impl RObject for CallbackQueryPayloadGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callbackQueryPayloadGame" }
  fn td_type(&self) -> RTDType { RTDType::CallbackQueryPayloadGame }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl CallbackQueryPayload for CallbackQueryPayloadGame {}


impl CallbackQueryPayloadGame {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callbackQueryPayloadGame".to_string(),
      game_short_name: None,
      
    }
  }
  
  pub fn game_short_name(&self) -> Option<String> { self.game_short_name.clone() }
  #[doc(hidden)] pub fn _set_game_short_name(&mut self, game_short_name: String) -> &mut Self { self.game_short_name = Some(game_short_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



