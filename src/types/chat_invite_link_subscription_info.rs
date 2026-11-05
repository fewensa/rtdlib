use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about subscription plan that must be paid by the user to use a chat invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinkSubscriptionInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about subscription plan that must be paid by the user to use the link
  pricing: StarSubscriptionPricing,
  /// True, if the user has already paid for the subscription and can use joinChatByInviteLink to join the subscribed chat again
  can_reuse: bool,
  /// Identifier of the payment form to use for subscription payment; 0 if the subscription can't be paid
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  form_id: isize,
}

impl RObject for ChatInviteLinkSubscriptionInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatInviteLinkSubscriptionInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatInviteLinkSubscriptionInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatInviteLinkSubscriptionInfoBuilder {
    let inner = ChatInviteLinkSubscriptionInfo {
      td_name: "chatInviteLinkSubscriptionInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatInviteLinkSubscriptionInfoBuilder { inner }
  }

  pub fn pricing(&self) -> &StarSubscriptionPricing {
    &self.pricing
  }

  pub fn can_reuse(&self) -> bool {
    self.can_reuse
  }

  pub fn form_id(&self) -> isize {
    self.form_id
  }
}

#[doc(hidden)]
pub struct RTDChatInviteLinkSubscriptionInfoBuilder {
  inner: ChatInviteLinkSubscriptionInfo,
}

impl RTDChatInviteLinkSubscriptionInfoBuilder {
  pub fn build(&self) -> ChatInviteLinkSubscriptionInfo {
    self.inner.clone()
  }

  pub fn pricing<T: AsRef<StarSubscriptionPricing>>(&mut self, pricing: T) -> &mut Self {
    self.inner.pricing = pricing.as_ref().clone();
    self
  }

  pub fn can_reuse(&mut self, can_reuse: bool) -> &mut Self {
    self.inner.can_reuse = can_reuse;
    self
  }

  pub fn form_id(&mut self, form_id: isize) -> &mut Self {
    self.inner.form_id = form_id;
    self
  }
}

impl AsRef<ChatInviteLinkSubscriptionInfo> for ChatInviteLinkSubscriptionInfo {
  fn as_ref(&self) -> &ChatInviteLinkSubscriptionInfo {
    self
  }
}

impl AsRef<ChatInviteLinkSubscriptionInfo> for RTDChatInviteLinkSubscriptionInfoBuilder {
  fn as_ref(&self) -> &ChatInviteLinkSubscriptionInfo {
    &self.inner
  }
}
