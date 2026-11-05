use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of downloaded files, found by a search
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundFileDownloads {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total number of suitable files, ignoring offset
  total_counts: DownloadedFileCounts,
  /// The list of files
  files: Vec<FileDownload>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for FoundFileDownloads {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "foundFileDownloads"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FoundFileDownloads {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFoundFileDownloadsBuilder {
    let inner = FoundFileDownloads {
      td_name: "foundFileDownloads".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFoundFileDownloadsBuilder { inner }
  }

  pub fn total_counts(&self) -> &DownloadedFileCounts {
    &self.total_counts
  }

  pub fn files(&self) -> &Vec<FileDownload> {
    &self.files
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDFoundFileDownloadsBuilder {
  inner: FoundFileDownloads,
}

impl RTDFoundFileDownloadsBuilder {
  pub fn build(&self) -> FoundFileDownloads {
    self.inner.clone()
  }

  pub fn total_counts<T: AsRef<DownloadedFileCounts>>(&mut self, total_counts: T) -> &mut Self {
    self.inner.total_counts = total_counts.as_ref().clone();
    self
  }

  pub fn files(&mut self, files: Vec<FileDownload>) -> &mut Self {
    self.inner.files = files;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<FoundFileDownloads> for FoundFileDownloads {
  fn as_ref(&self) -> &FoundFileDownloads {
    self
  }
}

impl AsRef<FoundFileDownloads> for RTDFoundFileDownloadsBuilder {
  fn as_ref(&self) -> &FoundFileDownloads {
    &self.inner
  }
}
