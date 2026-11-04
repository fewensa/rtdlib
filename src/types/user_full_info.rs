use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains full information about a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User profile photo set by the current user for the contact; may be null. If null and user.profile_photo is null, then the photo is empty; otherwise, it is unknown. If non-null, then it is the same photo as in user.profile_photo and chat.photo. This photo isn't returned in the list of user photos
  personal_photo: Option<ChatPhoto>,
  /// User profile photo; may be null. If null and user.profile_photo is null, then the photo is empty; otherwise, it is unknown. If non-null and personal_photo is null, then it is the same photo as in user.profile_photo and chat.photo
  photo: Option<ChatPhoto>,
  /// User profile photo visible if the main photo is hidden by privacy settings; may be null. If null and user.profile_photo is null, then the photo is empty; otherwise, it is unknown. If non-null and both photo and personal_photo are null, then it is the same photo as in user.profile_photo and chat.photo. This photo isn't returned in the list of user photos
  public_photo: Option<ChatPhoto>,
  /// Identifier of the community to which chat with the bot was added; for bots only
  community_id: i64,
  /// Block list to which the user is added; may be null if none
  block_list: Option<BlockList>,
  /// True, if the user can be called
  can_be_called: bool,
  /// True, if a video call can be created with the user
  supports_video_calls: bool,
  /// True, if the user can't be called due to their privacy settings
  has_private_calls: bool,
  /// True, if the user can't be linked in forwarded messages due to their privacy settings
  has_private_forwards: bool,
  /// True, if voice and video notes can't be sent or forwarded to the user
  has_restricted_voice_and_video_note_messages: bool,
  /// True, if the user has posted to profile stories
  has_posted_to_profile_stories: bool,
  /// True, if the user always enabled sponsored messages; known only for the current user
  has_sponsored_messages_enabled: bool,
  /// True, if the current user needs to explicitly allow to share their phone number with the user when the method addContact is used
  need_phone_number_privacy_exception: bool,
  /// True, if the user set chat background for both chat users and it wasn't reverted yet
  set_chat_background: bool,
  /// True, if the user uses an unofficial application that poses a security risk
  uses_unofficial_app: bool,
  /// A short user bio; may be null for bots
  bio: Option<FormattedText>,
  /// Birthdate of the user; may be null if unknown
  birthdate: Option<Birthdate>,
  /// Identifier of the personal chat of the user; 0 if none
  personal_chat_id: i64,
  /// Number of saved to profile gifts for other users or the total number of received gifts for the current user
  gift_count: i64,
  /// Number of group chats where both the other user and the current user are a member; 0 for the current user
  group_in_common_count: i64,
  /// Number of Telegram Stars that must be paid by the user for each sent message to the current user
  incoming_paid_message_star_count: i64,
  /// Number of Telegram Stars that must be paid by the current user for each sent message to the user
  outgoing_paid_message_star_count: i64,
  /// Settings for gift receiving for the user
  gift_settings: GiftSettings,
  /// Information about verification status of the user provided by a bot; may be null if none or unknown
  bot_verification: Option<BotVerification>,
  /// The main tab chosen by the user; may be null if not chosen manually
  main_profile_tab: Option<ProfileTab>,
  /// The first audio file added to the user's profile; may be null if none
  first_profile_audio: Option<Audio>,
  /// The current rating of the user; may be null if none
  rating: Option<UserRating>,
  /// The rating of the user after the next change; may be null if the user isn't the current user or there are no pending rating changes
  pending_rating: Option<UserRating>,
  /// Unix timestamp when rating of the user will change to pending_rating; 0 if the user isn't the current user or there are no pending rating changes
  pending_rating_date: i64,
  /// Note added to the user's contact; may be null if none
  note: Option<FormattedText>,
  /// TON wallet card of the user; may be null if none
  ton_wallet_card: Option<TonWalletCard>,
  /// Theme of the screen for sharing a link to the user; may be null if the default theme is used
  share_theme: Option<ProfileShareTheme>,
  /// Information about business settings for Telegram Business accounts; may be null if none
  business_info: Option<BusinessInfo>,
  /// For bots, information about the bot; may be null if the user isn't a bot
  bot_info: Option<BotInfo>,
}

impl RObject for UserFullInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userFullInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UserFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserFullInfoBuilder {
    let inner = UserFullInfo {
      td_name: "userFullInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserFullInfoBuilder { inner }
  }

  pub fn personal_photo(&self) -> &Option<ChatPhoto> {
    &self.personal_photo
  }

  pub fn photo(&self) -> &Option<ChatPhoto> {
    &self.photo
  }

  pub fn public_photo(&self) -> &Option<ChatPhoto> {
    &self.public_photo
  }

  pub fn community_id(&self) -> i64 {
    self.community_id
  }

  pub fn block_list(&self) -> &Option<BlockList> {
    &self.block_list
  }

  pub fn can_be_called(&self) -> bool {
    self.can_be_called
  }

  pub fn supports_video_calls(&self) -> bool {
    self.supports_video_calls
  }

  pub fn has_private_calls(&self) -> bool {
    self.has_private_calls
  }

  pub fn has_private_forwards(&self) -> bool {
    self.has_private_forwards
  }

  pub fn has_restricted_voice_and_video_note_messages(&self) -> bool {
    self.has_restricted_voice_and_video_note_messages
  }

  pub fn has_posted_to_profile_stories(&self) -> bool {
    self.has_posted_to_profile_stories
  }

  pub fn has_sponsored_messages_enabled(&self) -> bool {
    self.has_sponsored_messages_enabled
  }

  pub fn need_phone_number_privacy_exception(&self) -> bool {
    self.need_phone_number_privacy_exception
  }

  pub fn set_chat_background(&self) -> bool {
    self.set_chat_background
  }

  pub fn uses_unofficial_app(&self) -> bool {
    self.uses_unofficial_app
  }

  pub fn bio(&self) -> &Option<FormattedText> {
    &self.bio
  }

  pub fn birthdate(&self) -> &Option<Birthdate> {
    &self.birthdate
  }

  pub fn personal_chat_id(&self) -> i64 {
    self.personal_chat_id
  }

  pub fn gift_count(&self) -> i64 {
    self.gift_count
  }

  pub fn group_in_common_count(&self) -> i64 {
    self.group_in_common_count
  }

  pub fn incoming_paid_message_star_count(&self) -> i64 {
    self.incoming_paid_message_star_count
  }

  pub fn outgoing_paid_message_star_count(&self) -> i64 {
    self.outgoing_paid_message_star_count
  }

  pub fn gift_settings(&self) -> &GiftSettings {
    &self.gift_settings
  }

  pub fn bot_verification(&self) -> &Option<BotVerification> {
    &self.bot_verification
  }

  pub fn main_profile_tab(&self) -> &Option<ProfileTab> {
    &self.main_profile_tab
  }

  pub fn first_profile_audio(&self) -> &Option<Audio> {
    &self.first_profile_audio
  }

  pub fn rating(&self) -> &Option<UserRating> {
    &self.rating
  }

  pub fn pending_rating(&self) -> &Option<UserRating> {
    &self.pending_rating
  }

  pub fn pending_rating_date(&self) -> i64 {
    self.pending_rating_date
  }

  pub fn note(&self) -> &Option<FormattedText> {
    &self.note
  }

  pub fn ton_wallet_card(&self) -> &Option<TonWalletCard> {
    &self.ton_wallet_card
  }

  pub fn share_theme(&self) -> &Option<ProfileShareTheme> {
    &self.share_theme
  }

  pub fn business_info(&self) -> &Option<BusinessInfo> {
    &self.business_info
  }

  pub fn bot_info(&self) -> &Option<BotInfo> {
    &self.bot_info
  }
}

#[doc(hidden)]
pub struct RTDUserFullInfoBuilder {
  inner: UserFullInfo,
}

impl RTDUserFullInfoBuilder {
  pub fn build(&self) -> UserFullInfo {
    self.inner.clone()
  }

  pub fn personal_photo<T: AsRef<ChatPhoto>>(&mut self, personal_photo: T) -> &mut Self {
    self.inner.personal_photo = Some(personal_photo.as_ref().clone());
    self
  }

  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn public_photo<T: AsRef<ChatPhoto>>(&mut self, public_photo: T) -> &mut Self {
    self.inner.public_photo = Some(public_photo.as_ref().clone());
    self
  }

  pub fn community_id(&mut self, community_id: i64) -> &mut Self {
    self.inner.community_id = community_id;
    self
  }

  pub fn block_list<T: AsRef<BlockList>>(&mut self, block_list: T) -> &mut Self {
    self.inner.block_list = Some(block_list.as_ref().clone());
    self
  }

  pub fn can_be_called(&mut self, can_be_called: bool) -> &mut Self {
    self.inner.can_be_called = can_be_called;
    self
  }

  pub fn supports_video_calls(&mut self, supports_video_calls: bool) -> &mut Self {
    self.inner.supports_video_calls = supports_video_calls;
    self
  }

  pub fn has_private_calls(&mut self, has_private_calls: bool) -> &mut Self {
    self.inner.has_private_calls = has_private_calls;
    self
  }

  pub fn has_private_forwards(&mut self, has_private_forwards: bool) -> &mut Self {
    self.inner.has_private_forwards = has_private_forwards;
    self
  }

  pub fn has_restricted_voice_and_video_note_messages(
    &mut self,
    has_restricted_voice_and_video_note_messages: bool,
  ) -> &mut Self {
    self.inner.has_restricted_voice_and_video_note_messages =
      has_restricted_voice_and_video_note_messages;
    self
  }

  pub fn has_posted_to_profile_stories(
    &mut self,
    has_posted_to_profile_stories: bool,
  ) -> &mut Self {
    self.inner.has_posted_to_profile_stories = has_posted_to_profile_stories;
    self
  }

  pub fn has_sponsored_messages_enabled(
    &mut self,
    has_sponsored_messages_enabled: bool,
  ) -> &mut Self {
    self.inner.has_sponsored_messages_enabled = has_sponsored_messages_enabled;
    self
  }

  pub fn need_phone_number_privacy_exception(
    &mut self,
    need_phone_number_privacy_exception: bool,
  ) -> &mut Self {
    self.inner.need_phone_number_privacy_exception = need_phone_number_privacy_exception;
    self
  }

  pub fn set_chat_background(&mut self, set_chat_background: bool) -> &mut Self {
    self.inner.set_chat_background = set_chat_background;
    self
  }

  pub fn uses_unofficial_app(&mut self, uses_unofficial_app: bool) -> &mut Self {
    self.inner.uses_unofficial_app = uses_unofficial_app;
    self
  }

  pub fn bio<T: AsRef<FormattedText>>(&mut self, bio: T) -> &mut Self {
    self.inner.bio = Some(bio.as_ref().clone());
    self
  }

  pub fn birthdate<T: AsRef<Birthdate>>(&mut self, birthdate: T) -> &mut Self {
    self.inner.birthdate = Some(birthdate.as_ref().clone());
    self
  }

  pub fn personal_chat_id(&mut self, personal_chat_id: i64) -> &mut Self {
    self.inner.personal_chat_id = personal_chat_id;
    self
  }

  pub fn gift_count(&mut self, gift_count: i64) -> &mut Self {
    self.inner.gift_count = gift_count;
    self
  }

  pub fn group_in_common_count(&mut self, group_in_common_count: i64) -> &mut Self {
    self.inner.group_in_common_count = group_in_common_count;
    self
  }

  pub fn incoming_paid_message_star_count(
    &mut self,
    incoming_paid_message_star_count: i64,
  ) -> &mut Self {
    self.inner.incoming_paid_message_star_count = incoming_paid_message_star_count;
    self
  }

  pub fn outgoing_paid_message_star_count(
    &mut self,
    outgoing_paid_message_star_count: i64,
  ) -> &mut Self {
    self.inner.outgoing_paid_message_star_count = outgoing_paid_message_star_count;
    self
  }

  pub fn gift_settings<T: AsRef<GiftSettings>>(&mut self, gift_settings: T) -> &mut Self {
    self.inner.gift_settings = gift_settings.as_ref().clone();
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

  pub fn first_profile_audio<T: AsRef<Audio>>(&mut self, first_profile_audio: T) -> &mut Self {
    self.inner.first_profile_audio = Some(first_profile_audio.as_ref().clone());
    self
  }

  pub fn rating<T: AsRef<UserRating>>(&mut self, rating: T) -> &mut Self {
    self.inner.rating = Some(rating.as_ref().clone());
    self
  }

  pub fn pending_rating<T: AsRef<UserRating>>(&mut self, pending_rating: T) -> &mut Self {
    self.inner.pending_rating = Some(pending_rating.as_ref().clone());
    self
  }

  pub fn pending_rating_date(&mut self, pending_rating_date: i64) -> &mut Self {
    self.inner.pending_rating_date = pending_rating_date;
    self
  }

  pub fn note<T: AsRef<FormattedText>>(&mut self, note: T) -> &mut Self {
    self.inner.note = Some(note.as_ref().clone());
    self
  }

  pub fn ton_wallet_card<T: AsRef<TonWalletCard>>(&mut self, ton_wallet_card: T) -> &mut Self {
    self.inner.ton_wallet_card = Some(ton_wallet_card.as_ref().clone());
    self
  }

  pub fn share_theme<T: AsRef<ProfileShareTheme>>(&mut self, share_theme: T) -> &mut Self {
    self.inner.share_theme = Some(share_theme.as_ref().clone());
    self
  }

  pub fn business_info<T: AsRef<BusinessInfo>>(&mut self, business_info: T) -> &mut Self {
    self.inner.business_info = Some(business_info.as_ref().clone());
    self
  }

  pub fn bot_info<T: AsRef<BotInfo>>(&mut self, bot_info: T) -> &mut Self {
    self.inner.bot_info = Some(bot_info.as_ref().clone());
    self
  }
}

impl AsRef<UserFullInfo> for UserFullInfo {
  fn as_ref(&self) -> &UserFullInfo {
    self
  }
}

impl AsRef<UserFullInfo> for RTDUserFullInfoBuilder {
  fn as_ref(&self) -> &UserFullInfo {
    &self.inner
  }
}
