
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A bot (see 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTypeBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // userTypeBot
  /// True, if the bot can be invited to basic group and supergroup chats.
  can_join_groups: Option<bool>,
  /// True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages.
  can_read_all_group_messages: Option<bool>,
  /// True, if the bot supports inline queries.
  is_inline: Option<bool>,
  /// Placeholder for inline queries (displayed on the client input field).
  inline_query_placeholder: Option<String>,
  /// True, if the location of the user should be sent with every inline query to this bot.
  need_location: Option<bool>,
  
}



impl Object for UserTypeBot {}
impl RObject for UserTypeBot {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userTypeBot" }
  fn td_type(&self) -> RTDType { RTDType::UserTypeBot }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl UserType for UserTypeBot {}


impl UserTypeBot {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "userTypeBot".to_string(),
      can_join_groups: None,
      can_read_all_group_messages: None,
      is_inline: None,
      inline_query_placeholder: None,
      need_location: None,
      
    }
  }
  
  pub fn can_join_groups(&self) -> Option<bool> { self.can_join_groups.clone() }
  #[doc(hidden)] pub fn _set_can_join_groups(&mut self, can_join_groups: bool) -> &mut Self { self.can_join_groups = Some(can_join_groups); self }
  
  pub fn can_read_all_group_messages(&self) -> Option<bool> { self.can_read_all_group_messages.clone() }
  #[doc(hidden)] pub fn _set_can_read_all_group_messages(&mut self, can_read_all_group_messages: bool) -> &mut Self { self.can_read_all_group_messages = Some(can_read_all_group_messages); self }
  
  pub fn is_inline(&self) -> Option<bool> { self.is_inline.clone() }
  #[doc(hidden)] pub fn _set_is_inline(&mut self, is_inline: bool) -> &mut Self { self.is_inline = Some(is_inline); self }
  
  pub fn inline_query_placeholder(&self) -> Option<String> { self.inline_query_placeholder.clone() }
  #[doc(hidden)] pub fn _set_inline_query_placeholder(&mut self, inline_query_placeholder: String) -> &mut Self { self.inline_query_placeholder = Some(inline_query_placeholder); self }
  
  pub fn need_location(&self) -> Option<bool> { self.need_location.clone() }
  #[doc(hidden)] pub fn _set_need_location(&mut self, need_location: bool) -> &mut Self { self.need_location = Some(need_location); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



