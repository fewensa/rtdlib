use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a location of a business
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The location; may be null if not specified
  location: Option<Location>,
  /// Location address; 1-96 characters
  address: String,
}

impl RObject for BusinessLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessLocationBuilder {
    let inner = BusinessLocation {
      td_name: "businessLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessLocationBuilder { inner }
  }

  pub fn location(&self) -> &Option<Location> {
    &self.location
  }

  pub fn address(&self) -> &String {
    &self.address
  }
}

#[doc(hidden)]
pub struct RTDBusinessLocationBuilder {
  inner: BusinessLocation,
}

impl RTDBusinessLocationBuilder {
  pub fn build(&self) -> BusinessLocation {
    self.inner.clone()
  }

  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = Some(location.as_ref().clone());
    self
  }

  pub fn address<T: AsRef<str>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().to_string();
    self
  }
}

impl AsRef<BusinessLocation> for BusinessLocation {
  fn as_ref(&self) -> &BusinessLocation {
    self
  }
}

impl AsRef<BusinessLocation> for RTDBusinessLocationBuilder {
  fn as_ref(&self) -> &BusinessLocation {
    &self.inner
  }
}
