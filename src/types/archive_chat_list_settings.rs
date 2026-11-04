use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains settings for automatic moving of chats to and from the Archive chat lists
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchiveChatListSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if new chats from non-contacts will be automatically archived and muted. Can be set to true only if the option "can_archive_and_mute_new_chats_from_unknown_users" is true
  archive_and_mute_new_chats_from_unknown_users: bool,
  /// True, if unmuted chats will be kept in the Archive chat list when they get a new message
  keep_unmuted_chats_archived: bool,
  /// True, if unmuted chats, that are always included or pinned in a folder, will be kept in the Archive chat list when they get a new message. Ignored if keep_unmuted_chats_archived == true
  keep_chats_from_folders_archived: bool,
}

impl RObject for ArchiveChatListSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "archiveChatListSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ArchiveChatListSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDArchiveChatListSettingsBuilder {
    let inner = ArchiveChatListSettings {
      td_name: "archiveChatListSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDArchiveChatListSettingsBuilder { inner }
  }

  pub fn archive_and_mute_new_chats_from_unknown_users(&self) -> bool {
    self.archive_and_mute_new_chats_from_unknown_users
  }

  pub fn keep_unmuted_chats_archived(&self) -> bool {
    self.keep_unmuted_chats_archived
  }

  pub fn keep_chats_from_folders_archived(&self) -> bool {
    self.keep_chats_from_folders_archived
  }
}

#[doc(hidden)]
pub struct RTDArchiveChatListSettingsBuilder {
  inner: ArchiveChatListSettings,
}

impl RTDArchiveChatListSettingsBuilder {
  pub fn build(&self) -> ArchiveChatListSettings {
    self.inner.clone()
  }

  pub fn archive_and_mute_new_chats_from_unknown_users(
    &mut self,
    archive_and_mute_new_chats_from_unknown_users: bool,
  ) -> &mut Self {
    self.inner.archive_and_mute_new_chats_from_unknown_users =
      archive_and_mute_new_chats_from_unknown_users;
    self
  }

  pub fn keep_unmuted_chats_archived(&mut self, keep_unmuted_chats_archived: bool) -> &mut Self {
    self.inner.keep_unmuted_chats_archived = keep_unmuted_chats_archived;
    self
  }

  pub fn keep_chats_from_folders_archived(
    &mut self,
    keep_chats_from_folders_archived: bool,
  ) -> &mut Self {
    self.inner.keep_chats_from_folders_archived = keep_chats_from_folders_archived;
    self
  }
}

impl AsRef<ArchiveChatListSettings> for ArchiveChatListSettings {
  fn as_ref(&self) -> &ArchiveChatListSettings {
    self
  }
}

impl AsRef<ArchiveChatListSettings> for RTDArchiveChatListSettingsBuilder {
  fn as_ref(&self) -> &ArchiveChatListSettings {
    &self.inner
  }
}
