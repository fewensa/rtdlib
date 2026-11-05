use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Message {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message identifier; unique for the chat to which the message belongs
  id: i64,
  /// Identifier of the sender of the message
  sender_id: MessageSender,
  /// Identifier of the user or the chat which received the ephemeral message; may be null. Always null for non-ephemeral messages
  receiver_id: Option<MessageSender>,
  /// Chat identifier
  chat_id: i64,
  /// The sending state of the message; may be null if the message isn't being sent and didn't fail to be sent
  sending_state: Option<MessageSendingState>,
  /// The scheduling state of the message; may be null if the message isn't scheduled
  scheduling_state: Option<MessageSchedulingState>,
  /// True, if the message is outgoing
  is_outgoing: bool,
  /// True, if the message is pinned
  is_pinned: bool,
  /// True, if the message was sent because of a scheduled action by the message sender, for example, as away, or greeting service message
  is_from_offline: bool,
  /// True, if content of the message can be saved locally
  can_be_saved: bool,
  /// True, if media timestamp entities refers to a media in this message as opposed to a media in the replied message
  has_timestamped_media: bool,
  /// True, if the message is a channel post. All messages to channels are channel posts, all other messages are not channel posts
  is_channel_post: bool,
  /// True, if the message is a suggested channel post which was paid in Telegram Stars; a warning must be shown if the message is deleted in less than getOption("suggested_post_lifetime_min") seconds after sending
  is_paid_star_suggested_post: bool,
  /// True, if the message is a suggested channel post which was paid in TON Grams; a warning must be shown if the message is deleted in less than getOption("suggested_post_lifetime_min") seconds after sending
  is_paid_gram_suggested_post: bool,
  /// True, if the message contains an unread mention for the current user
  contains_unread_mention: bool,
  /// True, if the message is a poll message with unread votes
  contains_unread_poll_votes: bool,
  /// Point in time (Unix timestamp) when the message was sent; 0 for scheduled messages
  date: i64,
  /// Point in time (Unix timestamp) when the message was last edited; 0 for scheduled messages. If getOption("show_message_edit_date_by_default") is true, then the date must be shown along with the message instead of the date when the message was sent
  edit_date: i64,
  /// Information about the initial message sender; may be null if none or unknown
  forward_info: Option<MessageForwardInfo>,
  /// Information about the initial message for messages created with importMessages; may be null if the message isn't imported
  import_info: Option<MessageImportInfo>,
  /// Information about interactions with the message; may be null if none
  interaction_info: Option<MessageInteractionInfo>,
  /// Information about unread reactions added to the message
  unread_reactions: Vec<UnreadReaction>,
  /// Information about fact-check added to the message; may be null if none
  fact_check: Option<FactCheck>,
  /// Information about the suggested post; may be null if the message isn't a suggested post
  suggested_post_info: Option<SuggestedPostInfo>,
  /// Information about the message or the story this message is replying to; may be null if none
  reply_to: Option<MessageReplyTo>,
  /// Identifier of the topic within the chat to which the message belongs; may be null if none; may change when the chat is converted to a forum or back
  topic_id: Option<MessageTopic>,
  /// The message's self-destruct type; may be null if none
  self_destruct_type: Option<MessageSelfDestructType>,
  /// Time left before the message self-destruct timer expires, in seconds; 0 if self-destruction isn't scheduled yet
  self_destruct_in: f32,
  /// Time left before the message will be automatically deleted by message_auto_delete_time setting of the chat, in seconds; 0 if never
  auto_delete_in: f32,
  /// If non-zero, the user identifier of the inline bot through which this message was sent
  via_bot_user_id: i64,
  /// The identifier of the user or chat which used a guest bot to send the message; may be null if none
  guest_bot_caller_id: Option<MessageSender>,
  /// If non-zero, the user identifier of the business bot that sent this message
  sender_business_bot_user_id: i64,
  /// Number of times the sender of the message boosted the supergroup at the time the message was sent; 0 if none or unknown. For messages sent by the current user, supergroupFullInfo.my_boost_count must be used instead
  sender_boost_count: i64,
  /// Tag of the sender of the message in the supergroup at the time the message was sent; may be empty if none or unknown. For messages sent in basic groups or by supergroup administrators, the current custom title or tag must be used instead
  sender_tag: String,
  /// The number of Telegram Stars the sender paid to send the message
  paid_message_star_count: i64,
  /// For channel posts and anonymous group messages, optional author signature
  author_signature: String,
  /// Unique identifier of an album this message belongs to; 0 if none. Only audios, documents, photos and videos can be grouped together in albums
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  media_album_id: isize,
  /// Unique identifier of the effect added to the message; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  effect_id: isize,
  /// Information about the restrictions that must be applied to the message content; may be null if none
  restriction_info: Option<RestrictionInfo>,
  /// IETF language tag of the message language on which it can be summarized; empty if summary isn't available for the message
  summary_language_code: String,
  /// Content of the message
  content: MessageContent,
  /// Content of the message, which is visible only to the current user and must be shown instead of the regular content; may be null if none
  ephemeral_content: Option<EphemeralMessageContent>,
  /// Reply markup for the message; may be null if none
  reply_markup: Option<ReplyMarkup>,
  /// Unique identifier of the ephemeral message if the message is ephemeral; for bots only
  ephemeral_message_id: i64,
  /// Identifier that uniquely corresponds to the chat to which the message was sent; for bots only
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  chat_instance: isize,
}

impl RObject for Message {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "message"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Message {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDMessageBuilder {
    let inner = Message {
      td_name: "message".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDMessageBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn receiver_id(&self) -> &Option<MessageSender> {
    &self.receiver_id
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn sending_state(&self) -> &Option<MessageSendingState> {
    &self.sending_state
  }

  pub fn scheduling_state(&self) -> &Option<MessageSchedulingState> {
    &self.scheduling_state
  }

  pub fn is_outgoing(&self) -> bool {
    self.is_outgoing
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }

  pub fn is_from_offline(&self) -> bool {
    self.is_from_offline
  }

  pub fn can_be_saved(&self) -> bool {
    self.can_be_saved
  }

  pub fn has_timestamped_media(&self) -> bool {
    self.has_timestamped_media
  }

  pub fn is_channel_post(&self) -> bool {
    self.is_channel_post
  }

  pub fn is_paid_star_suggested_post(&self) -> bool {
    self.is_paid_star_suggested_post
  }

  pub fn is_paid_gram_suggested_post(&self) -> bool {
    self.is_paid_gram_suggested_post
  }

  pub fn contains_unread_mention(&self) -> bool {
    self.contains_unread_mention
  }

  pub fn contains_unread_poll_votes(&self) -> bool {
    self.contains_unread_poll_votes
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn edit_date(&self) -> i64 {
    self.edit_date
  }

  pub fn forward_info(&self) -> &Option<MessageForwardInfo> {
    &self.forward_info
  }

  pub fn import_info(&self) -> &Option<MessageImportInfo> {
    &self.import_info
  }

  pub fn interaction_info(&self) -> &Option<MessageInteractionInfo> {
    &self.interaction_info
  }

  pub fn unread_reactions(&self) -> &Vec<UnreadReaction> {
    &self.unread_reactions
  }

  pub fn fact_check(&self) -> &Option<FactCheck> {
    &self.fact_check
  }

  pub fn suggested_post_info(&self) -> &Option<SuggestedPostInfo> {
    &self.suggested_post_info
  }

  pub fn reply_to(&self) -> &Option<MessageReplyTo> {
    &self.reply_to
  }

  pub fn topic_id(&self) -> &Option<MessageTopic> {
    &self.topic_id
  }

  pub fn self_destruct_type(&self) -> &Option<MessageSelfDestructType> {
    &self.self_destruct_type
  }

  pub fn self_destruct_in(&self) -> f32 {
    self.self_destruct_in
  }

  pub fn auto_delete_in(&self) -> f32 {
    self.auto_delete_in
  }

  pub fn via_bot_user_id(&self) -> i64 {
    self.via_bot_user_id
  }

  pub fn guest_bot_caller_id(&self) -> &Option<MessageSender> {
    &self.guest_bot_caller_id
  }

  pub fn sender_business_bot_user_id(&self) -> i64 {
    self.sender_business_bot_user_id
  }

  pub fn sender_boost_count(&self) -> i64 {
    self.sender_boost_count
  }

  pub fn sender_tag(&self) -> &String {
    &self.sender_tag
  }

  pub fn paid_message_star_count(&self) -> i64 {
    self.paid_message_star_count
  }

  pub fn author_signature(&self) -> &String {
    &self.author_signature
  }

  pub fn media_album_id(&self) -> isize {
    self.media_album_id
  }

  pub fn effect_id(&self) -> isize {
    self.effect_id
  }

  pub fn restriction_info(&self) -> &Option<RestrictionInfo> {
    &self.restriction_info
  }

  pub fn summary_language_code(&self) -> &String {
    &self.summary_language_code
  }

  pub fn content(&self) -> &MessageContent {
    &self.content
  }

  pub fn ephemeral_content(&self) -> &Option<EphemeralMessageContent> {
    &self.ephemeral_content
  }

  pub fn reply_markup(&self) -> &Option<ReplyMarkup> {
    &self.reply_markup
  }

  pub fn ephemeral_message_id(&self) -> i64 {
    self.ephemeral_message_id
  }

  pub fn chat_instance(&self) -> isize {
    self.chat_instance
  }
}

#[doc(hidden)]
pub struct RTDMessageBuilder {
  inner: Message,
}

impl RTDMessageBuilder {
  pub fn build(&self) -> Message {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = sender_id.as_ref().clone();
    self
  }

  pub fn receiver_id<T: AsRef<MessageSender>>(&mut self, receiver_id: T) -> &mut Self {
    self.inner.receiver_id = Some(receiver_id.as_ref().clone());
    self
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn sending_state<T: AsRef<MessageSendingState>>(&mut self, sending_state: T) -> &mut Self {
    self.inner.sending_state = Some(sending_state.as_ref().clone());
    self
  }

  pub fn scheduling_state<T: AsRef<MessageSchedulingState>>(
    &mut self,
    scheduling_state: T,
  ) -> &mut Self {
    self.inner.scheduling_state = Some(scheduling_state.as_ref().clone());
    self
  }

  pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
    self.inner.is_outgoing = is_outgoing;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }

  pub fn is_from_offline(&mut self, is_from_offline: bool) -> &mut Self {
    self.inner.is_from_offline = is_from_offline;
    self
  }

  pub fn can_be_saved(&mut self, can_be_saved: bool) -> &mut Self {
    self.inner.can_be_saved = can_be_saved;
    self
  }

  pub fn has_timestamped_media(&mut self, has_timestamped_media: bool) -> &mut Self {
    self.inner.has_timestamped_media = has_timestamped_media;
    self
  }

  pub fn is_channel_post(&mut self, is_channel_post: bool) -> &mut Self {
    self.inner.is_channel_post = is_channel_post;
    self
  }

  pub fn is_paid_star_suggested_post(&mut self, is_paid_star_suggested_post: bool) -> &mut Self {
    self.inner.is_paid_star_suggested_post = is_paid_star_suggested_post;
    self
  }

  pub fn is_paid_gram_suggested_post(&mut self, is_paid_gram_suggested_post: bool) -> &mut Self {
    self.inner.is_paid_gram_suggested_post = is_paid_gram_suggested_post;
    self
  }

  pub fn contains_unread_mention(&mut self, contains_unread_mention: bool) -> &mut Self {
    self.inner.contains_unread_mention = contains_unread_mention;
    self
  }

  pub fn contains_unread_poll_votes(&mut self, contains_unread_poll_votes: bool) -> &mut Self {
    self.inner.contains_unread_poll_votes = contains_unread_poll_votes;
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn edit_date(&mut self, edit_date: i64) -> &mut Self {
    self.inner.edit_date = edit_date;
    self
  }

  pub fn forward_info<T: AsRef<MessageForwardInfo>>(&mut self, forward_info: T) -> &mut Self {
    self.inner.forward_info = Some(forward_info.as_ref().clone());
    self
  }

  pub fn import_info<T: AsRef<MessageImportInfo>>(&mut self, import_info: T) -> &mut Self {
    self.inner.import_info = Some(import_info.as_ref().clone());
    self
  }

  pub fn interaction_info<T: AsRef<MessageInteractionInfo>>(
    &mut self,
    interaction_info: T,
  ) -> &mut Self {
    self.inner.interaction_info = Some(interaction_info.as_ref().clone());
    self
  }

  pub fn unread_reactions(&mut self, unread_reactions: Vec<UnreadReaction>) -> &mut Self {
    self.inner.unread_reactions = unread_reactions;
    self
  }

  pub fn fact_check<T: AsRef<FactCheck>>(&mut self, fact_check: T) -> &mut Self {
    self.inner.fact_check = Some(fact_check.as_ref().clone());
    self
  }

  pub fn suggested_post_info<T: AsRef<SuggestedPostInfo>>(
    &mut self,
    suggested_post_info: T,
  ) -> &mut Self {
    self.inner.suggested_post_info = Some(suggested_post_info.as_ref().clone());
    self
  }

  pub fn reply_to<T: AsRef<MessageReplyTo>>(&mut self, reply_to: T) -> &mut Self {
    self.inner.reply_to = Some(reply_to.as_ref().clone());
    self
  }

  pub fn topic_id<T: AsRef<MessageTopic>>(&mut self, topic_id: T) -> &mut Self {
    self.inner.topic_id = Some(topic_id.as_ref().clone());
    self
  }

  pub fn self_destruct_type<T: AsRef<MessageSelfDestructType>>(
    &mut self,
    self_destruct_type: T,
  ) -> &mut Self {
    self.inner.self_destruct_type = Some(self_destruct_type.as_ref().clone());
    self
  }

  pub fn self_destruct_in(&mut self, self_destruct_in: f32) -> &mut Self {
    self.inner.self_destruct_in = self_destruct_in;
    self
  }

  pub fn auto_delete_in(&mut self, auto_delete_in: f32) -> &mut Self {
    self.inner.auto_delete_in = auto_delete_in;
    self
  }

  pub fn via_bot_user_id(&mut self, via_bot_user_id: i64) -> &mut Self {
    self.inner.via_bot_user_id = via_bot_user_id;
    self
  }

  pub fn guest_bot_caller_id<T: AsRef<MessageSender>>(
    &mut self,
    guest_bot_caller_id: T,
  ) -> &mut Self {
    self.inner.guest_bot_caller_id = Some(guest_bot_caller_id.as_ref().clone());
    self
  }

  pub fn sender_business_bot_user_id(&mut self, sender_business_bot_user_id: i64) -> &mut Self {
    self.inner.sender_business_bot_user_id = sender_business_bot_user_id;
    self
  }

  pub fn sender_boost_count(&mut self, sender_boost_count: i64) -> &mut Self {
    self.inner.sender_boost_count = sender_boost_count;
    self
  }

  pub fn sender_tag<T: AsRef<str>>(&mut self, sender_tag: T) -> &mut Self {
    self.inner.sender_tag = sender_tag.as_ref().to_string();
    self
  }

  pub fn paid_message_star_count(&mut self, paid_message_star_count: i64) -> &mut Self {
    self.inner.paid_message_star_count = paid_message_star_count;
    self
  }

  pub fn author_signature<T: AsRef<str>>(&mut self, author_signature: T) -> &mut Self {
    self.inner.author_signature = author_signature.as_ref().to_string();
    self
  }

  pub fn media_album_id(&mut self, media_album_id: isize) -> &mut Self {
    self.inner.media_album_id = media_album_id;
    self
  }

  pub fn effect_id(&mut self, effect_id: isize) -> &mut Self {
    self.inner.effect_id = effect_id;
    self
  }

  pub fn restriction_info<T: AsRef<RestrictionInfo>>(&mut self, restriction_info: T) -> &mut Self {
    self.inner.restriction_info = Some(restriction_info.as_ref().clone());
    self
  }

  pub fn summary_language_code<T: AsRef<str>>(&mut self, summary_language_code: T) -> &mut Self {
    self.inner.summary_language_code = summary_language_code.as_ref().to_string();
    self
  }

  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

  pub fn ephemeral_content<T: AsRef<EphemeralMessageContent>>(
    &mut self,
    ephemeral_content: T,
  ) -> &mut Self {
    self.inner.ephemeral_content = Some(ephemeral_content.as_ref().clone());
    self
  }

  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = Some(reply_markup.as_ref().clone());
    self
  }

  pub fn ephemeral_message_id(&mut self, ephemeral_message_id: i64) -> &mut Self {
    self.inner.ephemeral_message_id = ephemeral_message_id;
    self
  }

  pub fn chat_instance(&mut self, chat_instance: isize) -> &mut Self {
    self.inner.chat_instance = chat_instance;
    self
  }
}

impl AsRef<Message> for Message {
  fn as_ref(&self) -> &Message {
    self
  }
}

impl AsRef<Message> for RTDMessageBuilder {
  fn as_ref(&self) -> &Message {
    &self.inner
  }
}
