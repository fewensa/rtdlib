use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Content of the message draft
pub trait TDDraftMessageContent: Debug + RObject {}

/// Content of the message draft
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DraftMessageContent {
  #[doc(hidden)]
  _Default(()),
  /// A rich message draft; only for setChatDraftMessage
  InputRichMessage(DraftMessageContentInputRichMessage),
  /// A rich message draft; not supported in setChatDraftMessage
  RichMessage(DraftMessageContentRichMessage),
  /// A text message draft
  Text(DraftMessageContentText),
  /// A video note message draft
  VideoNote(DraftMessageContentVideoNote),
  /// A voice note message draft
  VoiceNote(DraftMessageContentVoiceNote),
}

impl Default for DraftMessageContent {
  fn default() -> Self {
    DraftMessageContent::_Default(())
  }
}

impl<'de> Deserialize<'de> for DraftMessageContent {
  fn deserialize<D>(deserializer: D) -> Result<DraftMessageContent, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      DraftMessageContent,
      (draftMessageContentInputRichMessage, InputRichMessage);
      (draftMessageContentRichMessage, RichMessage);
      (draftMessageContentText, Text);
      (draftMessageContentVideoNote, VideoNote);
      (draftMessageContentVoiceNote, VoiceNote);

    )(deserializer)
  }
}

impl RObject for DraftMessageContent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      DraftMessageContent::InputRichMessage(t) => t.td_name(),
      DraftMessageContent::RichMessage(t) => t.td_name(),
      DraftMessageContent::Text(t) => t.td_name(),
      DraftMessageContent::VideoNote(t) => t.td_name(),
      DraftMessageContent::VoiceNote(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      DraftMessageContent::InputRichMessage(t) => t.extra(),
      DraftMessageContent::RichMessage(t) => t.extra(),
      DraftMessageContent::Text(t) => t.extra(),
      DraftMessageContent::VideoNote(t) => t.extra(),
      DraftMessageContent::VoiceNote(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl DraftMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, DraftMessageContent::_Default(_))
  }

  pub fn is_input_rich_message(&self) -> bool {
    matches!(self, DraftMessageContent::InputRichMessage(_))
  }
  pub fn is_rich_message(&self) -> bool {
    matches!(self, DraftMessageContent::RichMessage(_))
  }
  pub fn is_text(&self) -> bool {
    matches!(self, DraftMessageContent::Text(_))
  }
  pub fn is_video_note(&self) -> bool {
    matches!(self, DraftMessageContent::VideoNote(_))
  }
  pub fn is_voice_note(&self) -> bool {
    matches!(self, DraftMessageContent::VoiceNote(_))
  }

  pub fn on_input_rich_message<F: FnOnce(&DraftMessageContentInputRichMessage)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let DraftMessageContent::InputRichMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_rich_message<F: FnOnce(&DraftMessageContentRichMessage)>(&self, fnc: F) -> &Self {
    if let DraftMessageContent::RichMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text<F: FnOnce(&DraftMessageContentText)>(&self, fnc: F) -> &Self {
    if let DraftMessageContent::Text(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_note<F: FnOnce(&DraftMessageContentVideoNote)>(&self, fnc: F) -> &Self {
    if let DraftMessageContent::VideoNote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_voice_note<F: FnOnce(&DraftMessageContentVoiceNote)>(&self, fnc: F) -> &Self {
    if let DraftMessageContent::VoiceNote(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_input_rich_message(&self) -> Option<&DraftMessageContentInputRichMessage> {
    if let DraftMessageContent::InputRichMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_rich_message(&self) -> Option<&DraftMessageContentRichMessage> {
    if let DraftMessageContent::RichMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text(&self) -> Option<&DraftMessageContentText> {
    if let DraftMessageContent::Text(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_note(&self) -> Option<&DraftMessageContentVideoNote> {
    if let DraftMessageContent::VideoNote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_voice_note(&self) -> Option<&DraftMessageContentVoiceNote> {
    if let DraftMessageContent::VoiceNote(t) = self {
      return Some(t);
    }
    None
  }

  pub fn input_rich_message<T: AsRef<DraftMessageContentInputRichMessage>>(t: T) -> Self {
    DraftMessageContent::InputRichMessage(t.as_ref().clone())
  }

  pub fn rich_message<T: AsRef<DraftMessageContentRichMessage>>(t: T) -> Self {
    DraftMessageContent::RichMessage(t.as_ref().clone())
  }

  pub fn text<T: AsRef<DraftMessageContentText>>(t: T) -> Self {
    DraftMessageContent::Text(t.as_ref().clone())
  }

  pub fn video_note<T: AsRef<DraftMessageContentVideoNote>>(t: T) -> Self {
    DraftMessageContent::VideoNote(t.as_ref().clone())
  }

  pub fn voice_note<T: AsRef<DraftMessageContentVoiceNote>>(t: T) -> Self {
    DraftMessageContent::VoiceNote(t.as_ref().clone())
  }
}

impl AsRef<DraftMessageContent> for DraftMessageContent {
  fn as_ref(&self) -> &DraftMessageContent {
    self
  }
}

/// A rich message draft; only for setChatDraftMessage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DraftMessageContentInputRichMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The rich message
  message: InputRichMessage,
}

impl RObject for DraftMessageContentInputRichMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "draftMessageContentInputRichMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDraftMessageContent for DraftMessageContentInputRichMessage {}

impl DraftMessageContentInputRichMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDraftMessageContentInputRichMessageBuilder {
    let inner = DraftMessageContentInputRichMessage {
      td_name: "draftMessageContentInputRichMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDraftMessageContentInputRichMessageBuilder { inner }
  }

  pub fn message(&self) -> &InputRichMessage {
    &self.message
  }
}

#[doc(hidden)]
pub struct RTDDraftMessageContentInputRichMessageBuilder {
  inner: DraftMessageContentInputRichMessage,
}

impl RTDDraftMessageContentInputRichMessageBuilder {
  pub fn build(&self) -> DraftMessageContentInputRichMessage {
    self.inner.clone()
  }

  pub fn message<T: AsRef<InputRichMessage>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }
}

impl AsRef<DraftMessageContentInputRichMessage> for DraftMessageContentInputRichMessage {
  fn as_ref(&self) -> &DraftMessageContentInputRichMessage {
    self
  }
}

impl AsRef<DraftMessageContentInputRichMessage> for RTDDraftMessageContentInputRichMessageBuilder {
  fn as_ref(&self) -> &DraftMessageContentInputRichMessage {
    &self.inner
  }
}

/// A rich message draft; not supported in setChatDraftMessage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DraftMessageContentRichMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The rich message
  message: RichMessage,
}

impl RObject for DraftMessageContentRichMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "draftMessageContentRichMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDraftMessageContent for DraftMessageContentRichMessage {}

impl DraftMessageContentRichMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDraftMessageContentRichMessageBuilder {
    let inner = DraftMessageContentRichMessage {
      td_name: "draftMessageContentRichMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDraftMessageContentRichMessageBuilder { inner }
  }

  pub fn message(&self) -> &RichMessage {
    &self.message
  }
}

#[doc(hidden)]
pub struct RTDDraftMessageContentRichMessageBuilder {
  inner: DraftMessageContentRichMessage,
}

impl RTDDraftMessageContentRichMessageBuilder {
  pub fn build(&self) -> DraftMessageContentRichMessage {
    self.inner.clone()
  }

  pub fn message<T: AsRef<RichMessage>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }
}

impl AsRef<DraftMessageContentRichMessage> for DraftMessageContentRichMessage {
  fn as_ref(&self) -> &DraftMessageContentRichMessage {
    self
  }
}

impl AsRef<DraftMessageContentRichMessage> for RTDDraftMessageContentRichMessageBuilder {
  fn as_ref(&self) -> &DraftMessageContentRichMessage {
    &self.inner
  }
}

/// A text message draft
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DraftMessageContentText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Formatted text to be saved as a draft; 0-getOption("message_text_length_max") characters
  text: FormattedText,
  /// Options to be used for generation of a link preview; may be null if none; pass null to use default link preview options
  link_preview_options: Option<LinkPreviewOptions>,
}

impl RObject for DraftMessageContentText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "draftMessageContentText"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDraftMessageContent for DraftMessageContentText {}

impl DraftMessageContentText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDraftMessageContentTextBuilder {
    let inner = DraftMessageContentText {
      td_name: "draftMessageContentText".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDraftMessageContentTextBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn link_preview_options(&self) -> &Option<LinkPreviewOptions> {
    &self.link_preview_options
  }
}

#[doc(hidden)]
pub struct RTDDraftMessageContentTextBuilder {
  inner: DraftMessageContentText,
}

impl RTDDraftMessageContentTextBuilder {
  pub fn build(&self) -> DraftMessageContentText {
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
}

impl AsRef<DraftMessageContentText> for DraftMessageContentText {
  fn as_ref(&self) -> &DraftMessageContentText {
    self
  }
}

impl AsRef<DraftMessageContentText> for RTDDraftMessageContentTextBuilder {
  fn as_ref(&self) -> &DraftMessageContentText {
    &self.inner
  }
}

/// A video note message draft
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DraftMessageContentVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Path to the file with the video note
  file_path: String,
  /// Duration of the video, in seconds; 0-60
  duration: i64,
  /// Video width and height; must be positive and not greater than 640
  length: i64,
  /// Video note self-destruct type; may be null if none; pass null if none; private chats only
  self_destruct_type: Option<MessageSelfDestructType>,
}

impl RObject for DraftMessageContentVideoNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "draftMessageContentVideoNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDraftMessageContent for DraftMessageContentVideoNote {}

impl DraftMessageContentVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDraftMessageContentVideoNoteBuilder {
    let inner = DraftMessageContentVideoNote {
      td_name: "draftMessageContentVideoNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDraftMessageContentVideoNoteBuilder { inner }
  }

  pub fn file_path(&self) -> &String {
    &self.file_path
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn length(&self) -> i64 {
    self.length
  }

  pub fn self_destruct_type(&self) -> &Option<MessageSelfDestructType> {
    &self.self_destruct_type
  }
}

#[doc(hidden)]
pub struct RTDDraftMessageContentVideoNoteBuilder {
  inner: DraftMessageContentVideoNote,
}

impl RTDDraftMessageContentVideoNoteBuilder {
  pub fn build(&self) -> DraftMessageContentVideoNote {
    self.inner.clone()
  }

  pub fn file_path<T: AsRef<str>>(&mut self, file_path: T) -> &mut Self {
    self.inner.file_path = file_path.as_ref().to_string();
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
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

impl AsRef<DraftMessageContentVideoNote> for DraftMessageContentVideoNote {
  fn as_ref(&self) -> &DraftMessageContentVideoNote {
    self
  }
}

impl AsRef<DraftMessageContentVideoNote> for RTDDraftMessageContentVideoNoteBuilder {
  fn as_ref(&self) -> &DraftMessageContentVideoNote {
    &self.inner
  }
}

/// A voice note message draft
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DraftMessageContentVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Path to the file with the voice note
  file_path: String,
  /// Duration of the voice note, in seconds
  duration: i64,
  /// Waveform representation of the voice note in 5-bit format
  waveform: String,
  /// Voice note self-destruct type; may be null if none; pass null if none; private chats only
  self_destruct_type: Option<MessageSelfDestructType>,
}

impl RObject for DraftMessageContentVoiceNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "draftMessageContentVoiceNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDDraftMessageContent for DraftMessageContentVoiceNote {}

impl DraftMessageContentVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDraftMessageContentVoiceNoteBuilder {
    let inner = DraftMessageContentVoiceNote {
      td_name: "draftMessageContentVoiceNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDraftMessageContentVoiceNoteBuilder { inner }
  }

  pub fn file_path(&self) -> &String {
    &self.file_path
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn waveform(&self) -> &String {
    &self.waveform
  }

  pub fn self_destruct_type(&self) -> &Option<MessageSelfDestructType> {
    &self.self_destruct_type
  }
}

#[doc(hidden)]
pub struct RTDDraftMessageContentVoiceNoteBuilder {
  inner: DraftMessageContentVoiceNote,
}

impl RTDDraftMessageContentVoiceNoteBuilder {
  pub fn build(&self) -> DraftMessageContentVoiceNote {
    self.inner.clone()
  }

  pub fn file_path<T: AsRef<str>>(&mut self, file_path: T) -> &mut Self {
    self.inner.file_path = file_path.as_ref().to_string();
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn waveform<T: AsRef<str>>(&mut self, waveform: T) -> &mut Self {
    self.inner.waveform = waveform.as_ref().to_string();
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

impl AsRef<DraftMessageContentVoiceNote> for DraftMessageContentVoiceNote {
  fn as_ref(&self) -> &DraftMessageContentVoiceNote {
    self
  }
}

impl AsRef<DraftMessageContentVoiceNote> for RTDDraftMessageContentVoiceNoteBuilder {
  fn as_ref(&self) -> &DraftMessageContentVoiceNote {
    &self.inner
  }
}
