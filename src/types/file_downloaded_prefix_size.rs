use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains size of downloaded prefix of a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileDownloadedPrefixSize {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The prefix size, in bytes
  size: i64,
}

impl RObject for FileDownloadedPrefixSize {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileDownloadedPrefixSize"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FileDownloadedPrefixSize {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileDownloadedPrefixSizeBuilder {
    let inner = FileDownloadedPrefixSize {
      td_name: "fileDownloadedPrefixSize".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFileDownloadedPrefixSizeBuilder { inner }
  }

  pub fn size(&self) -> i64 {
    self.size
  }
}

#[doc(hidden)]
pub struct RTDFileDownloadedPrefixSizeBuilder {
  inner: FileDownloadedPrefixSize,
}

impl RTDFileDownloadedPrefixSizeBuilder {
  pub fn build(&self) -> FileDownloadedPrefixSize {
    self.inner.clone()
  }

  pub fn size(&mut self, size: i64) -> &mut Self {
    self.inner.size = size;
    self
  }
}

impl AsRef<FileDownloadedPrefixSize> for FileDownloadedPrefixSize {
  fn as_ref(&self) -> &FileDownloadedPrefixSize {
    self
  }
}

impl AsRef<FileDownloadedPrefixSize> for RTDFileDownloadedPrefixSizeBuilder {
  fn as_ref(&self) -> &FileDownloadedPrefixSize {
    &self.inner
  }
}
