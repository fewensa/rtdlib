
use crate::types::*;
use crate::errors::*;




/// An object of this type is returned on a successful function call for certain functions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ok {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for Ok {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "ok" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Ok {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDOkBuilder {
    let mut inner = Ok::default();
    inner.td_name = "ok".to_string();
    RTDOkBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDOkBuilder {
  inner: Ok
}

impl RTDOkBuilder {
  pub fn build(&self) -> Ok { self.inner.clone() }

}

impl AsRef<Ok> for Ok {
  fn as_ref(&self) -> &Ok { self }
}

impl AsRef<Ok> for RTDOkBuilder {
  fn as_ref(&self) -> &Ok { &self.inner }
}



