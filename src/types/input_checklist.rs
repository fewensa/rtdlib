use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a checklist to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChecklist {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Title of the checklist; 1-getOption("checklist_title_length_max") characters. May contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities
  title: FormattedText,
  /// List of tasks in the checklist; 1-getOption("checklist_task_count_max") tasks
  tasks: Vec<InputChecklistTask>,
  /// True, if other users can add tasks to the list
  others_can_add_tasks: bool,
  /// True, if other users can mark tasks as done or not done
  others_can_mark_tasks_as_done: bool,
}

impl RObject for InputChecklist {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputChecklist"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputChecklist {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputChecklistBuilder {
    let inner = InputChecklist {
      td_name: "inputChecklist".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputChecklistBuilder { inner }
  }

  pub fn title(&self) -> &FormattedText {
    &self.title
  }

  pub fn tasks(&self) -> &Vec<InputChecklistTask> {
    &self.tasks
  }

  pub fn others_can_add_tasks(&self) -> bool {
    self.others_can_add_tasks
  }

  pub fn others_can_mark_tasks_as_done(&self) -> bool {
    self.others_can_mark_tasks_as_done
  }
}

#[doc(hidden)]
pub struct RTDInputChecklistBuilder {
  inner: InputChecklist,
}

impl RTDInputChecklistBuilder {
  pub fn build(&self) -> InputChecklist {
    self.inner.clone()
  }

  pub fn title<T: AsRef<FormattedText>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().clone();
    self
  }

  pub fn tasks(&mut self, tasks: Vec<InputChecklistTask>) -> &mut Self {
    self.inner.tasks = tasks;
    self
  }

  pub fn others_can_add_tasks(&mut self, others_can_add_tasks: bool) -> &mut Self {
    self.inner.others_can_add_tasks = others_can_add_tasks;
    self
  }

  pub fn others_can_mark_tasks_as_done(
    &mut self,
    others_can_mark_tasks_as_done: bool,
  ) -> &mut Self {
    self.inner.others_can_mark_tasks_as_done = others_can_mark_tasks_as_done;
    self
  }
}

impl AsRef<InputChecklist> for InputChecklist {
  fn as_ref(&self) -> &InputChecklist {
    self
  }
}

impl AsRef<InputChecklist> for RTDInputChecklistBuilder {
  fn as_ref(&self) -> &InputChecklist {
    &self.inner
  }
}
