
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user has started to play a game. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionStartPlayingGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionStartPlayingGame
  
}



impl Object for ChatActionStartPlayingGame {}
impl RObject for ChatActionStartPlayingGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionStartPlayingGame" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionStartPlayingGame }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionStartPlayingGame {}


impl ChatActionStartPlayingGame {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionStartPlayingGame".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



