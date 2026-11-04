use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of passkeys
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Passkeys {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of passkeys
  passkeys: Vec<Passkey>,
}

impl RObject for Passkeys {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "passkeys"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Passkeys {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPasskeysBuilder {
    let inner = Passkeys {
      td_name: "passkeys".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPasskeysBuilder { inner }
  }

  pub fn passkeys(&self) -> &Vec<Passkey> {
    &self.passkeys
  }
}

#[doc(hidden)]
pub struct RTDPasskeysBuilder {
  inner: Passkeys,
}

impl RTDPasskeysBuilder {
  pub fn build(&self) -> Passkeys {
    self.inner.clone()
  }

  pub fn passkeys(&mut self, passkeys: Vec<Passkey>) -> &mut Self {
    self.inner.passkeys = passkeys;
    self
  }
}

impl AsRef<Passkeys> for Passkeys {
  fn as_ref(&self) -> &Passkeys {
    self
  }
}

impl AsRef<Passkeys> for RTDPasskeysBuilder {
  fn as_ref(&self) -> &Passkeys {
    &self.inner
  }
}
