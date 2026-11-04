use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of link preview
pub trait TDLinkPreviewType: Debug + RObject {}

/// Describes type of link preview
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LinkPreviewType {
  #[doc(hidden)]
  _Default(()),
  /// The link is a link to a media album consisting of photos and videos
  Album(LinkPreviewTypeAlbum),
  /// The link is a link to an animation
  Animation(LinkPreviewTypeAnimation),
  /// The link is a link to an app at App Store or Google Play
  App(LinkPreviewTypeApp),
  /// The link is a link to a website
  Article(LinkPreviewTypeArticle),
  /// The link is a link to an audio
  Audio(LinkPreviewTypeAudio),
  /// The link is a link to a background. Link preview title and description are available only for filled backgrounds
  Background(LinkPreviewTypeBackground),
  /// The link is a link to boost a channel chat
  ChannelBoost(LinkPreviewTypeChannelBoost),
  /// The link is a link to a chat
  Chat(LinkPreviewTypeChat),
  /// The link is a link to a direct messages chat of a channel
  DirectMessagesChat(LinkPreviewTypeDirectMessagesChat),
  /// The link is a link to a general file
  Document(LinkPreviewTypeDocument),
  /// The link is a link to an animation player
  EmbeddedAnimationPlayer(LinkPreviewTypeEmbeddedAnimationPlayer),
  /// The link is a link to an audio player
  EmbeddedAudioPlayer(LinkPreviewTypeEmbeddedAudioPlayer),
  /// The link is a link to a video player
  EmbeddedVideoPlayer(LinkPreviewTypeEmbeddedVideoPlayer),
  /// The link is a link to an audio file
  ExternalAudio(LinkPreviewTypeExternalAudio),
  /// The link is a link to a video file
  ExternalVideo(LinkPreviewTypeExternalVideo),
  /// The link is a link to a gift auction
  GiftAuction(LinkPreviewTypeGiftAuction),
  /// The link is a link to a gift collection
  GiftCollection(LinkPreviewTypeGiftCollection),
  /// The link is a link to a group call that isn't bound to a chat
  GroupCall(LinkPreviewTypeGroupCall),
  /// The link is a link to an invoice
  Invoice(LinkPreviewTypeInvoice),
  /// The link is a link to a live story group call
  LiveStory(LinkPreviewTypeLiveStory),
  /// The link is a link to a text or a poll Telegram message
  Message(LinkPreviewTypeMessage),
  /// The link is a link to a photo
  Photo(LinkPreviewTypePhoto),
  /// The link is a link to a Telegram Premium gift code
  PremiumGiftCode(LinkPreviewTypePremiumGiftCode),
  /// The link is a link to a dialog for creating a managed bot
  RequestManagedBot(LinkPreviewTypeRequestManagedBot),
  /// The link is a link to a shareable chat folder
  ShareableChatFolder(LinkPreviewTypeShareableChatFolder),
  /// The link is a link to a sticker
  Sticker(LinkPreviewTypeSticker),
  /// The link is a link to a sticker set
  StickerSet(LinkPreviewTypeStickerSet),
  /// The link is a link to a story. Link preview description is unavailable
  Story(LinkPreviewTypeStory),
  /// The link is a link to an album of stories
  StoryAlbum(LinkPreviewTypeStoryAlbum),
  /// The link is a link to boost a supergroup chat
  SupergroupBoost(LinkPreviewTypeSupergroupBoost),
  /// The link is a link to a text composition style
  TextCompositionStyle(LinkPreviewTypeTextCompositionStyle),
  /// The link is a link to a cloud theme. TDLib has no theme support yet
  Theme(LinkPreviewTypeTheme),
  /// The link preview type is unsupported yet
  Unsupported(LinkPreviewTypeUnsupported),
  /// The link is a link to an upgraded gift
  UpgradedGift(LinkPreviewTypeUpgradedGift),
  /// The link is a link to a user
  User(LinkPreviewTypeUser),
  /// The link is a link to a video
  Video(LinkPreviewTypeVideo),
  /// The link is a link to a video chat
  VideoChat(LinkPreviewTypeVideoChat),
  /// The link is a link to a video note message
  VideoNote(LinkPreviewTypeVideoNote),
  /// The link is a link to a voice note message
  VoiceNote(LinkPreviewTypeVoiceNote),
  /// The link is a link to a Web App
  WebApp(LinkPreviewTypeWebApp),
}

impl Default for LinkPreviewType {
  fn default() -> Self {
    LinkPreviewType::_Default(())
  }
}

impl<'de> Deserialize<'de> for LinkPreviewType {
  fn deserialize<D>(deserializer: D) -> Result<LinkPreviewType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      LinkPreviewType,
      (linkPreviewTypeAlbum, Album);
      (linkPreviewTypeAnimation, Animation);
      (linkPreviewTypeApp, App);
      (linkPreviewTypeArticle, Article);
      (linkPreviewTypeAudio, Audio);
      (linkPreviewTypeBackground, Background);
      (linkPreviewTypeChannelBoost, ChannelBoost);
      (linkPreviewTypeChat, Chat);
      (linkPreviewTypeDirectMessagesChat, DirectMessagesChat);
      (linkPreviewTypeDocument, Document);
      (linkPreviewTypeEmbeddedAnimationPlayer, EmbeddedAnimationPlayer);
      (linkPreviewTypeEmbeddedAudioPlayer, EmbeddedAudioPlayer);
      (linkPreviewTypeEmbeddedVideoPlayer, EmbeddedVideoPlayer);
      (linkPreviewTypeExternalAudio, ExternalAudio);
      (linkPreviewTypeExternalVideo, ExternalVideo);
      (linkPreviewTypeGiftAuction, GiftAuction);
      (linkPreviewTypeGiftCollection, GiftCollection);
      (linkPreviewTypeGroupCall, GroupCall);
      (linkPreviewTypeInvoice, Invoice);
      (linkPreviewTypeLiveStory, LiveStory);
      (linkPreviewTypeMessage, Message);
      (linkPreviewTypePhoto, Photo);
      (linkPreviewTypePremiumGiftCode, PremiumGiftCode);
      (linkPreviewTypeRequestManagedBot, RequestManagedBot);
      (linkPreviewTypeShareableChatFolder, ShareableChatFolder);
      (linkPreviewTypeSticker, Sticker);
      (linkPreviewTypeStickerSet, StickerSet);
      (linkPreviewTypeStory, Story);
      (linkPreviewTypeStoryAlbum, StoryAlbum);
      (linkPreviewTypeSupergroupBoost, SupergroupBoost);
      (linkPreviewTypeTextCompositionStyle, TextCompositionStyle);
      (linkPreviewTypeTheme, Theme);
      (linkPreviewTypeUnsupported, Unsupported);
      (linkPreviewTypeUpgradedGift, UpgradedGift);
      (linkPreviewTypeUser, User);
      (linkPreviewTypeVideo, Video);
      (linkPreviewTypeVideoChat, VideoChat);
      (linkPreviewTypeVideoNote, VideoNote);
      (linkPreviewTypeVoiceNote, VoiceNote);
      (linkPreviewTypeWebApp, WebApp);

    )(deserializer)
  }
}

impl RObject for LinkPreviewType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      LinkPreviewType::Album(t) => t.td_name(),
      LinkPreviewType::Animation(t) => t.td_name(),
      LinkPreviewType::App(t) => t.td_name(),
      LinkPreviewType::Article(t) => t.td_name(),
      LinkPreviewType::Audio(t) => t.td_name(),
      LinkPreviewType::Background(t) => t.td_name(),
      LinkPreviewType::ChannelBoost(t) => t.td_name(),
      LinkPreviewType::Chat(t) => t.td_name(),
      LinkPreviewType::DirectMessagesChat(t) => t.td_name(),
      LinkPreviewType::Document(t) => t.td_name(),
      LinkPreviewType::EmbeddedAnimationPlayer(t) => t.td_name(),
      LinkPreviewType::EmbeddedAudioPlayer(t) => t.td_name(),
      LinkPreviewType::EmbeddedVideoPlayer(t) => t.td_name(),
      LinkPreviewType::ExternalAudio(t) => t.td_name(),
      LinkPreviewType::ExternalVideo(t) => t.td_name(),
      LinkPreviewType::GiftAuction(t) => t.td_name(),
      LinkPreviewType::GiftCollection(t) => t.td_name(),
      LinkPreviewType::GroupCall(t) => t.td_name(),
      LinkPreviewType::Invoice(t) => t.td_name(),
      LinkPreviewType::LiveStory(t) => t.td_name(),
      LinkPreviewType::Message(t) => t.td_name(),
      LinkPreviewType::Photo(t) => t.td_name(),
      LinkPreviewType::PremiumGiftCode(t) => t.td_name(),
      LinkPreviewType::RequestManagedBot(t) => t.td_name(),
      LinkPreviewType::ShareableChatFolder(t) => t.td_name(),
      LinkPreviewType::Sticker(t) => t.td_name(),
      LinkPreviewType::StickerSet(t) => t.td_name(),
      LinkPreviewType::Story(t) => t.td_name(),
      LinkPreviewType::StoryAlbum(t) => t.td_name(),
      LinkPreviewType::SupergroupBoost(t) => t.td_name(),
      LinkPreviewType::TextCompositionStyle(t) => t.td_name(),
      LinkPreviewType::Theme(t) => t.td_name(),
      LinkPreviewType::Unsupported(t) => t.td_name(),
      LinkPreviewType::UpgradedGift(t) => t.td_name(),
      LinkPreviewType::User(t) => t.td_name(),
      LinkPreviewType::Video(t) => t.td_name(),
      LinkPreviewType::VideoChat(t) => t.td_name(),
      LinkPreviewType::VideoNote(t) => t.td_name(),
      LinkPreviewType::VoiceNote(t) => t.td_name(),
      LinkPreviewType::WebApp(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      LinkPreviewType::Album(t) => t.extra(),
      LinkPreviewType::Animation(t) => t.extra(),
      LinkPreviewType::App(t) => t.extra(),
      LinkPreviewType::Article(t) => t.extra(),
      LinkPreviewType::Audio(t) => t.extra(),
      LinkPreviewType::Background(t) => t.extra(),
      LinkPreviewType::ChannelBoost(t) => t.extra(),
      LinkPreviewType::Chat(t) => t.extra(),
      LinkPreviewType::DirectMessagesChat(t) => t.extra(),
      LinkPreviewType::Document(t) => t.extra(),
      LinkPreviewType::EmbeddedAnimationPlayer(t) => t.extra(),
      LinkPreviewType::EmbeddedAudioPlayer(t) => t.extra(),
      LinkPreviewType::EmbeddedVideoPlayer(t) => t.extra(),
      LinkPreviewType::ExternalAudio(t) => t.extra(),
      LinkPreviewType::ExternalVideo(t) => t.extra(),
      LinkPreviewType::GiftAuction(t) => t.extra(),
      LinkPreviewType::GiftCollection(t) => t.extra(),
      LinkPreviewType::GroupCall(t) => t.extra(),
      LinkPreviewType::Invoice(t) => t.extra(),
      LinkPreviewType::LiveStory(t) => t.extra(),
      LinkPreviewType::Message(t) => t.extra(),
      LinkPreviewType::Photo(t) => t.extra(),
      LinkPreviewType::PremiumGiftCode(t) => t.extra(),
      LinkPreviewType::RequestManagedBot(t) => t.extra(),
      LinkPreviewType::ShareableChatFolder(t) => t.extra(),
      LinkPreviewType::Sticker(t) => t.extra(),
      LinkPreviewType::StickerSet(t) => t.extra(),
      LinkPreviewType::Story(t) => t.extra(),
      LinkPreviewType::StoryAlbum(t) => t.extra(),
      LinkPreviewType::SupergroupBoost(t) => t.extra(),
      LinkPreviewType::TextCompositionStyle(t) => t.extra(),
      LinkPreviewType::Theme(t) => t.extra(),
      LinkPreviewType::Unsupported(t) => t.extra(),
      LinkPreviewType::UpgradedGift(t) => t.extra(),
      LinkPreviewType::User(t) => t.extra(),
      LinkPreviewType::Video(t) => t.extra(),
      LinkPreviewType::VideoChat(t) => t.extra(),
      LinkPreviewType::VideoNote(t) => t.extra(),
      LinkPreviewType::VoiceNote(t) => t.extra(),
      LinkPreviewType::WebApp(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl LinkPreviewType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, LinkPreviewType::_Default(_))
  }

  pub fn is_album(&self) -> bool {
    matches!(self, LinkPreviewType::Album(_))
  }
  pub fn is_animation(&self) -> bool {
    matches!(self, LinkPreviewType::Animation(_))
  }
  pub fn is_app(&self) -> bool {
    matches!(self, LinkPreviewType::App(_))
  }
  pub fn is_article(&self) -> bool {
    matches!(self, LinkPreviewType::Article(_))
  }
  pub fn is_audio(&self) -> bool {
    matches!(self, LinkPreviewType::Audio(_))
  }
  pub fn is_background(&self) -> bool {
    matches!(self, LinkPreviewType::Background(_))
  }
  pub fn is_channel_boost(&self) -> bool {
    matches!(self, LinkPreviewType::ChannelBoost(_))
  }
  pub fn is_chat(&self) -> bool {
    matches!(self, LinkPreviewType::Chat(_))
  }
  pub fn is_direct_messages_chat(&self) -> bool {
    matches!(self, LinkPreviewType::DirectMessagesChat(_))
  }
  pub fn is_document(&self) -> bool {
    matches!(self, LinkPreviewType::Document(_))
  }
  pub fn is_embedded_animation_player(&self) -> bool {
    matches!(self, LinkPreviewType::EmbeddedAnimationPlayer(_))
  }
  pub fn is_embedded_audio_player(&self) -> bool {
    matches!(self, LinkPreviewType::EmbeddedAudioPlayer(_))
  }
  pub fn is_embedded_video_player(&self) -> bool {
    matches!(self, LinkPreviewType::EmbeddedVideoPlayer(_))
  }
  pub fn is_external_audio(&self) -> bool {
    matches!(self, LinkPreviewType::ExternalAudio(_))
  }
  pub fn is_external_video(&self) -> bool {
    matches!(self, LinkPreviewType::ExternalVideo(_))
  }
  pub fn is_gift_auction(&self) -> bool {
    matches!(self, LinkPreviewType::GiftAuction(_))
  }
  pub fn is_gift_collection(&self) -> bool {
    matches!(self, LinkPreviewType::GiftCollection(_))
  }
  pub fn is_group_call(&self) -> bool {
    matches!(self, LinkPreviewType::GroupCall(_))
  }
  pub fn is_invoice(&self) -> bool {
    matches!(self, LinkPreviewType::Invoice(_))
  }
  pub fn is_live_story(&self) -> bool {
    matches!(self, LinkPreviewType::LiveStory(_))
  }
  pub fn is_message(&self) -> bool {
    matches!(self, LinkPreviewType::Message(_))
  }
  pub fn is_photo(&self) -> bool {
    matches!(self, LinkPreviewType::Photo(_))
  }
  pub fn is_premium_gift_code(&self) -> bool {
    matches!(self, LinkPreviewType::PremiumGiftCode(_))
  }
  pub fn is_request_managed_bot(&self) -> bool {
    matches!(self, LinkPreviewType::RequestManagedBot(_))
  }
  pub fn is_shareable_chat_folder(&self) -> bool {
    matches!(self, LinkPreviewType::ShareableChatFolder(_))
  }
  pub fn is_sticker(&self) -> bool {
    matches!(self, LinkPreviewType::Sticker(_))
  }
  pub fn is_sticker_set(&self) -> bool {
    matches!(self, LinkPreviewType::StickerSet(_))
  }
  pub fn is_story(&self) -> bool {
    matches!(self, LinkPreviewType::Story(_))
  }
  pub fn is_story_album(&self) -> bool {
    matches!(self, LinkPreviewType::StoryAlbum(_))
  }
  pub fn is_supergroup_boost(&self) -> bool {
    matches!(self, LinkPreviewType::SupergroupBoost(_))
  }
  pub fn is_text_composition_style(&self) -> bool {
    matches!(self, LinkPreviewType::TextCompositionStyle(_))
  }
  pub fn is_theme(&self) -> bool {
    matches!(self, LinkPreviewType::Theme(_))
  }
  pub fn is_unsupported(&self) -> bool {
    matches!(self, LinkPreviewType::Unsupported(_))
  }
  pub fn is_upgraded_gift(&self) -> bool {
    matches!(self, LinkPreviewType::UpgradedGift(_))
  }
  pub fn is_user(&self) -> bool {
    matches!(self, LinkPreviewType::User(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, LinkPreviewType::Video(_))
  }
  pub fn is_video_chat(&self) -> bool {
    matches!(self, LinkPreviewType::VideoChat(_))
  }
  pub fn is_video_note(&self) -> bool {
    matches!(self, LinkPreviewType::VideoNote(_))
  }
  pub fn is_voice_note(&self) -> bool {
    matches!(self, LinkPreviewType::VoiceNote(_))
  }
  pub fn is_web_app(&self) -> bool {
    matches!(self, LinkPreviewType::WebApp(_))
  }

  pub fn on_album<F: FnOnce(&LinkPreviewTypeAlbum)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Album(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_animation<F: FnOnce(&LinkPreviewTypeAnimation)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Animation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_app<F: FnOnce(&LinkPreviewTypeApp)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::App(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_article<F: FnOnce(&LinkPreviewTypeArticle)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Article(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_audio<F: FnOnce(&LinkPreviewTypeAudio)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Audio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_background<F: FnOnce(&LinkPreviewTypeBackground)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Background(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel_boost<F: FnOnce(&LinkPreviewTypeChannelBoost)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::ChannelBoost(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat<F: FnOnce(&LinkPreviewTypeChat)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Chat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_direct_messages_chat<F: FnOnce(&LinkPreviewTypeDirectMessagesChat)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let LinkPreviewType::DirectMessagesChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_document<F: FnOnce(&LinkPreviewTypeDocument)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Document(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_embedded_animation_player<F: FnOnce(&LinkPreviewTypeEmbeddedAnimationPlayer)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let LinkPreviewType::EmbeddedAnimationPlayer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_embedded_audio_player<F: FnOnce(&LinkPreviewTypeEmbeddedAudioPlayer)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let LinkPreviewType::EmbeddedAudioPlayer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_embedded_video_player<F: FnOnce(&LinkPreviewTypeEmbeddedVideoPlayer)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let LinkPreviewType::EmbeddedVideoPlayer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_external_audio<F: FnOnce(&LinkPreviewTypeExternalAudio)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::ExternalAudio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_external_video<F: FnOnce(&LinkPreviewTypeExternalVideo)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::ExternalVideo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_auction<F: FnOnce(&LinkPreviewTypeGiftAuction)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::GiftAuction(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_collection<F: FnOnce(&LinkPreviewTypeGiftCollection)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::GiftCollection(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_group_call<F: FnOnce(&LinkPreviewTypeGroupCall)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::GroupCall(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_invoice<F: FnOnce(&LinkPreviewTypeInvoice)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Invoice(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_live_story<F: FnOnce(&LinkPreviewTypeLiveStory)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::LiveStory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message<F: FnOnce(&LinkPreviewTypeMessage)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_photo<F: FnOnce(&LinkPreviewTypePhoto)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_gift_code<F: FnOnce(&LinkPreviewTypePremiumGiftCode)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::PremiumGiftCode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_request_managed_bot<F: FnOnce(&LinkPreviewTypeRequestManagedBot)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let LinkPreviewType::RequestManagedBot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_shareable_chat_folder<F: FnOnce(&LinkPreviewTypeShareableChatFolder)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let LinkPreviewType::ShareableChatFolder(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sticker<F: FnOnce(&LinkPreviewTypeSticker)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Sticker(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sticker_set<F: FnOnce(&LinkPreviewTypeStickerSet)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::StickerSet(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story<F: FnOnce(&LinkPreviewTypeStory)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Story(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_album<F: FnOnce(&LinkPreviewTypeStoryAlbum)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::StoryAlbum(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_supergroup_boost<F: FnOnce(&LinkPreviewTypeSupergroupBoost)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::SupergroupBoost(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text_composition_style<F: FnOnce(&LinkPreviewTypeTextCompositionStyle)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let LinkPreviewType::TextCompositionStyle(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_theme<F: FnOnce(&LinkPreviewTypeTheme)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Theme(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unsupported<F: FnOnce(&LinkPreviewTypeUnsupported)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Unsupported(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift<F: FnOnce(&LinkPreviewTypeUpgradedGift)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::UpgradedGift(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user<F: FnOnce(&LinkPreviewTypeUser)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::User(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&LinkPreviewTypeVideo)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::Video(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_chat<F: FnOnce(&LinkPreviewTypeVideoChat)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::VideoChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_note<F: FnOnce(&LinkPreviewTypeVideoNote)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::VideoNote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_voice_note<F: FnOnce(&LinkPreviewTypeVoiceNote)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::VoiceNote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_app<F: FnOnce(&LinkPreviewTypeWebApp)>(&self, fnc: F) -> &Self {
    if let LinkPreviewType::WebApp(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_album(&self) -> Option<&LinkPreviewTypeAlbum> {
    if let LinkPreviewType::Album(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_animation(&self) -> Option<&LinkPreviewTypeAnimation> {
    if let LinkPreviewType::Animation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_app(&self) -> Option<&LinkPreviewTypeApp> {
    if let LinkPreviewType::App(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_article(&self) -> Option<&LinkPreviewTypeArticle> {
    if let LinkPreviewType::Article(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_audio(&self) -> Option<&LinkPreviewTypeAudio> {
    if let LinkPreviewType::Audio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_background(&self) -> Option<&LinkPreviewTypeBackground> {
    if let LinkPreviewType::Background(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel_boost(&self) -> Option<&LinkPreviewTypeChannelBoost> {
    if let LinkPreviewType::ChannelBoost(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat(&self) -> Option<&LinkPreviewTypeChat> {
    if let LinkPreviewType::Chat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_direct_messages_chat(&self) -> Option<&LinkPreviewTypeDirectMessagesChat> {
    if let LinkPreviewType::DirectMessagesChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_document(&self) -> Option<&LinkPreviewTypeDocument> {
    if let LinkPreviewType::Document(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_embedded_animation_player(&self) -> Option<&LinkPreviewTypeEmbeddedAnimationPlayer> {
    if let LinkPreviewType::EmbeddedAnimationPlayer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_embedded_audio_player(&self) -> Option<&LinkPreviewTypeEmbeddedAudioPlayer> {
    if let LinkPreviewType::EmbeddedAudioPlayer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_embedded_video_player(&self) -> Option<&LinkPreviewTypeEmbeddedVideoPlayer> {
    if let LinkPreviewType::EmbeddedVideoPlayer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_external_audio(&self) -> Option<&LinkPreviewTypeExternalAudio> {
    if let LinkPreviewType::ExternalAudio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_external_video(&self) -> Option<&LinkPreviewTypeExternalVideo> {
    if let LinkPreviewType::ExternalVideo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_auction(&self) -> Option<&LinkPreviewTypeGiftAuction> {
    if let LinkPreviewType::GiftAuction(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_collection(&self) -> Option<&LinkPreviewTypeGiftCollection> {
    if let LinkPreviewType::GiftCollection(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_group_call(&self) -> Option<&LinkPreviewTypeGroupCall> {
    if let LinkPreviewType::GroupCall(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_invoice(&self) -> Option<&LinkPreviewTypeInvoice> {
    if let LinkPreviewType::Invoice(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_live_story(&self) -> Option<&LinkPreviewTypeLiveStory> {
    if let LinkPreviewType::LiveStory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message(&self) -> Option<&LinkPreviewTypeMessage> {
    if let LinkPreviewType::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_photo(&self) -> Option<&LinkPreviewTypePhoto> {
    if let LinkPreviewType::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_gift_code(&self) -> Option<&LinkPreviewTypePremiumGiftCode> {
    if let LinkPreviewType::PremiumGiftCode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_request_managed_bot(&self) -> Option<&LinkPreviewTypeRequestManagedBot> {
    if let LinkPreviewType::RequestManagedBot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_shareable_chat_folder(&self) -> Option<&LinkPreviewTypeShareableChatFolder> {
    if let LinkPreviewType::ShareableChatFolder(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sticker(&self) -> Option<&LinkPreviewTypeSticker> {
    if let LinkPreviewType::Sticker(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sticker_set(&self) -> Option<&LinkPreviewTypeStickerSet> {
    if let LinkPreviewType::StickerSet(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story(&self) -> Option<&LinkPreviewTypeStory> {
    if let LinkPreviewType::Story(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_album(&self) -> Option<&LinkPreviewTypeStoryAlbum> {
    if let LinkPreviewType::StoryAlbum(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_supergroup_boost(&self) -> Option<&LinkPreviewTypeSupergroupBoost> {
    if let LinkPreviewType::SupergroupBoost(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text_composition_style(&self) -> Option<&LinkPreviewTypeTextCompositionStyle> {
    if let LinkPreviewType::TextCompositionStyle(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_theme(&self) -> Option<&LinkPreviewTypeTheme> {
    if let LinkPreviewType::Theme(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unsupported(&self) -> Option<&LinkPreviewTypeUnsupported> {
    if let LinkPreviewType::Unsupported(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift(&self) -> Option<&LinkPreviewTypeUpgradedGift> {
    if let LinkPreviewType::UpgradedGift(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user(&self) -> Option<&LinkPreviewTypeUser> {
    if let LinkPreviewType::User(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&LinkPreviewTypeVideo> {
    if let LinkPreviewType::Video(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_chat(&self) -> Option<&LinkPreviewTypeVideoChat> {
    if let LinkPreviewType::VideoChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_note(&self) -> Option<&LinkPreviewTypeVideoNote> {
    if let LinkPreviewType::VideoNote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_voice_note(&self) -> Option<&LinkPreviewTypeVoiceNote> {
    if let LinkPreviewType::VoiceNote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_app(&self) -> Option<&LinkPreviewTypeWebApp> {
    if let LinkPreviewType::WebApp(t) = self {
      return Some(t);
    }
    None
  }

  pub fn album<T: AsRef<LinkPreviewTypeAlbum>>(t: T) -> Self {
    LinkPreviewType::Album(t.as_ref().clone())
  }

  pub fn animation<T: AsRef<LinkPreviewTypeAnimation>>(t: T) -> Self {
    LinkPreviewType::Animation(t.as_ref().clone())
  }

  pub fn app<T: AsRef<LinkPreviewTypeApp>>(t: T) -> Self {
    LinkPreviewType::App(t.as_ref().clone())
  }

  pub fn article<T: AsRef<LinkPreviewTypeArticle>>(t: T) -> Self {
    LinkPreviewType::Article(t.as_ref().clone())
  }

  pub fn audio<T: AsRef<LinkPreviewTypeAudio>>(t: T) -> Self {
    LinkPreviewType::Audio(t.as_ref().clone())
  }

  pub fn background<T: AsRef<LinkPreviewTypeBackground>>(t: T) -> Self {
    LinkPreviewType::Background(t.as_ref().clone())
  }

  pub fn channel_boost<T: AsRef<LinkPreviewTypeChannelBoost>>(t: T) -> Self {
    LinkPreviewType::ChannelBoost(t.as_ref().clone())
  }

  pub fn chat<T: AsRef<LinkPreviewTypeChat>>(t: T) -> Self {
    LinkPreviewType::Chat(t.as_ref().clone())
  }

  pub fn direct_messages_chat<T: AsRef<LinkPreviewTypeDirectMessagesChat>>(t: T) -> Self {
    LinkPreviewType::DirectMessagesChat(t.as_ref().clone())
  }

  pub fn document<T: AsRef<LinkPreviewTypeDocument>>(t: T) -> Self {
    LinkPreviewType::Document(t.as_ref().clone())
  }

  pub fn embedded_animation_player<T: AsRef<LinkPreviewTypeEmbeddedAnimationPlayer>>(t: T) -> Self {
    LinkPreviewType::EmbeddedAnimationPlayer(t.as_ref().clone())
  }

  pub fn embedded_audio_player<T: AsRef<LinkPreviewTypeEmbeddedAudioPlayer>>(t: T) -> Self {
    LinkPreviewType::EmbeddedAudioPlayer(t.as_ref().clone())
  }

  pub fn embedded_video_player<T: AsRef<LinkPreviewTypeEmbeddedVideoPlayer>>(t: T) -> Self {
    LinkPreviewType::EmbeddedVideoPlayer(t.as_ref().clone())
  }

  pub fn external_audio<T: AsRef<LinkPreviewTypeExternalAudio>>(t: T) -> Self {
    LinkPreviewType::ExternalAudio(t.as_ref().clone())
  }

  pub fn external_video<T: AsRef<LinkPreviewTypeExternalVideo>>(t: T) -> Self {
    LinkPreviewType::ExternalVideo(t.as_ref().clone())
  }

  pub fn gift_auction<T: AsRef<LinkPreviewTypeGiftAuction>>(t: T) -> Self {
    LinkPreviewType::GiftAuction(t.as_ref().clone())
  }

  pub fn gift_collection<T: AsRef<LinkPreviewTypeGiftCollection>>(t: T) -> Self {
    LinkPreviewType::GiftCollection(t.as_ref().clone())
  }

  pub fn group_call<T: AsRef<LinkPreviewTypeGroupCall>>(t: T) -> Self {
    LinkPreviewType::GroupCall(t.as_ref().clone())
  }

  pub fn invoice<T: AsRef<LinkPreviewTypeInvoice>>(t: T) -> Self {
    LinkPreviewType::Invoice(t.as_ref().clone())
  }

  pub fn live_story<T: AsRef<LinkPreviewTypeLiveStory>>(t: T) -> Self {
    LinkPreviewType::LiveStory(t.as_ref().clone())
  }

  pub fn message<T: AsRef<LinkPreviewTypeMessage>>(t: T) -> Self {
    LinkPreviewType::Message(t.as_ref().clone())
  }

  pub fn photo<T: AsRef<LinkPreviewTypePhoto>>(t: T) -> Self {
    LinkPreviewType::Photo(t.as_ref().clone())
  }

  pub fn premium_gift_code<T: AsRef<LinkPreviewTypePremiumGiftCode>>(t: T) -> Self {
    LinkPreviewType::PremiumGiftCode(t.as_ref().clone())
  }

  pub fn request_managed_bot<T: AsRef<LinkPreviewTypeRequestManagedBot>>(t: T) -> Self {
    LinkPreviewType::RequestManagedBot(t.as_ref().clone())
  }

  pub fn shareable_chat_folder<T: AsRef<LinkPreviewTypeShareableChatFolder>>(t: T) -> Self {
    LinkPreviewType::ShareableChatFolder(t.as_ref().clone())
  }

  pub fn sticker<T: AsRef<LinkPreviewTypeSticker>>(t: T) -> Self {
    LinkPreviewType::Sticker(t.as_ref().clone())
  }

  pub fn sticker_set<T: AsRef<LinkPreviewTypeStickerSet>>(t: T) -> Self {
    LinkPreviewType::StickerSet(t.as_ref().clone())
  }

  pub fn story<T: AsRef<LinkPreviewTypeStory>>(t: T) -> Self {
    LinkPreviewType::Story(t.as_ref().clone())
  }

  pub fn story_album<T: AsRef<LinkPreviewTypeStoryAlbum>>(t: T) -> Self {
    LinkPreviewType::StoryAlbum(t.as_ref().clone())
  }

  pub fn supergroup_boost<T: AsRef<LinkPreviewTypeSupergroupBoost>>(t: T) -> Self {
    LinkPreviewType::SupergroupBoost(t.as_ref().clone())
  }

  pub fn text_composition_style<T: AsRef<LinkPreviewTypeTextCompositionStyle>>(t: T) -> Self {
    LinkPreviewType::TextCompositionStyle(t.as_ref().clone())
  }

  pub fn theme<T: AsRef<LinkPreviewTypeTheme>>(t: T) -> Self {
    LinkPreviewType::Theme(t.as_ref().clone())
  }

  pub fn unsupported<T: AsRef<LinkPreviewTypeUnsupported>>(t: T) -> Self {
    LinkPreviewType::Unsupported(t.as_ref().clone())
  }

  pub fn upgraded_gift<T: AsRef<LinkPreviewTypeUpgradedGift>>(t: T) -> Self {
    LinkPreviewType::UpgradedGift(t.as_ref().clone())
  }

  pub fn user<T: AsRef<LinkPreviewTypeUser>>(t: T) -> Self {
    LinkPreviewType::User(t.as_ref().clone())
  }

  pub fn video<T: AsRef<LinkPreviewTypeVideo>>(t: T) -> Self {
    LinkPreviewType::Video(t.as_ref().clone())
  }

  pub fn video_chat<T: AsRef<LinkPreviewTypeVideoChat>>(t: T) -> Self {
    LinkPreviewType::VideoChat(t.as_ref().clone())
  }

  pub fn video_note<T: AsRef<LinkPreviewTypeVideoNote>>(t: T) -> Self {
    LinkPreviewType::VideoNote(t.as_ref().clone())
  }

  pub fn voice_note<T: AsRef<LinkPreviewTypeVoiceNote>>(t: T) -> Self {
    LinkPreviewType::VoiceNote(t.as_ref().clone())
  }

  pub fn web_app<T: AsRef<LinkPreviewTypeWebApp>>(t: T) -> Self {
    LinkPreviewType::WebApp(t.as_ref().clone())
  }
}

impl AsRef<LinkPreviewType> for LinkPreviewType {
  fn as_ref(&self) -> &LinkPreviewType {
    self
  }
}

/// The link is a link to a media album consisting of photos and videos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeAlbum {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of album media
  media: Vec<LinkPreviewAlbumMedia>,
  /// Album caption
  caption: String,
}

impl RObject for LinkPreviewTypeAlbum {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeAlbum"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeAlbum {}

impl LinkPreviewTypeAlbum {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeAlbumBuilder {
    let inner = LinkPreviewTypeAlbum {
      td_name: "linkPreviewTypeAlbum".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeAlbumBuilder { inner }
  }

  pub fn media(&self) -> &Vec<LinkPreviewAlbumMedia> {
    &self.media
  }

  pub fn caption(&self) -> &String {
    &self.caption
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeAlbumBuilder {
  inner: LinkPreviewTypeAlbum,
}

impl RTDLinkPreviewTypeAlbumBuilder {
  pub fn build(&self) -> LinkPreviewTypeAlbum {
    self.inner.clone()
  }

  pub fn media(&mut self, media: Vec<LinkPreviewAlbumMedia>) -> &mut Self {
    self.inner.media = media;
    self
  }

  pub fn caption<T: AsRef<str>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().to_string();
    self
  }
}

impl AsRef<LinkPreviewTypeAlbum> for LinkPreviewTypeAlbum {
  fn as_ref(&self) -> &LinkPreviewTypeAlbum {
    self
  }
}

impl AsRef<LinkPreviewTypeAlbum> for RTDLinkPreviewTypeAlbumBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeAlbum {
    &self.inner
  }
}

/// The link is a link to an animation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The animation
  animation: Animation,
}

impl RObject for LinkPreviewTypeAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeAnimation {}

impl LinkPreviewTypeAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeAnimationBuilder {
    let inner = LinkPreviewTypeAnimation {
      td_name: "linkPreviewTypeAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &Animation {
    &self.animation
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeAnimationBuilder {
  inner: LinkPreviewTypeAnimation,
}

impl RTDLinkPreviewTypeAnimationBuilder {
  pub fn build(&self) -> LinkPreviewTypeAnimation {
    self.inner.clone()
  }

  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewTypeAnimation> for LinkPreviewTypeAnimation {
  fn as_ref(&self) -> &LinkPreviewTypeAnimation {
    self
  }
}

impl AsRef<LinkPreviewTypeAnimation> for RTDLinkPreviewTypeAnimationBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeAnimation {
    &self.inner
  }
}

/// The link is a link to an app at App Store or Google Play
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo for the app
  photo: Photo,
}

impl RObject for LinkPreviewTypeApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeApp {}

impl LinkPreviewTypeApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeAppBuilder {
    let inner = LinkPreviewTypeApp {
      td_name: "linkPreviewTypeApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeAppBuilder { inner }
  }

  pub fn photo(&self) -> &Photo {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeAppBuilder {
  inner: LinkPreviewTypeApp,
}

impl RTDLinkPreviewTypeAppBuilder {
  pub fn build(&self) -> LinkPreviewTypeApp {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewTypeApp> for LinkPreviewTypeApp {
  fn as_ref(&self) -> &LinkPreviewTypeApp {
    self
  }
}

impl AsRef<LinkPreviewTypeApp> for RTDLinkPreviewTypeAppBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeApp {
    &self.inner
  }
}

/// The link is a link to a website
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeArticle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Article's main photo; may be null
  photo: Option<Photo>,
}

impl RObject for LinkPreviewTypeArticle {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeArticle"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeArticle {}

impl LinkPreviewTypeArticle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeArticleBuilder {
    let inner = LinkPreviewTypeArticle {
      td_name: "linkPreviewTypeArticle".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeArticleBuilder { inner }
  }

  pub fn photo(&self) -> &Option<Photo> {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeArticleBuilder {
  inner: LinkPreviewTypeArticle,
}

impl RTDLinkPreviewTypeArticleBuilder {
  pub fn build(&self) -> LinkPreviewTypeArticle {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }
}

impl AsRef<LinkPreviewTypeArticle> for LinkPreviewTypeArticle {
  fn as_ref(&self) -> &LinkPreviewTypeArticle {
    self
  }
}

impl AsRef<LinkPreviewTypeArticle> for RTDLinkPreviewTypeArticleBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeArticle {
    &self.inner
  }
}

/// The link is a link to an audio
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The audio description
  audio: Audio,
}

impl RObject for LinkPreviewTypeAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeAudio {}

impl LinkPreviewTypeAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeAudioBuilder {
    let inner = LinkPreviewTypeAudio {
      td_name: "linkPreviewTypeAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeAudioBuilder { inner }
  }

  pub fn audio(&self) -> &Audio {
    &self.audio
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeAudioBuilder {
  inner: LinkPreviewTypeAudio,
}

impl RTDLinkPreviewTypeAudioBuilder {
  pub fn build(&self) -> LinkPreviewTypeAudio {
    self.inner.clone()
  }

  pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewTypeAudio> for LinkPreviewTypeAudio {
  fn as_ref(&self) -> &LinkPreviewTypeAudio {
    self
  }
}

impl AsRef<LinkPreviewTypeAudio> for RTDLinkPreviewTypeAudioBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeAudio {
    &self.inner
  }
}

/// The link is a link to a background. Link preview title and description are available only for filled backgrounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Document with the background; may be null for filled backgrounds
  document: Option<Document>,
  /// Type of the background; may be null if unknown
  background_type: Option<BackgroundType>,
}

impl RObject for LinkPreviewTypeBackground {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeBackground"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeBackground {}

impl LinkPreviewTypeBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeBackgroundBuilder {
    let inner = LinkPreviewTypeBackground {
      td_name: "linkPreviewTypeBackground".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeBackgroundBuilder { inner }
  }

  pub fn document(&self) -> &Option<Document> {
    &self.document
  }

  pub fn background_type(&self) -> &Option<BackgroundType> {
    &self.background_type
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeBackgroundBuilder {
  inner: LinkPreviewTypeBackground,
}

impl RTDLinkPreviewTypeBackgroundBuilder {
  pub fn build(&self) -> LinkPreviewTypeBackground {
    self.inner.clone()
  }

  pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
    self.inner.document = Some(document.as_ref().clone());
    self
  }

  pub fn background_type<T: AsRef<BackgroundType>>(&mut self, background_type: T) -> &mut Self {
    self.inner.background_type = Some(background_type.as_ref().clone());
    self
  }
}

impl AsRef<LinkPreviewTypeBackground> for LinkPreviewTypeBackground {
  fn as_ref(&self) -> &LinkPreviewTypeBackground {
    self
  }
}

impl AsRef<LinkPreviewTypeBackground> for RTDLinkPreviewTypeBackgroundBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeBackground {
    &self.inner
  }
}

/// The link is a link to boost a channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeChannelBoost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo of the chat; may be null
  photo: Option<ChatPhoto>,
}

impl RObject for LinkPreviewTypeChannelBoost {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeChannelBoost"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeChannelBoost {}

impl LinkPreviewTypeChannelBoost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeChannelBoostBuilder {
    let inner = LinkPreviewTypeChannelBoost {
      td_name: "linkPreviewTypeChannelBoost".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeChannelBoostBuilder { inner }
  }

  pub fn photo(&self) -> &Option<ChatPhoto> {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeChannelBoostBuilder {
  inner: LinkPreviewTypeChannelBoost,
}

impl RTDLinkPreviewTypeChannelBoostBuilder {
  pub fn build(&self) -> LinkPreviewTypeChannelBoost {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }
}

impl AsRef<LinkPreviewTypeChannelBoost> for LinkPreviewTypeChannelBoost {
  fn as_ref(&self) -> &LinkPreviewTypeChannelBoost {
    self
  }
}

impl AsRef<LinkPreviewTypeChannelBoost> for RTDLinkPreviewTypeChannelBoostBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeChannelBoost {
    &self.inner
  }
}

/// The link is a link to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the chat
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: InviteLinkChatType,
  /// Photo of the chat; may be null
  photo: Option<ChatPhoto>,
  /// True, if the link only creates join request
  creates_join_request: bool,
}

impl RObject for LinkPreviewTypeChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeChat {}

impl LinkPreviewTypeChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeChatBuilder {
    let inner = LinkPreviewTypeChat {
      td_name: "linkPreviewTypeChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeChatBuilder { inner }
  }

  pub fn type_(&self) -> &InviteLinkChatType {
    &self.type_
  }

  pub fn photo(&self) -> &Option<ChatPhoto> {
    &self.photo
  }

  pub fn creates_join_request(&self) -> bool {
    self.creates_join_request
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeChatBuilder {
  inner: LinkPreviewTypeChat,
}

impl RTDLinkPreviewTypeChatBuilder {
  pub fn build(&self) -> LinkPreviewTypeChat {
    self.inner.clone()
  }

  pub fn type_<T: AsRef<InviteLinkChatType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn creates_join_request(&mut self, creates_join_request: bool) -> &mut Self {
    self.inner.creates_join_request = creates_join_request;
    self
  }
}

impl AsRef<LinkPreviewTypeChat> for LinkPreviewTypeChat {
  fn as_ref(&self) -> &LinkPreviewTypeChat {
    self
  }
}

impl AsRef<LinkPreviewTypeChat> for RTDLinkPreviewTypeChatBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeChat {
    &self.inner
  }
}

/// The link is a link to a direct messages chat of a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeDirectMessagesChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo of the channel chat; may be null
  photo: Option<ChatPhoto>,
}

impl RObject for LinkPreviewTypeDirectMessagesChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeDirectMessagesChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeDirectMessagesChat {}

impl LinkPreviewTypeDirectMessagesChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeDirectMessagesChatBuilder {
    let inner = LinkPreviewTypeDirectMessagesChat {
      td_name: "linkPreviewTypeDirectMessagesChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeDirectMessagesChatBuilder { inner }
  }

  pub fn photo(&self) -> &Option<ChatPhoto> {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeDirectMessagesChatBuilder {
  inner: LinkPreviewTypeDirectMessagesChat,
}

impl RTDLinkPreviewTypeDirectMessagesChatBuilder {
  pub fn build(&self) -> LinkPreviewTypeDirectMessagesChat {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }
}

impl AsRef<LinkPreviewTypeDirectMessagesChat> for LinkPreviewTypeDirectMessagesChat {
  fn as_ref(&self) -> &LinkPreviewTypeDirectMessagesChat {
    self
  }
}

impl AsRef<LinkPreviewTypeDirectMessagesChat> for RTDLinkPreviewTypeDirectMessagesChatBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeDirectMessagesChat {
    &self.inner
  }
}

/// The link is a link to a general file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The document description
  document: Document,
}

impl RObject for LinkPreviewTypeDocument {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeDocument"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeDocument {}

impl LinkPreviewTypeDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeDocumentBuilder {
    let inner = LinkPreviewTypeDocument {
      td_name: "linkPreviewTypeDocument".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeDocumentBuilder { inner }
  }

  pub fn document(&self) -> &Document {
    &self.document
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeDocumentBuilder {
  inner: LinkPreviewTypeDocument,
}

impl RTDLinkPreviewTypeDocumentBuilder {
  pub fn build(&self) -> LinkPreviewTypeDocument {
    self.inner.clone()
  }

  pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewTypeDocument> for LinkPreviewTypeDocument {
  fn as_ref(&self) -> &LinkPreviewTypeDocument {
    self
  }
}

impl AsRef<LinkPreviewTypeDocument> for RTDLinkPreviewTypeDocumentBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeDocument {
    &self.inner
  }
}

/// The link is a link to an animation player
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeEmbeddedAnimationPlayer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the external animation player
  url: String,
  /// The cached animation; may be null if unknown
  animation: Option<Animation>,
  /// Thumbnail of the animation; may be null if unknown
  thumbnail: Option<Photo>,
  /// Duration of the animation, in seconds
  duration: i64,
  /// Expected width of the embedded player
  width: i64,
  /// Expected height of the embedded player
  height: i64,
}

impl RObject for LinkPreviewTypeEmbeddedAnimationPlayer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeEmbeddedAnimationPlayer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeEmbeddedAnimationPlayer {}

impl LinkPreviewTypeEmbeddedAnimationPlayer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeEmbeddedAnimationPlayerBuilder {
    let inner = LinkPreviewTypeEmbeddedAnimationPlayer {
      td_name: "linkPreviewTypeEmbeddedAnimationPlayer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeEmbeddedAnimationPlayerBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn animation(&self) -> &Option<Animation> {
    &self.animation
  }

  pub fn thumbnail(&self) -> &Option<Photo> {
    &self.thumbnail
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeEmbeddedAnimationPlayerBuilder {
  inner: LinkPreviewTypeEmbeddedAnimationPlayer,
}

impl RTDLinkPreviewTypeEmbeddedAnimationPlayerBuilder {
  pub fn build(&self) -> LinkPreviewTypeEmbeddedAnimationPlayer {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = Some(animation.as_ref().clone());
    self
  }

  pub fn thumbnail<T: AsRef<Photo>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }
}

impl AsRef<LinkPreviewTypeEmbeddedAnimationPlayer> for LinkPreviewTypeEmbeddedAnimationPlayer {
  fn as_ref(&self) -> &LinkPreviewTypeEmbeddedAnimationPlayer {
    self
  }
}

impl AsRef<LinkPreviewTypeEmbeddedAnimationPlayer>
  for RTDLinkPreviewTypeEmbeddedAnimationPlayerBuilder
{
  fn as_ref(&self) -> &LinkPreviewTypeEmbeddedAnimationPlayer {
    &self.inner
  }
}

/// The link is a link to an audio player
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeEmbeddedAudioPlayer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the external audio player
  url: String,
  /// The cached audio; may be null if unknown
  audio: Option<Audio>,
  /// Thumbnail of the audio; may be null if unknown
  thumbnail: Option<Photo>,
  /// Duration of the audio, in seconds
  duration: i64,
  /// Expected width of the embedded player
  width: i64,
  /// Expected height of the embedded player
  height: i64,
}

impl RObject for LinkPreviewTypeEmbeddedAudioPlayer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeEmbeddedAudioPlayer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeEmbeddedAudioPlayer {}

impl LinkPreviewTypeEmbeddedAudioPlayer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeEmbeddedAudioPlayerBuilder {
    let inner = LinkPreviewTypeEmbeddedAudioPlayer {
      td_name: "linkPreviewTypeEmbeddedAudioPlayer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeEmbeddedAudioPlayerBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn audio(&self) -> &Option<Audio> {
    &self.audio
  }

  pub fn thumbnail(&self) -> &Option<Photo> {
    &self.thumbnail
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeEmbeddedAudioPlayerBuilder {
  inner: LinkPreviewTypeEmbeddedAudioPlayer,
}

impl RTDLinkPreviewTypeEmbeddedAudioPlayerBuilder {
  pub fn build(&self) -> LinkPreviewTypeEmbeddedAudioPlayer {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = Some(audio.as_ref().clone());
    self
  }

  pub fn thumbnail<T: AsRef<Photo>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }
}

impl AsRef<LinkPreviewTypeEmbeddedAudioPlayer> for LinkPreviewTypeEmbeddedAudioPlayer {
  fn as_ref(&self) -> &LinkPreviewTypeEmbeddedAudioPlayer {
    self
  }
}

impl AsRef<LinkPreviewTypeEmbeddedAudioPlayer> for RTDLinkPreviewTypeEmbeddedAudioPlayerBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeEmbeddedAudioPlayer {
    &self.inner
  }
}

/// The link is a link to a video player
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeEmbeddedVideoPlayer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the external video player
  url: String,
  /// The cached video; may be null if unknown
  video: Option<Video>,
  /// Thumbnail of the video; may be null if unknown
  thumbnail: Option<Photo>,
  /// Duration of the video, in seconds
  duration: i64,
  /// Expected width of the embedded player
  width: i64,
  /// Expected height of the embedded player
  height: i64,
}

impl RObject for LinkPreviewTypeEmbeddedVideoPlayer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeEmbeddedVideoPlayer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeEmbeddedVideoPlayer {}

impl LinkPreviewTypeEmbeddedVideoPlayer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeEmbeddedVideoPlayerBuilder {
    let inner = LinkPreviewTypeEmbeddedVideoPlayer {
      td_name: "linkPreviewTypeEmbeddedVideoPlayer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeEmbeddedVideoPlayerBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn video(&self) -> &Option<Video> {
    &self.video
  }

  pub fn thumbnail(&self) -> &Option<Photo> {
    &self.thumbnail
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeEmbeddedVideoPlayerBuilder {
  inner: LinkPreviewTypeEmbeddedVideoPlayer,
}

impl RTDLinkPreviewTypeEmbeddedVideoPlayerBuilder {
  pub fn build(&self) -> LinkPreviewTypeEmbeddedVideoPlayer {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = Some(video.as_ref().clone());
    self
  }

  pub fn thumbnail<T: AsRef<Photo>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }
}

impl AsRef<LinkPreviewTypeEmbeddedVideoPlayer> for LinkPreviewTypeEmbeddedVideoPlayer {
  fn as_ref(&self) -> &LinkPreviewTypeEmbeddedVideoPlayer {
    self
  }
}

impl AsRef<LinkPreviewTypeEmbeddedVideoPlayer> for RTDLinkPreviewTypeEmbeddedVideoPlayerBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeEmbeddedVideoPlayer {
    &self.inner
  }
}

/// The link is a link to an audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeExternalAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the audio file
  url: String,
  /// MIME type of the audio file
  mime_type: String,
  /// Duration of the audio, in seconds; 0 if unknown
  duration: i64,
}

impl RObject for LinkPreviewTypeExternalAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeExternalAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeExternalAudio {}

impl LinkPreviewTypeExternalAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeExternalAudioBuilder {
    let inner = LinkPreviewTypeExternalAudio {
      td_name: "linkPreviewTypeExternalAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeExternalAudioBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn mime_type(&self) -> &String {
    &self.mime_type
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeExternalAudioBuilder {
  inner: LinkPreviewTypeExternalAudio,
}

impl RTDLinkPreviewTypeExternalAudioBuilder {
  pub fn build(&self) -> LinkPreviewTypeExternalAudio {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
    self.inner.mime_type = mime_type.as_ref().to_string();
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }
}

impl AsRef<LinkPreviewTypeExternalAudio> for LinkPreviewTypeExternalAudio {
  fn as_ref(&self) -> &LinkPreviewTypeExternalAudio {
    self
  }
}

impl AsRef<LinkPreviewTypeExternalAudio> for RTDLinkPreviewTypeExternalAudioBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeExternalAudio {
    &self.inner
  }
}

/// The link is a link to a video file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeExternalVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the video file
  url: String,
  /// MIME type of the video file
  mime_type: String,
  /// Expected width of the video preview; 0 if unknown
  width: i64,
  /// Expected height of the video preview; 0 if unknown
  height: i64,
  /// Duration of the video, in seconds; 0 if unknown
  duration: i64,
}

impl RObject for LinkPreviewTypeExternalVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeExternalVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeExternalVideo {}

impl LinkPreviewTypeExternalVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeExternalVideoBuilder {
    let inner = LinkPreviewTypeExternalVideo {
      td_name: "linkPreviewTypeExternalVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeExternalVideoBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn mime_type(&self) -> &String {
    &self.mime_type
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeExternalVideoBuilder {
  inner: LinkPreviewTypeExternalVideo,
}

impl RTDLinkPreviewTypeExternalVideoBuilder {
  pub fn build(&self) -> LinkPreviewTypeExternalVideo {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
    self.inner.mime_type = mime_type.as_ref().to_string();
    self
  }

  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }
}

impl AsRef<LinkPreviewTypeExternalVideo> for LinkPreviewTypeExternalVideo {
  fn as_ref(&self) -> &LinkPreviewTypeExternalVideo {
    self
  }
}

impl AsRef<LinkPreviewTypeExternalVideo> for RTDLinkPreviewTypeExternalVideoBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeExternalVideo {
    &self.inner
  }
}

/// The link is a link to a gift auction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeGiftAuction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: Gift,
  /// Point in time (Unix timestamp) when the auction will be ended
  auction_end_date: i64,
}

impl RObject for LinkPreviewTypeGiftAuction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeGiftAuction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeGiftAuction {}

impl LinkPreviewTypeGiftAuction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeGiftAuctionBuilder {
    let inner = LinkPreviewTypeGiftAuction {
      td_name: "linkPreviewTypeGiftAuction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeGiftAuctionBuilder { inner }
  }

  pub fn gift(&self) -> &Gift {
    &self.gift
  }

  pub fn auction_end_date(&self) -> i64 {
    self.auction_end_date
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeGiftAuctionBuilder {
  inner: LinkPreviewTypeGiftAuction,
}

impl RTDLinkPreviewTypeGiftAuctionBuilder {
  pub fn build(&self) -> LinkPreviewTypeGiftAuction {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<Gift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }

  pub fn auction_end_date(&mut self, auction_end_date: i64) -> &mut Self {
    self.inner.auction_end_date = auction_end_date;
    self
  }
}

impl AsRef<LinkPreviewTypeGiftAuction> for LinkPreviewTypeGiftAuction {
  fn as_ref(&self) -> &LinkPreviewTypeGiftAuction {
    self
  }
}

impl AsRef<LinkPreviewTypeGiftAuction> for RTDLinkPreviewTypeGiftAuctionBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeGiftAuction {
    &self.inner
  }
}

/// The link is a link to a gift collection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeGiftCollection {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Icons for some gifts from the collection; may be empty
  icons: Vec<Sticker>,
}

impl RObject for LinkPreviewTypeGiftCollection {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeGiftCollection"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeGiftCollection {}

impl LinkPreviewTypeGiftCollection {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeGiftCollectionBuilder {
    let inner = LinkPreviewTypeGiftCollection {
      td_name: "linkPreviewTypeGiftCollection".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeGiftCollectionBuilder { inner }
  }

  pub fn icons(&self) -> &Vec<Sticker> {
    &self.icons
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeGiftCollectionBuilder {
  inner: LinkPreviewTypeGiftCollection,
}

impl RTDLinkPreviewTypeGiftCollectionBuilder {
  pub fn build(&self) -> LinkPreviewTypeGiftCollection {
    self.inner.clone()
  }

  pub fn icons(&mut self, icons: Vec<Sticker>) -> &mut Self {
    self.inner.icons = icons;
    self
  }
}

impl AsRef<LinkPreviewTypeGiftCollection> for LinkPreviewTypeGiftCollection {
  fn as_ref(&self) -> &LinkPreviewTypeGiftCollection {
    self
  }
}

impl AsRef<LinkPreviewTypeGiftCollection> for RTDLinkPreviewTypeGiftCollectionBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeGiftCollection {
    &self.inner
  }
}

/// The link is a link to a group call that isn't bound to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeGroupCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for LinkPreviewTypeGroupCall {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeGroupCall"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeGroupCall {}

impl LinkPreviewTypeGroupCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeGroupCallBuilder {
    let inner = LinkPreviewTypeGroupCall {
      td_name: "linkPreviewTypeGroupCall".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDLinkPreviewTypeGroupCallBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeGroupCallBuilder {
  inner: LinkPreviewTypeGroupCall,
}

impl RTDLinkPreviewTypeGroupCallBuilder {
  pub fn build(&self) -> LinkPreviewTypeGroupCall {
    self.inner.clone()
  }
}

impl AsRef<LinkPreviewTypeGroupCall> for LinkPreviewTypeGroupCall {
  fn as_ref(&self) -> &LinkPreviewTypeGroupCall {
    self
  }
}

impl AsRef<LinkPreviewTypeGroupCall> for RTDLinkPreviewTypeGroupCallBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeGroupCall {
    &self.inner
  }
}

/// The link is a link to an invoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeInvoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for LinkPreviewTypeInvoice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeInvoice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeInvoice {}

impl LinkPreviewTypeInvoice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeInvoiceBuilder {
    let inner = LinkPreviewTypeInvoice {
      td_name: "linkPreviewTypeInvoice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDLinkPreviewTypeInvoiceBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeInvoiceBuilder {
  inner: LinkPreviewTypeInvoice,
}

impl RTDLinkPreviewTypeInvoiceBuilder {
  pub fn build(&self) -> LinkPreviewTypeInvoice {
    self.inner.clone()
  }
}

impl AsRef<LinkPreviewTypeInvoice> for LinkPreviewTypeInvoice {
  fn as_ref(&self) -> &LinkPreviewTypeInvoice {
    self
  }
}

impl AsRef<LinkPreviewTypeInvoice> for RTDLinkPreviewTypeInvoiceBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeInvoice {
    &self.inner
  }
}

/// The link is a link to a live story group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeLiveStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the chat that posted the story
  story_poster_chat_id: i64,
  /// Story identifier
  story_id: i64,
}

impl RObject for LinkPreviewTypeLiveStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeLiveStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeLiveStory {}

impl LinkPreviewTypeLiveStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeLiveStoryBuilder {
    let inner = LinkPreviewTypeLiveStory {
      td_name: "linkPreviewTypeLiveStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeLiveStoryBuilder { inner }
  }

  pub fn story_poster_chat_id(&self) -> i64 {
    self.story_poster_chat_id
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeLiveStoryBuilder {
  inner: LinkPreviewTypeLiveStory,
}

impl RTDLinkPreviewTypeLiveStoryBuilder {
  pub fn build(&self) -> LinkPreviewTypeLiveStory {
    self.inner.clone()
  }

  pub fn story_poster_chat_id(&mut self, story_poster_chat_id: i64) -> &mut Self {
    self.inner.story_poster_chat_id = story_poster_chat_id;
    self
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<LinkPreviewTypeLiveStory> for LinkPreviewTypeLiveStory {
  fn as_ref(&self) -> &LinkPreviewTypeLiveStory {
    self
  }
}

impl AsRef<LinkPreviewTypeLiveStory> for RTDLinkPreviewTypeLiveStoryBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeLiveStory {
    &self.inner
  }
}

/// The link is a link to a text or a poll Telegram message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for LinkPreviewTypeMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeMessage {}

impl LinkPreviewTypeMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeMessageBuilder {
    let inner = LinkPreviewTypeMessage {
      td_name: "linkPreviewTypeMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDLinkPreviewTypeMessageBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeMessageBuilder {
  inner: LinkPreviewTypeMessage,
}

impl RTDLinkPreviewTypeMessageBuilder {
  pub fn build(&self) -> LinkPreviewTypeMessage {
    self.inner.clone()
  }
}

impl AsRef<LinkPreviewTypeMessage> for LinkPreviewTypeMessage {
  fn as_ref(&self) -> &LinkPreviewTypeMessage {
    self
  }
}

impl AsRef<LinkPreviewTypeMessage> for RTDLinkPreviewTypeMessageBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeMessage {
    &self.inner
  }
}

/// The link is a link to a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The photo
  photo: Photo,
}

impl RObject for LinkPreviewTypePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypePhoto {}

impl LinkPreviewTypePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypePhotoBuilder {
    let inner = LinkPreviewTypePhoto {
      td_name: "linkPreviewTypePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypePhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Photo {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypePhotoBuilder {
  inner: LinkPreviewTypePhoto,
}

impl RTDLinkPreviewTypePhotoBuilder {
  pub fn build(&self) -> LinkPreviewTypePhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewTypePhoto> for LinkPreviewTypePhoto {
  fn as_ref(&self) -> &LinkPreviewTypePhoto {
    self
  }
}

impl AsRef<LinkPreviewTypePhoto> for RTDLinkPreviewTypePhotoBuilder {
  fn as_ref(&self) -> &LinkPreviewTypePhoto {
    &self.inner
  }
}

/// The link is a link to a Telegram Premium gift code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypePremiumGiftCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for LinkPreviewTypePremiumGiftCode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypePremiumGiftCode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypePremiumGiftCode {}

impl LinkPreviewTypePremiumGiftCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypePremiumGiftCodeBuilder {
    let inner = LinkPreviewTypePremiumGiftCode {
      td_name: "linkPreviewTypePremiumGiftCode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDLinkPreviewTypePremiumGiftCodeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypePremiumGiftCodeBuilder {
  inner: LinkPreviewTypePremiumGiftCode,
}

impl RTDLinkPreviewTypePremiumGiftCodeBuilder {
  pub fn build(&self) -> LinkPreviewTypePremiumGiftCode {
    self.inner.clone()
  }
}

impl AsRef<LinkPreviewTypePremiumGiftCode> for LinkPreviewTypePremiumGiftCode {
  fn as_ref(&self) -> &LinkPreviewTypePremiumGiftCode {
    self
  }
}

impl AsRef<LinkPreviewTypePremiumGiftCode> for RTDLinkPreviewTypePremiumGiftCodeBuilder {
  fn as_ref(&self) -> &LinkPreviewTypePremiumGiftCode {
    &self.inner
  }
}

/// The link is a link to a dialog for creating a managed bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeRequestManagedBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for LinkPreviewTypeRequestManagedBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeRequestManagedBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeRequestManagedBot {}

impl LinkPreviewTypeRequestManagedBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeRequestManagedBotBuilder {
    let inner = LinkPreviewTypeRequestManagedBot {
      td_name: "linkPreviewTypeRequestManagedBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDLinkPreviewTypeRequestManagedBotBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeRequestManagedBotBuilder {
  inner: LinkPreviewTypeRequestManagedBot,
}

impl RTDLinkPreviewTypeRequestManagedBotBuilder {
  pub fn build(&self) -> LinkPreviewTypeRequestManagedBot {
    self.inner.clone()
  }
}

impl AsRef<LinkPreviewTypeRequestManagedBot> for LinkPreviewTypeRequestManagedBot {
  fn as_ref(&self) -> &LinkPreviewTypeRequestManagedBot {
    self
  }
}

impl AsRef<LinkPreviewTypeRequestManagedBot> for RTDLinkPreviewTypeRequestManagedBotBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeRequestManagedBot {
    &self.inner
  }
}

/// The link is a link to a shareable chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeShareableChatFolder {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for LinkPreviewTypeShareableChatFolder {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeShareableChatFolder"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeShareableChatFolder {}

impl LinkPreviewTypeShareableChatFolder {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeShareableChatFolderBuilder {
    let inner = LinkPreviewTypeShareableChatFolder {
      td_name: "linkPreviewTypeShareableChatFolder".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDLinkPreviewTypeShareableChatFolderBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeShareableChatFolderBuilder {
  inner: LinkPreviewTypeShareableChatFolder,
}

impl RTDLinkPreviewTypeShareableChatFolderBuilder {
  pub fn build(&self) -> LinkPreviewTypeShareableChatFolder {
    self.inner.clone()
  }
}

impl AsRef<LinkPreviewTypeShareableChatFolder> for LinkPreviewTypeShareableChatFolder {
  fn as_ref(&self) -> &LinkPreviewTypeShareableChatFolder {
    self
  }
}

impl AsRef<LinkPreviewTypeShareableChatFolder> for RTDLinkPreviewTypeShareableChatFolderBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeShareableChatFolder {
    &self.inner
  }
}

/// The link is a link to a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The sticker. It can be an arbitrary WEBP image and can have dimensions bigger than 512
  sticker: Sticker,
}

impl RObject for LinkPreviewTypeSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeSticker {}

impl LinkPreviewTypeSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeStickerBuilder {
    let inner = LinkPreviewTypeSticker {
      td_name: "linkPreviewTypeSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &Sticker {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeStickerBuilder {
  inner: LinkPreviewTypeSticker,
}

impl RTDLinkPreviewTypeStickerBuilder {
  pub fn build(&self) -> LinkPreviewTypeSticker {
    self.inner.clone()
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewTypeSticker> for LinkPreviewTypeSticker {
  fn as_ref(&self) -> &LinkPreviewTypeSticker {
    self
  }
}

impl AsRef<LinkPreviewTypeSticker> for RTDLinkPreviewTypeStickerBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeSticker {
    &self.inner
  }
}

/// The link is a link to a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Up to 4 stickers from the sticker set
  stickers: Vec<Sticker>,
}

impl RObject for LinkPreviewTypeStickerSet {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeStickerSet"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeStickerSet {}

impl LinkPreviewTypeStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeStickerSetBuilder {
    let inner = LinkPreviewTypeStickerSet {
      td_name: "linkPreviewTypeStickerSet".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeStickerSetBuilder { inner }
  }

  pub fn stickers(&self) -> &Vec<Sticker> {
    &self.stickers
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeStickerSetBuilder {
  inner: LinkPreviewTypeStickerSet,
}

impl RTDLinkPreviewTypeStickerSetBuilder {
  pub fn build(&self) -> LinkPreviewTypeStickerSet {
    self.inner.clone()
  }

  pub fn stickers(&mut self, stickers: Vec<Sticker>) -> &mut Self {
    self.inner.stickers = stickers;
    self
  }
}

impl AsRef<LinkPreviewTypeStickerSet> for LinkPreviewTypeStickerSet {
  fn as_ref(&self) -> &LinkPreviewTypeStickerSet {
    self
  }
}

impl AsRef<LinkPreviewTypeStickerSet> for RTDLinkPreviewTypeStickerSetBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeStickerSet {
    &self.inner
  }
}

/// The link is a link to a story. Link preview description is unavailable
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The identifier of the chat that posted the story
  story_poster_chat_id: i64,
  /// Story identifier
  story_id: i64,
}

impl RObject for LinkPreviewTypeStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeStory {}

impl LinkPreviewTypeStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeStoryBuilder {
    let inner = LinkPreviewTypeStory {
      td_name: "linkPreviewTypeStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeStoryBuilder { inner }
  }

  pub fn story_poster_chat_id(&self) -> i64 {
    self.story_poster_chat_id
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeStoryBuilder {
  inner: LinkPreviewTypeStory,
}

impl RTDLinkPreviewTypeStoryBuilder {
  pub fn build(&self) -> LinkPreviewTypeStory {
    self.inner.clone()
  }

  pub fn story_poster_chat_id(&mut self, story_poster_chat_id: i64) -> &mut Self {
    self.inner.story_poster_chat_id = story_poster_chat_id;
    self
  }

  pub fn story_id(&mut self, story_id: i64) -> &mut Self {
    self.inner.story_id = story_id;
    self
  }
}

impl AsRef<LinkPreviewTypeStory> for LinkPreviewTypeStory {
  fn as_ref(&self) -> &LinkPreviewTypeStory {
    self
  }
}

impl AsRef<LinkPreviewTypeStory> for RTDLinkPreviewTypeStoryBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeStory {
    &self.inner
  }
}

/// The link is a link to an album of stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeStoryAlbum {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Icon of the album; may be null if none
  photo_icon: Option<Photo>,
  /// Video icon of the album; may be null if none
  video_icon: Option<Video>,
}

impl RObject for LinkPreviewTypeStoryAlbum {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeStoryAlbum"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeStoryAlbum {}

impl LinkPreviewTypeStoryAlbum {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeStoryAlbumBuilder {
    let inner = LinkPreviewTypeStoryAlbum {
      td_name: "linkPreviewTypeStoryAlbum".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeStoryAlbumBuilder { inner }
  }

  pub fn photo_icon(&self) -> &Option<Photo> {
    &self.photo_icon
  }

  pub fn video_icon(&self) -> &Option<Video> {
    &self.video_icon
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeStoryAlbumBuilder {
  inner: LinkPreviewTypeStoryAlbum,
}

impl RTDLinkPreviewTypeStoryAlbumBuilder {
  pub fn build(&self) -> LinkPreviewTypeStoryAlbum {
    self.inner.clone()
  }

  pub fn photo_icon<T: AsRef<Photo>>(&mut self, photo_icon: T) -> &mut Self {
    self.inner.photo_icon = Some(photo_icon.as_ref().clone());
    self
  }

  pub fn video_icon<T: AsRef<Video>>(&mut self, video_icon: T) -> &mut Self {
    self.inner.video_icon = Some(video_icon.as_ref().clone());
    self
  }
}

impl AsRef<LinkPreviewTypeStoryAlbum> for LinkPreviewTypeStoryAlbum {
  fn as_ref(&self) -> &LinkPreviewTypeStoryAlbum {
    self
  }
}

impl AsRef<LinkPreviewTypeStoryAlbum> for RTDLinkPreviewTypeStoryAlbumBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeStoryAlbum {
    &self.inner
  }
}

/// The link is a link to boost a supergroup chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeSupergroupBoost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo of the chat; may be null
  photo: Option<ChatPhoto>,
}

impl RObject for LinkPreviewTypeSupergroupBoost {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeSupergroupBoost"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeSupergroupBoost {}

impl LinkPreviewTypeSupergroupBoost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeSupergroupBoostBuilder {
    let inner = LinkPreviewTypeSupergroupBoost {
      td_name: "linkPreviewTypeSupergroupBoost".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeSupergroupBoostBuilder { inner }
  }

  pub fn photo(&self) -> &Option<ChatPhoto> {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeSupergroupBoostBuilder {
  inner: LinkPreviewTypeSupergroupBoost,
}

impl RTDLinkPreviewTypeSupergroupBoostBuilder {
  pub fn build(&self) -> LinkPreviewTypeSupergroupBoost {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }
}

impl AsRef<LinkPreviewTypeSupergroupBoost> for LinkPreviewTypeSupergroupBoost {
  fn as_ref(&self) -> &LinkPreviewTypeSupergroupBoost {
    self
  }
}

impl AsRef<LinkPreviewTypeSupergroupBoost> for RTDLinkPreviewTypeSupergroupBoostBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeSupergroupBoost {
    &self.inner
  }
}

/// The link is a link to a text composition style
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeTextCompositionStyle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the custom emoji corresponding to the style; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  custom_emoji_id: isize,
}

impl RObject for LinkPreviewTypeTextCompositionStyle {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeTextCompositionStyle"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeTextCompositionStyle {}

impl LinkPreviewTypeTextCompositionStyle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeTextCompositionStyleBuilder {
    let inner = LinkPreviewTypeTextCompositionStyle {
      td_name: "linkPreviewTypeTextCompositionStyle".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeTextCompositionStyleBuilder { inner }
  }

  pub fn custom_emoji_id(&self) -> isize {
    self.custom_emoji_id
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeTextCompositionStyleBuilder {
  inner: LinkPreviewTypeTextCompositionStyle,
}

impl RTDLinkPreviewTypeTextCompositionStyleBuilder {
  pub fn build(&self) -> LinkPreviewTypeTextCompositionStyle {
    self.inner.clone()
  }

  pub fn custom_emoji_id(&mut self, custom_emoji_id: isize) -> &mut Self {
    self.inner.custom_emoji_id = custom_emoji_id;
    self
  }
}

impl AsRef<LinkPreviewTypeTextCompositionStyle> for LinkPreviewTypeTextCompositionStyle {
  fn as_ref(&self) -> &LinkPreviewTypeTextCompositionStyle {
    self
  }
}

impl AsRef<LinkPreviewTypeTextCompositionStyle> for RTDLinkPreviewTypeTextCompositionStyleBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeTextCompositionStyle {
    &self.inner
  }
}

/// The link is a link to a cloud theme. TDLib has no theme support yet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeTheme {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of files with theme description
  documents: Vec<Document>,
  /// Settings for the cloud theme; may be null if unknown
  settings: Option<ThemeSettings>,
}

impl RObject for LinkPreviewTypeTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeTheme"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeTheme {}

impl LinkPreviewTypeTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeThemeBuilder {
    let inner = LinkPreviewTypeTheme {
      td_name: "linkPreviewTypeTheme".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeThemeBuilder { inner }
  }

  pub fn documents(&self) -> &Vec<Document> {
    &self.documents
  }

  pub fn settings(&self) -> &Option<ThemeSettings> {
    &self.settings
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeThemeBuilder {
  inner: LinkPreviewTypeTheme,
}

impl RTDLinkPreviewTypeThemeBuilder {
  pub fn build(&self) -> LinkPreviewTypeTheme {
    self.inner.clone()
  }

  pub fn documents(&mut self, documents: Vec<Document>) -> &mut Self {
    self.inner.documents = documents;
    self
  }

  pub fn settings<T: AsRef<ThemeSettings>>(&mut self, settings: T) -> &mut Self {
    self.inner.settings = Some(settings.as_ref().clone());
    self
  }
}

impl AsRef<LinkPreviewTypeTheme> for LinkPreviewTypeTheme {
  fn as_ref(&self) -> &LinkPreviewTypeTheme {
    self
  }
}

impl AsRef<LinkPreviewTypeTheme> for RTDLinkPreviewTypeThemeBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeTheme {
    &self.inner
  }
}

/// The link preview type is unsupported yet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for LinkPreviewTypeUnsupported {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeUnsupported"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeUnsupported {}

impl LinkPreviewTypeUnsupported {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeUnsupportedBuilder {
    let inner = LinkPreviewTypeUnsupported {
      td_name: "linkPreviewTypeUnsupported".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDLinkPreviewTypeUnsupportedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeUnsupportedBuilder {
  inner: LinkPreviewTypeUnsupported,
}

impl RTDLinkPreviewTypeUnsupportedBuilder {
  pub fn build(&self) -> LinkPreviewTypeUnsupported {
    self.inner.clone()
  }
}

impl AsRef<LinkPreviewTypeUnsupported> for LinkPreviewTypeUnsupported {
  fn as_ref(&self) -> &LinkPreviewTypeUnsupported {
    self
  }
}

impl AsRef<LinkPreviewTypeUnsupported> for RTDLinkPreviewTypeUnsupportedBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeUnsupported {
    &self.inner
  }
}

/// The link is a link to an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeUpgradedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: UpgradedGift,
}

impl RObject for LinkPreviewTypeUpgradedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeUpgradedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeUpgradedGift {}

impl LinkPreviewTypeUpgradedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeUpgradedGiftBuilder {
    let inner = LinkPreviewTypeUpgradedGift {
      td_name: "linkPreviewTypeUpgradedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeUpgradedGiftBuilder { inner }
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeUpgradedGiftBuilder {
  inner: LinkPreviewTypeUpgradedGift,
}

impl RTDLinkPreviewTypeUpgradedGiftBuilder {
  pub fn build(&self) -> LinkPreviewTypeUpgradedGift {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewTypeUpgradedGift> for LinkPreviewTypeUpgradedGift {
  fn as_ref(&self) -> &LinkPreviewTypeUpgradedGift {
    self
  }
}

impl AsRef<LinkPreviewTypeUpgradedGift> for RTDLinkPreviewTypeUpgradedGiftBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeUpgradedGift {
    &self.inner
  }
}

/// The link is a link to a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo of the user; may be null if none
  photo: Option<ChatPhoto>,
  /// True, if the user is a bot
  is_bot: bool,
}

impl RObject for LinkPreviewTypeUser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeUser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeUser {}

impl LinkPreviewTypeUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeUserBuilder {
    let inner = LinkPreviewTypeUser {
      td_name: "linkPreviewTypeUser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeUserBuilder { inner }
  }

  pub fn photo(&self) -> &Option<ChatPhoto> {
    &self.photo
  }

  pub fn is_bot(&self) -> bool {
    self.is_bot
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeUserBuilder {
  inner: LinkPreviewTypeUser,
}

impl RTDLinkPreviewTypeUserBuilder {
  pub fn build(&self) -> LinkPreviewTypeUser {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn is_bot(&mut self, is_bot: bool) -> &mut Self {
    self.inner.is_bot = is_bot;
    self
  }
}

impl AsRef<LinkPreviewTypeUser> for LinkPreviewTypeUser {
  fn as_ref(&self) -> &LinkPreviewTypeUser {
    self
  }
}

impl AsRef<LinkPreviewTypeUser> for RTDLinkPreviewTypeUserBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeUser {
    &self.inner
  }
}

/// The link is a link to a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The video description
  video: Video,
  /// Cover of the video; may be null if none
  cover: Option<Photo>,
  /// Timestamp from which the video playing must start, in seconds
  start_timestamp: i64,
}

impl RObject for LinkPreviewTypeVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeVideo {}

impl LinkPreviewTypeVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeVideoBuilder {
    let inner = LinkPreviewTypeVideo {
      td_name: "linkPreviewTypeVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeVideoBuilder { inner }
  }

  pub fn video(&self) -> &Video {
    &self.video
  }

  pub fn cover(&self) -> &Option<Photo> {
    &self.cover
  }

  pub fn start_timestamp(&self) -> i64 {
    self.start_timestamp
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeVideoBuilder {
  inner: LinkPreviewTypeVideo,
}

impl RTDLinkPreviewTypeVideoBuilder {
  pub fn build(&self) -> LinkPreviewTypeVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

  pub fn cover<T: AsRef<Photo>>(&mut self, cover: T) -> &mut Self {
    self.inner.cover = Some(cover.as_ref().clone());
    self
  }

  pub fn start_timestamp(&mut self, start_timestamp: i64) -> &mut Self {
    self.inner.start_timestamp = start_timestamp;
    self
  }
}

impl AsRef<LinkPreviewTypeVideo> for LinkPreviewTypeVideo {
  fn as_ref(&self) -> &LinkPreviewTypeVideo {
    self
  }
}

impl AsRef<LinkPreviewTypeVideo> for RTDLinkPreviewTypeVideoBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeVideo {
    &self.inner
  }
}

/// The link is a link to a video chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeVideoChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo of the chat with the video chat; may be null if none
  photo: Option<ChatPhoto>,
  /// True, if the video chat is expected to be a live stream in a channel or a broadcast group
  is_live_stream: bool,
  /// True, if the user can use the link to join the video chat without being muted by administrators
  joins_as_speaker: bool,
}

impl RObject for LinkPreviewTypeVideoChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeVideoChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeVideoChat {}

impl LinkPreviewTypeVideoChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeVideoChatBuilder {
    let inner = LinkPreviewTypeVideoChat {
      td_name: "linkPreviewTypeVideoChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeVideoChatBuilder { inner }
  }

  pub fn photo(&self) -> &Option<ChatPhoto> {
    &self.photo
  }

  pub fn is_live_stream(&self) -> bool {
    self.is_live_stream
  }

  pub fn joins_as_speaker(&self) -> bool {
    self.joins_as_speaker
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeVideoChatBuilder {
  inner: LinkPreviewTypeVideoChat,
}

impl RTDLinkPreviewTypeVideoChatBuilder {
  pub fn build(&self) -> LinkPreviewTypeVideoChat {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn is_live_stream(&mut self, is_live_stream: bool) -> &mut Self {
    self.inner.is_live_stream = is_live_stream;
    self
  }

  pub fn joins_as_speaker(&mut self, joins_as_speaker: bool) -> &mut Self {
    self.inner.joins_as_speaker = joins_as_speaker;
    self
  }
}

impl AsRef<LinkPreviewTypeVideoChat> for LinkPreviewTypeVideoChat {
  fn as_ref(&self) -> &LinkPreviewTypeVideoChat {
    self
  }
}

impl AsRef<LinkPreviewTypeVideoChat> for RTDLinkPreviewTypeVideoChatBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeVideoChat {
    &self.inner
  }
}

/// The link is a link to a video note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The video note
  video_note: VideoNote,
}

impl RObject for LinkPreviewTypeVideoNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeVideoNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeVideoNote {}

impl LinkPreviewTypeVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeVideoNoteBuilder {
    let inner = LinkPreviewTypeVideoNote {
      td_name: "linkPreviewTypeVideoNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeVideoNoteBuilder { inner }
  }

  pub fn video_note(&self) -> &VideoNote {
    &self.video_note
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeVideoNoteBuilder {
  inner: LinkPreviewTypeVideoNote,
}

impl RTDLinkPreviewTypeVideoNoteBuilder {
  pub fn build(&self) -> LinkPreviewTypeVideoNote {
    self.inner.clone()
  }

  pub fn video_note<T: AsRef<VideoNote>>(&mut self, video_note: T) -> &mut Self {
    self.inner.video_note = video_note.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewTypeVideoNote> for LinkPreviewTypeVideoNote {
  fn as_ref(&self) -> &LinkPreviewTypeVideoNote {
    self
  }
}

impl AsRef<LinkPreviewTypeVideoNote> for RTDLinkPreviewTypeVideoNoteBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeVideoNote {
    &self.inner
  }
}

/// The link is a link to a voice note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The voice note
  voice_note: VoiceNote,
}

impl RObject for LinkPreviewTypeVoiceNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeVoiceNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeVoiceNote {}

impl LinkPreviewTypeVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeVoiceNoteBuilder {
    let inner = LinkPreviewTypeVoiceNote {
      td_name: "linkPreviewTypeVoiceNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeVoiceNoteBuilder { inner }
  }

  pub fn voice_note(&self) -> &VoiceNote {
    &self.voice_note
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeVoiceNoteBuilder {
  inner: LinkPreviewTypeVoiceNote,
}

impl RTDLinkPreviewTypeVoiceNoteBuilder {
  pub fn build(&self) -> LinkPreviewTypeVoiceNote {
    self.inner.clone()
  }

  pub fn voice_note<T: AsRef<VoiceNote>>(&mut self, voice_note: T) -> &mut Self {
    self.inner.voice_note = voice_note.as_ref().clone();
    self
  }
}

impl AsRef<LinkPreviewTypeVoiceNote> for LinkPreviewTypeVoiceNote {
  fn as_ref(&self) -> &LinkPreviewTypeVoiceNote {
    self
  }
}

impl AsRef<LinkPreviewTypeVoiceNote> for RTDLinkPreviewTypeVoiceNoteBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeVoiceNote {
    &self.inner
  }
}

/// The link is a link to a Web App
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewTypeWebApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Web App photo; may be null if none
  photo: Option<Photo>,
}

impl RObject for LinkPreviewTypeWebApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "linkPreviewTypeWebApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDLinkPreviewType for LinkPreviewTypeWebApp {}

impl LinkPreviewTypeWebApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLinkPreviewTypeWebAppBuilder {
    let inner = LinkPreviewTypeWebApp {
      td_name: "linkPreviewTypeWebApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLinkPreviewTypeWebAppBuilder { inner }
  }

  pub fn photo(&self) -> &Option<Photo> {
    &self.photo
  }
}

#[doc(hidden)]
pub struct RTDLinkPreviewTypeWebAppBuilder {
  inner: LinkPreviewTypeWebApp,
}

impl RTDLinkPreviewTypeWebAppBuilder {
  pub fn build(&self) -> LinkPreviewTypeWebApp {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }
}

impl AsRef<LinkPreviewTypeWebApp> for LinkPreviewTypeWebApp {
  fn as_ref(&self) -> &LinkPreviewTypeWebApp {
    self
  }
}

impl AsRef<LinkPreviewTypeWebApp> for RTDLinkPreviewTypeWebAppBuilder {
  fn as_ref(&self) -> &LinkPreviewTypeWebApp {
    &self.inner
  }
}
