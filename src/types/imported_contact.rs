use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a contact to import
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportedContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Phone number of the user
  phone_number: String,
  /// First name of the user; 1-64 characters
  first_name: String,
  /// Last name of the user; 0-64 characters
  last_name: String,
  /// Note to add about the user; 0-getOption("user_note_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed; pass null to keep the current user's note
  note: FormattedText,
}

impl RObject for ImportedContact {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "importedContact"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ImportedContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDImportedContactBuilder {
    let inner = ImportedContact {
      td_name: "importedContact".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDImportedContactBuilder { inner }
  }

  pub fn phone_number(&self) -> &String {
    &self.phone_number
  }

  pub fn first_name(&self) -> &String {
    &self.first_name
  }

  pub fn last_name(&self) -> &String {
    &self.last_name
  }

  pub fn note(&self) -> &FormattedText {
    &self.note
  }
}

#[doc(hidden)]
pub struct RTDImportedContactBuilder {
  inner: ImportedContact,
}

impl RTDImportedContactBuilder {
  pub fn build(&self) -> ImportedContact {
    self.inner.clone()
  }

  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }

  pub fn first_name<T: AsRef<str>>(&mut self, first_name: T) -> &mut Self {
    self.inner.first_name = first_name.as_ref().to_string();
    self
  }

  pub fn last_name<T: AsRef<str>>(&mut self, last_name: T) -> &mut Self {
    self.inner.last_name = last_name.as_ref().to_string();
    self
  }

  pub fn note<T: AsRef<FormattedText>>(&mut self, note: T) -> &mut Self {
    self.inner.note = note.as_ref().clone();
    self
  }
}

impl AsRef<ImportedContact> for ImportedContact {
  fn as_ref(&self) -> &ImportedContact {
    self
  }
}

impl AsRef<ImportedContact> for RTDImportedContactBuilder {
  fn as_ref(&self) -> &ImportedContact {
    &self.inner
  }
}
