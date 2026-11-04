use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a checklist
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Checklist {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Title of the checklist; may contain only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities
  title: FormattedText,
  /// List of tasks in the checklist
  tasks: Vec<ChecklistTask>,
  /// True, if users other than creator of the list can add tasks to the list
  others_can_add_tasks: bool,
  /// True, if the current user can add tasks to the list if they have Telegram Premium subscription
  can_add_tasks: bool,
  /// True, if users other than creator of the list can mark tasks as done or not done. If true, then the checklist is called "group checklist"
  others_can_mark_tasks_as_done: bool,
  /// True, if the current user can mark tasks as done or not done if they have Telegram Premium subscription
  can_mark_tasks_as_done: bool,
}

impl RObject for Checklist {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "checklist"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Checklist {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChecklistBuilder {
    let inner = Checklist {
      td_name: "checklist".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChecklistBuilder { inner }
  }

  pub fn title(&self) -> &FormattedText {
    &self.title
  }

  pub fn tasks(&self) -> &Vec<ChecklistTask> {
    &self.tasks
  }

  pub fn others_can_add_tasks(&self) -> bool {
    self.others_can_add_tasks
  }

  pub fn can_add_tasks(&self) -> bool {
    self.can_add_tasks
  }

  pub fn others_can_mark_tasks_as_done(&self) -> bool {
    self.others_can_mark_tasks_as_done
  }

  pub fn can_mark_tasks_as_done(&self) -> bool {
    self.can_mark_tasks_as_done
  }
}

#[doc(hidden)]
pub struct RTDChecklistBuilder {
  inner: Checklist,
}

impl RTDChecklistBuilder {
  pub fn build(&self) -> Checklist {
    self.inner.clone()
  }

  pub fn title<T: AsRef<FormattedText>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().clone();
    self
  }

  pub fn tasks(&mut self, tasks: Vec<ChecklistTask>) -> &mut Self {
    self.inner.tasks = tasks;
    self
  }

  pub fn others_can_add_tasks(&mut self, others_can_add_tasks: bool) -> &mut Self {
    self.inner.others_can_add_tasks = others_can_add_tasks;
    self
  }

  pub fn can_add_tasks(&mut self, can_add_tasks: bool) -> &mut Self {
    self.inner.can_add_tasks = can_add_tasks;
    self
  }

  pub fn others_can_mark_tasks_as_done(
    &mut self,
    others_can_mark_tasks_as_done: bool,
  ) -> &mut Self {
    self.inner.others_can_mark_tasks_as_done = others_can_mark_tasks_as_done;
    self
  }

  pub fn can_mark_tasks_as_done(&mut self, can_mark_tasks_as_done: bool) -> &mut Self {
    self.inner.can_mark_tasks_as_done = can_mark_tasks_as_done;
    self
  }
}

impl AsRef<Checklist> for Checklist {
  fn as_ref(&self) -> &Checklist {
    self
  }
}

impl AsRef<Checklist> for RTDChecklistBuilder {
  fn as_ref(&self) -> &Checklist {
    &self.inner
  }
}
