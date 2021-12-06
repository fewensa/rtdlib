
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a color replacement for animated emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColorReplacement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Original animated emoji color in the RGB24 format
  old_color: i64,
  /// Replacement animated emoji color in the RGB24 format
  new_color: i64,
  
}

impl RObject for ColorReplacement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "colorReplacement" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ColorReplacement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDColorReplacementBuilder {
    let mut inner = ColorReplacement::default();
    inner.td_name = "colorReplacement".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDColorReplacementBuilder { inner }
  }

  pub fn old_color(&self) -> i64 { self.old_color }

  pub fn new_color(&self) -> i64 { self.new_color }

}

#[doc(hidden)]
pub struct RTDColorReplacementBuilder {
  inner: ColorReplacement
}

impl RTDColorReplacementBuilder {
  pub fn build(&self) -> ColorReplacement { self.inner.clone() }

   
  pub fn old_color(&mut self, old_color: i64) -> &mut Self {
    self.inner.old_color = old_color;
    self
  }

   
  pub fn new_color(&mut self, new_color: i64) -> &mut Self {
    self.inner.new_color = new_color;
    self
  }

}

impl AsRef<ColorReplacement> for ColorReplacement {
  fn as_ref(&self) -> &ColorReplacement { self }
}

impl AsRef<ColorReplacement> for RTDColorReplacementBuilder {
  fn as_ref(&self) -> &ColorReplacement { &self.inner }
}



