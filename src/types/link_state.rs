
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the relationship between user A and user B. For incoming_link, user A is the current user; for outgoing_link, user B is the current user
pub trait TDLinkState: Debug + RObject {}

/// Represents the relationship between user A and user B. For incoming_link, user A is the current user; for outgoing_link, user B is the current user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LinkState {
  #[doc(hidden)] _Default(()),
  /// The phone number of user A has been saved to the contact list of user B
  IsContact(LinkStateIsContact),
  /// The phone number of user A is known but that number has not been saved to the contact list of user B
  KnowsPhoneNumber(LinkStateKnowsPhoneNumber),
  /// The phone number of user A is not known to user B
  None(LinkStateNone),

}

impl Default for LinkState {
  fn default() -> Self { LinkState::_Default(()) }
}

impl<'de> Deserialize<'de> for LinkState {
  fn deserialize<D>(deserializer: D) -> Result<LinkState, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      LinkState,
      (linkStateIsContact, IsContact);
      (linkStateKnowsPhoneNumber, KnowsPhoneNumber);
      (linkStateNone, None);

    )(deserializer)
  }
}

impl RObject for LinkState {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      LinkState::IsContact(t) => t.td_name(),
      LinkState::KnowsPhoneNumber(t) => t.td_name(),
      LinkState::None(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      LinkState::IsContact(t) => t.extra(),
      LinkState::KnowsPhoneNumber(t) => t.extra(),
      LinkState::None(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl LinkState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let LinkState::_Default(_) = self { true } else { false } }

  pub fn is_is_contact(&self) -> bool { if let LinkState::IsContact(_) = self { true } else { false } }
  pub fn is_knows_phone_number(&self) -> bool { if let LinkState::KnowsPhoneNumber(_) = self { true } else { false } }
  pub fn is_none(&self) -> bool { if let LinkState::None(_) = self { true } else { false } }

  pub fn on_is_contact<F: FnOnce(&LinkStateIsContact)>(&self, fnc: F) -> &Self { if let LinkState::IsContact(t) = self { fnc(t) }; self }
  pub fn on_knows_phone_number<F: FnOnce(&LinkStateKnowsPhoneNumber)>(&self, fnc: F) -> &Self { if let LinkState::KnowsPhoneNumber(t) = self { fnc(t) }; self }
  pub fn on_none<F: FnOnce(&LinkStateNone)>(&self, fnc: F) -> &Self { if let LinkState::None(t) = self { fnc(t) }; self }

  pub fn as_is_contact(&self) -> Option<&LinkStateIsContact> { if let LinkState::IsContact(t) = self { return Some(t) } None }
  pub fn as_knows_phone_number(&self) -> Option<&LinkStateKnowsPhoneNumber> { if let LinkState::KnowsPhoneNumber(t) = self { return Some(t) } None }
  pub fn as_none(&self) -> Option<&LinkStateNone> { if let LinkState::None(t) = self { return Some(t) } None }



  pub fn is_contact<T: AsRef<LinkStateIsContact>>(t: T) -> Self { LinkState::IsContact(t.as_ref().clone()) }

  pub fn knows_phone_number<T: AsRef<LinkStateKnowsPhoneNumber>>(t: T) -> Self { LinkState::KnowsPhoneNumber(t.as_ref().clone()) }

  pub fn none<T: AsRef<LinkStateNone>>(t: T) -> Self { LinkState::None(t.as_ref().clone()) }

}

impl AsRef<LinkState> for LinkState {
  fn as_ref(&self) -> &LinkState { self }
}







/// The phone number of user A has been saved to the contact list of user B
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkStateIsContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for LinkStateIsContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "linkStateIsContact" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLinkState for LinkStateIsContact {}



impl LinkStateIsContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLinkStateIsContactBuilder {
    let mut inner = LinkStateIsContact::default();
    inner.td_name = "linkStateIsContact".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLinkStateIsContactBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDLinkStateIsContactBuilder {
  inner: LinkStateIsContact
}

impl RTDLinkStateIsContactBuilder {
  pub fn build(&self) -> LinkStateIsContact { self.inner.clone() }

}

impl AsRef<LinkStateIsContact> for LinkStateIsContact {
  fn as_ref(&self) -> &LinkStateIsContact { self }
}

impl AsRef<LinkStateIsContact> for RTDLinkStateIsContactBuilder {
  fn as_ref(&self) -> &LinkStateIsContact { &self.inner }
}







/// The phone number of user A is known but that number has not been saved to the contact list of user B
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkStateKnowsPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for LinkStateKnowsPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "linkStateKnowsPhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLinkState for LinkStateKnowsPhoneNumber {}



impl LinkStateKnowsPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLinkStateKnowsPhoneNumberBuilder {
    let mut inner = LinkStateKnowsPhoneNumber::default();
    inner.td_name = "linkStateKnowsPhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLinkStateKnowsPhoneNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDLinkStateKnowsPhoneNumberBuilder {
  inner: LinkStateKnowsPhoneNumber
}

impl RTDLinkStateKnowsPhoneNumberBuilder {
  pub fn build(&self) -> LinkStateKnowsPhoneNumber { self.inner.clone() }

}

impl AsRef<LinkStateKnowsPhoneNumber> for LinkStateKnowsPhoneNumber {
  fn as_ref(&self) -> &LinkStateKnowsPhoneNumber { self }
}

impl AsRef<LinkStateKnowsPhoneNumber> for RTDLinkStateKnowsPhoneNumberBuilder {
  fn as_ref(&self) -> &LinkStateKnowsPhoneNumber { &self.inner }
}







/// The phone number of user A is not known to user B
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkStateNone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for LinkStateNone {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "linkStateNone" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLinkState for LinkStateNone {}



impl LinkStateNone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLinkStateNoneBuilder {
    let mut inner = LinkStateNone::default();
    inner.td_name = "linkStateNone".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLinkStateNoneBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDLinkStateNoneBuilder {
  inner: LinkStateNone
}

impl RTDLinkStateNoneBuilder {
  pub fn build(&self) -> LinkStateNone { self.inner.clone() }

}

impl AsRef<LinkStateNone> for LinkStateNone {
  fn as_ref(&self) -> &LinkStateNone { self }
}

impl AsRef<LinkStateNone> for RTDLinkStateNoneBuilder {
  fn as_ref(&self) -> &LinkStateNone { &self.inner }
}



