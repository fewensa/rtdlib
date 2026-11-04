use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains properties of a message and describes actions that can be done with the message right now
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageProperties {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if an offer can be added to the message using addOffer
  can_add_offer: bool,
  /// True, if tasks can be added to the message's checklist using addChecklistTasks if the current user has Telegram Premium subscription
  can_add_tasks: bool,
  /// True, if the message is a suggested post that can be approved by the user using approveSuggestedPost
  can_be_approved: bool,
  /// True, if content of the message can be copied using inputMessageForwarded or forwardMessages with copy options
  can_be_copied: bool,
  /// True, if content of the message can be copied to a secret chat using inputMessageForwarded or forwardMessages with copy options
  can_be_copied_to_secret_chat: bool,
  /// True, if the message is a suggested post that can be declined by the user using declineSuggestedPost
  can_be_declined: bool,
  /// True, if the message can be deleted only for the current user while other users will continue to see it using the method deleteMessages with revoke == false
  can_be_deleted_only_for_self: bool,
  /// True, if the message can be deleted for all users using the method deleteMessages with revoke == true
  can_be_deleted_for_all_users: bool,
  /// True, if the message can be edited using the methods editMessageText, editMessageCaption, or editMessageReplyMarkup. For live location, poll, and checklist messages this field shows whether editMessageLiveLocation, stopPoll, or editMessageChecklist respectively can be used with this message
  can_be_edited: bool,
  /// True, if the message can be forwarded using inputMessageForwarded or forwardMessages without copy options
  can_be_forwarded: bool,
  /// True, if the message can be paid using inputInvoiceMessage
  can_be_paid: bool,
  /// True, if the message can be pinned or unpinned in the chat using pinChatMessage or unpinChatMessage
  can_be_pinned: bool,
  /// True, if the message can be replied in the same chat and forum topic using inputMessageReplyToMessage. Ephemeral messages can be replied only by other ephemeral messages
  can_be_replied: bool,
  /// True, if the message can be replied in another chat or forum topic using inputMessageReplyToExternalMessage
  can_be_replied_in_another_chat: bool,
  /// True, if content of the message can be saved locally
  can_be_saved: bool,
  /// True, if the message can be shared in a story using inputStoryAreaTypeMessage
  can_be_shared_in_story: bool,
  /// True, if the user can delete reactions of other users in the message using the method deleteMessageReactionsFromSender
  can_delete_reactions: bool,
  /// True, if the message can be edited using the method editMessageMedia
  can_edit_media: bool,
  /// True, if scheduling state of the message can be edited
  can_edit_scheduling_state: bool,
  /// True, if another price or post send time can be suggested using addOffer
  can_edit_suggested_post_info: bool,
  /// True, if author of the message sent on behalf of a chat can be received through getMessageAuthor
  can_get_author: bool,
  /// True, if code for message embedding can be received using getMessageEmbeddingCode
  can_get_embedding_code: bool,
  /// True, if a link can be generated for the message using getMessageLink
  can_get_link: bool,
  /// True, if media timestamp links can be generated for media timestamp entities in the message text, caption, or link preview description using getMessageLink
  can_get_media_timestamp_links: bool,
  /// True, if information about the message thread is available through getMessageThread and getMessageThreadHistory
  can_get_message_thread: bool,
  /// True, if the message is a poll and vote statistics are available through getPollVoteStatistics
  can_get_poll_vote_statistics: bool,
  /// True, if read date of the message can be received through getMessageReadDate
  can_get_read_date: bool,
  /// True, if message statistics are available through getMessageStatistics and message forwards can be received using getMessagePublicForwards
  can_get_statistics: bool,
  /// True, if advertisements for video of the message can be received through getVideoMessageAdvertisements
  can_get_video_advertisements: bool,
  /// True, if chat members already viewed the message can be received through getMessageViewers
  can_get_viewers: bool,
  /// True, if tasks can be marked as done or not done in the message's checklist using markChecklistTasksAsDone if the current user has Telegram Premium subscription
  can_mark_tasks_as_done: bool,
  /// True, if speech can be recognized for the message through recognizeSpeech
  can_recognize_speech: bool,
  /// True, if the message can be reported using reportChat
  can_report_chat: bool,
  /// True, if reactions on the message can be reported through reportMessageReactions
  can_report_reactions: bool,
  /// True, if the message can be reported using reportSupergroupSpam
  can_report_supergroup_spam: bool,
  /// True, if fact check for the message can be changed through setMessageFactCheck
  can_set_fact_check: bool,
  /// Identifiers of custom emoji used in the message
  custom_emoji_ids: Vec<isize>,
  /// True, if content of the message can't be saved locally, because it is protected by the current user; if true, then can_be_saved is false
  has_protected_content_by_current_user: bool,
  /// True, if content of the message can't be saved locally, because it is protected by the other user; if true, then can_be_saved is false
  has_protected_content_by_other_user: bool,
  /// True, if message statistics must be available from context menu of the message
  need_show_statistics: bool,
}

impl RObject for MessageProperties {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "messageProperties"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl MessageProperties {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessagePropertiesBuilder {
    let inner = MessageProperties {
      td_name: "messageProperties".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessagePropertiesBuilder { inner }
  }

  pub fn can_add_offer(&self) -> bool {
    self.can_add_offer
  }

  pub fn can_add_tasks(&self) -> bool {
    self.can_add_tasks
  }

  pub fn can_be_approved(&self) -> bool {
    self.can_be_approved
  }

  pub fn can_be_copied(&self) -> bool {
    self.can_be_copied
  }

  pub fn can_be_copied_to_secret_chat(&self) -> bool {
    self.can_be_copied_to_secret_chat
  }

  pub fn can_be_declined(&self) -> bool {
    self.can_be_declined
  }

  pub fn can_be_deleted_only_for_self(&self) -> bool {
    self.can_be_deleted_only_for_self
  }

  pub fn can_be_deleted_for_all_users(&self) -> bool {
    self.can_be_deleted_for_all_users
  }

  pub fn can_be_edited(&self) -> bool {
    self.can_be_edited
  }

  pub fn can_be_forwarded(&self) -> bool {
    self.can_be_forwarded
  }

  pub fn can_be_paid(&self) -> bool {
    self.can_be_paid
  }

  pub fn can_be_pinned(&self) -> bool {
    self.can_be_pinned
  }

  pub fn can_be_replied(&self) -> bool {
    self.can_be_replied
  }

  pub fn can_be_replied_in_another_chat(&self) -> bool {
    self.can_be_replied_in_another_chat
  }

  pub fn can_be_saved(&self) -> bool {
    self.can_be_saved
  }

  pub fn can_be_shared_in_story(&self) -> bool {
    self.can_be_shared_in_story
  }

  pub fn can_delete_reactions(&self) -> bool {
    self.can_delete_reactions
  }

  pub fn can_edit_media(&self) -> bool {
    self.can_edit_media
  }

  pub fn can_edit_scheduling_state(&self) -> bool {
    self.can_edit_scheduling_state
  }

  pub fn can_edit_suggested_post_info(&self) -> bool {
    self.can_edit_suggested_post_info
  }

  pub fn can_get_author(&self) -> bool {
    self.can_get_author
  }

  pub fn can_get_embedding_code(&self) -> bool {
    self.can_get_embedding_code
  }

  pub fn can_get_link(&self) -> bool {
    self.can_get_link
  }

  pub fn can_get_media_timestamp_links(&self) -> bool {
    self.can_get_media_timestamp_links
  }

  pub fn can_get_message_thread(&self) -> bool {
    self.can_get_message_thread
  }

  pub fn can_get_poll_vote_statistics(&self) -> bool {
    self.can_get_poll_vote_statistics
  }

  pub fn can_get_read_date(&self) -> bool {
    self.can_get_read_date
  }

  pub fn can_get_statistics(&self) -> bool {
    self.can_get_statistics
  }

  pub fn can_get_video_advertisements(&self) -> bool {
    self.can_get_video_advertisements
  }

  pub fn can_get_viewers(&self) -> bool {
    self.can_get_viewers
  }

  pub fn can_mark_tasks_as_done(&self) -> bool {
    self.can_mark_tasks_as_done
  }

  pub fn can_recognize_speech(&self) -> bool {
    self.can_recognize_speech
  }

  pub fn can_report_chat(&self) -> bool {
    self.can_report_chat
  }

  pub fn can_report_reactions(&self) -> bool {
    self.can_report_reactions
  }

  pub fn can_report_supergroup_spam(&self) -> bool {
    self.can_report_supergroup_spam
  }

  pub fn can_set_fact_check(&self) -> bool {
    self.can_set_fact_check
  }

  pub fn custom_emoji_ids(&self) -> &Vec<isize> {
    &self.custom_emoji_ids
  }

  pub fn has_protected_content_by_current_user(&self) -> bool {
    self.has_protected_content_by_current_user
  }

  pub fn has_protected_content_by_other_user(&self) -> bool {
    self.has_protected_content_by_other_user
  }

  pub fn need_show_statistics(&self) -> bool {
    self.need_show_statistics
  }
}

#[doc(hidden)]
pub struct RTDMessagePropertiesBuilder {
  inner: MessageProperties,
}

impl RTDMessagePropertiesBuilder {
  pub fn build(&self) -> MessageProperties {
    self.inner.clone()
  }

  pub fn can_add_offer(&mut self, can_add_offer: bool) -> &mut Self {
    self.inner.can_add_offer = can_add_offer;
    self
  }

  pub fn can_add_tasks(&mut self, can_add_tasks: bool) -> &mut Self {
    self.inner.can_add_tasks = can_add_tasks;
    self
  }

  pub fn can_be_approved(&mut self, can_be_approved: bool) -> &mut Self {
    self.inner.can_be_approved = can_be_approved;
    self
  }

  pub fn can_be_copied(&mut self, can_be_copied: bool) -> &mut Self {
    self.inner.can_be_copied = can_be_copied;
    self
  }

  pub fn can_be_copied_to_secret_chat(&mut self, can_be_copied_to_secret_chat: bool) -> &mut Self {
    self.inner.can_be_copied_to_secret_chat = can_be_copied_to_secret_chat;
    self
  }

  pub fn can_be_declined(&mut self, can_be_declined: bool) -> &mut Self {
    self.inner.can_be_declined = can_be_declined;
    self
  }

  pub fn can_be_deleted_only_for_self(&mut self, can_be_deleted_only_for_self: bool) -> &mut Self {
    self.inner.can_be_deleted_only_for_self = can_be_deleted_only_for_self;
    self
  }

  pub fn can_be_deleted_for_all_users(&mut self, can_be_deleted_for_all_users: bool) -> &mut Self {
    self.inner.can_be_deleted_for_all_users = can_be_deleted_for_all_users;
    self
  }

  pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
    self.inner.can_be_edited = can_be_edited;
    self
  }

  pub fn can_be_forwarded(&mut self, can_be_forwarded: bool) -> &mut Self {
    self.inner.can_be_forwarded = can_be_forwarded;
    self
  }

  pub fn can_be_paid(&mut self, can_be_paid: bool) -> &mut Self {
    self.inner.can_be_paid = can_be_paid;
    self
  }

  pub fn can_be_pinned(&mut self, can_be_pinned: bool) -> &mut Self {
    self.inner.can_be_pinned = can_be_pinned;
    self
  }

  pub fn can_be_replied(&mut self, can_be_replied: bool) -> &mut Self {
    self.inner.can_be_replied = can_be_replied;
    self
  }

  pub fn can_be_replied_in_another_chat(
    &mut self,
    can_be_replied_in_another_chat: bool,
  ) -> &mut Self {
    self.inner.can_be_replied_in_another_chat = can_be_replied_in_another_chat;
    self
  }

  pub fn can_be_saved(&mut self, can_be_saved: bool) -> &mut Self {
    self.inner.can_be_saved = can_be_saved;
    self
  }

  pub fn can_be_shared_in_story(&mut self, can_be_shared_in_story: bool) -> &mut Self {
    self.inner.can_be_shared_in_story = can_be_shared_in_story;
    self
  }

  pub fn can_delete_reactions(&mut self, can_delete_reactions: bool) -> &mut Self {
    self.inner.can_delete_reactions = can_delete_reactions;
    self
  }

  pub fn can_edit_media(&mut self, can_edit_media: bool) -> &mut Self {
    self.inner.can_edit_media = can_edit_media;
    self
  }

  pub fn can_edit_scheduling_state(&mut self, can_edit_scheduling_state: bool) -> &mut Self {
    self.inner.can_edit_scheduling_state = can_edit_scheduling_state;
    self
  }

  pub fn can_edit_suggested_post_info(&mut self, can_edit_suggested_post_info: bool) -> &mut Self {
    self.inner.can_edit_suggested_post_info = can_edit_suggested_post_info;
    self
  }

  pub fn can_get_author(&mut self, can_get_author: bool) -> &mut Self {
    self.inner.can_get_author = can_get_author;
    self
  }

  pub fn can_get_embedding_code(&mut self, can_get_embedding_code: bool) -> &mut Self {
    self.inner.can_get_embedding_code = can_get_embedding_code;
    self
  }

  pub fn can_get_link(&mut self, can_get_link: bool) -> &mut Self {
    self.inner.can_get_link = can_get_link;
    self
  }

  pub fn can_get_media_timestamp_links(
    &mut self,
    can_get_media_timestamp_links: bool,
  ) -> &mut Self {
    self.inner.can_get_media_timestamp_links = can_get_media_timestamp_links;
    self
  }

  pub fn can_get_message_thread(&mut self, can_get_message_thread: bool) -> &mut Self {
    self.inner.can_get_message_thread = can_get_message_thread;
    self
  }

  pub fn can_get_poll_vote_statistics(&mut self, can_get_poll_vote_statistics: bool) -> &mut Self {
    self.inner.can_get_poll_vote_statistics = can_get_poll_vote_statistics;
    self
  }

  pub fn can_get_read_date(&mut self, can_get_read_date: bool) -> &mut Self {
    self.inner.can_get_read_date = can_get_read_date;
    self
  }

  pub fn can_get_statistics(&mut self, can_get_statistics: bool) -> &mut Self {
    self.inner.can_get_statistics = can_get_statistics;
    self
  }

  pub fn can_get_video_advertisements(&mut self, can_get_video_advertisements: bool) -> &mut Self {
    self.inner.can_get_video_advertisements = can_get_video_advertisements;
    self
  }

  pub fn can_get_viewers(&mut self, can_get_viewers: bool) -> &mut Self {
    self.inner.can_get_viewers = can_get_viewers;
    self
  }

  pub fn can_mark_tasks_as_done(&mut self, can_mark_tasks_as_done: bool) -> &mut Self {
    self.inner.can_mark_tasks_as_done = can_mark_tasks_as_done;
    self
  }

  pub fn can_recognize_speech(&mut self, can_recognize_speech: bool) -> &mut Self {
    self.inner.can_recognize_speech = can_recognize_speech;
    self
  }

  pub fn can_report_chat(&mut self, can_report_chat: bool) -> &mut Self {
    self.inner.can_report_chat = can_report_chat;
    self
  }

  pub fn can_report_reactions(&mut self, can_report_reactions: bool) -> &mut Self {
    self.inner.can_report_reactions = can_report_reactions;
    self
  }

  pub fn can_report_supergroup_spam(&mut self, can_report_supergroup_spam: bool) -> &mut Self {
    self.inner.can_report_supergroup_spam = can_report_supergroup_spam;
    self
  }

  pub fn can_set_fact_check(&mut self, can_set_fact_check: bool) -> &mut Self {
    self.inner.can_set_fact_check = can_set_fact_check;
    self
  }

  pub fn custom_emoji_ids(&mut self, custom_emoji_ids: Vec<isize>) -> &mut Self {
    self.inner.custom_emoji_ids = custom_emoji_ids;
    self
  }

  pub fn has_protected_content_by_current_user(
    &mut self,
    has_protected_content_by_current_user: bool,
  ) -> &mut Self {
    self.inner.has_protected_content_by_current_user = has_protected_content_by_current_user;
    self
  }

  pub fn has_protected_content_by_other_user(
    &mut self,
    has_protected_content_by_other_user: bool,
  ) -> &mut Self {
    self.inner.has_protected_content_by_other_user = has_protected_content_by_other_user;
    self
  }

  pub fn need_show_statistics(&mut self, need_show_statistics: bool) -> &mut Self {
    self.inner.need_show_statistics = need_show_statistics;
    self
  }
}

impl AsRef<MessageProperties> for MessageProperties {
  fn as_ref(&self) -> &MessageProperties {
    self
  }
}

impl AsRef<MessageProperties> for RTDMessagePropertiesBuilder {
  fn as_ref(&self) -> &MessageProperties {
    &self.inner
  }
}
