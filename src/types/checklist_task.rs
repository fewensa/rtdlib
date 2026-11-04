use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a task in a checklist
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChecklistTask {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the task
  id: i64,
  /// Text of the task; may contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, DateTime and automatically found entities
  text: FormattedText,
  /// Identifier of the user or chat that completed the task; may be null if the task isn't completed yet
  completed_by: Option<MessageSender>,
  /// Point in time (Unix timestamp) when the task was completed; 0 if the task isn't completed
  completion_date: i64,
}

impl RObject for ChecklistTask {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "checklistTask"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChecklistTask {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChecklistTaskBuilder {
    let inner = ChecklistTask {
      td_name: "checklistTask".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChecklistTaskBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn completed_by(&self) -> &Option<MessageSender> {
    &self.completed_by
  }

  pub fn completion_date(&self) -> i64 {
    self.completion_date
  }
}

#[doc(hidden)]
pub struct RTDChecklistTaskBuilder {
  inner: ChecklistTask,
}

impl RTDChecklistTaskBuilder {
  pub fn build(&self) -> ChecklistTask {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn completed_by<T: AsRef<MessageSender>>(&mut self, completed_by: T) -> &mut Self {
    self.inner.completed_by = Some(completed_by.as_ref().clone());
    self
  }

  pub fn completion_date(&mut self, completion_date: i64) -> &mut Self {
    self.inner.completion_date = completion_date;
    self
  }
}

impl AsRef<ChecklistTask> for ChecklistTask {
  fn as_ref(&self) -> &ChecklistTask {
    self
  }
}

impl AsRef<ChecklistTask> for RTDChecklistTaskBuilder {
  fn as_ref(&self) -> &ChecklistTask {
    &self.inner
  }
}
