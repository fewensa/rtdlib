use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an option to report an entity to Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the option
  id: String,
  /// Text of the option
  text: String,
}

impl RObject for ReportOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReportOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportOptionBuilder {
    let inner = ReportOption {
      td_name: "reportOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReportOptionBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn text(&self) -> &String {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDReportOptionBuilder {
  inner: ReportOption,
}

impl RTDReportOptionBuilder {
  pub fn build(&self) -> ReportOption {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }
}

impl AsRef<ReportOption> for ReportOption {
  fn as_ref(&self) -> &ReportOption {
    self
  }
}

impl AsRef<ReportOption> for RTDReportOptionBuilder {
  fn as_ref(&self) -> &ReportOption {
    &self.inner
  }
}
