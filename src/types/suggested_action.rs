use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes an action suggested to the current user
pub trait TDSuggestedAction: Debug + RObject {}

/// Describes an action suggested to the current user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SuggestedAction {
  #[doc(hidden)]
  _Default(()),
  /// Suggests the user to add a passkey for login using addLoginPasskey
  AddLoginPasskey(SuggestedActionAddLoginPasskey),
  /// Suggests the user to check whether they still remember their 2-step verification password
  CheckPassword(SuggestedActionCheckPassword),
  /// Suggests the user to check whether authorization phone number is correct and change the phone number if it is inaccessible
  CheckPhoneNumber(SuggestedActionCheckPhoneNumber),
  /// Suggests the user to convert specified supergroup to a broadcast group
  ConvertToBroadcastGroup(SuggestedActionConvertToBroadcastGroup),
  /// A custom suggestion to be shown at the top of the chat list
  Custom(SuggestedActionCustom),
  /// Suggests the user to enable archive_and_mute_new_chats_from_unknown_users setting in archiveChatListSettings
  EnableArchiveAndMuteNewChats(SuggestedActionEnableArchiveAndMuteNewChats),
  /// Suggests the user to extend their expiring Telegram Premium subscription
  ExtendPremium(SuggestedActionExtendPremium),
  /// Suggests the user to extend their expiring Telegram Star subscriptions. Call getStarSubscriptions with only_expiring == true to get the number of expiring subscriptions and the number of required to buy Telegram Stars
  ExtendStarSubscriptions(SuggestedActionExtendStarSubscriptions),
  /// Suggests the user to gift Telegram Premium to friends for Christmas
  GiftPremiumForChristmas(SuggestedActionGiftPremiumForChristmas),
  /// Suggests the user to restore a recently expired Premium subscription
  RestorePremium(SuggestedActionRestorePremium),
  /// Suggests the user to set birthdate
  SetBirthdate(SuggestedActionSetBirthdate),
  /// Suggests the user to add login email address. Call isLoginEmailAddressRequired, and then setLoginEmailAddress or checkLoginEmailAddressCode to change the login email address
  SetLoginEmailAddress(SuggestedActionSetLoginEmailAddress),
  /// Suggests the user to set a 2-step verification password to be able to log in again
  SetPassword(SuggestedActionSetPassword),
  /// Suggests the user to set profile photo
  SetProfilePhoto(SuggestedActionSetProfilePhoto),
  /// Suggests the user to subscribe to the Premium subscription with annual payments
  SubscribeToAnnualPremium(SuggestedActionSubscribeToAnnualPremium),
  /// Suggests the user to upgrade the Premium subscription from monthly payments to annual payments
  UpgradePremium(SuggestedActionUpgradePremium),
  /// Suggests the user to view a hint about the meaning of one and two check marks on sent messages
  ViewChecksHint(SuggestedActionViewChecksHint),
  /// Suggests the user to view a hint about receiving of the first TON wallet transfer
  ViewFirstTonWalletTransferHint(SuggestedActionViewFirstTonWalletTransferHint),
}

impl Default for SuggestedAction {
  fn default() -> Self {
    SuggestedAction::_Default(())
  }
}

impl<'de> Deserialize<'de> for SuggestedAction {
  fn deserialize<D>(deserializer: D) -> Result<SuggestedAction, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SuggestedAction,
      (suggestedActionAddLoginPasskey, AddLoginPasskey);
      (suggestedActionCheckPassword, CheckPassword);
      (suggestedActionCheckPhoneNumber, CheckPhoneNumber);
      (suggestedActionConvertToBroadcastGroup, ConvertToBroadcastGroup);
      (suggestedActionCustom, Custom);
      (suggestedActionEnableArchiveAndMuteNewChats, EnableArchiveAndMuteNewChats);
      (suggestedActionExtendPremium, ExtendPremium);
      (suggestedActionExtendStarSubscriptions, ExtendStarSubscriptions);
      (suggestedActionGiftPremiumForChristmas, GiftPremiumForChristmas);
      (suggestedActionRestorePremium, RestorePremium);
      (suggestedActionSetBirthdate, SetBirthdate);
      (suggestedActionSetLoginEmailAddress, SetLoginEmailAddress);
      (suggestedActionSetPassword, SetPassword);
      (suggestedActionSetProfilePhoto, SetProfilePhoto);
      (suggestedActionSubscribeToAnnualPremium, SubscribeToAnnualPremium);
      (suggestedActionUpgradePremium, UpgradePremium);
      (suggestedActionViewChecksHint, ViewChecksHint);
      (suggestedActionViewFirstTonWalletTransferHint, ViewFirstTonWalletTransferHint);

    )(deserializer)
  }
}

impl RObject for SuggestedAction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SuggestedAction::AddLoginPasskey(t) => t.td_name(),
      SuggestedAction::CheckPassword(t) => t.td_name(),
      SuggestedAction::CheckPhoneNumber(t) => t.td_name(),
      SuggestedAction::ConvertToBroadcastGroup(t) => t.td_name(),
      SuggestedAction::Custom(t) => t.td_name(),
      SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.td_name(),
      SuggestedAction::ExtendPremium(t) => t.td_name(),
      SuggestedAction::ExtendStarSubscriptions(t) => t.td_name(),
      SuggestedAction::GiftPremiumForChristmas(t) => t.td_name(),
      SuggestedAction::RestorePremium(t) => t.td_name(),
      SuggestedAction::SetBirthdate(t) => t.td_name(),
      SuggestedAction::SetLoginEmailAddress(t) => t.td_name(),
      SuggestedAction::SetPassword(t) => t.td_name(),
      SuggestedAction::SetProfilePhoto(t) => t.td_name(),
      SuggestedAction::SubscribeToAnnualPremium(t) => t.td_name(),
      SuggestedAction::UpgradePremium(t) => t.td_name(),
      SuggestedAction::ViewChecksHint(t) => t.td_name(),
      SuggestedAction::ViewFirstTonWalletTransferHint(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SuggestedAction::AddLoginPasskey(t) => t.extra(),
      SuggestedAction::CheckPassword(t) => t.extra(),
      SuggestedAction::CheckPhoneNumber(t) => t.extra(),
      SuggestedAction::ConvertToBroadcastGroup(t) => t.extra(),
      SuggestedAction::Custom(t) => t.extra(),
      SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.extra(),
      SuggestedAction::ExtendPremium(t) => t.extra(),
      SuggestedAction::ExtendStarSubscriptions(t) => t.extra(),
      SuggestedAction::GiftPremiumForChristmas(t) => t.extra(),
      SuggestedAction::RestorePremium(t) => t.extra(),
      SuggestedAction::SetBirthdate(t) => t.extra(),
      SuggestedAction::SetLoginEmailAddress(t) => t.extra(),
      SuggestedAction::SetPassword(t) => t.extra(),
      SuggestedAction::SetProfilePhoto(t) => t.extra(),
      SuggestedAction::SubscribeToAnnualPremium(t) => t.extra(),
      SuggestedAction::UpgradePremium(t) => t.extra(),
      SuggestedAction::ViewChecksHint(t) => t.extra(),
      SuggestedAction::ViewFirstTonWalletTransferHint(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SuggestedAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SuggestedAction::_Default(_))
  }

  pub fn is_add_login_passkey(&self) -> bool {
    matches!(self, SuggestedAction::AddLoginPasskey(_))
  }
  pub fn is_check_password(&self) -> bool {
    matches!(self, SuggestedAction::CheckPassword(_))
  }
  pub fn is_check_phone_number(&self) -> bool {
    matches!(self, SuggestedAction::CheckPhoneNumber(_))
  }
  pub fn is_convert_to_broadcast_group(&self) -> bool {
    matches!(self, SuggestedAction::ConvertToBroadcastGroup(_))
  }
  pub fn is_custom(&self) -> bool {
    matches!(self, SuggestedAction::Custom(_))
  }
  pub fn is_enable_archive_and_mute_new_chats(&self) -> bool {
    matches!(self, SuggestedAction::EnableArchiveAndMuteNewChats(_))
  }
  pub fn is_extend_premium(&self) -> bool {
    matches!(self, SuggestedAction::ExtendPremium(_))
  }
  pub fn is_extend_star_subscriptions(&self) -> bool {
    matches!(self, SuggestedAction::ExtendStarSubscriptions(_))
  }
  pub fn is_gift_premium_for_christmas(&self) -> bool {
    matches!(self, SuggestedAction::GiftPremiumForChristmas(_))
  }
  pub fn is_restore_premium(&self) -> bool {
    matches!(self, SuggestedAction::RestorePremium(_))
  }
  pub fn is_set_birthdate(&self) -> bool {
    matches!(self, SuggestedAction::SetBirthdate(_))
  }
  pub fn is_set_login_email_address(&self) -> bool {
    matches!(self, SuggestedAction::SetLoginEmailAddress(_))
  }
  pub fn is_set_password(&self) -> bool {
    matches!(self, SuggestedAction::SetPassword(_))
  }
  pub fn is_set_profile_photo(&self) -> bool {
    matches!(self, SuggestedAction::SetProfilePhoto(_))
  }
  pub fn is_subscribe_to_annual_premium(&self) -> bool {
    matches!(self, SuggestedAction::SubscribeToAnnualPremium(_))
  }
  pub fn is_upgrade_premium(&self) -> bool {
    matches!(self, SuggestedAction::UpgradePremium(_))
  }
  pub fn is_view_checks_hint(&self) -> bool {
    matches!(self, SuggestedAction::ViewChecksHint(_))
  }
  pub fn is_view_first_ton_wallet_transfer_hint(&self) -> bool {
    matches!(self, SuggestedAction::ViewFirstTonWalletTransferHint(_))
  }

  pub fn on_add_login_passkey<F: FnOnce(&SuggestedActionAddLoginPasskey)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::AddLoginPasskey(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_check_password<F: FnOnce(&SuggestedActionCheckPassword)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::CheckPassword(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_check_phone_number<F: FnOnce(&SuggestedActionCheckPhoneNumber)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let SuggestedAction::CheckPhoneNumber(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_convert_to_broadcast_group<F: FnOnce(&SuggestedActionConvertToBroadcastGroup)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let SuggestedAction::ConvertToBroadcastGroup(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_custom<F: FnOnce(&SuggestedActionCustom)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::Custom(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_enable_archive_and_mute_new_chats<
    F: FnOnce(&SuggestedActionEnableArchiveAndMuteNewChats),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let SuggestedAction::EnableArchiveAndMuteNewChats(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_extend_premium<F: FnOnce(&SuggestedActionExtendPremium)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::ExtendPremium(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_extend_star_subscriptions<F: FnOnce(&SuggestedActionExtendStarSubscriptions)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let SuggestedAction::ExtendStarSubscriptions(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_premium_for_christmas<F: FnOnce(&SuggestedActionGiftPremiumForChristmas)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let SuggestedAction::GiftPremiumForChristmas(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_restore_premium<F: FnOnce(&SuggestedActionRestorePremium)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::RestorePremium(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_set_birthdate<F: FnOnce(&SuggestedActionSetBirthdate)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::SetBirthdate(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_set_login_email_address<F: FnOnce(&SuggestedActionSetLoginEmailAddress)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let SuggestedAction::SetLoginEmailAddress(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_set_password<F: FnOnce(&SuggestedActionSetPassword)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::SetPassword(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_set_profile_photo<F: FnOnce(&SuggestedActionSetProfilePhoto)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::SetProfilePhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_subscribe_to_annual_premium<F: FnOnce(&SuggestedActionSubscribeToAnnualPremium)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let SuggestedAction::SubscribeToAnnualPremium(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgrade_premium<F: FnOnce(&SuggestedActionUpgradePremium)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::UpgradePremium(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_view_checks_hint<F: FnOnce(&SuggestedActionViewChecksHint)>(&self, fnc: F) -> &Self {
    if let SuggestedAction::ViewChecksHint(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_view_first_ton_wallet_transfer_hint<
    F: FnOnce(&SuggestedActionViewFirstTonWalletTransferHint),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let SuggestedAction::ViewFirstTonWalletTransferHint(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_add_login_passkey(&self) -> Option<&SuggestedActionAddLoginPasskey> {
    if let SuggestedAction::AddLoginPasskey(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_check_password(&self) -> Option<&SuggestedActionCheckPassword> {
    if let SuggestedAction::CheckPassword(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_check_phone_number(&self) -> Option<&SuggestedActionCheckPhoneNumber> {
    if let SuggestedAction::CheckPhoneNumber(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_convert_to_broadcast_group(&self) -> Option<&SuggestedActionConvertToBroadcastGroup> {
    if let SuggestedAction::ConvertToBroadcastGroup(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_custom(&self) -> Option<&SuggestedActionCustom> {
    if let SuggestedAction::Custom(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_enable_archive_and_mute_new_chats(
    &self,
  ) -> Option<&SuggestedActionEnableArchiveAndMuteNewChats> {
    if let SuggestedAction::EnableArchiveAndMuteNewChats(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_extend_premium(&self) -> Option<&SuggestedActionExtendPremium> {
    if let SuggestedAction::ExtendPremium(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_extend_star_subscriptions(&self) -> Option<&SuggestedActionExtendStarSubscriptions> {
    if let SuggestedAction::ExtendStarSubscriptions(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_premium_for_christmas(&self) -> Option<&SuggestedActionGiftPremiumForChristmas> {
    if let SuggestedAction::GiftPremiumForChristmas(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_restore_premium(&self) -> Option<&SuggestedActionRestorePremium> {
    if let SuggestedAction::RestorePremium(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_set_birthdate(&self) -> Option<&SuggestedActionSetBirthdate> {
    if let SuggestedAction::SetBirthdate(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_set_login_email_address(&self) -> Option<&SuggestedActionSetLoginEmailAddress> {
    if let SuggestedAction::SetLoginEmailAddress(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_set_password(&self) -> Option<&SuggestedActionSetPassword> {
    if let SuggestedAction::SetPassword(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_set_profile_photo(&self) -> Option<&SuggestedActionSetProfilePhoto> {
    if let SuggestedAction::SetProfilePhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_subscribe_to_annual_premium(&self) -> Option<&SuggestedActionSubscribeToAnnualPremium> {
    if let SuggestedAction::SubscribeToAnnualPremium(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgrade_premium(&self) -> Option<&SuggestedActionUpgradePremium> {
    if let SuggestedAction::UpgradePremium(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_view_checks_hint(&self) -> Option<&SuggestedActionViewChecksHint> {
    if let SuggestedAction::ViewChecksHint(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_view_first_ton_wallet_transfer_hint(
    &self,
  ) -> Option<&SuggestedActionViewFirstTonWalletTransferHint> {
    if let SuggestedAction::ViewFirstTonWalletTransferHint(t) = self {
      return Some(t);
    }
    None
  }

  pub fn add_login_passkey<T: AsRef<SuggestedActionAddLoginPasskey>>(t: T) -> Self {
    SuggestedAction::AddLoginPasskey(t.as_ref().clone())
  }

  pub fn check_password<T: AsRef<SuggestedActionCheckPassword>>(t: T) -> Self {
    SuggestedAction::CheckPassword(t.as_ref().clone())
  }

  pub fn check_phone_number<T: AsRef<SuggestedActionCheckPhoneNumber>>(t: T) -> Self {
    SuggestedAction::CheckPhoneNumber(t.as_ref().clone())
  }

  pub fn convert_to_broadcast_group<T: AsRef<SuggestedActionConvertToBroadcastGroup>>(
    t: T,
  ) -> Self {
    SuggestedAction::ConvertToBroadcastGroup(t.as_ref().clone())
  }

  pub fn custom<T: AsRef<SuggestedActionCustom>>(t: T) -> Self {
    SuggestedAction::Custom(t.as_ref().clone())
  }

  pub fn enable_archive_and_mute_new_chats<
    T: AsRef<SuggestedActionEnableArchiveAndMuteNewChats>,
  >(
    t: T,
  ) -> Self {
    SuggestedAction::EnableArchiveAndMuteNewChats(t.as_ref().clone())
  }

  pub fn extend_premium<T: AsRef<SuggestedActionExtendPremium>>(t: T) -> Self {
    SuggestedAction::ExtendPremium(t.as_ref().clone())
  }

  pub fn extend_star_subscriptions<T: AsRef<SuggestedActionExtendStarSubscriptions>>(t: T) -> Self {
    SuggestedAction::ExtendStarSubscriptions(t.as_ref().clone())
  }

  pub fn gift_premium_for_christmas<T: AsRef<SuggestedActionGiftPremiumForChristmas>>(
    t: T,
  ) -> Self {
    SuggestedAction::GiftPremiumForChristmas(t.as_ref().clone())
  }

  pub fn restore_premium<T: AsRef<SuggestedActionRestorePremium>>(t: T) -> Self {
    SuggestedAction::RestorePremium(t.as_ref().clone())
  }

  pub fn set_birthdate<T: AsRef<SuggestedActionSetBirthdate>>(t: T) -> Self {
    SuggestedAction::SetBirthdate(t.as_ref().clone())
  }

  pub fn set_login_email_address<T: AsRef<SuggestedActionSetLoginEmailAddress>>(t: T) -> Self {
    SuggestedAction::SetLoginEmailAddress(t.as_ref().clone())
  }

  pub fn set_password<T: AsRef<SuggestedActionSetPassword>>(t: T) -> Self {
    SuggestedAction::SetPassword(t.as_ref().clone())
  }

  pub fn set_profile_photo<T: AsRef<SuggestedActionSetProfilePhoto>>(t: T) -> Self {
    SuggestedAction::SetProfilePhoto(t.as_ref().clone())
  }

  pub fn subscribe_to_annual_premium<T: AsRef<SuggestedActionSubscribeToAnnualPremium>>(
    t: T,
  ) -> Self {
    SuggestedAction::SubscribeToAnnualPremium(t.as_ref().clone())
  }

  pub fn upgrade_premium<T: AsRef<SuggestedActionUpgradePremium>>(t: T) -> Self {
    SuggestedAction::UpgradePremium(t.as_ref().clone())
  }

  pub fn view_checks_hint<T: AsRef<SuggestedActionViewChecksHint>>(t: T) -> Self {
    SuggestedAction::ViewChecksHint(t.as_ref().clone())
  }

  pub fn view_first_ton_wallet_transfer_hint<
    T: AsRef<SuggestedActionViewFirstTonWalletTransferHint>,
  >(
    t: T,
  ) -> Self {
    SuggestedAction::ViewFirstTonWalletTransferHint(t.as_ref().clone())
  }
}

impl AsRef<SuggestedAction> for SuggestedAction {
  fn as_ref(&self) -> &SuggestedAction {
    self
  }
}

/// Suggests the user to add a passkey for login using addLoginPasskey
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionAddLoginPasskey {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionAddLoginPasskey {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionAddLoginPasskey"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionAddLoginPasskey {}

impl SuggestedActionAddLoginPasskey {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionAddLoginPasskeyBuilder {
    let inner = SuggestedActionAddLoginPasskey {
      td_name: "suggestedActionAddLoginPasskey".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionAddLoginPasskeyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionAddLoginPasskeyBuilder {
  inner: SuggestedActionAddLoginPasskey,
}

impl RTDSuggestedActionAddLoginPasskeyBuilder {
  pub fn build(&self) -> SuggestedActionAddLoginPasskey {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionAddLoginPasskey> for SuggestedActionAddLoginPasskey {
  fn as_ref(&self) -> &SuggestedActionAddLoginPasskey {
    self
  }
}

impl AsRef<SuggestedActionAddLoginPasskey> for RTDSuggestedActionAddLoginPasskeyBuilder {
  fn as_ref(&self) -> &SuggestedActionAddLoginPasskey {
    &self.inner
  }
}

/// Suggests the user to check whether they still remember their 2-step verification password
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionCheckPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionCheckPassword {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionCheckPassword"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionCheckPassword {}

impl SuggestedActionCheckPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionCheckPasswordBuilder {
    let inner = SuggestedActionCheckPassword {
      td_name: "suggestedActionCheckPassword".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionCheckPasswordBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionCheckPasswordBuilder {
  inner: SuggestedActionCheckPassword,
}

impl RTDSuggestedActionCheckPasswordBuilder {
  pub fn build(&self) -> SuggestedActionCheckPassword {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionCheckPassword> for SuggestedActionCheckPassword {
  fn as_ref(&self) -> &SuggestedActionCheckPassword {
    self
  }
}

impl AsRef<SuggestedActionCheckPassword> for RTDSuggestedActionCheckPasswordBuilder {
  fn as_ref(&self) -> &SuggestedActionCheckPassword {
    &self.inner
  }
}

/// Suggests the user to check whether authorization phone number is correct and change the phone number if it is inaccessible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionCheckPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionCheckPhoneNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionCheckPhoneNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionCheckPhoneNumber {}

impl SuggestedActionCheckPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionCheckPhoneNumberBuilder {
    let inner = SuggestedActionCheckPhoneNumber {
      td_name: "suggestedActionCheckPhoneNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionCheckPhoneNumberBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionCheckPhoneNumberBuilder {
  inner: SuggestedActionCheckPhoneNumber,
}

impl RTDSuggestedActionCheckPhoneNumberBuilder {
  pub fn build(&self) -> SuggestedActionCheckPhoneNumber {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionCheckPhoneNumber> for SuggestedActionCheckPhoneNumber {
  fn as_ref(&self) -> &SuggestedActionCheckPhoneNumber {
    self
  }
}

impl AsRef<SuggestedActionCheckPhoneNumber> for RTDSuggestedActionCheckPhoneNumberBuilder {
  fn as_ref(&self) -> &SuggestedActionCheckPhoneNumber {
    &self.inner
  }
}

/// Suggests the user to convert specified supergroup to a broadcast group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionConvertToBroadcastGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Supergroup identifier
  supergroup_id: i64,
}

impl RObject for SuggestedActionConvertToBroadcastGroup {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionConvertToBroadcastGroup"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionConvertToBroadcastGroup {}

impl SuggestedActionConvertToBroadcastGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionConvertToBroadcastGroupBuilder {
    let inner = SuggestedActionConvertToBroadcastGroup {
      td_name: "suggestedActionConvertToBroadcastGroup".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSuggestedActionConvertToBroadcastGroupBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 {
    self.supergroup_id
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionConvertToBroadcastGroupBuilder {
  inner: SuggestedActionConvertToBroadcastGroup,
}

impl RTDSuggestedActionConvertToBroadcastGroupBuilder {
  pub fn build(&self) -> SuggestedActionConvertToBroadcastGroup {
    self.inner.clone()
  }

  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }
}

impl AsRef<SuggestedActionConvertToBroadcastGroup> for SuggestedActionConvertToBroadcastGroup {
  fn as_ref(&self) -> &SuggestedActionConvertToBroadcastGroup {
    self
  }
}

impl AsRef<SuggestedActionConvertToBroadcastGroup>
  for RTDSuggestedActionConvertToBroadcastGroupBuilder
{
  fn as_ref(&self) -> &SuggestedActionConvertToBroadcastGroup {
    &self.inner
  }
}

/// A custom suggestion to be shown at the top of the chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionCustom {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique name of the suggestion
  name: String,
  /// Title of the suggestion
  title: FormattedText,
  /// A custom suggestion to be shown at the top of the chat list
  description: FormattedText,
  /// The link to open when the suggestion is clicked
  url: String,
}

impl RObject for SuggestedActionCustom {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionCustom"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionCustom {}

impl SuggestedActionCustom {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionCustomBuilder {
    let inner = SuggestedActionCustom {
      td_name: "suggestedActionCustom".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSuggestedActionCustomBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn title(&self) -> &FormattedText {
    &self.title
  }

  pub fn description(&self) -> &FormattedText {
    &self.description
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionCustomBuilder {
  inner: SuggestedActionCustom,
}

impl RTDSuggestedActionCustomBuilder {
  pub fn build(&self) -> SuggestedActionCustom {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn title<T: AsRef<FormattedText>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().clone();
    self
  }

  pub fn description<T: AsRef<FormattedText>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().clone();
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<SuggestedActionCustom> for SuggestedActionCustom {
  fn as_ref(&self) -> &SuggestedActionCustom {
    self
  }
}

impl AsRef<SuggestedActionCustom> for RTDSuggestedActionCustomBuilder {
  fn as_ref(&self) -> &SuggestedActionCustom {
    &self.inner
  }
}

/// Suggests the user to enable archive_and_mute_new_chats_from_unknown_users setting in archiveChatListSettings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionEnableArchiveAndMuteNewChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionEnableArchiveAndMuteNewChats {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionEnableArchiveAndMuteNewChats"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionEnableArchiveAndMuteNewChats {}

impl SuggestedActionEnableArchiveAndMuteNewChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
    let inner = SuggestedActionEnableArchiveAndMuteNewChats {
      td_name: "suggestedActionEnableArchiveAndMuteNewChats".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
  inner: SuggestedActionEnableArchiveAndMuteNewChats,
}

impl RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
  pub fn build(&self) -> SuggestedActionEnableArchiveAndMuteNewChats {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionEnableArchiveAndMuteNewChats>
  for SuggestedActionEnableArchiveAndMuteNewChats
{
  fn as_ref(&self) -> &SuggestedActionEnableArchiveAndMuteNewChats {
    self
  }
}

impl AsRef<SuggestedActionEnableArchiveAndMuteNewChats>
  for RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder
{
  fn as_ref(&self) -> &SuggestedActionEnableArchiveAndMuteNewChats {
    &self.inner
  }
}

/// Suggests the user to extend their expiring Telegram Premium subscription
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionExtendPremium {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A URL for managing Telegram Premium subscription
  manage_premium_subscription_url: String,
}

impl RObject for SuggestedActionExtendPremium {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionExtendPremium"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionExtendPremium {}

impl SuggestedActionExtendPremium {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionExtendPremiumBuilder {
    let inner = SuggestedActionExtendPremium {
      td_name: "suggestedActionExtendPremium".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSuggestedActionExtendPremiumBuilder { inner }
  }

  pub fn manage_premium_subscription_url(&self) -> &String {
    &self.manage_premium_subscription_url
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionExtendPremiumBuilder {
  inner: SuggestedActionExtendPremium,
}

impl RTDSuggestedActionExtendPremiumBuilder {
  pub fn build(&self) -> SuggestedActionExtendPremium {
    self.inner.clone()
  }

  pub fn manage_premium_subscription_url<T: AsRef<str>>(
    &mut self,
    manage_premium_subscription_url: T,
  ) -> &mut Self {
    self.inner.manage_premium_subscription_url =
      manage_premium_subscription_url.as_ref().to_string();
    self
  }
}

impl AsRef<SuggestedActionExtendPremium> for SuggestedActionExtendPremium {
  fn as_ref(&self) -> &SuggestedActionExtendPremium {
    self
  }
}

impl AsRef<SuggestedActionExtendPremium> for RTDSuggestedActionExtendPremiumBuilder {
  fn as_ref(&self) -> &SuggestedActionExtendPremium {
    &self.inner
  }
}

/// Suggests the user to extend their expiring Telegram Star subscriptions. Call getStarSubscriptions with only_expiring == true to get the number of expiring subscriptions and the number of required to buy Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionExtendStarSubscriptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionExtendStarSubscriptions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionExtendStarSubscriptions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionExtendStarSubscriptions {}

impl SuggestedActionExtendStarSubscriptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionExtendStarSubscriptionsBuilder {
    let inner = SuggestedActionExtendStarSubscriptions {
      td_name: "suggestedActionExtendStarSubscriptions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionExtendStarSubscriptionsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionExtendStarSubscriptionsBuilder {
  inner: SuggestedActionExtendStarSubscriptions,
}

impl RTDSuggestedActionExtendStarSubscriptionsBuilder {
  pub fn build(&self) -> SuggestedActionExtendStarSubscriptions {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionExtendStarSubscriptions> for SuggestedActionExtendStarSubscriptions {
  fn as_ref(&self) -> &SuggestedActionExtendStarSubscriptions {
    self
  }
}

impl AsRef<SuggestedActionExtendStarSubscriptions>
  for RTDSuggestedActionExtendStarSubscriptionsBuilder
{
  fn as_ref(&self) -> &SuggestedActionExtendStarSubscriptions {
    &self.inner
  }
}

/// Suggests the user to gift Telegram Premium to friends for Christmas
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionGiftPremiumForChristmas {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionGiftPremiumForChristmas {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionGiftPremiumForChristmas"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionGiftPremiumForChristmas {}

impl SuggestedActionGiftPremiumForChristmas {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionGiftPremiumForChristmasBuilder {
    let inner = SuggestedActionGiftPremiumForChristmas {
      td_name: "suggestedActionGiftPremiumForChristmas".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionGiftPremiumForChristmasBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionGiftPremiumForChristmasBuilder {
  inner: SuggestedActionGiftPremiumForChristmas,
}

impl RTDSuggestedActionGiftPremiumForChristmasBuilder {
  pub fn build(&self) -> SuggestedActionGiftPremiumForChristmas {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionGiftPremiumForChristmas> for SuggestedActionGiftPremiumForChristmas {
  fn as_ref(&self) -> &SuggestedActionGiftPremiumForChristmas {
    self
  }
}

impl AsRef<SuggestedActionGiftPremiumForChristmas>
  for RTDSuggestedActionGiftPremiumForChristmasBuilder
{
  fn as_ref(&self) -> &SuggestedActionGiftPremiumForChristmas {
    &self.inner
  }
}

/// Suggests the user to restore a recently expired Premium subscription
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionRestorePremium {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionRestorePremium {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionRestorePremium"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionRestorePremium {}

impl SuggestedActionRestorePremium {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionRestorePremiumBuilder {
    let inner = SuggestedActionRestorePremium {
      td_name: "suggestedActionRestorePremium".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionRestorePremiumBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionRestorePremiumBuilder {
  inner: SuggestedActionRestorePremium,
}

impl RTDSuggestedActionRestorePremiumBuilder {
  pub fn build(&self) -> SuggestedActionRestorePremium {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionRestorePremium> for SuggestedActionRestorePremium {
  fn as_ref(&self) -> &SuggestedActionRestorePremium {
    self
  }
}

impl AsRef<SuggestedActionRestorePremium> for RTDSuggestedActionRestorePremiumBuilder {
  fn as_ref(&self) -> &SuggestedActionRestorePremium {
    &self.inner
  }
}

/// Suggests the user to set birthdate
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionSetBirthdate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionSetBirthdate {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionSetBirthdate"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionSetBirthdate {}

impl SuggestedActionSetBirthdate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionSetBirthdateBuilder {
    let inner = SuggestedActionSetBirthdate {
      td_name: "suggestedActionSetBirthdate".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionSetBirthdateBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionSetBirthdateBuilder {
  inner: SuggestedActionSetBirthdate,
}

impl RTDSuggestedActionSetBirthdateBuilder {
  pub fn build(&self) -> SuggestedActionSetBirthdate {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionSetBirthdate> for SuggestedActionSetBirthdate {
  fn as_ref(&self) -> &SuggestedActionSetBirthdate {
    self
  }
}

impl AsRef<SuggestedActionSetBirthdate> for RTDSuggestedActionSetBirthdateBuilder {
  fn as_ref(&self) -> &SuggestedActionSetBirthdate {
    &self.inner
  }
}

/// Suggests the user to add login email address. Call isLoginEmailAddressRequired, and then setLoginEmailAddress or checkLoginEmailAddressCode to change the login email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionSetLoginEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the suggested action can be hidden using hideSuggestedAction. Otherwise, the user must not be able to use the application without setting up the email address
  can_be_hidden: bool,
}

impl RObject for SuggestedActionSetLoginEmailAddress {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionSetLoginEmailAddress"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionSetLoginEmailAddress {}

impl SuggestedActionSetLoginEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionSetLoginEmailAddressBuilder {
    let inner = SuggestedActionSetLoginEmailAddress {
      td_name: "suggestedActionSetLoginEmailAddress".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSuggestedActionSetLoginEmailAddressBuilder { inner }
  }

  pub fn can_be_hidden(&self) -> bool {
    self.can_be_hidden
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionSetLoginEmailAddressBuilder {
  inner: SuggestedActionSetLoginEmailAddress,
}

impl RTDSuggestedActionSetLoginEmailAddressBuilder {
  pub fn build(&self) -> SuggestedActionSetLoginEmailAddress {
    self.inner.clone()
  }

  pub fn can_be_hidden(&mut self, can_be_hidden: bool) -> &mut Self {
    self.inner.can_be_hidden = can_be_hidden;
    self
  }
}

impl AsRef<SuggestedActionSetLoginEmailAddress> for SuggestedActionSetLoginEmailAddress {
  fn as_ref(&self) -> &SuggestedActionSetLoginEmailAddress {
    self
  }
}

impl AsRef<SuggestedActionSetLoginEmailAddress> for RTDSuggestedActionSetLoginEmailAddressBuilder {
  fn as_ref(&self) -> &SuggestedActionSetLoginEmailAddress {
    &self.inner
  }
}

/// Suggests the user to set a 2-step verification password to be able to log in again
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionSetPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of days to pass between consecutive authorizations if the user declines to set password; if 0, then the user is advised to set the password for security reasons
  authorization_delay: i64,
}

impl RObject for SuggestedActionSetPassword {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionSetPassword"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionSetPassword {}

impl SuggestedActionSetPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionSetPasswordBuilder {
    let inner = SuggestedActionSetPassword {
      td_name: "suggestedActionSetPassword".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSuggestedActionSetPasswordBuilder { inner }
  }

  pub fn authorization_delay(&self) -> i64 {
    self.authorization_delay
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionSetPasswordBuilder {
  inner: SuggestedActionSetPassword,
}

impl RTDSuggestedActionSetPasswordBuilder {
  pub fn build(&self) -> SuggestedActionSetPassword {
    self.inner.clone()
  }

  pub fn authorization_delay(&mut self, authorization_delay: i64) -> &mut Self {
    self.inner.authorization_delay = authorization_delay;
    self
  }
}

impl AsRef<SuggestedActionSetPassword> for SuggestedActionSetPassword {
  fn as_ref(&self) -> &SuggestedActionSetPassword {
    self
  }
}

impl AsRef<SuggestedActionSetPassword> for RTDSuggestedActionSetPasswordBuilder {
  fn as_ref(&self) -> &SuggestedActionSetPassword {
    &self.inner
  }
}

/// Suggests the user to set profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionSetProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionSetProfilePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionSetProfilePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionSetProfilePhoto {}

impl SuggestedActionSetProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionSetProfilePhotoBuilder {
    let inner = SuggestedActionSetProfilePhoto {
      td_name: "suggestedActionSetProfilePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionSetProfilePhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionSetProfilePhotoBuilder {
  inner: SuggestedActionSetProfilePhoto,
}

impl RTDSuggestedActionSetProfilePhotoBuilder {
  pub fn build(&self) -> SuggestedActionSetProfilePhoto {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionSetProfilePhoto> for SuggestedActionSetProfilePhoto {
  fn as_ref(&self) -> &SuggestedActionSetProfilePhoto {
    self
  }
}

impl AsRef<SuggestedActionSetProfilePhoto> for RTDSuggestedActionSetProfilePhotoBuilder {
  fn as_ref(&self) -> &SuggestedActionSetProfilePhoto {
    &self.inner
  }
}

/// Suggests the user to subscribe to the Premium subscription with annual payments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionSubscribeToAnnualPremium {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionSubscribeToAnnualPremium {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionSubscribeToAnnualPremium"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionSubscribeToAnnualPremium {}

impl SuggestedActionSubscribeToAnnualPremium {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionSubscribeToAnnualPremiumBuilder {
    let inner = SuggestedActionSubscribeToAnnualPremium {
      td_name: "suggestedActionSubscribeToAnnualPremium".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionSubscribeToAnnualPremiumBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionSubscribeToAnnualPremiumBuilder {
  inner: SuggestedActionSubscribeToAnnualPremium,
}

impl RTDSuggestedActionSubscribeToAnnualPremiumBuilder {
  pub fn build(&self) -> SuggestedActionSubscribeToAnnualPremium {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionSubscribeToAnnualPremium> for SuggestedActionSubscribeToAnnualPremium {
  fn as_ref(&self) -> &SuggestedActionSubscribeToAnnualPremium {
    self
  }
}

impl AsRef<SuggestedActionSubscribeToAnnualPremium>
  for RTDSuggestedActionSubscribeToAnnualPremiumBuilder
{
  fn as_ref(&self) -> &SuggestedActionSubscribeToAnnualPremium {
    &self.inner
  }
}

/// Suggests the user to upgrade the Premium subscription from monthly payments to annual payments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionUpgradePremium {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionUpgradePremium {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionUpgradePremium"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionUpgradePremium {}

impl SuggestedActionUpgradePremium {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionUpgradePremiumBuilder {
    let inner = SuggestedActionUpgradePremium {
      td_name: "suggestedActionUpgradePremium".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionUpgradePremiumBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionUpgradePremiumBuilder {
  inner: SuggestedActionUpgradePremium,
}

impl RTDSuggestedActionUpgradePremiumBuilder {
  pub fn build(&self) -> SuggestedActionUpgradePremium {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionUpgradePremium> for SuggestedActionUpgradePremium {
  fn as_ref(&self) -> &SuggestedActionUpgradePremium {
    self
  }
}

impl AsRef<SuggestedActionUpgradePremium> for RTDSuggestedActionUpgradePremiumBuilder {
  fn as_ref(&self) -> &SuggestedActionUpgradePremium {
    &self.inner
  }
}

/// Suggests the user to view a hint about the meaning of one and two check marks on sent messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionViewChecksHint {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionViewChecksHint {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionViewChecksHint"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionViewChecksHint {}

impl SuggestedActionViewChecksHint {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionViewChecksHintBuilder {
    let inner = SuggestedActionViewChecksHint {
      td_name: "suggestedActionViewChecksHint".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionViewChecksHintBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionViewChecksHintBuilder {
  inner: SuggestedActionViewChecksHint,
}

impl RTDSuggestedActionViewChecksHintBuilder {
  pub fn build(&self) -> SuggestedActionViewChecksHint {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionViewChecksHint> for SuggestedActionViewChecksHint {
  fn as_ref(&self) -> &SuggestedActionViewChecksHint {
    self
  }
}

impl AsRef<SuggestedActionViewChecksHint> for RTDSuggestedActionViewChecksHintBuilder {
  fn as_ref(&self) -> &SuggestedActionViewChecksHint {
    &self.inner
  }
}

/// Suggests the user to view a hint about receiving of the first TON wallet transfer
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionViewFirstTonWalletTransferHint {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SuggestedActionViewFirstTonWalletTransferHint {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedActionViewFirstTonWalletTransferHint"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedAction for SuggestedActionViewFirstTonWalletTransferHint {}

impl SuggestedActionViewFirstTonWalletTransferHint {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedActionViewFirstTonWalletTransferHintBuilder {
    let inner = SuggestedActionViewFirstTonWalletTransferHint {
      td_name: "suggestedActionViewFirstTonWalletTransferHint".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSuggestedActionViewFirstTonWalletTransferHintBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSuggestedActionViewFirstTonWalletTransferHintBuilder {
  inner: SuggestedActionViewFirstTonWalletTransferHint,
}

impl RTDSuggestedActionViewFirstTonWalletTransferHintBuilder {
  pub fn build(&self) -> SuggestedActionViewFirstTonWalletTransferHint {
    self.inner.clone()
  }
}

impl AsRef<SuggestedActionViewFirstTonWalletTransferHint>
  for SuggestedActionViewFirstTonWalletTransferHint
{
  fn as_ref(&self) -> &SuggestedActionViewFirstTonWalletTransferHint {
    self
  }
}

impl AsRef<SuggestedActionViewFirstTonWalletTransferHint>
  for RTDSuggestedActionViewFirstTonWalletTransferHintBuilder
{
  fn as_ref(&self) -> &SuggestedActionViewFirstTonWalletTransferHint {
    &self.inner
  }
}
