use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  id: i64,
  /// First name of the user
  first_name: String,
  /// Last name of the user
  last_name: String,
  /// Usernames of the user; may be null
  usernames: Option<Usernames>,
  /// Phone number of the user
  phone_number: String,
  /// Current online status of the user
  status: UserStatus,
  /// Profile photo of the user; may be null
  profile_photo: Option<ProfilePhoto>,
  /// Identifier of the accent color for name, and backgrounds of profile photo, reply header, and link preview
  accent_color_id: i64,
  /// Identifier of a custom emoji to be shown on the reply header and link preview background; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  background_custom_emoji_id: isize,
  /// Color scheme based on an upgraded gift to be used for the user instead of accent_color_id and background_custom_emoji_id; may be null if none
  upgraded_gift_colors: Option<UpgradedGiftColors>,
  /// Identifier of the accent color for the user's profile; 1 if none
  profile_accent_color_id: i64,
  /// Identifier of a custom emoji to be shown on the background of the user's profile; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  profile_background_custom_emoji_id: isize,
  /// Emoji status to be shown instead of the default Telegram Premium badge; may be null
  emoji_status: Option<EmojiStatus>,
  /// The user is a contact of the current user
  is_contact: bool,
  /// The user is a contact of the current user and the current user is a contact of the user
  is_mutual_contact: bool,
  /// The user is a close friend of the current user; implies that the user is a contact
  is_close_friend: bool,
  /// Information about verification status of the user; may be null if none
  verification_status: Option<VerificationStatus>,
  /// True, if the user is a Telegram Premium user
  is_premium: bool,
  /// True, if the user is Telegram support account
  is_support: bool,
  /// Information about restrictions that must be applied to the corresponding private chat; may be null if none
  restriction_info: Option<RestrictionInfo>,
  /// State of active stories of the user; may be null if the user has no active stories
  active_story_state: Option<ActiveStoryState>,
  /// True, if the user may restrict new chats with non-contacts. Use canSendMessageToUser to check whether the current user can message the user or try to create a chat with them
  restricts_new_chats: bool,
  /// Number of Telegram Stars that must be paid by general user for each sent message to the user. If positive and userFullInfo is unknown, use canSendMessageToUser to check whether the current user must pay
  paid_message_star_count: i64,
  /// If false, the user is inaccessible, and the only information known about the user is inside this class. Identifier of the user can't be passed to any method
  have_access: bool,
  /// Type of the user
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: UserType,
  /// IETF language tag of the user's language; only available to bots
  language_code: String,
  /// True, if the user added the current bot to attachment menu; only available to bots
  added_to_attachment_menu: bool,
}

impl RObject for User {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "user"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl User {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserBuilder {
    let inner = User {
      td_name: "user".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn first_name(&self) -> &String {
    &self.first_name
  }

  pub fn last_name(&self) -> &String {
    &self.last_name
  }

  pub fn usernames(&self) -> &Option<Usernames> {
    &self.usernames
  }

  pub fn phone_number(&self) -> &String {
    &self.phone_number
  }

  pub fn status(&self) -> &UserStatus {
    &self.status
  }

  pub fn profile_photo(&self) -> &Option<ProfilePhoto> {
    &self.profile_photo
  }

  pub fn accent_color_id(&self) -> i64 {
    self.accent_color_id
  }

  pub fn background_custom_emoji_id(&self) -> isize {
    self.background_custom_emoji_id
  }

  pub fn upgraded_gift_colors(&self) -> &Option<UpgradedGiftColors> {
    &self.upgraded_gift_colors
  }

  pub fn profile_accent_color_id(&self) -> i64 {
    self.profile_accent_color_id
  }

  pub fn profile_background_custom_emoji_id(&self) -> isize {
    self.profile_background_custom_emoji_id
  }

  pub fn emoji_status(&self) -> &Option<EmojiStatus> {
    &self.emoji_status
  }

  pub fn is_contact(&self) -> bool {
    self.is_contact
  }

  pub fn is_mutual_contact(&self) -> bool {
    self.is_mutual_contact
  }

  pub fn is_close_friend(&self) -> bool {
    self.is_close_friend
  }

  pub fn verification_status(&self) -> &Option<VerificationStatus> {
    &self.verification_status
  }

  pub fn is_premium(&self) -> bool {
    self.is_premium
  }

  pub fn is_support(&self) -> bool {
    self.is_support
  }

  pub fn restriction_info(&self) -> &Option<RestrictionInfo> {
    &self.restriction_info
  }

  pub fn active_story_state(&self) -> &Option<ActiveStoryState> {
    &self.active_story_state
  }

  pub fn restricts_new_chats(&self) -> bool {
    self.restricts_new_chats
  }

  pub fn paid_message_star_count(&self) -> i64 {
    self.paid_message_star_count
  }

  pub fn have_access(&self) -> bool {
    self.have_access
  }

  pub fn type_(&self) -> &UserType {
    &self.type_
  }

  pub fn language_code(&self) -> &String {
    &self.language_code
  }

  pub fn added_to_attachment_menu(&self) -> bool {
    self.added_to_attachment_menu
  }
}

#[doc(hidden)]
pub struct RTDUserBuilder {
  inner: User,
}

impl RTDUserBuilder {
  pub fn build(&self) -> User {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn first_name<T: AsRef<str>>(&mut self, first_name: T) -> &mut Self {
    self.inner.first_name = first_name.as_ref().to_string();
    self
  }

  pub fn last_name<T: AsRef<str>>(&mut self, last_name: T) -> &mut Self {
    self.inner.last_name = last_name.as_ref().to_string();
    self
  }

  pub fn usernames<T: AsRef<Usernames>>(&mut self, usernames: T) -> &mut Self {
    self.inner.usernames = Some(usernames.as_ref().clone());
    self
  }

  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }

  pub fn status<T: AsRef<UserStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

  pub fn profile_photo<T: AsRef<ProfilePhoto>>(&mut self, profile_photo: T) -> &mut Self {
    self.inner.profile_photo = Some(profile_photo.as_ref().clone());
    self
  }

  pub fn accent_color_id(&mut self, accent_color_id: i64) -> &mut Self {
    self.inner.accent_color_id = accent_color_id;
    self
  }

  pub fn background_custom_emoji_id(&mut self, background_custom_emoji_id: isize) -> &mut Self {
    self.inner.background_custom_emoji_id = background_custom_emoji_id;
    self
  }

  pub fn upgraded_gift_colors<T: AsRef<UpgradedGiftColors>>(
    &mut self,
    upgraded_gift_colors: T,
  ) -> &mut Self {
    self.inner.upgraded_gift_colors = Some(upgraded_gift_colors.as_ref().clone());
    self
  }

  pub fn profile_accent_color_id(&mut self, profile_accent_color_id: i64) -> &mut Self {
    self.inner.profile_accent_color_id = profile_accent_color_id;
    self
  }

  pub fn profile_background_custom_emoji_id(
    &mut self,
    profile_background_custom_emoji_id: isize,
  ) -> &mut Self {
    self.inner.profile_background_custom_emoji_id = profile_background_custom_emoji_id;
    self
  }

  pub fn emoji_status<T: AsRef<EmojiStatus>>(&mut self, emoji_status: T) -> &mut Self {
    self.inner.emoji_status = Some(emoji_status.as_ref().clone());
    self
  }

  pub fn is_contact(&mut self, is_contact: bool) -> &mut Self {
    self.inner.is_contact = is_contact;
    self
  }

  pub fn is_mutual_contact(&mut self, is_mutual_contact: bool) -> &mut Self {
    self.inner.is_mutual_contact = is_mutual_contact;
    self
  }

  pub fn is_close_friend(&mut self, is_close_friend: bool) -> &mut Self {
    self.inner.is_close_friend = is_close_friend;
    self
  }

  pub fn verification_status<T: AsRef<VerificationStatus>>(
    &mut self,
    verification_status: T,
  ) -> &mut Self {
    self.inner.verification_status = Some(verification_status.as_ref().clone());
    self
  }

  pub fn is_premium(&mut self, is_premium: bool) -> &mut Self {
    self.inner.is_premium = is_premium;
    self
  }

  pub fn is_support(&mut self, is_support: bool) -> &mut Self {
    self.inner.is_support = is_support;
    self
  }

  pub fn restriction_info<T: AsRef<RestrictionInfo>>(&mut self, restriction_info: T) -> &mut Self {
    self.inner.restriction_info = Some(restriction_info.as_ref().clone());
    self
  }

  pub fn active_story_state<T: AsRef<ActiveStoryState>>(
    &mut self,
    active_story_state: T,
  ) -> &mut Self {
    self.inner.active_story_state = Some(active_story_state.as_ref().clone());
    self
  }

  pub fn restricts_new_chats(&mut self, restricts_new_chats: bool) -> &mut Self {
    self.inner.restricts_new_chats = restricts_new_chats;
    self
  }

  pub fn paid_message_star_count(&mut self, paid_message_star_count: i64) -> &mut Self {
    self.inner.paid_message_star_count = paid_message_star_count;
    self
  }

  pub fn have_access(&mut self, have_access: bool) -> &mut Self {
    self.inner.have_access = have_access;
    self
  }

  pub fn type_<T: AsRef<UserType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn language_code<T: AsRef<str>>(&mut self, language_code: T) -> &mut Self {
    self.inner.language_code = language_code.as_ref().to_string();
    self
  }

  pub fn added_to_attachment_menu(&mut self, added_to_attachment_menu: bool) -> &mut Self {
    self.inner.added_to_attachment_menu = added_to_attachment_menu;
    self
  }
}

impl AsRef<User> for User {
  fn as_ref(&self) -> &User {
    self
  }
}

impl AsRef<User> for RTDUserBuilder {
  fn as_ref(&self) -> &User {
    &self.inner
  }
}
