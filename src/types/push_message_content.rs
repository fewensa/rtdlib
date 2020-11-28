
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains content of a push message notification
pub trait TDPushMessageContent: Debug + RObject {}

/// Contains content of a push message notification
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PushMessageContent {
  #[doc(hidden)] _Default(()),
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
  /// A new member joined the chat by invite link
  ChatJoinByLink(PushMessageContentChatJoinByLink),
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
  /// A general message with hidden content
  Hidden(PushMessageContentHidden),
  /// A message with an invoice from a bot
  Invoice(PushMessageContentInvoice),
  /// A message with a location
  Location(PushMessageContentLocation),
  /// A media album
  MediaAlbum(PushMessageContentMediaAlbum),
  /// A forwarded messages
  MessageForwards(PushMessageContentMessageForwards),
  /// A photo message
  Photo(PushMessageContentPhoto),
  /// A message with a poll
  Poll(PushMessageContentPoll),
  /// A screenshot of a message in the chat has been taken
  ScreenshotTaken(PushMessageContentScreenshotTaken),
  /// A message with a sticker
  Sticker(PushMessageContentSticker),
  /// A text message
  Text(PushMessageContentText),
  /// A video message
  Video(PushMessageContentVideo),
  /// A video note message
  VideoNote(PushMessageContentVideoNote),
  /// A voice note message
  VoiceNote(PushMessageContentVoiceNote),

}

impl Default for PushMessageContent {
  fn default() -> Self { PushMessageContent::_Default(()) }
}

impl<'de> Deserialize<'de> for PushMessageContent {
  fn deserialize<D>(deserializer: D) -> Result<PushMessageContent, D::Error> where D: Deserializer<'de> {
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
      (pushMessageContentContact, Contact);
      (pushMessageContentContactRegistered, ContactRegistered);
      (pushMessageContentDocument, Document);
      (pushMessageContentGame, Game);
      (pushMessageContentGameScore, GameScore);
      (pushMessageContentHidden, Hidden);
      (pushMessageContentInvoice, Invoice);
      (pushMessageContentLocation, Location);
      (pushMessageContentMediaAlbum, MediaAlbum);
      (pushMessageContentMessageForwards, MessageForwards);
      (pushMessageContentPhoto, Photo);
      (pushMessageContentPoll, Poll);
      (pushMessageContentScreenshotTaken, ScreenshotTaken);
      (pushMessageContentSticker, Sticker);
      (pushMessageContentText, Text);
      (pushMessageContentVideo, Video);
      (pushMessageContentVideoNote, VideoNote);
      (pushMessageContentVoiceNote, VoiceNote);

    )(deserializer)
  }
}

impl RObject for PushMessageContent {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      PushMessageContent::Animation(t) => t.td_name(),
      PushMessageContent::Audio(t) => t.td_name(),
      PushMessageContent::BasicGroupChatCreate(t) => t.td_name(),
      PushMessageContent::ChatAddMembers(t) => t.td_name(),
      PushMessageContent::ChatChangePhoto(t) => t.td_name(),
      PushMessageContent::ChatChangeTitle(t) => t.td_name(),
      PushMessageContent::ChatDeleteMember(t) => t.td_name(),
      PushMessageContent::ChatJoinByLink(t) => t.td_name(),
      PushMessageContent::Contact(t) => t.td_name(),
      PushMessageContent::ContactRegistered(t) => t.td_name(),
      PushMessageContent::Document(t) => t.td_name(),
      PushMessageContent::Game(t) => t.td_name(),
      PushMessageContent::GameScore(t) => t.td_name(),
      PushMessageContent::Hidden(t) => t.td_name(),
      PushMessageContent::Invoice(t) => t.td_name(),
      PushMessageContent::Location(t) => t.td_name(),
      PushMessageContent::MediaAlbum(t) => t.td_name(),
      PushMessageContent::MessageForwards(t) => t.td_name(),
      PushMessageContent::Photo(t) => t.td_name(),
      PushMessageContent::Poll(t) => t.td_name(),
      PushMessageContent::ScreenshotTaken(t) => t.td_name(),
      PushMessageContent::Sticker(t) => t.td_name(),
      PushMessageContent::Text(t) => t.td_name(),
      PushMessageContent::Video(t) => t.td_name(),
      PushMessageContent::VideoNote(t) => t.td_name(),
      PushMessageContent::VoiceNote(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      PushMessageContent::Animation(t) => t.extra(),
      PushMessageContent::Audio(t) => t.extra(),
      PushMessageContent::BasicGroupChatCreate(t) => t.extra(),
      PushMessageContent::ChatAddMembers(t) => t.extra(),
      PushMessageContent::ChatChangePhoto(t) => t.extra(),
      PushMessageContent::ChatChangeTitle(t) => t.extra(),
      PushMessageContent::ChatDeleteMember(t) => t.extra(),
      PushMessageContent::ChatJoinByLink(t) => t.extra(),
      PushMessageContent::Contact(t) => t.extra(),
      PushMessageContent::ContactRegistered(t) => t.extra(),
      PushMessageContent::Document(t) => t.extra(),
      PushMessageContent::Game(t) => t.extra(),
      PushMessageContent::GameScore(t) => t.extra(),
      PushMessageContent::Hidden(t) => t.extra(),
      PushMessageContent::Invoice(t) => t.extra(),
      PushMessageContent::Location(t) => t.extra(),
      PushMessageContent::MediaAlbum(t) => t.extra(),
      PushMessageContent::MessageForwards(t) => t.extra(),
      PushMessageContent::Photo(t) => t.extra(),
      PushMessageContent::Poll(t) => t.extra(),
      PushMessageContent::ScreenshotTaken(t) => t.extra(),
      PushMessageContent::Sticker(t) => t.extra(),
      PushMessageContent::Text(t) => t.extra(),
      PushMessageContent::Video(t) => t.extra(),
      PushMessageContent::VideoNote(t) => t.extra(),
      PushMessageContent::VoiceNote(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl PushMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let PushMessageContent::_Default(_) = self { true } else { false } }

  pub fn is_animation(&self) -> bool { if let PushMessageContent::Animation(_) = self { true } else { false } }
  pub fn is_audio(&self) -> bool { if let PushMessageContent::Audio(_) = self { true } else { false } }
  pub fn is_basic_group_chat_create(&self) -> bool { if let PushMessageContent::BasicGroupChatCreate(_) = self { true } else { false } }
  pub fn is_chat_add_members(&self) -> bool { if let PushMessageContent::ChatAddMembers(_) = self { true } else { false } }
  pub fn is_chat_change_photo(&self) -> bool { if let PushMessageContent::ChatChangePhoto(_) = self { true } else { false } }
  pub fn is_chat_change_title(&self) -> bool { if let PushMessageContent::ChatChangeTitle(_) = self { true } else { false } }
  pub fn is_chat_delete_member(&self) -> bool { if let PushMessageContent::ChatDeleteMember(_) = self { true } else { false } }
  pub fn is_chat_join_by_link(&self) -> bool { if let PushMessageContent::ChatJoinByLink(_) = self { true } else { false } }
  pub fn is_contact(&self) -> bool { if let PushMessageContent::Contact(_) = self { true } else { false } }
  pub fn is_contact_registered(&self) -> bool { if let PushMessageContent::ContactRegistered(_) = self { true } else { false } }
  pub fn is_document(&self) -> bool { if let PushMessageContent::Document(_) = self { true } else { false } }
  pub fn is_game(&self) -> bool { if let PushMessageContent::Game(_) = self { true } else { false } }
  pub fn is_game_score(&self) -> bool { if let PushMessageContent::GameScore(_) = self { true } else { false } }
  pub fn is_hidden(&self) -> bool { if let PushMessageContent::Hidden(_) = self { true } else { false } }
  pub fn is_invoice(&self) -> bool { if let PushMessageContent::Invoice(_) = self { true } else { false } }
  pub fn is_location(&self) -> bool { if let PushMessageContent::Location(_) = self { true } else { false } }
  pub fn is_media_album(&self) -> bool { if let PushMessageContent::MediaAlbum(_) = self { true } else { false } }
  pub fn is_message_forwards(&self) -> bool { if let PushMessageContent::MessageForwards(_) = self { true } else { false } }
  pub fn is_photo(&self) -> bool { if let PushMessageContent::Photo(_) = self { true } else { false } }
  pub fn is_poll(&self) -> bool { if let PushMessageContent::Poll(_) = self { true } else { false } }
  pub fn is_screenshot_taken(&self) -> bool { if let PushMessageContent::ScreenshotTaken(_) = self { true } else { false } }
  pub fn is_sticker(&self) -> bool { if let PushMessageContent::Sticker(_) = self { true } else { false } }
  pub fn is_text(&self) -> bool { if let PushMessageContent::Text(_) = self { true } else { false } }
  pub fn is_video(&self) -> bool { if let PushMessageContent::Video(_) = self { true } else { false } }
  pub fn is_video_note(&self) -> bool { if let PushMessageContent::VideoNote(_) = self { true } else { false } }
  pub fn is_voice_note(&self) -> bool { if let PushMessageContent::VoiceNote(_) = self { true } else { false } }

  pub fn on_animation<F: FnOnce(&PushMessageContentAnimation)>(&self, fnc: F) -> &Self { if let PushMessageContent::Animation(t) = self { fnc(t) }; self }
  pub fn on_audio<F: FnOnce(&PushMessageContentAudio)>(&self, fnc: F) -> &Self { if let PushMessageContent::Audio(t) = self { fnc(t) }; self }
  pub fn on_basic_group_chat_create<F: FnOnce(&PushMessageContentBasicGroupChatCreate)>(&self, fnc: F) -> &Self { if let PushMessageContent::BasicGroupChatCreate(t) = self { fnc(t) }; self }
  pub fn on_chat_add_members<F: FnOnce(&PushMessageContentChatAddMembers)>(&self, fnc: F) -> &Self { if let PushMessageContent::ChatAddMembers(t) = self { fnc(t) }; self }
  pub fn on_chat_change_photo<F: FnOnce(&PushMessageContentChatChangePhoto)>(&self, fnc: F) -> &Self { if let PushMessageContent::ChatChangePhoto(t) = self { fnc(t) }; self }
  pub fn on_chat_change_title<F: FnOnce(&PushMessageContentChatChangeTitle)>(&self, fnc: F) -> &Self { if let PushMessageContent::ChatChangeTitle(t) = self { fnc(t) }; self }
  pub fn on_chat_delete_member<F: FnOnce(&PushMessageContentChatDeleteMember)>(&self, fnc: F) -> &Self { if let PushMessageContent::ChatDeleteMember(t) = self { fnc(t) }; self }
  pub fn on_chat_join_by_link<F: FnOnce(&PushMessageContentChatJoinByLink)>(&self, fnc: F) -> &Self { if let PushMessageContent::ChatJoinByLink(t) = self { fnc(t) }; self }
  pub fn on_contact<F: FnOnce(&PushMessageContentContact)>(&self, fnc: F) -> &Self { if let PushMessageContent::Contact(t) = self { fnc(t) }; self }
  pub fn on_contact_registered<F: FnOnce(&PushMessageContentContactRegistered)>(&self, fnc: F) -> &Self { if let PushMessageContent::ContactRegistered(t) = self { fnc(t) }; self }
  pub fn on_document<F: FnOnce(&PushMessageContentDocument)>(&self, fnc: F) -> &Self { if let PushMessageContent::Document(t) = self { fnc(t) }; self }
  pub fn on_game<F: FnOnce(&PushMessageContentGame)>(&self, fnc: F) -> &Self { if let PushMessageContent::Game(t) = self { fnc(t) }; self }
  pub fn on_game_score<F: FnOnce(&PushMessageContentGameScore)>(&self, fnc: F) -> &Self { if let PushMessageContent::GameScore(t) = self { fnc(t) }; self }
  pub fn on_hidden<F: FnOnce(&PushMessageContentHidden)>(&self, fnc: F) -> &Self { if let PushMessageContent::Hidden(t) = self { fnc(t) }; self }
  pub fn on_invoice<F: FnOnce(&PushMessageContentInvoice)>(&self, fnc: F) -> &Self { if let PushMessageContent::Invoice(t) = self { fnc(t) }; self }
  pub fn on_location<F: FnOnce(&PushMessageContentLocation)>(&self, fnc: F) -> &Self { if let PushMessageContent::Location(t) = self { fnc(t) }; self }
  pub fn on_media_album<F: FnOnce(&PushMessageContentMediaAlbum)>(&self, fnc: F) -> &Self { if let PushMessageContent::MediaAlbum(t) = self { fnc(t) }; self }
  pub fn on_message_forwards<F: FnOnce(&PushMessageContentMessageForwards)>(&self, fnc: F) -> &Self { if let PushMessageContent::MessageForwards(t) = self { fnc(t) }; self }
  pub fn on_photo<F: FnOnce(&PushMessageContentPhoto)>(&self, fnc: F) -> &Self { if let PushMessageContent::Photo(t) = self { fnc(t) }; self }
  pub fn on_poll<F: FnOnce(&PushMessageContentPoll)>(&self, fnc: F) -> &Self { if let PushMessageContent::Poll(t) = self { fnc(t) }; self }
  pub fn on_screenshot_taken<F: FnOnce(&PushMessageContentScreenshotTaken)>(&self, fnc: F) -> &Self { if let PushMessageContent::ScreenshotTaken(t) = self { fnc(t) }; self }
  pub fn on_sticker<F: FnOnce(&PushMessageContentSticker)>(&self, fnc: F) -> &Self { if let PushMessageContent::Sticker(t) = self { fnc(t) }; self }
  pub fn on_text<F: FnOnce(&PushMessageContentText)>(&self, fnc: F) -> &Self { if let PushMessageContent::Text(t) = self { fnc(t) }; self }
  pub fn on_video<F: FnOnce(&PushMessageContentVideo)>(&self, fnc: F) -> &Self { if let PushMessageContent::Video(t) = self { fnc(t) }; self }
  pub fn on_video_note<F: FnOnce(&PushMessageContentVideoNote)>(&self, fnc: F) -> &Self { if let PushMessageContent::VideoNote(t) = self { fnc(t) }; self }
  pub fn on_voice_note<F: FnOnce(&PushMessageContentVoiceNote)>(&self, fnc: F) -> &Self { if let PushMessageContent::VoiceNote(t) = self { fnc(t) }; self }

  pub fn as_animation(&self) -> Option<&PushMessageContentAnimation> { if let PushMessageContent::Animation(t) = self { return Some(t) } None }
  pub fn as_audio(&self) -> Option<&PushMessageContentAudio> { if let PushMessageContent::Audio(t) = self { return Some(t) } None }
  pub fn as_basic_group_chat_create(&self) -> Option<&PushMessageContentBasicGroupChatCreate> { if let PushMessageContent::BasicGroupChatCreate(t) = self { return Some(t) } None }
  pub fn as_chat_add_members(&self) -> Option<&PushMessageContentChatAddMembers> { if let PushMessageContent::ChatAddMembers(t) = self { return Some(t) } None }
  pub fn as_chat_change_photo(&self) -> Option<&PushMessageContentChatChangePhoto> { if let PushMessageContent::ChatChangePhoto(t) = self { return Some(t) } None }
  pub fn as_chat_change_title(&self) -> Option<&PushMessageContentChatChangeTitle> { if let PushMessageContent::ChatChangeTitle(t) = self { return Some(t) } None }
  pub fn as_chat_delete_member(&self) -> Option<&PushMessageContentChatDeleteMember> { if let PushMessageContent::ChatDeleteMember(t) = self { return Some(t) } None }
  pub fn as_chat_join_by_link(&self) -> Option<&PushMessageContentChatJoinByLink> { if let PushMessageContent::ChatJoinByLink(t) = self { return Some(t) } None }
  pub fn as_contact(&self) -> Option<&PushMessageContentContact> { if let PushMessageContent::Contact(t) = self { return Some(t) } None }
  pub fn as_contact_registered(&self) -> Option<&PushMessageContentContactRegistered> { if let PushMessageContent::ContactRegistered(t) = self { return Some(t) } None }
  pub fn as_document(&self) -> Option<&PushMessageContentDocument> { if let PushMessageContent::Document(t) = self { return Some(t) } None }
  pub fn as_game(&self) -> Option<&PushMessageContentGame> { if let PushMessageContent::Game(t) = self { return Some(t) } None }
  pub fn as_game_score(&self) -> Option<&PushMessageContentGameScore> { if let PushMessageContent::GameScore(t) = self { return Some(t) } None }
  pub fn as_hidden(&self) -> Option<&PushMessageContentHidden> { if let PushMessageContent::Hidden(t) = self { return Some(t) } None }
  pub fn as_invoice(&self) -> Option<&PushMessageContentInvoice> { if let PushMessageContent::Invoice(t) = self { return Some(t) } None }
  pub fn as_location(&self) -> Option<&PushMessageContentLocation> { if let PushMessageContent::Location(t) = self { return Some(t) } None }
  pub fn as_media_album(&self) -> Option<&PushMessageContentMediaAlbum> { if let PushMessageContent::MediaAlbum(t) = self { return Some(t) } None }
  pub fn as_message_forwards(&self) -> Option<&PushMessageContentMessageForwards> { if let PushMessageContent::MessageForwards(t) = self { return Some(t) } None }
  pub fn as_photo(&self) -> Option<&PushMessageContentPhoto> { if let PushMessageContent::Photo(t) = self { return Some(t) } None }
  pub fn as_poll(&self) -> Option<&PushMessageContentPoll> { if let PushMessageContent::Poll(t) = self { return Some(t) } None }
  pub fn as_screenshot_taken(&self) -> Option<&PushMessageContentScreenshotTaken> { if let PushMessageContent::ScreenshotTaken(t) = self { return Some(t) } None }
  pub fn as_sticker(&self) -> Option<&PushMessageContentSticker> { if let PushMessageContent::Sticker(t) = self { return Some(t) } None }
  pub fn as_text(&self) -> Option<&PushMessageContentText> { if let PushMessageContent::Text(t) = self { return Some(t) } None }
  pub fn as_video(&self) -> Option<&PushMessageContentVideo> { if let PushMessageContent::Video(t) = self { return Some(t) } None }
  pub fn as_video_note(&self) -> Option<&PushMessageContentVideoNote> { if let PushMessageContent::VideoNote(t) = self { return Some(t) } None }
  pub fn as_voice_note(&self) -> Option<&PushMessageContentVoiceNote> { if let PushMessageContent::VoiceNote(t) = self { return Some(t) } None }



  pub fn animation<T: AsRef<PushMessageContentAnimation>>(t: T) -> Self { PushMessageContent::Animation(t.as_ref().clone()) }

  pub fn audio<T: AsRef<PushMessageContentAudio>>(t: T) -> Self { PushMessageContent::Audio(t.as_ref().clone()) }

  pub fn basic_group_chat_create<T: AsRef<PushMessageContentBasicGroupChatCreate>>(t: T) -> Self { PushMessageContent::BasicGroupChatCreate(t.as_ref().clone()) }

  pub fn chat_add_members<T: AsRef<PushMessageContentChatAddMembers>>(t: T) -> Self { PushMessageContent::ChatAddMembers(t.as_ref().clone()) }

  pub fn chat_change_photo<T: AsRef<PushMessageContentChatChangePhoto>>(t: T) -> Self { PushMessageContent::ChatChangePhoto(t.as_ref().clone()) }

  pub fn chat_change_title<T: AsRef<PushMessageContentChatChangeTitle>>(t: T) -> Self { PushMessageContent::ChatChangeTitle(t.as_ref().clone()) }

  pub fn chat_delete_member<T: AsRef<PushMessageContentChatDeleteMember>>(t: T) -> Self { PushMessageContent::ChatDeleteMember(t.as_ref().clone()) }

  pub fn chat_join_by_link<T: AsRef<PushMessageContentChatJoinByLink>>(t: T) -> Self { PushMessageContent::ChatJoinByLink(t.as_ref().clone()) }

  pub fn contact<T: AsRef<PushMessageContentContact>>(t: T) -> Self { PushMessageContent::Contact(t.as_ref().clone()) }

  pub fn contact_registered<T: AsRef<PushMessageContentContactRegistered>>(t: T) -> Self { PushMessageContent::ContactRegistered(t.as_ref().clone()) }

  pub fn document<T: AsRef<PushMessageContentDocument>>(t: T) -> Self { PushMessageContent::Document(t.as_ref().clone()) }

  pub fn game<T: AsRef<PushMessageContentGame>>(t: T) -> Self { PushMessageContent::Game(t.as_ref().clone()) }

  pub fn game_score<T: AsRef<PushMessageContentGameScore>>(t: T) -> Self { PushMessageContent::GameScore(t.as_ref().clone()) }

  pub fn hidden<T: AsRef<PushMessageContentHidden>>(t: T) -> Self { PushMessageContent::Hidden(t.as_ref().clone()) }

  pub fn invoice<T: AsRef<PushMessageContentInvoice>>(t: T) -> Self { PushMessageContent::Invoice(t.as_ref().clone()) }

  pub fn location<T: AsRef<PushMessageContentLocation>>(t: T) -> Self { PushMessageContent::Location(t.as_ref().clone()) }

  pub fn media_album<T: AsRef<PushMessageContentMediaAlbum>>(t: T) -> Self { PushMessageContent::MediaAlbum(t.as_ref().clone()) }

  pub fn message_forwards<T: AsRef<PushMessageContentMessageForwards>>(t: T) -> Self { PushMessageContent::MessageForwards(t.as_ref().clone()) }

  pub fn photo<T: AsRef<PushMessageContentPhoto>>(t: T) -> Self { PushMessageContent::Photo(t.as_ref().clone()) }

  pub fn poll<T: AsRef<PushMessageContentPoll>>(t: T) -> Self { PushMessageContent::Poll(t.as_ref().clone()) }

  pub fn screenshot_taken<T: AsRef<PushMessageContentScreenshotTaken>>(t: T) -> Self { PushMessageContent::ScreenshotTaken(t.as_ref().clone()) }

  pub fn sticker<T: AsRef<PushMessageContentSticker>>(t: T) -> Self { PushMessageContent::Sticker(t.as_ref().clone()) }

  pub fn text<T: AsRef<PushMessageContentText>>(t: T) -> Self { PushMessageContent::Text(t.as_ref().clone()) }

  pub fn video<T: AsRef<PushMessageContentVideo>>(t: T) -> Self { PushMessageContent::Video(t.as_ref().clone()) }

  pub fn video_note<T: AsRef<PushMessageContentVideoNote>>(t: T) -> Self { PushMessageContent::VideoNote(t.as_ref().clone()) }

  pub fn voice_note<T: AsRef<PushMessageContentVoiceNote>>(t: T) -> Self { PushMessageContent::VoiceNote(t.as_ref().clone()) }

}

impl AsRef<PushMessageContent> for PushMessageContent {
  fn as_ref(&self) -> &PushMessageContent { self }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentAnimation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentAnimation {}



impl PushMessageContentAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentAnimationBuilder {
    let mut inner = PushMessageContentAnimation::default();
    inner.td_name = "pushMessageContentAnimation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &Option<Animation> { &self.animation }

  pub fn caption(&self) -> &String { &self.caption }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentAnimationBuilder {
  inner: PushMessageContentAnimation
}

impl RTDPushMessageContentAnimationBuilder {
  pub fn build(&self) -> PushMessageContentAnimation { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentAnimation { self }
}

impl AsRef<PushMessageContentAnimation> for RTDPushMessageContentAnimationBuilder {
  fn as_ref(&self) -> &PushMessageContentAnimation { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentAudio" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentAudio {}



impl PushMessageContentAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentAudioBuilder {
    let mut inner = PushMessageContentAudio::default();
    inner.td_name = "pushMessageContentAudio".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentAudioBuilder { inner }
  }

  pub fn audio(&self) -> &Option<Audio> { &self.audio }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentAudioBuilder {
  inner: PushMessageContentAudio
}

impl RTDPushMessageContentAudioBuilder {
  pub fn build(&self) -> PushMessageContentAudio { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentAudio { self }
}

impl AsRef<PushMessageContentAudio> for RTDPushMessageContentAudioBuilder {
  fn as_ref(&self) -> &PushMessageContentAudio { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentBasicGroupChatCreate" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentBasicGroupChatCreate {}



impl PushMessageContentBasicGroupChatCreate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentBasicGroupChatCreateBuilder {
    let mut inner = PushMessageContentBasicGroupChatCreate::default();
    inner.td_name = "pushMessageContentBasicGroupChatCreate".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentBasicGroupChatCreateBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPushMessageContentBasicGroupChatCreateBuilder {
  inner: PushMessageContentBasicGroupChatCreate
}

impl RTDPushMessageContentBasicGroupChatCreateBuilder {
  pub fn build(&self) -> PushMessageContentBasicGroupChatCreate { self.inner.clone() }

}

impl AsRef<PushMessageContentBasicGroupChatCreate> for PushMessageContentBasicGroupChatCreate {
  fn as_ref(&self) -> &PushMessageContentBasicGroupChatCreate { self }
}

impl AsRef<PushMessageContentBasicGroupChatCreate> for RTDPushMessageContentBasicGroupChatCreateBuilder {
  fn as_ref(&self) -> &PushMessageContentBasicGroupChatCreate { &self.inner }
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
  /// True, if the user has returned to the group themself
  is_returned: bool,
  
}

impl RObject for PushMessageContentChatAddMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatAddMembers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentChatAddMembers {}



impl PushMessageContentChatAddMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentChatAddMembersBuilder {
    let mut inner = PushMessageContentChatAddMembers::default();
    inner.td_name = "pushMessageContentChatAddMembers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentChatAddMembersBuilder { inner }
  }

  pub fn member_name(&self) -> &String { &self.member_name }

  pub fn is_current_user(&self) -> bool { self.is_current_user }

  pub fn is_returned(&self) -> bool { self.is_returned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentChatAddMembersBuilder {
  inner: PushMessageContentChatAddMembers
}

impl RTDPushMessageContentChatAddMembersBuilder {
  pub fn build(&self) -> PushMessageContentChatAddMembers { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentChatAddMembers { self }
}

impl AsRef<PushMessageContentChatAddMembers> for RTDPushMessageContentChatAddMembersBuilder {
  fn as_ref(&self) -> &PushMessageContentChatAddMembers { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatChangePhoto" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentChatChangePhoto {}



impl PushMessageContentChatChangePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentChatChangePhotoBuilder {
    let mut inner = PushMessageContentChatChangePhoto::default();
    inner.td_name = "pushMessageContentChatChangePhoto".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentChatChangePhotoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPushMessageContentChatChangePhotoBuilder {
  inner: PushMessageContentChatChangePhoto
}

impl RTDPushMessageContentChatChangePhotoBuilder {
  pub fn build(&self) -> PushMessageContentChatChangePhoto { self.inner.clone() }

}

impl AsRef<PushMessageContentChatChangePhoto> for PushMessageContentChatChangePhoto {
  fn as_ref(&self) -> &PushMessageContentChatChangePhoto { self }
}

impl AsRef<PushMessageContentChatChangePhoto> for RTDPushMessageContentChatChangePhotoBuilder {
  fn as_ref(&self) -> &PushMessageContentChatChangePhoto { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatChangeTitle" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentChatChangeTitle {}



impl PushMessageContentChatChangeTitle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentChatChangeTitleBuilder {
    let mut inner = PushMessageContentChatChangeTitle::default();
    inner.td_name = "pushMessageContentChatChangeTitle".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentChatChangeTitleBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDPushMessageContentChatChangeTitleBuilder {
  inner: PushMessageContentChatChangeTitle
}

impl RTDPushMessageContentChatChangeTitleBuilder {
  pub fn build(&self) -> PushMessageContentChatChangeTitle { self.inner.clone() }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<PushMessageContentChatChangeTitle> for PushMessageContentChatChangeTitle {
  fn as_ref(&self) -> &PushMessageContentChatChangeTitle { self }
}

impl AsRef<PushMessageContentChatChangeTitle> for RTDPushMessageContentChatChangeTitleBuilder {
  fn as_ref(&self) -> &PushMessageContentChatChangeTitle { &self.inner }
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
  /// True, if the user has left the group themself
  is_left: bool,
  
}

impl RObject for PushMessageContentChatDeleteMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatDeleteMember" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentChatDeleteMember {}



impl PushMessageContentChatDeleteMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentChatDeleteMemberBuilder {
    let mut inner = PushMessageContentChatDeleteMember::default();
    inner.td_name = "pushMessageContentChatDeleteMember".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentChatDeleteMemberBuilder { inner }
  }

  pub fn member_name(&self) -> &String { &self.member_name }

  pub fn is_current_user(&self) -> bool { self.is_current_user }

  pub fn is_left(&self) -> bool { self.is_left }

}

#[doc(hidden)]
pub struct RTDPushMessageContentChatDeleteMemberBuilder {
  inner: PushMessageContentChatDeleteMember
}

impl RTDPushMessageContentChatDeleteMemberBuilder {
  pub fn build(&self) -> PushMessageContentChatDeleteMember { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentChatDeleteMember { self }
}

impl AsRef<PushMessageContentChatDeleteMember> for RTDPushMessageContentChatDeleteMemberBuilder {
  fn as_ref(&self) -> &PushMessageContentChatDeleteMember { &self.inner }
}







/// A new member joined the chat by invite link
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentChatJoinByLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentChatJoinByLink {}



impl PushMessageContentChatJoinByLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentChatJoinByLinkBuilder {
    let mut inner = PushMessageContentChatJoinByLink::default();
    inner.td_name = "pushMessageContentChatJoinByLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentChatJoinByLinkBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPushMessageContentChatJoinByLinkBuilder {
  inner: PushMessageContentChatJoinByLink
}

impl RTDPushMessageContentChatJoinByLinkBuilder {
  pub fn build(&self) -> PushMessageContentChatJoinByLink { self.inner.clone() }

}

impl AsRef<PushMessageContentChatJoinByLink> for PushMessageContentChatJoinByLink {
  fn as_ref(&self) -> &PushMessageContentChatJoinByLink { self }
}

impl AsRef<PushMessageContentChatJoinByLink> for RTDPushMessageContentChatJoinByLinkBuilder {
  fn as_ref(&self) -> &PushMessageContentChatJoinByLink { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentContact" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentContact {}



impl PushMessageContentContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentContactBuilder {
    let mut inner = PushMessageContentContact::default();
    inner.td_name = "pushMessageContentContact".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentContactBuilder { inner }
  }

  pub fn name(&self) -> &String { &self.name }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentContactBuilder {
  inner: PushMessageContentContact
}

impl RTDPushMessageContentContactBuilder {
  pub fn build(&self) -> PushMessageContentContact { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentContact { self }
}

impl AsRef<PushMessageContentContact> for RTDPushMessageContentContactBuilder {
  fn as_ref(&self) -> &PushMessageContentContact { &self.inner }
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
  
}

impl RObject for PushMessageContentContactRegistered {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentContactRegistered" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentContactRegistered {}



impl PushMessageContentContactRegistered {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentContactRegisteredBuilder {
    let mut inner = PushMessageContentContactRegistered::default();
    inner.td_name = "pushMessageContentContactRegistered".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentContactRegisteredBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPushMessageContentContactRegisteredBuilder {
  inner: PushMessageContentContactRegistered
}

impl RTDPushMessageContentContactRegisteredBuilder {
  pub fn build(&self) -> PushMessageContentContactRegistered { self.inner.clone() }

}

impl AsRef<PushMessageContentContactRegistered> for PushMessageContentContactRegistered {
  fn as_ref(&self) -> &PushMessageContentContactRegistered { self }
}

impl AsRef<PushMessageContentContactRegistered> for RTDPushMessageContentContactRegisteredBuilder {
  fn as_ref(&self) -> &PushMessageContentContactRegistered { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentDocument" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentDocument {}



impl PushMessageContentDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentDocumentBuilder {
    let mut inner = PushMessageContentDocument::default();
    inner.td_name = "pushMessageContentDocument".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentDocumentBuilder { inner }
  }

  pub fn document(&self) -> &Option<Document> { &self.document }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentDocumentBuilder {
  inner: PushMessageContentDocument
}

impl RTDPushMessageContentDocumentBuilder {
  pub fn build(&self) -> PushMessageContentDocument { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentDocument { self }
}

impl AsRef<PushMessageContentDocument> for RTDPushMessageContentDocumentBuilder {
  fn as_ref(&self) -> &PushMessageContentDocument { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentGame" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentGame {}



impl PushMessageContentGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentGameBuilder {
    let mut inner = PushMessageContentGame::default();
    inner.td_name = "pushMessageContentGame".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentGameBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentGameBuilder {
  inner: PushMessageContentGame
}

impl RTDPushMessageContentGameBuilder {
  pub fn build(&self) -> PushMessageContentGame { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentGame { self }
}

impl AsRef<PushMessageContentGame> for RTDPushMessageContentGameBuilder {
  fn as_ref(&self) -> &PushMessageContentGame { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentGameScore" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentGameScore {}



impl PushMessageContentGameScore {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentGameScoreBuilder {
    let mut inner = PushMessageContentGameScore::default();
    inner.td_name = "pushMessageContentGameScore".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentGameScoreBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

  pub fn score(&self) -> i64 { self.score }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentGameScoreBuilder {
  inner: PushMessageContentGameScore
}

impl RTDPushMessageContentGameScoreBuilder {
  pub fn build(&self) -> PushMessageContentGameScore { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentGameScore { self }
}

impl AsRef<PushMessageContentGameScore> for RTDPushMessageContentGameScoreBuilder {
  fn as_ref(&self) -> &PushMessageContentGameScore { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentHidden" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentHidden {}



impl PushMessageContentHidden {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentHiddenBuilder {
    let mut inner = PushMessageContentHidden::default();
    inner.td_name = "pushMessageContentHidden".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentHiddenBuilder { inner }
  }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentHiddenBuilder {
  inner: PushMessageContentHidden
}

impl RTDPushMessageContentHiddenBuilder {
  pub fn build(&self) -> PushMessageContentHidden { self.inner.clone() }

   
  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }

}

impl AsRef<PushMessageContentHidden> for PushMessageContentHidden {
  fn as_ref(&self) -> &PushMessageContentHidden { self }
}

impl AsRef<PushMessageContentHidden> for RTDPushMessageContentHiddenBuilder {
  fn as_ref(&self) -> &PushMessageContentHidden { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentInvoice" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentInvoice {}



impl PushMessageContentInvoice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentInvoiceBuilder {
    let mut inner = PushMessageContentInvoice::default();
    inner.td_name = "pushMessageContentInvoice".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentInvoiceBuilder { inner }
  }

  pub fn price(&self) -> &String { &self.price }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentInvoiceBuilder {
  inner: PushMessageContentInvoice
}

impl RTDPushMessageContentInvoiceBuilder {
  pub fn build(&self) -> PushMessageContentInvoice { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentInvoice { self }
}

impl AsRef<PushMessageContentInvoice> for RTDPushMessageContentInvoiceBuilder {
  fn as_ref(&self) -> &PushMessageContentInvoice { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentLocation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentLocation {}



impl PushMessageContentLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentLocationBuilder {
    let mut inner = PushMessageContentLocation::default();
    inner.td_name = "pushMessageContentLocation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentLocationBuilder { inner }
  }

  pub fn is_live(&self) -> bool { self.is_live }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentLocationBuilder {
  inner: PushMessageContentLocation
}

impl RTDPushMessageContentLocationBuilder {
  pub fn build(&self) -> PushMessageContentLocation { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentLocation { self }
}

impl AsRef<PushMessageContentLocation> for RTDPushMessageContentLocationBuilder {
  fn as_ref(&self) -> &PushMessageContentLocation { &self.inner }
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
  /// True, if the album has at least one video
  has_videos: bool,
  
}

impl RObject for PushMessageContentMediaAlbum {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentMediaAlbum" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentMediaAlbum {}



impl PushMessageContentMediaAlbum {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentMediaAlbumBuilder {
    let mut inner = PushMessageContentMediaAlbum::default();
    inner.td_name = "pushMessageContentMediaAlbum".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentMediaAlbumBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn has_photos(&self) -> bool { self.has_photos }

  pub fn has_videos(&self) -> bool { self.has_videos }

}

#[doc(hidden)]
pub struct RTDPushMessageContentMediaAlbumBuilder {
  inner: PushMessageContentMediaAlbum
}

impl RTDPushMessageContentMediaAlbumBuilder {
  pub fn build(&self) -> PushMessageContentMediaAlbum { self.inner.clone() }

   
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

}

impl AsRef<PushMessageContentMediaAlbum> for PushMessageContentMediaAlbum {
  fn as_ref(&self) -> &PushMessageContentMediaAlbum { self }
}

impl AsRef<PushMessageContentMediaAlbum> for RTDPushMessageContentMediaAlbumBuilder {
  fn as_ref(&self) -> &PushMessageContentMediaAlbum { &self.inner }
}







/// A forwarded messages
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentMessageForwards" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentMessageForwards {}



impl PushMessageContentMessageForwards {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentMessageForwardsBuilder {
    let mut inner = PushMessageContentMessageForwards::default();
    inner.td_name = "pushMessageContentMessageForwards".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentMessageForwardsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

}

#[doc(hidden)]
pub struct RTDPushMessageContentMessageForwardsBuilder {
  inner: PushMessageContentMessageForwards
}

impl RTDPushMessageContentMessageForwardsBuilder {
  pub fn build(&self) -> PushMessageContentMessageForwards { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

}

impl AsRef<PushMessageContentMessageForwards> for PushMessageContentMessageForwards {
  fn as_ref(&self) -> &PushMessageContentMessageForwards { self }
}

impl AsRef<PushMessageContentMessageForwards> for RTDPushMessageContentMessageForwardsBuilder {
  fn as_ref(&self) -> &PushMessageContentMessageForwards { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentPhoto" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentPhoto {}



impl PushMessageContentPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentPhotoBuilder {
    let mut inner = PushMessageContentPhoto::default();
    inner.td_name = "pushMessageContentPhoto".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Option<Photo> { &self.photo }

  pub fn caption(&self) -> &String { &self.caption }

  pub fn is_secret(&self) -> bool { self.is_secret }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentPhotoBuilder {
  inner: PushMessageContentPhoto
}

impl RTDPushMessageContentPhotoBuilder {
  pub fn build(&self) -> PushMessageContentPhoto { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentPhoto { self }
}

impl AsRef<PushMessageContentPhoto> for RTDPushMessageContentPhotoBuilder {
  fn as_ref(&self) -> &PushMessageContentPhoto { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentPoll" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentPoll {}



impl PushMessageContentPoll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentPollBuilder {
    let mut inner = PushMessageContentPoll::default();
    inner.td_name = "pushMessageContentPoll".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentPollBuilder { inner }
  }

  pub fn question(&self) -> &String { &self.question }

  pub fn is_regular(&self) -> bool { self.is_regular }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentPollBuilder {
  inner: PushMessageContentPoll
}

impl RTDPushMessageContentPollBuilder {
  pub fn build(&self) -> PushMessageContentPoll { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentPoll { self }
}

impl AsRef<PushMessageContentPoll> for RTDPushMessageContentPollBuilder {
  fn as_ref(&self) -> &PushMessageContentPoll { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentScreenshotTaken" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentScreenshotTaken {}



impl PushMessageContentScreenshotTaken {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentScreenshotTakenBuilder {
    let mut inner = PushMessageContentScreenshotTaken::default();
    inner.td_name = "pushMessageContentScreenshotTaken".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentScreenshotTakenBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPushMessageContentScreenshotTakenBuilder {
  inner: PushMessageContentScreenshotTaken
}

impl RTDPushMessageContentScreenshotTakenBuilder {
  pub fn build(&self) -> PushMessageContentScreenshotTaken { self.inner.clone() }

}

impl AsRef<PushMessageContentScreenshotTaken> for PushMessageContentScreenshotTaken {
  fn as_ref(&self) -> &PushMessageContentScreenshotTaken { self }
}

impl AsRef<PushMessageContentScreenshotTaken> for RTDPushMessageContentScreenshotTakenBuilder {
  fn as_ref(&self) -> &PushMessageContentScreenshotTaken { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentSticker" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentSticker {}



impl PushMessageContentSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentStickerBuilder {
    let mut inner = PushMessageContentSticker::default();
    inner.td_name = "pushMessageContentSticker".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &Option<Sticker> { &self.sticker }

  pub fn emoji(&self) -> &String { &self.emoji }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentStickerBuilder {
  inner: PushMessageContentSticker
}

impl RTDPushMessageContentStickerBuilder {
  pub fn build(&self) -> PushMessageContentSticker { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentSticker { self }
}

impl AsRef<PushMessageContentSticker> for RTDPushMessageContentStickerBuilder {
  fn as_ref(&self) -> &PushMessageContentSticker { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentText" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentText {}



impl PushMessageContentText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentTextBuilder {
    let mut inner = PushMessageContentText::default();
    inner.td_name = "pushMessageContentText".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentTextBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentTextBuilder {
  inner: PushMessageContentText
}

impl RTDPushMessageContentTextBuilder {
  pub fn build(&self) -> PushMessageContentText { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentText { self }
}

impl AsRef<PushMessageContentText> for RTDPushMessageContentTextBuilder {
  fn as_ref(&self) -> &PushMessageContentText { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentVideo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentVideo {}



impl PushMessageContentVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentVideoBuilder {
    let mut inner = PushMessageContentVideo::default();
    inner.td_name = "pushMessageContentVideo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentVideoBuilder { inner }
  }

  pub fn video(&self) -> &Option<Video> { &self.video }

  pub fn caption(&self) -> &String { &self.caption }

  pub fn is_secret(&self) -> bool { self.is_secret }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentVideoBuilder {
  inner: PushMessageContentVideo
}

impl RTDPushMessageContentVideoBuilder {
  pub fn build(&self) -> PushMessageContentVideo { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentVideo { self }
}

impl AsRef<PushMessageContentVideo> for RTDPushMessageContentVideoBuilder {
  fn as_ref(&self) -> &PushMessageContentVideo { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentVideoNote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentVideoNote {}



impl PushMessageContentVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentVideoNoteBuilder {
    let mut inner = PushMessageContentVideoNote::default();
    inner.td_name = "pushMessageContentVideoNote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentVideoNoteBuilder { inner }
  }

  pub fn video_note(&self) -> &Option<VideoNote> { &self.video_note }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentVideoNoteBuilder {
  inner: PushMessageContentVideoNote
}

impl RTDPushMessageContentVideoNoteBuilder {
  pub fn build(&self) -> PushMessageContentVideoNote { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentVideoNote { self }
}

impl AsRef<PushMessageContentVideoNote> for RTDPushMessageContentVideoNoteBuilder {
  fn as_ref(&self) -> &PushMessageContentVideoNote { &self.inner }
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
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentVoiceNote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPushMessageContent for PushMessageContentVoiceNote {}



impl PushMessageContentVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushMessageContentVoiceNoteBuilder {
    let mut inner = PushMessageContentVoiceNote::default();
    inner.td_name = "pushMessageContentVoiceNote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPushMessageContentVoiceNoteBuilder { inner }
  }

  pub fn voice_note(&self) -> &Option<VoiceNote> { &self.voice_note }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

}

#[doc(hidden)]
pub struct RTDPushMessageContentVoiceNoteBuilder {
  inner: PushMessageContentVoiceNote
}

impl RTDPushMessageContentVoiceNoteBuilder {
  pub fn build(&self) -> PushMessageContentVoiceNote { self.inner.clone() }

   
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
  fn as_ref(&self) -> &PushMessageContentVoiceNote { self }
}

impl AsRef<PushMessageContentVoiceNote> for RTDPushMessageContentVoiceNoteBuilder {
  fn as_ref(&self) -> &PushMessageContentVoiceNote { &self.inner }
}



