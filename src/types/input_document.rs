use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A document (general file) to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File to be sent
  document: InputFile,
  /// Document thumbnail; pass null to skip thumbnail uploading
  thumbnail: InputThumbnail,
  /// Pass true to disable automatic file type detection and send the document as a file. Always true for files sent to secret chats
  disable_content_type_detection: bool,
}

impl RObject for InputDocument {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputDocument"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputDocumentBuilder {
    let inner = InputDocument {
      td_name: "inputDocument".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputDocumentBuilder { inner }
  }

  pub fn document(&self) -> &InputFile {
    &self.document
  }

  pub fn thumbnail(&self) -> &InputThumbnail {
    &self.thumbnail
  }

  pub fn disable_content_type_detection(&self) -> bool {
    self.disable_content_type_detection
  }
}

#[doc(hidden)]
pub struct RTDInputDocumentBuilder {
  inner: InputDocument,
}

impl RTDInputDocumentBuilder {
  pub fn build(&self) -> InputDocument {
    self.inner.clone()
  }

  pub fn document<T: AsRef<InputFile>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
    self
  }

  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

  pub fn disable_content_type_detection(
    &mut self,
    disable_content_type_detection: bool,
  ) -> &mut Self {
    self.inner.disable_content_type_detection = disable_content_type_detection;
    self
  }
}

impl AsRef<InputDocument> for InputDocument {
  fn as_ref(&self) -> &InputDocument {
    self
  }
}

impl AsRef<InputDocument> for RTDInputDocumentBuilder {
  fn as_ref(&self) -> &InputDocument {
    &self.inner
  }
}
