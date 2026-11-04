use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes private chats chosen for automatic interaction with a business
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessRecipients {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifiers of selected private chats
  chat_ids: Vec<i64>,
  /// Identifiers of private chats that are always excluded; for businessConnectedBot only
  excluded_chat_ids: Vec<i64>,
  /// True, if all existing private chats are selected
  select_existing_chats: bool,
  /// True, if all new private chats are selected
  select_new_chats: bool,
  /// True, if all private chats with contacts are selected
  select_contacts: bool,
  /// True, if all private chats with non-contacts are selected
  select_non_contacts: bool,
  /// If true, then all private chats except the selected are chosen. Otherwise, only the selected chats are chosen
  exclude_selected: bool,
}

impl RObject for BusinessRecipients {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessRecipients"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessRecipients {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessRecipientsBuilder {
    let inner = BusinessRecipients {
      td_name: "businessRecipients".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessRecipientsBuilder { inner }
  }

  pub fn chat_ids(&self) -> &Vec<i64> {
    &self.chat_ids
  }

  pub fn excluded_chat_ids(&self) -> &Vec<i64> {
    &self.excluded_chat_ids
  }

  pub fn select_existing_chats(&self) -> bool {
    self.select_existing_chats
  }

  pub fn select_new_chats(&self) -> bool {
    self.select_new_chats
  }

  pub fn select_contacts(&self) -> bool {
    self.select_contacts
  }

  pub fn select_non_contacts(&self) -> bool {
    self.select_non_contacts
  }

  pub fn exclude_selected(&self) -> bool {
    self.exclude_selected
  }
}

#[doc(hidden)]
pub struct RTDBusinessRecipientsBuilder {
  inner: BusinessRecipients,
}

impl RTDBusinessRecipientsBuilder {
  pub fn build(&self) -> BusinessRecipients {
    self.inner.clone()
  }

  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }

  pub fn excluded_chat_ids(&mut self, excluded_chat_ids: Vec<i64>) -> &mut Self {
    self.inner.excluded_chat_ids = excluded_chat_ids;
    self
  }

  pub fn select_existing_chats(&mut self, select_existing_chats: bool) -> &mut Self {
    self.inner.select_existing_chats = select_existing_chats;
    self
  }

  pub fn select_new_chats(&mut self, select_new_chats: bool) -> &mut Self {
    self.inner.select_new_chats = select_new_chats;
    self
  }

  pub fn select_contacts(&mut self, select_contacts: bool) -> &mut Self {
    self.inner.select_contacts = select_contacts;
    self
  }

  pub fn select_non_contacts(&mut self, select_non_contacts: bool) -> &mut Self {
    self.inner.select_non_contacts = select_non_contacts;
    self
  }

  pub fn exclude_selected(&mut self, exclude_selected: bool) -> &mut Self {
    self.inner.exclude_selected = exclude_selected;
    self
  }
}

impl AsRef<BusinessRecipients> for BusinessRecipients {
  fn as_ref(&self) -> &BusinessRecipients {
    self
  }
}

impl AsRef<BusinessRecipients> for RTDBusinessRecipientsBuilder {
  fn as_ref(&self) -> &BusinessRecipients {
    &self.inner
  }
}
