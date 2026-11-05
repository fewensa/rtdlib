use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a chat invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the invite link; 0 if the user has no access to the chat before joining
  chat_id: i64,
  /// If non-zero, the amount of time for which read access to the chat will remain available, in seconds
  accessible_for: i64,
  /// Type of the chat
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: InviteLinkChatType,
  /// Title of the chat
  title: String,
  /// Chat photo; may be null
  photo: Option<ChatPhotoInfo>,
  /// Identifier of the accent color for chat title and background of chat photo
  accent_color_id: i64,
  /// Contains information about a chat invite link
  description: String,
  /// Number of members in the chat
  member_count: i64,
  /// User identifiers of some chat members that may be known to the current user
  member_user_ids: Vec<i64>,
  /// Information about subscription plan that must be paid by the user to use the link; may be null if the link doesn't require subscription
  subscription_info: Option<ChatInviteLinkSubscriptionInfo>,
  /// True, if the link only creates join request
  creates_join_request: bool,
  /// True, if the chat is a public supergroup or channel, i.e. it has a username or it is a location-based supergroup
  is_public: bool,
  /// Information about verification status of the chat; may be null if none
  verification_status: Option<VerificationStatus>,
}

impl RObject for ChatInviteLinkInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatInviteLinkInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatInviteLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatInviteLinkInfoBuilder {
    let inner = ChatInviteLinkInfo {
      td_name: "chatInviteLinkInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatInviteLinkInfoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn accessible_for(&self) -> i64 {
    self.accessible_for
  }

  pub fn type_(&self) -> &InviteLinkChatType {
    &self.type_
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn photo(&self) -> &Option<ChatPhotoInfo> {
    &self.photo
  }

  pub fn accent_color_id(&self) -> i64 {
    self.accent_color_id
  }

  pub fn description(&self) -> &String {
    &self.description
  }

  pub fn member_count(&self) -> i64 {
    self.member_count
  }

  pub fn member_user_ids(&self) -> &Vec<i64> {
    &self.member_user_ids
  }

  pub fn subscription_info(&self) -> &Option<ChatInviteLinkSubscriptionInfo> {
    &self.subscription_info
  }

  pub fn creates_join_request(&self) -> bool {
    self.creates_join_request
  }

  pub fn is_public(&self) -> bool {
    self.is_public
  }

  pub fn verification_status(&self) -> &Option<VerificationStatus> {
    &self.verification_status
  }
}

#[doc(hidden)]
pub struct RTDChatInviteLinkInfoBuilder {
  inner: ChatInviteLinkInfo,
}

impl RTDChatInviteLinkInfoBuilder {
  pub fn build(&self) -> ChatInviteLinkInfo {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn accessible_for(&mut self, accessible_for: i64) -> &mut Self {
    self.inner.accessible_for = accessible_for;
    self
  }

  pub fn type_<T: AsRef<InviteLinkChatType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn photo<T: AsRef<ChatPhotoInfo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn accent_color_id(&mut self, accent_color_id: i64) -> &mut Self {
    self.inner.accent_color_id = accent_color_id;
    self
  }

  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

  pub fn member_count(&mut self, member_count: i64) -> &mut Self {
    self.inner.member_count = member_count;
    self
  }

  pub fn member_user_ids(&mut self, member_user_ids: Vec<i64>) -> &mut Self {
    self.inner.member_user_ids = member_user_ids;
    self
  }

  pub fn subscription_info<T: AsRef<ChatInviteLinkSubscriptionInfo>>(
    &mut self,
    subscription_info: T,
  ) -> &mut Self {
    self.inner.subscription_info = Some(subscription_info.as_ref().clone());
    self
  }

  pub fn creates_join_request(&mut self, creates_join_request: bool) -> &mut Self {
    self.inner.creates_join_request = creates_join_request;
    self
  }

  pub fn is_public(&mut self, is_public: bool) -> &mut Self {
    self.inner.is_public = is_public;
    self
  }

  pub fn verification_status<T: AsRef<VerificationStatus>>(
    &mut self,
    verification_status: T,
  ) -> &mut Self {
    self.inner.verification_status = Some(verification_status.as_ref().clone());
    self
  }
}

impl AsRef<ChatInviteLinkInfo> for ChatInviteLinkInfo {
  fn as_ref(&self) -> &ChatInviteLinkInfo {
    self
  }
}

impl AsRef<ChatInviteLinkInfo> for RTDChatInviteLinkInfoBuilder {
  fn as_ref(&self) -> &ChatInviteLinkInfo {
    &self.inner
  }
}
