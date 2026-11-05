use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an address of a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A two-letter ISO 3166-1 alpha-2 country code
  country_code: String,
  /// State, if applicable; empty if unknown
  state: String,
  /// City; empty if unknown
  city: String,
  /// The address; empty if unknown
  street: String,
}

impl RObject for LocationAddress {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "locationAddress"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl LocationAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLocationAddressBuilder {
    let inner = LocationAddress {
      td_name: "locationAddress".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLocationAddressBuilder { inner }
  }

  pub fn country_code(&self) -> &String {
    &self.country_code
  }

  pub fn state(&self) -> &String {
    &self.state
  }

  pub fn city(&self) -> &String {
    &self.city
  }

  pub fn street(&self) -> &String {
    &self.street
  }
}

#[doc(hidden)]
pub struct RTDLocationAddressBuilder {
  inner: LocationAddress,
}

impl RTDLocationAddressBuilder {
  pub fn build(&self) -> LocationAddress {
    self.inner.clone()
  }

  pub fn country_code<T: AsRef<str>>(&mut self, country_code: T) -> &mut Self {
    self.inner.country_code = country_code.as_ref().to_string();
    self
  }

  pub fn state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().to_string();
    self
  }

  pub fn city<T: AsRef<str>>(&mut self, city: T) -> &mut Self {
    self.inner.city = city.as_ref().to_string();
    self
  }

  pub fn street<T: AsRef<str>>(&mut self, street: T) -> &mut Self {
    self.inner.street = street.as_ref().to_string();
    self
  }
}

impl AsRef<LocationAddress> for LocationAddress {
  fn as_ref(&self) -> &LocationAddress {
    self
  }
}

impl AsRef<LocationAddress> for RTDLocationAddressBuilder {
  fn as_ref(&self) -> &LocationAddress {
    &self.inner
  }
}
