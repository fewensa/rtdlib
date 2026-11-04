use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes usernames assigned to a user, a supergroup, or a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usernames {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of active usernames; the first one must be shown as the primary username. The order of active usernames can be changed with reorderActiveUsernames, reorderBotActiveUsernames, or reorderSupergroupActiveUsernames
  active_usernames: Vec<String>,
  /// List of currently disabled usernames; the username can be activated with toggleUsernameIsActive, toggleBotUsernameIsActive, or toggleSupergroupUsernameIsActive
  disabled_usernames: Vec<String>,
  /// Active or disabled username, which may be changed with setUsername or setSupergroupUsername
  editable_username: String,
  /// Collectible usernames that were purchased at https://fragment.com and can be passed to getCollectibleItemInfo for more details
  collectible_usernames: Vec<String>,
  /// Secondary non-collectible username. Currently, must be non-empty only for bot accounts. Can't be used in toggleBotUsernameIsActive; use addBotSecondaryUsername or deleteBotSecondaryUsername to edit it. If the username is disabled, then Telegram Premium subscription is required to re-enable it
  secondary_username: String,
}

impl RObject for Usernames {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "usernames"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Usernames {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUsernamesBuilder {
    let inner = Usernames {
      td_name: "usernames".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUsernamesBuilder { inner }
  }

  pub fn active_usernames(&self) -> &Vec<String> {
    &self.active_usernames
  }

  pub fn disabled_usernames(&self) -> &Vec<String> {
    &self.disabled_usernames
  }

  pub fn editable_username(&self) -> &String {
    &self.editable_username
  }

  pub fn collectible_usernames(&self) -> &Vec<String> {
    &self.collectible_usernames
  }

  pub fn secondary_username(&self) -> &String {
    &self.secondary_username
  }
}

#[doc(hidden)]
pub struct RTDUsernamesBuilder {
  inner: Usernames,
}

impl RTDUsernamesBuilder {
  pub fn build(&self) -> Usernames {
    self.inner.clone()
  }

  pub fn active_usernames(&mut self, active_usernames: Vec<String>) -> &mut Self {
    self.inner.active_usernames = active_usernames;
    self
  }

  pub fn disabled_usernames(&mut self, disabled_usernames: Vec<String>) -> &mut Self {
    self.inner.disabled_usernames = disabled_usernames;
    self
  }

  pub fn editable_username<T: AsRef<str>>(&mut self, editable_username: T) -> &mut Self {
    self.inner.editable_username = editable_username.as_ref().to_string();
    self
  }

  pub fn collectible_usernames(&mut self, collectible_usernames: Vec<String>) -> &mut Self {
    self.inner.collectible_usernames = collectible_usernames;
    self
  }

  pub fn secondary_username<T: AsRef<str>>(&mut self, secondary_username: T) -> &mut Self {
    self.inner.secondary_username = secondary_username.as_ref().to_string();
    self
  }
}

impl AsRef<Usernames> for Usernames {
  fn as_ref(&self) -> &Usernames {
    self
  }
}

impl AsRef<Usernames> for RTDUsernamesBuilder {
  fn as_ref(&self) -> &Usernames {
    &self.inner
  }
}
