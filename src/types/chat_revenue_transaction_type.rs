use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of transaction for revenue earned from sponsored messages in a chat
pub trait TDChatRevenueTransactionType: Debug + RObject {}

/// Describes type of transaction for revenue earned from sponsored messages in a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatRevenueTransactionType {
  #[doc(hidden)]
  _Default(()),
  /// Describes a refund for failed withdrawal of earnings through Fragment
  FragmentRefund(ChatRevenueTransactionTypeFragmentRefund),
  /// Describes a withdrawal of earnings through Fragment
  FragmentWithdrawal(ChatRevenueTransactionTypeFragmentWithdrawal),
  /// Describes earnings from sponsored messages in a chat in some time frame
  SponsoredMessageEarnings(ChatRevenueTransactionTypeSponsoredMessageEarnings),
  /// Describes earnings from a published suggested post
  SuggestedPostEarnings(ChatRevenueTransactionTypeSuggestedPostEarnings),
  /// Describes an unsupported transaction
  Unsupported(ChatRevenueTransactionTypeUnsupported),
}

impl Default for ChatRevenueTransactionType {
  fn default() -> Self {
    ChatRevenueTransactionType::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatRevenueTransactionType {
  fn deserialize<D>(deserializer: D) -> Result<ChatRevenueTransactionType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatRevenueTransactionType,
      (chatRevenueTransactionTypeFragmentRefund, FragmentRefund);
      (chatRevenueTransactionTypeFragmentWithdrawal, FragmentWithdrawal);
      (chatRevenueTransactionTypeSponsoredMessageEarnings, SponsoredMessageEarnings);
      (chatRevenueTransactionTypeSuggestedPostEarnings, SuggestedPostEarnings);
      (chatRevenueTransactionTypeUnsupported, Unsupported);

    )(deserializer)
  }
}

impl RObject for ChatRevenueTransactionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatRevenueTransactionType::FragmentRefund(t) => t.td_name(),
      ChatRevenueTransactionType::FragmentWithdrawal(t) => t.td_name(),
      ChatRevenueTransactionType::SponsoredMessageEarnings(t) => t.td_name(),
      ChatRevenueTransactionType::SuggestedPostEarnings(t) => t.td_name(),
      ChatRevenueTransactionType::Unsupported(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatRevenueTransactionType::FragmentRefund(t) => t.extra(),
      ChatRevenueTransactionType::FragmentWithdrawal(t) => t.extra(),
      ChatRevenueTransactionType::SponsoredMessageEarnings(t) => t.extra(),
      ChatRevenueTransactionType::SuggestedPostEarnings(t) => t.extra(),
      ChatRevenueTransactionType::Unsupported(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatRevenueTransactionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatRevenueTransactionType::_Default(_))
  }

  pub fn is_fragment_refund(&self) -> bool {
    matches!(self, ChatRevenueTransactionType::FragmentRefund(_))
  }
  pub fn is_fragment_withdrawal(&self) -> bool {
    matches!(self, ChatRevenueTransactionType::FragmentWithdrawal(_))
  }
  pub fn is_sponsored_message_earnings(&self) -> bool {
    matches!(
      self,
      ChatRevenueTransactionType::SponsoredMessageEarnings(_)
    )
  }
  pub fn is_suggested_post_earnings(&self) -> bool {
    matches!(self, ChatRevenueTransactionType::SuggestedPostEarnings(_))
  }
  pub fn is_unsupported(&self) -> bool {
    matches!(self, ChatRevenueTransactionType::Unsupported(_))
  }

  pub fn on_fragment_refund<F: FnOnce(&ChatRevenueTransactionTypeFragmentRefund)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ChatRevenueTransactionType::FragmentRefund(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fragment_withdrawal<F: FnOnce(&ChatRevenueTransactionTypeFragmentWithdrawal)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ChatRevenueTransactionType::FragmentWithdrawal(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_sponsored_message_earnings<
    F: FnOnce(&ChatRevenueTransactionTypeSponsoredMessageEarnings),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let ChatRevenueTransactionType::SponsoredMessageEarnings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_suggested_post_earnings<F: FnOnce(&ChatRevenueTransactionTypeSuggestedPostEarnings)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ChatRevenueTransactionType::SuggestedPostEarnings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unsupported<F: FnOnce(&ChatRevenueTransactionTypeUnsupported)>(&self, fnc: F) -> &Self {
    if let ChatRevenueTransactionType::Unsupported(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_fragment_refund(&self) -> Option<&ChatRevenueTransactionTypeFragmentRefund> {
    if let ChatRevenueTransactionType::FragmentRefund(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fragment_withdrawal(&self) -> Option<&ChatRevenueTransactionTypeFragmentWithdrawal> {
    if let ChatRevenueTransactionType::FragmentWithdrawal(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_sponsored_message_earnings(
    &self,
  ) -> Option<&ChatRevenueTransactionTypeSponsoredMessageEarnings> {
    if let ChatRevenueTransactionType::SponsoredMessageEarnings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_suggested_post_earnings(
    &self,
  ) -> Option<&ChatRevenueTransactionTypeSuggestedPostEarnings> {
    if let ChatRevenueTransactionType::SuggestedPostEarnings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unsupported(&self) -> Option<&ChatRevenueTransactionTypeUnsupported> {
    if let ChatRevenueTransactionType::Unsupported(t) = self {
      return Some(t);
    }
    None
  }

  pub fn fragment_refund<T: AsRef<ChatRevenueTransactionTypeFragmentRefund>>(t: T) -> Self {
    ChatRevenueTransactionType::FragmentRefund(t.as_ref().clone())
  }

  pub fn fragment_withdrawal<T: AsRef<ChatRevenueTransactionTypeFragmentWithdrawal>>(t: T) -> Self {
    ChatRevenueTransactionType::FragmentWithdrawal(t.as_ref().clone())
  }

  pub fn sponsored_message_earnings<
    T: AsRef<ChatRevenueTransactionTypeSponsoredMessageEarnings>,
  >(
    t: T,
  ) -> Self {
    ChatRevenueTransactionType::SponsoredMessageEarnings(t.as_ref().clone())
  }

  pub fn suggested_post_earnings<T: AsRef<ChatRevenueTransactionTypeSuggestedPostEarnings>>(
    t: T,
  ) -> Self {
    ChatRevenueTransactionType::SuggestedPostEarnings(t.as_ref().clone())
  }

  pub fn unsupported<T: AsRef<ChatRevenueTransactionTypeUnsupported>>(t: T) -> Self {
    ChatRevenueTransactionType::Unsupported(t.as_ref().clone())
  }
}

impl AsRef<ChatRevenueTransactionType> for ChatRevenueTransactionType {
  fn as_ref(&self) -> &ChatRevenueTransactionType {
    self
  }
}

/// Describes a refund for failed withdrawal of earnings through Fragment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatRevenueTransactionTypeFragmentRefund {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the transaction was refunded
  refund_date: i64,
}

impl RObject for ChatRevenueTransactionTypeFragmentRefund {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatRevenueTransactionTypeFragmentRefund"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatRevenueTransactionType for ChatRevenueTransactionTypeFragmentRefund {}

impl ChatRevenueTransactionTypeFragmentRefund {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatRevenueTransactionTypeFragmentRefundBuilder {
    let inner = ChatRevenueTransactionTypeFragmentRefund {
      td_name: "chatRevenueTransactionTypeFragmentRefund".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatRevenueTransactionTypeFragmentRefundBuilder { inner }
  }

  pub fn refund_date(&self) -> i64 {
    self.refund_date
  }
}

#[doc(hidden)]
pub struct RTDChatRevenueTransactionTypeFragmentRefundBuilder {
  inner: ChatRevenueTransactionTypeFragmentRefund,
}

impl RTDChatRevenueTransactionTypeFragmentRefundBuilder {
  pub fn build(&self) -> ChatRevenueTransactionTypeFragmentRefund {
    self.inner.clone()
  }

  pub fn refund_date(&mut self, refund_date: i64) -> &mut Self {
    self.inner.refund_date = refund_date;
    self
  }
}

impl AsRef<ChatRevenueTransactionTypeFragmentRefund> for ChatRevenueTransactionTypeFragmentRefund {
  fn as_ref(&self) -> &ChatRevenueTransactionTypeFragmentRefund {
    self
  }
}

impl AsRef<ChatRevenueTransactionTypeFragmentRefund>
  for RTDChatRevenueTransactionTypeFragmentRefundBuilder
{
  fn as_ref(&self) -> &ChatRevenueTransactionTypeFragmentRefund {
    &self.inner
  }
}

/// Describes a withdrawal of earnings through Fragment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatRevenueTransactionTypeFragmentWithdrawal {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the earnings withdrawal started
  withdrawal_date: i64,
  /// State of the withdrawal
  state: RevenueWithdrawalState,
}

impl RObject for ChatRevenueTransactionTypeFragmentWithdrawal {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatRevenueTransactionTypeFragmentWithdrawal"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatRevenueTransactionType for ChatRevenueTransactionTypeFragmentWithdrawal {}

impl ChatRevenueTransactionTypeFragmentWithdrawal {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatRevenueTransactionTypeFragmentWithdrawalBuilder {
    let inner = ChatRevenueTransactionTypeFragmentWithdrawal {
      td_name: "chatRevenueTransactionTypeFragmentWithdrawal".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatRevenueTransactionTypeFragmentWithdrawalBuilder { inner }
  }

  pub fn withdrawal_date(&self) -> i64 {
    self.withdrawal_date
  }

  pub fn state(&self) -> &RevenueWithdrawalState {
    &self.state
  }
}

#[doc(hidden)]
pub struct RTDChatRevenueTransactionTypeFragmentWithdrawalBuilder {
  inner: ChatRevenueTransactionTypeFragmentWithdrawal,
}

impl RTDChatRevenueTransactionTypeFragmentWithdrawalBuilder {
  pub fn build(&self) -> ChatRevenueTransactionTypeFragmentWithdrawal {
    self.inner.clone()
  }

  pub fn withdrawal_date(&mut self, withdrawal_date: i64) -> &mut Self {
    self.inner.withdrawal_date = withdrawal_date;
    self
  }

  pub fn state<T: AsRef<RevenueWithdrawalState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }
}

impl AsRef<ChatRevenueTransactionTypeFragmentWithdrawal>
  for ChatRevenueTransactionTypeFragmentWithdrawal
{
  fn as_ref(&self) -> &ChatRevenueTransactionTypeFragmentWithdrawal {
    self
  }
}

impl AsRef<ChatRevenueTransactionTypeFragmentWithdrawal>
  for RTDChatRevenueTransactionTypeFragmentWithdrawalBuilder
{
  fn as_ref(&self) -> &ChatRevenueTransactionTypeFragmentWithdrawal {
    &self.inner
  }
}

/// Describes earnings from sponsored messages in a chat in some time frame
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatRevenueTransactionTypeSponsoredMessageEarnings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the earnings started
  start_date: i64,
  /// Point in time (Unix timestamp) when the earnings ended
  end_date: i64,
}

impl RObject for ChatRevenueTransactionTypeSponsoredMessageEarnings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatRevenueTransactionTypeSponsoredMessageEarnings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatRevenueTransactionType for ChatRevenueTransactionTypeSponsoredMessageEarnings {}

impl ChatRevenueTransactionTypeSponsoredMessageEarnings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatRevenueTransactionTypeSponsoredMessageEarningsBuilder {
    let inner = ChatRevenueTransactionTypeSponsoredMessageEarnings {
      td_name: "chatRevenueTransactionTypeSponsoredMessageEarnings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatRevenueTransactionTypeSponsoredMessageEarningsBuilder { inner }
  }

  pub fn start_date(&self) -> i64 {
    self.start_date
  }

  pub fn end_date(&self) -> i64 {
    self.end_date
  }
}

#[doc(hidden)]
pub struct RTDChatRevenueTransactionTypeSponsoredMessageEarningsBuilder {
  inner: ChatRevenueTransactionTypeSponsoredMessageEarnings,
}

impl RTDChatRevenueTransactionTypeSponsoredMessageEarningsBuilder {
  pub fn build(&self) -> ChatRevenueTransactionTypeSponsoredMessageEarnings {
    self.inner.clone()
  }

  pub fn start_date(&mut self, start_date: i64) -> &mut Self {
    self.inner.start_date = start_date;
    self
  }

  pub fn end_date(&mut self, end_date: i64) -> &mut Self {
    self.inner.end_date = end_date;
    self
  }
}

impl AsRef<ChatRevenueTransactionTypeSponsoredMessageEarnings>
  for ChatRevenueTransactionTypeSponsoredMessageEarnings
{
  fn as_ref(&self) -> &ChatRevenueTransactionTypeSponsoredMessageEarnings {
    self
  }
}

impl AsRef<ChatRevenueTransactionTypeSponsoredMessageEarnings>
  for RTDChatRevenueTransactionTypeSponsoredMessageEarningsBuilder
{
  fn as_ref(&self) -> &ChatRevenueTransactionTypeSponsoredMessageEarnings {
    &self.inner
  }
}

/// Describes earnings from a published suggested post
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatRevenueTransactionTypeSuggestedPostEarnings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who paid for the suggested post
  user_id: i64,
}

impl RObject for ChatRevenueTransactionTypeSuggestedPostEarnings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatRevenueTransactionTypeSuggestedPostEarnings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatRevenueTransactionType for ChatRevenueTransactionTypeSuggestedPostEarnings {}

impl ChatRevenueTransactionTypeSuggestedPostEarnings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatRevenueTransactionTypeSuggestedPostEarningsBuilder {
    let inner = ChatRevenueTransactionTypeSuggestedPostEarnings {
      td_name: "chatRevenueTransactionTypeSuggestedPostEarnings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatRevenueTransactionTypeSuggestedPostEarningsBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }
}

#[doc(hidden)]
pub struct RTDChatRevenueTransactionTypeSuggestedPostEarningsBuilder {
  inner: ChatRevenueTransactionTypeSuggestedPostEarnings,
}

impl RTDChatRevenueTransactionTypeSuggestedPostEarningsBuilder {
  pub fn build(&self) -> ChatRevenueTransactionTypeSuggestedPostEarnings {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }
}

impl AsRef<ChatRevenueTransactionTypeSuggestedPostEarnings>
  for ChatRevenueTransactionTypeSuggestedPostEarnings
{
  fn as_ref(&self) -> &ChatRevenueTransactionTypeSuggestedPostEarnings {
    self
  }
}

impl AsRef<ChatRevenueTransactionTypeSuggestedPostEarnings>
  for RTDChatRevenueTransactionTypeSuggestedPostEarningsBuilder
{
  fn as_ref(&self) -> &ChatRevenueTransactionTypeSuggestedPostEarnings {
    &self.inner
  }
}

/// Describes an unsupported transaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatRevenueTransactionTypeUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ChatRevenueTransactionTypeUnsupported {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatRevenueTransactionTypeUnsupported"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatRevenueTransactionType for ChatRevenueTransactionTypeUnsupported {}

impl ChatRevenueTransactionTypeUnsupported {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatRevenueTransactionTypeUnsupportedBuilder {
    let inner = ChatRevenueTransactionTypeUnsupported {
      td_name: "chatRevenueTransactionTypeUnsupported".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDChatRevenueTransactionTypeUnsupportedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDChatRevenueTransactionTypeUnsupportedBuilder {
  inner: ChatRevenueTransactionTypeUnsupported,
}

impl RTDChatRevenueTransactionTypeUnsupportedBuilder {
  pub fn build(&self) -> ChatRevenueTransactionTypeUnsupported {
    self.inner.clone()
  }
}

impl AsRef<ChatRevenueTransactionTypeUnsupported> for ChatRevenueTransactionTypeUnsupported {
  fn as_ref(&self) -> &ChatRevenueTransactionTypeUnsupported {
    self
  }
}

impl AsRef<ChatRevenueTransactionTypeUnsupported>
  for RTDChatRevenueTransactionTypeUnsupportedBuilder
{
  fn as_ref(&self) -> &ChatRevenueTransactionTypeUnsupported {
    &self.inner
  }
}
