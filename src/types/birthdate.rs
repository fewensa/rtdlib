use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a birthdate of a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Birthdate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Day of the month; 1-31
  day: i64,
  /// Month of the year; 1-12
  month: i64,
  /// Birth year; 0 if unknown
  year: i64,
}

impl RObject for Birthdate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "birthdate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Birthdate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBirthdateBuilder {
    let inner = Birthdate {
      td_name: "birthdate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBirthdateBuilder { inner }
  }

  pub fn day(&self) -> i64 {
    self.day
  }

  pub fn month(&self) -> i64 {
    self.month
  }

  pub fn year(&self) -> i64 {
    self.year
  }
}

#[doc(hidden)]
pub struct RTDBirthdateBuilder {
  inner: Birthdate,
}

impl RTDBirthdateBuilder {
  pub fn build(&self) -> Birthdate {
    self.inner.clone()
  }

  pub fn day(&mut self, day: i64) -> &mut Self {
    self.inner.day = day;
    self
  }

  pub fn month(&mut self, month: i64) -> &mut Self {
    self.inner.month = month;
    self
  }

  pub fn year(&mut self, year: i64) -> &mut Self {
    self.inner.year = year;
    self
  }
}

impl AsRef<Birthdate> for Birthdate {
  fn as_ref(&self) -> &Birthdate {
    self
  }
}

impl AsRef<Birthdate> for RTDBirthdateBuilder {
  fn as_ref(&self) -> &Birthdate {
    &self.inner
  }
}
