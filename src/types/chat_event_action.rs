
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a chat event
pub trait TDChatEventAction: Debug + RObject {}

/// Represents a chat event
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatEventAction {
  #[doc(hidden)] _Default(()),
  /// The chat description was changed
  ChatEventDescriptionChanged(ChatEventDescriptionChanged),
  /// A revoked chat invite link was deleted
  ChatEventInviteLinkDeleted(ChatEventInviteLinkDeleted),
  /// A chat invite link was edited
  ChatEventInviteLinkEdited(ChatEventInviteLinkEdited),
  /// A chat invite link was revoked
  ChatEventInviteLinkRevoked(ChatEventInviteLinkRevoked),
  /// The can_invite_users permission of a supergroup chat was toggled
  ChatEventInvitesToggled(ChatEventInvitesToggled),
  /// The is_all_history_available setting of a supergroup was toggled
  ChatEventIsAllHistoryAvailableToggled(ChatEventIsAllHistoryAvailableToggled),
  /// The linked chat of a supergroup was changed
  ChatEventLinkedChatChanged(ChatEventLinkedChatChanged),
  /// The supergroup location was changed
  ChatEventLocationChanged(ChatEventLocationChanged),
  /// A new chat member was invited
  ChatEventMemberInvited(ChatEventMemberInvited),
  /// A new member joined the chat
  ChatEventMemberJoined(ChatEventMemberJoined),
  /// A new member joined the chat by an invite link
  ChatEventMemberJoinedByInviteLink(ChatEventMemberJoinedByInviteLink),
  /// A new member was accepted to the chat by an administrator
  ChatEventMemberJoinedByRequest(ChatEventMemberJoinedByRequest),
  /// A member left the chat
  ChatEventMemberLeft(ChatEventMemberLeft),
  /// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
  ChatEventMemberPromoted(ChatEventMemberPromoted),
  /// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
  ChatEventMemberRestricted(ChatEventMemberRestricted),
  /// A message was deleted
  ChatEventMessageDeleted(ChatEventMessageDeleted),
  /// A message was edited
  ChatEventMessageEdited(ChatEventMessageEdited),
  /// A message was pinned
  ChatEventMessagePinned(ChatEventMessagePinned),
  /// The message TTL setting was changed
  ChatEventMessageTtlSettingChanged(ChatEventMessageTtlSettingChanged),
  /// A message was unpinned
  ChatEventMessageUnpinned(ChatEventMessageUnpinned),
  /// The chat permissions was changed
  ChatEventPermissionsChanged(ChatEventPermissionsChanged),
  /// The chat photo was changed
  ChatEventPhotoChanged(ChatEventPhotoChanged),
  /// A poll in a message was stopped
  ChatEventPollStopped(ChatEventPollStopped),
  /// The sign_messages setting of a channel was toggled
  ChatEventSignMessagesToggled(ChatEventSignMessagesToggled),
  /// The slow_mode_delay setting of a supergroup was changed
  ChatEventSlowModeDelayChanged(ChatEventSlowModeDelayChanged),
  /// The supergroup sticker set was changed
  ChatEventStickerSetChanged(ChatEventStickerSetChanged),
  /// The chat title was changed
  ChatEventTitleChanged(ChatEventTitleChanged),
  /// The chat username was changed
  ChatEventUsernameChanged(ChatEventUsernameChanged),
  /// A video chat was created
  ChatEventVideoChatCreated(ChatEventVideoChatCreated),
  /// A video chat was discarded
  ChatEventVideoChatDiscarded(ChatEventVideoChatDiscarded),
  /// The mute_new_participants setting of a video chat was toggled
  ChatEventVideoChatMuteNewParticipantsToggled(ChatEventVideoChatMuteNewParticipantsToggled),
  /// A video chat participant was muted or unmuted
  ChatEventVideoChatParticipantIsMutedToggled(ChatEventVideoChatParticipantIsMutedToggled),
  /// A video chat participant volume level was changed
  ChatEventVideoChatParticipantVolumeLevelChanged(ChatEventVideoChatParticipantVolumeLevelChanged),

}

impl Default for ChatEventAction {
  fn default() -> Self { ChatEventAction::_Default(()) }
}

impl<'de> Deserialize<'de> for ChatEventAction {
  fn deserialize<D>(deserializer: D) -> Result<ChatEventAction, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatEventAction,
      (chatEventDescriptionChanged, ChatEventDescriptionChanged);
      (chatEventInviteLinkDeleted, ChatEventInviteLinkDeleted);
      (chatEventInviteLinkEdited, ChatEventInviteLinkEdited);
      (chatEventInviteLinkRevoked, ChatEventInviteLinkRevoked);
      (chatEventInvitesToggled, ChatEventInvitesToggled);
      (chatEventIsAllHistoryAvailableToggled, ChatEventIsAllHistoryAvailableToggled);
      (chatEventLinkedChatChanged, ChatEventLinkedChatChanged);
      (chatEventLocationChanged, ChatEventLocationChanged);
      (chatEventMemberInvited, ChatEventMemberInvited);
      (chatEventMemberJoined, ChatEventMemberJoined);
      (chatEventMemberJoinedByInviteLink, ChatEventMemberJoinedByInviteLink);
      (chatEventMemberJoinedByRequest, ChatEventMemberJoinedByRequest);
      (chatEventMemberLeft, ChatEventMemberLeft);
      (chatEventMemberPromoted, ChatEventMemberPromoted);
      (chatEventMemberRestricted, ChatEventMemberRestricted);
      (chatEventMessageDeleted, ChatEventMessageDeleted);
      (chatEventMessageEdited, ChatEventMessageEdited);
      (chatEventMessagePinned, ChatEventMessagePinned);
      (chatEventMessageTtlSettingChanged, ChatEventMessageTtlSettingChanged);
      (chatEventMessageUnpinned, ChatEventMessageUnpinned);
      (chatEventPermissionsChanged, ChatEventPermissionsChanged);
      (chatEventPhotoChanged, ChatEventPhotoChanged);
      (chatEventPollStopped, ChatEventPollStopped);
      (chatEventSignMessagesToggled, ChatEventSignMessagesToggled);
      (chatEventSlowModeDelayChanged, ChatEventSlowModeDelayChanged);
      (chatEventStickerSetChanged, ChatEventStickerSetChanged);
      (chatEventTitleChanged, ChatEventTitleChanged);
      (chatEventUsernameChanged, ChatEventUsernameChanged);
      (chatEventVideoChatCreated, ChatEventVideoChatCreated);
      (chatEventVideoChatDiscarded, ChatEventVideoChatDiscarded);
      (chatEventVideoChatMuteNewParticipantsToggled, ChatEventVideoChatMuteNewParticipantsToggled);
      (chatEventVideoChatParticipantIsMutedToggled, ChatEventVideoChatParticipantIsMutedToggled);
      (chatEventVideoChatParticipantVolumeLevelChanged, ChatEventVideoChatParticipantVolumeLevelChanged);

    )(deserializer)
  }
}

impl RObject for ChatEventAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ChatEventAction::ChatEventDescriptionChanged(t) => t.td_name(),
      ChatEventAction::ChatEventInviteLinkDeleted(t) => t.td_name(),
      ChatEventAction::ChatEventInviteLinkEdited(t) => t.td_name(),
      ChatEventAction::ChatEventInviteLinkRevoked(t) => t.td_name(),
      ChatEventAction::ChatEventInvitesToggled(t) => t.td_name(),
      ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t) => t.td_name(),
      ChatEventAction::ChatEventLinkedChatChanged(t) => t.td_name(),
      ChatEventAction::ChatEventLocationChanged(t) => t.td_name(),
      ChatEventAction::ChatEventMemberInvited(t) => t.td_name(),
      ChatEventAction::ChatEventMemberJoined(t) => t.td_name(),
      ChatEventAction::ChatEventMemberJoinedByInviteLink(t) => t.td_name(),
      ChatEventAction::ChatEventMemberJoinedByRequest(t) => t.td_name(),
      ChatEventAction::ChatEventMemberLeft(t) => t.td_name(),
      ChatEventAction::ChatEventMemberPromoted(t) => t.td_name(),
      ChatEventAction::ChatEventMemberRestricted(t) => t.td_name(),
      ChatEventAction::ChatEventMessageDeleted(t) => t.td_name(),
      ChatEventAction::ChatEventMessageEdited(t) => t.td_name(),
      ChatEventAction::ChatEventMessagePinned(t) => t.td_name(),
      ChatEventAction::ChatEventMessageTtlSettingChanged(t) => t.td_name(),
      ChatEventAction::ChatEventMessageUnpinned(t) => t.td_name(),
      ChatEventAction::ChatEventPermissionsChanged(t) => t.td_name(),
      ChatEventAction::ChatEventPhotoChanged(t) => t.td_name(),
      ChatEventAction::ChatEventPollStopped(t) => t.td_name(),
      ChatEventAction::ChatEventSignMessagesToggled(t) => t.td_name(),
      ChatEventAction::ChatEventSlowModeDelayChanged(t) => t.td_name(),
      ChatEventAction::ChatEventStickerSetChanged(t) => t.td_name(),
      ChatEventAction::ChatEventTitleChanged(t) => t.td_name(),
      ChatEventAction::ChatEventUsernameChanged(t) => t.td_name(),
      ChatEventAction::ChatEventVideoChatCreated(t) => t.td_name(),
      ChatEventAction::ChatEventVideoChatDiscarded(t) => t.td_name(),
      ChatEventAction::ChatEventVideoChatMuteNewParticipantsToggled(t) => t.td_name(),
      ChatEventAction::ChatEventVideoChatParticipantIsMutedToggled(t) => t.td_name(),
      ChatEventAction::ChatEventVideoChatParticipantVolumeLevelChanged(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      ChatEventAction::ChatEventDescriptionChanged(t) => t.extra(),
      ChatEventAction::ChatEventInviteLinkDeleted(t) => t.extra(),
      ChatEventAction::ChatEventInviteLinkEdited(t) => t.extra(),
      ChatEventAction::ChatEventInviteLinkRevoked(t) => t.extra(),
      ChatEventAction::ChatEventInvitesToggled(t) => t.extra(),
      ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t) => t.extra(),
      ChatEventAction::ChatEventLinkedChatChanged(t) => t.extra(),
      ChatEventAction::ChatEventLocationChanged(t) => t.extra(),
      ChatEventAction::ChatEventMemberInvited(t) => t.extra(),
      ChatEventAction::ChatEventMemberJoined(t) => t.extra(),
      ChatEventAction::ChatEventMemberJoinedByInviteLink(t) => t.extra(),
      ChatEventAction::ChatEventMemberJoinedByRequest(t) => t.extra(),
      ChatEventAction::ChatEventMemberLeft(t) => t.extra(),
      ChatEventAction::ChatEventMemberPromoted(t) => t.extra(),
      ChatEventAction::ChatEventMemberRestricted(t) => t.extra(),
      ChatEventAction::ChatEventMessageDeleted(t) => t.extra(),
      ChatEventAction::ChatEventMessageEdited(t) => t.extra(),
      ChatEventAction::ChatEventMessagePinned(t) => t.extra(),
      ChatEventAction::ChatEventMessageTtlSettingChanged(t) => t.extra(),
      ChatEventAction::ChatEventMessageUnpinned(t) => t.extra(),
      ChatEventAction::ChatEventPermissionsChanged(t) => t.extra(),
      ChatEventAction::ChatEventPhotoChanged(t) => t.extra(),
      ChatEventAction::ChatEventPollStopped(t) => t.extra(),
      ChatEventAction::ChatEventSignMessagesToggled(t) => t.extra(),
      ChatEventAction::ChatEventSlowModeDelayChanged(t) => t.extra(),
      ChatEventAction::ChatEventStickerSetChanged(t) => t.extra(),
      ChatEventAction::ChatEventTitleChanged(t) => t.extra(),
      ChatEventAction::ChatEventUsernameChanged(t) => t.extra(),
      ChatEventAction::ChatEventVideoChatCreated(t) => t.extra(),
      ChatEventAction::ChatEventVideoChatDiscarded(t) => t.extra(),
      ChatEventAction::ChatEventVideoChatMuteNewParticipantsToggled(t) => t.extra(),
      ChatEventAction::ChatEventVideoChatParticipantIsMutedToggled(t) => t.extra(),
      ChatEventAction::ChatEventVideoChatParticipantVolumeLevelChanged(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ChatEventAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ChatEventAction::_Default(_) = self { true } else { false } }

  pub fn is_chat_event_description_changed(&self) -> bool { if let ChatEventAction::ChatEventDescriptionChanged(_) = self { true } else { false } }
  pub fn is_chat_event_invite_link_deleted(&self) -> bool { if let ChatEventAction::ChatEventInviteLinkDeleted(_) = self { true } else { false } }
  pub fn is_chat_event_invite_link_edited(&self) -> bool { if let ChatEventAction::ChatEventInviteLinkEdited(_) = self { true } else { false } }
  pub fn is_chat_event_invite_link_revoked(&self) -> bool { if let ChatEventAction::ChatEventInviteLinkRevoked(_) = self { true } else { false } }
  pub fn is_chat_event_invites_toggled(&self) -> bool { if let ChatEventAction::ChatEventInvitesToggled(_) = self { true } else { false } }
  pub fn is_chat_event_is_all_history_available_toggled(&self) -> bool { if let ChatEventAction::ChatEventIsAllHistoryAvailableToggled(_) = self { true } else { false } }
  pub fn is_chat_event_linked_chat_changed(&self) -> bool { if let ChatEventAction::ChatEventLinkedChatChanged(_) = self { true } else { false } }
  pub fn is_chat_event_location_changed(&self) -> bool { if let ChatEventAction::ChatEventLocationChanged(_) = self { true } else { false } }
  pub fn is_chat_event_member_invited(&self) -> bool { if let ChatEventAction::ChatEventMemberInvited(_) = self { true } else { false } }
  pub fn is_chat_event_member_joined(&self) -> bool { if let ChatEventAction::ChatEventMemberJoined(_) = self { true } else { false } }
  pub fn is_chat_event_member_joined_by_invite_link(&self) -> bool { if let ChatEventAction::ChatEventMemberJoinedByInviteLink(_) = self { true } else { false } }
  pub fn is_chat_event_member_joined_by_request(&self) -> bool { if let ChatEventAction::ChatEventMemberJoinedByRequest(_) = self { true } else { false } }
  pub fn is_chat_event_member_left(&self) -> bool { if let ChatEventAction::ChatEventMemberLeft(_) = self { true } else { false } }
  pub fn is_chat_event_member_promoted(&self) -> bool { if let ChatEventAction::ChatEventMemberPromoted(_) = self { true } else { false } }
  pub fn is_chat_event_member_restricted(&self) -> bool { if let ChatEventAction::ChatEventMemberRestricted(_) = self { true } else { false } }
  pub fn is_chat_event_message_deleted(&self) -> bool { if let ChatEventAction::ChatEventMessageDeleted(_) = self { true } else { false } }
  pub fn is_chat_event_message_edited(&self) -> bool { if let ChatEventAction::ChatEventMessageEdited(_) = self { true } else { false } }
  pub fn is_chat_event_message_pinned(&self) -> bool { if let ChatEventAction::ChatEventMessagePinned(_) = self { true } else { false } }
  pub fn is_chat_event_message_ttl_setting_changed(&self) -> bool { if let ChatEventAction::ChatEventMessageTtlSettingChanged(_) = self { true } else { false } }
  pub fn is_chat_event_message_unpinned(&self) -> bool { if let ChatEventAction::ChatEventMessageUnpinned(_) = self { true } else { false } }
  pub fn is_chat_event_permissions_changed(&self) -> bool { if let ChatEventAction::ChatEventPermissionsChanged(_) = self { true } else { false } }
  pub fn is_chat_event_photo_changed(&self) -> bool { if let ChatEventAction::ChatEventPhotoChanged(_) = self { true } else { false } }
  pub fn is_chat_event_poll_stopped(&self) -> bool { if let ChatEventAction::ChatEventPollStopped(_) = self { true } else { false } }
  pub fn is_chat_event_sign_messages_toggled(&self) -> bool { if let ChatEventAction::ChatEventSignMessagesToggled(_) = self { true } else { false } }
  pub fn is_chat_event_slow_mode_delay_changed(&self) -> bool { if let ChatEventAction::ChatEventSlowModeDelayChanged(_) = self { true } else { false } }
  pub fn is_chat_event_sticker_set_changed(&self) -> bool { if let ChatEventAction::ChatEventStickerSetChanged(_) = self { true } else { false } }
  pub fn is_chat_event_title_changed(&self) -> bool { if let ChatEventAction::ChatEventTitleChanged(_) = self { true } else { false } }
  pub fn is_chat_event_username_changed(&self) -> bool { if let ChatEventAction::ChatEventUsernameChanged(_) = self { true } else { false } }
  pub fn is_chat_event_video_chat_created(&self) -> bool { if let ChatEventAction::ChatEventVideoChatCreated(_) = self { true } else { false } }
  pub fn is_chat_event_video_chat_discarded(&self) -> bool { if let ChatEventAction::ChatEventVideoChatDiscarded(_) = self { true } else { false } }
  pub fn is_chat_event_video_chat_mute_new_participants_toggled(&self) -> bool { if let ChatEventAction::ChatEventVideoChatMuteNewParticipantsToggled(_) = self { true } else { false } }
  pub fn is_chat_event_video_chat_participant_is_muted_toggled(&self) -> bool { if let ChatEventAction::ChatEventVideoChatParticipantIsMutedToggled(_) = self { true } else { false } }
  pub fn is_chat_event_video_chat_participant_volume_level_changed(&self) -> bool { if let ChatEventAction::ChatEventVideoChatParticipantVolumeLevelChanged(_) = self { true } else { false } }

  pub fn on_chat_event_description_changed<F: FnOnce(&ChatEventDescriptionChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventDescriptionChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_invite_link_deleted<F: FnOnce(&ChatEventInviteLinkDeleted)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventInviteLinkDeleted(t) = self { fnc(t) }; self }
  pub fn on_chat_event_invite_link_edited<F: FnOnce(&ChatEventInviteLinkEdited)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventInviteLinkEdited(t) = self { fnc(t) }; self }
  pub fn on_chat_event_invite_link_revoked<F: FnOnce(&ChatEventInviteLinkRevoked)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventInviteLinkRevoked(t) = self { fnc(t) }; self }
  pub fn on_chat_event_invites_toggled<F: FnOnce(&ChatEventInvitesToggled)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventInvitesToggled(t) = self { fnc(t) }; self }
  pub fn on_chat_event_is_all_history_available_toggled<F: FnOnce(&ChatEventIsAllHistoryAvailableToggled)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t) = self { fnc(t) }; self }
  pub fn on_chat_event_linked_chat_changed<F: FnOnce(&ChatEventLinkedChatChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventLinkedChatChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_location_changed<F: FnOnce(&ChatEventLocationChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventLocationChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_invited<F: FnOnce(&ChatEventMemberInvited)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberInvited(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_joined<F: FnOnce(&ChatEventMemberJoined)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberJoined(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_joined_by_invite_link<F: FnOnce(&ChatEventMemberJoinedByInviteLink)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberJoinedByInviteLink(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_joined_by_request<F: FnOnce(&ChatEventMemberJoinedByRequest)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberJoinedByRequest(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_left<F: FnOnce(&ChatEventMemberLeft)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberLeft(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_promoted<F: FnOnce(&ChatEventMemberPromoted)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberPromoted(t) = self { fnc(t) }; self }
  pub fn on_chat_event_member_restricted<F: FnOnce(&ChatEventMemberRestricted)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMemberRestricted(t) = self { fnc(t) }; self }
  pub fn on_chat_event_message_deleted<F: FnOnce(&ChatEventMessageDeleted)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMessageDeleted(t) = self { fnc(t) }; self }
  pub fn on_chat_event_message_edited<F: FnOnce(&ChatEventMessageEdited)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMessageEdited(t) = self { fnc(t) }; self }
  pub fn on_chat_event_message_pinned<F: FnOnce(&ChatEventMessagePinned)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMessagePinned(t) = self { fnc(t) }; self }
  pub fn on_chat_event_message_ttl_setting_changed<F: FnOnce(&ChatEventMessageTtlSettingChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMessageTtlSettingChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_message_unpinned<F: FnOnce(&ChatEventMessageUnpinned)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventMessageUnpinned(t) = self { fnc(t) }; self }
  pub fn on_chat_event_permissions_changed<F: FnOnce(&ChatEventPermissionsChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventPermissionsChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_photo_changed<F: FnOnce(&ChatEventPhotoChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventPhotoChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_poll_stopped<F: FnOnce(&ChatEventPollStopped)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventPollStopped(t) = self { fnc(t) }; self }
  pub fn on_chat_event_sign_messages_toggled<F: FnOnce(&ChatEventSignMessagesToggled)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventSignMessagesToggled(t) = self { fnc(t) }; self }
  pub fn on_chat_event_slow_mode_delay_changed<F: FnOnce(&ChatEventSlowModeDelayChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventSlowModeDelayChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_sticker_set_changed<F: FnOnce(&ChatEventStickerSetChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventStickerSetChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_title_changed<F: FnOnce(&ChatEventTitleChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventTitleChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_username_changed<F: FnOnce(&ChatEventUsernameChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventUsernameChanged(t) = self { fnc(t) }; self }
  pub fn on_chat_event_video_chat_created<F: FnOnce(&ChatEventVideoChatCreated)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventVideoChatCreated(t) = self { fnc(t) }; self }
  pub fn on_chat_event_video_chat_discarded<F: FnOnce(&ChatEventVideoChatDiscarded)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventVideoChatDiscarded(t) = self { fnc(t) }; self }
  pub fn on_chat_event_video_chat_mute_new_participants_toggled<F: FnOnce(&ChatEventVideoChatMuteNewParticipantsToggled)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventVideoChatMuteNewParticipantsToggled(t) = self { fnc(t) }; self }
  pub fn on_chat_event_video_chat_participant_is_muted_toggled<F: FnOnce(&ChatEventVideoChatParticipantIsMutedToggled)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventVideoChatParticipantIsMutedToggled(t) = self { fnc(t) }; self }
  pub fn on_chat_event_video_chat_participant_volume_level_changed<F: FnOnce(&ChatEventVideoChatParticipantVolumeLevelChanged)>(&self, fnc: F) -> &Self { if let ChatEventAction::ChatEventVideoChatParticipantVolumeLevelChanged(t) = self { fnc(t) }; self }

  pub fn as_chat_event_description_changed(&self) -> Option<&ChatEventDescriptionChanged> { if let ChatEventAction::ChatEventDescriptionChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_invite_link_deleted(&self) -> Option<&ChatEventInviteLinkDeleted> { if let ChatEventAction::ChatEventInviteLinkDeleted(t) = self { return Some(t) } None }
  pub fn as_chat_event_invite_link_edited(&self) -> Option<&ChatEventInviteLinkEdited> { if let ChatEventAction::ChatEventInviteLinkEdited(t) = self { return Some(t) } None }
  pub fn as_chat_event_invite_link_revoked(&self) -> Option<&ChatEventInviteLinkRevoked> { if let ChatEventAction::ChatEventInviteLinkRevoked(t) = self { return Some(t) } None }
  pub fn as_chat_event_invites_toggled(&self) -> Option<&ChatEventInvitesToggled> { if let ChatEventAction::ChatEventInvitesToggled(t) = self { return Some(t) } None }
  pub fn as_chat_event_is_all_history_available_toggled(&self) -> Option<&ChatEventIsAllHistoryAvailableToggled> { if let ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t) = self { return Some(t) } None }
  pub fn as_chat_event_linked_chat_changed(&self) -> Option<&ChatEventLinkedChatChanged> { if let ChatEventAction::ChatEventLinkedChatChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_location_changed(&self) -> Option<&ChatEventLocationChanged> { if let ChatEventAction::ChatEventLocationChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_invited(&self) -> Option<&ChatEventMemberInvited> { if let ChatEventAction::ChatEventMemberInvited(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_joined(&self) -> Option<&ChatEventMemberJoined> { if let ChatEventAction::ChatEventMemberJoined(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_joined_by_invite_link(&self) -> Option<&ChatEventMemberJoinedByInviteLink> { if let ChatEventAction::ChatEventMemberJoinedByInviteLink(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_joined_by_request(&self) -> Option<&ChatEventMemberJoinedByRequest> { if let ChatEventAction::ChatEventMemberJoinedByRequest(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_left(&self) -> Option<&ChatEventMemberLeft> { if let ChatEventAction::ChatEventMemberLeft(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_promoted(&self) -> Option<&ChatEventMemberPromoted> { if let ChatEventAction::ChatEventMemberPromoted(t) = self { return Some(t) } None }
  pub fn as_chat_event_member_restricted(&self) -> Option<&ChatEventMemberRestricted> { if let ChatEventAction::ChatEventMemberRestricted(t) = self { return Some(t) } None }
  pub fn as_chat_event_message_deleted(&self) -> Option<&ChatEventMessageDeleted> { if let ChatEventAction::ChatEventMessageDeleted(t) = self { return Some(t) } None }
  pub fn as_chat_event_message_edited(&self) -> Option<&ChatEventMessageEdited> { if let ChatEventAction::ChatEventMessageEdited(t) = self { return Some(t) } None }
  pub fn as_chat_event_message_pinned(&self) -> Option<&ChatEventMessagePinned> { if let ChatEventAction::ChatEventMessagePinned(t) = self { return Some(t) } None }
  pub fn as_chat_event_message_ttl_setting_changed(&self) -> Option<&ChatEventMessageTtlSettingChanged> { if let ChatEventAction::ChatEventMessageTtlSettingChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_message_unpinned(&self) -> Option<&ChatEventMessageUnpinned> { if let ChatEventAction::ChatEventMessageUnpinned(t) = self { return Some(t) } None }
  pub fn as_chat_event_permissions_changed(&self) -> Option<&ChatEventPermissionsChanged> { if let ChatEventAction::ChatEventPermissionsChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_photo_changed(&self) -> Option<&ChatEventPhotoChanged> { if let ChatEventAction::ChatEventPhotoChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_poll_stopped(&self) -> Option<&ChatEventPollStopped> { if let ChatEventAction::ChatEventPollStopped(t) = self { return Some(t) } None }
  pub fn as_chat_event_sign_messages_toggled(&self) -> Option<&ChatEventSignMessagesToggled> { if let ChatEventAction::ChatEventSignMessagesToggled(t) = self { return Some(t) } None }
  pub fn as_chat_event_slow_mode_delay_changed(&self) -> Option<&ChatEventSlowModeDelayChanged> { if let ChatEventAction::ChatEventSlowModeDelayChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_sticker_set_changed(&self) -> Option<&ChatEventStickerSetChanged> { if let ChatEventAction::ChatEventStickerSetChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_title_changed(&self) -> Option<&ChatEventTitleChanged> { if let ChatEventAction::ChatEventTitleChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_username_changed(&self) -> Option<&ChatEventUsernameChanged> { if let ChatEventAction::ChatEventUsernameChanged(t) = self { return Some(t) } None }
  pub fn as_chat_event_video_chat_created(&self) -> Option<&ChatEventVideoChatCreated> { if let ChatEventAction::ChatEventVideoChatCreated(t) = self { return Some(t) } None }
  pub fn as_chat_event_video_chat_discarded(&self) -> Option<&ChatEventVideoChatDiscarded> { if let ChatEventAction::ChatEventVideoChatDiscarded(t) = self { return Some(t) } None }
  pub fn as_chat_event_video_chat_mute_new_participants_toggled(&self) -> Option<&ChatEventVideoChatMuteNewParticipantsToggled> { if let ChatEventAction::ChatEventVideoChatMuteNewParticipantsToggled(t) = self { return Some(t) } None }
  pub fn as_chat_event_video_chat_participant_is_muted_toggled(&self) -> Option<&ChatEventVideoChatParticipantIsMutedToggled> { if let ChatEventAction::ChatEventVideoChatParticipantIsMutedToggled(t) = self { return Some(t) } None }
  pub fn as_chat_event_video_chat_participant_volume_level_changed(&self) -> Option<&ChatEventVideoChatParticipantVolumeLevelChanged> { if let ChatEventAction::ChatEventVideoChatParticipantVolumeLevelChanged(t) = self { return Some(t) } None }



  pub fn chat_event_description_changed<T: AsRef<ChatEventDescriptionChanged>>(t: T) -> Self { ChatEventAction::ChatEventDescriptionChanged(t.as_ref().clone()) }

  pub fn chat_event_invite_link_deleted<T: AsRef<ChatEventInviteLinkDeleted>>(t: T) -> Self { ChatEventAction::ChatEventInviteLinkDeleted(t.as_ref().clone()) }

  pub fn chat_event_invite_link_edited<T: AsRef<ChatEventInviteLinkEdited>>(t: T) -> Self { ChatEventAction::ChatEventInviteLinkEdited(t.as_ref().clone()) }

  pub fn chat_event_invite_link_revoked<T: AsRef<ChatEventInviteLinkRevoked>>(t: T) -> Self { ChatEventAction::ChatEventInviteLinkRevoked(t.as_ref().clone()) }

  pub fn chat_event_invites_toggled<T: AsRef<ChatEventInvitesToggled>>(t: T) -> Self { ChatEventAction::ChatEventInvitesToggled(t.as_ref().clone()) }

  pub fn chat_event_is_all_history_available_toggled<T: AsRef<ChatEventIsAllHistoryAvailableToggled>>(t: T) -> Self { ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t.as_ref().clone()) }

  pub fn chat_event_linked_chat_changed<T: AsRef<ChatEventLinkedChatChanged>>(t: T) -> Self { ChatEventAction::ChatEventLinkedChatChanged(t.as_ref().clone()) }

  pub fn chat_event_location_changed<T: AsRef<ChatEventLocationChanged>>(t: T) -> Self { ChatEventAction::ChatEventLocationChanged(t.as_ref().clone()) }

  pub fn chat_event_member_invited<T: AsRef<ChatEventMemberInvited>>(t: T) -> Self { ChatEventAction::ChatEventMemberInvited(t.as_ref().clone()) }

  pub fn chat_event_member_joined<T: AsRef<ChatEventMemberJoined>>(t: T) -> Self { ChatEventAction::ChatEventMemberJoined(t.as_ref().clone()) }

  pub fn chat_event_member_joined_by_invite_link<T: AsRef<ChatEventMemberJoinedByInviteLink>>(t: T) -> Self { ChatEventAction::ChatEventMemberJoinedByInviteLink(t.as_ref().clone()) }

  pub fn chat_event_member_joined_by_request<T: AsRef<ChatEventMemberJoinedByRequest>>(t: T) -> Self { ChatEventAction::ChatEventMemberJoinedByRequest(t.as_ref().clone()) }

  pub fn chat_event_member_left<T: AsRef<ChatEventMemberLeft>>(t: T) -> Self { ChatEventAction::ChatEventMemberLeft(t.as_ref().clone()) }

  pub fn chat_event_member_promoted<T: AsRef<ChatEventMemberPromoted>>(t: T) -> Self { ChatEventAction::ChatEventMemberPromoted(t.as_ref().clone()) }

  pub fn chat_event_member_restricted<T: AsRef<ChatEventMemberRestricted>>(t: T) -> Self { ChatEventAction::ChatEventMemberRestricted(t.as_ref().clone()) }

  pub fn chat_event_message_deleted<T: AsRef<ChatEventMessageDeleted>>(t: T) -> Self { ChatEventAction::ChatEventMessageDeleted(t.as_ref().clone()) }

  pub fn chat_event_message_edited<T: AsRef<ChatEventMessageEdited>>(t: T) -> Self { ChatEventAction::ChatEventMessageEdited(t.as_ref().clone()) }

  pub fn chat_event_message_pinned<T: AsRef<ChatEventMessagePinned>>(t: T) -> Self { ChatEventAction::ChatEventMessagePinned(t.as_ref().clone()) }

  pub fn chat_event_message_ttl_setting_changed<T: AsRef<ChatEventMessageTtlSettingChanged>>(t: T) -> Self { ChatEventAction::ChatEventMessageTtlSettingChanged(t.as_ref().clone()) }

  pub fn chat_event_message_unpinned<T: AsRef<ChatEventMessageUnpinned>>(t: T) -> Self { ChatEventAction::ChatEventMessageUnpinned(t.as_ref().clone()) }

  pub fn chat_event_permissions_changed<T: AsRef<ChatEventPermissionsChanged>>(t: T) -> Self { ChatEventAction::ChatEventPermissionsChanged(t.as_ref().clone()) }

  pub fn chat_event_photo_changed<T: AsRef<ChatEventPhotoChanged>>(t: T) -> Self { ChatEventAction::ChatEventPhotoChanged(t.as_ref().clone()) }

  pub fn chat_event_poll_stopped<T: AsRef<ChatEventPollStopped>>(t: T) -> Self { ChatEventAction::ChatEventPollStopped(t.as_ref().clone()) }

  pub fn chat_event_sign_messages_toggled<T: AsRef<ChatEventSignMessagesToggled>>(t: T) -> Self { ChatEventAction::ChatEventSignMessagesToggled(t.as_ref().clone()) }

  pub fn chat_event_slow_mode_delay_changed<T: AsRef<ChatEventSlowModeDelayChanged>>(t: T) -> Self { ChatEventAction::ChatEventSlowModeDelayChanged(t.as_ref().clone()) }

  pub fn chat_event_sticker_set_changed<T: AsRef<ChatEventStickerSetChanged>>(t: T) -> Self { ChatEventAction::ChatEventStickerSetChanged(t.as_ref().clone()) }

  pub fn chat_event_title_changed<T: AsRef<ChatEventTitleChanged>>(t: T) -> Self { ChatEventAction::ChatEventTitleChanged(t.as_ref().clone()) }

  pub fn chat_event_username_changed<T: AsRef<ChatEventUsernameChanged>>(t: T) -> Self { ChatEventAction::ChatEventUsernameChanged(t.as_ref().clone()) }

  pub fn chat_event_video_chat_created<T: AsRef<ChatEventVideoChatCreated>>(t: T) -> Self { ChatEventAction::ChatEventVideoChatCreated(t.as_ref().clone()) }

  pub fn chat_event_video_chat_discarded<T: AsRef<ChatEventVideoChatDiscarded>>(t: T) -> Self { ChatEventAction::ChatEventVideoChatDiscarded(t.as_ref().clone()) }

  pub fn chat_event_video_chat_mute_new_participants_toggled<T: AsRef<ChatEventVideoChatMuteNewParticipantsToggled>>(t: T) -> Self { ChatEventAction::ChatEventVideoChatMuteNewParticipantsToggled(t.as_ref().clone()) }

  pub fn chat_event_video_chat_participant_is_muted_toggled<T: AsRef<ChatEventVideoChatParticipantIsMutedToggled>>(t: T) -> Self { ChatEventAction::ChatEventVideoChatParticipantIsMutedToggled(t.as_ref().clone()) }

  pub fn chat_event_video_chat_participant_volume_level_changed<T: AsRef<ChatEventVideoChatParticipantVolumeLevelChanged>>(t: T) -> Self { ChatEventAction::ChatEventVideoChatParticipantVolumeLevelChanged(t.as_ref().clone()) }

}

impl AsRef<ChatEventAction> for ChatEventAction {
  fn as_ref(&self) -> &ChatEventAction { self }
}







/// The chat description was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventDescriptionChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous chat description
  old_description: String,
  /// New chat description
  new_description: String,
  
}

impl RObject for ChatEventDescriptionChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventDescriptionChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventDescriptionChanged {}



impl ChatEventDescriptionChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventDescriptionChangedBuilder {
    let mut inner = ChatEventDescriptionChanged::default();
    inner.td_name = "chatEventDescriptionChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventDescriptionChangedBuilder { inner }
  }

  pub fn old_description(&self) -> &String { &self.old_description }

  pub fn new_description(&self) -> &String { &self.new_description }

}

#[doc(hidden)]
pub struct RTDChatEventDescriptionChangedBuilder {
  inner: ChatEventDescriptionChanged
}

impl RTDChatEventDescriptionChangedBuilder {
  pub fn build(&self) -> ChatEventDescriptionChanged { self.inner.clone() }

   
  pub fn old_description<T: AsRef<str>>(&mut self, old_description: T) -> &mut Self {
    self.inner.old_description = old_description.as_ref().to_string();
    self
  }

   
  pub fn new_description<T: AsRef<str>>(&mut self, new_description: T) -> &mut Self {
    self.inner.new_description = new_description.as_ref().to_string();
    self
  }

}

impl AsRef<ChatEventDescriptionChanged> for ChatEventDescriptionChanged {
  fn as_ref(&self) -> &ChatEventDescriptionChanged { self }
}

impl AsRef<ChatEventDescriptionChanged> for RTDChatEventDescriptionChangedBuilder {
  fn as_ref(&self) -> &ChatEventDescriptionChanged { &self.inner }
}







/// A revoked chat invite link was deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventInviteLinkDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The invite link
  invite_link: ChatInviteLink,
  
}

impl RObject for ChatEventInviteLinkDeleted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventInviteLinkDeleted" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventInviteLinkDeleted {}



impl ChatEventInviteLinkDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventInviteLinkDeletedBuilder {
    let mut inner = ChatEventInviteLinkDeleted::default();
    inner.td_name = "chatEventInviteLinkDeleted".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventInviteLinkDeletedBuilder { inner }
  }

  pub fn invite_link(&self) -> &ChatInviteLink { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDChatEventInviteLinkDeletedBuilder {
  inner: ChatEventInviteLinkDeleted
}

impl RTDChatEventInviteLinkDeletedBuilder {
  pub fn build(&self) -> ChatEventInviteLinkDeleted { self.inner.clone() }

   
  pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventInviteLinkDeleted> for ChatEventInviteLinkDeleted {
  fn as_ref(&self) -> &ChatEventInviteLinkDeleted { self }
}

impl AsRef<ChatEventInviteLinkDeleted> for RTDChatEventInviteLinkDeletedBuilder {
  fn as_ref(&self) -> &ChatEventInviteLinkDeleted { &self.inner }
}







/// A chat invite link was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventInviteLinkEdited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous information about the invite link
  old_invite_link: ChatInviteLink,
  /// New information about the invite link
  new_invite_link: ChatInviteLink,
  
}

impl RObject for ChatEventInviteLinkEdited {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventInviteLinkEdited" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventInviteLinkEdited {}



impl ChatEventInviteLinkEdited {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventInviteLinkEditedBuilder {
    let mut inner = ChatEventInviteLinkEdited::default();
    inner.td_name = "chatEventInviteLinkEdited".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventInviteLinkEditedBuilder { inner }
  }

  pub fn old_invite_link(&self) -> &ChatInviteLink { &self.old_invite_link }

  pub fn new_invite_link(&self) -> &ChatInviteLink { &self.new_invite_link }

}

#[doc(hidden)]
pub struct RTDChatEventInviteLinkEditedBuilder {
  inner: ChatEventInviteLinkEdited
}

impl RTDChatEventInviteLinkEditedBuilder {
  pub fn build(&self) -> ChatEventInviteLinkEdited { self.inner.clone() }

   
  pub fn old_invite_link<T: AsRef<ChatInviteLink>>(&mut self, old_invite_link: T) -> &mut Self {
    self.inner.old_invite_link = old_invite_link.as_ref().clone();
    self
  }

   
  pub fn new_invite_link<T: AsRef<ChatInviteLink>>(&mut self, new_invite_link: T) -> &mut Self {
    self.inner.new_invite_link = new_invite_link.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventInviteLinkEdited> for ChatEventInviteLinkEdited {
  fn as_ref(&self) -> &ChatEventInviteLinkEdited { self }
}

impl AsRef<ChatEventInviteLinkEdited> for RTDChatEventInviteLinkEditedBuilder {
  fn as_ref(&self) -> &ChatEventInviteLinkEdited { &self.inner }
}







/// A chat invite link was revoked
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventInviteLinkRevoked {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The invite link
  invite_link: ChatInviteLink,
  
}

impl RObject for ChatEventInviteLinkRevoked {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventInviteLinkRevoked" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventInviteLinkRevoked {}



impl ChatEventInviteLinkRevoked {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventInviteLinkRevokedBuilder {
    let mut inner = ChatEventInviteLinkRevoked::default();
    inner.td_name = "chatEventInviteLinkRevoked".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventInviteLinkRevokedBuilder { inner }
  }

  pub fn invite_link(&self) -> &ChatInviteLink { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDChatEventInviteLinkRevokedBuilder {
  inner: ChatEventInviteLinkRevoked
}

impl RTDChatEventInviteLinkRevokedBuilder {
  pub fn build(&self) -> ChatEventInviteLinkRevoked { self.inner.clone() }

   
  pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventInviteLinkRevoked> for ChatEventInviteLinkRevoked {
  fn as_ref(&self) -> &ChatEventInviteLinkRevoked { self }
}

impl AsRef<ChatEventInviteLinkRevoked> for RTDChatEventInviteLinkRevokedBuilder {
  fn as_ref(&self) -> &ChatEventInviteLinkRevoked { &self.inner }
}







/// The can_invite_users permission of a supergroup chat was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventInvitesToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New value of can_invite_users permission
  can_invite_users: bool,
  
}

impl RObject for ChatEventInvitesToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventInvitesToggled" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventInvitesToggled {}



impl ChatEventInvitesToggled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventInvitesToggledBuilder {
    let mut inner = ChatEventInvitesToggled::default();
    inner.td_name = "chatEventInvitesToggled".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventInvitesToggledBuilder { inner }
  }

  pub fn can_invite_users(&self) -> bool { self.can_invite_users }

}

#[doc(hidden)]
pub struct RTDChatEventInvitesToggledBuilder {
  inner: ChatEventInvitesToggled
}

impl RTDChatEventInvitesToggledBuilder {
  pub fn build(&self) -> ChatEventInvitesToggled { self.inner.clone() }

   
  pub fn can_invite_users(&mut self, can_invite_users: bool) -> &mut Self {
    self.inner.can_invite_users = can_invite_users;
    self
  }

}

impl AsRef<ChatEventInvitesToggled> for ChatEventInvitesToggled {
  fn as_ref(&self) -> &ChatEventInvitesToggled { self }
}

impl AsRef<ChatEventInvitesToggled> for RTDChatEventInvitesToggledBuilder {
  fn as_ref(&self) -> &ChatEventInvitesToggled { &self.inner }
}







/// The is_all_history_available setting of a supergroup was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventIsAllHistoryAvailableToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New value of is_all_history_available
  is_all_history_available: bool,
  
}

impl RObject for ChatEventIsAllHistoryAvailableToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventIsAllHistoryAvailableToggled" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventIsAllHistoryAvailableToggled {}



impl ChatEventIsAllHistoryAvailableToggled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventIsAllHistoryAvailableToggledBuilder {
    let mut inner = ChatEventIsAllHistoryAvailableToggled::default();
    inner.td_name = "chatEventIsAllHistoryAvailableToggled".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventIsAllHistoryAvailableToggledBuilder { inner }
  }

  pub fn is_all_history_available(&self) -> bool { self.is_all_history_available }

}

#[doc(hidden)]
pub struct RTDChatEventIsAllHistoryAvailableToggledBuilder {
  inner: ChatEventIsAllHistoryAvailableToggled
}

impl RTDChatEventIsAllHistoryAvailableToggledBuilder {
  pub fn build(&self) -> ChatEventIsAllHistoryAvailableToggled { self.inner.clone() }

   
  pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self {
    self.inner.is_all_history_available = is_all_history_available;
    self
  }

}

impl AsRef<ChatEventIsAllHistoryAvailableToggled> for ChatEventIsAllHistoryAvailableToggled {
  fn as_ref(&self) -> &ChatEventIsAllHistoryAvailableToggled { self }
}

impl AsRef<ChatEventIsAllHistoryAvailableToggled> for RTDChatEventIsAllHistoryAvailableToggledBuilder {
  fn as_ref(&self) -> &ChatEventIsAllHistoryAvailableToggled { &self.inner }
}







/// The linked chat of a supergroup was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventLinkedChatChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous supergroup linked chat identifier
  old_linked_chat_id: i64,
  /// New supergroup linked chat identifier
  new_linked_chat_id: i64,
  
}

impl RObject for ChatEventLinkedChatChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventLinkedChatChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventLinkedChatChanged {}



impl ChatEventLinkedChatChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventLinkedChatChangedBuilder {
    let mut inner = ChatEventLinkedChatChanged::default();
    inner.td_name = "chatEventLinkedChatChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventLinkedChatChangedBuilder { inner }
  }

  pub fn old_linked_chat_id(&self) -> i64 { self.old_linked_chat_id }

  pub fn new_linked_chat_id(&self) -> i64 { self.new_linked_chat_id }

}

#[doc(hidden)]
pub struct RTDChatEventLinkedChatChangedBuilder {
  inner: ChatEventLinkedChatChanged
}

impl RTDChatEventLinkedChatChangedBuilder {
  pub fn build(&self) -> ChatEventLinkedChatChanged { self.inner.clone() }

   
  pub fn old_linked_chat_id(&mut self, old_linked_chat_id: i64) -> &mut Self {
    self.inner.old_linked_chat_id = old_linked_chat_id;
    self
  }

   
  pub fn new_linked_chat_id(&mut self, new_linked_chat_id: i64) -> &mut Self {
    self.inner.new_linked_chat_id = new_linked_chat_id;
    self
  }

}

impl AsRef<ChatEventLinkedChatChanged> for ChatEventLinkedChatChanged {
  fn as_ref(&self) -> &ChatEventLinkedChatChanged { self }
}

impl AsRef<ChatEventLinkedChatChanged> for RTDChatEventLinkedChatChangedBuilder {
  fn as_ref(&self) -> &ChatEventLinkedChatChanged { &self.inner }
}







/// The supergroup location was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventLocationChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous location; may be null
  old_location: Option<ChatLocation>,
  /// New location; may be null
  new_location: Option<ChatLocation>,
  
}

impl RObject for ChatEventLocationChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventLocationChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventLocationChanged {}



impl ChatEventLocationChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventLocationChangedBuilder {
    let mut inner = ChatEventLocationChanged::default();
    inner.td_name = "chatEventLocationChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventLocationChangedBuilder { inner }
  }

  pub fn old_location(&self) -> &Option<ChatLocation> { &self.old_location }

  pub fn new_location(&self) -> &Option<ChatLocation> { &self.new_location }

}

#[doc(hidden)]
pub struct RTDChatEventLocationChangedBuilder {
  inner: ChatEventLocationChanged
}

impl RTDChatEventLocationChangedBuilder {
  pub fn build(&self) -> ChatEventLocationChanged { self.inner.clone() }

   
  pub fn old_location<T: AsRef<ChatLocation>>(&mut self, old_location: T) -> &mut Self {
    self.inner.old_location = Some(old_location.as_ref().clone());
    self
  }

   
  pub fn new_location<T: AsRef<ChatLocation>>(&mut self, new_location: T) -> &mut Self {
    self.inner.new_location = Some(new_location.as_ref().clone());
    self
  }

}

impl AsRef<ChatEventLocationChanged> for ChatEventLocationChanged {
  fn as_ref(&self) -> &ChatEventLocationChanged { self }
}

impl AsRef<ChatEventLocationChanged> for RTDChatEventLocationChangedBuilder {
  fn as_ref(&self) -> &ChatEventLocationChanged { &self.inner }
}







/// A new chat member was invited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberInvited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New member user identifier
  user_id: i64,
  /// New member status
  status: ChatMemberStatus,
  
}

impl RObject for ChatEventMemberInvited {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberInvited" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberInvited {}



impl ChatEventMemberInvited {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberInvitedBuilder {
    let mut inner = ChatEventMemberInvited::default();
    inner.td_name = "chatEventMemberInvited".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMemberInvitedBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn status(&self) -> &ChatMemberStatus { &self.status }

}

#[doc(hidden)]
pub struct RTDChatEventMemberInvitedBuilder {
  inner: ChatEventMemberInvited
}

impl RTDChatEventMemberInvitedBuilder {
  pub fn build(&self) -> ChatEventMemberInvited { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMemberInvited> for ChatEventMemberInvited {
  fn as_ref(&self) -> &ChatEventMemberInvited { self }
}

impl AsRef<ChatEventMemberInvited> for RTDChatEventMemberInvitedBuilder {
  fn as_ref(&self) -> &ChatEventMemberInvited { &self.inner }
}







/// A new member joined the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberJoined {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatEventMemberJoined {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberJoined" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberJoined {}



impl ChatEventMemberJoined {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberJoinedBuilder {
    let mut inner = ChatEventMemberJoined::default();
    inner.td_name = "chatEventMemberJoined".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMemberJoinedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatEventMemberJoinedBuilder {
  inner: ChatEventMemberJoined
}

impl RTDChatEventMemberJoinedBuilder {
  pub fn build(&self) -> ChatEventMemberJoined { self.inner.clone() }

}

impl AsRef<ChatEventMemberJoined> for ChatEventMemberJoined {
  fn as_ref(&self) -> &ChatEventMemberJoined { self }
}

impl AsRef<ChatEventMemberJoined> for RTDChatEventMemberJoinedBuilder {
  fn as_ref(&self) -> &ChatEventMemberJoined { &self.inner }
}







/// A new member joined the chat by an invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberJoinedByInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Invite link used to join the chat
  invite_link: ChatInviteLink,
  
}

impl RObject for ChatEventMemberJoinedByInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberJoinedByInviteLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberJoinedByInviteLink {}



impl ChatEventMemberJoinedByInviteLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberJoinedByInviteLinkBuilder {
    let mut inner = ChatEventMemberJoinedByInviteLink::default();
    inner.td_name = "chatEventMemberJoinedByInviteLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMemberJoinedByInviteLinkBuilder { inner }
  }

  pub fn invite_link(&self) -> &ChatInviteLink { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDChatEventMemberJoinedByInviteLinkBuilder {
  inner: ChatEventMemberJoinedByInviteLink
}

impl RTDChatEventMemberJoinedByInviteLinkBuilder {
  pub fn build(&self) -> ChatEventMemberJoinedByInviteLink { self.inner.clone() }

   
  pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMemberJoinedByInviteLink> for ChatEventMemberJoinedByInviteLink {
  fn as_ref(&self) -> &ChatEventMemberJoinedByInviteLink { self }
}

impl AsRef<ChatEventMemberJoinedByInviteLink> for RTDChatEventMemberJoinedByInviteLinkBuilder {
  fn as_ref(&self) -> &ChatEventMemberJoinedByInviteLink { &self.inner }
}







/// A new member was accepted to the chat by an administrator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberJoinedByRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the chat administrator, approved user join request
  approver_user_id: i64,
  /// Invite link used to join the chat; may be null
  invite_link: Option<ChatInviteLink>,
  
}

impl RObject for ChatEventMemberJoinedByRequest {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberJoinedByRequest" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberJoinedByRequest {}



impl ChatEventMemberJoinedByRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberJoinedByRequestBuilder {
    let mut inner = ChatEventMemberJoinedByRequest::default();
    inner.td_name = "chatEventMemberJoinedByRequest".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMemberJoinedByRequestBuilder { inner }
  }

  pub fn approver_user_id(&self) -> i64 { self.approver_user_id }

  pub fn invite_link(&self) -> &Option<ChatInviteLink> { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDChatEventMemberJoinedByRequestBuilder {
  inner: ChatEventMemberJoinedByRequest
}

impl RTDChatEventMemberJoinedByRequestBuilder {
  pub fn build(&self) -> ChatEventMemberJoinedByRequest { self.inner.clone() }

   
  pub fn approver_user_id(&mut self, approver_user_id: i64) -> &mut Self {
    self.inner.approver_user_id = approver_user_id;
    self
  }

   
  pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = Some(invite_link.as_ref().clone());
    self
  }

}

impl AsRef<ChatEventMemberJoinedByRequest> for ChatEventMemberJoinedByRequest {
  fn as_ref(&self) -> &ChatEventMemberJoinedByRequest { self }
}

impl AsRef<ChatEventMemberJoinedByRequest> for RTDChatEventMemberJoinedByRequestBuilder {
  fn as_ref(&self) -> &ChatEventMemberJoinedByRequest { &self.inner }
}







/// A member left the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberLeft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatEventMemberLeft {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberLeft" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberLeft {}



impl ChatEventMemberLeft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberLeftBuilder {
    let mut inner = ChatEventMemberLeft::default();
    inner.td_name = "chatEventMemberLeft".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMemberLeftBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatEventMemberLeftBuilder {
  inner: ChatEventMemberLeft
}

impl RTDChatEventMemberLeftBuilder {
  pub fn build(&self) -> ChatEventMemberLeft { self.inner.clone() }

}

impl AsRef<ChatEventMemberLeft> for ChatEventMemberLeft {
  fn as_ref(&self) -> &ChatEventMemberLeft { self }
}

impl AsRef<ChatEventMemberLeft> for RTDChatEventMemberLeftBuilder {
  fn as_ref(&self) -> &ChatEventMemberLeft { &self.inner }
}







/// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberPromoted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Affected chat member user identifier
  user_id: i64,
  /// Previous status of the chat member
  old_status: ChatMemberStatus,
  /// New status of the chat member
  new_status: ChatMemberStatus,
  
}

impl RObject for ChatEventMemberPromoted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberPromoted" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberPromoted {}



impl ChatEventMemberPromoted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberPromotedBuilder {
    let mut inner = ChatEventMemberPromoted::default();
    inner.td_name = "chatEventMemberPromoted".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMemberPromotedBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn old_status(&self) -> &ChatMemberStatus { &self.old_status }

  pub fn new_status(&self) -> &ChatMemberStatus { &self.new_status }

}

#[doc(hidden)]
pub struct RTDChatEventMemberPromotedBuilder {
  inner: ChatEventMemberPromoted
}

impl RTDChatEventMemberPromotedBuilder {
  pub fn build(&self) -> ChatEventMemberPromoted { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn old_status<T: AsRef<ChatMemberStatus>>(&mut self, old_status: T) -> &mut Self {
    self.inner.old_status = old_status.as_ref().clone();
    self
  }

   
  pub fn new_status<T: AsRef<ChatMemberStatus>>(&mut self, new_status: T) -> &mut Self {
    self.inner.new_status = new_status.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMemberPromoted> for ChatEventMemberPromoted {
  fn as_ref(&self) -> &ChatEventMemberPromoted { self }
}

impl AsRef<ChatEventMemberPromoted> for RTDChatEventMemberPromotedBuilder {
  fn as_ref(&self) -> &ChatEventMemberPromoted { &self.inner }
}







/// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Affected chat member identifier
  member_id: MessageSender,
  /// Previous status of the chat member
  old_status: ChatMemberStatus,
  /// New status of the chat member
  new_status: ChatMemberStatus,
  
}

impl RObject for ChatEventMemberRestricted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMemberRestricted" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMemberRestricted {}



impl ChatEventMemberRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMemberRestrictedBuilder {
    let mut inner = ChatEventMemberRestricted::default();
    inner.td_name = "chatEventMemberRestricted".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMemberRestrictedBuilder { inner }
  }

  pub fn member_id(&self) -> &MessageSender { &self.member_id }

  pub fn old_status(&self) -> &ChatMemberStatus { &self.old_status }

  pub fn new_status(&self) -> &ChatMemberStatus { &self.new_status }

}

#[doc(hidden)]
pub struct RTDChatEventMemberRestrictedBuilder {
  inner: ChatEventMemberRestricted
}

impl RTDChatEventMemberRestrictedBuilder {
  pub fn build(&self) -> ChatEventMemberRestricted { self.inner.clone() }

   
  pub fn member_id<T: AsRef<MessageSender>>(&mut self, member_id: T) -> &mut Self {
    self.inner.member_id = member_id.as_ref().clone();
    self
  }

   
  pub fn old_status<T: AsRef<ChatMemberStatus>>(&mut self, old_status: T) -> &mut Self {
    self.inner.old_status = old_status.as_ref().clone();
    self
  }

   
  pub fn new_status<T: AsRef<ChatMemberStatus>>(&mut self, new_status: T) -> &mut Self {
    self.inner.new_status = new_status.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMemberRestricted> for ChatEventMemberRestricted {
  fn as_ref(&self) -> &ChatEventMemberRestricted { self }
}

impl AsRef<ChatEventMemberRestricted> for RTDChatEventMemberRestrictedBuilder {
  fn as_ref(&self) -> &ChatEventMemberRestricted { &self.inner }
}







/// A message was deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Deleted message
  message: Message,
  
}

impl RObject for ChatEventMessageDeleted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageDeleted" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMessageDeleted {}



impl ChatEventMessageDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMessageDeletedBuilder {
    let mut inner = ChatEventMessageDeleted::default();
    inner.td_name = "chatEventMessageDeleted".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMessageDeletedBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

}

#[doc(hidden)]
pub struct RTDChatEventMessageDeletedBuilder {
  inner: ChatEventMessageDeleted
}

impl RTDChatEventMessageDeletedBuilder {
  pub fn build(&self) -> ChatEventMessageDeleted { self.inner.clone() }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMessageDeleted> for ChatEventMessageDeleted {
  fn as_ref(&self) -> &ChatEventMessageDeleted { self }
}

impl AsRef<ChatEventMessageDeleted> for RTDChatEventMessageDeletedBuilder {
  fn as_ref(&self) -> &ChatEventMessageDeleted { &self.inner }
}







/// A message was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageEdited {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The original message before the edit
  old_message: Message,
  /// The message after it was edited
  new_message: Message,
  
}

impl RObject for ChatEventMessageEdited {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageEdited" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMessageEdited {}



impl ChatEventMessageEdited {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMessageEditedBuilder {
    let mut inner = ChatEventMessageEdited::default();
    inner.td_name = "chatEventMessageEdited".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMessageEditedBuilder { inner }
  }

  pub fn old_message(&self) -> &Message { &self.old_message }

  pub fn new_message(&self) -> &Message { &self.new_message }

}

#[doc(hidden)]
pub struct RTDChatEventMessageEditedBuilder {
  inner: ChatEventMessageEdited
}

impl RTDChatEventMessageEditedBuilder {
  pub fn build(&self) -> ChatEventMessageEdited { self.inner.clone() }

   
  pub fn old_message<T: AsRef<Message>>(&mut self, old_message: T) -> &mut Self {
    self.inner.old_message = old_message.as_ref().clone();
    self
  }

   
  pub fn new_message<T: AsRef<Message>>(&mut self, new_message: T) -> &mut Self {
    self.inner.new_message = new_message.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMessageEdited> for ChatEventMessageEdited {
  fn as_ref(&self) -> &ChatEventMessageEdited { self }
}

impl AsRef<ChatEventMessageEdited> for RTDChatEventMessageEditedBuilder {
  fn as_ref(&self) -> &ChatEventMessageEdited { &self.inner }
}







/// A message was pinned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessagePinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pinned message
  message: Message,
  
}

impl RObject for ChatEventMessagePinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessagePinned" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMessagePinned {}



impl ChatEventMessagePinned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMessagePinnedBuilder {
    let mut inner = ChatEventMessagePinned::default();
    inner.td_name = "chatEventMessagePinned".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMessagePinnedBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

}

#[doc(hidden)]
pub struct RTDChatEventMessagePinnedBuilder {
  inner: ChatEventMessagePinned
}

impl RTDChatEventMessagePinnedBuilder {
  pub fn build(&self) -> ChatEventMessagePinned { self.inner.clone() }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMessagePinned> for ChatEventMessagePinned {
  fn as_ref(&self) -> &ChatEventMessagePinned { self }
}

impl AsRef<ChatEventMessagePinned> for RTDChatEventMessagePinnedBuilder {
  fn as_ref(&self) -> &ChatEventMessagePinned { &self.inner }
}







/// The message TTL setting was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageTtlSettingChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous value of message_ttl_setting
  old_message_ttl_setting: i64,
  /// New value of message_ttl_setting
  new_message_ttl_setting: i64,
  
}

impl RObject for ChatEventMessageTtlSettingChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageTtlSettingChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMessageTtlSettingChanged {}



impl ChatEventMessageTtlSettingChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMessageTtlSettingChangedBuilder {
    let mut inner = ChatEventMessageTtlSettingChanged::default();
    inner.td_name = "chatEventMessageTtlSettingChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMessageTtlSettingChangedBuilder { inner }
  }

  pub fn old_message_ttl_setting(&self) -> i64 { self.old_message_ttl_setting }

  pub fn new_message_ttl_setting(&self) -> i64 { self.new_message_ttl_setting }

}

#[doc(hidden)]
pub struct RTDChatEventMessageTtlSettingChangedBuilder {
  inner: ChatEventMessageTtlSettingChanged
}

impl RTDChatEventMessageTtlSettingChangedBuilder {
  pub fn build(&self) -> ChatEventMessageTtlSettingChanged { self.inner.clone() }

   
  pub fn old_message_ttl_setting(&mut self, old_message_ttl_setting: i64) -> &mut Self {
    self.inner.old_message_ttl_setting = old_message_ttl_setting;
    self
  }

   
  pub fn new_message_ttl_setting(&mut self, new_message_ttl_setting: i64) -> &mut Self {
    self.inner.new_message_ttl_setting = new_message_ttl_setting;
    self
  }

}

impl AsRef<ChatEventMessageTtlSettingChanged> for ChatEventMessageTtlSettingChanged {
  fn as_ref(&self) -> &ChatEventMessageTtlSettingChanged { self }
}

impl AsRef<ChatEventMessageTtlSettingChanged> for RTDChatEventMessageTtlSettingChangedBuilder {
  fn as_ref(&self) -> &ChatEventMessageTtlSettingChanged { &self.inner }
}







/// A message was unpinned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageUnpinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unpinned message
  message: Message,
  
}

impl RObject for ChatEventMessageUnpinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageUnpinned" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventMessageUnpinned {}



impl ChatEventMessageUnpinned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventMessageUnpinnedBuilder {
    let mut inner = ChatEventMessageUnpinned::default();
    inner.td_name = "chatEventMessageUnpinned".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventMessageUnpinnedBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

}

#[doc(hidden)]
pub struct RTDChatEventMessageUnpinnedBuilder {
  inner: ChatEventMessageUnpinned
}

impl RTDChatEventMessageUnpinnedBuilder {
  pub fn build(&self) -> ChatEventMessageUnpinned { self.inner.clone() }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventMessageUnpinned> for ChatEventMessageUnpinned {
  fn as_ref(&self) -> &ChatEventMessageUnpinned { self }
}

impl AsRef<ChatEventMessageUnpinned> for RTDChatEventMessageUnpinnedBuilder {
  fn as_ref(&self) -> &ChatEventMessageUnpinned { &self.inner }
}







/// The chat permissions was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventPermissionsChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous chat permissions
  old_permissions: ChatPermissions,
  /// New chat permissions
  new_permissions: ChatPermissions,
  
}

impl RObject for ChatEventPermissionsChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventPermissionsChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventPermissionsChanged {}



impl ChatEventPermissionsChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventPermissionsChangedBuilder {
    let mut inner = ChatEventPermissionsChanged::default();
    inner.td_name = "chatEventPermissionsChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventPermissionsChangedBuilder { inner }
  }

  pub fn old_permissions(&self) -> &ChatPermissions { &self.old_permissions }

  pub fn new_permissions(&self) -> &ChatPermissions { &self.new_permissions }

}

#[doc(hidden)]
pub struct RTDChatEventPermissionsChangedBuilder {
  inner: ChatEventPermissionsChanged
}

impl RTDChatEventPermissionsChangedBuilder {
  pub fn build(&self) -> ChatEventPermissionsChanged { self.inner.clone() }

   
  pub fn old_permissions<T: AsRef<ChatPermissions>>(&mut self, old_permissions: T) -> &mut Self {
    self.inner.old_permissions = old_permissions.as_ref().clone();
    self
  }

   
  pub fn new_permissions<T: AsRef<ChatPermissions>>(&mut self, new_permissions: T) -> &mut Self {
    self.inner.new_permissions = new_permissions.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventPermissionsChanged> for ChatEventPermissionsChanged {
  fn as_ref(&self) -> &ChatEventPermissionsChanged { self }
}

impl AsRef<ChatEventPermissionsChanged> for RTDChatEventPermissionsChangedBuilder {
  fn as_ref(&self) -> &ChatEventPermissionsChanged { &self.inner }
}







/// The chat photo was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventPhotoChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous chat photo value; may be null
  old_photo: Option<ChatPhoto>,
  /// New chat photo value; may be null
  new_photo: Option<ChatPhoto>,
  
}

impl RObject for ChatEventPhotoChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventPhotoChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventPhotoChanged {}



impl ChatEventPhotoChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventPhotoChangedBuilder {
    let mut inner = ChatEventPhotoChanged::default();
    inner.td_name = "chatEventPhotoChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventPhotoChangedBuilder { inner }
  }

  pub fn old_photo(&self) -> &Option<ChatPhoto> { &self.old_photo }

  pub fn new_photo(&self) -> &Option<ChatPhoto> { &self.new_photo }

}

#[doc(hidden)]
pub struct RTDChatEventPhotoChangedBuilder {
  inner: ChatEventPhotoChanged
}

impl RTDChatEventPhotoChangedBuilder {
  pub fn build(&self) -> ChatEventPhotoChanged { self.inner.clone() }

   
  pub fn old_photo<T: AsRef<ChatPhoto>>(&mut self, old_photo: T) -> &mut Self {
    self.inner.old_photo = Some(old_photo.as_ref().clone());
    self
  }

   
  pub fn new_photo<T: AsRef<ChatPhoto>>(&mut self, new_photo: T) -> &mut Self {
    self.inner.new_photo = Some(new_photo.as_ref().clone());
    self
  }

}

impl AsRef<ChatEventPhotoChanged> for ChatEventPhotoChanged {
  fn as_ref(&self) -> &ChatEventPhotoChanged { self }
}

impl AsRef<ChatEventPhotoChanged> for RTDChatEventPhotoChangedBuilder {
  fn as_ref(&self) -> &ChatEventPhotoChanged { &self.inner }
}







/// A poll in a message was stopped
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventPollStopped {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The message with the poll
  message: Message,
  
}

impl RObject for ChatEventPollStopped {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventPollStopped" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventPollStopped {}



impl ChatEventPollStopped {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventPollStoppedBuilder {
    let mut inner = ChatEventPollStopped::default();
    inner.td_name = "chatEventPollStopped".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventPollStoppedBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

}

#[doc(hidden)]
pub struct RTDChatEventPollStoppedBuilder {
  inner: ChatEventPollStopped
}

impl RTDChatEventPollStoppedBuilder {
  pub fn build(&self) -> ChatEventPollStopped { self.inner.clone() }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

}

impl AsRef<ChatEventPollStopped> for ChatEventPollStopped {
  fn as_ref(&self) -> &ChatEventPollStopped { self }
}

impl AsRef<ChatEventPollStopped> for RTDChatEventPollStoppedBuilder {
  fn as_ref(&self) -> &ChatEventPollStopped { &self.inner }
}







/// The sign_messages setting of a channel was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventSignMessagesToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New value of sign_messages
  sign_messages: bool,
  
}

impl RObject for ChatEventSignMessagesToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventSignMessagesToggled" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventSignMessagesToggled {}



impl ChatEventSignMessagesToggled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventSignMessagesToggledBuilder {
    let mut inner = ChatEventSignMessagesToggled::default();
    inner.td_name = "chatEventSignMessagesToggled".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventSignMessagesToggledBuilder { inner }
  }

  pub fn sign_messages(&self) -> bool { self.sign_messages }

}

#[doc(hidden)]
pub struct RTDChatEventSignMessagesToggledBuilder {
  inner: ChatEventSignMessagesToggled
}

impl RTDChatEventSignMessagesToggledBuilder {
  pub fn build(&self) -> ChatEventSignMessagesToggled { self.inner.clone() }

   
  pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self {
    self.inner.sign_messages = sign_messages;
    self
  }

}

impl AsRef<ChatEventSignMessagesToggled> for ChatEventSignMessagesToggled {
  fn as_ref(&self) -> &ChatEventSignMessagesToggled { self }
}

impl AsRef<ChatEventSignMessagesToggled> for RTDChatEventSignMessagesToggledBuilder {
  fn as_ref(&self) -> &ChatEventSignMessagesToggled { &self.inner }
}







/// The slow_mode_delay setting of a supergroup was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventSlowModeDelayChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous value of slow_mode_delay, in seconds
  old_slow_mode_delay: i64,
  /// New value of slow_mode_delay, in seconds
  new_slow_mode_delay: i64,
  
}

impl RObject for ChatEventSlowModeDelayChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventSlowModeDelayChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventSlowModeDelayChanged {}



impl ChatEventSlowModeDelayChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventSlowModeDelayChangedBuilder {
    let mut inner = ChatEventSlowModeDelayChanged::default();
    inner.td_name = "chatEventSlowModeDelayChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventSlowModeDelayChangedBuilder { inner }
  }

  pub fn old_slow_mode_delay(&self) -> i64 { self.old_slow_mode_delay }

  pub fn new_slow_mode_delay(&self) -> i64 { self.new_slow_mode_delay }

}

#[doc(hidden)]
pub struct RTDChatEventSlowModeDelayChangedBuilder {
  inner: ChatEventSlowModeDelayChanged
}

impl RTDChatEventSlowModeDelayChangedBuilder {
  pub fn build(&self) -> ChatEventSlowModeDelayChanged { self.inner.clone() }

   
  pub fn old_slow_mode_delay(&mut self, old_slow_mode_delay: i64) -> &mut Self {
    self.inner.old_slow_mode_delay = old_slow_mode_delay;
    self
  }

   
  pub fn new_slow_mode_delay(&mut self, new_slow_mode_delay: i64) -> &mut Self {
    self.inner.new_slow_mode_delay = new_slow_mode_delay;
    self
  }

}

impl AsRef<ChatEventSlowModeDelayChanged> for ChatEventSlowModeDelayChanged {
  fn as_ref(&self) -> &ChatEventSlowModeDelayChanged { self }
}

impl AsRef<ChatEventSlowModeDelayChanged> for RTDChatEventSlowModeDelayChangedBuilder {
  fn as_ref(&self) -> &ChatEventSlowModeDelayChanged { &self.inner }
}







/// The supergroup sticker set was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventStickerSetChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous identifier of the chat sticker set; 0 if none
  old_sticker_set_id: isize,
  /// New identifier of the chat sticker set; 0 if none
  new_sticker_set_id: isize,
  
}

impl RObject for ChatEventStickerSetChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventStickerSetChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventStickerSetChanged {}



impl ChatEventStickerSetChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventStickerSetChangedBuilder {
    let mut inner = ChatEventStickerSetChanged::default();
    inner.td_name = "chatEventStickerSetChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventStickerSetChangedBuilder { inner }
  }

  pub fn old_sticker_set_id(&self) -> isize { self.old_sticker_set_id }

  pub fn new_sticker_set_id(&self) -> isize { self.new_sticker_set_id }

}

#[doc(hidden)]
pub struct RTDChatEventStickerSetChangedBuilder {
  inner: ChatEventStickerSetChanged
}

impl RTDChatEventStickerSetChangedBuilder {
  pub fn build(&self) -> ChatEventStickerSetChanged { self.inner.clone() }

   
  pub fn old_sticker_set_id(&mut self, old_sticker_set_id: isize) -> &mut Self {
    self.inner.old_sticker_set_id = old_sticker_set_id;
    self
  }

   
  pub fn new_sticker_set_id(&mut self, new_sticker_set_id: isize) -> &mut Self {
    self.inner.new_sticker_set_id = new_sticker_set_id;
    self
  }

}

impl AsRef<ChatEventStickerSetChanged> for ChatEventStickerSetChanged {
  fn as_ref(&self) -> &ChatEventStickerSetChanged { self }
}

impl AsRef<ChatEventStickerSetChanged> for RTDChatEventStickerSetChangedBuilder {
  fn as_ref(&self) -> &ChatEventStickerSetChanged { &self.inner }
}







/// The chat title was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventTitleChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous chat title
  old_title: String,
  /// New chat title
  new_title: String,
  
}

impl RObject for ChatEventTitleChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventTitleChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventTitleChanged {}



impl ChatEventTitleChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventTitleChangedBuilder {
    let mut inner = ChatEventTitleChanged::default();
    inner.td_name = "chatEventTitleChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventTitleChangedBuilder { inner }
  }

  pub fn old_title(&self) -> &String { &self.old_title }

  pub fn new_title(&self) -> &String { &self.new_title }

}

#[doc(hidden)]
pub struct RTDChatEventTitleChangedBuilder {
  inner: ChatEventTitleChanged
}

impl RTDChatEventTitleChangedBuilder {
  pub fn build(&self) -> ChatEventTitleChanged { self.inner.clone() }

   
  pub fn old_title<T: AsRef<str>>(&mut self, old_title: T) -> &mut Self {
    self.inner.old_title = old_title.as_ref().to_string();
    self
  }

   
  pub fn new_title<T: AsRef<str>>(&mut self, new_title: T) -> &mut Self {
    self.inner.new_title = new_title.as_ref().to_string();
    self
  }

}

impl AsRef<ChatEventTitleChanged> for ChatEventTitleChanged {
  fn as_ref(&self) -> &ChatEventTitleChanged { self }
}

impl AsRef<ChatEventTitleChanged> for RTDChatEventTitleChangedBuilder {
  fn as_ref(&self) -> &ChatEventTitleChanged { &self.inner }
}







/// The chat username was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventUsernameChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Previous chat username
  old_username: String,
  /// New chat username
  new_username: String,
  
}

impl RObject for ChatEventUsernameChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventUsernameChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventUsernameChanged {}



impl ChatEventUsernameChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventUsernameChangedBuilder {
    let mut inner = ChatEventUsernameChanged::default();
    inner.td_name = "chatEventUsernameChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventUsernameChangedBuilder { inner }
  }

  pub fn old_username(&self) -> &String { &self.old_username }

  pub fn new_username(&self) -> &String { &self.new_username }

}

#[doc(hidden)]
pub struct RTDChatEventUsernameChangedBuilder {
  inner: ChatEventUsernameChanged
}

impl RTDChatEventUsernameChangedBuilder {
  pub fn build(&self) -> ChatEventUsernameChanged { self.inner.clone() }

   
  pub fn old_username<T: AsRef<str>>(&mut self, old_username: T) -> &mut Self {
    self.inner.old_username = old_username.as_ref().to_string();
    self
  }

   
  pub fn new_username<T: AsRef<str>>(&mut self, new_username: T) -> &mut Self {
    self.inner.new_username = new_username.as_ref().to_string();
    self
  }

}

impl AsRef<ChatEventUsernameChanged> for ChatEventUsernameChanged {
  fn as_ref(&self) -> &ChatEventUsernameChanged { self }
}

impl AsRef<ChatEventUsernameChanged> for RTDChatEventUsernameChangedBuilder {
  fn as_ref(&self) -> &ChatEventUsernameChanged { &self.inner }
}







/// A video chat was created
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventVideoChatCreated {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the video chat. The video chat can be received through the method getGroupCall
  group_call_id: i64,
  
}

impl RObject for ChatEventVideoChatCreated {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventVideoChatCreated" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventVideoChatCreated {}



impl ChatEventVideoChatCreated {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventVideoChatCreatedBuilder {
    let mut inner = ChatEventVideoChatCreated::default();
    inner.td_name = "chatEventVideoChatCreated".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventVideoChatCreatedBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 { self.group_call_id }

}

#[doc(hidden)]
pub struct RTDChatEventVideoChatCreatedBuilder {
  inner: ChatEventVideoChatCreated
}

impl RTDChatEventVideoChatCreatedBuilder {
  pub fn build(&self) -> ChatEventVideoChatCreated { self.inner.clone() }

   
  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

}

impl AsRef<ChatEventVideoChatCreated> for ChatEventVideoChatCreated {
  fn as_ref(&self) -> &ChatEventVideoChatCreated { self }
}

impl AsRef<ChatEventVideoChatCreated> for RTDChatEventVideoChatCreatedBuilder {
  fn as_ref(&self) -> &ChatEventVideoChatCreated { &self.inner }
}







/// A video chat was discarded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventVideoChatDiscarded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the video chat. The video chat can be received through the method getGroupCall
  group_call_id: i64,
  
}

impl RObject for ChatEventVideoChatDiscarded {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventVideoChatDiscarded" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventVideoChatDiscarded {}



impl ChatEventVideoChatDiscarded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventVideoChatDiscardedBuilder {
    let mut inner = ChatEventVideoChatDiscarded::default();
    inner.td_name = "chatEventVideoChatDiscarded".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventVideoChatDiscardedBuilder { inner }
  }

  pub fn group_call_id(&self) -> i64 { self.group_call_id }

}

#[doc(hidden)]
pub struct RTDChatEventVideoChatDiscardedBuilder {
  inner: ChatEventVideoChatDiscarded
}

impl RTDChatEventVideoChatDiscardedBuilder {
  pub fn build(&self) -> ChatEventVideoChatDiscarded { self.inner.clone() }

   
  pub fn group_call_id(&mut self, group_call_id: i64) -> &mut Self {
    self.inner.group_call_id = group_call_id;
    self
  }

}

impl AsRef<ChatEventVideoChatDiscarded> for ChatEventVideoChatDiscarded {
  fn as_ref(&self) -> &ChatEventVideoChatDiscarded { self }
}

impl AsRef<ChatEventVideoChatDiscarded> for RTDChatEventVideoChatDiscardedBuilder {
  fn as_ref(&self) -> &ChatEventVideoChatDiscarded { &self.inner }
}







/// The mute_new_participants setting of a video chat was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventVideoChatMuteNewParticipantsToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// New value of the mute_new_participants setting
  mute_new_participants: bool,
  
}

impl RObject for ChatEventVideoChatMuteNewParticipantsToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventVideoChatMuteNewParticipantsToggled" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventVideoChatMuteNewParticipantsToggled {}



impl ChatEventVideoChatMuteNewParticipantsToggled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventVideoChatMuteNewParticipantsToggledBuilder {
    let mut inner = ChatEventVideoChatMuteNewParticipantsToggled::default();
    inner.td_name = "chatEventVideoChatMuteNewParticipantsToggled".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventVideoChatMuteNewParticipantsToggledBuilder { inner }
  }

  pub fn mute_new_participants(&self) -> bool { self.mute_new_participants }

}

#[doc(hidden)]
pub struct RTDChatEventVideoChatMuteNewParticipantsToggledBuilder {
  inner: ChatEventVideoChatMuteNewParticipantsToggled
}

impl RTDChatEventVideoChatMuteNewParticipantsToggledBuilder {
  pub fn build(&self) -> ChatEventVideoChatMuteNewParticipantsToggled { self.inner.clone() }

   
  pub fn mute_new_participants(&mut self, mute_new_participants: bool) -> &mut Self {
    self.inner.mute_new_participants = mute_new_participants;
    self
  }

}

impl AsRef<ChatEventVideoChatMuteNewParticipantsToggled> for ChatEventVideoChatMuteNewParticipantsToggled {
  fn as_ref(&self) -> &ChatEventVideoChatMuteNewParticipantsToggled { self }
}

impl AsRef<ChatEventVideoChatMuteNewParticipantsToggled> for RTDChatEventVideoChatMuteNewParticipantsToggledBuilder {
  fn as_ref(&self) -> &ChatEventVideoChatMuteNewParticipantsToggled { &self.inner }
}







/// A video chat participant was muted or unmuted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventVideoChatParticipantIsMutedToggled {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the affected group call participant
  participant_id: MessageSender,
  /// New value of is_muted
  is_muted: bool,
  
}

impl RObject for ChatEventVideoChatParticipantIsMutedToggled {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventVideoChatParticipantIsMutedToggled" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventVideoChatParticipantIsMutedToggled {}



impl ChatEventVideoChatParticipantIsMutedToggled {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventVideoChatParticipantIsMutedToggledBuilder {
    let mut inner = ChatEventVideoChatParticipantIsMutedToggled::default();
    inner.td_name = "chatEventVideoChatParticipantIsMutedToggled".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventVideoChatParticipantIsMutedToggledBuilder { inner }
  }

  pub fn participant_id(&self) -> &MessageSender { &self.participant_id }

  pub fn is_muted(&self) -> bool { self.is_muted }

}

#[doc(hidden)]
pub struct RTDChatEventVideoChatParticipantIsMutedToggledBuilder {
  inner: ChatEventVideoChatParticipantIsMutedToggled
}

impl RTDChatEventVideoChatParticipantIsMutedToggledBuilder {
  pub fn build(&self) -> ChatEventVideoChatParticipantIsMutedToggled { self.inner.clone() }

   
  pub fn participant_id<T: AsRef<MessageSender>>(&mut self, participant_id: T) -> &mut Self {
    self.inner.participant_id = participant_id.as_ref().clone();
    self
  }

   
  pub fn is_muted(&mut self, is_muted: bool) -> &mut Self {
    self.inner.is_muted = is_muted;
    self
  }

}

impl AsRef<ChatEventVideoChatParticipantIsMutedToggled> for ChatEventVideoChatParticipantIsMutedToggled {
  fn as_ref(&self) -> &ChatEventVideoChatParticipantIsMutedToggled { self }
}

impl AsRef<ChatEventVideoChatParticipantIsMutedToggled> for RTDChatEventVideoChatParticipantIsMutedToggledBuilder {
  fn as_ref(&self) -> &ChatEventVideoChatParticipantIsMutedToggled { &self.inner }
}







/// A video chat participant volume level was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventVideoChatParticipantVolumeLevelChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the affected group call participant
  participant_id: MessageSender,
  /// New value of volume_level; 1-20000 in hundreds of percents
  volume_level: i64,
  
}

impl RObject for ChatEventVideoChatParticipantVolumeLevelChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventVideoChatParticipantVolumeLevelChanged" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatEventAction for ChatEventVideoChatParticipantVolumeLevelChanged {}



impl ChatEventVideoChatParticipantVolumeLevelChanged {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventVideoChatParticipantVolumeLevelChangedBuilder {
    let mut inner = ChatEventVideoChatParticipantVolumeLevelChanged::default();
    inner.td_name = "chatEventVideoChatParticipantVolumeLevelChanged".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventVideoChatParticipantVolumeLevelChangedBuilder { inner }
  }

  pub fn participant_id(&self) -> &MessageSender { &self.participant_id }

  pub fn volume_level(&self) -> i64 { self.volume_level }

}

#[doc(hidden)]
pub struct RTDChatEventVideoChatParticipantVolumeLevelChangedBuilder {
  inner: ChatEventVideoChatParticipantVolumeLevelChanged
}

impl RTDChatEventVideoChatParticipantVolumeLevelChangedBuilder {
  pub fn build(&self) -> ChatEventVideoChatParticipantVolumeLevelChanged { self.inner.clone() }

   
  pub fn participant_id<T: AsRef<MessageSender>>(&mut self, participant_id: T) -> &mut Self {
    self.inner.participant_id = participant_id.as_ref().clone();
    self
  }

   
  pub fn volume_level(&mut self, volume_level: i64) -> &mut Self {
    self.inner.volume_level = volume_level;
    self
  }

}

impl AsRef<ChatEventVideoChatParticipantVolumeLevelChanged> for ChatEventVideoChatParticipantVolumeLevelChanged {
  fn as_ref(&self) -> &ChatEventVideoChatParticipantVolumeLevelChanged { self }
}

impl AsRef<ChatEventVideoChatParticipantVolumeLevelChanged> for RTDChatEventVideoChatParticipantVolumeLevelChangedBuilder {
  fn as_ref(&self) -> &ChatEventVideoChatParticipantVolumeLevelChanged { &self.inner }
}



