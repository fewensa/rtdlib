
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a message. 
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // message
  /// Message identifier, unique for the chat to which the message belongs.
  id: Option<i64>,
  /// Identifier of the user who sent the message; 0 if unknown. It is unknown for channel posts.
  sender_user_id: Option<i32>,
  /// Chat identifier.
  chat_id: Option<i64>,
  /// Information about the sending state of the message; may be null.
  sending_state: Option<Box<MessageSendingState>>,
  /// True, if the message is outgoing.
  is_outgoing: Option<bool>,
  /// True, if the message can be edited. For live location and poll messages this fields shows, whether editMessageLiveLocation or stopPoll can be used with this message by the client.
  can_be_edited: Option<bool>,
  /// True, if the message can be forwarded.
  can_be_forwarded: Option<bool>,
  /// True, if the message can be deleted only for the current user while other users will continue to see it.
  can_be_deleted_only_for_self: Option<bool>,
  /// True, if the message can be deleted for all users.
  can_be_deleted_for_all_users: Option<bool>,
  /// True, if the message is a channel post. All messages to channels are channel posts, all other messages are not channel posts.
  is_channel_post: Option<bool>,
  /// True, if the message contains an unread mention for the current user.
  contains_unread_mention: Option<bool>,
  /// Point in time (Unix timestamp) when the message was sent.
  date: Option<i32>,
  /// Point in time (Unix timestamp) when the message was last edited.
  edit_date: Option<i32>,
  /// Information about the initial message sender; may be null.
  forward_info: Option<MessageForwardInfo>,
  /// If non-zero, the identifier of the message this message is replying to; can be the identifier of a deleted message.
  reply_to_message_id: Option<i64>,
  /// For self-destructing messages, the message's TTL (Time To Live), in seconds; 0 if none. TDLib will send updateDeleteMessages or updateMessageContent once the TTL expires.
  ttl: Option<i32>,
  /// Time left before the message expires, in seconds.
  ttl_expires_in: Option<f64>,
  /// If non-zero, the user identifier of the bot through which this message was sent.
  via_bot_user_id: Option<i32>,
  /// For channel posts, optional author signature.
  author_signature: Option<String>,
  /// Number of times this message was viewed.
  views: Option<i32>,
  /// Unique identifier of an album this message belongs to. Only photos and videos can be grouped together in albums.
  media_album_id: Option<String>,
  /// Content of the message.
  content: Option<Box<MessageContent>>,
  /// Reply markup for the message; may be null.
  reply_markup: Option<Box<ReplyMarkup>>,
  
}


impl Clone for Message {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for Message {}
impl RObject for Message {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "message" }
  fn td_type(&self) -> RTDType { RTDType::Message }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Message {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "message".to_string(),
      id: None,
      sender_user_id: None,
      chat_id: None,
      sending_state: None,
      is_outgoing: None,
      can_be_edited: None,
      can_be_forwarded: None,
      can_be_deleted_only_for_self: None,
      can_be_deleted_for_all_users: None,
      is_channel_post: None,
      contains_unread_mention: None,
      date: None,
      edit_date: None,
      forward_info: None,
      reply_to_message_id: None,
      ttl: None,
      ttl_expires_in: None,
      via_bot_user_id: None,
      author_signature: None,
      views: None,
      media_album_id: None,
      content: None,
      reply_markup: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn sending_state(&self) -> Option<Box<MessageSendingState>> { self.sending_state.clone() }
  #[doc(hidden)] pub fn _set_sending_state(&mut self, sending_state: Box<MessageSendingState>) -> &mut Self { self.sending_state = Some(sending_state); self }
  
  pub fn is_outgoing(&self) -> Option<bool> { self.is_outgoing.clone() }
  #[doc(hidden)] pub fn _set_is_outgoing(&mut self, is_outgoing: bool) -> &mut Self { self.is_outgoing = Some(is_outgoing); self }
  
  pub fn can_be_edited(&self) -> Option<bool> { self.can_be_edited.clone() }
  #[doc(hidden)] pub fn _set_can_be_edited(&mut self, can_be_edited: bool) -> &mut Self { self.can_be_edited = Some(can_be_edited); self }
  
  pub fn can_be_forwarded(&self) -> Option<bool> { self.can_be_forwarded.clone() }
  #[doc(hidden)] pub fn _set_can_be_forwarded(&mut self, can_be_forwarded: bool) -> &mut Self { self.can_be_forwarded = Some(can_be_forwarded); self }
  
  pub fn can_be_deleted_only_for_self(&self) -> Option<bool> { self.can_be_deleted_only_for_self.clone() }
  #[doc(hidden)] pub fn _set_can_be_deleted_only_for_self(&mut self, can_be_deleted_only_for_self: bool) -> &mut Self { self.can_be_deleted_only_for_self = Some(can_be_deleted_only_for_self); self }
  
  pub fn can_be_deleted_for_all_users(&self) -> Option<bool> { self.can_be_deleted_for_all_users.clone() }
  #[doc(hidden)] pub fn _set_can_be_deleted_for_all_users(&mut self, can_be_deleted_for_all_users: bool) -> &mut Self { self.can_be_deleted_for_all_users = Some(can_be_deleted_for_all_users); self }
  
  pub fn is_channel_post(&self) -> Option<bool> { self.is_channel_post.clone() }
  #[doc(hidden)] pub fn _set_is_channel_post(&mut self, is_channel_post: bool) -> &mut Self { self.is_channel_post = Some(is_channel_post); self }
  
  pub fn contains_unread_mention(&self) -> Option<bool> { self.contains_unread_mention.clone() }
  #[doc(hidden)] pub fn _set_contains_unread_mention(&mut self, contains_unread_mention: bool) -> &mut Self { self.contains_unread_mention = Some(contains_unread_mention); self }
  
  pub fn date(&self) -> Option<i32> { self.date.clone() }
  #[doc(hidden)] pub fn _set_date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  
  pub fn edit_date(&self) -> Option<i32> { self.edit_date.clone() }
  #[doc(hidden)] pub fn _set_edit_date(&mut self, edit_date: i32) -> &mut Self { self.edit_date = Some(edit_date); self }
  
  pub fn forward_info(&self) -> Option<MessageForwardInfo> { self.forward_info.clone() }
  #[doc(hidden)] pub fn _set_forward_info(&mut self, forward_info: MessageForwardInfo) -> &mut Self { self.forward_info = Some(forward_info); self }
  
  pub fn reply_to_message_id(&self) -> Option<i64> { self.reply_to_message_id.clone() }
  #[doc(hidden)] pub fn _set_reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self { self.reply_to_message_id = Some(reply_to_message_id); self }
  
  pub fn ttl(&self) -> Option<i32> { self.ttl.clone() }
  #[doc(hidden)] pub fn _set_ttl(&mut self, ttl: i32) -> &mut Self { self.ttl = Some(ttl); self }
  
  pub fn ttl_expires_in(&self) -> Option<f64> { self.ttl_expires_in.clone() }
  #[doc(hidden)] pub fn _set_ttl_expires_in(&mut self, ttl_expires_in: f64) -> &mut Self { self.ttl_expires_in = Some(ttl_expires_in); self }
  
  pub fn via_bot_user_id(&self) -> Option<i32> { self.via_bot_user_id.clone() }
  #[doc(hidden)] pub fn _set_via_bot_user_id(&mut self, via_bot_user_id: i32) -> &mut Self { self.via_bot_user_id = Some(via_bot_user_id); self }
  
  pub fn author_signature(&self) -> Option<String> { self.author_signature.clone() }
  #[doc(hidden)] pub fn _set_author_signature(&mut self, author_signature: String) -> &mut Self { self.author_signature = Some(author_signature); self }
  
  pub fn views(&self) -> Option<i32> { self.views.clone() }
  #[doc(hidden)] pub fn _set_views(&mut self, views: i32) -> &mut Self { self.views = Some(views); self }
  
  pub fn media_album_id(&self) -> Option<String> { self.media_album_id.clone() }
  #[doc(hidden)] pub fn _set_media_album_id(&mut self, media_album_id: String) -> &mut Self { self.media_album_id = Some(media_album_id); self }
  
  pub fn content(&self) -> Option<Box<MessageContent>> { self.content.clone() }
  #[doc(hidden)] pub fn _set_content(&mut self, content: Box<MessageContent>) -> &mut Self { self.content = Some(content); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



