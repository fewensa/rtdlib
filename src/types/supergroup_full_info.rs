use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains full information about a supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat photo; may be null if empty or unknown. If non-null, then it is the same photo as in chat.photo
  photo: Option<ChatPhoto>,
  /// Identifier of the community to which the corresponding chat was added
  community_id: i64,
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
  /// Chat identifier of a discussion group for the channel, or a channel, for which the supergroup is the designated discussion group; 0 if none or unknown
  linked_chat_id: i64,
  /// Chat identifier of a direct messages group for the channel, or a channel, for which the supergroup is the designated direct messages group; 0 if none
  direct_messages_chat_id: i64,
  /// Delay between consecutive sent messages for non-administrator supergroup members, in seconds
  slow_mode_delay: i64,
  /// Time left before next message can be sent in the supergroup, in seconds. An updateSupergroupFullInfo update is not triggered when value of this field changes, but both new and old values are non-zero
  slow_mode_delay_expires_in: f32,
  /// True, if paid messages can be enabled in the supergroup chat; for supergroup only
  can_enable_paid_messages: bool,
  /// True, if paid reaction can be enabled in the channel chat; for channels only
  can_enable_paid_reaction: bool,
  /// True, if members of the chat can be retrieved via getSupergroupMembers or searchChatMembers
  can_get_members: bool,
  /// True, if non-administrators can receive only administrators and bots using getSupergroupMembers or searchChatMembers
  has_hidden_members: bool,
  /// True, if non-administrators and non-bots can be hidden in responses to getSupergroupMembers and searchChatMembers for non-administrators
  can_hide_members: bool,
  /// True, if the supergroup sticker set can be changed
  can_set_sticker_set: bool,
  /// True, if the supergroup location can be changed
  can_set_location: bool,
  /// True, if the supergroup or channel statistics are available
  can_get_statistics: bool,
  /// True, if the supergroup or channel revenue statistics are available
  can_get_revenue_statistics: bool,
  /// True, if the supergroup or channel Telegram Star revenue statistics are available
  can_get_star_revenue_statistics: bool,
  /// True, if the user can send a gift to the supergroup or channel using sendGift or transferGift
  can_send_gift: bool,
  /// True, if aggressive anti-spam checks can be enabled or disabled in the supergroup
  can_toggle_aggressive_anti_spam: bool,
  /// True, if new chat members will have access to old messages. In public, discussion, or forum groups and all channels, old messages are always available, so this option affects only private non-forum supergroups without a linked chat. The value of this field is only available to chat administrators
  is_all_history_available: bool,
  /// True, if the chat can have sponsored messages. The value of this field is only available to the owner of the chat
  can_have_sponsored_messages: bool,
  /// True, if aggressive anti-spam checks are enabled in the supergroup. The value of this field is only available to chat administrators
  has_aggressive_anti_spam_enabled: bool,
  /// True, if paid media can be sent and forwarded to the channel chat; for channels only
  has_paid_media_allowed: bool,
  /// True, if the supergroup or channel has pinned stories
  has_pinned_stories: bool,
  /// Number of saved to profile gifts for channels without can_post_messages administrator right, otherwise, the total number of received gifts
  gift_count: i64,
  /// Number of times the current user boosted the supergroup or channel
  my_boost_count: i64,
  /// Number of times the supergroup must be boosted by a user to ignore slow mode and chat permission restrictions; 0 if unspecified
  unrestrict_boost_count: i64,
  /// Number of Telegram Stars that must be paid by the current user for each sent message to the supergroup
  outgoing_paid_message_star_count: i64,
  /// Identifier of the supergroup sticker set that must be shown before user sticker sets; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  sticker_set_id: isize,
  /// Identifier of the custom emoji sticker set that can be used in the supergroup without Telegram Premium subscription; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  custom_emoji_sticker_set_id: isize,
  /// Location to which the supergroup is connected; may be null if none
  location: Option<ChatLocation>,
  /// Primary invite link for the chat; may be null. For chat administrators with can_invite_users right only
  invite_link: Option<ChatInviteLink>,
  /// User identifier of the guard bot in the group; for chat administrators only
  guard_bot_user_id: i64,
  /// List of commands of bots in the group
  bot_commands: Vec<BotCommands>,
  /// Information about verification status of the supergroup or the channel provided by a bot; may be null if none or unknown
  bot_verification: Option<BotVerification>,
  /// The main tab chosen by the administrators of the channel; may be null if not chosen manually
  main_profile_tab: Option<ProfileTab>,
  /// Theme of the screen for sharing a link to the supergroup or the channel; may be null if the default theme is used
  share_theme: Option<ProfileShareTheme>,
  /// Identifier of the basic group from which supergroup was upgraded; 0 if none
  upgraded_from_basic_group_id: i64,
  /// Identifier of the last message in the basic group from which supergroup was upgraded; 0 if none
  upgraded_from_max_message_id: i64,
}

impl RObject for SupergroupFullInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "supergroupFullInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SupergroupFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSupergroupFullInfoBuilder {
    let inner = SupergroupFullInfo {
      td_name: "supergroupFullInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSupergroupFullInfoBuilder { inner }
  }

  pub fn photo(&self) -> &Option<ChatPhoto> {
    &self.photo
  }

  pub fn community_id(&self) -> i64 {
    self.community_id
  }

  pub fn description(&self) -> &String {
    &self.description
  }

  pub fn member_count(&self) -> i64 {
    self.member_count
  }

  pub fn administrator_count(&self) -> i64 {
    self.administrator_count
  }

  pub fn restricted_count(&self) -> i64 {
    self.restricted_count
  }

  pub fn banned_count(&self) -> i64 {
    self.banned_count
  }

  pub fn linked_chat_id(&self) -> i64 {
    self.linked_chat_id
  }

  pub fn direct_messages_chat_id(&self) -> i64 {
    self.direct_messages_chat_id
  }

  pub fn slow_mode_delay(&self) -> i64 {
    self.slow_mode_delay
  }

  pub fn slow_mode_delay_expires_in(&self) -> f32 {
    self.slow_mode_delay_expires_in
  }

  pub fn can_enable_paid_messages(&self) -> bool {
    self.can_enable_paid_messages
  }

  pub fn can_enable_paid_reaction(&self) -> bool {
    self.can_enable_paid_reaction
  }

  pub fn can_get_members(&self) -> bool {
    self.can_get_members
  }

  pub fn has_hidden_members(&self) -> bool {
    self.has_hidden_members
  }

  pub fn can_hide_members(&self) -> bool {
    self.can_hide_members
  }

  pub fn can_set_sticker_set(&self) -> bool {
    self.can_set_sticker_set
  }

  pub fn can_set_location(&self) -> bool {
    self.can_set_location
  }

  pub fn can_get_statistics(&self) -> bool {
    self.can_get_statistics
  }

  pub fn can_get_revenue_statistics(&self) -> bool {
    self.can_get_revenue_statistics
  }

  pub fn can_get_star_revenue_statistics(&self) -> bool {
    self.can_get_star_revenue_statistics
  }

  pub fn can_send_gift(&self) -> bool {
    self.can_send_gift
  }

  pub fn can_toggle_aggressive_anti_spam(&self) -> bool {
    self.can_toggle_aggressive_anti_spam
  }

  pub fn is_all_history_available(&self) -> bool {
    self.is_all_history_available
  }

  pub fn can_have_sponsored_messages(&self) -> bool {
    self.can_have_sponsored_messages
  }

  pub fn has_aggressive_anti_spam_enabled(&self) -> bool {
    self.has_aggressive_anti_spam_enabled
  }

  pub fn has_paid_media_allowed(&self) -> bool {
    self.has_paid_media_allowed
  }

  pub fn has_pinned_stories(&self) -> bool {
    self.has_pinned_stories
  }

  pub fn gift_count(&self) -> i64 {
    self.gift_count
  }

  pub fn my_boost_count(&self) -> i64 {
    self.my_boost_count
  }

  pub fn unrestrict_boost_count(&self) -> i64 {
    self.unrestrict_boost_count
  }

  pub fn outgoing_paid_message_star_count(&self) -> i64 {
    self.outgoing_paid_message_star_count
  }

  pub fn sticker_set_id(&self) -> isize {
    self.sticker_set_id
  }

  pub fn custom_emoji_sticker_set_id(&self) -> isize {
    self.custom_emoji_sticker_set_id
  }

  pub fn location(&self) -> &Option<ChatLocation> {
    &self.location
  }

  pub fn invite_link(&self) -> &Option<ChatInviteLink> {
    &self.invite_link
  }

  pub fn guard_bot_user_id(&self) -> i64 {
    self.guard_bot_user_id
  }

  pub fn bot_commands(&self) -> &Vec<BotCommands> {
    &self.bot_commands
  }

  pub fn bot_verification(&self) -> &Option<BotVerification> {
    &self.bot_verification
  }

  pub fn main_profile_tab(&self) -> &Option<ProfileTab> {
    &self.main_profile_tab
  }

  pub fn share_theme(&self) -> &Option<ProfileShareTheme> {
    &self.share_theme
  }

  pub fn upgraded_from_basic_group_id(&self) -> i64 {
    self.upgraded_from_basic_group_id
  }

  pub fn upgraded_from_max_message_id(&self) -> i64 {
    self.upgraded_from_max_message_id
  }
}

#[doc(hidden)]
pub struct RTDSupergroupFullInfoBuilder {
  inner: SupergroupFullInfo,
}

impl RTDSupergroupFullInfoBuilder {
  pub fn build(&self) -> SupergroupFullInfo {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn community_id(&mut self, community_id: i64) -> &mut Self {
    self.inner.community_id = community_id;
    self
  }

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

  pub fn linked_chat_id(&mut self, linked_chat_id: i64) -> &mut Self {
    self.inner.linked_chat_id = linked_chat_id;
    self
  }

  pub fn direct_messages_chat_id(&mut self, direct_messages_chat_id: i64) -> &mut Self {
    self.inner.direct_messages_chat_id = direct_messages_chat_id;
    self
  }

  pub fn slow_mode_delay(&mut self, slow_mode_delay: i64) -> &mut Self {
    self.inner.slow_mode_delay = slow_mode_delay;
    self
  }

  pub fn slow_mode_delay_expires_in(&mut self, slow_mode_delay_expires_in: f32) -> &mut Self {
    self.inner.slow_mode_delay_expires_in = slow_mode_delay_expires_in;
    self
  }

  pub fn can_enable_paid_messages(&mut self, can_enable_paid_messages: bool) -> &mut Self {
    self.inner.can_enable_paid_messages = can_enable_paid_messages;
    self
  }

  pub fn can_enable_paid_reaction(&mut self, can_enable_paid_reaction: bool) -> &mut Self {
    self.inner.can_enable_paid_reaction = can_enable_paid_reaction;
    self
  }

  pub fn can_get_members(&mut self, can_get_members: bool) -> &mut Self {
    self.inner.can_get_members = can_get_members;
    self
  }

  pub fn has_hidden_members(&mut self, has_hidden_members: bool) -> &mut Self {
    self.inner.has_hidden_members = has_hidden_members;
    self
  }

  pub fn can_hide_members(&mut self, can_hide_members: bool) -> &mut Self {
    self.inner.can_hide_members = can_hide_members;
    self
  }

  pub fn can_set_sticker_set(&mut self, can_set_sticker_set: bool) -> &mut Self {
    self.inner.can_set_sticker_set = can_set_sticker_set;
    self
  }

  pub fn can_set_location(&mut self, can_set_location: bool) -> &mut Self {
    self.inner.can_set_location = can_set_location;
    self
  }

  pub fn can_get_statistics(&mut self, can_get_statistics: bool) -> &mut Self {
    self.inner.can_get_statistics = can_get_statistics;
    self
  }

  pub fn can_get_revenue_statistics(&mut self, can_get_revenue_statistics: bool) -> &mut Self {
    self.inner.can_get_revenue_statistics = can_get_revenue_statistics;
    self
  }

  pub fn can_get_star_revenue_statistics(
    &mut self,
    can_get_star_revenue_statistics: bool,
  ) -> &mut Self {
    self.inner.can_get_star_revenue_statistics = can_get_star_revenue_statistics;
    self
  }

  pub fn can_send_gift(&mut self, can_send_gift: bool) -> &mut Self {
    self.inner.can_send_gift = can_send_gift;
    self
  }

  pub fn can_toggle_aggressive_anti_spam(
    &mut self,
    can_toggle_aggressive_anti_spam: bool,
  ) -> &mut Self {
    self.inner.can_toggle_aggressive_anti_spam = can_toggle_aggressive_anti_spam;
    self
  }

  pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self {
    self.inner.is_all_history_available = is_all_history_available;
    self
  }

  pub fn can_have_sponsored_messages(&mut self, can_have_sponsored_messages: bool) -> &mut Self {
    self.inner.can_have_sponsored_messages = can_have_sponsored_messages;
    self
  }

  pub fn has_aggressive_anti_spam_enabled(
    &mut self,
    has_aggressive_anti_spam_enabled: bool,
  ) -> &mut Self {
    self.inner.has_aggressive_anti_spam_enabled = has_aggressive_anti_spam_enabled;
    self
  }

  pub fn has_paid_media_allowed(&mut self, has_paid_media_allowed: bool) -> &mut Self {
    self.inner.has_paid_media_allowed = has_paid_media_allowed;
    self
  }

  pub fn has_pinned_stories(&mut self, has_pinned_stories: bool) -> &mut Self {
    self.inner.has_pinned_stories = has_pinned_stories;
    self
  }

  pub fn gift_count(&mut self, gift_count: i64) -> &mut Self {
    self.inner.gift_count = gift_count;
    self
  }

  pub fn my_boost_count(&mut self, my_boost_count: i64) -> &mut Self {
    self.inner.my_boost_count = my_boost_count;
    self
  }

  pub fn unrestrict_boost_count(&mut self, unrestrict_boost_count: i64) -> &mut Self {
    self.inner.unrestrict_boost_count = unrestrict_boost_count;
    self
  }

  pub fn outgoing_paid_message_star_count(
    &mut self,
    outgoing_paid_message_star_count: i64,
  ) -> &mut Self {
    self.inner.outgoing_paid_message_star_count = outgoing_paid_message_star_count;
    self
  }

  pub fn sticker_set_id(&mut self, sticker_set_id: isize) -> &mut Self {
    self.inner.sticker_set_id = sticker_set_id;
    self
  }

  pub fn custom_emoji_sticker_set_id(&mut self, custom_emoji_sticker_set_id: isize) -> &mut Self {
    self.inner.custom_emoji_sticker_set_id = custom_emoji_sticker_set_id;
    self
  }

  pub fn location<T: AsRef<ChatLocation>>(&mut self, location: T) -> &mut Self {
    self.inner.location = Some(location.as_ref().clone());
    self
  }

  pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = Some(invite_link.as_ref().clone());
    self
  }

  pub fn guard_bot_user_id(&mut self, guard_bot_user_id: i64) -> &mut Self {
    self.inner.guard_bot_user_id = guard_bot_user_id;
    self
  }

  pub fn bot_commands(&mut self, bot_commands: Vec<BotCommands>) -> &mut Self {
    self.inner.bot_commands = bot_commands;
    self
  }

  pub fn bot_verification<T: AsRef<BotVerification>>(&mut self, bot_verification: T) -> &mut Self {
    self.inner.bot_verification = Some(bot_verification.as_ref().clone());
    self
  }

  pub fn main_profile_tab<T: AsRef<ProfileTab>>(&mut self, main_profile_tab: T) -> &mut Self {
    self.inner.main_profile_tab = Some(main_profile_tab.as_ref().clone());
    self
  }

  pub fn share_theme<T: AsRef<ProfileShareTheme>>(&mut self, share_theme: T) -> &mut Self {
    self.inner.share_theme = Some(share_theme.as_ref().clone());
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
  fn as_ref(&self) -> &SupergroupFullInfo {
    self
  }
}

impl AsRef<SupergroupFullInfo> for RTDSupergroupFullInfoBuilder {
  fn as_ref(&self) -> &SupergroupFullInfo {
    &self.inner
  }
}
