use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a passkey
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Passkey {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the passkey
  id: String,
  /// Name of the passkey
  name: String,
  /// Point in time (Unix timestamp) when the passkey was added
  addition_date: i64,
  /// Point in time (Unix timestamp) when the passkey was used last time; 0 if never
  last_usage_date: i64,
  /// Identifier of the custom emoji that is used as the icon of the software that created the passkey; 0 if unknown
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  software_icon_custom_emoji_id: isize,
}

impl RObject for Passkey {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "passkey"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Passkey {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPasskeyBuilder {
    let inner = Passkey {
      td_name: "passkey".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPasskeyBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn addition_date(&self) -> i64 {
    self.addition_date
  }

  pub fn last_usage_date(&self) -> i64 {
    self.last_usage_date
  }

  pub fn software_icon_custom_emoji_id(&self) -> isize {
    self.software_icon_custom_emoji_id
  }
}

#[doc(hidden)]
pub struct RTDPasskeyBuilder {
  inner: Passkey,
}

impl RTDPasskeyBuilder {
  pub fn build(&self) -> Passkey {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn addition_date(&mut self, addition_date: i64) -> &mut Self {
    self.inner.addition_date = addition_date;
    self
  }

  pub fn last_usage_date(&mut self, last_usage_date: i64) -> &mut Self {
    self.inner.last_usage_date = last_usage_date;
    self
  }

  pub fn software_icon_custom_emoji_id(
    &mut self,
    software_icon_custom_emoji_id: isize,
  ) -> &mut Self {
    self.inner.software_icon_custom_emoji_id = software_icon_custom_emoji_id;
    self
  }
}

impl AsRef<Passkey> for Passkey {
  fn as_ref(&self) -> &Passkey {
    self
  }
}

impl AsRef<Passkey> for RTDPasskeyBuilder {
  fn as_ref(&self) -> &Passkey {
    &self.inner
  }
}
