use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a task in a checklist to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChecklistTask {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the task; must be positive
  id: i64,
  /// Text of the task; 1-getOption("checklist_task_text_length_max") characters without line feeds. May contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities
  text: FormattedText,
}

impl RObject for InputChecklistTask {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputChecklistTask"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputChecklistTask {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputChecklistTaskBuilder {
    let inner = InputChecklistTask {
      td_name: "inputChecklistTask".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputChecklistTaskBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDInputChecklistTaskBuilder {
  inner: InputChecklistTask,
}

impl RTDInputChecklistTaskBuilder {
  pub fn build(&self) -> InputChecklistTask {
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
}

impl AsRef<InputChecklistTask> for InputChecklistTask {
  fn as_ref(&self) -> &InputChecklistTask {
    self
  }
}

impl AsRef<InputChecklistTask> for RTDInputChecklistTaskBuilder {
  fn as_ref(&self) -> &InputChecklistTask {
    &self.inner
  }
}
