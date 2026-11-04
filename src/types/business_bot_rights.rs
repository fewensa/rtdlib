use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes rights of a business bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessBotRights {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the bot can send and edit messages in the private chats that had incoming messages in the last 24 hours
  can_reply: bool,
  /// True, if the bot can mark incoming private messages as read
  can_read_messages: bool,
  /// True, if the bot can delete sent messages
  can_delete_sent_messages: bool,
  /// True, if the bot can delete any message
  can_delete_all_messages: bool,
  /// True, if the bot can edit name of the business account
  can_edit_name: bool,
  /// True, if the bot can edit bio of the business account
  can_edit_bio: bool,
  /// True, if the bot can edit profile photo of the business account
  can_edit_profile_photo: bool,
  /// True, if the bot can edit username of the business account
  can_edit_username: bool,
  /// True, if the bot can view gifts and Telegram Star amount owned by the business account
  can_view_gifts_and_stars: bool,
  /// True, if the bot can sell or delete regular gifts received by the business account
  can_sell_gifts: bool,
  /// True, if the bot can change gift receiving settings of the business account
  can_change_gift_settings: bool,
  /// True, if the bot can transfer and upgrade gifts received by the business account
  can_transfer_and_upgrade_gifts: bool,
  /// True, if the bot can transfer Telegram Stars received by the business account to account of the bot, or use them to upgrade and transfer gifts
  can_transfer_stars: bool,
  /// True, if the bot can post, edit and delete stories
  can_manage_stories: bool,
}

impl RObject for BusinessBotRights {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessBotRights"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessBotRights {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessBotRightsBuilder {
    let inner = BusinessBotRights {
      td_name: "businessBotRights".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessBotRightsBuilder { inner }
  }

  pub fn can_reply(&self) -> bool {
    self.can_reply
  }

  pub fn can_read_messages(&self) -> bool {
    self.can_read_messages
  }

  pub fn can_delete_sent_messages(&self) -> bool {
    self.can_delete_sent_messages
  }

  pub fn can_delete_all_messages(&self) -> bool {
    self.can_delete_all_messages
  }

  pub fn can_edit_name(&self) -> bool {
    self.can_edit_name
  }

  pub fn can_edit_bio(&self) -> bool {
    self.can_edit_bio
  }

  pub fn can_edit_profile_photo(&self) -> bool {
    self.can_edit_profile_photo
  }

  pub fn can_edit_username(&self) -> bool {
    self.can_edit_username
  }

  pub fn can_view_gifts_and_stars(&self) -> bool {
    self.can_view_gifts_and_stars
  }

  pub fn can_sell_gifts(&self) -> bool {
    self.can_sell_gifts
  }

  pub fn can_change_gift_settings(&self) -> bool {
    self.can_change_gift_settings
  }

  pub fn can_transfer_and_upgrade_gifts(&self) -> bool {
    self.can_transfer_and_upgrade_gifts
  }

  pub fn can_transfer_stars(&self) -> bool {
    self.can_transfer_stars
  }

  pub fn can_manage_stories(&self) -> bool {
    self.can_manage_stories
  }
}

#[doc(hidden)]
pub struct RTDBusinessBotRightsBuilder {
  inner: BusinessBotRights,
}

impl RTDBusinessBotRightsBuilder {
  pub fn build(&self) -> BusinessBotRights {
    self.inner.clone()
  }

  pub fn can_reply(&mut self, can_reply: bool) -> &mut Self {
    self.inner.can_reply = can_reply;
    self
  }

  pub fn can_read_messages(&mut self, can_read_messages: bool) -> &mut Self {
    self.inner.can_read_messages = can_read_messages;
    self
  }

  pub fn can_delete_sent_messages(&mut self, can_delete_sent_messages: bool) -> &mut Self {
    self.inner.can_delete_sent_messages = can_delete_sent_messages;
    self
  }

  pub fn can_delete_all_messages(&mut self, can_delete_all_messages: bool) -> &mut Self {
    self.inner.can_delete_all_messages = can_delete_all_messages;
    self
  }

  pub fn can_edit_name(&mut self, can_edit_name: bool) -> &mut Self {
    self.inner.can_edit_name = can_edit_name;
    self
  }

  pub fn can_edit_bio(&mut self, can_edit_bio: bool) -> &mut Self {
    self.inner.can_edit_bio = can_edit_bio;
    self
  }

  pub fn can_edit_profile_photo(&mut self, can_edit_profile_photo: bool) -> &mut Self {
    self.inner.can_edit_profile_photo = can_edit_profile_photo;
    self
  }

  pub fn can_edit_username(&mut self, can_edit_username: bool) -> &mut Self {
    self.inner.can_edit_username = can_edit_username;
    self
  }

  pub fn can_view_gifts_and_stars(&mut self, can_view_gifts_and_stars: bool) -> &mut Self {
    self.inner.can_view_gifts_and_stars = can_view_gifts_and_stars;
    self
  }

  pub fn can_sell_gifts(&mut self, can_sell_gifts: bool) -> &mut Self {
    self.inner.can_sell_gifts = can_sell_gifts;
    self
  }

  pub fn can_change_gift_settings(&mut self, can_change_gift_settings: bool) -> &mut Self {
    self.inner.can_change_gift_settings = can_change_gift_settings;
    self
  }

  pub fn can_transfer_and_upgrade_gifts(
    &mut self,
    can_transfer_and_upgrade_gifts: bool,
  ) -> &mut Self {
    self.inner.can_transfer_and_upgrade_gifts = can_transfer_and_upgrade_gifts;
    self
  }

  pub fn can_transfer_stars(&mut self, can_transfer_stars: bool) -> &mut Self {
    self.inner.can_transfer_stars = can_transfer_stars;
    self
  }

  pub fn can_manage_stories(&mut self, can_manage_stories: bool) -> &mut Self {
    self.inner.can_manage_stories = can_manage_stories;
    self
  }
}

impl AsRef<BusinessBotRights> for BusinessBotRights {
  fn as_ref(&self) -> &BusinessBotRights {
    self
  }
}

impl AsRef<BusinessBotRights> for RTDBusinessBotRightsBuilder {
  fn as_ref(&self) -> &BusinessBotRights {
    &self.inner
  }
}
