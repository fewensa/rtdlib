
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains the description of an error in a Telegram Passport element
pub trait TDPassportElementErrorSource: Debug + RObject {}

/// Contains the description of an error in a Telegram Passport element
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PassportElementErrorSource {
  #[doc(hidden)] _Default(()),
  /// One of the data fields contains an error. The error will be considered resolved when the value of the field changes
  DataField(PassportElementErrorSourceDataField),
  /// The file contains an error. The error will be considered resolved when the file changes
  File(PassportElementErrorSourceFile),
  /// The list of attached files contains an error. The error will be considered resolved when the list of files changes
  Files(PassportElementErrorSourceFiles),
  /// The front side of the document contains an error. The error will be considered resolved when the file with the front side changes
  FrontSide(PassportElementErrorSourceFrontSide),
  /// The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes
  ReverseSide(PassportElementErrorSourceReverseSide),
  /// The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes
  Selfie(PassportElementErrorSourceSelfie),
  /// One of files with the translation of the document contains an error. The error will be considered resolved when the file changes
  TranslationFile(PassportElementErrorSourceTranslationFile),
  /// The translation of the document contains an error. The error will be considered resolved when the list of translation files changes
  TranslationFiles(PassportElementErrorSourceTranslationFiles),
  /// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
  Unspecified(PassportElementErrorSourceUnspecified),

}

impl Default for PassportElementErrorSource {
  fn default() -> Self { PassportElementErrorSource::_Default(()) }
}

impl<'de> Deserialize<'de> for PassportElementErrorSource {
  fn deserialize<D>(deserializer: D) -> Result<PassportElementErrorSource, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PassportElementErrorSource,
      (passportElementErrorSourceDataField, DataField);
      (passportElementErrorSourceFile, File);
      (passportElementErrorSourceFiles, Files);
      (passportElementErrorSourceFrontSide, FrontSide);
      (passportElementErrorSourceReverseSide, ReverseSide);
      (passportElementErrorSourceSelfie, Selfie);
      (passportElementErrorSourceTranslationFile, TranslationFile);
      (passportElementErrorSourceTranslationFiles, TranslationFiles);
      (passportElementErrorSourceUnspecified, Unspecified);

    )(deserializer)
  }
}

impl RObject for PassportElementErrorSource {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      PassportElementErrorSource::DataField(t) => t.td_name(),
      PassportElementErrorSource::File(t) => t.td_name(),
      PassportElementErrorSource::Files(t) => t.td_name(),
      PassportElementErrorSource::FrontSide(t) => t.td_name(),
      PassportElementErrorSource::ReverseSide(t) => t.td_name(),
      PassportElementErrorSource::Selfie(t) => t.td_name(),
      PassportElementErrorSource::TranslationFile(t) => t.td_name(),
      PassportElementErrorSource::TranslationFiles(t) => t.td_name(),
      PassportElementErrorSource::Unspecified(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      PassportElementErrorSource::DataField(t) => t.extra(),
      PassportElementErrorSource::File(t) => t.extra(),
      PassportElementErrorSource::Files(t) => t.extra(),
      PassportElementErrorSource::FrontSide(t) => t.extra(),
      PassportElementErrorSource::ReverseSide(t) => t.extra(),
      PassportElementErrorSource::Selfie(t) => t.extra(),
      PassportElementErrorSource::TranslationFile(t) => t.extra(),
      PassportElementErrorSource::TranslationFiles(t) => t.extra(),
      PassportElementErrorSource::Unspecified(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl PassportElementErrorSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let PassportElementErrorSource::_Default(_) = self { true } else { false } }

  pub fn is_data_field(&self) -> bool { if let PassportElementErrorSource::DataField(_) = self { true } else { false } }
  pub fn is_file(&self) -> bool { if let PassportElementErrorSource::File(_) = self { true } else { false } }
  pub fn is_files(&self) -> bool { if let PassportElementErrorSource::Files(_) = self { true } else { false } }
  pub fn is_front_side(&self) -> bool { if let PassportElementErrorSource::FrontSide(_) = self { true } else { false } }
  pub fn is_reverse_side(&self) -> bool { if let PassportElementErrorSource::ReverseSide(_) = self { true } else { false } }
  pub fn is_selfie(&self) -> bool { if let PassportElementErrorSource::Selfie(_) = self { true } else { false } }
  pub fn is_translation_file(&self) -> bool { if let PassportElementErrorSource::TranslationFile(_) = self { true } else { false } }
  pub fn is_translation_files(&self) -> bool { if let PassportElementErrorSource::TranslationFiles(_) = self { true } else { false } }
  pub fn is_unspecified(&self) -> bool { if let PassportElementErrorSource::Unspecified(_) = self { true } else { false } }

  pub fn on_data_field<F: FnOnce(&PassportElementErrorSourceDataField)>(&self, fnc: F) -> &Self { if let PassportElementErrorSource::DataField(t) = self { fnc(t) }; self }
  pub fn on_file<F: FnOnce(&PassportElementErrorSourceFile)>(&self, fnc: F) -> &Self { if let PassportElementErrorSource::File(t) = self { fnc(t) }; self }
  pub fn on_files<F: FnOnce(&PassportElementErrorSourceFiles)>(&self, fnc: F) -> &Self { if let PassportElementErrorSource::Files(t) = self { fnc(t) }; self }
  pub fn on_front_side<F: FnOnce(&PassportElementErrorSourceFrontSide)>(&self, fnc: F) -> &Self { if let PassportElementErrorSource::FrontSide(t) = self { fnc(t) }; self }
  pub fn on_reverse_side<F: FnOnce(&PassportElementErrorSourceReverseSide)>(&self, fnc: F) -> &Self { if let PassportElementErrorSource::ReverseSide(t) = self { fnc(t) }; self }
  pub fn on_selfie<F: FnOnce(&PassportElementErrorSourceSelfie)>(&self, fnc: F) -> &Self { if let PassportElementErrorSource::Selfie(t) = self { fnc(t) }; self }
  pub fn on_translation_file<F: FnOnce(&PassportElementErrorSourceTranslationFile)>(&self, fnc: F) -> &Self { if let PassportElementErrorSource::TranslationFile(t) = self { fnc(t) }; self }
  pub fn on_translation_files<F: FnOnce(&PassportElementErrorSourceTranslationFiles)>(&self, fnc: F) -> &Self { if let PassportElementErrorSource::TranslationFiles(t) = self { fnc(t) }; self }
  pub fn on_unspecified<F: FnOnce(&PassportElementErrorSourceUnspecified)>(&self, fnc: F) -> &Self { if let PassportElementErrorSource::Unspecified(t) = self { fnc(t) }; self }

  pub fn as_data_field(&self) -> Option<&PassportElementErrorSourceDataField> { if let PassportElementErrorSource::DataField(t) = self { return Some(t) } None }
  pub fn as_file(&self) -> Option<&PassportElementErrorSourceFile> { if let PassportElementErrorSource::File(t) = self { return Some(t) } None }
  pub fn as_files(&self) -> Option<&PassportElementErrorSourceFiles> { if let PassportElementErrorSource::Files(t) = self { return Some(t) } None }
  pub fn as_front_side(&self) -> Option<&PassportElementErrorSourceFrontSide> { if let PassportElementErrorSource::FrontSide(t) = self { return Some(t) } None }
  pub fn as_reverse_side(&self) -> Option<&PassportElementErrorSourceReverseSide> { if let PassportElementErrorSource::ReverseSide(t) = self { return Some(t) } None }
  pub fn as_selfie(&self) -> Option<&PassportElementErrorSourceSelfie> { if let PassportElementErrorSource::Selfie(t) = self { return Some(t) } None }
  pub fn as_translation_file(&self) -> Option<&PassportElementErrorSourceTranslationFile> { if let PassportElementErrorSource::TranslationFile(t) = self { return Some(t) } None }
  pub fn as_translation_files(&self) -> Option<&PassportElementErrorSourceTranslationFiles> { if let PassportElementErrorSource::TranslationFiles(t) = self { return Some(t) } None }
  pub fn as_unspecified(&self) -> Option<&PassportElementErrorSourceUnspecified> { if let PassportElementErrorSource::Unspecified(t) = self { return Some(t) } None }



  pub fn data_field<T: AsRef<PassportElementErrorSourceDataField>>(t: T) -> Self { PassportElementErrorSource::DataField(t.as_ref().clone()) }

  pub fn file<T: AsRef<PassportElementErrorSourceFile>>(t: T) -> Self { PassportElementErrorSource::File(t.as_ref().clone()) }

  pub fn files<T: AsRef<PassportElementErrorSourceFiles>>(t: T) -> Self { PassportElementErrorSource::Files(t.as_ref().clone()) }

  pub fn front_side<T: AsRef<PassportElementErrorSourceFrontSide>>(t: T) -> Self { PassportElementErrorSource::FrontSide(t.as_ref().clone()) }

  pub fn reverse_side<T: AsRef<PassportElementErrorSourceReverseSide>>(t: T) -> Self { PassportElementErrorSource::ReverseSide(t.as_ref().clone()) }

  pub fn selfie<T: AsRef<PassportElementErrorSourceSelfie>>(t: T) -> Self { PassportElementErrorSource::Selfie(t.as_ref().clone()) }

  pub fn translation_file<T: AsRef<PassportElementErrorSourceTranslationFile>>(t: T) -> Self { PassportElementErrorSource::TranslationFile(t.as_ref().clone()) }

  pub fn translation_files<T: AsRef<PassportElementErrorSourceTranslationFiles>>(t: T) -> Self { PassportElementErrorSource::TranslationFiles(t.as_ref().clone()) }

  pub fn unspecified<T: AsRef<PassportElementErrorSourceUnspecified>>(t: T) -> Self { PassportElementErrorSource::Unspecified(t.as_ref().clone()) }

}

impl AsRef<PassportElementErrorSource> for PassportElementErrorSource {
  fn as_ref(&self) -> &PassportElementErrorSource { self }
}







/// One of the data fields contains an error. The error will be considered resolved when the value of the field changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceDataField {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Field name
  field_name: String,
  
}

impl RObject for PassportElementErrorSourceDataField {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceDataField" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementErrorSource for PassportElementErrorSourceDataField {}



impl PassportElementErrorSourceDataField {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorSourceDataFieldBuilder {
    let mut inner = PassportElementErrorSourceDataField::default();
    inner.td_name = "passportElementErrorSourceDataField".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorSourceDataFieldBuilder { inner }
  }

  pub fn field_name(&self) -> &String { &self.field_name }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorSourceDataFieldBuilder {
  inner: PassportElementErrorSourceDataField
}

impl RTDPassportElementErrorSourceDataFieldBuilder {
  pub fn build(&self) -> PassportElementErrorSourceDataField { self.inner.clone() }

   
  pub fn field_name<T: AsRef<str>>(&mut self, field_name: T) -> &mut Self {
    self.inner.field_name = field_name.as_ref().to_string();
    self
  }

}

impl AsRef<PassportElementErrorSourceDataField> for PassportElementErrorSourceDataField {
  fn as_ref(&self) -> &PassportElementErrorSourceDataField { self }
}

impl AsRef<PassportElementErrorSourceDataField> for RTDPassportElementErrorSourceDataFieldBuilder {
  fn as_ref(&self) -> &PassportElementErrorSourceDataField { &self.inner }
}







/// The file contains an error. The error will be considered resolved when the file changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Index of a file with the error
  file_index: i64,
  
}

impl RObject for PassportElementErrorSourceFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementErrorSource for PassportElementErrorSourceFile {}



impl PassportElementErrorSourceFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorSourceFileBuilder {
    let mut inner = PassportElementErrorSourceFile::default();
    inner.td_name = "passportElementErrorSourceFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorSourceFileBuilder { inner }
  }

  pub fn file_index(&self) -> i64 { self.file_index }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorSourceFileBuilder {
  inner: PassportElementErrorSourceFile
}

impl RTDPassportElementErrorSourceFileBuilder {
  pub fn build(&self) -> PassportElementErrorSourceFile { self.inner.clone() }

   
  pub fn file_index(&mut self, file_index: i64) -> &mut Self {
    self.inner.file_index = file_index;
    self
  }

}

impl AsRef<PassportElementErrorSourceFile> for PassportElementErrorSourceFile {
  fn as_ref(&self) -> &PassportElementErrorSourceFile { self }
}

impl AsRef<PassportElementErrorSourceFile> for RTDPassportElementErrorSourceFileBuilder {
  fn as_ref(&self) -> &PassportElementErrorSourceFile { &self.inner }
}







/// The list of attached files contains an error. The error will be considered resolved when the list of files changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceFiles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for PassportElementErrorSourceFiles {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceFiles" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementErrorSource for PassportElementErrorSourceFiles {}



impl PassportElementErrorSourceFiles {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorSourceFilesBuilder {
    let mut inner = PassportElementErrorSourceFiles::default();
    inner.td_name = "passportElementErrorSourceFiles".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorSourceFilesBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorSourceFilesBuilder {
  inner: PassportElementErrorSourceFiles
}

impl RTDPassportElementErrorSourceFilesBuilder {
  pub fn build(&self) -> PassportElementErrorSourceFiles { self.inner.clone() }

}

impl AsRef<PassportElementErrorSourceFiles> for PassportElementErrorSourceFiles {
  fn as_ref(&self) -> &PassportElementErrorSourceFiles { self }
}

impl AsRef<PassportElementErrorSourceFiles> for RTDPassportElementErrorSourceFilesBuilder {
  fn as_ref(&self) -> &PassportElementErrorSourceFiles { &self.inner }
}







/// The front side of the document contains an error. The error will be considered resolved when the file with the front side changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceFrontSide {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for PassportElementErrorSourceFrontSide {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceFrontSide" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementErrorSource for PassportElementErrorSourceFrontSide {}



impl PassportElementErrorSourceFrontSide {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorSourceFrontSideBuilder {
    let mut inner = PassportElementErrorSourceFrontSide::default();
    inner.td_name = "passportElementErrorSourceFrontSide".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorSourceFrontSideBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorSourceFrontSideBuilder {
  inner: PassportElementErrorSourceFrontSide
}

impl RTDPassportElementErrorSourceFrontSideBuilder {
  pub fn build(&self) -> PassportElementErrorSourceFrontSide { self.inner.clone() }

}

impl AsRef<PassportElementErrorSourceFrontSide> for PassportElementErrorSourceFrontSide {
  fn as_ref(&self) -> &PassportElementErrorSourceFrontSide { self }
}

impl AsRef<PassportElementErrorSourceFrontSide> for RTDPassportElementErrorSourceFrontSideBuilder {
  fn as_ref(&self) -> &PassportElementErrorSourceFrontSide { &self.inner }
}







/// The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceReverseSide {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for PassportElementErrorSourceReverseSide {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceReverseSide" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementErrorSource for PassportElementErrorSourceReverseSide {}



impl PassportElementErrorSourceReverseSide {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorSourceReverseSideBuilder {
    let mut inner = PassportElementErrorSourceReverseSide::default();
    inner.td_name = "passportElementErrorSourceReverseSide".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorSourceReverseSideBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorSourceReverseSideBuilder {
  inner: PassportElementErrorSourceReverseSide
}

impl RTDPassportElementErrorSourceReverseSideBuilder {
  pub fn build(&self) -> PassportElementErrorSourceReverseSide { self.inner.clone() }

}

impl AsRef<PassportElementErrorSourceReverseSide> for PassportElementErrorSourceReverseSide {
  fn as_ref(&self) -> &PassportElementErrorSourceReverseSide { self }
}

impl AsRef<PassportElementErrorSourceReverseSide> for RTDPassportElementErrorSourceReverseSideBuilder {
  fn as_ref(&self) -> &PassportElementErrorSourceReverseSide { &self.inner }
}







/// The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceSelfie {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for PassportElementErrorSourceSelfie {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceSelfie" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementErrorSource for PassportElementErrorSourceSelfie {}



impl PassportElementErrorSourceSelfie {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorSourceSelfieBuilder {
    let mut inner = PassportElementErrorSourceSelfie::default();
    inner.td_name = "passportElementErrorSourceSelfie".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorSourceSelfieBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorSourceSelfieBuilder {
  inner: PassportElementErrorSourceSelfie
}

impl RTDPassportElementErrorSourceSelfieBuilder {
  pub fn build(&self) -> PassportElementErrorSourceSelfie { self.inner.clone() }

}

impl AsRef<PassportElementErrorSourceSelfie> for PassportElementErrorSourceSelfie {
  fn as_ref(&self) -> &PassportElementErrorSourceSelfie { self }
}

impl AsRef<PassportElementErrorSourceSelfie> for RTDPassportElementErrorSourceSelfieBuilder {
  fn as_ref(&self) -> &PassportElementErrorSourceSelfie { &self.inner }
}







/// One of files with the translation of the document contains an error. The error will be considered resolved when the file changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceTranslationFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Index of a file with the error
  file_index: i64,
  
}

impl RObject for PassportElementErrorSourceTranslationFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceTranslationFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementErrorSource for PassportElementErrorSourceTranslationFile {}



impl PassportElementErrorSourceTranslationFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorSourceTranslationFileBuilder {
    let mut inner = PassportElementErrorSourceTranslationFile::default();
    inner.td_name = "passportElementErrorSourceTranslationFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorSourceTranslationFileBuilder { inner }
  }

  pub fn file_index(&self) -> i64 { self.file_index }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorSourceTranslationFileBuilder {
  inner: PassportElementErrorSourceTranslationFile
}

impl RTDPassportElementErrorSourceTranslationFileBuilder {
  pub fn build(&self) -> PassportElementErrorSourceTranslationFile { self.inner.clone() }

   
  pub fn file_index(&mut self, file_index: i64) -> &mut Self {
    self.inner.file_index = file_index;
    self
  }

}

impl AsRef<PassportElementErrorSourceTranslationFile> for PassportElementErrorSourceTranslationFile {
  fn as_ref(&self) -> &PassportElementErrorSourceTranslationFile { self }
}

impl AsRef<PassportElementErrorSourceTranslationFile> for RTDPassportElementErrorSourceTranslationFileBuilder {
  fn as_ref(&self) -> &PassportElementErrorSourceTranslationFile { &self.inner }
}







/// The translation of the document contains an error. The error will be considered resolved when the list of translation files changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceTranslationFiles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for PassportElementErrorSourceTranslationFiles {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceTranslationFiles" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementErrorSource for PassportElementErrorSourceTranslationFiles {}



impl PassportElementErrorSourceTranslationFiles {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorSourceTranslationFilesBuilder {
    let mut inner = PassportElementErrorSourceTranslationFiles::default();
    inner.td_name = "passportElementErrorSourceTranslationFiles".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorSourceTranslationFilesBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorSourceTranslationFilesBuilder {
  inner: PassportElementErrorSourceTranslationFiles
}

impl RTDPassportElementErrorSourceTranslationFilesBuilder {
  pub fn build(&self) -> PassportElementErrorSourceTranslationFiles { self.inner.clone() }

}

impl AsRef<PassportElementErrorSourceTranslationFiles> for PassportElementErrorSourceTranslationFiles {
  fn as_ref(&self) -> &PassportElementErrorSourceTranslationFiles { self }
}

impl AsRef<PassportElementErrorSourceTranslationFiles> for RTDPassportElementErrorSourceTranslationFilesBuilder {
  fn as_ref(&self) -> &PassportElementErrorSourceTranslationFiles { &self.inner }
}







/// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementErrorSourceUnspecified {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for PassportElementErrorSourceUnspecified {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementErrorSourceUnspecified" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPassportElementErrorSource for PassportElementErrorSourceUnspecified {}



impl PassportElementErrorSourceUnspecified {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorSourceUnspecifiedBuilder {
    let mut inner = PassportElementErrorSourceUnspecified::default();
    inner.td_name = "passportElementErrorSourceUnspecified".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorSourceUnspecifiedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorSourceUnspecifiedBuilder {
  inner: PassportElementErrorSourceUnspecified
}

impl RTDPassportElementErrorSourceUnspecifiedBuilder {
  pub fn build(&self) -> PassportElementErrorSourceUnspecified { self.inner.clone() }

}

impl AsRef<PassportElementErrorSourceUnspecified> for PassportElementErrorSourceUnspecified {
  fn as_ref(&self) -> &PassportElementErrorSourceUnspecified { self }
}

impl AsRef<PassportElementErrorSourceUnspecified> for RTDPassportElementErrorSourceUnspecifiedBuilder {
  fn as_ref(&self) -> &PassportElementErrorSourceUnspecified { &self.inner }
}



