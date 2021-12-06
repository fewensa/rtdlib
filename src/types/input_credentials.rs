
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains information about the payment method chosen by the user
pub trait TDInputCredentials: Debug + RObject {}

/// Contains information about the payment method chosen by the user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputCredentials {
  #[doc(hidden)] _Default(()),
  /// Applies if a user enters new credentials using Apple Pay
  ApplePay(InputCredentialsApplePay),
  /// Applies if a user enters new credentials using Google Pay
  GooglePay(InputCredentialsGooglePay),
  /// Applies if a user enters new credentials on a payment provider website
  New(InputCredentialsNew),
  /// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
  Saved(InputCredentialsSaved),

}

impl Default for InputCredentials {
  fn default() -> Self { InputCredentials::_Default(()) }
}

impl<'de> Deserialize<'de> for InputCredentials {
  fn deserialize<D>(deserializer: D) -> Result<InputCredentials, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputCredentials,
      (inputCredentialsApplePay, ApplePay);
      (inputCredentialsGooglePay, GooglePay);
      (inputCredentialsNew, New);
      (inputCredentialsSaved, Saved);

    )(deserializer)
  }
}

impl RObject for InputCredentials {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      InputCredentials::ApplePay(t) => t.td_name(),
      InputCredentials::GooglePay(t) => t.td_name(),
      InputCredentials::New(t) => t.td_name(),
      InputCredentials::Saved(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      InputCredentials::ApplePay(t) => t.extra(),
      InputCredentials::GooglePay(t) => t.extra(),
      InputCredentials::New(t) => t.extra(),
      InputCredentials::Saved(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl InputCredentials {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let InputCredentials::_Default(_) = self { true } else { false } }

  pub fn is_apple_pay(&self) -> bool { if let InputCredentials::ApplePay(_) = self { true } else { false } }
  pub fn is_google_pay(&self) -> bool { if let InputCredentials::GooglePay(_) = self { true } else { false } }
  pub fn is_new(&self) -> bool { if let InputCredentials::New(_) = self { true } else { false } }
  pub fn is_saved(&self) -> bool { if let InputCredentials::Saved(_) = self { true } else { false } }

  pub fn on_apple_pay<F: FnOnce(&InputCredentialsApplePay)>(&self, fnc: F) -> &Self { if let InputCredentials::ApplePay(t) = self { fnc(t) }; self }
  pub fn on_google_pay<F: FnOnce(&InputCredentialsGooglePay)>(&self, fnc: F) -> &Self { if let InputCredentials::GooglePay(t) = self { fnc(t) }; self }
  pub fn on_new<F: FnOnce(&InputCredentialsNew)>(&self, fnc: F) -> &Self { if let InputCredentials::New(t) = self { fnc(t) }; self }
  pub fn on_saved<F: FnOnce(&InputCredentialsSaved)>(&self, fnc: F) -> &Self { if let InputCredentials::Saved(t) = self { fnc(t) }; self }

  pub fn as_apple_pay(&self) -> Option<&InputCredentialsApplePay> { if let InputCredentials::ApplePay(t) = self { return Some(t) } None }
  pub fn as_google_pay(&self) -> Option<&InputCredentialsGooglePay> { if let InputCredentials::GooglePay(t) = self { return Some(t) } None }
  pub fn as_new(&self) -> Option<&InputCredentialsNew> { if let InputCredentials::New(t) = self { return Some(t) } None }
  pub fn as_saved(&self) -> Option<&InputCredentialsSaved> { if let InputCredentials::Saved(t) = self { return Some(t) } None }



  pub fn apple_pay<T: AsRef<InputCredentialsApplePay>>(t: T) -> Self { InputCredentials::ApplePay(t.as_ref().clone()) }

  pub fn google_pay<T: AsRef<InputCredentialsGooglePay>>(t: T) -> Self { InputCredentials::GooglePay(t.as_ref().clone()) }

  pub fn new<T: AsRef<InputCredentialsNew>>(t: T) -> Self { InputCredentials::New(t.as_ref().clone()) }

  pub fn saved<T: AsRef<InputCredentialsSaved>>(t: T) -> Self { InputCredentials::Saved(t.as_ref().clone()) }

}

impl AsRef<InputCredentials> for InputCredentials {
  fn as_ref(&self) -> &InputCredentials { self }
}







/// Applies if a user enters new credentials using Apple Pay
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsApplePay {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// JSON-encoded data with the credential identifier
  data: String,
  
}

impl RObject for InputCredentialsApplePay {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputCredentialsApplePay" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputCredentials for InputCredentialsApplePay {}



impl InputCredentialsApplePay {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputCredentialsApplePayBuilder {
    let mut inner = InputCredentialsApplePay::default();
    inner.td_name = "inputCredentialsApplePay".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInputCredentialsApplePayBuilder { inner }
  }

  pub fn data(&self) -> &String { &self.data }

}

#[doc(hidden)]
pub struct RTDInputCredentialsApplePayBuilder {
  inner: InputCredentialsApplePay
}

impl RTDInputCredentialsApplePayBuilder {
  pub fn build(&self) -> InputCredentialsApplePay { self.inner.clone() }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

}

impl AsRef<InputCredentialsApplePay> for InputCredentialsApplePay {
  fn as_ref(&self) -> &InputCredentialsApplePay { self }
}

impl AsRef<InputCredentialsApplePay> for RTDInputCredentialsApplePayBuilder {
  fn as_ref(&self) -> &InputCredentialsApplePay { &self.inner }
}







/// Applies if a user enters new credentials using Google Pay
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsGooglePay {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// JSON-encoded data with the credential identifier
  data: String,
  
}

impl RObject for InputCredentialsGooglePay {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputCredentialsGooglePay" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputCredentials for InputCredentialsGooglePay {}



impl InputCredentialsGooglePay {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputCredentialsGooglePayBuilder {
    let mut inner = InputCredentialsGooglePay::default();
    inner.td_name = "inputCredentialsGooglePay".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInputCredentialsGooglePayBuilder { inner }
  }

  pub fn data(&self) -> &String { &self.data }

}

#[doc(hidden)]
pub struct RTDInputCredentialsGooglePayBuilder {
  inner: InputCredentialsGooglePay
}

impl RTDInputCredentialsGooglePayBuilder {
  pub fn build(&self) -> InputCredentialsGooglePay { self.inner.clone() }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

}

impl AsRef<InputCredentialsGooglePay> for InputCredentialsGooglePay {
  fn as_ref(&self) -> &InputCredentialsGooglePay { self }
}

impl AsRef<InputCredentialsGooglePay> for RTDInputCredentialsGooglePayBuilder {
  fn as_ref(&self) -> &InputCredentialsGooglePay { &self.inner }
}







/// Applies if a user enters new credentials on a payment provider website
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsNew {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// JSON-encoded data with the credential identifier from the payment provider
  data: String,
  /// True, if the credential identifier can be saved on the server side
  allow_save: bool,
  
}

impl RObject for InputCredentialsNew {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputCredentialsNew" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputCredentials for InputCredentialsNew {}



impl InputCredentialsNew {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputCredentialsNewBuilder {
    let mut inner = InputCredentialsNew::default();
    inner.td_name = "inputCredentialsNew".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInputCredentialsNewBuilder { inner }
  }

  pub fn data(&self) -> &String { &self.data }

  pub fn allow_save(&self) -> bool { self.allow_save }

}

#[doc(hidden)]
pub struct RTDInputCredentialsNewBuilder {
  inner: InputCredentialsNew
}

impl RTDInputCredentialsNewBuilder {
  pub fn build(&self) -> InputCredentialsNew { self.inner.clone() }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

   
  pub fn allow_save(&mut self, allow_save: bool) -> &mut Self {
    self.inner.allow_save = allow_save;
    self
  }

}

impl AsRef<InputCredentialsNew> for InputCredentialsNew {
  fn as_ref(&self) -> &InputCredentialsNew { self }
}

impl AsRef<InputCredentialsNew> for RTDInputCredentialsNewBuilder {
  fn as_ref(&self) -> &InputCredentialsNew { &self.inner }
}







/// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsSaved {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the saved credentials
  saved_credentials_id: String,
  
}

impl RObject for InputCredentialsSaved {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputCredentialsSaved" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputCredentials for InputCredentialsSaved {}



impl InputCredentialsSaved {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputCredentialsSavedBuilder {
    let mut inner = InputCredentialsSaved::default();
    inner.td_name = "inputCredentialsSaved".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInputCredentialsSavedBuilder { inner }
  }

  pub fn saved_credentials_id(&self) -> &String { &self.saved_credentials_id }

}

#[doc(hidden)]
pub struct RTDInputCredentialsSavedBuilder {
  inner: InputCredentialsSaved
}

impl RTDInputCredentialsSavedBuilder {
  pub fn build(&self) -> InputCredentialsSaved { self.inner.clone() }

   
  pub fn saved_credentials_id<T: AsRef<str>>(&mut self, saved_credentials_id: T) -> &mut Self {
    self.inner.saved_credentials_id = saved_credentials_id.as_ref().to_string();
    self
  }

}

impl AsRef<InputCredentialsSaved> for InputCredentialsSaved {
  fn as_ref(&self) -> &InputCredentialsSaved { self }
}

impl AsRef<InputCredentialsSaved> for RTDInputCredentialsSavedBuilder {
  fn as_ref(&self) -> &InputCredentialsSaved { &self.inner }
}



