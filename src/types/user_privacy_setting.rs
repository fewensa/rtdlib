use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes available user privacy settings
pub trait TDUserPrivacySetting: Debug + RObject {}

/// Describes available user privacy settings
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UserPrivacySetting {
  #[doc(hidden)]
  _Default(()),
  /// A privacy setting for managing whether the user can be called
  AllowCalls(UserPrivacySettingAllowCalls),
  /// A privacy setting for managing whether the user can be invited to chats
  AllowChatInvites(UserPrivacySettingAllowChatInvites),
  /// A privacy setting for managing whether the user can be found by their phone number. Checked only if the phone number is not known to the other user. Can be set only to "Allow contacts" or "Allow all"
  AllowFindingByPhoneNumber(UserPrivacySettingAllowFindingByPhoneNumber),
  /// A privacy setting for managing whether peer-to-peer connections can be used for calls
  AllowPeerToPeerCalls(UserPrivacySettingAllowPeerToPeerCalls),
  /// A privacy setting for managing whether the user can receive voice and video messages in private chats; for Telegram Premium users only
  AllowPrivateVoiceAndVideoNoteMessages(UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages),
  /// A privacy setting for managing whether the user can receive messages without additional payment
  AllowUnpaidMessages(UserPrivacySettingAllowUnpaidMessages),
  /// A privacy setting for managing whether received gifts are automatically shown on the user's profile page
  AutosaveGifts(UserPrivacySettingAutosaveGifts),
  /// A privacy setting for managing whether the user's bio is visible
  ShowBio(UserPrivacySettingShowBio),
  /// A privacy setting for managing whether the user's birthdate is visible
  ShowBirthdate(UserPrivacySettingShowBirthdate),
  /// A privacy setting for managing whether a link to the user's account is included in forwarded messages
  ShowLinkInForwardedMessages(UserPrivacySettingShowLinkInForwardedMessages),
  /// A privacy setting for managing whether the user's phone number is visible
  ShowPhoneNumber(UserPrivacySettingShowPhoneNumber),
  /// A privacy setting for managing whether the user's profile audio files are visible
  ShowProfileAudio(UserPrivacySettingShowProfileAudio),
  /// A privacy setting for managing whether the user's profile photo is visible
  ShowProfilePhoto(UserPrivacySettingShowProfilePhoto),
  /// A privacy setting for managing whether the user's online status is visible
  ShowStatus(UserPrivacySettingShowStatus),
}

impl Default for UserPrivacySetting {
  fn default() -> Self {
    UserPrivacySetting::_Default(())
  }
}

impl<'de> Deserialize<'de> for UserPrivacySetting {
  fn deserialize<D>(deserializer: D) -> Result<UserPrivacySetting, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      UserPrivacySetting,
      (userPrivacySettingAllowCalls, AllowCalls);
      (userPrivacySettingAllowChatInvites, AllowChatInvites);
      (userPrivacySettingAllowFindingByPhoneNumber, AllowFindingByPhoneNumber);
      (userPrivacySettingAllowPeerToPeerCalls, AllowPeerToPeerCalls);
      (userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages, AllowPrivateVoiceAndVideoNoteMessages);
      (userPrivacySettingAllowUnpaidMessages, AllowUnpaidMessages);
      (userPrivacySettingAutosaveGifts, AutosaveGifts);
      (userPrivacySettingShowBio, ShowBio);
      (userPrivacySettingShowBirthdate, ShowBirthdate);
      (userPrivacySettingShowLinkInForwardedMessages, ShowLinkInForwardedMessages);
      (userPrivacySettingShowPhoneNumber, ShowPhoneNumber);
      (userPrivacySettingShowProfileAudio, ShowProfileAudio);
      (userPrivacySettingShowProfilePhoto, ShowProfilePhoto);
      (userPrivacySettingShowStatus, ShowStatus);

    )(deserializer)
  }
}

impl RObject for UserPrivacySetting {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      UserPrivacySetting::AllowCalls(t) => t.td_name(),
      UserPrivacySetting::AllowChatInvites(t) => t.td_name(),
      UserPrivacySetting::AllowFindingByPhoneNumber(t) => t.td_name(),
      UserPrivacySetting::AllowPeerToPeerCalls(t) => t.td_name(),
      UserPrivacySetting::AllowPrivateVoiceAndVideoNoteMessages(t) => t.td_name(),
      UserPrivacySetting::AllowUnpaidMessages(t) => t.td_name(),
      UserPrivacySetting::AutosaveGifts(t) => t.td_name(),
      UserPrivacySetting::ShowBio(t) => t.td_name(),
      UserPrivacySetting::ShowBirthdate(t) => t.td_name(),
      UserPrivacySetting::ShowLinkInForwardedMessages(t) => t.td_name(),
      UserPrivacySetting::ShowPhoneNumber(t) => t.td_name(),
      UserPrivacySetting::ShowProfileAudio(t) => t.td_name(),
      UserPrivacySetting::ShowProfilePhoto(t) => t.td_name(),
      UserPrivacySetting::ShowStatus(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      UserPrivacySetting::AllowCalls(t) => t.extra(),
      UserPrivacySetting::AllowChatInvites(t) => t.extra(),
      UserPrivacySetting::AllowFindingByPhoneNumber(t) => t.extra(),
      UserPrivacySetting::AllowPeerToPeerCalls(t) => t.extra(),
      UserPrivacySetting::AllowPrivateVoiceAndVideoNoteMessages(t) => t.extra(),
      UserPrivacySetting::AllowUnpaidMessages(t) => t.extra(),
      UserPrivacySetting::AutosaveGifts(t) => t.extra(),
      UserPrivacySetting::ShowBio(t) => t.extra(),
      UserPrivacySetting::ShowBirthdate(t) => t.extra(),
      UserPrivacySetting::ShowLinkInForwardedMessages(t) => t.extra(),
      UserPrivacySetting::ShowPhoneNumber(t) => t.extra(),
      UserPrivacySetting::ShowProfileAudio(t) => t.extra(),
      UserPrivacySetting::ShowProfilePhoto(t) => t.extra(),
      UserPrivacySetting::ShowStatus(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UserPrivacySetting {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, UserPrivacySetting::_Default(_))
  }

  pub fn is_allow_calls(&self) -> bool {
    matches!(self, UserPrivacySetting::AllowCalls(_))
  }
  pub fn is_allow_chat_invites(&self) -> bool {
    matches!(self, UserPrivacySetting::AllowChatInvites(_))
  }
  pub fn is_allow_finding_by_phone_number(&self) -> bool {
    matches!(self, UserPrivacySetting::AllowFindingByPhoneNumber(_))
  }
  pub fn is_allow_peer_to_peer_calls(&self) -> bool {
    matches!(self, UserPrivacySetting::AllowPeerToPeerCalls(_))
  }
  pub fn is_allow_private_voice_and_video_note_messages(&self) -> bool {
    matches!(
      self,
      UserPrivacySetting::AllowPrivateVoiceAndVideoNoteMessages(_)
    )
  }
  pub fn is_allow_unpaid_messages(&self) -> bool {
    matches!(self, UserPrivacySetting::AllowUnpaidMessages(_))
  }
  pub fn is_autosave_gifts(&self) -> bool {
    matches!(self, UserPrivacySetting::AutosaveGifts(_))
  }
  pub fn is_show_bio(&self) -> bool {
    matches!(self, UserPrivacySetting::ShowBio(_))
  }
  pub fn is_show_birthdate(&self) -> bool {
    matches!(self, UserPrivacySetting::ShowBirthdate(_))
  }
  pub fn is_show_link_in_forwarded_messages(&self) -> bool {
    matches!(self, UserPrivacySetting::ShowLinkInForwardedMessages(_))
  }
  pub fn is_show_phone_number(&self) -> bool {
    matches!(self, UserPrivacySetting::ShowPhoneNumber(_))
  }
  pub fn is_show_profile_audio(&self) -> bool {
    matches!(self, UserPrivacySetting::ShowProfileAudio(_))
  }
  pub fn is_show_profile_photo(&self) -> bool {
    matches!(self, UserPrivacySetting::ShowProfilePhoto(_))
  }
  pub fn is_show_status(&self) -> bool {
    matches!(self, UserPrivacySetting::ShowStatus(_))
  }

  pub fn on_allow_calls<F: FnOnce(&UserPrivacySettingAllowCalls)>(&self, fnc: F) -> &Self {
    if let UserPrivacySetting::AllowCalls(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_chat_invites<F: FnOnce(&UserPrivacySettingAllowChatInvites)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySetting::AllowChatInvites(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_finding_by_phone_number<
    F: FnOnce(&UserPrivacySettingAllowFindingByPhoneNumber),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySetting::AllowFindingByPhoneNumber(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_peer_to_peer_calls<F: FnOnce(&UserPrivacySettingAllowPeerToPeerCalls)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySetting::AllowPeerToPeerCalls(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_private_voice_and_video_note_messages<
    F: FnOnce(&UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySetting::AllowPrivateVoiceAndVideoNoteMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_allow_unpaid_messages<F: FnOnce(&UserPrivacySettingAllowUnpaidMessages)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySetting::AllowUnpaidMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_autosave_gifts<F: FnOnce(&UserPrivacySettingAutosaveGifts)>(&self, fnc: F) -> &Self {
    if let UserPrivacySetting::AutosaveGifts(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_show_bio<F: FnOnce(&UserPrivacySettingShowBio)>(&self, fnc: F) -> &Self {
    if let UserPrivacySetting::ShowBio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_show_birthdate<F: FnOnce(&UserPrivacySettingShowBirthdate)>(&self, fnc: F) -> &Self {
    if let UserPrivacySetting::ShowBirthdate(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_show_link_in_forwarded_messages<
    F: FnOnce(&UserPrivacySettingShowLinkInForwardedMessages),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySetting::ShowLinkInForwardedMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_show_phone_number<F: FnOnce(&UserPrivacySettingShowPhoneNumber)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySetting::ShowPhoneNumber(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_show_profile_audio<F: FnOnce(&UserPrivacySettingShowProfileAudio)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySetting::ShowProfileAudio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_show_profile_photo<F: FnOnce(&UserPrivacySettingShowProfilePhoto)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let UserPrivacySetting::ShowProfilePhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_show_status<F: FnOnce(&UserPrivacySettingShowStatus)>(&self, fnc: F) -> &Self {
    if let UserPrivacySetting::ShowStatus(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_allow_calls(&self) -> Option<&UserPrivacySettingAllowCalls> {
    if let UserPrivacySetting::AllowCalls(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_chat_invites(&self) -> Option<&UserPrivacySettingAllowChatInvites> {
    if let UserPrivacySetting::AllowChatInvites(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_finding_by_phone_number(
    &self,
  ) -> Option<&UserPrivacySettingAllowFindingByPhoneNumber> {
    if let UserPrivacySetting::AllowFindingByPhoneNumber(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_peer_to_peer_calls(&self) -> Option<&UserPrivacySettingAllowPeerToPeerCalls> {
    if let UserPrivacySetting::AllowPeerToPeerCalls(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_private_voice_and_video_note_messages(
    &self,
  ) -> Option<&UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages> {
    if let UserPrivacySetting::AllowPrivateVoiceAndVideoNoteMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_allow_unpaid_messages(&self) -> Option<&UserPrivacySettingAllowUnpaidMessages> {
    if let UserPrivacySetting::AllowUnpaidMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_autosave_gifts(&self) -> Option<&UserPrivacySettingAutosaveGifts> {
    if let UserPrivacySetting::AutosaveGifts(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_show_bio(&self) -> Option<&UserPrivacySettingShowBio> {
    if let UserPrivacySetting::ShowBio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_show_birthdate(&self) -> Option<&UserPrivacySettingShowBirthdate> {
    if let UserPrivacySetting::ShowBirthdate(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_show_link_in_forwarded_messages(
    &self,
  ) -> Option<&UserPrivacySettingShowLinkInForwardedMessages> {
    if let UserPrivacySetting::ShowLinkInForwardedMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_show_phone_number(&self) -> Option<&UserPrivacySettingShowPhoneNumber> {
    if let UserPrivacySetting::ShowPhoneNumber(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_show_profile_audio(&self) -> Option<&UserPrivacySettingShowProfileAudio> {
    if let UserPrivacySetting::ShowProfileAudio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_show_profile_photo(&self) -> Option<&UserPrivacySettingShowProfilePhoto> {
    if let UserPrivacySetting::ShowProfilePhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_show_status(&self) -> Option<&UserPrivacySettingShowStatus> {
    if let UserPrivacySetting::ShowStatus(t) = self {
      return Some(t);
    }
    None
  }

  pub fn allow_calls<T: AsRef<UserPrivacySettingAllowCalls>>(t: T) -> Self {
    UserPrivacySetting::AllowCalls(t.as_ref().clone())
  }

  pub fn allow_chat_invites<T: AsRef<UserPrivacySettingAllowChatInvites>>(t: T) -> Self {
    UserPrivacySetting::AllowChatInvites(t.as_ref().clone())
  }

  pub fn allow_finding_by_phone_number<T: AsRef<UserPrivacySettingAllowFindingByPhoneNumber>>(
    t: T,
  ) -> Self {
    UserPrivacySetting::AllowFindingByPhoneNumber(t.as_ref().clone())
  }

  pub fn allow_peer_to_peer_calls<T: AsRef<UserPrivacySettingAllowPeerToPeerCalls>>(t: T) -> Self {
    UserPrivacySetting::AllowPeerToPeerCalls(t.as_ref().clone())
  }

  pub fn allow_private_voice_and_video_note_messages<
    T: AsRef<UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages>,
  >(
    t: T,
  ) -> Self {
    UserPrivacySetting::AllowPrivateVoiceAndVideoNoteMessages(t.as_ref().clone())
  }

  pub fn allow_unpaid_messages<T: AsRef<UserPrivacySettingAllowUnpaidMessages>>(t: T) -> Self {
    UserPrivacySetting::AllowUnpaidMessages(t.as_ref().clone())
  }

  pub fn autosave_gifts<T: AsRef<UserPrivacySettingAutosaveGifts>>(t: T) -> Self {
    UserPrivacySetting::AutosaveGifts(t.as_ref().clone())
  }

  pub fn show_bio<T: AsRef<UserPrivacySettingShowBio>>(t: T) -> Self {
    UserPrivacySetting::ShowBio(t.as_ref().clone())
  }

  pub fn show_birthdate<T: AsRef<UserPrivacySettingShowBirthdate>>(t: T) -> Self {
    UserPrivacySetting::ShowBirthdate(t.as_ref().clone())
  }

  pub fn show_link_in_forwarded_messages<
    T: AsRef<UserPrivacySettingShowLinkInForwardedMessages>,
  >(
    t: T,
  ) -> Self {
    UserPrivacySetting::ShowLinkInForwardedMessages(t.as_ref().clone())
  }

  pub fn show_phone_number<T: AsRef<UserPrivacySettingShowPhoneNumber>>(t: T) -> Self {
    UserPrivacySetting::ShowPhoneNumber(t.as_ref().clone())
  }

  pub fn show_profile_audio<T: AsRef<UserPrivacySettingShowProfileAudio>>(t: T) -> Self {
    UserPrivacySetting::ShowProfileAudio(t.as_ref().clone())
  }

  pub fn show_profile_photo<T: AsRef<UserPrivacySettingShowProfilePhoto>>(t: T) -> Self {
    UserPrivacySetting::ShowProfilePhoto(t.as_ref().clone())
  }

  pub fn show_status<T: AsRef<UserPrivacySettingShowStatus>>(t: T) -> Self {
    UserPrivacySetting::ShowStatus(t.as_ref().clone())
  }
}

impl AsRef<UserPrivacySetting> for UserPrivacySetting {
  fn as_ref(&self) -> &UserPrivacySetting {
    self
  }
}

/// A privacy setting for managing whether the user can be called
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowCalls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingAllowCalls {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingAllowCalls"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowCalls {}

impl UserPrivacySettingAllowCalls {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingAllowCallsBuilder {
    let inner = UserPrivacySettingAllowCalls {
      td_name: "userPrivacySettingAllowCalls".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingAllowCallsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAllowCallsBuilder {
  inner: UserPrivacySettingAllowCalls,
}

impl RTDUserPrivacySettingAllowCallsBuilder {
  pub fn build(&self) -> UserPrivacySettingAllowCalls {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingAllowCalls> for UserPrivacySettingAllowCalls {
  fn as_ref(&self) -> &UserPrivacySettingAllowCalls {
    self
  }
}

impl AsRef<UserPrivacySettingAllowCalls> for RTDUserPrivacySettingAllowCallsBuilder {
  fn as_ref(&self) -> &UserPrivacySettingAllowCalls {
    &self.inner
  }
}

/// A privacy setting for managing whether the user can be invited to chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowChatInvites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingAllowChatInvites {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingAllowChatInvites"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowChatInvites {}

impl UserPrivacySettingAllowChatInvites {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingAllowChatInvitesBuilder {
    let inner = UserPrivacySettingAllowChatInvites {
      td_name: "userPrivacySettingAllowChatInvites".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingAllowChatInvitesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAllowChatInvitesBuilder {
  inner: UserPrivacySettingAllowChatInvites,
}

impl RTDUserPrivacySettingAllowChatInvitesBuilder {
  pub fn build(&self) -> UserPrivacySettingAllowChatInvites {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingAllowChatInvites> for UserPrivacySettingAllowChatInvites {
  fn as_ref(&self) -> &UserPrivacySettingAllowChatInvites {
    self
  }
}

impl AsRef<UserPrivacySettingAllowChatInvites> for RTDUserPrivacySettingAllowChatInvitesBuilder {
  fn as_ref(&self) -> &UserPrivacySettingAllowChatInvites {
    &self.inner
  }
}

/// A privacy setting for managing whether the user can be found by their phone number. Checked only if the phone number is not known to the other user. Can be set only to "Allow contacts" or "Allow all"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowFindingByPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingAllowFindingByPhoneNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingAllowFindingByPhoneNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowFindingByPhoneNumber {}

impl UserPrivacySettingAllowFindingByPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingAllowFindingByPhoneNumberBuilder {
    let inner = UserPrivacySettingAllowFindingByPhoneNumber {
      td_name: "userPrivacySettingAllowFindingByPhoneNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingAllowFindingByPhoneNumberBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAllowFindingByPhoneNumberBuilder {
  inner: UserPrivacySettingAllowFindingByPhoneNumber,
}

impl RTDUserPrivacySettingAllowFindingByPhoneNumberBuilder {
  pub fn build(&self) -> UserPrivacySettingAllowFindingByPhoneNumber {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingAllowFindingByPhoneNumber>
  for UserPrivacySettingAllowFindingByPhoneNumber
{
  fn as_ref(&self) -> &UserPrivacySettingAllowFindingByPhoneNumber {
    self
  }
}

impl AsRef<UserPrivacySettingAllowFindingByPhoneNumber>
  for RTDUserPrivacySettingAllowFindingByPhoneNumberBuilder
{
  fn as_ref(&self) -> &UserPrivacySettingAllowFindingByPhoneNumber {
    &self.inner
  }
}

/// A privacy setting for managing whether peer-to-peer connections can be used for calls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowPeerToPeerCalls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingAllowPeerToPeerCalls {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingAllowPeerToPeerCalls"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowPeerToPeerCalls {}

impl UserPrivacySettingAllowPeerToPeerCalls {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingAllowPeerToPeerCallsBuilder {
    let inner = UserPrivacySettingAllowPeerToPeerCalls {
      td_name: "userPrivacySettingAllowPeerToPeerCalls".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingAllowPeerToPeerCallsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAllowPeerToPeerCallsBuilder {
  inner: UserPrivacySettingAllowPeerToPeerCalls,
}

impl RTDUserPrivacySettingAllowPeerToPeerCallsBuilder {
  pub fn build(&self) -> UserPrivacySettingAllowPeerToPeerCalls {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingAllowPeerToPeerCalls> for UserPrivacySettingAllowPeerToPeerCalls {
  fn as_ref(&self) -> &UserPrivacySettingAllowPeerToPeerCalls {
    self
  }
}

impl AsRef<UserPrivacySettingAllowPeerToPeerCalls>
  for RTDUserPrivacySettingAllowPeerToPeerCallsBuilder
{
  fn as_ref(&self) -> &UserPrivacySettingAllowPeerToPeerCalls {
    &self.inner
  }
}

/// A privacy setting for managing whether the user can receive voice and video messages in private chats; for Telegram Premium users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages {}

impl UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingAllowPrivateVoiceAndVideoNoteMessagesBuilder {
    let inner = UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages {
      td_name: "userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingAllowPrivateVoiceAndVideoNoteMessagesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAllowPrivateVoiceAndVideoNoteMessagesBuilder {
  inner: UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages,
}

impl RTDUserPrivacySettingAllowPrivateVoiceAndVideoNoteMessagesBuilder {
  pub fn build(&self) -> UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages>
  for UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages
{
  fn as_ref(&self) -> &UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages {
    self
  }
}

impl AsRef<UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages>
  for RTDUserPrivacySettingAllowPrivateVoiceAndVideoNoteMessagesBuilder
{
  fn as_ref(&self) -> &UserPrivacySettingAllowPrivateVoiceAndVideoNoteMessages {
    &self.inner
  }
}

/// A privacy setting for managing whether the user can receive messages without additional payment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowUnpaidMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingAllowUnpaidMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingAllowUnpaidMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowUnpaidMessages {}

impl UserPrivacySettingAllowUnpaidMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingAllowUnpaidMessagesBuilder {
    let inner = UserPrivacySettingAllowUnpaidMessages {
      td_name: "userPrivacySettingAllowUnpaidMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingAllowUnpaidMessagesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAllowUnpaidMessagesBuilder {
  inner: UserPrivacySettingAllowUnpaidMessages,
}

impl RTDUserPrivacySettingAllowUnpaidMessagesBuilder {
  pub fn build(&self) -> UserPrivacySettingAllowUnpaidMessages {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingAllowUnpaidMessages> for UserPrivacySettingAllowUnpaidMessages {
  fn as_ref(&self) -> &UserPrivacySettingAllowUnpaidMessages {
    self
  }
}

impl AsRef<UserPrivacySettingAllowUnpaidMessages>
  for RTDUserPrivacySettingAllowUnpaidMessagesBuilder
{
  fn as_ref(&self) -> &UserPrivacySettingAllowUnpaidMessages {
    &self.inner
  }
}

/// A privacy setting for managing whether received gifts are automatically shown on the user's profile page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAutosaveGifts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingAutosaveGifts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingAutosaveGifts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingAutosaveGifts {}

impl UserPrivacySettingAutosaveGifts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingAutosaveGiftsBuilder {
    let inner = UserPrivacySettingAutosaveGifts {
      td_name: "userPrivacySettingAutosaveGifts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingAutosaveGiftsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingAutosaveGiftsBuilder {
  inner: UserPrivacySettingAutosaveGifts,
}

impl RTDUserPrivacySettingAutosaveGiftsBuilder {
  pub fn build(&self) -> UserPrivacySettingAutosaveGifts {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingAutosaveGifts> for UserPrivacySettingAutosaveGifts {
  fn as_ref(&self) -> &UserPrivacySettingAutosaveGifts {
    self
  }
}

impl AsRef<UserPrivacySettingAutosaveGifts> for RTDUserPrivacySettingAutosaveGiftsBuilder {
  fn as_ref(&self) -> &UserPrivacySettingAutosaveGifts {
    &self.inner
  }
}

/// A privacy setting for managing whether the user's bio is visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowBio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingShowBio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingShowBio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingShowBio {}

impl UserPrivacySettingShowBio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingShowBioBuilder {
    let inner = UserPrivacySettingShowBio {
      td_name: "userPrivacySettingShowBio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingShowBioBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingShowBioBuilder {
  inner: UserPrivacySettingShowBio,
}

impl RTDUserPrivacySettingShowBioBuilder {
  pub fn build(&self) -> UserPrivacySettingShowBio {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingShowBio> for UserPrivacySettingShowBio {
  fn as_ref(&self) -> &UserPrivacySettingShowBio {
    self
  }
}

impl AsRef<UserPrivacySettingShowBio> for RTDUserPrivacySettingShowBioBuilder {
  fn as_ref(&self) -> &UserPrivacySettingShowBio {
    &self.inner
  }
}

/// A privacy setting for managing whether the user's birthdate is visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowBirthdate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingShowBirthdate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingShowBirthdate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingShowBirthdate {}

impl UserPrivacySettingShowBirthdate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingShowBirthdateBuilder {
    let inner = UserPrivacySettingShowBirthdate {
      td_name: "userPrivacySettingShowBirthdate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingShowBirthdateBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingShowBirthdateBuilder {
  inner: UserPrivacySettingShowBirthdate,
}

impl RTDUserPrivacySettingShowBirthdateBuilder {
  pub fn build(&self) -> UserPrivacySettingShowBirthdate {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingShowBirthdate> for UserPrivacySettingShowBirthdate {
  fn as_ref(&self) -> &UserPrivacySettingShowBirthdate {
    self
  }
}

impl AsRef<UserPrivacySettingShowBirthdate> for RTDUserPrivacySettingShowBirthdateBuilder {
  fn as_ref(&self) -> &UserPrivacySettingShowBirthdate {
    &self.inner
  }
}

/// A privacy setting for managing whether a link to the user's account is included in forwarded messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowLinkInForwardedMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingShowLinkInForwardedMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingShowLinkInForwardedMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingShowLinkInForwardedMessages {}

impl UserPrivacySettingShowLinkInForwardedMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingShowLinkInForwardedMessagesBuilder {
    let inner = UserPrivacySettingShowLinkInForwardedMessages {
      td_name: "userPrivacySettingShowLinkInForwardedMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingShowLinkInForwardedMessagesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingShowLinkInForwardedMessagesBuilder {
  inner: UserPrivacySettingShowLinkInForwardedMessages,
}

impl RTDUserPrivacySettingShowLinkInForwardedMessagesBuilder {
  pub fn build(&self) -> UserPrivacySettingShowLinkInForwardedMessages {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingShowLinkInForwardedMessages>
  for UserPrivacySettingShowLinkInForwardedMessages
{
  fn as_ref(&self) -> &UserPrivacySettingShowLinkInForwardedMessages {
    self
  }
}

impl AsRef<UserPrivacySettingShowLinkInForwardedMessages>
  for RTDUserPrivacySettingShowLinkInForwardedMessagesBuilder
{
  fn as_ref(&self) -> &UserPrivacySettingShowLinkInForwardedMessages {
    &self.inner
  }
}

/// A privacy setting for managing whether the user's phone number is visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingShowPhoneNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingShowPhoneNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingShowPhoneNumber {}

impl UserPrivacySettingShowPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingShowPhoneNumberBuilder {
    let inner = UserPrivacySettingShowPhoneNumber {
      td_name: "userPrivacySettingShowPhoneNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingShowPhoneNumberBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingShowPhoneNumberBuilder {
  inner: UserPrivacySettingShowPhoneNumber,
}

impl RTDUserPrivacySettingShowPhoneNumberBuilder {
  pub fn build(&self) -> UserPrivacySettingShowPhoneNumber {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingShowPhoneNumber> for UserPrivacySettingShowPhoneNumber {
  fn as_ref(&self) -> &UserPrivacySettingShowPhoneNumber {
    self
  }
}

impl AsRef<UserPrivacySettingShowPhoneNumber> for RTDUserPrivacySettingShowPhoneNumberBuilder {
  fn as_ref(&self) -> &UserPrivacySettingShowPhoneNumber {
    &self.inner
  }
}

/// A privacy setting for managing whether the user's profile audio files are visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowProfileAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingShowProfileAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingShowProfileAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingShowProfileAudio {}

impl UserPrivacySettingShowProfileAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingShowProfileAudioBuilder {
    let inner = UserPrivacySettingShowProfileAudio {
      td_name: "userPrivacySettingShowProfileAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingShowProfileAudioBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingShowProfileAudioBuilder {
  inner: UserPrivacySettingShowProfileAudio,
}

impl RTDUserPrivacySettingShowProfileAudioBuilder {
  pub fn build(&self) -> UserPrivacySettingShowProfileAudio {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingShowProfileAudio> for UserPrivacySettingShowProfileAudio {
  fn as_ref(&self) -> &UserPrivacySettingShowProfileAudio {
    self
  }
}

impl AsRef<UserPrivacySettingShowProfileAudio> for RTDUserPrivacySettingShowProfileAudioBuilder {
  fn as_ref(&self) -> &UserPrivacySettingShowProfileAudio {
    &self.inner
  }
}

/// A privacy setting for managing whether the user's profile photo is visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingShowProfilePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingShowProfilePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingShowProfilePhoto {}

impl UserPrivacySettingShowProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingShowProfilePhotoBuilder {
    let inner = UserPrivacySettingShowProfilePhoto {
      td_name: "userPrivacySettingShowProfilePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingShowProfilePhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingShowProfilePhotoBuilder {
  inner: UserPrivacySettingShowProfilePhoto,
}

impl RTDUserPrivacySettingShowProfilePhotoBuilder {
  pub fn build(&self) -> UserPrivacySettingShowProfilePhoto {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingShowProfilePhoto> for UserPrivacySettingShowProfilePhoto {
  fn as_ref(&self) -> &UserPrivacySettingShowProfilePhoto {
    self
  }
}

impl AsRef<UserPrivacySettingShowProfilePhoto> for RTDUserPrivacySettingShowProfilePhotoBuilder {
  fn as_ref(&self) -> &UserPrivacySettingShowProfilePhoto {
    &self.inner
  }
}

/// A privacy setting for managing whether the user's online status is visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UserPrivacySettingShowStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userPrivacySettingShowStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUserPrivacySetting for UserPrivacySettingShowStatus {}

impl UserPrivacySettingShowStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserPrivacySettingShowStatusBuilder {
    let inner = UserPrivacySettingShowStatus {
      td_name: "userPrivacySettingShowStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUserPrivacySettingShowStatusBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingShowStatusBuilder {
  inner: UserPrivacySettingShowStatus,
}

impl RTDUserPrivacySettingShowStatusBuilder {
  pub fn build(&self) -> UserPrivacySettingShowStatus {
    self.inner.clone()
  }
}

impl AsRef<UserPrivacySettingShowStatus> for UserPrivacySettingShowStatus {
  fn as_ref(&self) -> &UserPrivacySettingShowStatus {
    self
  }
}

impl AsRef<UserPrivacySettingShowStatus> for RTDUserPrivacySettingShowStatusBuilder {
  fn as_ref(&self) -> &UserPrivacySettingShowStatus {
    &self.inner
  }
}
