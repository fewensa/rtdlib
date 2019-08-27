
use crate::types::*;
use crate::errors::*;




/// Contains information about a language pack
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Unique language pack identifier
  id: String,
  /// Language name
  name: String,
  /// Name of the language in that language
  native_name: String,
  /// Total number of non-deleted strings from the language pack available locally
  local_string_count: i64,
  
}

impl RObject for LanguagePackInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackInfo" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LanguagePackInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLanguagePackInfoBuilder {
    let mut inner = LanguagePackInfo::default();
    inner.td_name = "languagePackInfo".to_string();
    RTDLanguagePackInfoBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn name(&self) -> &String { &self.name }

  pub fn native_name(&self) -> &String { &self.native_name }

  pub fn local_string_count(&self) -> i64 { self.local_string_count }

}

#[doc(hidden)]
pub struct RTDLanguagePackInfoBuilder {
  inner: LanguagePackInfo
}

impl RTDLanguagePackInfoBuilder {
  pub fn build(&self) -> LanguagePackInfo { self.inner.clone() }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn native_name<T: AsRef<str>>(&mut self, native_name: T) -> &mut Self {
    self.inner.native_name = native_name.as_ref().to_string();
    self
  }

   
  pub fn local_string_count(&mut self, local_string_count: i64) -> &mut Self {
    self.inner.local_string_count = local_string_count;
    self
  }

}

impl AsRef<LanguagePackInfo> for LanguagePackInfo {
  fn as_ref(&self) -> &LanguagePackInfo { self }
}

impl AsRef<LanguagePackInfo> for RTDLanguagePackInfoBuilder {
  fn as_ref(&self) -> &LanguagePackInfo { &self.inner }
}



