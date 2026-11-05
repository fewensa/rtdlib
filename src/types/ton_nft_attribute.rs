use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents an attribute of an NFT in the TON blockchain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonNftAttribute {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Trait type
  trait_type: String,
  /// Value of the attribute
  value: String,
}

impl RObject for TonNftAttribute {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonNftAttribute"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonNftAttribute {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonNftAttributeBuilder {
    let inner = TonNftAttribute {
      td_name: "tonNftAttribute".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonNftAttributeBuilder { inner }
  }

  pub fn trait_type(&self) -> &String {
    &self.trait_type
  }

  pub fn value(&self) -> &String {
    &self.value
  }
}

#[doc(hidden)]
pub struct RTDTonNftAttributeBuilder {
  inner: TonNftAttribute,
}

impl RTDTonNftAttributeBuilder {
  pub fn build(&self) -> TonNftAttribute {
    self.inner.clone()
  }

  pub fn trait_type<T: AsRef<str>>(&mut self, trait_type: T) -> &mut Self {
    self.inner.trait_type = trait_type.as_ref().to_string();
    self
  }

  pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().to_string();
    self
  }
}

impl AsRef<TonNftAttribute> for TonNftAttribute {
  fn as_ref(&self) -> &TonNftAttribute {
    self
  }
}

impl AsRef<TonNftAttribute> for RTDTonNftAttributeBuilder {
  fn as_ref(&self) -> &TonNftAttribute {
    &self.inner
  }
}
