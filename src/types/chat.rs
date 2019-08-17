
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat. (Can be a private chat, basic group, supergroup, or secret chat.) 
#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chat
  /// Chat unique identifier.
  id: Option<i64>,
  /// Type of the chat.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<ChatType>>,
  /// Chat title.
  title: Option<String>,
  /// Chat photo; may be null.
  photo: Option<ChatPhoto>,
  /// Last message in the chat; may be null.
  last_message: Option<Message>,
  /// Descending parameter by which chats are sorted in the main chat list. If the order number of two chats is the same, they must be sorted in descending order by ID. If 0, the position of the chat in the list is undetermined.
  order: Option<String>,
  /// True, if the chat is pinned.
  is_pinned: Option<bool>,
  /// True, if the chat is marked as unread.
  is_marked_as_unread: Option<bool>,
  /// True, if the chat is sponsored by the user's MTProxy server.
  is_sponsored: Option<bool>,
  /// True, if the chat messages can be deleted only for the current user while other users will continue to see the messages.
  can_be_deleted_only_for_self: Option<bool>,
  /// True, if the chat messages can be deleted for all users.
  can_be_deleted_for_all_users: Option<bool>,
  /// True, if the chat can be reported to Telegram moderators through reportChat.
  can_be_reported: Option<bool>,
  /// Default value of the disable_notification parameter, used when a message is sent to the chat.
  default_disable_notification: Option<bool>,
  /// Number of unread messages in the chat.
  unread_count: Option<i32>,
  /// Identifier of the last read incoming message.
  last_read_inbox_message_id: Option<i64>,
  /// Identifier of the last read outgoing message.
  last_read_outbox_message_id: Option<i64>,
  /// Number of unread messages with a mention/reply in the chat.
  unread_mention_count: Option<i32>,
  /// Notification settings for this chat.
  notification_settings: Option<ChatNotificationSettings>,
  /// Identifier of the pinned message in the chat; 0 if none.
  pinned_message_id: Option<i64>,
  /// Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat.
  reply_markup_message_id: Option<i64>,
  /// A draft of a message in the chat; may be null.
  draft_message: Option<DraftMessage>,
  /// Contains client-specific data associated with the chat. (For example, the chat position or local chat notification settings can be stored here.) Persistent if a message database is used.
  client_data: Option<String>,
  
}


impl Clone for Chat {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for Chat {}
impl RObject for Chat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chat" }
  fn td_type(&self) -> RTDType { RTDType::Chat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Chat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chat".to_string(),
      id: None,
      type_: None,
      title: None,
      photo: None,
      last_message: None,
      order: None,
      is_pinned: None,
      is_marked_as_unread: None,
      is_sponsored: None,
      can_be_deleted_only_for_self: None,
      can_be_deleted_for_all_users: None,
      can_be_reported: None,
      default_disable_notification: None,
      unread_count: None,
      last_read_inbox_message_id: None,
      last_read_outbox_message_id: None,
      unread_mention_count: None,
      notification_settings: None,
      pinned_message_id: None,
      reply_markup_message_id: None,
      draft_message: None,
      client_data: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn type_(&self) -> Option<Box<ChatType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<ChatType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn photo(&self) -> Option<ChatPhoto> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: ChatPhoto) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn last_message(&self) -> Option<Message> { self.last_message.clone() }
  #[doc(hidden)] pub fn _set_last_message(&mut self, last_message: Message) -> &mut Self { self.last_message = Some(last_message); self }
  
  pub fn order(&self) -> Option<String> { self.order.clone() }
  #[doc(hidden)] pub fn _set_order(&mut self, order: String) -> &mut Self { self.order = Some(order); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn is_marked_as_unread(&self) -> Option<bool> { self.is_marked_as_unread.clone() }
  #[doc(hidden)] pub fn _set_is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self { self.is_marked_as_unread = Some(is_marked_as_unread); self }
  
  pub fn is_sponsored(&self) -> Option<bool> { self.is_sponsored.clone() }
  #[doc(hidden)] pub fn _set_is_sponsored(&mut self, is_sponsored: bool) -> &mut Self { self.is_sponsored = Some(is_sponsored); self }
  
  pub fn can_be_deleted_only_for_self(&self) -> Option<bool> { self.can_be_deleted_only_for_self.clone() }
  #[doc(hidden)] pub fn _set_can_be_deleted_only_for_self(&mut self, can_be_deleted_only_for_self: bool) -> &mut Self { self.can_be_deleted_only_for_self = Some(can_be_deleted_only_for_self); self }
  
  pub fn can_be_deleted_for_all_users(&self) -> Option<bool> { self.can_be_deleted_for_all_users.clone() }
  #[doc(hidden)] pub fn _set_can_be_deleted_for_all_users(&mut self, can_be_deleted_for_all_users: bool) -> &mut Self { self.can_be_deleted_for_all_users = Some(can_be_deleted_for_all_users); self }
  
  pub fn can_be_reported(&self) -> Option<bool> { self.can_be_reported.clone() }
  #[doc(hidden)] pub fn _set_can_be_reported(&mut self, can_be_reported: bool) -> &mut Self { self.can_be_reported = Some(can_be_reported); self }
  
  pub fn default_disable_notification(&self) -> Option<bool> { self.default_disable_notification.clone() }
  #[doc(hidden)] pub fn _set_default_disable_notification(&mut self, default_disable_notification: bool) -> &mut Self { self.default_disable_notification = Some(default_disable_notification); self }
  
  pub fn unread_count(&self) -> Option<i32> { self.unread_count.clone() }
  #[doc(hidden)] pub fn _set_unread_count(&mut self, unread_count: i32) -> &mut Self { self.unread_count = Some(unread_count); self }
  
  pub fn last_read_inbox_message_id(&self) -> Option<i64> { self.last_read_inbox_message_id.clone() }
  #[doc(hidden)] pub fn _set_last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self { self.last_read_inbox_message_id = Some(last_read_inbox_message_id); self }
  
  pub fn last_read_outbox_message_id(&self) -> Option<i64> { self.last_read_outbox_message_id.clone() }
  #[doc(hidden)] pub fn _set_last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self { self.last_read_outbox_message_id = Some(last_read_outbox_message_id); self }
  
  pub fn unread_mention_count(&self) -> Option<i32> { self.unread_mention_count.clone() }
  #[doc(hidden)] pub fn _set_unread_mention_count(&mut self, unread_mention_count: i32) -> &mut Self { self.unread_mention_count = Some(unread_mention_count); self }
  
  pub fn notification_settings(&self) -> Option<ChatNotificationSettings> { self.notification_settings.clone() }
  #[doc(hidden)] pub fn _set_notification_settings(&mut self, notification_settings: ChatNotificationSettings) -> &mut Self { self.notification_settings = Some(notification_settings); self }
  
  pub fn pinned_message_id(&self) -> Option<i64> { self.pinned_message_id.clone() }
  #[doc(hidden)] pub fn _set_pinned_message_id(&mut self, pinned_message_id: i64) -> &mut Self { self.pinned_message_id = Some(pinned_message_id); self }
  
  pub fn reply_markup_message_id(&self) -> Option<i64> { self.reply_markup_message_id.clone() }
  #[doc(hidden)] pub fn _set_reply_markup_message_id(&mut self, reply_markup_message_id: i64) -> &mut Self { self.reply_markup_message_id = Some(reply_markup_message_id); self }
  
  pub fn draft_message(&self) -> Option<DraftMessage> { self.draft_message.clone() }
  #[doc(hidden)] pub fn _set_draft_message(&mut self, draft_message: DraftMessage) -> &mut Self { self.draft_message = Some(draft_message); self }
  
  pub fn client_data(&self) -> Option<String> { self.client_data.clone() }
  #[doc(hidden)] pub fn _set_client_data(&mut self, client_data: String) -> &mut Self { self.client_data = Some(client_data); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



