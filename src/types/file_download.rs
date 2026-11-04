use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a file added to file download list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileDownload {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File identifier
  file_id: i64,
  /// The message with the file
  message: Message,
  /// Point in time (Unix timestamp) when the file was added to the download list
  add_date: i64,
  /// Point in time (Unix timestamp) when the file downloading was completed; 0 if the file downloading isn't completed
  complete_date: i64,
  /// True, if downloading of the file is paused
  is_paused: bool,
}

impl RObject for FileDownload {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "fileDownload"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FileDownload {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFileDownloadBuilder {
    let inner = FileDownload {
      td_name: "fileDownload".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFileDownloadBuilder { inner }
  }

  pub fn file_id(&self) -> i64 {
    self.file_id
  }

  pub fn message(&self) -> &Message {
    &self.message
  }

  pub fn add_date(&self) -> i64 {
    self.add_date
  }

  pub fn complete_date(&self) -> i64 {
    self.complete_date
  }

  pub fn is_paused(&self) -> bool {
    self.is_paused
  }
}

#[doc(hidden)]
pub struct RTDFileDownloadBuilder {
  inner: FileDownload,
}

impl RTDFileDownloadBuilder {
  pub fn build(&self) -> FileDownload {
    self.inner.clone()
  }

  pub fn file_id(&mut self, file_id: i64) -> &mut Self {
    self.inner.file_id = file_id;
    self
  }

  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

  pub fn add_date(&mut self, add_date: i64) -> &mut Self {
    self.inner.add_date = add_date;
    self
  }

  pub fn complete_date(&mut self, complete_date: i64) -> &mut Self {
    self.inner.complete_date = complete_date;
    self
  }

  pub fn is_paused(&mut self, is_paused: bool) -> &mut Self {
    self.inner.is_paused = is_paused;
    self
  }
}

impl AsRef<FileDownload> for FileDownload {
  fn as_ref(&self) -> &FileDownload {
    self
  }
}

impl AsRef<FileDownload> for RTDFileDownloadBuilder {
  fn as_ref(&self) -> &FileDownload {
    &self.inner
  }
}
