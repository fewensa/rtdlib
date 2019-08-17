
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Provides information about a bot and its supported commands. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // botInfo
  /// Long description shown on the user info page.
  description: Option<String>,
  /// A list of commands supported by the bot.
  commands: Option<Vec<BotCommand>>,
  
}



impl Object for BotInfo {}
impl RObject for BotInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botInfo" }
  fn td_type(&self) -> RTDType { RTDType::BotInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl BotInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "botInfo".to_string(),
      description: None,
      commands: None,
      
    }
  }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn commands(&self) -> Option<Vec<BotCommand>> { self.commands.clone() }
  #[doc(hidden)] pub fn _set_commands(&mut self, commands: Vec<BotCommand>) -> &mut Self { self.commands = Some(commands); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



