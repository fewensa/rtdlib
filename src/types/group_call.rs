use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Group call identifier
  id: i64,
  /// Persistent unique group call identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  unique_id: isize,
  /// Group call title; for video chats only
  title: String,
  /// Invite link for the group call; for group calls that aren't bound to a chat. For video chats call getVideoChatInviteLink to get the link. For live stories in chats with username call getInternalLink with internalLinkTypeLiveStory
  invite_link: String,
  /// The minimum number of Telegram Stars that must be paid by general participant for each sent message to the call; for live stories only
  paid_message_star_count: i64,
  /// Point in time (Unix timestamp) when the group call is expected to be started by an administrator; 0 if it is already active or was ended; for video chats only
  scheduled_start_date: i64,
  /// True, if the group call is scheduled and the current user will receive a notification when the group call starts; for video chats only
  enabled_start_notification: bool,
  /// True, if the call is active
  is_active: bool,
  /// True, if the call is bound to a chat
  is_video_chat: bool,
  /// True, if the call is a live story of a chat
  is_live_story: bool,
  /// True, if the call is an RTMP stream instead of an ordinary video chat; for video chats and live stories only
  is_rtmp_stream: bool,
  /// True, if the call is joined
  is_joined: bool,
  /// True, if user was kicked from the call because of network loss and the call needs to be rejoined
  need_rejoin: bool,
  /// True, if the user is the owner of the call and can end the call, change volume level of other users, or ban users there; for group calls that aren't bound to a chat
  is_owned: bool,
  /// True, if the current user can manage the group call; for video chats and live stories only
  can_be_managed: bool,
  /// Number of participants in the group call
  participant_count: i64,
  /// True, if group call participants, which are muted, aren't returned in participant list; for video chats only
  has_hidden_listeners: bool,
  /// True, if all group call participants are loaded
  loaded_all_participants: bool,
  /// Message sender chosen to send messages to the group call; for live stories only; may be null if the call isn't a live story
  message_sender_id: Option<MessageSender>,
  /// At most 3 recently speaking users in the group call
  recent_speakers: Vec<GroupCallRecentSpeaker>,
  /// True, if the current user's video is enabled
  is_my_video_enabled: bool,
  /// True, if the current user's video is paused
  is_my_video_paused: bool,
  /// True, if the current user can broadcast video or share screen
  can_enable_video: bool,
  /// True, if only group call administrators can unmute new participants; for video chats only
  mute_new_participants: bool,
  /// True, if the current user can enable or disable mute_new_participants setting; for video chats only
  can_toggle_mute_new_participants: bool,
  /// True, if the current user can send messages to the group call
  can_send_messages: bool,
  /// True, if sending of messages is allowed in the group call
  are_messages_allowed: bool,
  /// True, if the current user can enable or disable sending of messages in the group call
  can_toggle_are_messages_allowed: bool,
  /// True, if the user can delete messages in the group call
  can_delete_messages: bool,
  /// Duration of the ongoing group call recording, in seconds; 0 if none. An updateGroupCall update is not triggered when value of this field changes, but the same recording goes on
  record_duration: i64,
  /// True, if a video file is being recorded for the call
  is_video_recorded: bool,
  /// Call duration, in seconds; for ended calls only
  duration: i64,
}

impl RObject for GroupCall {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "groupCall"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GroupCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGroupCallBuilder {
    let inner = GroupCall {
      td_name: "groupCall".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGroupCallBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn unique_id(&self) -> isize {
    self.unique_id
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn invite_link(&self) -> &String {
    &self.invite_link
  }

  pub fn paid_message_star_count(&self) -> i64 {
    self.paid_message_star_count
  }

  pub fn scheduled_start_date(&self) -> i64 {
    self.scheduled_start_date
  }

  pub fn enabled_start_notification(&self) -> bool {
    self.enabled_start_notification
  }

  pub fn is_active(&self) -> bool {
    self.is_active
  }

  pub fn is_video_chat(&self) -> bool {
    self.is_video_chat
  }

  pub fn is_live_story(&self) -> bool {
    self.is_live_story
  }

  pub fn is_rtmp_stream(&self) -> bool {
    self.is_rtmp_stream
  }

  pub fn is_joined(&self) -> bool {
    self.is_joined
  }

  pub fn need_rejoin(&self) -> bool {
    self.need_rejoin
  }

  pub fn is_owned(&self) -> bool {
    self.is_owned
  }

  pub fn can_be_managed(&self) -> bool {
    self.can_be_managed
  }

  pub fn participant_count(&self) -> i64 {
    self.participant_count
  }

  pub fn has_hidden_listeners(&self) -> bool {
    self.has_hidden_listeners
  }

  pub fn loaded_all_participants(&self) -> bool {
    self.loaded_all_participants
  }

  pub fn message_sender_id(&self) -> &Option<MessageSender> {
    &self.message_sender_id
  }

  pub fn recent_speakers(&self) -> &Vec<GroupCallRecentSpeaker> {
    &self.recent_speakers
  }

  pub fn is_my_video_enabled(&self) -> bool {
    self.is_my_video_enabled
  }

  pub fn is_my_video_paused(&self) -> bool {
    self.is_my_video_paused
  }

  pub fn can_enable_video(&self) -> bool {
    self.can_enable_video
  }

  pub fn mute_new_participants(&self) -> bool {
    self.mute_new_participants
  }

  pub fn can_toggle_mute_new_participants(&self) -> bool {
    self.can_toggle_mute_new_participants
  }

  pub fn can_send_messages(&self) -> bool {
    self.can_send_messages
  }

  pub fn are_messages_allowed(&self) -> bool {
    self.are_messages_allowed
  }

  pub fn can_toggle_are_messages_allowed(&self) -> bool {
    self.can_toggle_are_messages_allowed
  }

  pub fn can_delete_messages(&self) -> bool {
    self.can_delete_messages
  }

  pub fn record_duration(&self) -> i64 {
    self.record_duration
  }

  pub fn is_video_recorded(&self) -> bool {
    self.is_video_recorded
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }
}

#[doc(hidden)]
pub struct RTDGroupCallBuilder {
  inner: GroupCall,
}

impl RTDGroupCallBuilder {
  pub fn build(&self) -> GroupCall {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn unique_id(&mut self, unique_id: isize) -> &mut Self {
    self.inner.unique_id = unique_id;
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }

  pub fn paid_message_star_count(&mut self, paid_message_star_count: i64) -> &mut Self {
    self.inner.paid_message_star_count = paid_message_star_count;
    self
  }

  pub fn scheduled_start_date(&mut self, scheduled_start_date: i64) -> &mut Self {
    self.inner.scheduled_start_date = scheduled_start_date;
    self
  }

  pub fn enabled_start_notification(&mut self, enabled_start_notification: bool) -> &mut Self {
    self.inner.enabled_start_notification = enabled_start_notification;
    self
  }

  pub fn is_active(&mut self, is_active: bool) -> &mut Self {
    self.inner.is_active = is_active;
    self
  }

  pub fn is_video_chat(&mut self, is_video_chat: bool) -> &mut Self {
    self.inner.is_video_chat = is_video_chat;
    self
  }

  pub fn is_live_story(&mut self, is_live_story: bool) -> &mut Self {
    self.inner.is_live_story = is_live_story;
    self
  }

  pub fn is_rtmp_stream(&mut self, is_rtmp_stream: bool) -> &mut Self {
    self.inner.is_rtmp_stream = is_rtmp_stream;
    self
  }

  pub fn is_joined(&mut self, is_joined: bool) -> &mut Self {
    self.inner.is_joined = is_joined;
    self
  }

  pub fn need_rejoin(&mut self, need_rejoin: bool) -> &mut Self {
    self.inner.need_rejoin = need_rejoin;
    self
  }

  pub fn is_owned(&mut self, is_owned: bool) -> &mut Self {
    self.inner.is_owned = is_owned;
    self
  }

  pub fn can_be_managed(&mut self, can_be_managed: bool) -> &mut Self {
    self.inner.can_be_managed = can_be_managed;
    self
  }

  pub fn participant_count(&mut self, participant_count: i64) -> &mut Self {
    self.inner.participant_count = participant_count;
    self
  }

  pub fn has_hidden_listeners(&mut self, has_hidden_listeners: bool) -> &mut Self {
    self.inner.has_hidden_listeners = has_hidden_listeners;
    self
  }

  pub fn loaded_all_participants(&mut self, loaded_all_participants: bool) -> &mut Self {
    self.inner.loaded_all_participants = loaded_all_participants;
    self
  }

  pub fn message_sender_id<T: AsRef<MessageSender>>(&mut self, message_sender_id: T) -> &mut Self {
    self.inner.message_sender_id = Some(message_sender_id.as_ref().clone());
    self
  }

  pub fn recent_speakers(&mut self, recent_speakers: Vec<GroupCallRecentSpeaker>) -> &mut Self {
    self.inner.recent_speakers = recent_speakers;
    self
  }

  pub fn is_my_video_enabled(&mut self, is_my_video_enabled: bool) -> &mut Self {
    self.inner.is_my_video_enabled = is_my_video_enabled;
    self
  }

  pub fn is_my_video_paused(&mut self, is_my_video_paused: bool) -> &mut Self {
    self.inner.is_my_video_paused = is_my_video_paused;
    self
  }

  pub fn can_enable_video(&mut self, can_enable_video: bool) -> &mut Self {
    self.inner.can_enable_video = can_enable_video;
    self
  }

  pub fn mute_new_participants(&mut self, mute_new_participants: bool) -> &mut Self {
    self.inner.mute_new_participants = mute_new_participants;
    self
  }

  pub fn can_toggle_mute_new_participants(
    &mut self,
    can_toggle_mute_new_participants: bool,
  ) -> &mut Self {
    self.inner.can_toggle_mute_new_participants = can_toggle_mute_new_participants;
    self
  }

  pub fn can_send_messages(&mut self, can_send_messages: bool) -> &mut Self {
    self.inner.can_send_messages = can_send_messages;
    self
  }

  pub fn are_messages_allowed(&mut self, are_messages_allowed: bool) -> &mut Self {
    self.inner.are_messages_allowed = are_messages_allowed;
    self
  }

  pub fn can_toggle_are_messages_allowed(
    &mut self,
    can_toggle_are_messages_allowed: bool,
  ) -> &mut Self {
    self.inner.can_toggle_are_messages_allowed = can_toggle_are_messages_allowed;
    self
  }

  pub fn can_delete_messages(&mut self, can_delete_messages: bool) -> &mut Self {
    self.inner.can_delete_messages = can_delete_messages;
    self
  }

  pub fn record_duration(&mut self, record_duration: i64) -> &mut Self {
    self.inner.record_duration = record_duration;
    self
  }

  pub fn is_video_recorded(&mut self, is_video_recorded: bool) -> &mut Self {
    self.inner.is_video_recorded = is_video_recorded;
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }
}

impl AsRef<GroupCall> for GroupCall {
  fn as_ref(&self) -> &GroupCall {
    self
  }
}

impl AsRef<GroupCall> for RTDGroupCallBuilder {
  fn as_ref(&self) -> &GroupCall {
    &self.inner
  }
}
