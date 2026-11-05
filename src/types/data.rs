use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains some binary data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Data
  data: String,
}

impl RObject for Data {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "data"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Data {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDataBuilder {
    let inner = Data {
      td_name: "data".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDataBuilder { inner }
  }

  pub fn data(&self) -> &String {
    &self.data
  }
}

#[doc(hidden)]
pub struct RTDDataBuilder {
  inner: Data,
}

impl RTDDataBuilder {
  pub fn build(&self) -> Data {
    self.inner.clone()
  }

  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }
}

impl AsRef<Data> for Data {
  fn as_ref(&self) -> &Data {
    self
  }
}

impl AsRef<Data> for RTDDataBuilder {
  fn as_ref(&self) -> &Data {
    &self.inner
  }
}
