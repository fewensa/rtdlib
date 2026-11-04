use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a feature available to Premium users
pub trait TDPremiumFeature: Debug + RObject {}

/// Describes a feature available to Premium users
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PremiumFeature {
  #[doc(hidden)]
  _Default(()),
  /// The ability to choose accent color for replies and user profile
  AccentColor(PremiumFeatureAccentColor),
  /// Ability to change position of the main chat list, archive and mute all new chats from non-contacts, and completely disable notifications about the user's contacts joined Telegram
  AdvancedChatManagement(PremiumFeatureAdvancedChatManagement),
  /// Profile photo animation on message and chat screens
  AnimatedProfilePhoto(PremiumFeatureAnimatedProfilePhoto),
  /// Allowed to set premium application icons
  AppIcons(PremiumFeatureAppIcons),
  /// The ability to set private chat background for both users
  BackgroundForBoth(PremiumFeatureBackgroundForBoth),
  /// The ability to use Business features
  Business(PremiumFeatureBusiness),
  /// The ability to boost chats
  ChatBoost(PremiumFeatureChatBoost),
  /// The ability to create and use checklist messages
  Checklists(PremiumFeatureChecklists),
  /// Allowed to use custom emoji stickers in message texts and captions
  CustomEmoji(PremiumFeatureCustomEmoji),
  /// Disabled ads
  DisabledAds(PremiumFeatureDisabledAds),
  /// The ability to show an emoji status along with the user's name
  EmojiStatus(PremiumFeatureEmojiStatus),
  /// The ability to set a custom emoji as a forum topic icon
  ForumTopicIcon(PremiumFeatureForumTopicIcon),
  /// Improved download speed
  ImprovedDownloadSpeed(PremiumFeatureImprovedDownloadSpeed),
  /// Increased limits
  IncreasedLimits(PremiumFeatureIncreasedLimits),
  /// Increased maximum upload file size
  IncreasedUploadFileSize(PremiumFeatureIncreasedUploadFileSize),
  /// The ability to view last seen and read times of other users even if they can't view last seen or read time for the current user
  LastSeenTimes(PremiumFeatureLastSeenTimes),
  /// The ability to use all available message effects
  MessageEffects(PremiumFeatureMessageEffects),
  /// The ability to disallow incoming voice and video note messages in private chats using setUserPrivacySettingRules with userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages and to restrict incoming messages from non-contacts using setNewChatPrivacySettings
  MessagePrivacy(PremiumFeatureMessagePrivacy),
  /// The ability to require a payment for incoming messages in new chats
  PaidMessages(PremiumFeaturePaidMessages),
  /// A badge in the user's profile
  ProfileBadge(PremiumFeatureProfileBadge),
  /// The ability to enable content protection in private chats
  ProtectPrivateChatContent(PremiumFeatureProtectPrivateChatContent),
  /// Allowed to translate chat messages real-time
  RealTimeChatTranslation(PremiumFeatureRealTimeChatTranslation),
  /// The ability to send rich messages
  RichMessages(PremiumFeatureRichMessages),
  /// The ability to use tags in Saved Messages
  SavedMessagesTags(PremiumFeatureSavedMessagesTags),
  /// The ability to compose text with AI
  TextComposition(PremiumFeatureTextComposition),
  /// Allowed to use more reactions
  UniqueReactions(PremiumFeatureUniqueReactions),
  /// Allowed to use premium stickers with unique effects
  UniqueStickers(PremiumFeatureUniqueStickers),
  /// Allowed to use many additional features for stories
  UpgradedStories(PremiumFeatureUpgradedStories),
  /// The ability to convert voice notes to text
  VoiceRecognition(PremiumFeatureVoiceRecognition),
}

impl Default for PremiumFeature {
  fn default() -> Self {
    PremiumFeature::_Default(())
  }
}

impl<'de> Deserialize<'de> for PremiumFeature {
  fn deserialize<D>(deserializer: D) -> Result<PremiumFeature, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PremiumFeature,
      (premiumFeatureAccentColor, AccentColor);
      (premiumFeatureAdvancedChatManagement, AdvancedChatManagement);
      (premiumFeatureAnimatedProfilePhoto, AnimatedProfilePhoto);
      (premiumFeatureAppIcons, AppIcons);
      (premiumFeatureBackgroundForBoth, BackgroundForBoth);
      (premiumFeatureBusiness, Business);
      (premiumFeatureChatBoost, ChatBoost);
      (premiumFeatureChecklists, Checklists);
      (premiumFeatureCustomEmoji, CustomEmoji);
      (premiumFeatureDisabledAds, DisabledAds);
      (premiumFeatureEmojiStatus, EmojiStatus);
      (premiumFeatureForumTopicIcon, ForumTopicIcon);
      (premiumFeatureImprovedDownloadSpeed, ImprovedDownloadSpeed);
      (premiumFeatureIncreasedLimits, IncreasedLimits);
      (premiumFeatureIncreasedUploadFileSize, IncreasedUploadFileSize);
      (premiumFeatureLastSeenTimes, LastSeenTimes);
      (premiumFeatureMessageEffects, MessageEffects);
      (premiumFeatureMessagePrivacy, MessagePrivacy);
      (premiumFeaturePaidMessages, PaidMessages);
      (premiumFeatureProfileBadge, ProfileBadge);
      (premiumFeatureProtectPrivateChatContent, ProtectPrivateChatContent);
      (premiumFeatureRealTimeChatTranslation, RealTimeChatTranslation);
      (premiumFeatureRichMessages, RichMessages);
      (premiumFeatureSavedMessagesTags, SavedMessagesTags);
      (premiumFeatureTextComposition, TextComposition);
      (premiumFeatureUniqueReactions, UniqueReactions);
      (premiumFeatureUniqueStickers, UniqueStickers);
      (premiumFeatureUpgradedStories, UpgradedStories);
      (premiumFeatureVoiceRecognition, VoiceRecognition);

    )(deserializer)
  }
}

impl RObject for PremiumFeature {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PremiumFeature::AccentColor(t) => t.td_name(),
      PremiumFeature::AdvancedChatManagement(t) => t.td_name(),
      PremiumFeature::AnimatedProfilePhoto(t) => t.td_name(),
      PremiumFeature::AppIcons(t) => t.td_name(),
      PremiumFeature::BackgroundForBoth(t) => t.td_name(),
      PremiumFeature::Business(t) => t.td_name(),
      PremiumFeature::ChatBoost(t) => t.td_name(),
      PremiumFeature::Checklists(t) => t.td_name(),
      PremiumFeature::CustomEmoji(t) => t.td_name(),
      PremiumFeature::DisabledAds(t) => t.td_name(),
      PremiumFeature::EmojiStatus(t) => t.td_name(),
      PremiumFeature::ForumTopicIcon(t) => t.td_name(),
      PremiumFeature::ImprovedDownloadSpeed(t) => t.td_name(),
      PremiumFeature::IncreasedLimits(t) => t.td_name(),
      PremiumFeature::IncreasedUploadFileSize(t) => t.td_name(),
      PremiumFeature::LastSeenTimes(t) => t.td_name(),
      PremiumFeature::MessageEffects(t) => t.td_name(),
      PremiumFeature::MessagePrivacy(t) => t.td_name(),
      PremiumFeature::PaidMessages(t) => t.td_name(),
      PremiumFeature::ProfileBadge(t) => t.td_name(),
      PremiumFeature::ProtectPrivateChatContent(t) => t.td_name(),
      PremiumFeature::RealTimeChatTranslation(t) => t.td_name(),
      PremiumFeature::RichMessages(t) => t.td_name(),
      PremiumFeature::SavedMessagesTags(t) => t.td_name(),
      PremiumFeature::TextComposition(t) => t.td_name(),
      PremiumFeature::UniqueReactions(t) => t.td_name(),
      PremiumFeature::UniqueStickers(t) => t.td_name(),
      PremiumFeature::UpgradedStories(t) => t.td_name(),
      PremiumFeature::VoiceRecognition(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PremiumFeature::AccentColor(t) => t.extra(),
      PremiumFeature::AdvancedChatManagement(t) => t.extra(),
      PremiumFeature::AnimatedProfilePhoto(t) => t.extra(),
      PremiumFeature::AppIcons(t) => t.extra(),
      PremiumFeature::BackgroundForBoth(t) => t.extra(),
      PremiumFeature::Business(t) => t.extra(),
      PremiumFeature::ChatBoost(t) => t.extra(),
      PremiumFeature::Checklists(t) => t.extra(),
      PremiumFeature::CustomEmoji(t) => t.extra(),
      PremiumFeature::DisabledAds(t) => t.extra(),
      PremiumFeature::EmojiStatus(t) => t.extra(),
      PremiumFeature::ForumTopicIcon(t) => t.extra(),
      PremiumFeature::ImprovedDownloadSpeed(t) => t.extra(),
      PremiumFeature::IncreasedLimits(t) => t.extra(),
      PremiumFeature::IncreasedUploadFileSize(t) => t.extra(),
      PremiumFeature::LastSeenTimes(t) => t.extra(),
      PremiumFeature::MessageEffects(t) => t.extra(),
      PremiumFeature::MessagePrivacy(t) => t.extra(),
      PremiumFeature::PaidMessages(t) => t.extra(),
      PremiumFeature::ProfileBadge(t) => t.extra(),
      PremiumFeature::ProtectPrivateChatContent(t) => t.extra(),
      PremiumFeature::RealTimeChatTranslation(t) => t.extra(),
      PremiumFeature::RichMessages(t) => t.extra(),
      PremiumFeature::SavedMessagesTags(t) => t.extra(),
      PremiumFeature::TextComposition(t) => t.extra(),
      PremiumFeature::UniqueReactions(t) => t.extra(),
      PremiumFeature::UniqueStickers(t) => t.extra(),
      PremiumFeature::UpgradedStories(t) => t.extra(),
      PremiumFeature::VoiceRecognition(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumFeature {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PremiumFeature::_Default(_))
  }

  pub fn is_accent_color(&self) -> bool {
    matches!(self, PremiumFeature::AccentColor(_))
  }
  pub fn is_advanced_chat_management(&self) -> bool {
    matches!(self, PremiumFeature::AdvancedChatManagement(_))
  }
  pub fn is_animated_profile_photo(&self) -> bool {
    matches!(self, PremiumFeature::AnimatedProfilePhoto(_))
  }
  pub fn is_app_icons(&self) -> bool {
    matches!(self, PremiumFeature::AppIcons(_))
  }
  pub fn is_background_for_both(&self) -> bool {
    matches!(self, PremiumFeature::BackgroundForBoth(_))
  }
  pub fn is_business(&self) -> bool {
    matches!(self, PremiumFeature::Business(_))
  }
  pub fn is_chat_boost(&self) -> bool {
    matches!(self, PremiumFeature::ChatBoost(_))
  }
  pub fn is_checklists(&self) -> bool {
    matches!(self, PremiumFeature::Checklists(_))
  }
  pub fn is_custom_emoji(&self) -> bool {
    matches!(self, PremiumFeature::CustomEmoji(_))
  }
  pub fn is_disabled_ads(&self) -> bool {
    matches!(self, PremiumFeature::DisabledAds(_))
  }
  pub fn is_emoji_status(&self) -> bool {
    matches!(self, PremiumFeature::EmojiStatus(_))
  }
  pub fn is_forum_topic_icon(&self) -> bool {
    matches!(self, PremiumFeature::ForumTopicIcon(_))
  }
  pub fn is_improved_download_speed(&self) -> bool {
    matches!(self, PremiumFeature::ImprovedDownloadSpeed(_))
  }
  pub fn is_increased_limits(&self) -> bool {
    matches!(self, PremiumFeature::IncreasedLimits(_))
  }
  pub fn is_increased_upload_file_size(&self) -> bool {
    matches!(self, PremiumFeature::IncreasedUploadFileSize(_))
  }
  pub fn is_last_seen_times(&self) -> bool {
    matches!(self, PremiumFeature::LastSeenTimes(_))
  }
  pub fn is_message_effects(&self) -> bool {
    matches!(self, PremiumFeature::MessageEffects(_))
  }
  pub fn is_message_privacy(&self) -> bool {
    matches!(self, PremiumFeature::MessagePrivacy(_))
  }
  pub fn is_paid_messages(&self) -> bool {
    matches!(self, PremiumFeature::PaidMessages(_))
  }
  pub fn is_profile_badge(&self) -> bool {
    matches!(self, PremiumFeature::ProfileBadge(_))
  }
  pub fn is_protect_private_chat_content(&self) -> bool {
    matches!(self, PremiumFeature::ProtectPrivateChatContent(_))
  }
  pub fn is_real_time_chat_translation(&self) -> bool {
    matches!(self, PremiumFeature::RealTimeChatTranslation(_))
  }
  pub fn is_rich_messages(&self) -> bool {
    matches!(self, PremiumFeature::RichMessages(_))
  }
  pub fn is_saved_messages_tags(&self) -> bool {
    matches!(self, PremiumFeature::SavedMessagesTags(_))
  }
  pub fn is_text_composition(&self) -> bool {
    matches!(self, PremiumFeature::TextComposition(_))
  }
  pub fn is_unique_reactions(&self) -> bool {
    matches!(self, PremiumFeature::UniqueReactions(_))
  }
  pub fn is_unique_stickers(&self) -> bool {
    matches!(self, PremiumFeature::UniqueStickers(_))
  }
  pub fn is_upgraded_stories(&self) -> bool {
    matches!(self, PremiumFeature::UpgradedStories(_))
  }
  pub fn is_voice_recognition(&self) -> bool {
    matches!(self, PremiumFeature::VoiceRecognition(_))
  }

  pub fn on_accent_color<F: FnOnce(&PremiumFeatureAccentColor)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::AccentColor(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_advanced_chat_management<F: FnOnce(&PremiumFeatureAdvancedChatManagement)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumFeature::AdvancedChatManagement(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_animated_profile_photo<F: FnOnce(&PremiumFeatureAnimatedProfilePhoto)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumFeature::AnimatedProfilePhoto(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_app_icons<F: FnOnce(&PremiumFeatureAppIcons)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::AppIcons(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_background_for_both<F: FnOnce(&PremiumFeatureBackgroundForBoth)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumFeature::BackgroundForBoth(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_business<F: FnOnce(&PremiumFeatureBusiness)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::Business(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_boost<F: FnOnce(&PremiumFeatureChatBoost)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::ChatBoost(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_checklists<F: FnOnce(&PremiumFeatureChecklists)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::Checklists(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_custom_emoji<F: FnOnce(&PremiumFeatureCustomEmoji)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::CustomEmoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_disabled_ads<F: FnOnce(&PremiumFeatureDisabledAds)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::DisabledAds(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_emoji_status<F: FnOnce(&PremiumFeatureEmojiStatus)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::EmojiStatus(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_forum_topic_icon<F: FnOnce(&PremiumFeatureForumTopicIcon)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::ForumTopicIcon(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_improved_download_speed<F: FnOnce(&PremiumFeatureImprovedDownloadSpeed)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumFeature::ImprovedDownloadSpeed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_increased_limits<F: FnOnce(&PremiumFeatureIncreasedLimits)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::IncreasedLimits(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_increased_upload_file_size<F: FnOnce(&PremiumFeatureIncreasedUploadFileSize)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumFeature::IncreasedUploadFileSize(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_last_seen_times<F: FnOnce(&PremiumFeatureLastSeenTimes)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::LastSeenTimes(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_effects<F: FnOnce(&PremiumFeatureMessageEffects)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::MessageEffects(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_privacy<F: FnOnce(&PremiumFeatureMessagePrivacy)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::MessagePrivacy(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid_messages<F: FnOnce(&PremiumFeaturePaidMessages)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::PaidMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_profile_badge<F: FnOnce(&PremiumFeatureProfileBadge)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::ProfileBadge(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_protect_private_chat_content<F: FnOnce(&PremiumFeatureProtectPrivateChatContent)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumFeature::ProtectPrivateChatContent(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_real_time_chat_translation<F: FnOnce(&PremiumFeatureRealTimeChatTranslation)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumFeature::RealTimeChatTranslation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_rich_messages<F: FnOnce(&PremiumFeatureRichMessages)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::RichMessages(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_messages_tags<F: FnOnce(&PremiumFeatureSavedMessagesTags)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumFeature::SavedMessagesTags(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text_composition<F: FnOnce(&PremiumFeatureTextComposition)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::TextComposition(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unique_reactions<F: FnOnce(&PremiumFeatureUniqueReactions)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::UniqueReactions(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unique_stickers<F: FnOnce(&PremiumFeatureUniqueStickers)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::UniqueStickers(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_stories<F: FnOnce(&PremiumFeatureUpgradedStories)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::UpgradedStories(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_voice_recognition<F: FnOnce(&PremiumFeatureVoiceRecognition)>(&self, fnc: F) -> &Self {
    if let PremiumFeature::VoiceRecognition(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_accent_color(&self) -> Option<&PremiumFeatureAccentColor> {
    if let PremiumFeature::AccentColor(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_advanced_chat_management(&self) -> Option<&PremiumFeatureAdvancedChatManagement> {
    if let PremiumFeature::AdvancedChatManagement(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_animated_profile_photo(&self) -> Option<&PremiumFeatureAnimatedProfilePhoto> {
    if let PremiumFeature::AnimatedProfilePhoto(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_app_icons(&self) -> Option<&PremiumFeatureAppIcons> {
    if let PremiumFeature::AppIcons(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_background_for_both(&self) -> Option<&PremiumFeatureBackgroundForBoth> {
    if let PremiumFeature::BackgroundForBoth(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_business(&self) -> Option<&PremiumFeatureBusiness> {
    if let PremiumFeature::Business(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_boost(&self) -> Option<&PremiumFeatureChatBoost> {
    if let PremiumFeature::ChatBoost(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_checklists(&self) -> Option<&PremiumFeatureChecklists> {
    if let PremiumFeature::Checklists(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_custom_emoji(&self) -> Option<&PremiumFeatureCustomEmoji> {
    if let PremiumFeature::CustomEmoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_disabled_ads(&self) -> Option<&PremiumFeatureDisabledAds> {
    if let PremiumFeature::DisabledAds(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_emoji_status(&self) -> Option<&PremiumFeatureEmojiStatus> {
    if let PremiumFeature::EmojiStatus(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_forum_topic_icon(&self) -> Option<&PremiumFeatureForumTopicIcon> {
    if let PremiumFeature::ForumTopicIcon(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_improved_download_speed(&self) -> Option<&PremiumFeatureImprovedDownloadSpeed> {
    if let PremiumFeature::ImprovedDownloadSpeed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_increased_limits(&self) -> Option<&PremiumFeatureIncreasedLimits> {
    if let PremiumFeature::IncreasedLimits(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_increased_upload_file_size(&self) -> Option<&PremiumFeatureIncreasedUploadFileSize> {
    if let PremiumFeature::IncreasedUploadFileSize(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_last_seen_times(&self) -> Option<&PremiumFeatureLastSeenTimes> {
    if let PremiumFeature::LastSeenTimes(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_effects(&self) -> Option<&PremiumFeatureMessageEffects> {
    if let PremiumFeature::MessageEffects(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_privacy(&self) -> Option<&PremiumFeatureMessagePrivacy> {
    if let PremiumFeature::MessagePrivacy(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid_messages(&self) -> Option<&PremiumFeaturePaidMessages> {
    if let PremiumFeature::PaidMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_profile_badge(&self) -> Option<&PremiumFeatureProfileBadge> {
    if let PremiumFeature::ProfileBadge(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_protect_private_chat_content(
    &self,
  ) -> Option<&PremiumFeatureProtectPrivateChatContent> {
    if let PremiumFeature::ProtectPrivateChatContent(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_real_time_chat_translation(&self) -> Option<&PremiumFeatureRealTimeChatTranslation> {
    if let PremiumFeature::RealTimeChatTranslation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_rich_messages(&self) -> Option<&PremiumFeatureRichMessages> {
    if let PremiumFeature::RichMessages(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_messages_tags(&self) -> Option<&PremiumFeatureSavedMessagesTags> {
    if let PremiumFeature::SavedMessagesTags(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text_composition(&self) -> Option<&PremiumFeatureTextComposition> {
    if let PremiumFeature::TextComposition(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unique_reactions(&self) -> Option<&PremiumFeatureUniqueReactions> {
    if let PremiumFeature::UniqueReactions(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unique_stickers(&self) -> Option<&PremiumFeatureUniqueStickers> {
    if let PremiumFeature::UniqueStickers(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_stories(&self) -> Option<&PremiumFeatureUpgradedStories> {
    if let PremiumFeature::UpgradedStories(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_voice_recognition(&self) -> Option<&PremiumFeatureVoiceRecognition> {
    if let PremiumFeature::VoiceRecognition(t) = self {
      return Some(t);
    }
    None
  }

  pub fn accent_color<T: AsRef<PremiumFeatureAccentColor>>(t: T) -> Self {
    PremiumFeature::AccentColor(t.as_ref().clone())
  }

  pub fn advanced_chat_management<T: AsRef<PremiumFeatureAdvancedChatManagement>>(t: T) -> Self {
    PremiumFeature::AdvancedChatManagement(t.as_ref().clone())
  }

  pub fn animated_profile_photo<T: AsRef<PremiumFeatureAnimatedProfilePhoto>>(t: T) -> Self {
    PremiumFeature::AnimatedProfilePhoto(t.as_ref().clone())
  }

  pub fn app_icons<T: AsRef<PremiumFeatureAppIcons>>(t: T) -> Self {
    PremiumFeature::AppIcons(t.as_ref().clone())
  }

  pub fn background_for_both<T: AsRef<PremiumFeatureBackgroundForBoth>>(t: T) -> Self {
    PremiumFeature::BackgroundForBoth(t.as_ref().clone())
  }

  pub fn business<T: AsRef<PremiumFeatureBusiness>>(t: T) -> Self {
    PremiumFeature::Business(t.as_ref().clone())
  }

  pub fn chat_boost<T: AsRef<PremiumFeatureChatBoost>>(t: T) -> Self {
    PremiumFeature::ChatBoost(t.as_ref().clone())
  }

  pub fn checklists<T: AsRef<PremiumFeatureChecklists>>(t: T) -> Self {
    PremiumFeature::Checklists(t.as_ref().clone())
  }

  pub fn custom_emoji<T: AsRef<PremiumFeatureCustomEmoji>>(t: T) -> Self {
    PremiumFeature::CustomEmoji(t.as_ref().clone())
  }

  pub fn disabled_ads<T: AsRef<PremiumFeatureDisabledAds>>(t: T) -> Self {
    PremiumFeature::DisabledAds(t.as_ref().clone())
  }

  pub fn emoji_status<T: AsRef<PremiumFeatureEmojiStatus>>(t: T) -> Self {
    PremiumFeature::EmojiStatus(t.as_ref().clone())
  }

  pub fn forum_topic_icon<T: AsRef<PremiumFeatureForumTopicIcon>>(t: T) -> Self {
    PremiumFeature::ForumTopicIcon(t.as_ref().clone())
  }

  pub fn improved_download_speed<T: AsRef<PremiumFeatureImprovedDownloadSpeed>>(t: T) -> Self {
    PremiumFeature::ImprovedDownloadSpeed(t.as_ref().clone())
  }

  pub fn increased_limits<T: AsRef<PremiumFeatureIncreasedLimits>>(t: T) -> Self {
    PremiumFeature::IncreasedLimits(t.as_ref().clone())
  }

  pub fn increased_upload_file_size<T: AsRef<PremiumFeatureIncreasedUploadFileSize>>(t: T) -> Self {
    PremiumFeature::IncreasedUploadFileSize(t.as_ref().clone())
  }

  pub fn last_seen_times<T: AsRef<PremiumFeatureLastSeenTimes>>(t: T) -> Self {
    PremiumFeature::LastSeenTimes(t.as_ref().clone())
  }

  pub fn message_effects<T: AsRef<PremiumFeatureMessageEffects>>(t: T) -> Self {
    PremiumFeature::MessageEffects(t.as_ref().clone())
  }

  pub fn message_privacy<T: AsRef<PremiumFeatureMessagePrivacy>>(t: T) -> Self {
    PremiumFeature::MessagePrivacy(t.as_ref().clone())
  }

  pub fn paid_messages<T: AsRef<PremiumFeaturePaidMessages>>(t: T) -> Self {
    PremiumFeature::PaidMessages(t.as_ref().clone())
  }

  pub fn profile_badge<T: AsRef<PremiumFeatureProfileBadge>>(t: T) -> Self {
    PremiumFeature::ProfileBadge(t.as_ref().clone())
  }

  pub fn protect_private_chat_content<T: AsRef<PremiumFeatureProtectPrivateChatContent>>(
    t: T,
  ) -> Self {
    PremiumFeature::ProtectPrivateChatContent(t.as_ref().clone())
  }

  pub fn real_time_chat_translation<T: AsRef<PremiumFeatureRealTimeChatTranslation>>(t: T) -> Self {
    PremiumFeature::RealTimeChatTranslation(t.as_ref().clone())
  }

  pub fn rich_messages<T: AsRef<PremiumFeatureRichMessages>>(t: T) -> Self {
    PremiumFeature::RichMessages(t.as_ref().clone())
  }

  pub fn saved_messages_tags<T: AsRef<PremiumFeatureSavedMessagesTags>>(t: T) -> Self {
    PremiumFeature::SavedMessagesTags(t.as_ref().clone())
  }

  pub fn text_composition<T: AsRef<PremiumFeatureTextComposition>>(t: T) -> Self {
    PremiumFeature::TextComposition(t.as_ref().clone())
  }

  pub fn unique_reactions<T: AsRef<PremiumFeatureUniqueReactions>>(t: T) -> Self {
    PremiumFeature::UniqueReactions(t.as_ref().clone())
  }

  pub fn unique_stickers<T: AsRef<PremiumFeatureUniqueStickers>>(t: T) -> Self {
    PremiumFeature::UniqueStickers(t.as_ref().clone())
  }

  pub fn upgraded_stories<T: AsRef<PremiumFeatureUpgradedStories>>(t: T) -> Self {
    PremiumFeature::UpgradedStories(t.as_ref().clone())
  }

  pub fn voice_recognition<T: AsRef<PremiumFeatureVoiceRecognition>>(t: T) -> Self {
    PremiumFeature::VoiceRecognition(t.as_ref().clone())
  }
}

impl AsRef<PremiumFeature> for PremiumFeature {
  fn as_ref(&self) -> &PremiumFeature {
    self
  }
}

/// The ability to choose accent color for replies and user profile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureAccentColor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureAccentColor {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureAccentColor"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureAccentColor {}

impl PremiumFeatureAccentColor {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureAccentColorBuilder {
    let inner = PremiumFeatureAccentColor {
      td_name: "premiumFeatureAccentColor".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureAccentColorBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureAccentColorBuilder {
  inner: PremiumFeatureAccentColor,
}

impl RTDPremiumFeatureAccentColorBuilder {
  pub fn build(&self) -> PremiumFeatureAccentColor {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureAccentColor> for PremiumFeatureAccentColor {
  fn as_ref(&self) -> &PremiumFeatureAccentColor {
    self
  }
}

impl AsRef<PremiumFeatureAccentColor> for RTDPremiumFeatureAccentColorBuilder {
  fn as_ref(&self) -> &PremiumFeatureAccentColor {
    &self.inner
  }
}

/// Ability to change position of the main chat list, archive and mute all new chats from non-contacts, and completely disable notifications about the user's contacts joined Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureAdvancedChatManagement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureAdvancedChatManagement {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureAdvancedChatManagement"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureAdvancedChatManagement {}

impl PremiumFeatureAdvancedChatManagement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureAdvancedChatManagementBuilder {
    let inner = PremiumFeatureAdvancedChatManagement {
      td_name: "premiumFeatureAdvancedChatManagement".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureAdvancedChatManagementBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureAdvancedChatManagementBuilder {
  inner: PremiumFeatureAdvancedChatManagement,
}

impl RTDPremiumFeatureAdvancedChatManagementBuilder {
  pub fn build(&self) -> PremiumFeatureAdvancedChatManagement {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureAdvancedChatManagement> for PremiumFeatureAdvancedChatManagement {
  fn as_ref(&self) -> &PremiumFeatureAdvancedChatManagement {
    self
  }
}

impl AsRef<PremiumFeatureAdvancedChatManagement>
  for RTDPremiumFeatureAdvancedChatManagementBuilder
{
  fn as_ref(&self) -> &PremiumFeatureAdvancedChatManagement {
    &self.inner
  }
}

/// Profile photo animation on message and chat screens
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureAnimatedProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureAnimatedProfilePhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureAnimatedProfilePhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureAnimatedProfilePhoto {}

impl PremiumFeatureAnimatedProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureAnimatedProfilePhotoBuilder {
    let inner = PremiumFeatureAnimatedProfilePhoto {
      td_name: "premiumFeatureAnimatedProfilePhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureAnimatedProfilePhotoBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureAnimatedProfilePhotoBuilder {
  inner: PremiumFeatureAnimatedProfilePhoto,
}

impl RTDPremiumFeatureAnimatedProfilePhotoBuilder {
  pub fn build(&self) -> PremiumFeatureAnimatedProfilePhoto {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureAnimatedProfilePhoto> for PremiumFeatureAnimatedProfilePhoto {
  fn as_ref(&self) -> &PremiumFeatureAnimatedProfilePhoto {
    self
  }
}

impl AsRef<PremiumFeatureAnimatedProfilePhoto> for RTDPremiumFeatureAnimatedProfilePhotoBuilder {
  fn as_ref(&self) -> &PremiumFeatureAnimatedProfilePhoto {
    &self.inner
  }
}

/// Allowed to set premium application icons
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureAppIcons {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureAppIcons {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureAppIcons"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureAppIcons {}

impl PremiumFeatureAppIcons {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureAppIconsBuilder {
    let inner = PremiumFeatureAppIcons {
      td_name: "premiumFeatureAppIcons".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureAppIconsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureAppIconsBuilder {
  inner: PremiumFeatureAppIcons,
}

impl RTDPremiumFeatureAppIconsBuilder {
  pub fn build(&self) -> PremiumFeatureAppIcons {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureAppIcons> for PremiumFeatureAppIcons {
  fn as_ref(&self) -> &PremiumFeatureAppIcons {
    self
  }
}

impl AsRef<PremiumFeatureAppIcons> for RTDPremiumFeatureAppIconsBuilder {
  fn as_ref(&self) -> &PremiumFeatureAppIcons {
    &self.inner
  }
}

/// The ability to set private chat background for both users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureBackgroundForBoth {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureBackgroundForBoth {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureBackgroundForBoth"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureBackgroundForBoth {}

impl PremiumFeatureBackgroundForBoth {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureBackgroundForBothBuilder {
    let inner = PremiumFeatureBackgroundForBoth {
      td_name: "premiumFeatureBackgroundForBoth".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureBackgroundForBothBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureBackgroundForBothBuilder {
  inner: PremiumFeatureBackgroundForBoth,
}

impl RTDPremiumFeatureBackgroundForBothBuilder {
  pub fn build(&self) -> PremiumFeatureBackgroundForBoth {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureBackgroundForBoth> for PremiumFeatureBackgroundForBoth {
  fn as_ref(&self) -> &PremiumFeatureBackgroundForBoth {
    self
  }
}

impl AsRef<PremiumFeatureBackgroundForBoth> for RTDPremiumFeatureBackgroundForBothBuilder {
  fn as_ref(&self) -> &PremiumFeatureBackgroundForBoth {
    &self.inner
  }
}

/// The ability to use Business features
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureBusiness {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureBusiness {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureBusiness"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureBusiness {}

impl PremiumFeatureBusiness {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureBusinessBuilder {
    let inner = PremiumFeatureBusiness {
      td_name: "premiumFeatureBusiness".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureBusinessBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureBusinessBuilder {
  inner: PremiumFeatureBusiness,
}

impl RTDPremiumFeatureBusinessBuilder {
  pub fn build(&self) -> PremiumFeatureBusiness {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureBusiness> for PremiumFeatureBusiness {
  fn as_ref(&self) -> &PremiumFeatureBusiness {
    self
  }
}

impl AsRef<PremiumFeatureBusiness> for RTDPremiumFeatureBusinessBuilder {
  fn as_ref(&self) -> &PremiumFeatureBusiness {
    &self.inner
  }
}

/// The ability to boost chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureChatBoost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureChatBoost {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureChatBoost"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureChatBoost {}

impl PremiumFeatureChatBoost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureChatBoostBuilder {
    let inner = PremiumFeatureChatBoost {
      td_name: "premiumFeatureChatBoost".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureChatBoostBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureChatBoostBuilder {
  inner: PremiumFeatureChatBoost,
}

impl RTDPremiumFeatureChatBoostBuilder {
  pub fn build(&self) -> PremiumFeatureChatBoost {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureChatBoost> for PremiumFeatureChatBoost {
  fn as_ref(&self) -> &PremiumFeatureChatBoost {
    self
  }
}

impl AsRef<PremiumFeatureChatBoost> for RTDPremiumFeatureChatBoostBuilder {
  fn as_ref(&self) -> &PremiumFeatureChatBoost {
    &self.inner
  }
}

/// The ability to create and use checklist messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureChecklists {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureChecklists {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureChecklists"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureChecklists {}

impl PremiumFeatureChecklists {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureChecklistsBuilder {
    let inner = PremiumFeatureChecklists {
      td_name: "premiumFeatureChecklists".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureChecklistsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureChecklistsBuilder {
  inner: PremiumFeatureChecklists,
}

impl RTDPremiumFeatureChecklistsBuilder {
  pub fn build(&self) -> PremiumFeatureChecklists {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureChecklists> for PremiumFeatureChecklists {
  fn as_ref(&self) -> &PremiumFeatureChecklists {
    self
  }
}

impl AsRef<PremiumFeatureChecklists> for RTDPremiumFeatureChecklistsBuilder {
  fn as_ref(&self) -> &PremiumFeatureChecklists {
    &self.inner
  }
}

/// Allowed to use custom emoji stickers in message texts and captions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureCustomEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureCustomEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureCustomEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureCustomEmoji {}

impl PremiumFeatureCustomEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureCustomEmojiBuilder {
    let inner = PremiumFeatureCustomEmoji {
      td_name: "premiumFeatureCustomEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureCustomEmojiBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureCustomEmojiBuilder {
  inner: PremiumFeatureCustomEmoji,
}

impl RTDPremiumFeatureCustomEmojiBuilder {
  pub fn build(&self) -> PremiumFeatureCustomEmoji {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureCustomEmoji> for PremiumFeatureCustomEmoji {
  fn as_ref(&self) -> &PremiumFeatureCustomEmoji {
    self
  }
}

impl AsRef<PremiumFeatureCustomEmoji> for RTDPremiumFeatureCustomEmojiBuilder {
  fn as_ref(&self) -> &PremiumFeatureCustomEmoji {
    &self.inner
  }
}

/// Disabled ads
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureDisabledAds {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureDisabledAds {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureDisabledAds"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureDisabledAds {}

impl PremiumFeatureDisabledAds {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureDisabledAdsBuilder {
    let inner = PremiumFeatureDisabledAds {
      td_name: "premiumFeatureDisabledAds".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureDisabledAdsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureDisabledAdsBuilder {
  inner: PremiumFeatureDisabledAds,
}

impl RTDPremiumFeatureDisabledAdsBuilder {
  pub fn build(&self) -> PremiumFeatureDisabledAds {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureDisabledAds> for PremiumFeatureDisabledAds {
  fn as_ref(&self) -> &PremiumFeatureDisabledAds {
    self
  }
}

impl AsRef<PremiumFeatureDisabledAds> for RTDPremiumFeatureDisabledAdsBuilder {
  fn as_ref(&self) -> &PremiumFeatureDisabledAds {
    &self.inner
  }
}

/// The ability to show an emoji status along with the user's name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureEmojiStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureEmojiStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureEmojiStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureEmojiStatus {}

impl PremiumFeatureEmojiStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureEmojiStatusBuilder {
    let inner = PremiumFeatureEmojiStatus {
      td_name: "premiumFeatureEmojiStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureEmojiStatusBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureEmojiStatusBuilder {
  inner: PremiumFeatureEmojiStatus,
}

impl RTDPremiumFeatureEmojiStatusBuilder {
  pub fn build(&self) -> PremiumFeatureEmojiStatus {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureEmojiStatus> for PremiumFeatureEmojiStatus {
  fn as_ref(&self) -> &PremiumFeatureEmojiStatus {
    self
  }
}

impl AsRef<PremiumFeatureEmojiStatus> for RTDPremiumFeatureEmojiStatusBuilder {
  fn as_ref(&self) -> &PremiumFeatureEmojiStatus {
    &self.inner
  }
}

/// The ability to set a custom emoji as a forum topic icon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureForumTopicIcon {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureForumTopicIcon {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureForumTopicIcon"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureForumTopicIcon {}

impl PremiumFeatureForumTopicIcon {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureForumTopicIconBuilder {
    let inner = PremiumFeatureForumTopicIcon {
      td_name: "premiumFeatureForumTopicIcon".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureForumTopicIconBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureForumTopicIconBuilder {
  inner: PremiumFeatureForumTopicIcon,
}

impl RTDPremiumFeatureForumTopicIconBuilder {
  pub fn build(&self) -> PremiumFeatureForumTopicIcon {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureForumTopicIcon> for PremiumFeatureForumTopicIcon {
  fn as_ref(&self) -> &PremiumFeatureForumTopicIcon {
    self
  }
}

impl AsRef<PremiumFeatureForumTopicIcon> for RTDPremiumFeatureForumTopicIconBuilder {
  fn as_ref(&self) -> &PremiumFeatureForumTopicIcon {
    &self.inner
  }
}

/// Improved download speed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureImprovedDownloadSpeed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureImprovedDownloadSpeed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureImprovedDownloadSpeed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureImprovedDownloadSpeed {}

impl PremiumFeatureImprovedDownloadSpeed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureImprovedDownloadSpeedBuilder {
    let inner = PremiumFeatureImprovedDownloadSpeed {
      td_name: "premiumFeatureImprovedDownloadSpeed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureImprovedDownloadSpeedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureImprovedDownloadSpeedBuilder {
  inner: PremiumFeatureImprovedDownloadSpeed,
}

impl RTDPremiumFeatureImprovedDownloadSpeedBuilder {
  pub fn build(&self) -> PremiumFeatureImprovedDownloadSpeed {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureImprovedDownloadSpeed> for PremiumFeatureImprovedDownloadSpeed {
  fn as_ref(&self) -> &PremiumFeatureImprovedDownloadSpeed {
    self
  }
}

impl AsRef<PremiumFeatureImprovedDownloadSpeed> for RTDPremiumFeatureImprovedDownloadSpeedBuilder {
  fn as_ref(&self) -> &PremiumFeatureImprovedDownloadSpeed {
    &self.inner
  }
}

/// Increased limits
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureIncreasedLimits {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureIncreasedLimits {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureIncreasedLimits"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureIncreasedLimits {}

impl PremiumFeatureIncreasedLimits {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureIncreasedLimitsBuilder {
    let inner = PremiumFeatureIncreasedLimits {
      td_name: "premiumFeatureIncreasedLimits".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureIncreasedLimitsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureIncreasedLimitsBuilder {
  inner: PremiumFeatureIncreasedLimits,
}

impl RTDPremiumFeatureIncreasedLimitsBuilder {
  pub fn build(&self) -> PremiumFeatureIncreasedLimits {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureIncreasedLimits> for PremiumFeatureIncreasedLimits {
  fn as_ref(&self) -> &PremiumFeatureIncreasedLimits {
    self
  }
}

impl AsRef<PremiumFeatureIncreasedLimits> for RTDPremiumFeatureIncreasedLimitsBuilder {
  fn as_ref(&self) -> &PremiumFeatureIncreasedLimits {
    &self.inner
  }
}

/// Increased maximum upload file size
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureIncreasedUploadFileSize {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureIncreasedUploadFileSize {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureIncreasedUploadFileSize"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureIncreasedUploadFileSize {}

impl PremiumFeatureIncreasedUploadFileSize {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureIncreasedUploadFileSizeBuilder {
    let inner = PremiumFeatureIncreasedUploadFileSize {
      td_name: "premiumFeatureIncreasedUploadFileSize".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureIncreasedUploadFileSizeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureIncreasedUploadFileSizeBuilder {
  inner: PremiumFeatureIncreasedUploadFileSize,
}

impl RTDPremiumFeatureIncreasedUploadFileSizeBuilder {
  pub fn build(&self) -> PremiumFeatureIncreasedUploadFileSize {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureIncreasedUploadFileSize> for PremiumFeatureIncreasedUploadFileSize {
  fn as_ref(&self) -> &PremiumFeatureIncreasedUploadFileSize {
    self
  }
}

impl AsRef<PremiumFeatureIncreasedUploadFileSize>
  for RTDPremiumFeatureIncreasedUploadFileSizeBuilder
{
  fn as_ref(&self) -> &PremiumFeatureIncreasedUploadFileSize {
    &self.inner
  }
}

/// The ability to view last seen and read times of other users even if they can't view last seen or read time for the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureLastSeenTimes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureLastSeenTimes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureLastSeenTimes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureLastSeenTimes {}

impl PremiumFeatureLastSeenTimes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureLastSeenTimesBuilder {
    let inner = PremiumFeatureLastSeenTimes {
      td_name: "premiumFeatureLastSeenTimes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureLastSeenTimesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureLastSeenTimesBuilder {
  inner: PremiumFeatureLastSeenTimes,
}

impl RTDPremiumFeatureLastSeenTimesBuilder {
  pub fn build(&self) -> PremiumFeatureLastSeenTimes {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureLastSeenTimes> for PremiumFeatureLastSeenTimes {
  fn as_ref(&self) -> &PremiumFeatureLastSeenTimes {
    self
  }
}

impl AsRef<PremiumFeatureLastSeenTimes> for RTDPremiumFeatureLastSeenTimesBuilder {
  fn as_ref(&self) -> &PremiumFeatureLastSeenTimes {
    &self.inner
  }
}

/// The ability to use all available message effects
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureMessageEffects {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureMessageEffects {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureMessageEffects"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureMessageEffects {}

impl PremiumFeatureMessageEffects {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureMessageEffectsBuilder {
    let inner = PremiumFeatureMessageEffects {
      td_name: "premiumFeatureMessageEffects".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureMessageEffectsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureMessageEffectsBuilder {
  inner: PremiumFeatureMessageEffects,
}

impl RTDPremiumFeatureMessageEffectsBuilder {
  pub fn build(&self) -> PremiumFeatureMessageEffects {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureMessageEffects> for PremiumFeatureMessageEffects {
  fn as_ref(&self) -> &PremiumFeatureMessageEffects {
    self
  }
}

impl AsRef<PremiumFeatureMessageEffects> for RTDPremiumFeatureMessageEffectsBuilder {
  fn as_ref(&self) -> &PremiumFeatureMessageEffects {
    &self.inner
  }
}

/// The ability to disallow incoming voice and video note messages in private chats using setUserPrivacySettingRules with userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages and to restrict incoming messages from non-contacts using setNewChatPrivacySettings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureMessagePrivacy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureMessagePrivacy {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureMessagePrivacy"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureMessagePrivacy {}

impl PremiumFeatureMessagePrivacy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureMessagePrivacyBuilder {
    let inner = PremiumFeatureMessagePrivacy {
      td_name: "premiumFeatureMessagePrivacy".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureMessagePrivacyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureMessagePrivacyBuilder {
  inner: PremiumFeatureMessagePrivacy,
}

impl RTDPremiumFeatureMessagePrivacyBuilder {
  pub fn build(&self) -> PremiumFeatureMessagePrivacy {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureMessagePrivacy> for PremiumFeatureMessagePrivacy {
  fn as_ref(&self) -> &PremiumFeatureMessagePrivacy {
    self
  }
}

impl AsRef<PremiumFeatureMessagePrivacy> for RTDPremiumFeatureMessagePrivacyBuilder {
  fn as_ref(&self) -> &PremiumFeatureMessagePrivacy {
    &self.inner
  }
}

/// The ability to require a payment for incoming messages in new chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeaturePaidMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeaturePaidMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeaturePaidMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeaturePaidMessages {}

impl PremiumFeaturePaidMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeaturePaidMessagesBuilder {
    let inner = PremiumFeaturePaidMessages {
      td_name: "premiumFeaturePaidMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeaturePaidMessagesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeaturePaidMessagesBuilder {
  inner: PremiumFeaturePaidMessages,
}

impl RTDPremiumFeaturePaidMessagesBuilder {
  pub fn build(&self) -> PremiumFeaturePaidMessages {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeaturePaidMessages> for PremiumFeaturePaidMessages {
  fn as_ref(&self) -> &PremiumFeaturePaidMessages {
    self
  }
}

impl AsRef<PremiumFeaturePaidMessages> for RTDPremiumFeaturePaidMessagesBuilder {
  fn as_ref(&self) -> &PremiumFeaturePaidMessages {
    &self.inner
  }
}

/// A badge in the user's profile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureProfileBadge {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureProfileBadge {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureProfileBadge"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureProfileBadge {}

impl PremiumFeatureProfileBadge {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureProfileBadgeBuilder {
    let inner = PremiumFeatureProfileBadge {
      td_name: "premiumFeatureProfileBadge".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureProfileBadgeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureProfileBadgeBuilder {
  inner: PremiumFeatureProfileBadge,
}

impl RTDPremiumFeatureProfileBadgeBuilder {
  pub fn build(&self) -> PremiumFeatureProfileBadge {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureProfileBadge> for PremiumFeatureProfileBadge {
  fn as_ref(&self) -> &PremiumFeatureProfileBadge {
    self
  }
}

impl AsRef<PremiumFeatureProfileBadge> for RTDPremiumFeatureProfileBadgeBuilder {
  fn as_ref(&self) -> &PremiumFeatureProfileBadge {
    &self.inner
  }
}

/// The ability to enable content protection in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureProtectPrivateChatContent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureProtectPrivateChatContent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureProtectPrivateChatContent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureProtectPrivateChatContent {}

impl PremiumFeatureProtectPrivateChatContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureProtectPrivateChatContentBuilder {
    let inner = PremiumFeatureProtectPrivateChatContent {
      td_name: "premiumFeatureProtectPrivateChatContent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureProtectPrivateChatContentBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureProtectPrivateChatContentBuilder {
  inner: PremiumFeatureProtectPrivateChatContent,
}

impl RTDPremiumFeatureProtectPrivateChatContentBuilder {
  pub fn build(&self) -> PremiumFeatureProtectPrivateChatContent {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureProtectPrivateChatContent> for PremiumFeatureProtectPrivateChatContent {
  fn as_ref(&self) -> &PremiumFeatureProtectPrivateChatContent {
    self
  }
}

impl AsRef<PremiumFeatureProtectPrivateChatContent>
  for RTDPremiumFeatureProtectPrivateChatContentBuilder
{
  fn as_ref(&self) -> &PremiumFeatureProtectPrivateChatContent {
    &self.inner
  }
}

/// Allowed to translate chat messages real-time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureRealTimeChatTranslation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureRealTimeChatTranslation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureRealTimeChatTranslation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureRealTimeChatTranslation {}

impl PremiumFeatureRealTimeChatTranslation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureRealTimeChatTranslationBuilder {
    let inner = PremiumFeatureRealTimeChatTranslation {
      td_name: "premiumFeatureRealTimeChatTranslation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureRealTimeChatTranslationBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureRealTimeChatTranslationBuilder {
  inner: PremiumFeatureRealTimeChatTranslation,
}

impl RTDPremiumFeatureRealTimeChatTranslationBuilder {
  pub fn build(&self) -> PremiumFeatureRealTimeChatTranslation {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureRealTimeChatTranslation> for PremiumFeatureRealTimeChatTranslation {
  fn as_ref(&self) -> &PremiumFeatureRealTimeChatTranslation {
    self
  }
}

impl AsRef<PremiumFeatureRealTimeChatTranslation>
  for RTDPremiumFeatureRealTimeChatTranslationBuilder
{
  fn as_ref(&self) -> &PremiumFeatureRealTimeChatTranslation {
    &self.inner
  }
}

/// The ability to send rich messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureRichMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureRichMessages {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureRichMessages"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureRichMessages {}

impl PremiumFeatureRichMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureRichMessagesBuilder {
    let inner = PremiumFeatureRichMessages {
      td_name: "premiumFeatureRichMessages".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureRichMessagesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureRichMessagesBuilder {
  inner: PremiumFeatureRichMessages,
}

impl RTDPremiumFeatureRichMessagesBuilder {
  pub fn build(&self) -> PremiumFeatureRichMessages {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureRichMessages> for PremiumFeatureRichMessages {
  fn as_ref(&self) -> &PremiumFeatureRichMessages {
    self
  }
}

impl AsRef<PremiumFeatureRichMessages> for RTDPremiumFeatureRichMessagesBuilder {
  fn as_ref(&self) -> &PremiumFeatureRichMessages {
    &self.inner
  }
}

/// The ability to use tags in Saved Messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureSavedMessagesTags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureSavedMessagesTags {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureSavedMessagesTags"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureSavedMessagesTags {}

impl PremiumFeatureSavedMessagesTags {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureSavedMessagesTagsBuilder {
    let inner = PremiumFeatureSavedMessagesTags {
      td_name: "premiumFeatureSavedMessagesTags".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureSavedMessagesTagsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureSavedMessagesTagsBuilder {
  inner: PremiumFeatureSavedMessagesTags,
}

impl RTDPremiumFeatureSavedMessagesTagsBuilder {
  pub fn build(&self) -> PremiumFeatureSavedMessagesTags {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureSavedMessagesTags> for PremiumFeatureSavedMessagesTags {
  fn as_ref(&self) -> &PremiumFeatureSavedMessagesTags {
    self
  }
}

impl AsRef<PremiumFeatureSavedMessagesTags> for RTDPremiumFeatureSavedMessagesTagsBuilder {
  fn as_ref(&self) -> &PremiumFeatureSavedMessagesTags {
    &self.inner
  }
}

/// The ability to compose text with AI
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureTextComposition {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureTextComposition {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureTextComposition"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureTextComposition {}

impl PremiumFeatureTextComposition {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureTextCompositionBuilder {
    let inner = PremiumFeatureTextComposition {
      td_name: "premiumFeatureTextComposition".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureTextCompositionBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureTextCompositionBuilder {
  inner: PremiumFeatureTextComposition,
}

impl RTDPremiumFeatureTextCompositionBuilder {
  pub fn build(&self) -> PremiumFeatureTextComposition {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureTextComposition> for PremiumFeatureTextComposition {
  fn as_ref(&self) -> &PremiumFeatureTextComposition {
    self
  }
}

impl AsRef<PremiumFeatureTextComposition> for RTDPremiumFeatureTextCompositionBuilder {
  fn as_ref(&self) -> &PremiumFeatureTextComposition {
    &self.inner
  }
}

/// Allowed to use more reactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureUniqueReactions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureUniqueReactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureUniqueReactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureUniqueReactions {}

impl PremiumFeatureUniqueReactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureUniqueReactionsBuilder {
    let inner = PremiumFeatureUniqueReactions {
      td_name: "premiumFeatureUniqueReactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureUniqueReactionsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureUniqueReactionsBuilder {
  inner: PremiumFeatureUniqueReactions,
}

impl RTDPremiumFeatureUniqueReactionsBuilder {
  pub fn build(&self) -> PremiumFeatureUniqueReactions {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureUniqueReactions> for PremiumFeatureUniqueReactions {
  fn as_ref(&self) -> &PremiumFeatureUniqueReactions {
    self
  }
}

impl AsRef<PremiumFeatureUniqueReactions> for RTDPremiumFeatureUniqueReactionsBuilder {
  fn as_ref(&self) -> &PremiumFeatureUniqueReactions {
    &self.inner
  }
}

/// Allowed to use premium stickers with unique effects
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureUniqueStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureUniqueStickers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureUniqueStickers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureUniqueStickers {}

impl PremiumFeatureUniqueStickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureUniqueStickersBuilder {
    let inner = PremiumFeatureUniqueStickers {
      td_name: "premiumFeatureUniqueStickers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureUniqueStickersBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureUniqueStickersBuilder {
  inner: PremiumFeatureUniqueStickers,
}

impl RTDPremiumFeatureUniqueStickersBuilder {
  pub fn build(&self) -> PremiumFeatureUniqueStickers {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureUniqueStickers> for PremiumFeatureUniqueStickers {
  fn as_ref(&self) -> &PremiumFeatureUniqueStickers {
    self
  }
}

impl AsRef<PremiumFeatureUniqueStickers> for RTDPremiumFeatureUniqueStickersBuilder {
  fn as_ref(&self) -> &PremiumFeatureUniqueStickers {
    &self.inner
  }
}

/// Allowed to use many additional features for stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureUpgradedStories {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureUpgradedStories {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureUpgradedStories"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureUpgradedStories {}

impl PremiumFeatureUpgradedStories {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureUpgradedStoriesBuilder {
    let inner = PremiumFeatureUpgradedStories {
      td_name: "premiumFeatureUpgradedStories".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureUpgradedStoriesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureUpgradedStoriesBuilder {
  inner: PremiumFeatureUpgradedStories,
}

impl RTDPremiumFeatureUpgradedStoriesBuilder {
  pub fn build(&self) -> PremiumFeatureUpgradedStories {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureUpgradedStories> for PremiumFeatureUpgradedStories {
  fn as_ref(&self) -> &PremiumFeatureUpgradedStories {
    self
  }
}

impl AsRef<PremiumFeatureUpgradedStories> for RTDPremiumFeatureUpgradedStoriesBuilder {
  fn as_ref(&self) -> &PremiumFeatureUpgradedStories {
    &self.inner
  }
}

/// The ability to convert voice notes to text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureVoiceRecognition {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumFeatureVoiceRecognition {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumFeatureVoiceRecognition"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumFeature for PremiumFeatureVoiceRecognition {}

impl PremiumFeatureVoiceRecognition {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumFeatureVoiceRecognitionBuilder {
    let inner = PremiumFeatureVoiceRecognition {
      td_name: "premiumFeatureVoiceRecognition".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumFeatureVoiceRecognitionBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureVoiceRecognitionBuilder {
  inner: PremiumFeatureVoiceRecognition,
}

impl RTDPremiumFeatureVoiceRecognitionBuilder {
  pub fn build(&self) -> PremiumFeatureVoiceRecognition {
    self.inner.clone()
  }
}

impl AsRef<PremiumFeatureVoiceRecognition> for PremiumFeatureVoiceRecognition {
  fn as_ref(&self) -> &PremiumFeatureVoiceRecognition {
    self
  }
}

impl AsRef<PremiumFeatureVoiceRecognition> for RTDPremiumFeatureVoiceRecognitionBuilder {
  fn as_ref(&self) -> &PremiumFeatureVoiceRecognition {
    &self.inner
  }
}
