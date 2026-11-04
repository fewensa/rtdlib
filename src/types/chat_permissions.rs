use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes actions that a user is allowed to take in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPermissions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the user can send text messages, rich messages, contacts, giveaways, giveaway winners, invoices, locations, and venues
  can_send_basic_messages: bool,
  /// True, if the user can send music files
  can_send_audios: bool,
  /// True, if the user can send documents
  can_send_documents: bool,
  /// True, if the user can send photos
  can_send_photos: bool,
  /// True, if the user can send videos
  can_send_videos: bool,
  /// True, if the user can send video notes
  can_send_video_notes: bool,
  /// True, if the user can send voice notes
  can_send_voice_notes: bool,
  /// True, if the user can send polls and checklists
  can_send_polls: bool,
  /// True, if the user can send animations, games, stickers, and dice and use inline bots
  can_send_other_messages: bool,
  /// True, if the user may add a link preview to their messages
  can_add_link_previews: bool,
  /// True, if the user can react to messages
  can_react_to_messages: bool,
  /// True, if the user may change the tag of self
  can_edit_tag: bool,
  /// True, if the user can change the chat title, photo, and other settings
  can_change_info: bool,
  /// True, if the user can invite new users to the chat
  can_invite_users: bool,
  /// True, if the user can pin messages
  can_pin_messages: bool,
  /// True, if the user can create topics
  can_create_topics: bool,
}

impl RObject for ChatPermissions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatPermissions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatPermissions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatPermissionsBuilder {
    let inner = ChatPermissions {
      td_name: "chatPermissions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatPermissionsBuilder { inner }
  }

  pub fn can_send_basic_messages(&self) -> bool {
    self.can_send_basic_messages
  }

  pub fn can_send_audios(&self) -> bool {
    self.can_send_audios
  }

  pub fn can_send_documents(&self) -> bool {
    self.can_send_documents
  }

  pub fn can_send_photos(&self) -> bool {
    self.can_send_photos
  }

  pub fn can_send_videos(&self) -> bool {
    self.can_send_videos
  }

  pub fn can_send_video_notes(&self) -> bool {
    self.can_send_video_notes
  }

  pub fn can_send_voice_notes(&self) -> bool {
    self.can_send_voice_notes
  }

  pub fn can_send_polls(&self) -> bool {
    self.can_send_polls
  }

  pub fn can_send_other_messages(&self) -> bool {
    self.can_send_other_messages
  }

  pub fn can_add_link_previews(&self) -> bool {
    self.can_add_link_previews
  }

  pub fn can_react_to_messages(&self) -> bool {
    self.can_react_to_messages
  }

  pub fn can_edit_tag(&self) -> bool {
    self.can_edit_tag
  }

  pub fn can_change_info(&self) -> bool {
    self.can_change_info
  }

  pub fn can_invite_users(&self) -> bool {
    self.can_invite_users
  }

  pub fn can_pin_messages(&self) -> bool {
    self.can_pin_messages
  }

  pub fn can_create_topics(&self) -> bool {
    self.can_create_topics
  }
}

#[doc(hidden)]
pub struct RTDChatPermissionsBuilder {
  inner: ChatPermissions,
}

impl RTDChatPermissionsBuilder {
  pub fn build(&self) -> ChatPermissions {
    self.inner.clone()
  }

  pub fn can_send_basic_messages(&mut self, can_send_basic_messages: bool) -> &mut Self {
    self.inner.can_send_basic_messages = can_send_basic_messages;
    self
  }

  pub fn can_send_audios(&mut self, can_send_audios: bool) -> &mut Self {
    self.inner.can_send_audios = can_send_audios;
    self
  }

  pub fn can_send_documents(&mut self, can_send_documents: bool) -> &mut Self {
    self.inner.can_send_documents = can_send_documents;
    self
  }

  pub fn can_send_photos(&mut self, can_send_photos: bool) -> &mut Self {
    self.inner.can_send_photos = can_send_photos;
    self
  }

  pub fn can_send_videos(&mut self, can_send_videos: bool) -> &mut Self {
    self.inner.can_send_videos = can_send_videos;
    self
  }

  pub fn can_send_video_notes(&mut self, can_send_video_notes: bool) -> &mut Self {
    self.inner.can_send_video_notes = can_send_video_notes;
    self
  }

  pub fn can_send_voice_notes(&mut self, can_send_voice_notes: bool) -> &mut Self {
    self.inner.can_send_voice_notes = can_send_voice_notes;
    self
  }

  pub fn can_send_polls(&mut self, can_send_polls: bool) -> &mut Self {
    self.inner.can_send_polls = can_send_polls;
    self
  }

  pub fn can_send_other_messages(&mut self, can_send_other_messages: bool) -> &mut Self {
    self.inner.can_send_other_messages = can_send_other_messages;
    self
  }

  pub fn can_add_link_previews(&mut self, can_add_link_previews: bool) -> &mut Self {
    self.inner.can_add_link_previews = can_add_link_previews;
    self
  }

  pub fn can_react_to_messages(&mut self, can_react_to_messages: bool) -> &mut Self {
    self.inner.can_react_to_messages = can_react_to_messages;
    self
  }

  pub fn can_edit_tag(&mut self, can_edit_tag: bool) -> &mut Self {
    self.inner.can_edit_tag = can_edit_tag;
    self
  }

  pub fn can_change_info(&mut self, can_change_info: bool) -> &mut Self {
    self.inner.can_change_info = can_change_info;
    self
  }

  pub fn can_invite_users(&mut self, can_invite_users: bool) -> &mut Self {
    self.inner.can_invite_users = can_invite_users;
    self
  }

  pub fn can_pin_messages(&mut self, can_pin_messages: bool) -> &mut Self {
    self.inner.can_pin_messages = can_pin_messages;
    self
  }

  pub fn can_create_topics(&mut self, can_create_topics: bool) -> &mut Self {
    self.inner.can_create_topics = can_create_topics;
    self
  }
}

impl AsRef<ChatPermissions> for ChatPermissions {
  fn as_ref(&self) -> &ChatPermissions {
    self
  }
}

impl AsRef<ChatPermissions> for RTDChatPermissionsBuilder {
  fn as_ref(&self) -> &ChatPermissions {
    &self.inner
  }
}
