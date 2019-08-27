
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a keyboard button type
pub trait TDKeyboardButtonType: Debug + RObject {}

/// Describes a keyboard button type
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum KeyboardButtonType {
  #[doc(hidden)] _Default(()),
  /// A simple button, with text that should be sent when the button is pressed
  Text(KeyboardButtonTypeText),
  /// A button that sends the user's phone number when pressed; available only in private chats
  RequestPhoneNumber(KeyboardButtonTypeRequestPhoneNumber),
  /// A button that sends the user's location when pressed; available only in private chats
  RequestLocation(KeyboardButtonTypeRequestLocation),

}

impl Default for KeyboardButtonType {
  fn default() -> Self { KeyboardButtonType::_Default(()) }
}

impl<'de> Deserialize<'de> for KeyboardButtonType {
  fn deserialize<D>(deserializer: D) -> Result<KeyboardButtonType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      KeyboardButtonType,
      (keyboardButtonTypeText, Text);
      (keyboardButtonTypeRequestPhoneNumber, RequestPhoneNumber);
      (keyboardButtonTypeRequestLocation, RequestLocation);

    )(deserializer)
  }
}

impl RObject for KeyboardButtonType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      KeyboardButtonType::Text(t) => t.td_name(),
      KeyboardButtonType::RequestPhoneNumber(t) => t.td_name(),
      KeyboardButtonType::RequestLocation(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl KeyboardButtonType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let KeyboardButtonType::_Default(_) = self { true } else { false } }

  pub fn is_text(&self) -> bool { if let KeyboardButtonType::Text(_) = self { true } else { false } }
  pub fn is_request_phone_number(&self) -> bool { if let KeyboardButtonType::RequestPhoneNumber(_) = self { true } else { false } }
  pub fn is_request_location(&self) -> bool { if let KeyboardButtonType::RequestLocation(_) = self { true } else { false } }

  pub fn on_text<F: FnOnce(&KeyboardButtonTypeText)>(&self, fnc: F) -> &Self { if let KeyboardButtonType::Text(t) = self { fnc(t) }; self }
  pub fn on_request_phone_number<F: FnOnce(&KeyboardButtonTypeRequestPhoneNumber)>(&self, fnc: F) -> &Self { if let KeyboardButtonType::RequestPhoneNumber(t) = self { fnc(t) }; self }
  pub fn on_request_location<F: FnOnce(&KeyboardButtonTypeRequestLocation)>(&self, fnc: F) -> &Self { if let KeyboardButtonType::RequestLocation(t) = self { fnc(t) }; self }

  pub fn as_text(&self) -> Option<&KeyboardButtonTypeText> { if let KeyboardButtonType::Text(t) = self { return Some(t) } None }
  pub fn as_request_phone_number(&self) -> Option<&KeyboardButtonTypeRequestPhoneNumber> { if let KeyboardButtonType::RequestPhoneNumber(t) = self { return Some(t) } None }
  pub fn as_request_location(&self) -> Option<&KeyboardButtonTypeRequestLocation> { if let KeyboardButtonType::RequestLocation(t) = self { return Some(t) } None }



  pub fn text<T: AsRef<KeyboardButtonTypeText>>(t: T) -> Self { KeyboardButtonType::Text(t.as_ref().clone()) }

  pub fn request_phone_number<T: AsRef<KeyboardButtonTypeRequestPhoneNumber>>(t: T) -> Self { KeyboardButtonType::RequestPhoneNumber(t.as_ref().clone()) }

  pub fn request_location<T: AsRef<KeyboardButtonTypeRequestLocation>>(t: T) -> Self { KeyboardButtonType::RequestLocation(t.as_ref().clone()) }

}

impl AsRef<KeyboardButtonType> for KeyboardButtonType {
  fn as_ref(&self) -> &KeyboardButtonType { self }
}







/// A simple button, with text that should be sent when the button is pressed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for KeyboardButtonTypeText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "keyboardButtonTypeText" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDKeyboardButtonType for KeyboardButtonTypeText {}



impl KeyboardButtonTypeText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDKeyboardButtonTypeTextBuilder {
    let mut inner = KeyboardButtonTypeText::default();
    inner.td_name = "keyboardButtonTypeText".to_string();
    RTDKeyboardButtonTypeTextBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeTextBuilder {
  inner: KeyboardButtonTypeText
}

impl RTDKeyboardButtonTypeTextBuilder {
  pub fn build(&self) -> KeyboardButtonTypeText { self.inner.clone() }

}

impl AsRef<KeyboardButtonTypeText> for KeyboardButtonTypeText {
  fn as_ref(&self) -> &KeyboardButtonTypeText { self }
}

impl AsRef<KeyboardButtonTypeText> for RTDKeyboardButtonTypeTextBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeText { &self.inner }
}







/// A button that sends the user's phone number when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for KeyboardButtonTypeRequestPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "keyboardButtonTypeRequestPhoneNumber" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDKeyboardButtonType for KeyboardButtonTypeRequestPhoneNumber {}



impl KeyboardButtonTypeRequestPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
    let mut inner = KeyboardButtonTypeRequestPhoneNumber::default();
    inner.td_name = "keyboardButtonTypeRequestPhoneNumber".to_string();
    RTDKeyboardButtonTypeRequestPhoneNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
  inner: KeyboardButtonTypeRequestPhoneNumber
}

impl RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
  pub fn build(&self) -> KeyboardButtonTypeRequestPhoneNumber { self.inner.clone() }

}

impl AsRef<KeyboardButtonTypeRequestPhoneNumber> for KeyboardButtonTypeRequestPhoneNumber {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestPhoneNumber { self }
}

impl AsRef<KeyboardButtonTypeRequestPhoneNumber> for RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestPhoneNumber { &self.inner }
}







/// A button that sends the user's location when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for KeyboardButtonTypeRequestLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "keyboardButtonTypeRequestLocation" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDKeyboardButtonType for KeyboardButtonTypeRequestLocation {}



impl KeyboardButtonTypeRequestLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDKeyboardButtonTypeRequestLocationBuilder {
    let mut inner = KeyboardButtonTypeRequestLocation::default();
    inner.td_name = "keyboardButtonTypeRequestLocation".to_string();
    RTDKeyboardButtonTypeRequestLocationBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestLocationBuilder {
  inner: KeyboardButtonTypeRequestLocation
}

impl RTDKeyboardButtonTypeRequestLocationBuilder {
  pub fn build(&self) -> KeyboardButtonTypeRequestLocation { self.inner.clone() }

}

impl AsRef<KeyboardButtonTypeRequestLocation> for KeyboardButtonTypeRequestLocation {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestLocation { self }
}

impl AsRef<KeyboardButtonTypeRequestLocation> for RTDKeyboardButtonTypeRequestLocationBuilder {
  fn as_ref(&self) -> &KeyboardButtonTypeRequestLocation { &self.inner }
}



