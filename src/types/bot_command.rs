
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents commands supported by a bot. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommand {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // botCommand
  /// Text of the bot command.
  command: Option<String>,
  /// Description of the bot command.
  description: Option<String>,
  
}



impl Object for BotCommand {}
impl RObject for BotCommand {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommand" }
  fn td_type(&self) -> RTDType { RTDType::BotCommand }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl BotCommand {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "botCommand".to_string(),
      command: None,
      description: None,
      
    }
  }
  
  pub fn command(&self) -> Option<String> { self.command.clone() }
  #[doc(hidden)] pub fn _set_command(&mut self, command: String) -> &mut Self { self.command = Some(command); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



