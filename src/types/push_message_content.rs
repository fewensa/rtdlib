use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains content of a push message notification
pub trait TDPushMessageContent: Debug + RObject {}

/// Contains content of a push message notification
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PushMessageContent {
  #[doc(hidden)]
  _Default(()),
  /// An animation message (GIF-style).
  Animation(PushMessageContentAnimation),
  /// An audio message
  Audio(PushMessageContentAudio),
  /// A newly created basic group
  BasicGroupChatCreate(PushMessageContentBasicGroupChatCreate),
  /// New chat members were invited to a group
  ChatAddMembers(PushMessageContentChatAddMembers),
  /// A chat photo was edited
  ChatChangePhoto(PushMessageContentChatChangePhoto),
  /// A chat title was edited
  ChatChangeTitle(PushMessageContentChatChangeTitle),
  /// A chat member was deleted
  ChatDeleteMember(PushMessageContentChatDeleteMember),
  /// A new member joined the chat via an invite link
  ChatJoinByLink(PushMessageContentChatJoinByLink),
  /// A new member was accepted to the chat by an administrator
  ChatJoinByRequest(PushMessageContentChatJoinByRequest),
  /// A chat background was edited
  ChatSetBackground(PushMessageContentChatSetBackground),
  /// A chat theme was edited
  ChatSetTheme(PushMessageContentChatSetTheme),
  /// A message with a checklist
  Checklist(PushMessageContentChecklist),
  /// Some tasks were added to a checklist
  ChecklistTasksAdded(PushMessageContentChecklistTasksAdded),
  /// Some tasks from a checklist were marked as done or not done
  ChecklistTasksDone(PushMessageContentChecklistTasksDone),
  /// A message with a user contact
  Contact(PushMessageContentContact),
  /// A contact has registered with Telegram
  ContactRegistered(PushMessageContentContactRegistered),
  /// A document message (a general file)
  Document(PushMessageContentDocument),
  /// A message with a game
  Game(PushMessageContentGame),
  /// A new high score was achieved in a game
  GameScore(PushMessageContentGameScore),
  /// A message with a gift
  Gift(PushMessageContentGift),
  /// A message with a giveaway
  Giveaway(PushMessageContentGiveaway),
  /// A general message with hidden content
  Hidden(PushMessageContentHidden),
  /// An invitation of participants to a video chat or live stream
  InviteVideoChatParticipants(PushMessageContentInviteVideoChatParticipants),
  /// A message with an invoice from a bot
  Invoice(PushMessageContentInvoice),
  /// A message with a location
  Location(PushMessageContentLocation),
  /// A media album
  MediaAlbum(PushMessageContentMediaAlbum),
  /// Forwarded messages
  MessageForwards(PushMessageContentMessageForwards),
  /// A message with paid media
  PaidMedia(PushMessageContentPaidMedia),
  /// A photo message
  Photo(PushMessageContentPhoto),
  /// A message with a poll
  Poll(PushMessageContentPoll),
  /// An option was added to a poll
  PollOptionAdded(PushMessageContentPollOptionAdded),
  /// A message with a Telegram Premium gift code created for the user
  PremiumGiftCode(PushMessageContentPremiumGiftCode),
  /// A user in the chat came within proximity alert range from the current user
  ProximityAlertTriggered(PushMessageContentProximityAlertTriggered),
  /// A new recurring payment was made by the current user
  RecurringPayment(PushMessageContentRecurringPayment),
  /// A screenshot of a message in the chat has been taken
  ScreenshotTaken(PushMessageContentScreenshotTaken),
  /// A message with a sticker
  Sticker(PushMessageContentSticker),
  /// A message with a story
  Story(PushMessageContentStory),
  /// A birthdate was suggested to be set
  SuggestBirthdate(PushMessageContentSuggestBirthdate),
  /// A profile photo was suggested to the user
  SuggestProfilePhoto(PushMessageContentSuggestProfilePhoto),
  /// A text message
  Text(PushMessageContentText),
  /// A message with a request from a dApp was received through TON Connect
  TonConnectRequest(PushMessageContentTonConnectRequest),
  /// A message with a TON wallet transfer
  TonWalletTransfer(PushMessageContentTonWalletTransfer),
  /// A message with an upgraded gift
  UpgradedGift(PushMessageContentUpgradedGift),
  /// A video message
  Video(PushMessageContentVideo),
  /// A video chat or live stream has ended
  VideoChatEnded(PushMessageContentVideoChatEnded),
  /// A video chat or live stream was started
  VideoChatStarted(PushMessageContentVideoChatStarted),
  /// A video note message
  VideoNote(PushMessageContentVideoNote),
  /// A voice note message
  VoiceNote(PushMessageContentVoiceNote),
}

impl Default for PushMessageContent {
  fn default() -> Self {
    PushMessageContent::_Default(())
  }
}

impl<'de> Deserialize<'de> for PushMessageContent {
  fn deserialize<D>(deserializer: D) -> Result<PushMessageContent, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PushMessageContent,
      (pushMessageContentAnimation, Animation);
      (pushMessageContentAudio, Audio);
      (pushMessageContentBasicGroupChatCreate, BasicGroupChatCreate);
      (pushMessageContentChatAddMembers, ChatAddMembers);
      (pushMessageContentChatChangePhoto, ChatChangePhoto);
      (pushMessageContentChatChangeTitle, ChatChangeTitle);
      (pushMessageContentChatDeleteMember, ChatDeleteMember);
      (pushMessageContentChatJoinByLink, ChatJoinByLink);
      (pushMessageContentChatJoinByRequest, ChatJoinByRequest);
      (pushMessageContentChatSetBackground, ChatSetBackground);
      (pushMessageContentChatSetTheme, ChatSetTheme);
      (pushMessageContentChecklist, Checklist);
      (pushMessageContentChecklistTasksAdded, ChecklistTasksAdded);
      (pushMessageContentChecklistTasksDone, ChecklistTasksDone);
      (pushMessageContentContact, Contact);
      (pushMessageContentContactRegistered, ContactRegistered);
      (pushMessageContentDocument, Document);
      (pushMessageContentGame, Game);
      (pushMessageContentGameScore, GameScore);
      (pushMessageContentGift, Gift);
      (pushMessageContentGiveaway, Giveaway);
      (pushMessageContentHidden, Hidden);
      (pushMessageContentInviteVideoChatParticipants, InviteVideoChatParticipants);
      (pushMessageContentInvoice, Invoice);
      (pushMessageContentLocation, Location);
      (pushMessageContentMediaAlbum, MediaAlbum);
      (pushMessageContentMessageForwards, MessageForwards);
      (pushMessageContentPaidMedia, PaidMedia);
      (pushMessageContentPhoto, Photo);
      (pushMessageContentPoll, Poll);
      (pushMessageContentPollOptionAdded, PollOptionAdded);
      (pushMessageContentPremiumGiftCode, PremiumGiftCode);
      (pushMessageContentProximityAlertTriggered, ProximityAlertTriggered);
      (pushMessageContentRecurringPayment, RecurringPayment);
      (pushMessageContentScreenshotTaken, ScreenshotTaken);
      (pushMessageContentSticker, Sticker);
      (pushMessageContentStory, Story);
      (pushMessageContentSuggestBirthdate, SuggestBirthdate);
      (pushMessageContentSuggestProfilePhoto, SuggestProfilePhoto);
      (pushMessageContentText, Text);
      (pushMessageContentTonConnectRequest, TonConnectRequest);
      (pushMessageContentTonWalletTransfer, TonWalletTransfer);
      (pushMessageContentUpgradedGift, UpgradedGift);
      (pushMessageContentVideo, Video);
      (pushMessageContentVideoChatEnded, VideoChatEnded);
      (pushMessageContentVideoChatStarted, VideoChatStarted);
      (pushMessageContentVideoNote, VideoNote);
      (pushMessageContentVoiceNote, VoiceNote);

    )(deserializer)
  }
}

impl RObject for PushMessageContent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PushMessageContent::Animation(t) => t.td_name(),
      PushMessageContent::Audio(t) => t.td_name(),
      PushMessageContent::BasicGroupChatCreate(t) => t.td_name(),
      PushMessageContent::ChatAddMembers(t) => t.td_name(),
      PushMessageContent::ChatChangePhoto(t) => t.td_name(),
      PushMessageContent::ChatChangeTitle(t) => t.td_name(),
      PushMessageContent::ChatDeleteMember(t) => t.td_name(),
      PushMessageContent::ChatJoinByLink(t) => t.td_name(),
      PushMessageContent::ChatJoinByRequest(t) => t.td_name(),
      PushMessageContent::ChatSetBackground(t) => t.td_name(),
      PushMessageContent::ChatSetTheme(t) => t.td_name(),
      PushMessageContent::Checklist(t) => t.td_name(),
      PushMessageContent::ChecklistTasksAdded(t) => t.td_name(),
      PushMessageContent::ChecklistTasksDone(t) => t.td_name(),
      PushMessageContent::Contact(t) => t.td_name(),
      PushMessageContent::ContactRegistered(t) => t.td_name(),
      PushMessageContent::Document(t) => t.td_name(),
      PushMessageContent::Game(t) => t.td_name(),
      PushMessageContent::GameScore(t) => t.td_name(),
      PushMessageContent::Gift(t) => t.td_name(),
      PushMessageContent::Giveaway(t) => t.td_name(),
      PushMessageContent::Hidden(t) => t.td_name(),
      PushMessageContent::InviteVideoChatParticipants(t) => t.td_name(),
      PushMessageContent::Invoice(t) => t.td_name(),
      PushMessageContent::Location(t) => t.td_name(),
      PushMessageContent::MediaAlbum(t) => t.td_name(),
      PushMessageContent::MessageForwards(t) => t.td_name(),
      PushMessageContent::PaidMedia(t) => t.td_name(),
      PushMessageContent::Photo(t) => t.td_name(),
      PushMessageContent::Poll(t) => t.td_name(),
      PushMessageContent::PollOptionAdded(t) => t.td_name(),
      PushMessageContent::PremiumGiftCode(t) => t.td_name(),
      PushMessageContent::ProximityAlertTriggered(t) => t.td_name(),
      PushMessageContent::RecurringPayment(t) => t.td_name(),
      PushMessageContent::ScreenshotTaken(t) => t.td_name(),
      PushMessageContent::Sticker(t) => t.td_name(),
      PushMessageContent::Story(t) => t.td_name(),
      PushMessageContent::SuggestBirthdate(t) => t.td_name(),
      PushMessageContent::SuggestProfilePhoto(t) => t.td_name(),
      PushMessageContent::Text(t) => t.td_name(),
      PushMessageContent::TonConnectRequest(t) => t.td_name(),
      PushMessageContent::TonWalletTransfer(t) => t.td_name(),
      PushMessageContent::UpgradedGift(t) => t.td_name(),
      PushMessageContent::Video(t) => t.td_name(),
      PushMessageContent::VideoChatEnded(t) => t.td_name(),
      PushMessageContent::VideoChatStarted(t) => t.td_name(),
      PushMessageContent::VideoNote(t) => t.td_name(),
      PushMessageContent::VoiceNote(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PushMessageContent::Animation(t) => t.extra(),
      PushMessageContent::Audio(t) => t.extra(),
      PushMessageContent::BasicGroupChatCreate(t) => t.extra(),
      PushMessageContent::ChatAddMembers(t) => t.extra(),
      PushMessageContent::ChatChangePhoto(t) => t.extra(),
      PushMessageContent::ChatChangeTitle(t) => t.extra(),
      PushMessageContent::ChatDeleteMember(t) => t.extra(),
      PushMessageContent::ChatJoinByLink(t) => t.extra(),
      PushMessageContent::ChatJoinByRequest(t) => t.extra(),
      PushMessageContent::ChatSetBackground(t) => t.extra(),
      PushMessageContent::ChatSetTheme(t) => t.extra(),
      PushMessageContent::Checklist(t) => t.extra(),
      PushMessageContent::ChecklistTasksAdded(t) => t.extra(),
      PushMessageContent::ChecklistTasksDone(t) => t.extra(),
      PushMessageContent::Contact(t) => t.extra(),
      PushMessageContent::ContactRegistered(t) => t.extra(),
      PushMessageContent::Document(t) => t.extra(),
      PushMessageContent::Game(t) => t.extra(),
      PushMessageContent::GameScore(t) => t.extra(),
      PushMessageContent::Gift(t) => t.extra(),
      PushMessageContent::Giveaway(t) => t.extra(),
      PushMessageContent::Hidden(t) => t.extra(),
      PushMessageContent::InviteVideoChatParticipants(t) => t.extra(),
      PushMessageContent::Invoice(t) => t.extra(),
      PushMessageContent::Location(t) => t.extra(),
      PushMessageContent::MediaAlbum(t) => t.extra(),
      PushMessageContent::MessageForwards(t) => t.extra(),
      PushMessageContent::PaidMedia(t) => t.extra(),
      PushMessageContent::Photo(t) => t.extra(),
      PushMessageContent::Poll(t) => t.extra(),
      PushMessageContent::PollOptionAdded(t) => t.extra(),
      PushMessageContent::PremiumGiftCode(t) => t.extra(),
      PushMessageContent::ProximityAlertTriggered(t) => t.extra(),
      PushMessageContent::RecurringPayment(t) => t.extra(),
      PushMessageContent::ScreenshotTaken(t) => t.extra(),
      PushMessageContent::Sticker(t) => t.extra(),
      PushMessageContent::Story(t) => t.extra(),
      PushMessageContent::SuggestBirthdate(t) => t.extra(),
      PushMessageContent::SuggestProfilePhoto(t) => t.extra(),
      PushMessageContent::Text(t) => t.extra(),
      PushMessageContent::TonConnectRequest(t) => t.extra(),
      PushMessageContent::TonWalletTransfer(t) => t.extra(),
      PushMessageContent::UpgradedGift(t) => t.extra(),
      PushMessageContent::Video(t) => t.extra(),
      PushMessageContent::VideoChatEnded(t) => t.extra(),
      PushMessageContent::VideoChatStarted(t) => t.extra(),
      PushMessageContent::VideoNote(t) => t.extra(),
      PushMessageContent::VoiceNote(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PushMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PushMessageContent::_Default(_))
  }

  pub fn is_animation(&self) -> bool {
    matches!(self, PushMessageContent::Animation(_))
  }
  pub fn is_audio(&self) -> bool {
    matches!(self, PushMessageContent::Audio(_))
  }
  pub fn is_basic_group_chat_create(&self) -> bool {
    matches!(self, PushMessageContent::BasicGroupChatCreate(_))
  }
  pub fn is_chat_add_members(&self) -> bool {
    matches!(self, PushMessageContent::ChatAddMembers(_))
  }
  pub fn is_chat_change_photo(&self) -> bool {
    matches!(self, PushMessageContent::ChatChangePhoto(_))
  }
  pub fn is_chat_change_title(&self) -> bool {
    matches!(self, PushMessageContent::ChatChangeTitle(_))
  }
  pub fn is_chat_delete_member(&self) -> bool {
    matches!(self, PushMessageContent::ChatDeleteMember(_))
  }
  pub fn is_chat_join_by_link(&self) -> bool {
    matches!(self, PushMessageContent::ChatJoinByLink(_))
  }
  pub fn is_chat_join_by_request(&self) -> bool {
    matches!(self, PushMessageContent::ChatJoinByRequest(_))
  }
  pub fn is_chat_set_background(&self) -> bool {
    matches!(self, PushMessageContent::ChatSetBackground(_))
  }
  pub fn is_chat_set_theme(&self) -> bool {
    matches!(self, PushMessageContent::ChatSetTheme(_))
  }
  pub fn is_checklist(&self) -> bool {
    matches!(self, PushMessageContent::Checklist(_))
  }
  pub fn is_checklist_tasks_added(&self) -> bool {
    matches!(self, PushMessageContent::ChecklistTasksAdded(_))
  }
  pub fn is_checklist_tasks_done(&self) -> bool {
    matches!(self, PushMessageContent::ChecklistTasksDone(_))
  }
  pub fn is_contact(&self) -> bool {
    matches!(self, PushMessageContent::Contact(_))
  }
  pub fn is_contact_registered(&self) -> bool {
    matches!(self, PushMessageContent::ContactRegistered(_))
  }
  pub fn is_document(&self) -> bool {
    matches!(self, PushMessageContent::Document(_))
  }
  pub fn is_game(&self) -> bool {
    matches!(self, PushMessageContent::Game(_))
  }
  pub fn is_game_score(&self) -> bool {
    matches!(self, PushMessageContent::GameScore(_))
  }
  pub fn is_gift(&self) -> bool {
    matches!(self, PushMessageContent::Gift(_))
  }
  pub fn is_giveaway(&self) -> bool {
    matches!(self, PushMessageContent::Giveaway(_))
  }
  pub fn is_hidden(&self) -> bool {
    matches!(self, PushMessageContent::Hidden(_))
  }
  pub fn is_invite_video_chat_participants(&self) -> bool {
    matches!(self, PushMessageContent::InviteVideoChatParticipants(_))
  }
  pub fn is_invoice(&self) -> bool {
    matches!(self, PushMessageContent::Invoice(_))
  }
  pub fn is_location(&self) -> bool {
    matches!(self, PushMessageContent::Location(_))
  }
  pub fn is_media_album(&self) -> bool {
    matches!(self, PushMessageContent::MediaAlbum(_))
  }
  pub fn is_message_forwards(&self) -> bool {
    matches!(self, PushMessageContent::MessageForwards(_))
  }
  pub fn is_paid_media(&self) -> bool {
    matches!(self, PushMessageContent::PaidMedia(_))
  }
  pub fn is_photo(&self) -> bool {
    matches!(self, PushMessageContent::Photo(_))
  }
  pub fn is_poll(&self) -> bool {
    matches!(self, PushMessageContent::Poll(_))
  }
  pub fn is_poll_option_added(&self) -> bool {
    matches!(self, PushMessageContent::PollOptionAdded(_))
  }
  pub fn is_premium_gift_code(&self) -> bool {
    matches!(self, PushMessageContent::PremiumGiftCode(_))
  }
  pub fn is_proximity_alert_triggered(&self) -> bool {
    matches!(self, PushMessageContent::ProximityAlertTriggered(_))
  }
  pub fn is_recurring_payment(&self) -> bool {
    matches!(self, PushMessageContent::RecurringPayment(_))
  }
  pub fn is_screenshot_taken(&self) -> bool {
    matches!(self, PushMessageContent::ScreenshotTaken(_))
  }
  pub fn is_sticker(&self) -> bool {
    matches!(self, PushMessageContent::Sticker(_))
  }
  pub fn is_story(&self) -> bool {
    matches!(self, PushMessageContent::Story(_))
  }
  pub fn is_suggest_birthdate(&self) -> bool {
    matches!(self, PushMessageContent::SuggestBirthdate(_))
  }
  pub fn is_suggest_profile_photo(&self) -> bool {
    matches!(self, PushMessageContent::SuggestProfilePhoto(_))
  }
  pub fn is_text(&self) -> bool {
    matches!(self, PushMessageContent::Text(_))
  }
  pub fn is_ton_connect_request(&self) -> bool {
    matches!(self, PushMessageContent::TonConnectRequest(_))
  }
  pub fn is_ton_wallet_transfer(&self) -> bool {
    matches!(self, PushMessageContent::TonWalletTransfer(_))
  }
  pub fn is_upgraded_gift(&self) -> bool {
    matches!(self, PushMessageContent::UpgradedGift(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, PushMessageContent::Video(_))
  }
  pub fn is_video_chat_ended(&self) -> bool {
    matches!(self, PushMessageContent::VideoChatEnded(_))
  }
  pub fn is_video_chat_started(&self) -> bool {
    matches!(self, PushMessageContent::VideoChatStarted(_))
  }
  pub fn is_video_note(&self) -> bool {
    matches!(self, PushMessageContent::VideoNote(_))
  }
  pub fn is_voice_note(&self) -> bool {
    matches!(self, PushMessageContent::VoiceNote(_))
  }

  pub fn on_animation<F: FnOnce(&PushMessageContentAnimation)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Animation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_audio<F: FnOnce(&PushMessageContentAudio)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Audio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_basic_group_chat_create<F: FnOnce(&PushMessageContentBasicGroupChatCreate)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::BasicGroupChatCreate(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_add_members<F: FnOnce(&PushMessageContentChatAddMembers)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::ChatAddMembers(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_change_photo<F: FnOnce(&PushMessageContentChatChangePhoto)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ChatChangePhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_change_title<F: FnOnce(&PushMessageContentChatChangeTitle)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ChatChangeTitle(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_delete_member<F: FnOnce(&PushMessageContentChatDeleteMember)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ChatDeleteMember(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_join_by_link<F: FnOnce(&PushMessageContentChatJoinByLink)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ChatJoinByLink(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_join_by_request<F: FnOnce(&PushMessageContentChatJoinByRequest)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ChatJoinByRequest(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_set_background<F: FnOnce(&PushMessageContentChatSetBackground)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ChatSetBackground(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_set_theme<F: FnOnce(&PushMessageContentChatSetTheme)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::ChatSetTheme(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_checklist<F: FnOnce(&PushMessageContentChecklist)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Checklist(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_checklist_tasks_added<F: FnOnce(&PushMessageContentChecklistTasksAdded)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ChecklistTasksAdded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_checklist_tasks_done<F: FnOnce(&PushMessageContentChecklistTasksDone)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ChecklistTasksDone(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_contact<F: FnOnce(&PushMessageContentContact)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Contact(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_contact_registered<F: FnOnce(&PushMessageContentContactRegistered)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ContactRegistered(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_document<F: FnOnce(&PushMessageContentDocument)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Document(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_game<F: FnOnce(&PushMessageContentGame)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Game(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_game_score<F: FnOnce(&PushMessageContentGameScore)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::GameScore(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift<F: FnOnce(&PushMessageContentGift)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Gift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_giveaway<F: FnOnce(&PushMessageContentGiveaway)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Giveaway(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_hidden<F: FnOnce(&PushMessageContentHidden)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Hidden(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_invite_video_chat_participants<
    F: FnOnce(&PushMessageContentInviteVideoChatParticipants),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::InviteVideoChatParticipants(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_invoice<F: FnOnce(&PushMessageContentInvoice)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Invoice(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_location<F: FnOnce(&PushMessageContentLocation)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Location(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_media_album<F: FnOnce(&PushMessageContentMediaAlbum)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::MediaAlbum(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_forwards<F: FnOnce(&PushMessageContentMessageForwards)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::MessageForwards(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid_media<F: FnOnce(&PushMessageContentPaidMedia)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::PaidMedia(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_photo<F: FnOnce(&PushMessageContentPhoto)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_poll<F: FnOnce(&PushMessageContentPoll)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Poll(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_poll_option_added<F: FnOnce(&PushMessageContentPollOptionAdded)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::PollOptionAdded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_gift_code<F: FnOnce(&PushMessageContentPremiumGiftCode)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::PremiumGiftCode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_proximity_alert_triggered<F: FnOnce(&PushMessageContentProximityAlertTriggered)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ProximityAlertTriggered(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_recurring_payment<F: FnOnce(&PushMessageContentRecurringPayment)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::RecurringPayment(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_screenshot_taken<F: FnOnce(&PushMessageContentScreenshotTaken)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::ScreenshotTaken(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sticker<F: FnOnce(&PushMessageContentSticker)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Sticker(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story<F: FnOnce(&PushMessageContentStory)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Story(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_suggest_birthdate<F: FnOnce(&PushMessageContentSuggestBirthdate)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::SuggestBirthdate(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_suggest_profile_photo<F: FnOnce(&PushMessageContentSuggestProfilePhoto)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::SuggestProfilePhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text<F: FnOnce(&PushMessageContentText)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Text(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ton_connect_request<F: FnOnce(&PushMessageContentTonConnectRequest)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::TonConnectRequest(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ton_wallet_transfer<F: FnOnce(&PushMessageContentTonWalletTransfer)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::TonWalletTransfer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift<F: FnOnce(&PushMessageContentUpgradedGift)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::UpgradedGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&PushMessageContentVideo)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::Video(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_chat_ended<F: FnOnce(&PushMessageContentVideoChatEnded)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::VideoChatEnded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_chat_started<F: FnOnce(&PushMessageContentVideoChatStarted)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PushMessageContent::VideoChatStarted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_note<F: FnOnce(&PushMessageContentVideoNote)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::VideoNote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_voice_note<F: FnOnce(&PushMessageContentVoiceNote)>(&self, fnc: F) -> &Self {
    if let PushMessageContent::VoiceNote(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_animation(&self) -> Option<&PushMessageContentAnimation> {
    if let PushMessageContent::Animation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_audio(&self) -> Option<&PushMessageContentAudio> {
    if let PushMessageContent::Audio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_basic_group_chat_create(&self) -> Option<&PushMessageContentBasicGroupChatCreate> {
    if let PushMessageContent::BasicGroupChatCreate(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_add_members(&self) -> Option<&PushMessageContentChatAddMembers> {
    if let PushMessageContent::ChatAddMembers(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_change_photo(&self) -> Option<&PushMessageContentChatChangePhoto> {
    if let PushMessageContent::ChatChangePhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_change_title(&self) -> Option<&PushMessageContentChatChangeTitle> {
    if let PushMessageContent::ChatChangeTitle(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_delete_member(&self) -> Option<&PushMessageContentChatDeleteMember> {
    if let PushMessageContent::ChatDeleteMember(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_join_by_link(&self) -> Option<&PushMessageContentChatJoinByLink> {
    if let PushMessageContent::ChatJoinByLink(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_join_by_request(&self) -> Option<&PushMessageContentChatJoinByRequest> {
    if let PushMessageContent::ChatJoinByRequest(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_set_background(&self) -> Option<&PushMessageContentChatSetBackground> {
    if let PushMessageContent::ChatSetBackground(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_set_theme(&self) -> Option<&PushMessageContentChatSetTheme> {
    if let PushMessageContent::ChatSetTheme(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_checklist(&self) -> Option<&PushMessageContentChecklist> {
    if let PushMessageContent::Checklist(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_checklist_tasks_added(&self) -> Option<&PushMessageContentChecklistTasksAdded> {
    if let PushMessageContent::ChecklistTasksAdded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_checklist_tasks_done(&self) -> Option<&PushMessageContentChecklistTasksDone> {
    if let PushMessageContent::ChecklistTasksDone(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_contact(&self) -> Option<&PushMessageContentContact> {
    if let PushMessageContent::Contact(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_contact_registered(&self) -> Option<&PushMessageContentContactRegistered> {
    if let PushMessageContent::ContactRegistered(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_document(&self) -> Option<&PushMessageContentDocument> {
    if let PushMessageContent::Document(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_game(&self) -> Option<&PushMessageContentGame> {
    if let PushMessageContent::Game(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_game_score(&self) -> Option<&PushMessageContentGameScore> {
    if let PushMessageContent::GameScore(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift(&self) -> Option<&PushMessageContentGift> {
    if let PushMessageContent::Gift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_giveaway(&self) -> Option<&PushMessageContentGiveaway> {
    if let PushMessageContent::Giveaway(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_hidden(&self) -> Option<&PushMessageContentHidden> {
    if let PushMessageContent::Hidden(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_invite_video_chat_participants(
    &self,
  ) -> Option<&PushMessageContentInviteVideoChatParticipants> {
    if let PushMessageContent::InviteVideoChatParticipants(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_invoice(&self) -> Option<&PushMessageContentInvoice> {
    if let PushMessageContent::Invoice(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_location(&self) -> Option<&PushMessageContentLocation> {
    if let PushMessageContent::Location(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_media_album(&self) -> Option<&PushMessageContentMediaAlbum> {
    if let PushMessageContent::MediaAlbum(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_forwards(&self) -> Option<&PushMessageContentMessageForwards> {
    if let PushMessageContent::MessageForwards(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid_media(&self) -> Option<&PushMessageContentPaidMedia> {
    if let PushMessageContent::PaidMedia(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_photo(&self) -> Option<&PushMessageContentPhoto> {
    if let PushMessageContent::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_poll(&self) -> Option<&PushMessageContentPoll> {
    if let PushMessageContent::Poll(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_poll_option_added(&self) -> Option<&PushMessageContentPollOptionAdded> {
    if let PushMessageContent::PollOptionAdded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_gift_code(&self) -> Option<&PushMessageContentPremiumGiftCode> {
    if let PushMessageContent::PremiumGiftCode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_proximity_alert_triggered(&self) -> Option<&PushMessageContentProximityAlertTriggered> {
    if let PushMessageContent::ProximityAlertTriggered(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_recurring_payment(&self) -> Option<&PushMessageContentRecurringPayment> {
    if let PushMessageContent::RecurringPayment(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_screenshot_taken(&self) -> Option<&PushMessageContentScreenshotTaken> {
    if let PushMessageContent::ScreenshotTaken(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sticker(&self) -> Option<&PushMessageContentSticker> {
    if let PushMessageContent::Sticker(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story(&self) -> Option<&PushMessageContentStory> {
    if let PushMessageContent::Story(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_suggest_birthdate(&self) -> Option<&PushMessageContentSuggestBirthdate> {
    if let PushMessageContent::SuggestBirthdate(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_suggest_profile_photo(&self) -> Option<&PushMessageContentSuggestProfilePhoto> {
    if let PushMessageContent::SuggestProfilePhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text(&self) -> Option<&PushMessageContentText> {
    if let PushMessageContent::Text(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ton_connect_request(&self) -> Option<&PushMessageContentTonConnectRequest> {
    if let PushMessageContent::TonConnectRequest(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ton_wallet_transfer(&self) -> Option<&PushMessageContentTonWalletTransfer> {
    if let PushMessageContent::TonWalletTransfer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift(&self) -> Option<&PushMessageContentUpgradedGift> {
    if let PushMessageContent::UpgradedGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&PushMessageContentVideo> {
    if let PushMessageContent::Video(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_chat_ended(&self) -> Option<&PushMessageContentVideoChatEnded> {
    if let PushMessageContent::VideoChatEnded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_chat_started(&self) -> Option<&PushMessageContentVideoChatStarted> {
    if let PushMessageContent::VideoChatStarted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_note(&self) -> Option<&PushMessageContentVideoNote> {
    if let PushMessageContent::VideoNote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_voice_note(&self) -> Option<&PushMessageContentVoiceNote> {
    if let PushMessageContent::VoiceNote(t) = self {
      return Some(t);
    }
    None
  }

  pub fn animation<T: AsRef<PushMessageContentAnimation>>(t: T) -> Self {
    PushMessageContent::Animation(t.as_ref().clone())
  }

  pub fn audio<T: AsRef<PushMessageContentAudio>>(t: T) -> Self {
    PushMessageContent::Audio(t.as_ref().clone())
  }

  pub fn basic_group_chat_create<T: AsRef<PushMessageContentBasicGroupChatCreate>>(t: T) -> Self {
    PushMessageContent::BasicGroupChatCreate(t.as_ref().clone())
  }

  pub fn chat_add_members<T: AsRef<PushMessageContentChatAddMembers>>(t: T) -> Self {
    PushMessageContent::ChatAddMembers(t.as_ref().clone())
  }

  pub fn chat_change_photo<T: AsRef<PushMessageContentChatChangePhoto>>(t: T) -> Self {
    PushMessageContent::ChatChangePhoto(t.as_ref().clone())
  }

  pub fn chat_change_title<T: AsRef<PushMessageContentChatChangeTitle>>(t: T) -> Self {
    PushMessageContent::ChatChangeTitle(t.as_ref().clone())
  }

  pub fn chat_delete_member<T: AsRef<PushMessageContentChatDeleteMember>>(t: T) -> Self {
    PushMessageContent::ChatDeleteMember(t.as_ref().clone())
  }

  pub fn chat_join_by_link<T: AsRef<PushMessageContentChatJoinByLink>>(t: T) -> Self {
    PushMessageContent::ChatJoinByLink(t.as_ref().clone())
  }

  pub fn chat_join_by_request<T: AsRef<PushMessageContentChatJoinByRequest>>(t: T) -> Self {
    PushMessageContent::ChatJoinByRequest(t.as_ref().clone())
  }

  pub fn chat_set_background<T: AsRef<PushMessageContentChatSetBackground>>(t: T) -> Self {
    PushMessageContent::ChatSetBackground(t.as_ref().clone())
  }

  pub fn chat_set_theme<T: AsRef<PushMessageContentChatSetTheme>>(t: T) -> Self {
    PushMessageContent::ChatSetTheme(t.as_ref().clone())
  }

  pub fn checklist<T: AsRef<PushMessageContentChecklist>>(t: T) -> Self {
    PushMessageContent::Checklist(t.as_ref().clone())
  }

  pub fn checklist_tasks_added<T: AsRef<PushMessageContentChecklistTasksAdded>>(t: T) -> Self {
    PushMessageContent::ChecklistTasksAdded(t.as_ref().clone())
  }

  pub fn checklist_tasks_done<T: AsRef<PushMessageContentChecklistTasksDone>>(t: T) -> Self {
    PushMessageContent::ChecklistTasksDone(t.as_ref().clone())
  }

  pub fn contact<T: AsRef<PushMessageContentContact>>(t: T) -> Self {
    PushMessageContent::Contact(t.as_ref().clone())
  }

  pub fn contact_registered<T: AsRef<PushMessageContentContactRegistered>>(t: T) -> Self {
    PushMessageContent::ContactRegistered(t.as_ref().clone())
  }

  pub fn document<T: AsRef<PushMessageContentDocument>>(t: T) -> Self {
    PushMessageContent::Document(t.as_ref().clone())
  }

  pub fn game<T: AsRef<PushMessageContentGame>>(t: T) -> Self {
    PushMessageContent::Game(t.as_ref().clone())
  }

  pub fn game_score<T: AsRef<PushMessageContentGameScore>>(t: T) -> Self {
    PushMessageContent::GameScore(t.as_ref().clone())
  }

  pub fn gift<T: AsRef<PushMessageContentGift>>(t: T) -> Self {
    PushMessageContent::Gift(t.as_ref().clone())
  }

  pub fn giveaway<T: AsRef<PushMessageContentGiveaway>>(t: T) -> Self {
    PushMessageContent::Giveaway(t.as_ref().clone())
  }

  pub fn hidden<T: AsRef<PushMessageContentHidden>>(t: T) -> Self {
    PushMessageContent::Hidden(t.as_ref().clone())
  }

  pub fn invite_video_chat_participants<T: AsRef<PushMessageContentInviteVideoChatParticipants>>(
    t: T,
  ) -> Self {
    PushMessageContent::InviteVideoChatParticipants(t.as_ref().clone())
  }

  pub fn invoice<T: AsRef<PushMessageContentInvoice>>(t: T) -> Self {
    PushMessageContent::Invoice(t.as_ref().clone())
  }

  pub fn location<T: AsRef<PushMessageContentLocation>>(t: T) -> Self {
    PushMessageContent::Location(t.as_ref().clone())
  }

  pub fn media_album<T: AsRef<PushMessageContentMediaAlbum>>(t: T) -> Self {
    PushMessageContent::MediaAlbum(t.as_ref().clone())
  }

  pub fn message_forwards<T: AsRef<PushMessageContentMessageForwards>>(t: T) -> Self {
    PushMessageContent::MessageForwards(t.as_ref().clone())
  }

  pub fn paid_media<T: AsRef<PushMessageContentPaidMedia>>(t: T) -> Self {
    PushMessageContent::PaidMedia(t.as_ref().clone())
  }

  pub fn photo<T: AsRef<PushMessageContentPhoto>>(t: T) -> Self {
    PushMessageContent::Photo(t.as_ref().clone())
  }

  pub fn poll<T: AsRef<PushMessageContentPoll>>(t: T) -> Self {
    PushMessageContent::Poll(t.as_ref().clone())
  }

  pub fn poll_option_added<T: AsRef<PushMessageContentPollOptionAdded>>(t: T) -> Self {
    PushMessageContent::PollOptionAdded(t.as_ref().clone())
  }

  pub fn premium_gift_code<T: AsRef<PushMessageContentPremiumGiftCode>>(t: T) -> Self {
    PushMessageContent::PremiumGiftCode(t.as_ref().clone())
  }

  pub fn proximity_alert_triggered<T: AsRef<PushMessageContentProximityAlertTriggered>>(
    t: T,
  ) -> Self {
    PushMessageContent::ProximityAlertTriggered(t.as_ref().clone())
  }

  pub fn recurring_payment<T: AsRef<PushMessageContentRecurringPayment>>(t: T) -> Self {
    PushMessageContent::RecurringPayment(t.as_ref().clone())
  }

  pub fn screenshot_taken<T: AsRef<PushMessageContentScreenshotTaken>>(t: T) -> Self {
    PushMessageContent::ScreenshotTaken(t.as_ref().clone())
  }

  pub fn sticker<T: AsRef<PushMessageContentSticker>>(t: T) -> Self {
    PushMessageContent::Sticker(t.as_ref().clone())
  }

  pub fn story<T: AsRef<PushMessageContentStory>>(t: T) -> Self {
    PushMessageContent::Story(t.as_ref().clone())
  }

  pub fn suggest_birthdate<T: AsRef<PushMessageContentSuggestBirthdate>>(t: T) -> Self {
    PushMessageContent::SuggestBirthdate(t.as_ref().clone())
  }

  pub fn suggest_profile_photo<T: AsRef<PushMessageContentSuggestProfilePhoto>>(t: T) -> Self {
    PushMessageContent::SuggestProfilePhoto(t.as_ref().clone())
  }

  pub fn text<T: AsRef<PushMessageContentText>>(t: T) -> Self {
    PushMessageContent::Text(t.as_ref().clone())
  }

  pub fn ton_connect_request<T: AsRef<PushMessageContentTonConnectRequest>>(t: T) -> Self {
    PushMessageContent::TonConnectRequest(t.as_ref().clone())
  }

  pub fn ton_wallet_transfer<T: AsRef<PushMessageContentTonWalletTransfer>>(t: T) -> Self {
    PushMessageContent::TonWalletTransfer(t.as_ref().clone())
  }

  pub fn upgraded_gift<T: AsRef<PushMessageContentUpgradedGift>>(t: T) -> Self {
    PushMessageContent::UpgradedGift(t.as_ref().clone())
  }

  pub fn video<T: AsRef<PushMessageContentVideo>>(t: T) -> Self {
    PushMessageContent::Video(t.as_ref().clone())
  }

  pub fn video_chat_ended<T: AsRef<PushMessageContentVideoChatEnded>>(t: T) -> Self {
    PushMessageContent::VideoChatEnded(t.as_ref().clone())
  }

  pub fn video_chat_started<T: AsRef<PushMessageContentVideoChatStarted>>(t: T) -> Self {
    PushMessageContent::VideoChatStarted(t.as_ref().clone())
  }

  pub fn video_note<T: AsRef<PushMessageContentVideoNote>>(t: T) -> Self {
    PushMessageContent::VideoNote(t.as_ref().clone())
  }

  pub fn voice_note<T: AsRef<PushMessageContentVoiceNote>>(t: T) -> Self {
    PushMessageContent::VoiceNote(t.as_ref().clone())
  }
}

impl AsRef<PushMessageContent> for PushMessageContent {
  fn as_ref(&self) -> &PushMessageContent {
    self
  }
}

/// An animation message (GIF-style).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message content; may be null
  animation: Option<Animation>,
  /// Animation caption
  caption: String,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentAnimation {}

impl PushMessageContentAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentAnimationBuilder {
    let inner = PushMessageContentAnimation {
      td_name: "pushMessageContentAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &Option<Animation> {
    &self.animation
  }

  pub fn caption(&self) -> &String {
    &self.caption
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentAnimationBuilder {
  inner: PushMessageContentAnimation,
}

impl RTDPushMessageContentAnimationBuilder {
  pub fn build(&self) -> PushMessageContentAnimation {
    self.inner.clone()
  }

  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = Some(animation.as_ref().clone());
    self
  }

  pub fn caption<T: AsRef<str>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().to_string();
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentAnimation> for PushMessageContentAnimation {
  fn as_ref(&self) -> &PushMessageContentAnimation {
    self
  }
}

impl AsRef<PushMessageContentAnimation> for RTDPushMessageContentAnimationBuilder {
  fn as_ref(&self) -> &PushMessageContentAnimation {
    &self.inner
  }
}

/// An audio message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message content; may be null
  audio: Option<Audio>,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentAudio {}

impl PushMessageContentAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentAudioBuilder {
    let inner = PushMessageContentAudio {
      td_name: "pushMessageContentAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentAudioBuilder { inner }
  }

  pub fn audio(&self) -> &Option<Audio> {
    &self.audio
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentAudioBuilder {
  inner: PushMessageContentAudio,
}

impl RTDPushMessageContentAudioBuilder {
  pub fn build(&self) -> PushMessageContentAudio {
    self.inner.clone()
  }

  pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = Some(audio.as_ref().clone());
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentAudio> for PushMessageContentAudio {
  fn as_ref(&self) -> &PushMessageContentAudio {
    self
  }
}

impl AsRef<PushMessageContentAudio> for RTDPushMessageContentAudioBuilder {
  fn as_ref(&self) -> &PushMessageContentAudio {
    &self.inner
  }
}

/// A newly created basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentBasicGroupChatCreate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PushMessageContentBasicGroupChatCreate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentBasicGroupChatCreate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentBasicGroupChatCreate {}

impl PushMessageContentBasicGroupChatCreate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentBasicGroupChatCreateBuilder {
    let inner = PushMessageContentBasicGroupChatCreate {
      td_name: "pushMessageContentBasicGroupChatCreate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPushMessageContentBasicGroupChatCreateBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentBasicGroupChatCreateBuilder {
  inner: PushMessageContentBasicGroupChatCreate,
}

impl RTDPushMessageContentBasicGroupChatCreateBuilder {
  pub fn build(&self) -> PushMessageContentBasicGroupChatCreate {
    self.inner.clone()
  }
}

impl AsRef<PushMessageContentBasicGroupChatCreate> for PushMessageContentBasicGroupChatCreate {
  fn as_ref(&self) -> &PushMessageContentBasicGroupChatCreate {
    self
  }
}

impl AsRef<PushMessageContentBasicGroupChatCreate>
  for RTDPushMessageContentBasicGroupChatCreateBuilder
{
  fn as_ref(&self) -> &PushMessageContentBasicGroupChatCreate {
    &self.inner
  }
}

/// New chat members were invited to a group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatAddMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the added member
  member_name: String,
  /// True, if the current user was added to the group
  is_current_user: bool,
  /// True, if the user has returned to the group themselves
  is_returned: bool,
}

impl RObject for PushMessageContentChatAddMembers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChatAddMembers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChatAddMembers {}

impl PushMessageContentChatAddMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChatAddMembersBuilder {
    let inner = PushMessageContentChatAddMembers {
      td_name: "pushMessageContentChatAddMembers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentChatAddMembersBuilder { inner }
  }

  pub fn member_name(&self) -> &String {
    &self.member_name
  }

  pub fn is_current_user(&self) -> bool {
    self.is_current_user
  }

  pub fn is_returned(&self) -> bool {
    self.is_returned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatAddMembersBuilder {
  inner: PushMessageContentChatAddMembers,
}

impl RTDPushMessageContentChatAddMembersBuilder {
  pub fn build(&self) -> PushMessageContentChatAddMembers {
    self.inner.clone()
  }

  pub fn member_name<T: AsRef<str>>(&mut self, member_name: T) -> &mut Self {
    self.inner.member_name = member_name.as_ref().to_string();
    self
  }

  pub fn is_current_user(&mut self, is_current_user: bool) -> &mut Self {
    self.inner.is_current_user = is_current_user;
    self
  }

  pub fn is_returned(&mut self, is_returned: bool) -> &mut Self {
    self.inner.is_returned = is_returned;
    self
  }
}

impl AsRef<PushMessageContentChatAddMembers> for PushMessageContentChatAddMembers {
  fn as_ref(&self) -> &PushMessageContentChatAddMembers {
    self
  }
}

impl AsRef<PushMessageContentChatAddMembers> for RTDPushMessageContentChatAddMembersBuilder {
  fn as_ref(&self) -> &PushMessageContentChatAddMembers {
    &self.inner
  }
}

/// A chat photo was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatChangePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PushMessageContentChatChangePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChatChangePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChatChangePhoto {}

impl PushMessageContentChatChangePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChatChangePhotoBuilder {
    let inner = PushMessageContentChatChangePhoto {
      td_name: "pushMessageContentChatChangePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPushMessageContentChatChangePhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatChangePhotoBuilder {
  inner: PushMessageContentChatChangePhoto,
}

impl RTDPushMessageContentChatChangePhotoBuilder {
  pub fn build(&self) -> PushMessageContentChatChangePhoto {
    self.inner.clone()
  }
}

impl AsRef<PushMessageContentChatChangePhoto> for PushMessageContentChatChangePhoto {
  fn as_ref(&self) -> &PushMessageContentChatChangePhoto {
    self
  }
}

impl AsRef<PushMessageContentChatChangePhoto> for RTDPushMessageContentChatChangePhotoBuilder {
  fn as_ref(&self) -> &PushMessageContentChatChangePhoto {
    &self.inner
  }
}

/// A chat title was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatChangeTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New chat title
  title: String,
}

impl RObject for PushMessageContentChatChangeTitle {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChatChangeTitle"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChatChangeTitle {}

impl PushMessageContentChatChangeTitle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChatChangeTitleBuilder {
    let inner = PushMessageContentChatChangeTitle {
      td_name: "pushMessageContentChatChangeTitle".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentChatChangeTitleBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatChangeTitleBuilder {
  inner: PushMessageContentChatChangeTitle,
}

impl RTDPushMessageContentChatChangeTitleBuilder {
  pub fn build(&self) -> PushMessageContentChatChangeTitle {
    self.inner.clone()
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }
}

impl AsRef<PushMessageContentChatChangeTitle> for PushMessageContentChatChangeTitle {
  fn as_ref(&self) -> &PushMessageContentChatChangeTitle {
    self
  }
}

impl AsRef<PushMessageContentChatChangeTitle> for RTDPushMessageContentChatChangeTitleBuilder {
  fn as_ref(&self) -> &PushMessageContentChatChangeTitle {
    &self.inner
  }
}

/// A chat member was deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatDeleteMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the deleted member
  member_name: String,
  /// True, if the current user was deleted from the group
  is_current_user: bool,
  /// True, if the user has left the group themselves
  is_left: bool,
}

impl RObject for PushMessageContentChatDeleteMember {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChatDeleteMember"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChatDeleteMember {}

impl PushMessageContentChatDeleteMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChatDeleteMemberBuilder {
    let inner = PushMessageContentChatDeleteMember {
      td_name: "pushMessageContentChatDeleteMember".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentChatDeleteMemberBuilder { inner }
  }

  pub fn member_name(&self) -> &String {
    &self.member_name
  }

  pub fn is_current_user(&self) -> bool {
    self.is_current_user
  }

  pub fn is_left(&self) -> bool {
    self.is_left
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatDeleteMemberBuilder {
  inner: PushMessageContentChatDeleteMember,
}

impl RTDPushMessageContentChatDeleteMemberBuilder {
  pub fn build(&self) -> PushMessageContentChatDeleteMember {
    self.inner.clone()
  }

  pub fn member_name<T: AsRef<str>>(&mut self, member_name: T) -> &mut Self {
    self.inner.member_name = member_name.as_ref().to_string();
    self
  }

  pub fn is_current_user(&mut self, is_current_user: bool) -> &mut Self {
    self.inner.is_current_user = is_current_user;
    self
  }

  pub fn is_left(&mut self, is_left: bool) -> &mut Self {
    self.inner.is_left = is_left;
    self
  }
}

impl AsRef<PushMessageContentChatDeleteMember> for PushMessageContentChatDeleteMember {
  fn as_ref(&self) -> &PushMessageContentChatDeleteMember {
    self
  }
}

impl AsRef<PushMessageContentChatDeleteMember> for RTDPushMessageContentChatDeleteMemberBuilder {
  fn as_ref(&self) -> &PushMessageContentChatDeleteMember {
    &self.inner
  }
}

/// A new member joined the chat via an invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatJoinByLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PushMessageContentChatJoinByLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChatJoinByLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChatJoinByLink {}

impl PushMessageContentChatJoinByLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChatJoinByLinkBuilder {
    let inner = PushMessageContentChatJoinByLink {
      td_name: "pushMessageContentChatJoinByLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPushMessageContentChatJoinByLinkBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatJoinByLinkBuilder {
  inner: PushMessageContentChatJoinByLink,
}

impl RTDPushMessageContentChatJoinByLinkBuilder {
  pub fn build(&self) -> PushMessageContentChatJoinByLink {
    self.inner.clone()
  }
}

impl AsRef<PushMessageContentChatJoinByLink> for PushMessageContentChatJoinByLink {
  fn as_ref(&self) -> &PushMessageContentChatJoinByLink {
    self
  }
}

impl AsRef<PushMessageContentChatJoinByLink> for RTDPushMessageContentChatJoinByLinkBuilder {
  fn as_ref(&self) -> &PushMessageContentChatJoinByLink {
    &self.inner
  }
}

/// A new member was accepted to the chat by an administrator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatJoinByRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PushMessageContentChatJoinByRequest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChatJoinByRequest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChatJoinByRequest {}

impl PushMessageContentChatJoinByRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChatJoinByRequestBuilder {
    let inner = PushMessageContentChatJoinByRequest {
      td_name: "pushMessageContentChatJoinByRequest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPushMessageContentChatJoinByRequestBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatJoinByRequestBuilder {
  inner: PushMessageContentChatJoinByRequest,
}

impl RTDPushMessageContentChatJoinByRequestBuilder {
  pub fn build(&self) -> PushMessageContentChatJoinByRequest {
    self.inner.clone()
  }
}

impl AsRef<PushMessageContentChatJoinByRequest> for PushMessageContentChatJoinByRequest {
  fn as_ref(&self) -> &PushMessageContentChatJoinByRequest {
    self
  }
}

impl AsRef<PushMessageContentChatJoinByRequest> for RTDPushMessageContentChatJoinByRequestBuilder {
  fn as_ref(&self) -> &PushMessageContentChatJoinByRequest {
    &self.inner
  }
}

/// A chat background was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatSetBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the set background is the same as the background of the current user
  is_same: bool,
}

impl RObject for PushMessageContentChatSetBackground {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChatSetBackground"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChatSetBackground {}

impl PushMessageContentChatSetBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChatSetBackgroundBuilder {
    let inner = PushMessageContentChatSetBackground {
      td_name: "pushMessageContentChatSetBackground".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentChatSetBackgroundBuilder { inner }
  }

  pub fn is_same(&self) -> bool {
    self.is_same
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatSetBackgroundBuilder {
  inner: PushMessageContentChatSetBackground,
}

impl RTDPushMessageContentChatSetBackgroundBuilder {
  pub fn build(&self) -> PushMessageContentChatSetBackground {
    self.inner.clone()
  }

  pub fn is_same(&mut self, is_same: bool) -> &mut Self {
    self.inner.is_same = is_same;
    self
  }
}

impl AsRef<PushMessageContentChatSetBackground> for PushMessageContentChatSetBackground {
  fn as_ref(&self) -> &PushMessageContentChatSetBackground {
    self
  }
}

impl AsRef<PushMessageContentChatSetBackground> for RTDPushMessageContentChatSetBackgroundBuilder {
  fn as_ref(&self) -> &PushMessageContentChatSetBackground {
    &self.inner
  }
}

/// A chat theme was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatSetTheme {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// If non-empty, human-readable name of the new theme. Otherwise, the chat theme was reset to the default one
  name: String,
}

impl RObject for PushMessageContentChatSetTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChatSetTheme"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChatSetTheme {}

impl PushMessageContentChatSetTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChatSetThemeBuilder {
    let inner = PushMessageContentChatSetTheme {
      td_name: "pushMessageContentChatSetTheme".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentChatSetThemeBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatSetThemeBuilder {
  inner: PushMessageContentChatSetTheme,
}

impl RTDPushMessageContentChatSetThemeBuilder {
  pub fn build(&self) -> PushMessageContentChatSetTheme {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }
}

impl AsRef<PushMessageContentChatSetTheme> for PushMessageContentChatSetTheme {
  fn as_ref(&self) -> &PushMessageContentChatSetTheme {
    self
  }
}

impl AsRef<PushMessageContentChatSetTheme> for RTDPushMessageContentChatSetThemeBuilder {
  fn as_ref(&self) -> &PushMessageContentChatSetTheme {
    &self.inner
  }
}

/// A message with a checklist
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChecklist {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Checklist title
  title: String,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentChecklist {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChecklist"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChecklist {}

impl PushMessageContentChecklist {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChecklistBuilder {
    let inner = PushMessageContentChecklist {
      td_name: "pushMessageContentChecklist".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentChecklistBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChecklistBuilder {
  inner: PushMessageContentChecklist,
}

impl RTDPushMessageContentChecklistBuilder {
  pub fn build(&self) -> PushMessageContentChecklist {
    self.inner.clone()
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentChecklist> for PushMessageContentChecklist {
  fn as_ref(&self) -> &PushMessageContentChecklist {
    self
  }
}

impl AsRef<PushMessageContentChecklist> for RTDPushMessageContentChecklistBuilder {
  fn as_ref(&self) -> &PushMessageContentChecklist {
    &self.inner
  }
}

/// Some tasks were added to a checklist
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChecklistTasksAdded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of added tasks
  task_count: i64,
}

impl RObject for PushMessageContentChecklistTasksAdded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChecklistTasksAdded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChecklistTasksAdded {}

impl PushMessageContentChecklistTasksAdded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChecklistTasksAddedBuilder {
    let inner = PushMessageContentChecklistTasksAdded {
      td_name: "pushMessageContentChecklistTasksAdded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentChecklistTasksAddedBuilder { inner }
  }

  pub fn task_count(&self) -> i64 {
    self.task_count
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChecklistTasksAddedBuilder {
  inner: PushMessageContentChecklistTasksAdded,
}

impl RTDPushMessageContentChecklistTasksAddedBuilder {
  pub fn build(&self) -> PushMessageContentChecklistTasksAdded {
    self.inner.clone()
  }

  pub fn task_count(&mut self, task_count: i64) -> &mut Self {
    self.inner.task_count = task_count;
    self
  }
}

impl AsRef<PushMessageContentChecklistTasksAdded> for PushMessageContentChecklistTasksAdded {
  fn as_ref(&self) -> &PushMessageContentChecklistTasksAdded {
    self
  }
}

impl AsRef<PushMessageContentChecklistTasksAdded>
  for RTDPushMessageContentChecklistTasksAddedBuilder
{
  fn as_ref(&self) -> &PushMessageContentChecklistTasksAdded {
    &self.inner
  }
}

/// Some tasks from a checklist were marked as done or not done
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChecklistTasksDone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of changed tasks
  task_count: i64,
}

impl RObject for PushMessageContentChecklistTasksDone {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentChecklistTasksDone"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentChecklistTasksDone {}

impl PushMessageContentChecklistTasksDone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentChecklistTasksDoneBuilder {
    let inner = PushMessageContentChecklistTasksDone {
      td_name: "pushMessageContentChecklistTasksDone".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentChecklistTasksDoneBuilder { inner }
  }

  pub fn task_count(&self) -> i64 {
    self.task_count
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChecklistTasksDoneBuilder {
  inner: PushMessageContentChecklistTasksDone,
}

impl RTDPushMessageContentChecklistTasksDoneBuilder {
  pub fn build(&self) -> PushMessageContentChecklistTasksDone {
    self.inner.clone()
  }

  pub fn task_count(&mut self, task_count: i64) -> &mut Self {
    self.inner.task_count = task_count;
    self
  }
}

impl AsRef<PushMessageContentChecklistTasksDone> for PushMessageContentChecklistTasksDone {
  fn as_ref(&self) -> &PushMessageContentChecklistTasksDone {
    self
  }
}

impl AsRef<PushMessageContentChecklistTasksDone>
  for RTDPushMessageContentChecklistTasksDoneBuilder
{
  fn as_ref(&self) -> &PushMessageContentChecklistTasksDone {
    &self.inner
  }
}

/// A message with a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Contact's name
  name: String,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentContact {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentContact"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentContact {}

impl PushMessageContentContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentContactBuilder {
    let inner = PushMessageContentContact {
      td_name: "pushMessageContentContact".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentContactBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentContactBuilder {
  inner: PushMessageContentContact,
}

impl RTDPushMessageContentContactBuilder {
  pub fn build(&self) -> PushMessageContentContact {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentContact> for PushMessageContentContact {
  fn as_ref(&self) -> &PushMessageContentContact {
    self
  }
}

impl AsRef<PushMessageContentContact> for RTDPushMessageContentContactBuilder {
  fn as_ref(&self) -> &PushMessageContentContact {
    &self.inner
  }
}

/// A contact has registered with Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentContactRegistered {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the user joined Telegram as a Telegram Premium account
  as_premium_account: bool,
}

impl RObject for PushMessageContentContactRegistered {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentContactRegistered"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentContactRegistered {}

impl PushMessageContentContactRegistered {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentContactRegisteredBuilder {
    let inner = PushMessageContentContactRegistered {
      td_name: "pushMessageContentContactRegistered".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentContactRegisteredBuilder { inner }
  }

  pub fn as_premium_account(&self) -> bool {
    self.as_premium_account
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentContactRegisteredBuilder {
  inner: PushMessageContentContactRegistered,
}

impl RTDPushMessageContentContactRegisteredBuilder {
  pub fn build(&self) -> PushMessageContentContactRegistered {
    self.inner.clone()
  }

  pub fn as_premium_account(&mut self, as_premium_account: bool) -> &mut Self {
    self.inner.as_premium_account = as_premium_account;
    self
  }
}

impl AsRef<PushMessageContentContactRegistered> for PushMessageContentContactRegistered {
  fn as_ref(&self) -> &PushMessageContentContactRegistered {
    self
  }
}

impl AsRef<PushMessageContentContactRegistered> for RTDPushMessageContentContactRegisteredBuilder {
  fn as_ref(&self) -> &PushMessageContentContactRegistered {
    &self.inner
  }
}

/// A document message (a general file)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message content; may be null
  document: Option<Document>,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentDocument {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentDocument"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentDocument {}

impl PushMessageContentDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentDocumentBuilder {
    let inner = PushMessageContentDocument {
      td_name: "pushMessageContentDocument".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentDocumentBuilder { inner }
  }

  pub fn document(&self) -> &Option<Document> {
    &self.document
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentDocumentBuilder {
  inner: PushMessageContentDocument,
}

impl RTDPushMessageContentDocumentBuilder {
  pub fn build(&self) -> PushMessageContentDocument {
    self.inner.clone()
  }

  pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
    self.inner.document = Some(document.as_ref().clone());
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentDocument> for PushMessageContentDocument {
  fn as_ref(&self) -> &PushMessageContentDocument {
    self
  }
}

impl AsRef<PushMessageContentDocument> for RTDPushMessageContentDocumentBuilder {
  fn as_ref(&self) -> &PushMessageContentDocument {
    &self.inner
  }
}

/// A message with a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Game title, empty for pinned game message
  title: String,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentGame {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentGame"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentGame {}

impl PushMessageContentGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentGameBuilder {
    let inner = PushMessageContentGame {
      td_name: "pushMessageContentGame".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentGameBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentGameBuilder {
  inner: PushMessageContentGame,
}

impl RTDPushMessageContentGameBuilder {
  pub fn build(&self) -> PushMessageContentGame {
    self.inner.clone()
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentGame> for PushMessageContentGame {
  fn as_ref(&self) -> &PushMessageContentGame {
    self
  }
}

impl AsRef<PushMessageContentGame> for RTDPushMessageContentGameBuilder {
  fn as_ref(&self) -> &PushMessageContentGame {
    &self.inner
  }
}

/// A new high score was achieved in a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentGameScore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Game title, empty for pinned message
  title: String,
  /// New score, 0 for pinned message
  score: i64,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentGameScore {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentGameScore"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentGameScore {}

impl PushMessageContentGameScore {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentGameScoreBuilder {
    let inner = PushMessageContentGameScore {
      td_name: "pushMessageContentGameScore".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentGameScoreBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn score(&self) -> i64 {
    self.score
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentGameScoreBuilder {
  inner: PushMessageContentGameScore,
}

impl RTDPushMessageContentGameScoreBuilder {
  pub fn build(&self) -> PushMessageContentGameScore {
    self.inner.clone()
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn score(&mut self, score: i64) -> &mut Self {
    self.inner.score = score;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentGameScore> for PushMessageContentGameScore {
  fn as_ref(&self) -> &PushMessageContentGameScore {
    self
  }
}

impl AsRef<PushMessageContentGameScore> for RTDPushMessageContentGameScoreBuilder {
  fn as_ref(&self) -> &PushMessageContentGameScore {
    &self.inner
  }
}

/// A message with a gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of Telegram Stars that sender paid for the gift
  star_count: i64,
  /// True, if the message is about prepaid upgrade of the gift by another user instead of actual receiving of a new gift
  is_prepaid_upgrade: bool,
}

impl RObject for PushMessageContentGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentGift {}

impl PushMessageContentGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentGiftBuilder {
    let inner = PushMessageContentGift {
      td_name: "pushMessageContentGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentGiftBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn is_prepaid_upgrade(&self) -> bool {
    self.is_prepaid_upgrade
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentGiftBuilder {
  inner: PushMessageContentGift,
}

impl RTDPushMessageContentGiftBuilder {
  pub fn build(&self) -> PushMessageContentGift {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn is_prepaid_upgrade(&mut self, is_prepaid_upgrade: bool) -> &mut Self {
    self.inner.is_prepaid_upgrade = is_prepaid_upgrade;
    self
  }
}

impl AsRef<PushMessageContentGift> for PushMessageContentGift {
  fn as_ref(&self) -> &PushMessageContentGift {
    self
  }
}

impl AsRef<PushMessageContentGift> for RTDPushMessageContentGiftBuilder {
  fn as_ref(&self) -> &PushMessageContentGift {
    &self.inner
  }
}

/// A message with a giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentGiveaway {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of users who will receive giveaway prizes; 0 for pinned message
  winner_count: i64,
  /// Prize of the giveaway; may be null for pinned message
  prize: Option<GiveawayPrize>,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentGiveaway {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentGiveaway"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentGiveaway {}

impl PushMessageContentGiveaway {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentGiveawayBuilder {
    let inner = PushMessageContentGiveaway {
      td_name: "pushMessageContentGiveaway".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentGiveawayBuilder { inner }
  }

  pub fn winner_count(&self) -> i64 {
    self.winner_count
  }

  pub fn prize(&self) -> &Option<GiveawayPrize> {
    &self.prize
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentGiveawayBuilder {
  inner: PushMessageContentGiveaway,
}

impl RTDPushMessageContentGiveawayBuilder {
  pub fn build(&self) -> PushMessageContentGiveaway {
    self.inner.clone()
  }

  pub fn winner_count(&mut self, winner_count: i64) -> &mut Self {
    self.inner.winner_count = winner_count;
    self
  }

  pub fn prize<T: AsRef<GiveawayPrize>>(&mut self, prize: T) -> &mut Self {
    self.inner.prize = Some(prize.as_ref().clone());
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentGiveaway> for PushMessageContentGiveaway {
  fn as_ref(&self) -> &PushMessageContentGiveaway {
    self
  }
}

impl AsRef<PushMessageContentGiveaway> for RTDPushMessageContentGiveawayBuilder {
  fn as_ref(&self) -> &PushMessageContentGiveaway {
    &self.inner
  }
}

/// A general message with hidden content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentHidden {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentHidden {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentHidden"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentHidden {}

impl PushMessageContentHidden {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentHiddenBuilder {
    let inner = PushMessageContentHidden {
      td_name: "pushMessageContentHidden".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentHiddenBuilder { inner }
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentHiddenBuilder {
  inner: PushMessageContentHidden,
}

impl RTDPushMessageContentHiddenBuilder {
  pub fn build(&self) -> PushMessageContentHidden {
    self.inner.clone()
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentHidden> for PushMessageContentHidden {
  fn as_ref(&self) -> &PushMessageContentHidden {
    self
  }
}

impl AsRef<PushMessageContentHidden> for RTDPushMessageContentHiddenBuilder {
  fn as_ref(&self) -> &PushMessageContentHidden {
    &self.inner
  }
}

/// An invitation of participants to a video chat or live stream
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentInviteVideoChatParticipants {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the current user was invited to the video chat or the live stream
  is_current_user: bool,
}

impl RObject for PushMessageContentInviteVideoChatParticipants {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentInviteVideoChatParticipants"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentInviteVideoChatParticipants {}

impl PushMessageContentInviteVideoChatParticipants {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentInviteVideoChatParticipantsBuilder {
    let inner = PushMessageContentInviteVideoChatParticipants {
      td_name: "pushMessageContentInviteVideoChatParticipants".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentInviteVideoChatParticipantsBuilder { inner }
  }

  pub fn is_current_user(&self) -> bool {
    self.is_current_user
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentInviteVideoChatParticipantsBuilder {
  inner: PushMessageContentInviteVideoChatParticipants,
}

impl RTDPushMessageContentInviteVideoChatParticipantsBuilder {
  pub fn build(&self) -> PushMessageContentInviteVideoChatParticipants {
    self.inner.clone()
  }

  pub fn is_current_user(&mut self, is_current_user: bool) -> &mut Self {
    self.inner.is_current_user = is_current_user;
    self
  }
}

impl AsRef<PushMessageContentInviteVideoChatParticipants>
  for PushMessageContentInviteVideoChatParticipants
{
  fn as_ref(&self) -> &PushMessageContentInviteVideoChatParticipants {
    self
  }
}

impl AsRef<PushMessageContentInviteVideoChatParticipants>
  for RTDPushMessageContentInviteVideoChatParticipantsBuilder
{
  fn as_ref(&self) -> &PushMessageContentInviteVideoChatParticipants {
    &self.inner
  }
}

/// A message with an invoice from a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentInvoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Product price
  price: String,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentInvoice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentInvoice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentInvoice {}

impl PushMessageContentInvoice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentInvoiceBuilder {
    let inner = PushMessageContentInvoice {
      td_name: "pushMessageContentInvoice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentInvoiceBuilder { inner }
  }

  pub fn price(&self) -> &String {
    &self.price
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentInvoiceBuilder {
  inner: PushMessageContentInvoice,
}

impl RTDPushMessageContentInvoiceBuilder {
  pub fn build(&self) -> PushMessageContentInvoice {
    self.inner.clone()
  }

  pub fn price<T: AsRef<str>>(&mut self, price: T) -> &mut Self {
    self.inner.price = price.as_ref().to_string();
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentInvoice> for PushMessageContentInvoice {
  fn as_ref(&self) -> &PushMessageContentInvoice {
    self
  }
}

impl AsRef<PushMessageContentInvoice> for RTDPushMessageContentInvoiceBuilder {
  fn as_ref(&self) -> &PushMessageContentInvoice {
    &self.inner
  }
}

/// A message with a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the location is live
  is_live: bool,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentLocation {}

impl PushMessageContentLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentLocationBuilder {
    let inner = PushMessageContentLocation {
      td_name: "pushMessageContentLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentLocationBuilder { inner }
  }

  pub fn is_live(&self) -> bool {
    self.is_live
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentLocationBuilder {
  inner: PushMessageContentLocation,
}

impl RTDPushMessageContentLocationBuilder {
  pub fn build(&self) -> PushMessageContentLocation {
    self.inner.clone()
  }

  pub fn is_live(&mut self, is_live: bool) -> &mut Self {
    self.inner.is_live = is_live;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentLocation> for PushMessageContentLocation {
  fn as_ref(&self) -> &PushMessageContentLocation {
    self
  }
}

impl AsRef<PushMessageContentLocation> for RTDPushMessageContentLocationBuilder {
  fn as_ref(&self) -> &PushMessageContentLocation {
    &self.inner
  }
}

/// A media album
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentMediaAlbum {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of messages in the album
  total_count: i64,
  /// True, if the album has at least one photo
  has_photos: bool,
  /// True, if the album has at least one video file
  has_videos: bool,
  /// True, if the album has at least one audio file
  has_audios: bool,
  /// True, if the album has at least one document
  has_documents: bool,
}

impl RObject for PushMessageContentMediaAlbum {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentMediaAlbum"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentMediaAlbum {}

impl PushMessageContentMediaAlbum {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentMediaAlbumBuilder {
    let inner = PushMessageContentMediaAlbum {
      td_name: "pushMessageContentMediaAlbum".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentMediaAlbumBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn has_photos(&self) -> bool {
    self.has_photos
  }

  pub fn has_videos(&self) -> bool {
    self.has_videos
  }

  pub fn has_audios(&self) -> bool {
    self.has_audios
  }

  pub fn has_documents(&self) -> bool {
    self.has_documents
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentMediaAlbumBuilder {
  inner: PushMessageContentMediaAlbum,
}

impl RTDPushMessageContentMediaAlbumBuilder {
  pub fn build(&self) -> PushMessageContentMediaAlbum {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn has_photos(&mut self, has_photos: bool) -> &mut Self {
    self.inner.has_photos = has_photos;
    self
  }

  pub fn has_videos(&mut self, has_videos: bool) -> &mut Self {
    self.inner.has_videos = has_videos;
    self
  }

  pub fn has_audios(&mut self, has_audios: bool) -> &mut Self {
    self.inner.has_audios = has_audios;
    self
  }

  pub fn has_documents(&mut self, has_documents: bool) -> &mut Self {
    self.inner.has_documents = has_documents;
    self
  }
}

impl AsRef<PushMessageContentMediaAlbum> for PushMessageContentMediaAlbum {
  fn as_ref(&self) -> &PushMessageContentMediaAlbum {
    self
  }
}

impl AsRef<PushMessageContentMediaAlbum> for RTDPushMessageContentMediaAlbumBuilder {
  fn as_ref(&self) -> &PushMessageContentMediaAlbum {
    &self.inner
  }
}

/// Forwarded messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentMessageForwards {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of forwarded messages
  total_count: i64,
}

impl RObject for PushMessageContentMessageForwards {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentMessageForwards"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentMessageForwards {}

impl PushMessageContentMessageForwards {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentMessageForwardsBuilder {
    let inner = PushMessageContentMessageForwards {
      td_name: "pushMessageContentMessageForwards".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentMessageForwardsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentMessageForwardsBuilder {
  inner: PushMessageContentMessageForwards,
}

impl RTDPushMessageContentMessageForwardsBuilder {
  pub fn build(&self) -> PushMessageContentMessageForwards {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }
}

impl AsRef<PushMessageContentMessageForwards> for PushMessageContentMessageForwards {
  fn as_ref(&self) -> &PushMessageContentMessageForwards {
    self
  }
}

impl AsRef<PushMessageContentMessageForwards> for RTDPushMessageContentMessageForwardsBuilder {
  fn as_ref(&self) -> &PushMessageContentMessageForwards {
    &self.inner
  }
}

/// A message with paid media
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentPaidMedia {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of Telegram Stars needed to buy access to the media in the message; 0 for pinned message
  star_count: i64,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentPaidMedia {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentPaidMedia"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentPaidMedia {}

impl PushMessageContentPaidMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentPaidMediaBuilder {
    let inner = PushMessageContentPaidMedia {
      td_name: "pushMessageContentPaidMedia".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentPaidMediaBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentPaidMediaBuilder {
  inner: PushMessageContentPaidMedia,
}

impl RTDPushMessageContentPaidMediaBuilder {
  pub fn build(&self) -> PushMessageContentPaidMedia {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentPaidMedia> for PushMessageContentPaidMedia {
  fn as_ref(&self) -> &PushMessageContentPaidMedia {
    self
  }
}

impl AsRef<PushMessageContentPaidMedia> for RTDPushMessageContentPaidMediaBuilder {
  fn as_ref(&self) -> &PushMessageContentPaidMedia {
    &self.inner
  }
}

/// A photo message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message content; may be null
  photo: Option<Photo>,
  /// Photo caption
  caption: String,
  /// True, if the photo is secret
  is_secret: bool,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentPhoto {}

impl PushMessageContentPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentPhotoBuilder {
    let inner = PushMessageContentPhoto {
      td_name: "pushMessageContentPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Option<Photo> {
    &self.photo
  }

  pub fn caption(&self) -> &String {
    &self.caption
  }

  pub fn is_secret(&self) -> bool {
    self.is_secret
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentPhotoBuilder {
  inner: PushMessageContentPhoto,
}

impl RTDPushMessageContentPhotoBuilder {
  pub fn build(&self) -> PushMessageContentPhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn caption<T: AsRef<str>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().to_string();
    self
  }

  pub fn is_secret(&mut self, is_secret: bool) -> &mut Self {
    self.inner.is_secret = is_secret;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentPhoto> for PushMessageContentPhoto {
  fn as_ref(&self) -> &PushMessageContentPhoto {
    self
  }
}

impl AsRef<PushMessageContentPhoto> for RTDPushMessageContentPhotoBuilder {
  fn as_ref(&self) -> &PushMessageContentPhoto {
    &self.inner
  }
}

/// A message with a poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentPoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Poll question
  question: String,
  /// True, if the poll is regular and not in quiz mode
  is_regular: bool,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentPoll {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentPoll"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentPoll {}

impl PushMessageContentPoll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentPollBuilder {
    let inner = PushMessageContentPoll {
      td_name: "pushMessageContentPoll".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentPollBuilder { inner }
  }

  pub fn question(&self) -> &String {
    &self.question
  }

  pub fn is_regular(&self) -> bool {
    self.is_regular
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentPollBuilder {
  inner: PushMessageContentPoll,
}

impl RTDPushMessageContentPollBuilder {
  pub fn build(&self) -> PushMessageContentPoll {
    self.inner.clone()
  }

  pub fn question<T: AsRef<str>>(&mut self, question: T) -> &mut Self {
    self.inner.question = question.as_ref().to_string();
    self
  }

  pub fn is_regular(&mut self, is_regular: bool) -> &mut Self {
    self.inner.is_regular = is_regular;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentPoll> for PushMessageContentPoll {
  fn as_ref(&self) -> &PushMessageContentPoll {
    self
  }
}

impl AsRef<PushMessageContentPoll> for RTDPushMessageContentPollBuilder {
  fn as_ref(&self) -> &PushMessageContentPoll {
    &self.inner
  }
}

/// An option was added to a poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentPollOptionAdded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the option
  text: String,
}

impl RObject for PushMessageContentPollOptionAdded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentPollOptionAdded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentPollOptionAdded {}

impl PushMessageContentPollOptionAdded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentPollOptionAddedBuilder {
    let inner = PushMessageContentPollOptionAdded {
      td_name: "pushMessageContentPollOptionAdded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentPollOptionAddedBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentPollOptionAddedBuilder {
  inner: PushMessageContentPollOptionAdded,
}

impl RTDPushMessageContentPollOptionAddedBuilder {
  pub fn build(&self) -> PushMessageContentPollOptionAdded {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }
}

impl AsRef<PushMessageContentPollOptionAdded> for PushMessageContentPollOptionAdded {
  fn as_ref(&self) -> &PushMessageContentPollOptionAdded {
    self
  }
}

impl AsRef<PushMessageContentPollOptionAdded> for RTDPushMessageContentPollOptionAddedBuilder {
  fn as_ref(&self) -> &PushMessageContentPollOptionAdded {
    &self.inner
  }
}

/// A message with a Telegram Premium gift code created for the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentPremiumGiftCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of months the Telegram Premium subscription will be active after code activation
  month_count: i64,
}

impl RObject for PushMessageContentPremiumGiftCode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentPremiumGiftCode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentPremiumGiftCode {}

impl PushMessageContentPremiumGiftCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentPremiumGiftCodeBuilder {
    let inner = PushMessageContentPremiumGiftCode {
      td_name: "pushMessageContentPremiumGiftCode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentPremiumGiftCodeBuilder { inner }
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentPremiumGiftCodeBuilder {
  inner: PushMessageContentPremiumGiftCode,
}

impl RTDPushMessageContentPremiumGiftCodeBuilder {
  pub fn build(&self) -> PushMessageContentPremiumGiftCode {
    self.inner.clone()
  }

  pub fn month_count(&mut self, month_count: i64) -> &mut Self {
    self.inner.month_count = month_count;
    self
  }
}

impl AsRef<PushMessageContentPremiumGiftCode> for PushMessageContentPremiumGiftCode {
  fn as_ref(&self) -> &PushMessageContentPremiumGiftCode {
    self
  }
}

impl AsRef<PushMessageContentPremiumGiftCode> for RTDPushMessageContentPremiumGiftCodeBuilder {
  fn as_ref(&self) -> &PushMessageContentPremiumGiftCode {
    &self.inner
  }
}

/// A user in the chat came within proximity alert range from the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentProximityAlertTriggered {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The distance to the user
  distance: i64,
}

impl RObject for PushMessageContentProximityAlertTriggered {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentProximityAlertTriggered"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentProximityAlertTriggered {}

impl PushMessageContentProximityAlertTriggered {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentProximityAlertTriggeredBuilder {
    let inner = PushMessageContentProximityAlertTriggered {
      td_name: "pushMessageContentProximityAlertTriggered".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentProximityAlertTriggeredBuilder { inner }
  }

  pub fn distance(&self) -> i64 {
    self.distance
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentProximityAlertTriggeredBuilder {
  inner: PushMessageContentProximityAlertTriggered,
}

impl RTDPushMessageContentProximityAlertTriggeredBuilder {
  pub fn build(&self) -> PushMessageContentProximityAlertTriggered {
    self.inner.clone()
  }

  pub fn distance(&mut self, distance: i64) -> &mut Self {
    self.inner.distance = distance;
    self
  }
}

impl AsRef<PushMessageContentProximityAlertTriggered>
  for PushMessageContentProximityAlertTriggered
{
  fn as_ref(&self) -> &PushMessageContentProximityAlertTriggered {
    self
  }
}

impl AsRef<PushMessageContentProximityAlertTriggered>
  for RTDPushMessageContentProximityAlertTriggeredBuilder
{
  fn as_ref(&self) -> &PushMessageContentProximityAlertTriggered {
    &self.inner
  }
}

/// A new recurring payment was made by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentRecurringPayment {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The paid amount
  amount: String,
}

impl RObject for PushMessageContentRecurringPayment {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentRecurringPayment"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentRecurringPayment {}

impl PushMessageContentRecurringPayment {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentRecurringPaymentBuilder {
    let inner = PushMessageContentRecurringPayment {
      td_name: "pushMessageContentRecurringPayment".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentRecurringPaymentBuilder { inner }
  }

  pub fn amount(&self) -> &String {
    &self.amount
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentRecurringPaymentBuilder {
  inner: PushMessageContentRecurringPayment,
}

impl RTDPushMessageContentRecurringPaymentBuilder {
  pub fn build(&self) -> PushMessageContentRecurringPayment {
    self.inner.clone()
  }

  pub fn amount<T: AsRef<str>>(&mut self, amount: T) -> &mut Self {
    self.inner.amount = amount.as_ref().to_string();
    self
  }
}

impl AsRef<PushMessageContentRecurringPayment> for PushMessageContentRecurringPayment {
  fn as_ref(&self) -> &PushMessageContentRecurringPayment {
    self
  }
}

impl AsRef<PushMessageContentRecurringPayment> for RTDPushMessageContentRecurringPaymentBuilder {
  fn as_ref(&self) -> &PushMessageContentRecurringPayment {
    &self.inner
  }
}

/// A screenshot of a message in the chat has been taken
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentScreenshotTaken {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PushMessageContentScreenshotTaken {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentScreenshotTaken"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentScreenshotTaken {}

impl PushMessageContentScreenshotTaken {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentScreenshotTakenBuilder {
    let inner = PushMessageContentScreenshotTaken {
      td_name: "pushMessageContentScreenshotTaken".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPushMessageContentScreenshotTakenBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentScreenshotTakenBuilder {
  inner: PushMessageContentScreenshotTaken,
}

impl RTDPushMessageContentScreenshotTakenBuilder {
  pub fn build(&self) -> PushMessageContentScreenshotTaken {
    self.inner.clone()
  }
}

impl AsRef<PushMessageContentScreenshotTaken> for PushMessageContentScreenshotTaken {
  fn as_ref(&self) -> &PushMessageContentScreenshotTaken {
    self
  }
}

impl AsRef<PushMessageContentScreenshotTaken> for RTDPushMessageContentScreenshotTakenBuilder {
  fn as_ref(&self) -> &PushMessageContentScreenshotTaken {
    &self.inner
  }
}

/// A message with a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message content; may be null
  sticker: Option<Sticker>,
  /// Emoji corresponding to the sticker; may be empty
  emoji: String,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentSticker {}

impl PushMessageContentSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentStickerBuilder {
    let inner = PushMessageContentSticker {
      td_name: "pushMessageContentSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &Option<Sticker> {
    &self.sticker
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentStickerBuilder {
  inner: PushMessageContentSticker,
}

impl RTDPushMessageContentStickerBuilder {
  pub fn build(&self) -> PushMessageContentSticker {
    self.inner.clone()
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentSticker> for PushMessageContentSticker {
  fn as_ref(&self) -> &PushMessageContentSticker {
    self
  }
}

impl AsRef<PushMessageContentSticker> for RTDPushMessageContentStickerBuilder {
  fn as_ref(&self) -> &PushMessageContentSticker {
    &self.inner
  }
}

/// A message with a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the user was mentioned in the story
  is_mention: bool,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentStory {}

impl PushMessageContentStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentStoryBuilder {
    let inner = PushMessageContentStory {
      td_name: "pushMessageContentStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentStoryBuilder { inner }
  }

  pub fn is_mention(&self) -> bool {
    self.is_mention
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentStoryBuilder {
  inner: PushMessageContentStory,
}

impl RTDPushMessageContentStoryBuilder {
  pub fn build(&self) -> PushMessageContentStory {
    self.inner.clone()
  }

  pub fn is_mention(&mut self, is_mention: bool) -> &mut Self {
    self.inner.is_mention = is_mention;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentStory> for PushMessageContentStory {
  fn as_ref(&self) -> &PushMessageContentStory {
    self
  }
}

impl AsRef<PushMessageContentStory> for RTDPushMessageContentStoryBuilder {
  fn as_ref(&self) -> &PushMessageContentStory {
    &self.inner
  }
}

/// A birthdate was suggested to be set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentSuggestBirthdate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PushMessageContentSuggestBirthdate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentSuggestBirthdate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentSuggestBirthdate {}

impl PushMessageContentSuggestBirthdate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentSuggestBirthdateBuilder {
    let inner = PushMessageContentSuggestBirthdate {
      td_name: "pushMessageContentSuggestBirthdate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPushMessageContentSuggestBirthdateBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentSuggestBirthdateBuilder {
  inner: PushMessageContentSuggestBirthdate,
}

impl RTDPushMessageContentSuggestBirthdateBuilder {
  pub fn build(&self) -> PushMessageContentSuggestBirthdate {
    self.inner.clone()
  }
}

impl AsRef<PushMessageContentSuggestBirthdate> for PushMessageContentSuggestBirthdate {
  fn as_ref(&self) -> &PushMessageContentSuggestBirthdate {
    self
  }
}

impl AsRef<PushMessageContentSuggestBirthdate> for RTDPushMessageContentSuggestBirthdateBuilder {
  fn as_ref(&self) -> &PushMessageContentSuggestBirthdate {
    &self.inner
  }
}

/// A profile photo was suggested to the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentSuggestProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PushMessageContentSuggestProfilePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentSuggestProfilePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentSuggestProfilePhoto {}

impl PushMessageContentSuggestProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentSuggestProfilePhotoBuilder {
    let inner = PushMessageContentSuggestProfilePhoto {
      td_name: "pushMessageContentSuggestProfilePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPushMessageContentSuggestProfilePhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentSuggestProfilePhotoBuilder {
  inner: PushMessageContentSuggestProfilePhoto,
}

impl RTDPushMessageContentSuggestProfilePhotoBuilder {
  pub fn build(&self) -> PushMessageContentSuggestProfilePhoto {
    self.inner.clone()
  }
}

impl AsRef<PushMessageContentSuggestProfilePhoto> for PushMessageContentSuggestProfilePhoto {
  fn as_ref(&self) -> &PushMessageContentSuggestProfilePhoto {
    self
  }
}

impl AsRef<PushMessageContentSuggestProfilePhoto>
  for RTDPushMessageContentSuggestProfilePhotoBuilder
{
  fn as_ref(&self) -> &PushMessageContentSuggestProfilePhoto {
    &self.inner
  }
}

/// A text message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message text
  text: String,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentText"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentText {}

impl PushMessageContentText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentTextBuilder {
    let inner = PushMessageContentText {
      td_name: "pushMessageContentText".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentTextBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentTextBuilder {
  inner: PushMessageContentText,
}

impl RTDPushMessageContentTextBuilder {
  pub fn build(&self) -> PushMessageContentText {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentText> for PushMessageContentText {
  fn as_ref(&self) -> &PushMessageContentText {
    self
  }
}

impl AsRef<PushMessageContentText> for RTDPushMessageContentTextBuilder {
  fn as_ref(&self) -> &PushMessageContentText {
    &self.inner
  }
}

/// A message with a request from a dApp was received through TON Connect
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentTonConnectRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the dApp; may be empty if unknown
  dapp_name: String,
}

impl RObject for PushMessageContentTonConnectRequest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentTonConnectRequest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentTonConnectRequest {}

impl PushMessageContentTonConnectRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentTonConnectRequestBuilder {
    let inner = PushMessageContentTonConnectRequest {
      td_name: "pushMessageContentTonConnectRequest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentTonConnectRequestBuilder { inner }
  }

  pub fn dapp_name(&self) -> &String {
    &self.dapp_name
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentTonConnectRequestBuilder {
  inner: PushMessageContentTonConnectRequest,
}

impl RTDPushMessageContentTonConnectRequestBuilder {
  pub fn build(&self) -> PushMessageContentTonConnectRequest {
    self.inner.clone()
  }

  pub fn dapp_name<T: AsRef<str>>(&mut self, dapp_name: T) -> &mut Self {
    self.inner.dapp_name = dapp_name.as_ref().to_string();
    self
  }
}

impl AsRef<PushMessageContentTonConnectRequest> for PushMessageContentTonConnectRequest {
  fn as_ref(&self) -> &PushMessageContentTonConnectRequest {
    self
  }
}

impl AsRef<PushMessageContentTonConnectRequest> for RTDPushMessageContentTonConnectRequestBuilder {
  fn as_ref(&self) -> &PushMessageContentTonConnectRequest {
    &self.inner
  }
}

/// A message with a TON wallet transfer
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentTonWalletTransfer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sender of the transfer; 0 if unknown
  sender_user_id: i64,
  /// String representation of the received amount
  amount: String,
  /// Comment added to the transaction if it is public
  comment: String,
}

impl RObject for PushMessageContentTonWalletTransfer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentTonWalletTransfer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentTonWalletTransfer {}

impl PushMessageContentTonWalletTransfer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentTonWalletTransferBuilder {
    let inner = PushMessageContentTonWalletTransfer {
      td_name: "pushMessageContentTonWalletTransfer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentTonWalletTransferBuilder { inner }
  }

  pub fn sender_user_id(&self) -> i64 {
    self.sender_user_id
  }

  pub fn amount(&self) -> &String {
    &self.amount
  }

  pub fn comment(&self) -> &String {
    &self.comment
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentTonWalletTransferBuilder {
  inner: PushMessageContentTonWalletTransfer,
}

impl RTDPushMessageContentTonWalletTransferBuilder {
  pub fn build(&self) -> PushMessageContentTonWalletTransfer {
    self.inner.clone()
  }

  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

  pub fn amount<T: AsRef<str>>(&mut self, amount: T) -> &mut Self {
    self.inner.amount = amount.as_ref().to_string();
    self
  }

  pub fn comment<T: AsRef<str>>(&mut self, comment: T) -> &mut Self {
    self.inner.comment = comment.as_ref().to_string();
    self
  }
}

impl AsRef<PushMessageContentTonWalletTransfer> for PushMessageContentTonWalletTransfer {
  fn as_ref(&self) -> &PushMessageContentTonWalletTransfer {
    self
  }
}

impl AsRef<PushMessageContentTonWalletTransfer> for RTDPushMessageContentTonWalletTransferBuilder {
  fn as_ref(&self) -> &PushMessageContentTonWalletTransfer {
    &self.inner
  }
}

/// A message with an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentUpgradedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the gift was obtained by upgrading of a previously received gift; otherwise, if is_prepaid_upgrade == false, then this is a transferred or resold gift
  is_upgrade: bool,
  /// True, if the message is about completion of prepaid upgrade of the gift instead of actual receiving of a new gift
  is_prepaid_upgrade: bool,
}

impl RObject for PushMessageContentUpgradedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentUpgradedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentUpgradedGift {}

impl PushMessageContentUpgradedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentUpgradedGiftBuilder {
    let inner = PushMessageContentUpgradedGift {
      td_name: "pushMessageContentUpgradedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentUpgradedGiftBuilder { inner }
  }

  pub fn is_upgrade(&self) -> bool {
    self.is_upgrade
  }

  pub fn is_prepaid_upgrade(&self) -> bool {
    self.is_prepaid_upgrade
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentUpgradedGiftBuilder {
  inner: PushMessageContentUpgradedGift,
}

impl RTDPushMessageContentUpgradedGiftBuilder {
  pub fn build(&self) -> PushMessageContentUpgradedGift {
    self.inner.clone()
  }

  pub fn is_upgrade(&mut self, is_upgrade: bool) -> &mut Self {
    self.inner.is_upgrade = is_upgrade;
    self
  }

  pub fn is_prepaid_upgrade(&mut self, is_prepaid_upgrade: bool) -> &mut Self {
    self.inner.is_prepaid_upgrade = is_prepaid_upgrade;
    self
  }
}

impl AsRef<PushMessageContentUpgradedGift> for PushMessageContentUpgradedGift {
  fn as_ref(&self) -> &PushMessageContentUpgradedGift {
    self
  }
}

impl AsRef<PushMessageContentUpgradedGift> for RTDPushMessageContentUpgradedGiftBuilder {
  fn as_ref(&self) -> &PushMessageContentUpgradedGift {
    &self.inner
  }
}

/// A video message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message content; may be null
  video: Option<Video>,
  /// Video caption
  caption: String,
  /// True, if the video is secret
  is_secret: bool,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentVideo {}

impl PushMessageContentVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentVideoBuilder {
    let inner = PushMessageContentVideo {
      td_name: "pushMessageContentVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentVideoBuilder { inner }
  }

  pub fn video(&self) -> &Option<Video> {
    &self.video
  }

  pub fn caption(&self) -> &String {
    &self.caption
  }

  pub fn is_secret(&self) -> bool {
    self.is_secret
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentVideoBuilder {
  inner: PushMessageContentVideo,
}

impl RTDPushMessageContentVideoBuilder {
  pub fn build(&self) -> PushMessageContentVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = Some(video.as_ref().clone());
    self
  }

  pub fn caption<T: AsRef<str>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().to_string();
    self
  }

  pub fn is_secret(&mut self, is_secret: bool) -> &mut Self {
    self.inner.is_secret = is_secret;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentVideo> for PushMessageContentVideo {
  fn as_ref(&self) -> &PushMessageContentVideo {
    self
  }
}

impl AsRef<PushMessageContentVideo> for RTDPushMessageContentVideoBuilder {
  fn as_ref(&self) -> &PushMessageContentVideo {
    &self.inner
  }
}

/// A video chat or live stream has ended
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentVideoChatEnded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PushMessageContentVideoChatEnded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentVideoChatEnded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentVideoChatEnded {}

impl PushMessageContentVideoChatEnded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentVideoChatEndedBuilder {
    let inner = PushMessageContentVideoChatEnded {
      td_name: "pushMessageContentVideoChatEnded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPushMessageContentVideoChatEndedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentVideoChatEndedBuilder {
  inner: PushMessageContentVideoChatEnded,
}

impl RTDPushMessageContentVideoChatEndedBuilder {
  pub fn build(&self) -> PushMessageContentVideoChatEnded {
    self.inner.clone()
  }
}

impl AsRef<PushMessageContentVideoChatEnded> for PushMessageContentVideoChatEnded {
  fn as_ref(&self) -> &PushMessageContentVideoChatEnded {
    self
  }
}

impl AsRef<PushMessageContentVideoChatEnded> for RTDPushMessageContentVideoChatEndedBuilder {
  fn as_ref(&self) -> &PushMessageContentVideoChatEnded {
    &self.inner
  }
}

/// A video chat or live stream was started
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentVideoChatStarted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PushMessageContentVideoChatStarted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentVideoChatStarted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentVideoChatStarted {}

impl PushMessageContentVideoChatStarted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentVideoChatStartedBuilder {
    let inner = PushMessageContentVideoChatStarted {
      td_name: "pushMessageContentVideoChatStarted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPushMessageContentVideoChatStartedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentVideoChatStartedBuilder {
  inner: PushMessageContentVideoChatStarted,
}

impl RTDPushMessageContentVideoChatStartedBuilder {
  pub fn build(&self) -> PushMessageContentVideoChatStarted {
    self.inner.clone()
  }
}

impl AsRef<PushMessageContentVideoChatStarted> for PushMessageContentVideoChatStarted {
  fn as_ref(&self) -> &PushMessageContentVideoChatStarted {
    self
  }
}

impl AsRef<PushMessageContentVideoChatStarted> for RTDPushMessageContentVideoChatStartedBuilder {
  fn as_ref(&self) -> &PushMessageContentVideoChatStarted {
    &self.inner
  }
}

/// A video note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message content; may be null
  video_note: Option<VideoNote>,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentVideoNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentVideoNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentVideoNote {}

impl PushMessageContentVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentVideoNoteBuilder {
    let inner = PushMessageContentVideoNote {
      td_name: "pushMessageContentVideoNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentVideoNoteBuilder { inner }
  }

  pub fn video_note(&self) -> &Option<VideoNote> {
    &self.video_note
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentVideoNoteBuilder {
  inner: PushMessageContentVideoNote,
}

impl RTDPushMessageContentVideoNoteBuilder {
  pub fn build(&self) -> PushMessageContentVideoNote {
    self.inner.clone()
  }

  pub fn video_note<T: AsRef<VideoNote>>(&mut self, video_note: T) -> &mut Self {
    self.inner.video_note = Some(video_note.as_ref().clone());
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentVideoNote> for PushMessageContentVideoNote {
  fn as_ref(&self) -> &PushMessageContentVideoNote {
    self
  }
}

impl AsRef<PushMessageContentVideoNote> for RTDPushMessageContentVideoNoteBuilder {
  fn as_ref(&self) -> &PushMessageContentVideoNote {
    &self.inner
  }
}

/// A voice note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message content; may be null
  voice_note: Option<VoiceNote>,
  /// True, if the message is a pinned message with the specified content
  is_pinned: bool,
}

impl RObject for PushMessageContentVoiceNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pushMessageContentVoiceNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPushMessageContent for PushMessageContentVoiceNote {}

impl PushMessageContentVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPushMessageContentVoiceNoteBuilder {
    let inner = PushMessageContentVoiceNote {
      td_name: "pushMessageContentVoiceNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPushMessageContentVoiceNoteBuilder { inner }
  }

  pub fn voice_note(&self) -> &Option<VoiceNote> {
    &self.voice_note
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }
}

#[doc(hidden)]
pub struct RTDPushMessageContentVoiceNoteBuilder {
  inner: PushMessageContentVoiceNote,
}

impl RTDPushMessageContentVoiceNoteBuilder {
  pub fn build(&self) -> PushMessageContentVoiceNote {
    self.inner.clone()
  }

  pub fn voice_note<T: AsRef<VoiceNote>>(&mut self, voice_note: T) -> &mut Self {
    self.inner.voice_note = Some(voice_note.as_ref().clone());
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }
}

impl AsRef<PushMessageContentVoiceNote> for PushMessageContentVoiceNote {
  fn as_ref(&self) -> &PushMessageContentVoiceNote {
    self
  }
}

impl AsRef<PushMessageContentVoiceNote> for RTDPushMessageContentVoiceNoteBuilder {
  fn as_ref(&self) -> &PushMessageContentVoiceNote {
    &self.inner
  }
}
