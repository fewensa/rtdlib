
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes actions which should be possible to do through a chat action bar
pub trait TDChatActionBar: Debug + RObject {}

/// Describes actions which should be possible to do through a chat action bar
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatActionBar {
  #[doc(hidden)] _Default(()),
  /// The chat is a private or secret chat and the other user can be added to the contact list using the method addContact
  AddContact(ChatActionBarAddContact),
  /// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be added to the contact list using the method addContact, or the other user can be blocked using the method blockUser
  ReportAddBlock(ChatActionBarReportAddBlock),
  /// The chat can be reported as spam using the method reportChat with the reason chatReportReasonSpam
  ReportSpam(ChatActionBarReportSpam),
  /// The chat is a location-based supergroup, which can be reported as having unrelated location using the method reportChat with the reason chatReportReasonUnrelatedLocation
  ReportUnrelatedLocation(ChatActionBarReportUnrelatedLocation),
  /// The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber
  SharePhoneNumber(ChatActionBarSharePhoneNumber),

}

impl Default for ChatActionBar {
  fn default() -> Self { ChatActionBar::_Default(()) }
}

impl<'de> Deserialize<'de> for ChatActionBar {
  fn deserialize<D>(deserializer: D) -> Result<ChatActionBar, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatActionBar,
      (chatActionBarAddContact, AddContact);
      (chatActionBarReportAddBlock, ReportAddBlock);
      (chatActionBarReportSpam, ReportSpam);
      (chatActionBarReportUnrelatedLocation, ReportUnrelatedLocation);
      (chatActionBarSharePhoneNumber, SharePhoneNumber);

    )(deserializer)
  }
}

impl RObject for ChatActionBar {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ChatActionBar::AddContact(t) => t.td_name(),
      ChatActionBar::ReportAddBlock(t) => t.td_name(),
      ChatActionBar::ReportSpam(t) => t.td_name(),
      ChatActionBar::ReportUnrelatedLocation(t) => t.td_name(),
      ChatActionBar::SharePhoneNumber(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      ChatActionBar::AddContact(t) => t.extra(),
      ChatActionBar::ReportAddBlock(t) => t.extra(),
      ChatActionBar::ReportSpam(t) => t.extra(),
      ChatActionBar::ReportUnrelatedLocation(t) => t.extra(),
      ChatActionBar::SharePhoneNumber(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ChatActionBar {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ChatActionBar::_Default(_) = self { true } else { false } }

  pub fn is_add_contact(&self) -> bool { if let ChatActionBar::AddContact(_) = self { true } else { false } }
  pub fn is_report_add_block(&self) -> bool { if let ChatActionBar::ReportAddBlock(_) = self { true } else { false } }
  pub fn is_report_spam(&self) -> bool { if let ChatActionBar::ReportSpam(_) = self { true } else { false } }
  pub fn is_report_unrelated_location(&self) -> bool { if let ChatActionBar::ReportUnrelatedLocation(_) = self { true } else { false } }
  pub fn is_share_phone_number(&self) -> bool { if let ChatActionBar::SharePhoneNumber(_) = self { true } else { false } }

  pub fn on_add_contact<F: FnOnce(&ChatActionBarAddContact)>(&self, fnc: F) -> &Self { if let ChatActionBar::AddContact(t) = self { fnc(t) }; self }
  pub fn on_report_add_block<F: FnOnce(&ChatActionBarReportAddBlock)>(&self, fnc: F) -> &Self { if let ChatActionBar::ReportAddBlock(t) = self { fnc(t) }; self }
  pub fn on_report_spam<F: FnOnce(&ChatActionBarReportSpam)>(&self, fnc: F) -> &Self { if let ChatActionBar::ReportSpam(t) = self { fnc(t) }; self }
  pub fn on_report_unrelated_location<F: FnOnce(&ChatActionBarReportUnrelatedLocation)>(&self, fnc: F) -> &Self { if let ChatActionBar::ReportUnrelatedLocation(t) = self { fnc(t) }; self }
  pub fn on_share_phone_number<F: FnOnce(&ChatActionBarSharePhoneNumber)>(&self, fnc: F) -> &Self { if let ChatActionBar::SharePhoneNumber(t) = self { fnc(t) }; self }

  pub fn as_add_contact(&self) -> Option<&ChatActionBarAddContact> { if let ChatActionBar::AddContact(t) = self { return Some(t) } None }
  pub fn as_report_add_block(&self) -> Option<&ChatActionBarReportAddBlock> { if let ChatActionBar::ReportAddBlock(t) = self { return Some(t) } None }
  pub fn as_report_spam(&self) -> Option<&ChatActionBarReportSpam> { if let ChatActionBar::ReportSpam(t) = self { return Some(t) } None }
  pub fn as_report_unrelated_location(&self) -> Option<&ChatActionBarReportUnrelatedLocation> { if let ChatActionBar::ReportUnrelatedLocation(t) = self { return Some(t) } None }
  pub fn as_share_phone_number(&self) -> Option<&ChatActionBarSharePhoneNumber> { if let ChatActionBar::SharePhoneNumber(t) = self { return Some(t) } None }



  pub fn add_contact<T: AsRef<ChatActionBarAddContact>>(t: T) -> Self { ChatActionBar::AddContact(t.as_ref().clone()) }

  pub fn report_add_block<T: AsRef<ChatActionBarReportAddBlock>>(t: T) -> Self { ChatActionBar::ReportAddBlock(t.as_ref().clone()) }

  pub fn report_spam<T: AsRef<ChatActionBarReportSpam>>(t: T) -> Self { ChatActionBar::ReportSpam(t.as_ref().clone()) }

  pub fn report_unrelated_location<T: AsRef<ChatActionBarReportUnrelatedLocation>>(t: T) -> Self { ChatActionBar::ReportUnrelatedLocation(t.as_ref().clone()) }

  pub fn share_phone_number<T: AsRef<ChatActionBarSharePhoneNumber>>(t: T) -> Self { ChatActionBar::SharePhoneNumber(t.as_ref().clone()) }

}

impl AsRef<ChatActionBar> for ChatActionBar {
  fn as_ref(&self) -> &ChatActionBar { self }
}







/// The chat is a private or secret chat and the other user can be added to the contact list using the method addContact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarAddContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionBarAddContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionBarAddContact" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatActionBar for ChatActionBarAddContact {}



impl ChatActionBarAddContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionBarAddContactBuilder {
    let mut inner = ChatActionBarAddContact::default();
    inner.td_name = "chatActionBarAddContact".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionBarAddContactBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionBarAddContactBuilder {
  inner: ChatActionBarAddContact
}

impl RTDChatActionBarAddContactBuilder {
  pub fn build(&self) -> ChatActionBarAddContact { self.inner.clone() }

}

impl AsRef<ChatActionBarAddContact> for ChatActionBarAddContact {
  fn as_ref(&self) -> &ChatActionBarAddContact { self }
}

impl AsRef<ChatActionBarAddContact> for RTDChatActionBarAddContactBuilder {
  fn as_ref(&self) -> &ChatActionBarAddContact { &self.inner }
}







/// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be added to the contact list using the method addContact, or the other user can be blocked using the method blockUser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarReportAddBlock {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionBarReportAddBlock {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionBarReportAddBlock" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatActionBar for ChatActionBarReportAddBlock {}



impl ChatActionBarReportAddBlock {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionBarReportAddBlockBuilder {
    let mut inner = ChatActionBarReportAddBlock::default();
    inner.td_name = "chatActionBarReportAddBlock".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionBarReportAddBlockBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionBarReportAddBlockBuilder {
  inner: ChatActionBarReportAddBlock
}

impl RTDChatActionBarReportAddBlockBuilder {
  pub fn build(&self) -> ChatActionBarReportAddBlock { self.inner.clone() }

}

impl AsRef<ChatActionBarReportAddBlock> for ChatActionBarReportAddBlock {
  fn as_ref(&self) -> &ChatActionBarReportAddBlock { self }
}

impl AsRef<ChatActionBarReportAddBlock> for RTDChatActionBarReportAddBlockBuilder {
  fn as_ref(&self) -> &ChatActionBarReportAddBlock { &self.inner }
}







/// The chat can be reported as spam using the method reportChat with the reason chatReportReasonSpam
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarReportSpam {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionBarReportSpam {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionBarReportSpam" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatActionBar for ChatActionBarReportSpam {}



impl ChatActionBarReportSpam {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionBarReportSpamBuilder {
    let mut inner = ChatActionBarReportSpam::default();
    inner.td_name = "chatActionBarReportSpam".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionBarReportSpamBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionBarReportSpamBuilder {
  inner: ChatActionBarReportSpam
}

impl RTDChatActionBarReportSpamBuilder {
  pub fn build(&self) -> ChatActionBarReportSpam { self.inner.clone() }

}

impl AsRef<ChatActionBarReportSpam> for ChatActionBarReportSpam {
  fn as_ref(&self) -> &ChatActionBarReportSpam { self }
}

impl AsRef<ChatActionBarReportSpam> for RTDChatActionBarReportSpamBuilder {
  fn as_ref(&self) -> &ChatActionBarReportSpam { &self.inner }
}







/// The chat is a location-based supergroup, which can be reported as having unrelated location using the method reportChat with the reason chatReportReasonUnrelatedLocation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarReportUnrelatedLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionBarReportUnrelatedLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionBarReportUnrelatedLocation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatActionBar for ChatActionBarReportUnrelatedLocation {}



impl ChatActionBarReportUnrelatedLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionBarReportUnrelatedLocationBuilder {
    let mut inner = ChatActionBarReportUnrelatedLocation::default();
    inner.td_name = "chatActionBarReportUnrelatedLocation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionBarReportUnrelatedLocationBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionBarReportUnrelatedLocationBuilder {
  inner: ChatActionBarReportUnrelatedLocation
}

impl RTDChatActionBarReportUnrelatedLocationBuilder {
  pub fn build(&self) -> ChatActionBarReportUnrelatedLocation { self.inner.clone() }

}

impl AsRef<ChatActionBarReportUnrelatedLocation> for ChatActionBarReportUnrelatedLocation {
  fn as_ref(&self) -> &ChatActionBarReportUnrelatedLocation { self }
}

impl AsRef<ChatActionBarReportUnrelatedLocation> for RTDChatActionBarReportUnrelatedLocationBuilder {
  fn as_ref(&self) -> &ChatActionBarReportUnrelatedLocation { &self.inner }
}







/// The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarSharePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionBarSharePhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionBarSharePhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatActionBar for ChatActionBarSharePhoneNumber {}



impl ChatActionBarSharePhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionBarSharePhoneNumberBuilder {
    let mut inner = ChatActionBarSharePhoneNumber::default();
    inner.td_name = "chatActionBarSharePhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionBarSharePhoneNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionBarSharePhoneNumberBuilder {
  inner: ChatActionBarSharePhoneNumber
}

impl RTDChatActionBarSharePhoneNumberBuilder {
  pub fn build(&self) -> ChatActionBarSharePhoneNumber { self.inner.clone() }

}

impl AsRef<ChatActionBarSharePhoneNumber> for ChatActionBarSharePhoneNumber {
  fn as_ref(&self) -> &ChatActionBarSharePhoneNumber { self }
}

impl AsRef<ChatActionBarSharePhoneNumber> for RTDChatActionBarSharePhoneNumberBuilder {
  fn as_ref(&self) -> &ChatActionBarSharePhoneNumber { &self.inner }
}



