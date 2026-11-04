use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a section of the application settings
pub trait TDSettingsSection: Debug + RObject {}

/// Describes a section of the application settings
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SettingsSection {
  #[doc(hidden)]
  _Default(()),
  /// The appearance section
  Appearance(SettingsSectionAppearance),
  /// The "Ask a question" section
  AskQuestion(SettingsSectionAskQuestion),
  /// The "Telegram Business" section
  Business(SettingsSectionBusiness),
  /// The chat folder settings section
  ChatFolders(SettingsSectionChatFolders),
  /// The data and storage settings section
  DataAndStorage(SettingsSectionDataAndStorage),
  /// The Devices section
  Devices(SettingsSectionDevices),
  /// The profile edit section
  EditProfile(SettingsSectionEditProfile),
  /// The FAQ section
  Faq(SettingsSectionFaq),
  /// The "Telegram Features" section
  Features(SettingsSectionFeatures),
  /// The in-app browser settings section
  InAppBrowser(SettingsSectionInAppBrowser),
  /// The application language section
  Language(SettingsSectionLanguage),
  /// The TON Gram balance and transaction section
  MyGrams(SettingsSectionMyGrams),
  /// The Telegram Star balance and transaction section
  MyStars(SettingsSectionMyStars),
  /// The notification settings section
  Notifications(SettingsSectionNotifications),
  /// The power saving settings section
  PowerSaving(SettingsSectionPowerSaving),
  /// The "Telegram Premium" section
  Premium(SettingsSectionPremium),
  /// The privacy and security section
  PrivacyAndSecurity(SettingsSectionPrivacyAndSecurity),
  /// The "Privacy Policy" section
  PrivacyPolicy(SettingsSectionPrivacyPolicy),
  /// The current user's QR code section
  QrCode(SettingsSectionQrCode),
  /// Search in Settings
  Search(SettingsSectionSearch),
  /// The "Send a gift" section
  SendGift(SettingsSectionSendGift),
}

impl Default for SettingsSection {
  fn default() -> Self {
    SettingsSection::_Default(())
  }
}

impl<'de> Deserialize<'de> for SettingsSection {
  fn deserialize<D>(deserializer: D) -> Result<SettingsSection, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SettingsSection,
      (settingsSectionAppearance, Appearance);
      (settingsSectionAskQuestion, AskQuestion);
      (settingsSectionBusiness, Business);
      (settingsSectionChatFolders, ChatFolders);
      (settingsSectionDataAndStorage, DataAndStorage);
      (settingsSectionDevices, Devices);
      (settingsSectionEditProfile, EditProfile);
      (settingsSectionFaq, Faq);
      (settingsSectionFeatures, Features);
      (settingsSectionInAppBrowser, InAppBrowser);
      (settingsSectionLanguage, Language);
      (settingsSectionMyGrams, MyGrams);
      (settingsSectionMyStars, MyStars);
      (settingsSectionNotifications, Notifications);
      (settingsSectionPowerSaving, PowerSaving);
      (settingsSectionPremium, Premium);
      (settingsSectionPrivacyAndSecurity, PrivacyAndSecurity);
      (settingsSectionPrivacyPolicy, PrivacyPolicy);
      (settingsSectionQrCode, QrCode);
      (settingsSectionSearch, Search);
      (settingsSectionSendGift, SendGift);

    )(deserializer)
  }
}

impl RObject for SettingsSection {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SettingsSection::Appearance(t) => t.td_name(),
      SettingsSection::AskQuestion(t) => t.td_name(),
      SettingsSection::Business(t) => t.td_name(),
      SettingsSection::ChatFolders(t) => t.td_name(),
      SettingsSection::DataAndStorage(t) => t.td_name(),
      SettingsSection::Devices(t) => t.td_name(),
      SettingsSection::EditProfile(t) => t.td_name(),
      SettingsSection::Faq(t) => t.td_name(),
      SettingsSection::Features(t) => t.td_name(),
      SettingsSection::InAppBrowser(t) => t.td_name(),
      SettingsSection::Language(t) => t.td_name(),
      SettingsSection::MyGrams(t) => t.td_name(),
      SettingsSection::MyStars(t) => t.td_name(),
      SettingsSection::Notifications(t) => t.td_name(),
      SettingsSection::PowerSaving(t) => t.td_name(),
      SettingsSection::Premium(t) => t.td_name(),
      SettingsSection::PrivacyAndSecurity(t) => t.td_name(),
      SettingsSection::PrivacyPolicy(t) => t.td_name(),
      SettingsSection::QrCode(t) => t.td_name(),
      SettingsSection::Search(t) => t.td_name(),
      SettingsSection::SendGift(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SettingsSection::Appearance(t) => t.extra(),
      SettingsSection::AskQuestion(t) => t.extra(),
      SettingsSection::Business(t) => t.extra(),
      SettingsSection::ChatFolders(t) => t.extra(),
      SettingsSection::DataAndStorage(t) => t.extra(),
      SettingsSection::Devices(t) => t.extra(),
      SettingsSection::EditProfile(t) => t.extra(),
      SettingsSection::Faq(t) => t.extra(),
      SettingsSection::Features(t) => t.extra(),
      SettingsSection::InAppBrowser(t) => t.extra(),
      SettingsSection::Language(t) => t.extra(),
      SettingsSection::MyGrams(t) => t.extra(),
      SettingsSection::MyStars(t) => t.extra(),
      SettingsSection::Notifications(t) => t.extra(),
      SettingsSection::PowerSaving(t) => t.extra(),
      SettingsSection::Premium(t) => t.extra(),
      SettingsSection::PrivacyAndSecurity(t) => t.extra(),
      SettingsSection::PrivacyPolicy(t) => t.extra(),
      SettingsSection::QrCode(t) => t.extra(),
      SettingsSection::Search(t) => t.extra(),
      SettingsSection::SendGift(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SettingsSection {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SettingsSection::_Default(_))
  }

  pub fn is_appearance(&self) -> bool {
    matches!(self, SettingsSection::Appearance(_))
  }
  pub fn is_ask_question(&self) -> bool {
    matches!(self, SettingsSection::AskQuestion(_))
  }
  pub fn is_business(&self) -> bool {
    matches!(self, SettingsSection::Business(_))
  }
  pub fn is_chat_folders(&self) -> bool {
    matches!(self, SettingsSection::ChatFolders(_))
  }
  pub fn is_data_and_storage(&self) -> bool {
    matches!(self, SettingsSection::DataAndStorage(_))
  }
  pub fn is_devices(&self) -> bool {
    matches!(self, SettingsSection::Devices(_))
  }
  pub fn is_edit_profile(&self) -> bool {
    matches!(self, SettingsSection::EditProfile(_))
  }
  pub fn is_faq(&self) -> bool {
    matches!(self, SettingsSection::Faq(_))
  }
  pub fn is_features(&self) -> bool {
    matches!(self, SettingsSection::Features(_))
  }
  pub fn is_in_app_browser(&self) -> bool {
    matches!(self, SettingsSection::InAppBrowser(_))
  }
  pub fn is_language(&self) -> bool {
    matches!(self, SettingsSection::Language(_))
  }
  pub fn is_my_grams(&self) -> bool {
    matches!(self, SettingsSection::MyGrams(_))
  }
  pub fn is_my_stars(&self) -> bool {
    matches!(self, SettingsSection::MyStars(_))
  }
  pub fn is_notifications(&self) -> bool {
    matches!(self, SettingsSection::Notifications(_))
  }
  pub fn is_power_saving(&self) -> bool {
    matches!(self, SettingsSection::PowerSaving(_))
  }
  pub fn is_premium(&self) -> bool {
    matches!(self, SettingsSection::Premium(_))
  }
  pub fn is_privacy_and_security(&self) -> bool {
    matches!(self, SettingsSection::PrivacyAndSecurity(_))
  }
  pub fn is_privacy_policy(&self) -> bool {
    matches!(self, SettingsSection::PrivacyPolicy(_))
  }
  pub fn is_qr_code(&self) -> bool {
    matches!(self, SettingsSection::QrCode(_))
  }
  pub fn is_search(&self) -> bool {
    matches!(self, SettingsSection::Search(_))
  }
  pub fn is_send_gift(&self) -> bool {
    matches!(self, SettingsSection::SendGift(_))
  }

  pub fn on_appearance<F: FnOnce(&SettingsSectionAppearance)>(&self, fnc: F) -> &Self {
    if let SettingsSection::Appearance(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ask_question<F: FnOnce(&SettingsSectionAskQuestion)>(&self, fnc: F) -> &Self {
    if let SettingsSection::AskQuestion(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_business<F: FnOnce(&SettingsSectionBusiness)>(&self, fnc: F) -> &Self {
    if let SettingsSection::Business(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_folders<F: FnOnce(&SettingsSectionChatFolders)>(&self, fnc: F) -> &Self {
    if let SettingsSection::ChatFolders(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_data_and_storage<F: FnOnce(&SettingsSectionDataAndStorage)>(&self, fnc: F) -> &Self {
    if let SettingsSection::DataAndStorage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_devices<F: FnOnce(&SettingsSectionDevices)>(&self, fnc: F) -> &Self {
    if let SettingsSection::Devices(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_edit_profile<F: FnOnce(&SettingsSectionEditProfile)>(&self, fnc: F) -> &Self {
    if let SettingsSection::EditProfile(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_faq<F: FnOnce(&SettingsSectionFaq)>(&self, fnc: F) -> &Self {
    if let SettingsSection::Faq(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_features<F: FnOnce(&SettingsSectionFeatures)>(&self, fnc: F) -> &Self {
    if let SettingsSection::Features(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_in_app_browser<F: FnOnce(&SettingsSectionInAppBrowser)>(&self, fnc: F) -> &Self {
    if let SettingsSection::InAppBrowser(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_language<F: FnOnce(&SettingsSectionLanguage)>(&self, fnc: F) -> &Self {
    if let SettingsSection::Language(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_my_grams<F: FnOnce(&SettingsSectionMyGrams)>(&self, fnc: F) -> &Self {
    if let SettingsSection::MyGrams(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_my_stars<F: FnOnce(&SettingsSectionMyStars)>(&self, fnc: F) -> &Self {
    if let SettingsSection::MyStars(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_notifications<F: FnOnce(&SettingsSectionNotifications)>(&self, fnc: F) -> &Self {
    if let SettingsSection::Notifications(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_power_saving<F: FnOnce(&SettingsSectionPowerSaving)>(&self, fnc: F) -> &Self {
    if let SettingsSection::PowerSaving(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium<F: FnOnce(&SettingsSectionPremium)>(&self, fnc: F) -> &Self {
    if let SettingsSection::Premium(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_privacy_and_security<F: FnOnce(&SettingsSectionPrivacyAndSecurity)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let SettingsSection::PrivacyAndSecurity(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_privacy_policy<F: FnOnce(&SettingsSectionPrivacyPolicy)>(&self, fnc: F) -> &Self {
    if let SettingsSection::PrivacyPolicy(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_qr_code<F: FnOnce(&SettingsSectionQrCode)>(&self, fnc: F) -> &Self {
    if let SettingsSection::QrCode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_search<F: FnOnce(&SettingsSectionSearch)>(&self, fnc: F) -> &Self {
    if let SettingsSection::Search(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_send_gift<F: FnOnce(&SettingsSectionSendGift)>(&self, fnc: F) -> &Self {
    if let SettingsSection::SendGift(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_appearance(&self) -> Option<&SettingsSectionAppearance> {
    if let SettingsSection::Appearance(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ask_question(&self) -> Option<&SettingsSectionAskQuestion> {
    if let SettingsSection::AskQuestion(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_business(&self) -> Option<&SettingsSectionBusiness> {
    if let SettingsSection::Business(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_folders(&self) -> Option<&SettingsSectionChatFolders> {
    if let SettingsSection::ChatFolders(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_data_and_storage(&self) -> Option<&SettingsSectionDataAndStorage> {
    if let SettingsSection::DataAndStorage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_devices(&self) -> Option<&SettingsSectionDevices> {
    if let SettingsSection::Devices(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_edit_profile(&self) -> Option<&SettingsSectionEditProfile> {
    if let SettingsSection::EditProfile(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_faq(&self) -> Option<&SettingsSectionFaq> {
    if let SettingsSection::Faq(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_features(&self) -> Option<&SettingsSectionFeatures> {
    if let SettingsSection::Features(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_in_app_browser(&self) -> Option<&SettingsSectionInAppBrowser> {
    if let SettingsSection::InAppBrowser(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_language(&self) -> Option<&SettingsSectionLanguage> {
    if let SettingsSection::Language(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_my_grams(&self) -> Option<&SettingsSectionMyGrams> {
    if let SettingsSection::MyGrams(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_my_stars(&self) -> Option<&SettingsSectionMyStars> {
    if let SettingsSection::MyStars(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_notifications(&self) -> Option<&SettingsSectionNotifications> {
    if let SettingsSection::Notifications(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_power_saving(&self) -> Option<&SettingsSectionPowerSaving> {
    if let SettingsSection::PowerSaving(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium(&self) -> Option<&SettingsSectionPremium> {
    if let SettingsSection::Premium(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_privacy_and_security(&self) -> Option<&SettingsSectionPrivacyAndSecurity> {
    if let SettingsSection::PrivacyAndSecurity(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_privacy_policy(&self) -> Option<&SettingsSectionPrivacyPolicy> {
    if let SettingsSection::PrivacyPolicy(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_qr_code(&self) -> Option<&SettingsSectionQrCode> {
    if let SettingsSection::QrCode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_search(&self) -> Option<&SettingsSectionSearch> {
    if let SettingsSection::Search(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_send_gift(&self) -> Option<&SettingsSectionSendGift> {
    if let SettingsSection::SendGift(t) = self {
      return Some(t);
    }
    None
  }

  pub fn appearance<T: AsRef<SettingsSectionAppearance>>(t: T) -> Self {
    SettingsSection::Appearance(t.as_ref().clone())
  }

  pub fn ask_question<T: AsRef<SettingsSectionAskQuestion>>(t: T) -> Self {
    SettingsSection::AskQuestion(t.as_ref().clone())
  }

  pub fn business<T: AsRef<SettingsSectionBusiness>>(t: T) -> Self {
    SettingsSection::Business(t.as_ref().clone())
  }

  pub fn chat_folders<T: AsRef<SettingsSectionChatFolders>>(t: T) -> Self {
    SettingsSection::ChatFolders(t.as_ref().clone())
  }

  pub fn data_and_storage<T: AsRef<SettingsSectionDataAndStorage>>(t: T) -> Self {
    SettingsSection::DataAndStorage(t.as_ref().clone())
  }

  pub fn devices<T: AsRef<SettingsSectionDevices>>(t: T) -> Self {
    SettingsSection::Devices(t.as_ref().clone())
  }

  pub fn edit_profile<T: AsRef<SettingsSectionEditProfile>>(t: T) -> Self {
    SettingsSection::EditProfile(t.as_ref().clone())
  }

  pub fn faq<T: AsRef<SettingsSectionFaq>>(t: T) -> Self {
    SettingsSection::Faq(t.as_ref().clone())
  }

  pub fn features<T: AsRef<SettingsSectionFeatures>>(t: T) -> Self {
    SettingsSection::Features(t.as_ref().clone())
  }

  pub fn in_app_browser<T: AsRef<SettingsSectionInAppBrowser>>(t: T) -> Self {
    SettingsSection::InAppBrowser(t.as_ref().clone())
  }

  pub fn language<T: AsRef<SettingsSectionLanguage>>(t: T) -> Self {
    SettingsSection::Language(t.as_ref().clone())
  }

  pub fn my_grams<T: AsRef<SettingsSectionMyGrams>>(t: T) -> Self {
    SettingsSection::MyGrams(t.as_ref().clone())
  }

  pub fn my_stars<T: AsRef<SettingsSectionMyStars>>(t: T) -> Self {
    SettingsSection::MyStars(t.as_ref().clone())
  }

  pub fn notifications<T: AsRef<SettingsSectionNotifications>>(t: T) -> Self {
    SettingsSection::Notifications(t.as_ref().clone())
  }

  pub fn power_saving<T: AsRef<SettingsSectionPowerSaving>>(t: T) -> Self {
    SettingsSection::PowerSaving(t.as_ref().clone())
  }

  pub fn premium<T: AsRef<SettingsSectionPremium>>(t: T) -> Self {
    SettingsSection::Premium(t.as_ref().clone())
  }

  pub fn privacy_and_security<T: AsRef<SettingsSectionPrivacyAndSecurity>>(t: T) -> Self {
    SettingsSection::PrivacyAndSecurity(t.as_ref().clone())
  }

  pub fn privacy_policy<T: AsRef<SettingsSectionPrivacyPolicy>>(t: T) -> Self {
    SettingsSection::PrivacyPolicy(t.as_ref().clone())
  }

  pub fn qr_code<T: AsRef<SettingsSectionQrCode>>(t: T) -> Self {
    SettingsSection::QrCode(t.as_ref().clone())
  }

  pub fn search<T: AsRef<SettingsSectionSearch>>(t: T) -> Self {
    SettingsSection::Search(t.as_ref().clone())
  }

  pub fn send_gift<T: AsRef<SettingsSectionSendGift>>(t: T) -> Self {
    SettingsSection::SendGift(t.as_ref().clone())
  }
}

impl AsRef<SettingsSection> for SettingsSection {
  fn as_ref(&self) -> &SettingsSection {
    self
  }
}

/// The appearance section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionAppearance {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "themes", "themes/edit", "themes/create", "wallpapers", "wallpapers/edit", "wallpapers/set", "wallpapers/choose-photo", "your-color/profile", "your-color/profile/add-icons", "your-color/profile/use-gift", "your-color/profile/reset", "your-color/name", "your-color/name/add-icons", "your-color/name/use-gift", "night-mode", "auto-night-mode", "text-size", "text-size/use-system", "message-corners", "animations", "stickers-and-emoji", "stickers-and-emoji/edit", "stickers-and-emoji/trending", "stickers-and-emoji/archived", "stickers-and-emoji/archived/edit", "stickers-and-emoji/emoji", "stickers-and-emoji/emoji/edit", "stickers-and-emoji/emoji/archived", "stickers-and-emoji/emoji/archived/edit", "stickers-and-emoji/emoji/suggest", "stickers-and-emoji/emoji/quick-reaction", "stickers-and-emoji/emoji/quick-reaction/choose", "stickers-and-emoji/suggest-by-emoji", "stickers-and-emoji/large-emoji", "stickers-and-emoji/dynamic-order", "stickers-and-emoji/emoji/show-more", "app-icon", "tap-for-next-media"
  subsection: String,
}

impl RObject for SettingsSectionAppearance {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionAppearance"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionAppearance {}

impl SettingsSectionAppearance {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionAppearanceBuilder {
    let inner = SettingsSectionAppearance {
      td_name: "settingsSectionAppearance".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionAppearanceBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionAppearanceBuilder {
  inner: SettingsSectionAppearance,
}

impl RTDSettingsSectionAppearanceBuilder {
  pub fn build(&self) -> SettingsSectionAppearance {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionAppearance> for SettingsSectionAppearance {
  fn as_ref(&self) -> &SettingsSectionAppearance {
    self
  }
}

impl AsRef<SettingsSectionAppearance> for RTDSettingsSectionAppearanceBuilder {
  fn as_ref(&self) -> &SettingsSectionAppearance {
    &self.inner
  }
}

/// The "Ask a question" section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionAskQuestion {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SettingsSectionAskQuestion {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionAskQuestion"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionAskQuestion {}

impl SettingsSectionAskQuestion {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionAskQuestionBuilder {
    let inner = SettingsSectionAskQuestion {
      td_name: "settingsSectionAskQuestion".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSettingsSectionAskQuestionBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionAskQuestionBuilder {
  inner: SettingsSectionAskQuestion,
}

impl RTDSettingsSectionAskQuestionBuilder {
  pub fn build(&self) -> SettingsSectionAskQuestion {
    self.inner.clone()
  }
}

impl AsRef<SettingsSectionAskQuestion> for SettingsSectionAskQuestion {
  fn as_ref(&self) -> &SettingsSectionAskQuestion {
    self
  }
}

impl AsRef<SettingsSectionAskQuestion> for RTDSettingsSectionAskQuestionBuilder {
  fn as_ref(&self) -> &SettingsSectionAskQuestion {
    &self.inner
  }
}

/// The "Telegram Business" section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionBusiness {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "do-not-hide-ads"
  subsection: String,
}

impl RObject for SettingsSectionBusiness {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionBusiness"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionBusiness {}

impl SettingsSectionBusiness {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionBusinessBuilder {
    let inner = SettingsSectionBusiness {
      td_name: "settingsSectionBusiness".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionBusinessBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionBusinessBuilder {
  inner: SettingsSectionBusiness,
}

impl RTDSettingsSectionBusinessBuilder {
  pub fn build(&self) -> SettingsSectionBusiness {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionBusiness> for SettingsSectionBusiness {
  fn as_ref(&self) -> &SettingsSectionBusiness {
    self
  }
}

impl AsRef<SettingsSectionBusiness> for RTDSettingsSectionBusinessBuilder {
  fn as_ref(&self) -> &SettingsSectionBusiness {
    &self.inner
  }
}

/// The chat folder settings section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionChatFolders {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "edit", "create", "add-recommended", "show-tags", "tab-view"
  subsection: String,
}

impl RObject for SettingsSectionChatFolders {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionChatFolders"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionChatFolders {}

impl SettingsSectionChatFolders {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionChatFoldersBuilder {
    let inner = SettingsSectionChatFolders {
      td_name: "settingsSectionChatFolders".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionChatFoldersBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionChatFoldersBuilder {
  inner: SettingsSectionChatFolders,
}

impl RTDSettingsSectionChatFoldersBuilder {
  pub fn build(&self) -> SettingsSectionChatFolders {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionChatFolders> for SettingsSectionChatFolders {
  fn as_ref(&self) -> &SettingsSectionChatFolders {
    self
  }
}

impl AsRef<SettingsSectionChatFolders> for RTDSettingsSectionChatFoldersBuilder {
  fn as_ref(&self) -> &SettingsSectionChatFolders {
    &self.inner
  }
}

/// The data and storage settings section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionDataAndStorage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "storage", "storage/edit", "storage/auto-remove", "storage/clear-cache", "storage/max-cache", "usage", "usage/mobile", "usage/wifi", "usage/reset", "usage/roaming", "auto-download/mobile", "auto-download/mobile/enable", "auto-download/mobile/usage", "auto-download/mobile/photos", "auto-download/mobile/stories", "auto-download/mobile/videos", "auto-download/mobile/files", "auto-download/wifi", "auto-download/wifi/enable", "auto-download/wifi/usage", "auto-download/wifi/photos", "auto-download/wifi/stories", "auto-download/wifi/videos", "auto-download/wifi/files", "auto-download/roaming", "auto-download/roaming/enable", "auto-download/roaming/usage", "auto-download/roaming/photos", "auto-download/roaming/stories", "auto-download/roaming/videos", "auto-download/roaming/files", "auto-download/reset", "save-to-photos/chats", "save-to-photos/chats/max-video-size", "save-to-photos/chats/add-exception", "save-to-photos/chats/delete-all", "save-to-photos/groups", "save-to-photos/groups/max-video-size", "save-to-photos/groups/add-exception", "save-to-photos/groups/delete-all", "save-to-photos/channels", "save-to-photos/channels/max-video-size", "save-to-photos/channels/add-exception", "save-to-photos/channels/delete-all", "less-data-calls", "open-links", "share-sheet", "share-sheet/suggested-chats", "share-sheet/suggest-by", "share-sheet/reset", "saved-edited-photos", "pause-music", "raise-to-listen", "raise-to-speak", "show-18-content", "proxy", "proxy/edit", "proxy/use-proxy", "proxy/add-proxy", "proxy/share-list", "proxy/use-for-calls"
  subsection: String,
}

impl RObject for SettingsSectionDataAndStorage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionDataAndStorage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionDataAndStorage {}

impl SettingsSectionDataAndStorage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionDataAndStorageBuilder {
    let inner = SettingsSectionDataAndStorage {
      td_name: "settingsSectionDataAndStorage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionDataAndStorageBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionDataAndStorageBuilder {
  inner: SettingsSectionDataAndStorage,
}

impl RTDSettingsSectionDataAndStorageBuilder {
  pub fn build(&self) -> SettingsSectionDataAndStorage {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionDataAndStorage> for SettingsSectionDataAndStorage {
  fn as_ref(&self) -> &SettingsSectionDataAndStorage {
    self
  }
}

impl AsRef<SettingsSectionDataAndStorage> for RTDSettingsSectionDataAndStorageBuilder {
  fn as_ref(&self) -> &SettingsSectionDataAndStorage {
    &self.inner
  }
}

/// The Devices section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionDevices {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "edit", "link-desktop", "terminate-sessions", "auto-terminate"
  subsection: String,
}

impl RObject for SettingsSectionDevices {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionDevices"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionDevices {}

impl SettingsSectionDevices {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionDevicesBuilder {
    let inner = SettingsSectionDevices {
      td_name: "settingsSectionDevices".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionDevicesBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionDevicesBuilder {
  inner: SettingsSectionDevices,
}

impl RTDSettingsSectionDevicesBuilder {
  pub fn build(&self) -> SettingsSectionDevices {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionDevices> for SettingsSectionDevices {
  fn as_ref(&self) -> &SettingsSectionDevices {
    self
  }
}

impl AsRef<SettingsSectionDevices> for RTDSettingsSectionDevicesBuilder {
  fn as_ref(&self) -> &SettingsSectionDevices {
    &self.inner
  }
}

/// The profile edit section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionEditProfile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "set-photo", "first-name", "last-name", "emoji-status", "bio", "birthday", "change-number", "username", "your-color", "channel", "add-account", "log-out", "profile-color/profile", "profile-color/profile/add-icons", "profile-color/profile/use-gift", "profile-color/name", "profile-color/name/add-icons", "profile-color/name/use-gift", "profile-photo/use-emoji"
  subsection: String,
}

impl RObject for SettingsSectionEditProfile {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionEditProfile"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionEditProfile {}

impl SettingsSectionEditProfile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionEditProfileBuilder {
    let inner = SettingsSectionEditProfile {
      td_name: "settingsSectionEditProfile".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionEditProfileBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionEditProfileBuilder {
  inner: SettingsSectionEditProfile,
}

impl RTDSettingsSectionEditProfileBuilder {
  pub fn build(&self) -> SettingsSectionEditProfile {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionEditProfile> for SettingsSectionEditProfile {
  fn as_ref(&self) -> &SettingsSectionEditProfile {
    self
  }
}

impl AsRef<SettingsSectionEditProfile> for RTDSettingsSectionEditProfileBuilder {
  fn as_ref(&self) -> &SettingsSectionEditProfile {
    &self.inner
  }
}

/// The FAQ section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionFaq {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SettingsSectionFaq {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionFaq"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionFaq {}

impl SettingsSectionFaq {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionFaqBuilder {
    let inner = SettingsSectionFaq {
      td_name: "settingsSectionFaq".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSettingsSectionFaqBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionFaqBuilder {
  inner: SettingsSectionFaq,
}

impl RTDSettingsSectionFaqBuilder {
  pub fn build(&self) -> SettingsSectionFaq {
    self.inner.clone()
  }
}

impl AsRef<SettingsSectionFaq> for SettingsSectionFaq {
  fn as_ref(&self) -> &SettingsSectionFaq {
    self
  }
}

impl AsRef<SettingsSectionFaq> for RTDSettingsSectionFaqBuilder {
  fn as_ref(&self) -> &SettingsSectionFaq {
    &self.inner
  }
}

/// The "Telegram Features" section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionFeatures {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SettingsSectionFeatures {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionFeatures"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionFeatures {}

impl SettingsSectionFeatures {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionFeaturesBuilder {
    let inner = SettingsSectionFeatures {
      td_name: "settingsSectionFeatures".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSettingsSectionFeaturesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionFeaturesBuilder {
  inner: SettingsSectionFeatures,
}

impl RTDSettingsSectionFeaturesBuilder {
  pub fn build(&self) -> SettingsSectionFeatures {
    self.inner.clone()
  }
}

impl AsRef<SettingsSectionFeatures> for SettingsSectionFeatures {
  fn as_ref(&self) -> &SettingsSectionFeatures {
    self
  }
}

impl AsRef<SettingsSectionFeatures> for RTDSettingsSectionFeaturesBuilder {
  fn as_ref(&self) -> &SettingsSectionFeatures {
    &self.inner
  }
}

/// The in-app browser settings section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionInAppBrowser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "enable-browser", "clear-cookies", "clear-cache", "history", "clear-history", "never-open", "clear-list", "search"
  subsection: String,
}

impl RObject for SettingsSectionInAppBrowser {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionInAppBrowser"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionInAppBrowser {}

impl SettingsSectionInAppBrowser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionInAppBrowserBuilder {
    let inner = SettingsSectionInAppBrowser {
      td_name: "settingsSectionInAppBrowser".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionInAppBrowserBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionInAppBrowserBuilder {
  inner: SettingsSectionInAppBrowser,
}

impl RTDSettingsSectionInAppBrowserBuilder {
  pub fn build(&self) -> SettingsSectionInAppBrowser {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionInAppBrowser> for SettingsSectionInAppBrowser {
  fn as_ref(&self) -> &SettingsSectionInAppBrowser {
    self
  }
}

impl AsRef<SettingsSectionInAppBrowser> for RTDSettingsSectionInAppBrowserBuilder {
  fn as_ref(&self) -> &SettingsSectionInAppBrowser {
    &self.inner
  }
}

/// The application language section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionLanguage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "show-button" for Show Translate Button toggle, "translate-chats" for Translate Entire Chats toggle, "do-not-translate"  for Do Not Translate language list
  subsection: String,
}

impl RObject for SettingsSectionLanguage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionLanguage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionLanguage {}

impl SettingsSectionLanguage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionLanguageBuilder {
    let inner = SettingsSectionLanguage {
      td_name: "settingsSectionLanguage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionLanguageBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionLanguageBuilder {
  inner: SettingsSectionLanguage,
}

impl RTDSettingsSectionLanguageBuilder {
  pub fn build(&self) -> SettingsSectionLanguage {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionLanguage> for SettingsSectionLanguage {
  fn as_ref(&self) -> &SettingsSectionLanguage {
    self
  }
}

impl AsRef<SettingsSectionLanguage> for RTDSettingsSectionLanguageBuilder {
  fn as_ref(&self) -> &SettingsSectionLanguage {
    &self.inner
  }
}

/// The TON Gram balance and transaction section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionMyGrams {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SettingsSectionMyGrams {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionMyGrams"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionMyGrams {}

impl SettingsSectionMyGrams {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionMyGramsBuilder {
    let inner = SettingsSectionMyGrams {
      td_name: "settingsSectionMyGrams".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSettingsSectionMyGramsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionMyGramsBuilder {
  inner: SettingsSectionMyGrams,
}

impl RTDSettingsSectionMyGramsBuilder {
  pub fn build(&self) -> SettingsSectionMyGrams {
    self.inner.clone()
  }
}

impl AsRef<SettingsSectionMyGrams> for SettingsSectionMyGrams {
  fn as_ref(&self) -> &SettingsSectionMyGrams {
    self
  }
}

impl AsRef<SettingsSectionMyGrams> for RTDSettingsSectionMyGramsBuilder {
  fn as_ref(&self) -> &SettingsSectionMyGrams {
    &self.inner
  }
}

/// The Telegram Star balance and transaction section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionMyStars {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "top-up", "stats", "gift", "earn"
  subsection: String,
}

impl RObject for SettingsSectionMyStars {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionMyStars"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionMyStars {}

impl SettingsSectionMyStars {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionMyStarsBuilder {
    let inner = SettingsSectionMyStars {
      td_name: "settingsSectionMyStars".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionMyStarsBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionMyStarsBuilder {
  inner: SettingsSectionMyStars,
}

impl RTDSettingsSectionMyStarsBuilder {
  pub fn build(&self) -> SettingsSectionMyStars {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionMyStars> for SettingsSectionMyStars {
  fn as_ref(&self) -> &SettingsSectionMyStars {
    self
  }
}

impl AsRef<SettingsSectionMyStars> for RTDSettingsSectionMyStarsBuilder {
  fn as_ref(&self) -> &SettingsSectionMyStars {
    &self.inner
  }
}

/// The notification settings section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionNotifications {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "accounts", "private-chats", "private-chats/edit", "private-chats/show", "private-chats/preview", "private-chats/sound", "private-chats/add-exception", "private-chats/delete-exceptions", "private-chats/light-color", "private-chats/vibrate", "private-chats/priority", "groups", "groups/edit", "groups/show", "groups/preview", "groups/sound", "groups/add-exception", "groups/delete-exceptions", "groups/light-color", "groups/vibrate", "groups/priority", "channels", "channels/edit", "channels/show", "channels/preview", "channels/sound", "channels/add-exception", "channels/delete-exceptions", "channels/light-color", "channels/vibrate", "channels/priority", "stories", "stories/new", "stories/important", "stories/show-sender", "stories/sound", "stories/add-exception", "stories/delete-exceptions", "stories/light-color", "stories/vibrate", "stories/priority", "reactions", "reactions/messages", "reactions/stories", "reactions/show-sender", "reactions/sound", "reactions/light-color", "reactions/vibrate", "reactions/priority", "in-app-sounds", "in-app-vibrate", "in-app-preview", "in-chat-sounds", "in-app-popup", "lock-screen-names", "include-channels", "include-muted-chats", "count-unread-messages", "new-contacts", "pinned-messages", "reset", "web"
  subsection: String,
}

impl RObject for SettingsSectionNotifications {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionNotifications"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionNotifications {}

impl SettingsSectionNotifications {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionNotificationsBuilder {
    let inner = SettingsSectionNotifications {
      td_name: "settingsSectionNotifications".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionNotificationsBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionNotificationsBuilder {
  inner: SettingsSectionNotifications,
}

impl RTDSettingsSectionNotificationsBuilder {
  pub fn build(&self) -> SettingsSectionNotifications {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionNotifications> for SettingsSectionNotifications {
  fn as_ref(&self) -> &SettingsSectionNotifications {
    self
  }
}

impl AsRef<SettingsSectionNotifications> for RTDSettingsSectionNotificationsBuilder {
  fn as_ref(&self) -> &SettingsSectionNotifications {
    &self.inner
  }
}

/// The power saving settings section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionPowerSaving {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "videos", "gifs", "stickers", "emoji", "effects", "preload", "background", "call-animations", "particles", "transitions"
  subsection: String,
}

impl RObject for SettingsSectionPowerSaving {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionPowerSaving"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionPowerSaving {}

impl SettingsSectionPowerSaving {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionPowerSavingBuilder {
    let inner = SettingsSectionPowerSaving {
      td_name: "settingsSectionPowerSaving".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionPowerSavingBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionPowerSavingBuilder {
  inner: SettingsSectionPowerSaving,
}

impl RTDSettingsSectionPowerSavingBuilder {
  pub fn build(&self) -> SettingsSectionPowerSaving {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionPowerSaving> for SettingsSectionPowerSaving {
  fn as_ref(&self) -> &SettingsSectionPowerSaving {
    self
  }
}

impl AsRef<SettingsSectionPowerSaving> for RTDSettingsSectionPowerSavingBuilder {
  fn as_ref(&self) -> &SettingsSectionPowerSaving {
    &self.inner
  }
}

/// The "Telegram Premium" section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionPremium {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SettingsSectionPremium {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionPremium"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionPremium {}

impl SettingsSectionPremium {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionPremiumBuilder {
    let inner = SettingsSectionPremium {
      td_name: "settingsSectionPremium".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSettingsSectionPremiumBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionPremiumBuilder {
  inner: SettingsSectionPremium,
}

impl RTDSettingsSectionPremiumBuilder {
  pub fn build(&self) -> SettingsSectionPremium {
    self.inner.clone()
  }
}

impl AsRef<SettingsSectionPremium> for SettingsSectionPremium {
  fn as_ref(&self) -> &SettingsSectionPremium {
    self
  }
}

impl AsRef<SettingsSectionPremium> for RTDSettingsSectionPremiumBuilder {
  fn as_ref(&self) -> &SettingsSectionPremium {
    &self.inner
  }
}

/// The privacy and security section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionPrivacyAndSecurity {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "blocked", "blocked/edit", "blocked/block-user", "blocked/block-user/chats", "blocked/block-user/contacts", "active-websites", "active-websites/edit", "active-websites/disconnect-all", "passcode", "passcode/disable", "passcode/change", "passcode/auto-lock", "passcode/face-id", "passcode/fingerprint", "2sv", "2sv/change", "2sv/disable", "2sv/change-email", "passkey", "passkey/create", "auto-delete", "auto-delete/set-custom", "login-email", "phone-number", "phone-number/never", "phone-number/always", "last-seen", "last-seen/never", "last-seen/always", "last-seen/hide-read-time", "profile-photos", "profile-photos/never", "profile-photos/always", "profile-photos/set-public", "profile-photos/update-public", "profile-photos/remove-public", "bio", "bio/never", "bio/always", "gifts", "gifts/show-icon", "gifts/never", "gifts/always", "gifts/accepted-types", "birthday", "birthday/add", "birthday/never", "birthday/always", "saved-music", "saved-music/never", "saved-music/always", "forwards", "forwards/never", "forwards/always", "calls", "calls/never", "calls/always", "calls/p2p", "calls/p2p/never", "calls/p2p/always", "calls/ios-integration", "voice", "voice/never", "voice/always", "messages", "messages/set-price", "messages/exceptions", "invites", "invites/never", "invites/always", "self-destruct", "data-settings", "data-settings/sync-contacts", "data-settings/delete-synced", "data-settings/suggest-contacts", "data-settings/delete-cloud-drafts", "data-settings/clear-payment-info", "data-settings/link-previews", "data-settings/bot-settings", "data-settings/map-provider", "archive-and-mute"
  subsection: String,
}

impl RObject for SettingsSectionPrivacyAndSecurity {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionPrivacyAndSecurity"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionPrivacyAndSecurity {}

impl SettingsSectionPrivacyAndSecurity {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionPrivacyAndSecurityBuilder {
    let inner = SettingsSectionPrivacyAndSecurity {
      td_name: "settingsSectionPrivacyAndSecurity".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionPrivacyAndSecurityBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionPrivacyAndSecurityBuilder {
  inner: SettingsSectionPrivacyAndSecurity,
}

impl RTDSettingsSectionPrivacyAndSecurityBuilder {
  pub fn build(&self) -> SettingsSectionPrivacyAndSecurity {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionPrivacyAndSecurity> for SettingsSectionPrivacyAndSecurity {
  fn as_ref(&self) -> &SettingsSectionPrivacyAndSecurity {
    self
  }
}

impl AsRef<SettingsSectionPrivacyAndSecurity> for RTDSettingsSectionPrivacyAndSecurityBuilder {
  fn as_ref(&self) -> &SettingsSectionPrivacyAndSecurity {
    &self.inner
  }
}

/// The "Privacy Policy" section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionPrivacyPolicy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SettingsSectionPrivacyPolicy {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionPrivacyPolicy"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionPrivacyPolicy {}

impl SettingsSectionPrivacyPolicy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionPrivacyPolicyBuilder {
    let inner = SettingsSectionPrivacyPolicy {
      td_name: "settingsSectionPrivacyPolicy".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSettingsSectionPrivacyPolicyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionPrivacyPolicyBuilder {
  inner: SettingsSectionPrivacyPolicy,
}

impl RTDSettingsSectionPrivacyPolicyBuilder {
  pub fn build(&self) -> SettingsSectionPrivacyPolicy {
    self.inner.clone()
  }
}

impl AsRef<SettingsSectionPrivacyPolicy> for SettingsSectionPrivacyPolicy {
  fn as_ref(&self) -> &SettingsSectionPrivacyPolicy {
    self
  }
}

impl AsRef<SettingsSectionPrivacyPolicy> for RTDSettingsSectionPrivacyPolicyBuilder {
  fn as_ref(&self) -> &SettingsSectionPrivacyPolicy {
    &self.inner
  }
}

/// The current user's QR code section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionQrCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "share", "scan"
  subsection: String,
}

impl RObject for SettingsSectionQrCode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionQrCode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionQrCode {}

impl SettingsSectionQrCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionQrCodeBuilder {
    let inner = SettingsSectionQrCode {
      td_name: "settingsSectionQrCode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionQrCodeBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionQrCodeBuilder {
  inner: SettingsSectionQrCode,
}

impl RTDSettingsSectionQrCodeBuilder {
  pub fn build(&self) -> SettingsSectionQrCode {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionQrCode> for SettingsSectionQrCode {
  fn as_ref(&self) -> &SettingsSectionQrCode {
    self
  }
}

impl AsRef<SettingsSectionQrCode> for RTDSettingsSectionQrCodeBuilder {
  fn as_ref(&self) -> &SettingsSectionQrCode {
    &self.inner
  }
}

/// Search in Settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionSearch {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SettingsSectionSearch {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionSearch"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionSearch {}

impl SettingsSectionSearch {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionSearchBuilder {
    let inner = SettingsSectionSearch {
      td_name: "settingsSectionSearch".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSettingsSectionSearchBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionSearchBuilder {
  inner: SettingsSectionSearch,
}

impl RTDSettingsSectionSearchBuilder {
  pub fn build(&self) -> SettingsSectionSearch {
    self.inner.clone()
  }
}

impl AsRef<SettingsSectionSearch> for SettingsSectionSearch {
  fn as_ref(&self) -> &SettingsSectionSearch {
    self
  }
}

impl AsRef<SettingsSectionSearch> for RTDSettingsSectionSearchBuilder {
  fn as_ref(&self) -> &SettingsSectionSearch {
    &self.inner
  }
}

/// The "Send a gift" section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsSectionSendGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subsection of the section; may be one of "", "self"
  subsection: String,
}

impl RObject for SettingsSectionSendGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "settingsSectionSendGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSettingsSection for SettingsSectionSendGift {}

impl SettingsSectionSendGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSettingsSectionSendGiftBuilder {
    let inner = SettingsSectionSendGift {
      td_name: "settingsSectionSendGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSettingsSectionSendGiftBuilder { inner }
  }

  pub fn subsection(&self) -> &String {
    &self.subsection
  }
}

#[doc(hidden)]
pub struct RTDSettingsSectionSendGiftBuilder {
  inner: SettingsSectionSendGift,
}

impl RTDSettingsSectionSendGiftBuilder {
  pub fn build(&self) -> SettingsSectionSendGift {
    self.inner.clone()
  }

  pub fn subsection<T: AsRef<str>>(&mut self, subsection: T) -> &mut Self {
    self.inner.subsection = subsection.as_ref().to_string();
    self
  }
}

impl AsRef<SettingsSectionSendGift> for SettingsSectionSendGift {
  fn as_ref(&self) -> &SettingsSectionSendGift {
    self
  }
}

impl AsRef<SettingsSectionSendGift> for RTDSettingsSectionSendGiftBuilder {
  fn as_ref(&self) -> &SettingsSectionSendGift {
    &self.inner
  }
}
