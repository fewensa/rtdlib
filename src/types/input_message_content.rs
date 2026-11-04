use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | The content of a message to send
pub trait TDInputMessageContent: Debug + RObject {}

/// The content of a message to send
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
  #[doc(hidden)]
  _Default(()),
  /// An animation message (GIF-style).
  InputMessageAnimation(InputMessageAnimation),
  /// An audio message
  InputMessageAudio(InputMessageAudio),
  /// A message with a checklist. Checklists can't be sent to secret chats, channel chats and channel direct messages chats; for Telegram Premium users only
  InputMessageChecklist(InputMessageChecklist),
  /// A message containing a user contact
  InputMessageContact(InputMessageContact),
  /// A dice message
  InputMessageDice(InputMessageDice),
  /// A document message (general file)
  InputMessageDocument(InputMessageDocument),
  /// A forwarded message
  InputMessageForwarded(InputMessageForwarded),
  /// A message with a game; not supported for channels or secret chats
  InputMessageGame(InputMessageGame),
  /// A message with an invoice; can be used only by bots
  InputMessageInvoice(InputMessageInvoice),
  /// A message with a live location
  InputMessageLiveLocation(InputMessageLiveLocation),
  /// A message with a location
  InputMessageLocation(InputMessageLocation),
  /// A message with paid media; can be used only in channel chats with supergroupFullInfo.has_paid_media_allowed
  InputMessagePaidMedia(InputMessagePaidMedia),
  /// A photo message
  InputMessagePhoto(InputMessagePhoto),
  /// A message with a poll. Polls can't be sent to secret chats and channel direct messages chats. Polls can be sent to a private chat only if the chat is a chat with a bot or the Saved Messages chat
  InputMessagePoll(InputMessagePoll),
  /// A rich message
  InputMessageRichMessage(InputMessageRichMessage),
  /// A stake dice message
  InputMessageStakeDice(InputMessageStakeDice),
  /// A sticker message
  InputMessageSticker(InputMessageSticker),
  /// A message with a forwarded story. Stories can't be forwarded to secret chats. A story can be forwarded only if story.can_be_forwarded
  InputMessageStory(InputMessageStory),
  /// A text message
  InputMessageText(InputMessageText),
  /// A message with information about a venue
  InputMessageVenue(InputMessageVenue),
  /// A video message
  InputMessageVideo(InputMessageVideo),
  /// A video note message
  InputMessageVideoNote(InputMessageVideoNote),
  /// A voice note message
  InputMessageVoiceNote(InputMessageVoiceNote),
}

impl Default for InputMessageContent {
  fn default() -> Self {
    InputMessageContent::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputMessageContent {
  fn deserialize<D>(deserializer: D) -> Result<InputMessageContent, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputMessageContent,
      (inputMessageAnimation, InputMessageAnimation);
      (inputMessageAudio, InputMessageAudio);
      (inputMessageChecklist, InputMessageChecklist);
      (inputMessageContact, InputMessageContact);
      (inputMessageDice, InputMessageDice);
      (inputMessageDocument, InputMessageDocument);
      (inputMessageForwarded, InputMessageForwarded);
      (inputMessageGame, InputMessageGame);
      (inputMessageInvoice, InputMessageInvoice);
      (inputMessageLiveLocation, InputMessageLiveLocation);
      (inputMessageLocation, InputMessageLocation);
      (inputMessagePaidMedia, InputMessagePaidMedia);
      (inputMessagePhoto, InputMessagePhoto);
      (inputMessagePoll, InputMessagePoll);
      (inputMessageRichMessage, InputMessageRichMessage);
      (inputMessageStakeDice, InputMessageStakeDice);
      (inputMessageSticker, InputMessageSticker);
      (inputMessageStory, InputMessageStory);
      (inputMessageText, InputMessageText);
      (inputMessageVenue, InputMessageVenue);
      (inputMessageVideo, InputMessageVideo);
      (inputMessageVideoNote, InputMessageVideoNote);
      (inputMessageVoiceNote, InputMessageVoiceNote);

    )(deserializer)
  }
}

impl RObject for InputMessageContent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputMessageContent::InputMessageAnimation(t) => t.td_name(),
      InputMessageContent::InputMessageAudio(t) => t.td_name(),
      InputMessageContent::InputMessageChecklist(t) => t.td_name(),
      InputMessageContent::InputMessageContact(t) => t.td_name(),
      InputMessageContent::InputMessageDice(t) => t.td_name(),
      InputMessageContent::InputMessageDocument(t) => t.td_name(),
      InputMessageContent::InputMessageForwarded(t) => t.td_name(),
      InputMessageContent::InputMessageGame(t) => t.td_name(),
      InputMessageContent::InputMessageInvoice(t) => t.td_name(),
      InputMessageContent::InputMessageLiveLocation(t) => t.td_name(),
      InputMessageContent::InputMessageLocation(t) => t.td_name(),
      InputMessageContent::InputMessagePaidMedia(t) => t.td_name(),
      InputMessageContent::InputMessagePhoto(t) => t.td_name(),
      InputMessageContent::InputMessagePoll(t) => t.td_name(),
      InputMessageContent::InputMessageRichMessage(t) => t.td_name(),
      InputMessageContent::InputMessageStakeDice(t) => t.td_name(),
      InputMessageContent::InputMessageSticker(t) => t.td_name(),
      InputMessageContent::InputMessageStory(t) => t.td_name(),
      InputMessageContent::InputMessageText(t) => t.td_name(),
      InputMessageContent::InputMessageVenue(t) => t.td_name(),
      InputMessageContent::InputMessageVideo(t) => t.td_name(),
      InputMessageContent::InputMessageVideoNote(t) => t.td_name(),
      InputMessageContent::InputMessageVoiceNote(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputMessageContent::InputMessageAnimation(t) => t.extra(),
      InputMessageContent::InputMessageAudio(t) => t.extra(),
      InputMessageContent::InputMessageChecklist(t) => t.extra(),
      InputMessageContent::InputMessageContact(t) => t.extra(),
      InputMessageContent::InputMessageDice(t) => t.extra(),
      InputMessageContent::InputMessageDocument(t) => t.extra(),
      InputMessageContent::InputMessageForwarded(t) => t.extra(),
      InputMessageContent::InputMessageGame(t) => t.extra(),
      InputMessageContent::InputMessageInvoice(t) => t.extra(),
      InputMessageContent::InputMessageLiveLocation(t) => t.extra(),
      InputMessageContent::InputMessageLocation(t) => t.extra(),
      InputMessageContent::InputMessagePaidMedia(t) => t.extra(),
      InputMessageContent::InputMessagePhoto(t) => t.extra(),
      InputMessageContent::InputMessagePoll(t) => t.extra(),
      InputMessageContent::InputMessageRichMessage(t) => t.extra(),
      InputMessageContent::InputMessageStakeDice(t) => t.extra(),
      InputMessageContent::InputMessageSticker(t) => t.extra(),
      InputMessageContent::InputMessageStory(t) => t.extra(),
      InputMessageContent::InputMessageText(t) => t.extra(),
      InputMessageContent::InputMessageVenue(t) => t.extra(),
      InputMessageContent::InputMessageVideo(t) => t.extra(),
      InputMessageContent::InputMessageVideoNote(t) => t.extra(),
      InputMessageContent::InputMessageVoiceNote(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputMessageContent::_Default(_))
  }

  pub fn is_input_message_animation(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageAnimation(_))
  }
  pub fn is_input_message_audio(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageAudio(_))
  }
  pub fn is_input_message_checklist(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageChecklist(_))
  }
  pub fn is_input_message_contact(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageContact(_))
  }
  pub fn is_input_message_dice(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageDice(_))
  }
  pub fn is_input_message_document(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageDocument(_))
  }
  pub fn is_input_message_forwarded(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageForwarded(_))
  }
  pub fn is_input_message_game(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageGame(_))
  }
  pub fn is_input_message_invoice(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageInvoice(_))
  }
  pub fn is_input_message_live_location(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageLiveLocation(_))
  }
  pub fn is_input_message_location(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageLocation(_))
  }
  pub fn is_input_message_paid_media(&self) -> bool {
    matches!(self, InputMessageContent::InputMessagePaidMedia(_))
  }
  pub fn is_input_message_photo(&self) -> bool {
    matches!(self, InputMessageContent::InputMessagePhoto(_))
  }
  pub fn is_input_message_poll(&self) -> bool {
    matches!(self, InputMessageContent::InputMessagePoll(_))
  }
  pub fn is_input_message_rich_message(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageRichMessage(_))
  }
  pub fn is_input_message_stake_dice(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageStakeDice(_))
  }
  pub fn is_input_message_sticker(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageSticker(_))
  }
  pub fn is_input_message_story(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageStory(_))
  }
  pub fn is_input_message_text(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageText(_))
  }
  pub fn is_input_message_venue(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageVenue(_))
  }
  pub fn is_input_message_video(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageVideo(_))
  }
  pub fn is_input_message_video_note(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageVideoNote(_))
  }
  pub fn is_input_message_voice_note(&self) -> bool {
    matches!(self, InputMessageContent::InputMessageVoiceNote(_))
  }

  pub fn on_input_message_animation<F: FnOnce(&InputMessageAnimation)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageAnimation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_audio<F: FnOnce(&InputMessageAudio)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageAudio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_checklist<F: FnOnce(&InputMessageChecklist)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageChecklist(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_contact<F: FnOnce(&InputMessageContact)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageContact(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_dice<F: FnOnce(&InputMessageDice)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageDice(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_document<F: FnOnce(&InputMessageDocument)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageDocument(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_forwarded<F: FnOnce(&InputMessageForwarded)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageForwarded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_game<F: FnOnce(&InputMessageGame)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageGame(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_invoice<F: FnOnce(&InputMessageInvoice)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageInvoice(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_live_location<F: FnOnce(&InputMessageLiveLocation)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InputMessageContent::InputMessageLiveLocation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_location<F: FnOnce(&InputMessageLocation)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageLocation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_paid_media<F: FnOnce(&InputMessagePaidMedia)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessagePaidMedia(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_photo<F: FnOnce(&InputMessagePhoto)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessagePhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_poll<F: FnOnce(&InputMessagePoll)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessagePoll(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_rich_message<F: FnOnce(&InputMessageRichMessage)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InputMessageContent::InputMessageRichMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_stake_dice<F: FnOnce(&InputMessageStakeDice)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageStakeDice(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_sticker<F: FnOnce(&InputMessageSticker)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageSticker(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_story<F: FnOnce(&InputMessageStory)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageStory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_text<F: FnOnce(&InputMessageText)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageText(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_venue<F: FnOnce(&InputMessageVenue)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageVenue(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_video<F: FnOnce(&InputMessageVideo)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageVideo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_video_note<F: FnOnce(&InputMessageVideoNote)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageVideoNote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_input_message_voice_note<F: FnOnce(&InputMessageVoiceNote)>(&self, fnc: F) -> &Self {
    if let InputMessageContent::InputMessageVoiceNote(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_input_message_animation(&self) -> Option<&InputMessageAnimation> {
    if let InputMessageContent::InputMessageAnimation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_audio(&self) -> Option<&InputMessageAudio> {
    if let InputMessageContent::InputMessageAudio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_checklist(&self) -> Option<&InputMessageChecklist> {
    if let InputMessageContent::InputMessageChecklist(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_contact(&self) -> Option<&InputMessageContact> {
    if let InputMessageContent::InputMessageContact(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_dice(&self) -> Option<&InputMessageDice> {
    if let InputMessageContent::InputMessageDice(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_document(&self) -> Option<&InputMessageDocument> {
    if let InputMessageContent::InputMessageDocument(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_forwarded(&self) -> Option<&InputMessageForwarded> {
    if let InputMessageContent::InputMessageForwarded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_game(&self) -> Option<&InputMessageGame> {
    if let InputMessageContent::InputMessageGame(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_invoice(&self) -> Option<&InputMessageInvoice> {
    if let InputMessageContent::InputMessageInvoice(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_live_location(&self) -> Option<&InputMessageLiveLocation> {
    if let InputMessageContent::InputMessageLiveLocation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_location(&self) -> Option<&InputMessageLocation> {
    if let InputMessageContent::InputMessageLocation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_paid_media(&self) -> Option<&InputMessagePaidMedia> {
    if let InputMessageContent::InputMessagePaidMedia(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_photo(&self) -> Option<&InputMessagePhoto> {
    if let InputMessageContent::InputMessagePhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_poll(&self) -> Option<&InputMessagePoll> {
    if let InputMessageContent::InputMessagePoll(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_rich_message(&self) -> Option<&InputMessageRichMessage> {
    if let InputMessageContent::InputMessageRichMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_stake_dice(&self) -> Option<&InputMessageStakeDice> {
    if let InputMessageContent::InputMessageStakeDice(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_sticker(&self) -> Option<&InputMessageSticker> {
    if let InputMessageContent::InputMessageSticker(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_story(&self) -> Option<&InputMessageStory> {
    if let InputMessageContent::InputMessageStory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_text(&self) -> Option<&InputMessageText> {
    if let InputMessageContent::InputMessageText(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_venue(&self) -> Option<&InputMessageVenue> {
    if let InputMessageContent::InputMessageVenue(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_video(&self) -> Option<&InputMessageVideo> {
    if let InputMessageContent::InputMessageVideo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_video_note(&self) -> Option<&InputMessageVideoNote> {
    if let InputMessageContent::InputMessageVideoNote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_input_message_voice_note(&self) -> Option<&InputMessageVoiceNote> {
    if let InputMessageContent::InputMessageVoiceNote(t) = self {
      return Some(t);
    }
    None
  }

  pub fn input_message_animation<T: AsRef<InputMessageAnimation>>(t: T) -> Self {
    InputMessageContent::InputMessageAnimation(t.as_ref().clone())
  }

  pub fn input_message_audio<T: AsRef<InputMessageAudio>>(t: T) -> Self {
    InputMessageContent::InputMessageAudio(t.as_ref().clone())
  }

  pub fn input_message_checklist<T: AsRef<InputMessageChecklist>>(t: T) -> Self {
    InputMessageContent::InputMessageChecklist(t.as_ref().clone())
  }

  pub fn input_message_contact<T: AsRef<InputMessageContact>>(t: T) -> Self {
    InputMessageContent::InputMessageContact(t.as_ref().clone())
  }

  pub fn input_message_dice<T: AsRef<InputMessageDice>>(t: T) -> Self {
    InputMessageContent::InputMessageDice(t.as_ref().clone())
  }

  pub fn input_message_document<T: AsRef<InputMessageDocument>>(t: T) -> Self {
    InputMessageContent::InputMessageDocument(t.as_ref().clone())
  }

  pub fn input_message_forwarded<T: AsRef<InputMessageForwarded>>(t: T) -> Self {
    InputMessageContent::InputMessageForwarded(t.as_ref().clone())
  }

  pub fn input_message_game<T: AsRef<InputMessageGame>>(t: T) -> Self {
    InputMessageContent::InputMessageGame(t.as_ref().clone())
  }

  pub fn input_message_invoice<T: AsRef<InputMessageInvoice>>(t: T) -> Self {
    InputMessageContent::InputMessageInvoice(t.as_ref().clone())
  }

  pub fn input_message_live_location<T: AsRef<InputMessageLiveLocation>>(t: T) -> Self {
    InputMessageContent::InputMessageLiveLocation(t.as_ref().clone())
  }

  pub fn input_message_location<T: AsRef<InputMessageLocation>>(t: T) -> Self {
    InputMessageContent::InputMessageLocation(t.as_ref().clone())
  }

  pub fn input_message_paid_media<T: AsRef<InputMessagePaidMedia>>(t: T) -> Self {
    InputMessageContent::InputMessagePaidMedia(t.as_ref().clone())
  }

  pub fn input_message_photo<T: AsRef<InputMessagePhoto>>(t: T) -> Self {
    InputMessageContent::InputMessagePhoto(t.as_ref().clone())
  }

  pub fn input_message_poll<T: AsRef<InputMessagePoll>>(t: T) -> Self {
    InputMessageContent::InputMessagePoll(t.as_ref().clone())
  }

  pub fn input_message_rich_message<T: AsRef<InputMessageRichMessage>>(t: T) -> Self {
    InputMessageContent::InputMessageRichMessage(t.as_ref().clone())
  }

  pub fn input_message_stake_dice<T: AsRef<InputMessageStakeDice>>(t: T) -> Self {
    InputMessageContent::InputMessageStakeDice(t.as_ref().clone())
  }

  pub fn input_message_sticker<T: AsRef<InputMessageSticker>>(t: T) -> Self {
    InputMessageContent::InputMessageSticker(t.as_ref().clone())
  }

  pub fn input_message_story<T: AsRef<InputMessageStory>>(t: T) -> Self {
    InputMessageContent::InputMessageStory(t.as_ref().clone())
  }

  pub fn input_message_text<T: AsRef<InputMessageText>>(t: T) -> Self {
    InputMessageContent::InputMessageText(t.as_ref().clone())
  }

  pub fn input_message_venue<T: AsRef<InputMessageVenue>>(t: T) -> Self {
    InputMessageContent::InputMessageVenue(t.as_ref().clone())
  }

  pub fn input_message_video<T: AsRef<InputMessageVideo>>(t: T) -> Self {
    InputMessageContent::InputMessageVideo(t.as_ref().clone())
  }

  pub fn input_message_video_note<T: AsRef<InputMessageVideoNote>>(t: T) -> Self {
    InputMessageContent::InputMessageVideoNote(t.as_ref().clone())
  }

  pub fn input_message_voice_note<T: AsRef<InputMessageVoiceNote>>(t: T) -> Self {
    InputMessageContent::InputMessageVoiceNote(t.as_ref().clone())
  }
}

impl AsRef<InputMessageContent> for InputMessageContent {
  fn as_ref(&self) -> &InputMessageContent {
    self
  }
}

/// An animation message (GIF-style).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The animation to be sent
  animation: InputAnimation,
  /// Animation caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
  caption: FormattedText,
  /// True, if the caption must be shown above the animation; otherwise, the caption must be shown below the animation; not supported in secret chats
  show_caption_above_media: bool,
  /// True, if the animation preview must be covered by a spoiler animation; not supported in secret chats
  has_spoiler: bool,
}

impl RObject for InputMessageAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageAnimation {}

impl InputMessageAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageAnimationBuilder {
    let inner = InputMessageAnimation {
      td_name: "inputMessageAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &InputAnimation {
    &self.animation
  }

  pub fn caption(&self) -> &FormattedText {
    &self.caption
  }

  pub fn show_caption_above_media(&self) -> bool {
    self.show_caption_above_media
  }

  pub fn has_spoiler(&self) -> bool {
    self.has_spoiler
  }
}

#[doc(hidden)]
pub struct RTDInputMessageAnimationBuilder {
  inner: InputMessageAnimation,
}

impl RTDInputMessageAnimationBuilder {
  pub fn build(&self) -> InputMessageAnimation {
    self.inner.clone()
  }

  pub fn animation<T: AsRef<InputAnimation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn show_caption_above_media(&mut self, show_caption_above_media: bool) -> &mut Self {
    self.inner.show_caption_above_media = show_caption_above_media;
    self
  }

  pub fn has_spoiler(&mut self, has_spoiler: bool) -> &mut Self {
    self.inner.has_spoiler = has_spoiler;
    self
  }
}

impl AsRef<InputMessageAnimation> for InputMessageAnimation {
  fn as_ref(&self) -> &InputMessageAnimation {
    self
  }
}

impl AsRef<InputMessageAnimation> for RTDInputMessageAnimationBuilder {
  fn as_ref(&self) -> &InputMessageAnimation {
    &self.inner
  }
}

/// An audio message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Audio to be sent
  audio: InputAudio,
  /// Audio caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
  caption: FormattedText,
}

impl RObject for InputMessageAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageAudio {}

impl InputMessageAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageAudioBuilder {
    let inner = InputMessageAudio {
      td_name: "inputMessageAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageAudioBuilder { inner }
  }

  pub fn audio(&self) -> &InputAudio {
    &self.audio
  }

  pub fn caption(&self) -> &FormattedText {
    &self.caption
  }
}

#[doc(hidden)]
pub struct RTDInputMessageAudioBuilder {
  inner: InputMessageAudio,
}

impl RTDInputMessageAudioBuilder {
  pub fn build(&self) -> InputMessageAudio {
    self.inner.clone()
  }

  pub fn audio<T: AsRef<InputAudio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }
}

impl AsRef<InputMessageAudio> for InputMessageAudio {
  fn as_ref(&self) -> &InputMessageAudio {
    self
  }
}

impl AsRef<InputMessageAudio> for RTDInputMessageAudioBuilder {
  fn as_ref(&self) -> &InputMessageAudio {
    &self.inner
  }
}

/// A message with a checklist. Checklists can't be sent to secret chats, channel chats and channel direct messages chats; for Telegram Premium users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageChecklist {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The checklist to send
  checklist: InputChecklist,
}

impl RObject for InputMessageChecklist {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageChecklist"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageChecklist {}

impl InputMessageChecklist {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageChecklistBuilder {
    let inner = InputMessageChecklist {
      td_name: "inputMessageChecklist".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageChecklistBuilder { inner }
  }

  pub fn checklist(&self) -> &InputChecklist {
    &self.checklist
  }
}

#[doc(hidden)]
pub struct RTDInputMessageChecklistBuilder {
  inner: InputMessageChecklist,
}

impl RTDInputMessageChecklistBuilder {
  pub fn build(&self) -> InputMessageChecklist {
    self.inner.clone()
  }

  pub fn checklist<T: AsRef<InputChecklist>>(&mut self, checklist: T) -> &mut Self {
    self.inner.checklist = checklist.as_ref().clone();
    self
  }
}

impl AsRef<InputMessageChecklist> for InputMessageChecklist {
  fn as_ref(&self) -> &InputMessageChecklist {
    self
  }
}

impl AsRef<InputMessageChecklist> for RTDInputMessageChecklistBuilder {
  fn as_ref(&self) -> &InputMessageChecklist {
    &self.inner
  }
}

/// A message containing a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Contact to send
  contact: Contact,
}

impl RObject for InputMessageContact {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageContact"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageContact {}

impl InputMessageContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageContactBuilder {
    let inner = InputMessageContact {
      td_name: "inputMessageContact".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageContactBuilder { inner }
  }

  pub fn contact(&self) -> &Contact {
    &self.contact
  }
}

#[doc(hidden)]
pub struct RTDInputMessageContactBuilder {
  inner: InputMessageContact,
}

impl RTDInputMessageContactBuilder {
  pub fn build(&self) -> InputMessageContact {
    self.inner.clone()
  }

  pub fn contact<T: AsRef<Contact>>(&mut self, contact: T) -> &mut Self {
    self.inner.contact = contact.as_ref().clone();
    self
  }
}

impl AsRef<InputMessageContact> for InputMessageContact {
  fn as_ref(&self) -> &InputMessageContact {
    self
  }
}

impl AsRef<InputMessageContact> for RTDInputMessageContactBuilder {
  fn as_ref(&self) -> &InputMessageContact {
    &self.inner
  }
}

/// A dice message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageDice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Emoji on which the dice throw animation is based
  emoji: String,
  /// Pass true to delete message draft in the chat
  clear_draft: bool,
}

impl RObject for InputMessageDice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageDice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageDice {}

impl InputMessageDice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageDiceBuilder {
    let inner = InputMessageDice {
      td_name: "inputMessageDice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageDiceBuilder { inner }
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }

  pub fn clear_draft(&self) -> bool {
    self.clear_draft
  }
}

#[doc(hidden)]
pub struct RTDInputMessageDiceBuilder {
  inner: InputMessageDice,
}

impl RTDInputMessageDiceBuilder {
  pub fn build(&self) -> InputMessageDice {
    self.inner.clone()
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

  pub fn clear_draft(&mut self, clear_draft: bool) -> &mut Self {
    self.inner.clear_draft = clear_draft;
    self
  }
}

impl AsRef<InputMessageDice> for InputMessageDice {
  fn as_ref(&self) -> &InputMessageDice {
    self
  }
}

impl AsRef<InputMessageDice> for RTDInputMessageDiceBuilder {
  fn as_ref(&self) -> &InputMessageDice {
    &self.inner
  }
}

/// A document message (general file)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Document to be sent
  document: InputDocument,
  /// Document caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
  caption: FormattedText,
}

impl RObject for InputMessageDocument {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageDocument"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageDocument {}

impl InputMessageDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageDocumentBuilder {
    let inner = InputMessageDocument {
      td_name: "inputMessageDocument".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageDocumentBuilder { inner }
  }

  pub fn document(&self) -> &InputDocument {
    &self.document
  }

  pub fn caption(&self) -> &FormattedText {
    &self.caption
  }
}

#[doc(hidden)]
pub struct RTDInputMessageDocumentBuilder {
  inner: InputMessageDocument,
}

impl RTDInputMessageDocumentBuilder {
  pub fn build(&self) -> InputMessageDocument {
    self.inner.clone()
  }

  pub fn document<T: AsRef<InputDocument>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }
}

impl AsRef<InputMessageDocument> for InputMessageDocument {
  fn as_ref(&self) -> &InputMessageDocument {
    self
  }
}

impl AsRef<InputMessageDocument> for RTDInputMessageDocumentBuilder {
  fn as_ref(&self) -> &InputMessageDocument {
    &self.inner
  }
}

/// A forwarded message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageForwarded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier for the chat this forwarded message came from
  from_chat_id: i64,
  /// Identifier of the message to forward. A message can be forwarded only if messageProperties.can_be_forwarded
  message_id: i64,
  /// Pass true if a game message is being shared from a launched game; applies only to game messages
  in_game_share: bool,
  /// Pass true to replace video start timestamp in the forwarded message
  replace_video_start_timestamp: bool,
  /// The new video start timestamp; ignored if replace_video_start_timestamp == false
  new_video_start_timestamp: i64,
  /// Options to be used to copy content of the message without reference to the original sender; pass null to forward the message as usual
  copy_options: MessageCopyOptions,
}

impl RObject for InputMessageForwarded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageForwarded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageForwarded {}

impl InputMessageForwarded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageForwardedBuilder {
    let inner = InputMessageForwarded {
      td_name: "inputMessageForwarded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageForwardedBuilder { inner }
  }

  pub fn from_chat_id(&self) -> i64 {
    self.from_chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn in_game_share(&self) -> bool {
    self.in_game_share
  }

  pub fn replace_video_start_timestamp(&self) -> bool {
    self.replace_video_start_timestamp
  }

  pub fn new_video_start_timestamp(&self) -> i64 {
    self.new_video_start_timestamp
  }

  pub fn copy_options(&self) -> &MessageCopyOptions {
    &self.copy_options
  }
}

#[doc(hidden)]
pub struct RTDInputMessageForwardedBuilder {
  inner: InputMessageForwarded,
}

impl RTDInputMessageForwardedBuilder {
  pub fn build(&self) -> InputMessageForwarded {
    self.inner.clone()
  }

  pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self {
    self.inner.from_chat_id = from_chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn in_game_share(&mut self, in_game_share: bool) -> &mut Self {
    self.inner.in_game_share = in_game_share;
    self
  }

  pub fn replace_video_start_timestamp(
    &mut self,
    replace_video_start_timestamp: bool,
  ) -> &mut Self {
    self.inner.replace_video_start_timestamp = replace_video_start_timestamp;
    self
  }

  pub fn new_video_start_timestamp(&mut self, new_video_start_timestamp: i64) -> &mut Self {
    self.inner.new_video_start_timestamp = new_video_start_timestamp;
    self
  }

  pub fn copy_options<T: AsRef<MessageCopyOptions>>(&mut self, copy_options: T) -> &mut Self {
    self.inner.copy_options = copy_options.as_ref().clone();
    self
  }
}

impl AsRef<InputMessageForwarded> for InputMessageForwarded {
  fn as_ref(&self) -> &InputMessageForwarded {
    self
  }
}

impl AsRef<InputMessageForwarded> for RTDInputMessageForwardedBuilder {
  fn as_ref(&self) -> &InputMessageForwarded {
    &self.inner
  }
}

/// A message with a game; not supported for channels or secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the bot that owns the game
  bot_user_id: i64,
  /// Short name of the game
  game_short_name: String,
}

impl RObject for InputMessageGame {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageGame"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageGame {}

impl InputMessageGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageGameBuilder {
    let inner = InputMessageGame {
      td_name: "inputMessageGame".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageGameBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn game_short_name(&self) -> &String {
    &self.game_short_name
  }
}

#[doc(hidden)]
pub struct RTDInputMessageGameBuilder {
  inner: InputMessageGame,
}

impl RTDInputMessageGameBuilder {
  pub fn build(&self) -> InputMessageGame {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn game_short_name<T: AsRef<str>>(&mut self, game_short_name: T) -> &mut Self {
    self.inner.game_short_name = game_short_name.as_ref().to_string();
    self
  }
}

impl AsRef<InputMessageGame> for InputMessageGame {
  fn as_ref(&self) -> &InputMessageGame {
    self
  }
}

impl AsRef<InputMessageGame> for RTDInputMessageGameBuilder {
  fn as_ref(&self) -> &InputMessageGame {
    &self.inner
  }
}

/// A message with an invoice; can be used only by bots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageInvoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Invoice
  invoice: Invoice,
  /// Product title; 1-32 characters
  title: String,
  /// A message with an invoice; can be used only by bots
  description: String,
  /// Product photo URL; optional
  photo_url: String,
  /// Product photo size
  photo_size: i64,
  /// Product photo width
  photo_width: i64,
  /// Product photo height
  photo_height: i64,
  /// The invoice payload
  payload: String,
  /// Payment provider token; may be empty for payments in Telegram Stars
  provider_token: String,
  /// JSON-encoded data about the invoice, which will be shared with the payment provider
  provider_data: String,
  /// Unique invoice bot deep link parameter for the generation of this invoice. If empty, it would be possible to pay directly from forwards of the invoice message
  start_parameter: String,
  /// The content of paid media attached to the invoice; pass null if none
  paid_media: InputPaidMedia,
  /// Paid media caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
  paid_media_caption: FormattedText,
}

impl RObject for InputMessageInvoice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageInvoice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageInvoice {}

impl InputMessageInvoice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageInvoiceBuilder {
    let inner = InputMessageInvoice {
      td_name: "inputMessageInvoice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageInvoiceBuilder { inner }
  }

  pub fn invoice(&self) -> &Invoice {
    &self.invoice
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn description(&self) -> &String {
    &self.description
  }

  pub fn photo_url(&self) -> &String {
    &self.photo_url
  }

  pub fn photo_size(&self) -> i64 {
    self.photo_size
  }

  pub fn photo_width(&self) -> i64 {
    self.photo_width
  }

  pub fn photo_height(&self) -> i64 {
    self.photo_height
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }

  pub fn provider_token(&self) -> &String {
    &self.provider_token
  }

  pub fn provider_data(&self) -> &String {
    &self.provider_data
  }

  pub fn start_parameter(&self) -> &String {
    &self.start_parameter
  }

  pub fn paid_media(&self) -> &InputPaidMedia {
    &self.paid_media
  }

  pub fn paid_media_caption(&self) -> &FormattedText {
    &self.paid_media_caption
  }
}

#[doc(hidden)]
pub struct RTDInputMessageInvoiceBuilder {
  inner: InputMessageInvoice,
}

impl RTDInputMessageInvoiceBuilder {
  pub fn build(&self) -> InputMessageInvoice {
    self.inner.clone()
  }

  pub fn invoice<T: AsRef<Invoice>>(&mut self, invoice: T) -> &mut Self {
    self.inner.invoice = invoice.as_ref().clone();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

  pub fn photo_url<T: AsRef<str>>(&mut self, photo_url: T) -> &mut Self {
    self.inner.photo_url = photo_url.as_ref().to_string();
    self
  }

  pub fn photo_size(&mut self, photo_size: i64) -> &mut Self {
    self.inner.photo_size = photo_size;
    self
  }

  pub fn photo_width(&mut self, photo_width: i64) -> &mut Self {
    self.inner.photo_width = photo_width;
    self
  }

  pub fn photo_height(&mut self, photo_height: i64) -> &mut Self {
    self.inner.photo_height = photo_height;
    self
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }

  pub fn provider_token<T: AsRef<str>>(&mut self, provider_token: T) -> &mut Self {
    self.inner.provider_token = provider_token.as_ref().to_string();
    self
  }

  pub fn provider_data<T: AsRef<str>>(&mut self, provider_data: T) -> &mut Self {
    self.inner.provider_data = provider_data.as_ref().to_string();
    self
  }

  pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
    self.inner.start_parameter = start_parameter.as_ref().to_string();
    self
  }

  pub fn paid_media<T: AsRef<InputPaidMedia>>(&mut self, paid_media: T) -> &mut Self {
    self.inner.paid_media = paid_media.as_ref().clone();
    self
  }

  pub fn paid_media_caption<T: AsRef<FormattedText>>(
    &mut self,
    paid_media_caption: T,
  ) -> &mut Self {
    self.inner.paid_media_caption = paid_media_caption.as_ref().clone();
    self
  }
}

impl AsRef<InputMessageInvoice> for InputMessageInvoice {
  fn as_ref(&self) -> &InputMessageInvoice {
    self
  }
}

impl AsRef<InputMessageInvoice> for RTDInputMessageInvoiceBuilder {
  fn as_ref(&self) -> &InputMessageInvoice {
    &self.inner
  }
}

/// A message with a live location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageLiveLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Initial state of the live location to be sent. Live period must be equal to 0x7FFFFFFF for permanent live locations, or between 60 and 86400
  location: LiveLocation,
}

impl RObject for InputMessageLiveLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageLiveLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageLiveLocation {}

impl InputMessageLiveLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageLiveLocationBuilder {
    let inner = InputMessageLiveLocation {
      td_name: "inputMessageLiveLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageLiveLocationBuilder { inner }
  }

  pub fn location(&self) -> &LiveLocation {
    &self.location
  }
}

#[doc(hidden)]
pub struct RTDInputMessageLiveLocationBuilder {
  inner: InputMessageLiveLocation,
}

impl RTDInputMessageLiveLocationBuilder {
  pub fn build(&self) -> InputMessageLiveLocation {
    self.inner.clone()
  }

  pub fn location<T: AsRef<LiveLocation>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }
}

impl AsRef<InputMessageLiveLocation> for InputMessageLiveLocation {
  fn as_ref(&self) -> &InputMessageLiveLocation {
    self
  }
}

impl AsRef<InputMessageLiveLocation> for RTDInputMessageLiveLocationBuilder {
  fn as_ref(&self) -> &InputMessageLiveLocation {
    &self.inner
  }
}

/// A message with a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Location to be sent
  location: Location,
}

impl RObject for InputMessageLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageLocation {}

impl InputMessageLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageLocationBuilder {
    let inner = InputMessageLocation {
      td_name: "inputMessageLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageLocationBuilder { inner }
  }

  pub fn location(&self) -> &Location {
    &self.location
  }
}

#[doc(hidden)]
pub struct RTDInputMessageLocationBuilder {
  inner: InputMessageLocation,
}

impl RTDInputMessageLocationBuilder {
  pub fn build(&self) -> InputMessageLocation {
    self.inner.clone()
  }

  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }
}

impl AsRef<InputMessageLocation> for InputMessageLocation {
  fn as_ref(&self) -> &InputMessageLocation {
    self
  }
}

impl AsRef<InputMessageLocation> for RTDInputMessageLocationBuilder {
  fn as_ref(&self) -> &InputMessageLocation {
    &self.inner
  }
}

/// A message with paid media; can be used only in channel chats with supergroupFullInfo.has_paid_media_allowed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessagePaidMedia {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of Telegram Stars that must be paid to see the media; 1-getOption("paid_media_message_star_count_max")
  star_count: i64,
  /// The content of the paid media
  paid_media: Vec<InputPaidMedia>,
  /// Message caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
  caption: FormattedText,
  /// True, if the caption must be shown above the media; otherwise, the caption must be shown below the media; not supported in secret chats
  show_caption_above_media: bool,
  /// Bot-provided data for the paid media; bots only
  payload: String,
}

impl RObject for InputMessagePaidMedia {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessagePaidMedia"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessagePaidMedia {}

impl InputMessagePaidMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessagePaidMediaBuilder {
    let inner = InputMessagePaidMedia {
      td_name: "inputMessagePaidMedia".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessagePaidMediaBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn paid_media(&self) -> &Vec<InputPaidMedia> {
    &self.paid_media
  }

  pub fn caption(&self) -> &FormattedText {
    &self.caption
  }

  pub fn show_caption_above_media(&self) -> bool {
    self.show_caption_above_media
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }
}

#[doc(hidden)]
pub struct RTDInputMessagePaidMediaBuilder {
  inner: InputMessagePaidMedia,
}

impl RTDInputMessagePaidMediaBuilder {
  pub fn build(&self) -> InputMessagePaidMedia {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn paid_media(&mut self, paid_media: Vec<InputPaidMedia>) -> &mut Self {
    self.inner.paid_media = paid_media;
    self
  }

  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn show_caption_above_media(&mut self, show_caption_above_media: bool) -> &mut Self {
    self.inner.show_caption_above_media = show_caption_above_media;
    self
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }
}

impl AsRef<InputMessagePaidMedia> for InputMessagePaidMedia {
  fn as_ref(&self) -> &InputMessagePaidMedia {
    self
  }
}

impl AsRef<InputMessagePaidMedia> for RTDInputMessagePaidMediaBuilder {
  fn as_ref(&self) -> &InputMessagePaidMedia {
    &self.inner
  }
}

/// A photo message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessagePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo to be sent
  photo: InputPhoto,
  /// Photo caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
  caption: FormattedText,
  /// True, if the caption must be shown above the photo; otherwise, the caption must be shown below the photo; not supported in secret chats
  show_caption_above_media: bool,
  /// Photo self-destruct type; pass null if none; private chats only
  self_destruct_type: MessageSelfDestructType,
  /// True, if the photo preview must be covered by a spoiler animation; not supported in secret chats
  has_spoiler: bool,
}

impl RObject for InputMessagePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessagePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessagePhoto {}

impl InputMessagePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessagePhotoBuilder {
    let inner = InputMessagePhoto {
      td_name: "inputMessagePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessagePhotoBuilder { inner }
  }

  pub fn photo(&self) -> &InputPhoto {
    &self.photo
  }

  pub fn caption(&self) -> &FormattedText {
    &self.caption
  }

  pub fn show_caption_above_media(&self) -> bool {
    self.show_caption_above_media
  }

  pub fn self_destruct_type(&self) -> &MessageSelfDestructType {
    &self.self_destruct_type
  }

  pub fn has_spoiler(&self) -> bool {
    self.has_spoiler
  }
}

#[doc(hidden)]
pub struct RTDInputMessagePhotoBuilder {
  inner: InputMessagePhoto,
}

impl RTDInputMessagePhotoBuilder {
  pub fn build(&self) -> InputMessagePhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<InputPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn show_caption_above_media(&mut self, show_caption_above_media: bool) -> &mut Self {
    self.inner.show_caption_above_media = show_caption_above_media;
    self
  }

  pub fn self_destruct_type<T: AsRef<MessageSelfDestructType>>(
    &mut self,
    self_destruct_type: T,
  ) -> &mut Self {
    self.inner.self_destruct_type = self_destruct_type.as_ref().clone();
    self
  }

  pub fn has_spoiler(&mut self, has_spoiler: bool) -> &mut Self {
    self.inner.has_spoiler = has_spoiler;
    self
  }
}

impl AsRef<InputMessagePhoto> for InputMessagePhoto {
  fn as_ref(&self) -> &InputMessagePhoto {
    self
  }
}

impl AsRef<InputMessagePhoto> for RTDInputMessagePhotoBuilder {
  fn as_ref(&self) -> &InputMessagePhoto {
    &self.inner
  }
}

/// A message with a poll. Polls can't be sent to secret chats and channel direct messages chats. Polls can be sent to a private chat only if the chat is a chat with a bot or the Saved Messages chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessagePoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Poll question; 1-255 characters (up to 300 characters for bots). Only custom emoji entities are allowed to be added and only by Premium users
  question: FormattedText,
  /// List of poll answer options; 1-getOption("poll_answer_count_max") options
  options: Vec<InputPollOption>,
  /// A message with a poll. Polls can't be sent to secret chats and channel direct messages chats. Polls can be sent to a private chat only if the chat is a chat with a bot or the Saved Messages chat
  description: FormattedText,
  /// Media attached to the poll; pass null if none. Must be one of the following types: inputPollMediaAnimation, inputPollMediaAudio, inputPollMediaDocument, inputPollMediaLocation, inputPollMediaPhoto, inputPollMediaVenue, or inputPollMediaVideo without caption
  media: InputPollMedia,
  /// True, if the poll voters are anonymous. Non-anonymous polls can't be sent or forwarded to channels
  is_anonymous: bool,
  /// True, if multiple answer options can be chosen simultaneously
  allows_multiple_answers: bool,
  /// True, if the poll can be answered multiple times
  allows_revoting: bool,
  /// True, if only the users that are members of the chat for more than a day will be able to vote; for channel chats only
  members_only: bool,
  /// The list of two-letter ISO 3166-1 alpha-2 codes of countries, users from which will be able to vote; for channel chats only. If empty, then all users can participate in the poll. There can be up to getOption("poll_country_count_max") chosen countries
  country_codes: Vec<String>,
  /// True, if poll options must be shown in a fixed random order
  shuffle_options: bool,
  /// True, if the poll results will appear only after the poll closes
  hide_results_until_closes: bool,
  /// Type of the poll
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: InputPollType,
  /// Amount of time the poll will be active after creation, in seconds; 0-getOption("poll_open_period_max"); pass 0 if not specified
  open_period: i64,
  /// Point in time (Unix timestamp) when the poll will automatically be closed; must be 0-getOption("poll_open_period_max") seconds in the future; pass 0 if not specified
  close_date: i64,
  /// True, if the poll needs to be sent already closed; for bots only
  is_closed: bool,
}

impl RObject for InputMessagePoll {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessagePoll"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessagePoll {}

impl InputMessagePoll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessagePollBuilder {
    let inner = InputMessagePoll {
      td_name: "inputMessagePoll".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessagePollBuilder { inner }
  }

  pub fn question(&self) -> &FormattedText {
    &self.question
  }

  pub fn options(&self) -> &Vec<InputPollOption> {
    &self.options
  }

  pub fn description(&self) -> &FormattedText {
    &self.description
  }

  pub fn media(&self) -> &InputPollMedia {
    &self.media
  }

  pub fn is_anonymous(&self) -> bool {
    self.is_anonymous
  }

  pub fn allows_multiple_answers(&self) -> bool {
    self.allows_multiple_answers
  }

  pub fn allows_revoting(&self) -> bool {
    self.allows_revoting
  }

  pub fn members_only(&self) -> bool {
    self.members_only
  }

  pub fn country_codes(&self) -> &Vec<String> {
    &self.country_codes
  }

  pub fn shuffle_options(&self) -> bool {
    self.shuffle_options
  }

  pub fn hide_results_until_closes(&self) -> bool {
    self.hide_results_until_closes
  }

  pub fn type_(&self) -> &InputPollType {
    &self.type_
  }

  pub fn open_period(&self) -> i64 {
    self.open_period
  }

  pub fn close_date(&self) -> i64 {
    self.close_date
  }

  pub fn is_closed(&self) -> bool {
    self.is_closed
  }
}

#[doc(hidden)]
pub struct RTDInputMessagePollBuilder {
  inner: InputMessagePoll,
}

impl RTDInputMessagePollBuilder {
  pub fn build(&self) -> InputMessagePoll {
    self.inner.clone()
  }

  pub fn question<T: AsRef<FormattedText>>(&mut self, question: T) -> &mut Self {
    self.inner.question = question.as_ref().clone();
    self
  }

  pub fn options(&mut self, options: Vec<InputPollOption>) -> &mut Self {
    self.inner.options = options;
    self
  }

  pub fn description<T: AsRef<FormattedText>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().clone();
    self
  }

  pub fn media<T: AsRef<InputPollMedia>>(&mut self, media: T) -> &mut Self {
    self.inner.media = media.as_ref().clone();
    self
  }

  pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
    self.inner.is_anonymous = is_anonymous;
    self
  }

  pub fn allows_multiple_answers(&mut self, allows_multiple_answers: bool) -> &mut Self {
    self.inner.allows_multiple_answers = allows_multiple_answers;
    self
  }

  pub fn allows_revoting(&mut self, allows_revoting: bool) -> &mut Self {
    self.inner.allows_revoting = allows_revoting;
    self
  }

  pub fn members_only(&mut self, members_only: bool) -> &mut Self {
    self.inner.members_only = members_only;
    self
  }

  pub fn country_codes(&mut self, country_codes: Vec<String>) -> &mut Self {
    self.inner.country_codes = country_codes;
    self
  }

  pub fn shuffle_options(&mut self, shuffle_options: bool) -> &mut Self {
    self.inner.shuffle_options = shuffle_options;
    self
  }

  pub fn hide_results_until_closes(&mut self, hide_results_until_closes: bool) -> &mut Self {
    self.inner.hide_results_until_closes = hide_results_until_closes;
    self
  }

  pub fn type_<T: AsRef<InputPollType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn open_period(&mut self, open_period: i64) -> &mut Self {
    self.inner.open_period = open_period;
    self
  }

  pub fn close_date(&mut self, close_date: i64) -> &mut Self {
    self.inner.close_date = close_date;
    self
  }

  pub fn is_closed(&mut self, is_closed: bool) -> &mut Self {
    self.inner.is_closed = is_closed;
    self
  }
}

impl AsRef<InputMessagePoll> for InputMessagePoll {
  fn as_ref(&self) -> &InputMessagePoll {
    self
  }
}

impl AsRef<InputMessagePoll> for RTDInputMessagePollBuilder {
  fn as_ref(&self) -> &InputMessagePoll {
    &self.inner
  }
}

/// A rich message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageRichMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The rich message to send
  message: InputRichMessage,
  /// Pass true to delete message draft in the chat
  clear_draft: bool,
}

impl RObject for InputMessageRichMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageRichMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageRichMessage {}

impl InputMessageRichMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageRichMessageBuilder {
    let inner = InputMessageRichMessage {
      td_name: "inputMessageRichMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageRichMessageBuilder { inner }
  }

  pub fn message(&self) -> &InputRichMessage {
    &self.message
  }

  pub fn clear_draft(&self) -> bool {
    self.clear_draft
  }
}

#[doc(hidden)]
pub struct RTDInputMessageRichMessageBuilder {
  inner: InputMessageRichMessage,
}

impl RTDInputMessageRichMessageBuilder {
  pub fn build(&self) -> InputMessageRichMessage {
    self.inner.clone()
  }

  pub fn message<T: AsRef<InputRichMessage>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

  pub fn clear_draft(&mut self, clear_draft: bool) -> &mut Self {
    self.inner.clear_draft = clear_draft;
    self
  }
}

impl AsRef<InputMessageRichMessage> for InputMessageRichMessage {
  fn as_ref(&self) -> &InputMessageRichMessage {
    self
  }
}

impl AsRef<InputMessageRichMessage> for RTDInputMessageRichMessageBuilder {
  fn as_ref(&self) -> &InputMessageRichMessage {
    &self.inner
  }
}

/// A stake dice message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageStakeDice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hash of the stake dice state. The state hash can be used only if it was received recently enough. Otherwise, a new state must be requested using getStakeDiceState
  state_hash: String,
  /// The TON Gram amount that will be staked; in the smallest units of the currency. Must be in the range getOption("stake_dice_stake_amount_min")-getOption("stake_dice_stake_amount_max")
  stake_gram_amount: i64,
  /// Pass true to delete message draft in the chat
  clear_draft: bool,
}

impl RObject for InputMessageStakeDice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageStakeDice"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageStakeDice {}

impl InputMessageStakeDice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageStakeDiceBuilder {
    let inner = InputMessageStakeDice {
      td_name: "inputMessageStakeDice".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageStakeDiceBuilder { inner }
  }

  pub fn state_hash(&self) -> &String {
    &self.state_hash
  }

  pub fn stake_gram_amount(&self) -> i64 {
    self.stake_gram_amount
  }

  pub fn clear_draft(&self) -> bool {
    self.clear_draft
  }
}

#[doc(hidden)]
pub struct RTDInputMessageStakeDiceBuilder {
  inner: InputMessageStakeDice,
}

impl RTDInputMessageStakeDiceBuilder {
  pub fn build(&self) -> InputMessageStakeDice {
    self.inner.clone()
  }

  pub fn state_hash<T: AsRef<str>>(&mut self, state_hash: T) -> &mut Self {
    self.inner.state_hash = state_hash.as_ref().to_string();
    self
  }

  pub fn stake_gram_amount(&mut self, stake_gram_amount: i64) -> &mut Self {
    self.inner.stake_gram_amount = stake_gram_amount;
    self
  }

  pub fn clear_draft(&mut self, clear_draft: bool) -> &mut Self {
    self.inner.clear_draft = clear_draft;
    self
  }
}

impl AsRef<InputMessageStakeDice> for InputMessageStakeDice {
  fn as_ref(&self) -> &InputMessageStakeDice {
    self
  }
}

impl AsRef<InputMessageStakeDice> for RTDInputMessageStakeDiceBuilder {
  fn as_ref(&self) -> &InputMessageStakeDice {
    &self.inner
  }
}

/// A sticker message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker to be sent
  sticker: InputSticker,
  /// Emoji used to choose the sticker
  emoji: String,
}

impl RObject for InputMessageSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageSticker {}

impl InputMessageSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageStickerBuilder {
    let inner = InputMessageSticker {
      td_name: "inputMessageSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &InputSticker {
    &self.sticker
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }
}

#[doc(hidden)]
pub struct RTDInputMessageStickerBuilder {
  inner: InputMessageSticker,
}

impl RTDInputMessageStickerBuilder {
  pub fn build(&self) -> InputMessageSticker {
    self.inner.clone()
  }

  pub fn sticker<T: AsRef<InputSticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }
}

impl AsRef<InputMessageSticker> for InputMessageSticker {
  fn as_ref(&self) -> &InputMessageSticker {
    self
  }
}

impl AsRef<InputMessageSticker> for RTDInputMessageStickerBuilder {
  fn as_ref(&self) -> &InputMessageSticker {
    &self.inner
  }
}

/// A message with a forwarded story. Stories can't be forwarded to secret chats. A story can be forwarded only if story.can_be_forwarded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that posted the story
  story_poster_chat_id: i64,
  /// Story identifier
  story_id: i64,
}

impl RObject for InputMessageStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageStory {}

impl InputMessageStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageStoryBuilder {
    let inner = InputMessageStory {
      td_name: "inputMessageStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageStoryBuilder { inner }
  }

  pub fn story_poster_chat_id(&self) -> i64 {
    self.story_poster_chat_id
  }

  pub fn story_id(&self) -> i64 {
    self.story_id
  }
}

#[doc(hidden)]
pub struct RTDInputMessageStoryBuilder {
  inner: InputMessageStory,
}

impl RTDInputMessageStoryBuilder {
  pub fn build(&self) -> InputMessageStory {
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

impl AsRef<InputMessageStory> for InputMessageStory {
  fn as_ref(&self) -> &InputMessageStory {
    self
  }
}

impl AsRef<InputMessageStory> for RTDInputMessageStoryBuilder {
  fn as_ref(&self) -> &InputMessageStory {
    &self.inner
  }
}

/// A text message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Formatted text to be sent; 0-getOption("message_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, BlockQuote, ExpandableBlockQuote, Code, Pre, PreCode, TextUrl, MentionName, and DateTime entities are allowed to be specified manually
  text: FormattedText,
  /// Options to be used for generation of a link preview; may be null if none; pass null to use default link preview options
  link_preview_options: Option<LinkPreviewOptions>,
  /// Pass true to delete message draft in the chat
  clear_draft: bool,
}

impl RObject for InputMessageText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageText"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageText {}

impl InputMessageText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageTextBuilder {
    let inner = InputMessageText {
      td_name: "inputMessageText".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageTextBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn link_preview_options(&self) -> &Option<LinkPreviewOptions> {
    &self.link_preview_options
  }

  pub fn clear_draft(&self) -> bool {
    self.clear_draft
  }
}

#[doc(hidden)]
pub struct RTDInputMessageTextBuilder {
  inner: InputMessageText,
}

impl RTDInputMessageTextBuilder {
  pub fn build(&self) -> InputMessageText {
    self.inner.clone()
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn link_preview_options<T: AsRef<LinkPreviewOptions>>(
    &mut self,
    link_preview_options: T,
  ) -> &mut Self {
    self.inner.link_preview_options = Some(link_preview_options.as_ref().clone());
    self
  }

  pub fn clear_draft(&mut self, clear_draft: bool) -> &mut Self {
    self.inner.clear_draft = clear_draft;
    self
  }
}

impl AsRef<InputMessageText> for InputMessageText {
  fn as_ref(&self) -> &InputMessageText {
    self
  }
}

impl AsRef<InputMessageText> for RTDInputMessageTextBuilder {
  fn as_ref(&self) -> &InputMessageText {
    &self.inner
  }
}

/// A message with information about a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Venue to send
  venue: Venue,
}

impl RObject for InputMessageVenue {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageVenue"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageVenue {}

impl InputMessageVenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageVenueBuilder {
    let inner = InputMessageVenue {
      td_name: "inputMessageVenue".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageVenueBuilder { inner }
  }

  pub fn venue(&self) -> &Venue {
    &self.venue
  }
}

#[doc(hidden)]
pub struct RTDInputMessageVenueBuilder {
  inner: InputMessageVenue,
}

impl RTDInputMessageVenueBuilder {
  pub fn build(&self) -> InputMessageVenue {
    self.inner.clone()
  }

  pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
    self.inner.venue = venue.as_ref().clone();
    self
  }
}

impl AsRef<InputMessageVenue> for InputMessageVenue {
  fn as_ref(&self) -> &InputMessageVenue {
    self
  }
}

impl AsRef<InputMessageVenue> for RTDInputMessageVenueBuilder {
  fn as_ref(&self) -> &InputMessageVenue {
    &self.inner
  }
}

/// A video message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Video to be sent
  video: InputVideo,
  /// Video caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
  caption: FormattedText,
  /// True, if the caption must be shown above the video; otherwise, the caption must be shown below the video; not supported in secret chats
  show_caption_above_media: bool,
  /// Video self-destruct type; pass null if none; private chats only
  self_destruct_type: MessageSelfDestructType,
  /// True, if the video preview must be covered by a spoiler animation; not supported in secret chats
  has_spoiler: bool,
}

impl RObject for InputMessageVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageVideo {}

impl InputMessageVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageVideoBuilder {
    let inner = InputMessageVideo {
      td_name: "inputMessageVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageVideoBuilder { inner }
  }

  pub fn video(&self) -> &InputVideo {
    &self.video
  }

  pub fn caption(&self) -> &FormattedText {
    &self.caption
  }

  pub fn show_caption_above_media(&self) -> bool {
    self.show_caption_above_media
  }

  pub fn self_destruct_type(&self) -> &MessageSelfDestructType {
    &self.self_destruct_type
  }

  pub fn has_spoiler(&self) -> bool {
    self.has_spoiler
  }
}

#[doc(hidden)]
pub struct RTDInputMessageVideoBuilder {
  inner: InputMessageVideo,
}

impl RTDInputMessageVideoBuilder {
  pub fn build(&self) -> InputMessageVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<InputVideo>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn show_caption_above_media(&mut self, show_caption_above_media: bool) -> &mut Self {
    self.inner.show_caption_above_media = show_caption_above_media;
    self
  }

  pub fn self_destruct_type<T: AsRef<MessageSelfDestructType>>(
    &mut self,
    self_destruct_type: T,
  ) -> &mut Self {
    self.inner.self_destruct_type = self_destruct_type.as_ref().clone();
    self
  }

  pub fn has_spoiler(&mut self, has_spoiler: bool) -> &mut Self {
    self.inner.has_spoiler = has_spoiler;
    self
  }
}

impl AsRef<InputMessageVideo> for InputMessageVideo {
  fn as_ref(&self) -> &InputMessageVideo {
    self
  }
}

impl AsRef<InputMessageVideo> for RTDInputMessageVideoBuilder {
  fn as_ref(&self) -> &InputMessageVideo {
    &self.inner
  }
}

/// A video note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Video note to be sent
  video_note: InputVideoNote,
  /// Video note self-destruct type; may be null if none; pass null if none; private chats only
  self_destruct_type: Option<MessageSelfDestructType>,
}

impl RObject for InputMessageVideoNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageVideoNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageVideoNote {}

impl InputMessageVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageVideoNoteBuilder {
    let inner = InputMessageVideoNote {
      td_name: "inputMessageVideoNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageVideoNoteBuilder { inner }
  }

  pub fn video_note(&self) -> &InputVideoNote {
    &self.video_note
  }

  pub fn self_destruct_type(&self) -> &Option<MessageSelfDestructType> {
    &self.self_destruct_type
  }
}

#[doc(hidden)]
pub struct RTDInputMessageVideoNoteBuilder {
  inner: InputMessageVideoNote,
}

impl RTDInputMessageVideoNoteBuilder {
  pub fn build(&self) -> InputMessageVideoNote {
    self.inner.clone()
  }

  pub fn video_note<T: AsRef<InputVideoNote>>(&mut self, video_note: T) -> &mut Self {
    self.inner.video_note = video_note.as_ref().clone();
    self
  }

  pub fn self_destruct_type<T: AsRef<MessageSelfDestructType>>(
    &mut self,
    self_destruct_type: T,
  ) -> &mut Self {
    self.inner.self_destruct_type = Some(self_destruct_type.as_ref().clone());
    self
  }
}

impl AsRef<InputMessageVideoNote> for InputMessageVideoNote {
  fn as_ref(&self) -> &InputMessageVideoNote {
    self
  }
}

impl AsRef<InputMessageVideoNote> for RTDInputMessageVideoNoteBuilder {
  fn as_ref(&self) -> &InputMessageVideoNote {
    &self.inner
  }
}

/// A voice note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Voice note to be sent
  voice_note: InputVoiceNote,
  /// Voice note caption; pass null to use an empty caption; 0-getOption("message_caption_length_max") characters
  caption: FormattedText,
  /// Voice note self-destruct type; may be null if none; pass null if none; private chats only
  self_destruct_type: Option<MessageSelfDestructType>,
}

impl RObject for InputMessageVoiceNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputMessageVoiceNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputMessageContent for InputMessageVoiceNote {}

impl InputMessageVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputMessageVoiceNoteBuilder {
    let inner = InputMessageVoiceNote {
      td_name: "inputMessageVoiceNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputMessageVoiceNoteBuilder { inner }
  }

  pub fn voice_note(&self) -> &InputVoiceNote {
    &self.voice_note
  }

  pub fn caption(&self) -> &FormattedText {
    &self.caption
  }

  pub fn self_destruct_type(&self) -> &Option<MessageSelfDestructType> {
    &self.self_destruct_type
  }
}

#[doc(hidden)]
pub struct RTDInputMessageVoiceNoteBuilder {
  inner: InputMessageVoiceNote,
}

impl RTDInputMessageVoiceNoteBuilder {
  pub fn build(&self) -> InputMessageVoiceNote {
    self.inner.clone()
  }

  pub fn voice_note<T: AsRef<InputVoiceNote>>(&mut self, voice_note: T) -> &mut Self {
    self.inner.voice_note = voice_note.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn self_destruct_type<T: AsRef<MessageSelfDestructType>>(
    &mut self,
    self_destruct_type: T,
  ) -> &mut Self {
    self.inner.self_destruct_type = Some(self_destruct_type.as_ref().clone());
    self
  }
}

impl AsRef<InputMessageVoiceNote> for InputMessageVoiceNote {
  fn as_ref(&self) -> &InputMessageVoiceNote {
    self
  }
}

impl AsRef<InputMessageVoiceNote> for RTDInputMessageVoiceNoteBuilder {
  fn as_ref(&self) -> &InputMessageVoiceNote {
    &self.inner
  }
}
