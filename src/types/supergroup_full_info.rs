
use crate::types::*;
use crate::errors::*;




/// Contains full information about a supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Contains full information about a supergroup or channel
  description: String,
  /// Number of members in the supergroup or channel; 0 if unknown
  member_count: i64,
  /// Number of privileged users in the supergroup or channel; 0 if unknown
  administrator_count: i64,
  /// Number of restricted users in the supergroup; 0 if unknown
  restricted_count: i64,
  /// Number of users banned from chat; 0 if unknown
  banned_count: i64,
  /// True, if members of the chat can be retrieved
  can_get_members: bool,
  /// True, if the chat can be made public
  can_set_username: bool,
  /// True, if the supergroup sticker set can be changed
  can_set_sticker_set: bool,
  /// True, if the channel statistics is available through getChatStatisticsUrl
  can_view_statistics: bool,
  /// True, if new chat members will have access to old messages. In public supergroups and both public and private channels, old messages are always available, so this option affects only private supergroups. The value of this field is only available for chat administrators
  is_all_history_available: bool,
  /// Identifier of the supergroup sticker set; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")] sticker_set_id: isize,
  /// Invite link for this chat
  invite_link: String,
  /// Identifier of the basic group from which supergroup was upgraded; 0 if none
  upgraded_from_basic_group_id: i64,
  /// Identifier of the last message in the basic group from which supergroup was upgraded; 0 if none
  upgraded_from_max_message_id: i64,
  
}

impl RObject for SupergroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "supergroupFullInfo" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl SupergroupFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSupergroupFullInfoBuilder {
    let mut inner = SupergroupFullInfo::default();
    inner.td_name = "supergroupFullInfo".to_string();
    RTDSupergroupFullInfoBuilder { inner }
  }

  pub fn description(&self) -> &String { &self.description }

  pub fn member_count(&self) -> i64 { self.member_count }

  pub fn administrator_count(&self) -> i64 { self.administrator_count }

  pub fn restricted_count(&self) -> i64 { self.restricted_count }

  pub fn banned_count(&self) -> i64 { self.banned_count }

  pub fn can_get_members(&self) -> bool { self.can_get_members }

  pub fn can_set_username(&self) -> bool { self.can_set_username }

  pub fn can_set_sticker_set(&self) -> bool { self.can_set_sticker_set }

  pub fn can_view_statistics(&self) -> bool { self.can_view_statistics }

  pub fn is_all_history_available(&self) -> bool { self.is_all_history_available }

  pub fn sticker_set_id(&self) -> isize { self.sticker_set_id }

  pub fn invite_link(&self) -> &String { &self.invite_link }

  pub fn upgraded_from_basic_group_id(&self) -> i64 { self.upgraded_from_basic_group_id }

  pub fn upgraded_from_max_message_id(&self) -> i64 { self.upgraded_from_max_message_id }

}

#[doc(hidden)]
pub struct RTDSupergroupFullInfoBuilder {
  inner: SupergroupFullInfo
}

impl RTDSupergroupFullInfoBuilder {
  pub fn build(&self) -> SupergroupFullInfo { self.inner.clone() }

   
  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

   
  pub fn member_count(&mut self, member_count: i64) -> &mut Self {
    self.inner.member_count = member_count;
    self
  }

   
  pub fn administrator_count(&mut self, administrator_count: i64) -> &mut Self {
    self.inner.administrator_count = administrator_count;
    self
  }

   
  pub fn restricted_count(&mut self, restricted_count: i64) -> &mut Self {
    self.inner.restricted_count = restricted_count;
    self
  }

   
  pub fn banned_count(&mut self, banned_count: i64) -> &mut Self {
    self.inner.banned_count = banned_count;
    self
  }

   
  pub fn can_get_members(&mut self, can_get_members: bool) -> &mut Self {
    self.inner.can_get_members = can_get_members;
    self
  }

   
  pub fn can_set_username(&mut self, can_set_username: bool) -> &mut Self {
    self.inner.can_set_username = can_set_username;
    self
  }

   
  pub fn can_set_sticker_set(&mut self, can_set_sticker_set: bool) -> &mut Self {
    self.inner.can_set_sticker_set = can_set_sticker_set;
    self
  }

   
  pub fn can_view_statistics(&mut self, can_view_statistics: bool) -> &mut Self {
    self.inner.can_view_statistics = can_view_statistics;
    self
  }

   
  pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self {
    self.inner.is_all_history_available = is_all_history_available;
    self
  }

   
  pub fn sticker_set_id(&mut self, sticker_set_id: isize) -> &mut Self {
    self.inner.sticker_set_id = sticker_set_id;
    self
  }

   
  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }

   
  pub fn upgraded_from_basic_group_id(&mut self, upgraded_from_basic_group_id: i64) -> &mut Self {
    self.inner.upgraded_from_basic_group_id = upgraded_from_basic_group_id;
    self
  }

   
  pub fn upgraded_from_max_message_id(&mut self, upgraded_from_max_message_id: i64) -> &mut Self {
    self.inner.upgraded_from_max_message_id = upgraded_from_max_message_id;
    self
  }

}

impl AsRef<SupergroupFullInfo> for SupergroupFullInfo {
  fn as_ref(&self) -> &SupergroupFullInfo { self }
}

impl AsRef<SupergroupFullInfo> for RTDSupergroupFullInfoBuilder {
  fn as_ref(&self) -> &SupergroupFullInfo { &self.inner }
}



