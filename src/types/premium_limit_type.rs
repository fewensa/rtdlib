use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of limit, increased for Premium users
pub trait TDPremiumLimitType: Debug + RObject {}

/// Describes type of limit, increased for Premium users
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PremiumLimitType {
  #[doc(hidden)]
  _Default(()),
  /// The maximum number of active stories
  ActiveStoryCount(PremiumLimitTypeActiveStoryCount),
  /// The maximum length of the user's bio
  BioLength(PremiumLimitTypeBioLength),
  /// The maximum length of sent media caption
  CaptionLength(PremiumLimitTypeCaptionLength),
  /// The maximum number of pinned and always included, or always excluded chats in a chat folder
  ChatFolderChosenChatCount(PremiumLimitTypeChatFolderChosenChatCount),
  /// The maximum number of chat folders
  ChatFolderCount(PremiumLimitTypeChatFolderCount),
  /// The maximum number of invite links for a chat folder
  ChatFolderInviteLinkCount(PremiumLimitTypeChatFolderInviteLinkCount),
  /// The maximum number of created public chats
  CreatedPublicChatCount(PremiumLimitTypeCreatedPublicChatCount),
  /// The maximum number of added text composition styles
  CustomTextCompositionStyleCount(PremiumLimitTypeCustomTextCompositionStyleCount),
  /// The maximum number of favorite stickers
  FavoriteStickerCount(PremiumLimitTypeFavoriteStickerCount),
  /// The maximum length of text of sent messages
  MessageTextLength(PremiumLimitTypeMessageTextLength),
  /// The maximum number of stories posted per month
  MonthlyPostedStoryCount(PremiumLimitTypeMonthlyPostedStoryCount),
  /// The maximum number of owned bots
  OwnedBotCount(PremiumLimitTypeOwnedBotCount),
  /// The maximum number of pinned chats in the archive chat list
  PinnedArchivedChatCount(PremiumLimitTypePinnedArchivedChatCount),
  /// The maximum number of pinned chats in the main chat list
  PinnedChatCount(PremiumLimitTypePinnedChatCount),
  /// The maximum number of pinned Saved Messages topics
  PinnedSavedMessagesTopicCount(PremiumLimitTypePinnedSavedMessagesTopicCount),
  /// The maximum number of saved animations
  SavedAnimationCount(PremiumLimitTypeSavedAnimationCount),
  /// The maximum number of added shareable chat folders
  ShareableChatFolderCount(PremiumLimitTypeShareableChatFolderCount),
  /// The maximum number of received similar chats
  SimilarChatCount(PremiumLimitTypeSimilarChatCount),
  /// The maximum length of captions of posted stories
  StoryCaptionLength(PremiumLimitTypeStoryCaptionLength),
  /// The maximum number of suggested reaction areas on a story
  StorySuggestedReactionAreaCount(PremiumLimitTypeStorySuggestedReactionAreaCount),
  /// The maximum number of joined supergroups and channels
  SupergroupCount(PremiumLimitTypeSupergroupCount),
  /// The maximum number of stories posted per week
  WeeklyPostedStoryCount(PremiumLimitTypeWeeklyPostedStoryCount),
}

impl Default for PremiumLimitType {
  fn default() -> Self {
    PremiumLimitType::_Default(())
  }
}

impl<'de> Deserialize<'de> for PremiumLimitType {
  fn deserialize<D>(deserializer: D) -> Result<PremiumLimitType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PremiumLimitType,
      (premiumLimitTypeActiveStoryCount, ActiveStoryCount);
      (premiumLimitTypeBioLength, BioLength);
      (premiumLimitTypeCaptionLength, CaptionLength);
      (premiumLimitTypeChatFolderChosenChatCount, ChatFolderChosenChatCount);
      (premiumLimitTypeChatFolderCount, ChatFolderCount);
      (premiumLimitTypeChatFolderInviteLinkCount, ChatFolderInviteLinkCount);
      (premiumLimitTypeCreatedPublicChatCount, CreatedPublicChatCount);
      (premiumLimitTypeCustomTextCompositionStyleCount, CustomTextCompositionStyleCount);
      (premiumLimitTypeFavoriteStickerCount, FavoriteStickerCount);
      (premiumLimitTypeMessageTextLength, MessageTextLength);
      (premiumLimitTypeMonthlyPostedStoryCount, MonthlyPostedStoryCount);
      (premiumLimitTypeOwnedBotCount, OwnedBotCount);
      (premiumLimitTypePinnedArchivedChatCount, PinnedArchivedChatCount);
      (premiumLimitTypePinnedChatCount, PinnedChatCount);
      (premiumLimitTypePinnedSavedMessagesTopicCount, PinnedSavedMessagesTopicCount);
      (premiumLimitTypeSavedAnimationCount, SavedAnimationCount);
      (premiumLimitTypeShareableChatFolderCount, ShareableChatFolderCount);
      (premiumLimitTypeSimilarChatCount, SimilarChatCount);
      (premiumLimitTypeStoryCaptionLength, StoryCaptionLength);
      (premiumLimitTypeStorySuggestedReactionAreaCount, StorySuggestedReactionAreaCount);
      (premiumLimitTypeSupergroupCount, SupergroupCount);
      (premiumLimitTypeWeeklyPostedStoryCount, WeeklyPostedStoryCount);

    )(deserializer)
  }
}

impl RObject for PremiumLimitType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PremiumLimitType::ActiveStoryCount(t) => t.td_name(),
      PremiumLimitType::BioLength(t) => t.td_name(),
      PremiumLimitType::CaptionLength(t) => t.td_name(),
      PremiumLimitType::ChatFolderChosenChatCount(t) => t.td_name(),
      PremiumLimitType::ChatFolderCount(t) => t.td_name(),
      PremiumLimitType::ChatFolderInviteLinkCount(t) => t.td_name(),
      PremiumLimitType::CreatedPublicChatCount(t) => t.td_name(),
      PremiumLimitType::CustomTextCompositionStyleCount(t) => t.td_name(),
      PremiumLimitType::FavoriteStickerCount(t) => t.td_name(),
      PremiumLimitType::MessageTextLength(t) => t.td_name(),
      PremiumLimitType::MonthlyPostedStoryCount(t) => t.td_name(),
      PremiumLimitType::OwnedBotCount(t) => t.td_name(),
      PremiumLimitType::PinnedArchivedChatCount(t) => t.td_name(),
      PremiumLimitType::PinnedChatCount(t) => t.td_name(),
      PremiumLimitType::PinnedSavedMessagesTopicCount(t) => t.td_name(),
      PremiumLimitType::SavedAnimationCount(t) => t.td_name(),
      PremiumLimitType::ShareableChatFolderCount(t) => t.td_name(),
      PremiumLimitType::SimilarChatCount(t) => t.td_name(),
      PremiumLimitType::StoryCaptionLength(t) => t.td_name(),
      PremiumLimitType::StorySuggestedReactionAreaCount(t) => t.td_name(),
      PremiumLimitType::SupergroupCount(t) => t.td_name(),
      PremiumLimitType::WeeklyPostedStoryCount(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PremiumLimitType::ActiveStoryCount(t) => t.extra(),
      PremiumLimitType::BioLength(t) => t.extra(),
      PremiumLimitType::CaptionLength(t) => t.extra(),
      PremiumLimitType::ChatFolderChosenChatCount(t) => t.extra(),
      PremiumLimitType::ChatFolderCount(t) => t.extra(),
      PremiumLimitType::ChatFolderInviteLinkCount(t) => t.extra(),
      PremiumLimitType::CreatedPublicChatCount(t) => t.extra(),
      PremiumLimitType::CustomTextCompositionStyleCount(t) => t.extra(),
      PremiumLimitType::FavoriteStickerCount(t) => t.extra(),
      PremiumLimitType::MessageTextLength(t) => t.extra(),
      PremiumLimitType::MonthlyPostedStoryCount(t) => t.extra(),
      PremiumLimitType::OwnedBotCount(t) => t.extra(),
      PremiumLimitType::PinnedArchivedChatCount(t) => t.extra(),
      PremiumLimitType::PinnedChatCount(t) => t.extra(),
      PremiumLimitType::PinnedSavedMessagesTopicCount(t) => t.extra(),
      PremiumLimitType::SavedAnimationCount(t) => t.extra(),
      PremiumLimitType::ShareableChatFolderCount(t) => t.extra(),
      PremiumLimitType::SimilarChatCount(t) => t.extra(),
      PremiumLimitType::StoryCaptionLength(t) => t.extra(),
      PremiumLimitType::StorySuggestedReactionAreaCount(t) => t.extra(),
      PremiumLimitType::SupergroupCount(t) => t.extra(),
      PremiumLimitType::WeeklyPostedStoryCount(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumLimitType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PremiumLimitType::_Default(_))
  }

  pub fn is_active_story_count(&self) -> bool {
    matches!(self, PremiumLimitType::ActiveStoryCount(_))
  }
  pub fn is_bio_length(&self) -> bool {
    matches!(self, PremiumLimitType::BioLength(_))
  }
  pub fn is_caption_length(&self) -> bool {
    matches!(self, PremiumLimitType::CaptionLength(_))
  }
  pub fn is_chat_folder_chosen_chat_count(&self) -> bool {
    matches!(self, PremiumLimitType::ChatFolderChosenChatCount(_))
  }
  pub fn is_chat_folder_count(&self) -> bool {
    matches!(self, PremiumLimitType::ChatFolderCount(_))
  }
  pub fn is_chat_folder_invite_link_count(&self) -> bool {
    matches!(self, PremiumLimitType::ChatFolderInviteLinkCount(_))
  }
  pub fn is_created_public_chat_count(&self) -> bool {
    matches!(self, PremiumLimitType::CreatedPublicChatCount(_))
  }
  pub fn is_custom_text_composition_style_count(&self) -> bool {
    matches!(self, PremiumLimitType::CustomTextCompositionStyleCount(_))
  }
  pub fn is_favorite_sticker_count(&self) -> bool {
    matches!(self, PremiumLimitType::FavoriteStickerCount(_))
  }
  pub fn is_message_text_length(&self) -> bool {
    matches!(self, PremiumLimitType::MessageTextLength(_))
  }
  pub fn is_monthly_posted_story_count(&self) -> bool {
    matches!(self, PremiumLimitType::MonthlyPostedStoryCount(_))
  }
  pub fn is_owned_bot_count(&self) -> bool {
    matches!(self, PremiumLimitType::OwnedBotCount(_))
  }
  pub fn is_pinned_archived_chat_count(&self) -> bool {
    matches!(self, PremiumLimitType::PinnedArchivedChatCount(_))
  }
  pub fn is_pinned_chat_count(&self) -> bool {
    matches!(self, PremiumLimitType::PinnedChatCount(_))
  }
  pub fn is_pinned_saved_messages_topic_count(&self) -> bool {
    matches!(self, PremiumLimitType::PinnedSavedMessagesTopicCount(_))
  }
  pub fn is_saved_animation_count(&self) -> bool {
    matches!(self, PremiumLimitType::SavedAnimationCount(_))
  }
  pub fn is_shareable_chat_folder_count(&self) -> bool {
    matches!(self, PremiumLimitType::ShareableChatFolderCount(_))
  }
  pub fn is_similar_chat_count(&self) -> bool {
    matches!(self, PremiumLimitType::SimilarChatCount(_))
  }
  pub fn is_story_caption_length(&self) -> bool {
    matches!(self, PremiumLimitType::StoryCaptionLength(_))
  }
  pub fn is_story_suggested_reaction_area_count(&self) -> bool {
    matches!(self, PremiumLimitType::StorySuggestedReactionAreaCount(_))
  }
  pub fn is_supergroup_count(&self) -> bool {
    matches!(self, PremiumLimitType::SupergroupCount(_))
  }
  pub fn is_weekly_posted_story_count(&self) -> bool {
    matches!(self, PremiumLimitType::WeeklyPostedStoryCount(_))
  }

  pub fn on_active_story_count<F: FnOnce(&PremiumLimitTypeActiveStoryCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::ActiveStoryCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bio_length<F: FnOnce(&PremiumLimitTypeBioLength)>(&self, fnc: F) -> &Self {
    if let PremiumLimitType::BioLength(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_caption_length<F: FnOnce(&PremiumLimitTypeCaptionLength)>(&self, fnc: F) -> &Self {
    if let PremiumLimitType::CaptionLength(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_folder_chosen_chat_count<F: FnOnce(&PremiumLimitTypeChatFolderChosenChatCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::ChatFolderChosenChatCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_folder_count<F: FnOnce(&PremiumLimitTypeChatFolderCount)>(&self, fnc: F) -> &Self {
    if let PremiumLimitType::ChatFolderCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chat_folder_invite_link_count<F: FnOnce(&PremiumLimitTypeChatFolderInviteLinkCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::ChatFolderInviteLinkCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_created_public_chat_count<F: FnOnce(&PremiumLimitTypeCreatedPublicChatCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::CreatedPublicChatCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_custom_text_composition_style_count<
    F: FnOnce(&PremiumLimitTypeCustomTextCompositionStyleCount),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::CustomTextCompositionStyleCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_favorite_sticker_count<F: FnOnce(&PremiumLimitTypeFavoriteStickerCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::FavoriteStickerCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_message_text_length<F: FnOnce(&PremiumLimitTypeMessageTextLength)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::MessageTextLength(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_monthly_posted_story_count<F: FnOnce(&PremiumLimitTypeMonthlyPostedStoryCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::MonthlyPostedStoryCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_owned_bot_count<F: FnOnce(&PremiumLimitTypeOwnedBotCount)>(&self, fnc: F) -> &Self {
    if let PremiumLimitType::OwnedBotCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pinned_archived_chat_count<F: FnOnce(&PremiumLimitTypePinnedArchivedChatCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::PinnedArchivedChatCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pinned_chat_count<F: FnOnce(&PremiumLimitTypePinnedChatCount)>(&self, fnc: F) -> &Self {
    if let PremiumLimitType::PinnedChatCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pinned_saved_messages_topic_count<
    F: FnOnce(&PremiumLimitTypePinnedSavedMessagesTopicCount),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::PinnedSavedMessagesTopicCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_animation_count<F: FnOnce(&PremiumLimitTypeSavedAnimationCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::SavedAnimationCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_shareable_chat_folder_count<F: FnOnce(&PremiumLimitTypeShareableChatFolderCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::ShareableChatFolderCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_similar_chat_count<F: FnOnce(&PremiumLimitTypeSimilarChatCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::SimilarChatCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_caption_length<F: FnOnce(&PremiumLimitTypeStoryCaptionLength)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::StoryCaptionLength(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_suggested_reaction_area_count<
    F: FnOnce(&PremiumLimitTypeStorySuggestedReactionAreaCount),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::StorySuggestedReactionAreaCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_supergroup_count<F: FnOnce(&PremiumLimitTypeSupergroupCount)>(&self, fnc: F) -> &Self {
    if let PremiumLimitType::SupergroupCount(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_weekly_posted_story_count<F: FnOnce(&PremiumLimitTypeWeeklyPostedStoryCount)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumLimitType::WeeklyPostedStoryCount(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_active_story_count(&self) -> Option<&PremiumLimitTypeActiveStoryCount> {
    if let PremiumLimitType::ActiveStoryCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bio_length(&self) -> Option<&PremiumLimitTypeBioLength> {
    if let PremiumLimitType::BioLength(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_caption_length(&self) -> Option<&PremiumLimitTypeCaptionLength> {
    if let PremiumLimitType::CaptionLength(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_folder_chosen_chat_count(
    &self,
  ) -> Option<&PremiumLimitTypeChatFolderChosenChatCount> {
    if let PremiumLimitType::ChatFolderChosenChatCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_folder_count(&self) -> Option<&PremiumLimitTypeChatFolderCount> {
    if let PremiumLimitType::ChatFolderCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chat_folder_invite_link_count(
    &self,
  ) -> Option<&PremiumLimitTypeChatFolderInviteLinkCount> {
    if let PremiumLimitType::ChatFolderInviteLinkCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_created_public_chat_count(&self) -> Option<&PremiumLimitTypeCreatedPublicChatCount> {
    if let PremiumLimitType::CreatedPublicChatCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_custom_text_composition_style_count(
    &self,
  ) -> Option<&PremiumLimitTypeCustomTextCompositionStyleCount> {
    if let PremiumLimitType::CustomTextCompositionStyleCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_favorite_sticker_count(&self) -> Option<&PremiumLimitTypeFavoriteStickerCount> {
    if let PremiumLimitType::FavoriteStickerCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_message_text_length(&self) -> Option<&PremiumLimitTypeMessageTextLength> {
    if let PremiumLimitType::MessageTextLength(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_monthly_posted_story_count(&self) -> Option<&PremiumLimitTypeMonthlyPostedStoryCount> {
    if let PremiumLimitType::MonthlyPostedStoryCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_owned_bot_count(&self) -> Option<&PremiumLimitTypeOwnedBotCount> {
    if let PremiumLimitType::OwnedBotCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pinned_archived_chat_count(&self) -> Option<&PremiumLimitTypePinnedArchivedChatCount> {
    if let PremiumLimitType::PinnedArchivedChatCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pinned_chat_count(&self) -> Option<&PremiumLimitTypePinnedChatCount> {
    if let PremiumLimitType::PinnedChatCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pinned_saved_messages_topic_count(
    &self,
  ) -> Option<&PremiumLimitTypePinnedSavedMessagesTopicCount> {
    if let PremiumLimitType::PinnedSavedMessagesTopicCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_animation_count(&self) -> Option<&PremiumLimitTypeSavedAnimationCount> {
    if let PremiumLimitType::SavedAnimationCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_shareable_chat_folder_count(
    &self,
  ) -> Option<&PremiumLimitTypeShareableChatFolderCount> {
    if let PremiumLimitType::ShareableChatFolderCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_similar_chat_count(&self) -> Option<&PremiumLimitTypeSimilarChatCount> {
    if let PremiumLimitType::SimilarChatCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_caption_length(&self) -> Option<&PremiumLimitTypeStoryCaptionLength> {
    if let PremiumLimitType::StoryCaptionLength(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_suggested_reaction_area_count(
    &self,
  ) -> Option<&PremiumLimitTypeStorySuggestedReactionAreaCount> {
    if let PremiumLimitType::StorySuggestedReactionAreaCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_supergroup_count(&self) -> Option<&PremiumLimitTypeSupergroupCount> {
    if let PremiumLimitType::SupergroupCount(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_weekly_posted_story_count(&self) -> Option<&PremiumLimitTypeWeeklyPostedStoryCount> {
    if let PremiumLimitType::WeeklyPostedStoryCount(t) = self {
      return Some(t);
    }
    None
  }

  pub fn active_story_count<T: AsRef<PremiumLimitTypeActiveStoryCount>>(t: T) -> Self {
    PremiumLimitType::ActiveStoryCount(t.as_ref().clone())
  }

  pub fn bio_length<T: AsRef<PremiumLimitTypeBioLength>>(t: T) -> Self {
    PremiumLimitType::BioLength(t.as_ref().clone())
  }

  pub fn caption_length<T: AsRef<PremiumLimitTypeCaptionLength>>(t: T) -> Self {
    PremiumLimitType::CaptionLength(t.as_ref().clone())
  }

  pub fn chat_folder_chosen_chat_count<T: AsRef<PremiumLimitTypeChatFolderChosenChatCount>>(
    t: T,
  ) -> Self {
    PremiumLimitType::ChatFolderChosenChatCount(t.as_ref().clone())
  }

  pub fn chat_folder_count<T: AsRef<PremiumLimitTypeChatFolderCount>>(t: T) -> Self {
    PremiumLimitType::ChatFolderCount(t.as_ref().clone())
  }

  pub fn chat_folder_invite_link_count<T: AsRef<PremiumLimitTypeChatFolderInviteLinkCount>>(
    t: T,
  ) -> Self {
    PremiumLimitType::ChatFolderInviteLinkCount(t.as_ref().clone())
  }

  pub fn created_public_chat_count<T: AsRef<PremiumLimitTypeCreatedPublicChatCount>>(t: T) -> Self {
    PremiumLimitType::CreatedPublicChatCount(t.as_ref().clone())
  }

  pub fn custom_text_composition_style_count<
    T: AsRef<PremiumLimitTypeCustomTextCompositionStyleCount>,
  >(
    t: T,
  ) -> Self {
    PremiumLimitType::CustomTextCompositionStyleCount(t.as_ref().clone())
  }

  pub fn favorite_sticker_count<T: AsRef<PremiumLimitTypeFavoriteStickerCount>>(t: T) -> Self {
    PremiumLimitType::FavoriteStickerCount(t.as_ref().clone())
  }

  pub fn message_text_length<T: AsRef<PremiumLimitTypeMessageTextLength>>(t: T) -> Self {
    PremiumLimitType::MessageTextLength(t.as_ref().clone())
  }

  pub fn monthly_posted_story_count<T: AsRef<PremiumLimitTypeMonthlyPostedStoryCount>>(
    t: T,
  ) -> Self {
    PremiumLimitType::MonthlyPostedStoryCount(t.as_ref().clone())
  }

  pub fn owned_bot_count<T: AsRef<PremiumLimitTypeOwnedBotCount>>(t: T) -> Self {
    PremiumLimitType::OwnedBotCount(t.as_ref().clone())
  }

  pub fn pinned_archived_chat_count<T: AsRef<PremiumLimitTypePinnedArchivedChatCount>>(
    t: T,
  ) -> Self {
    PremiumLimitType::PinnedArchivedChatCount(t.as_ref().clone())
  }

  pub fn pinned_chat_count<T: AsRef<PremiumLimitTypePinnedChatCount>>(t: T) -> Self {
    PremiumLimitType::PinnedChatCount(t.as_ref().clone())
  }

  pub fn pinned_saved_messages_topic_count<
    T: AsRef<PremiumLimitTypePinnedSavedMessagesTopicCount>,
  >(
    t: T,
  ) -> Self {
    PremiumLimitType::PinnedSavedMessagesTopicCount(t.as_ref().clone())
  }

  pub fn saved_animation_count<T: AsRef<PremiumLimitTypeSavedAnimationCount>>(t: T) -> Self {
    PremiumLimitType::SavedAnimationCount(t.as_ref().clone())
  }

  pub fn shareable_chat_folder_count<T: AsRef<PremiumLimitTypeShareableChatFolderCount>>(
    t: T,
  ) -> Self {
    PremiumLimitType::ShareableChatFolderCount(t.as_ref().clone())
  }

  pub fn similar_chat_count<T: AsRef<PremiumLimitTypeSimilarChatCount>>(t: T) -> Self {
    PremiumLimitType::SimilarChatCount(t.as_ref().clone())
  }

  pub fn story_caption_length<T: AsRef<PremiumLimitTypeStoryCaptionLength>>(t: T) -> Self {
    PremiumLimitType::StoryCaptionLength(t.as_ref().clone())
  }

  pub fn story_suggested_reaction_area_count<
    T: AsRef<PremiumLimitTypeStorySuggestedReactionAreaCount>,
  >(
    t: T,
  ) -> Self {
    PremiumLimitType::StorySuggestedReactionAreaCount(t.as_ref().clone())
  }

  pub fn supergroup_count<T: AsRef<PremiumLimitTypeSupergroupCount>>(t: T) -> Self {
    PremiumLimitType::SupergroupCount(t.as_ref().clone())
  }

  pub fn weekly_posted_story_count<T: AsRef<PremiumLimitTypeWeeklyPostedStoryCount>>(t: T) -> Self {
    PremiumLimitType::WeeklyPostedStoryCount(t.as_ref().clone())
  }
}

impl AsRef<PremiumLimitType> for PremiumLimitType {
  fn as_ref(&self) -> &PremiumLimitType {
    self
  }
}

/// The maximum number of active stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeActiveStoryCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeActiveStoryCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeActiveStoryCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeActiveStoryCount {}

impl PremiumLimitTypeActiveStoryCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeActiveStoryCountBuilder {
    let inner = PremiumLimitTypeActiveStoryCount {
      td_name: "premiumLimitTypeActiveStoryCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeActiveStoryCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeActiveStoryCountBuilder {
  inner: PremiumLimitTypeActiveStoryCount,
}

impl RTDPremiumLimitTypeActiveStoryCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeActiveStoryCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeActiveStoryCount> for PremiumLimitTypeActiveStoryCount {
  fn as_ref(&self) -> &PremiumLimitTypeActiveStoryCount {
    self
  }
}

impl AsRef<PremiumLimitTypeActiveStoryCount> for RTDPremiumLimitTypeActiveStoryCountBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeActiveStoryCount {
    &self.inner
  }
}

/// The maximum length of the user's bio
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeBioLength {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeBioLength {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeBioLength"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeBioLength {}

impl PremiumLimitTypeBioLength {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeBioLengthBuilder {
    let inner = PremiumLimitTypeBioLength {
      td_name: "premiumLimitTypeBioLength".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeBioLengthBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeBioLengthBuilder {
  inner: PremiumLimitTypeBioLength,
}

impl RTDPremiumLimitTypeBioLengthBuilder {
  pub fn build(&self) -> PremiumLimitTypeBioLength {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeBioLength> for PremiumLimitTypeBioLength {
  fn as_ref(&self) -> &PremiumLimitTypeBioLength {
    self
  }
}

impl AsRef<PremiumLimitTypeBioLength> for RTDPremiumLimitTypeBioLengthBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeBioLength {
    &self.inner
  }
}

/// The maximum length of sent media caption
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeCaptionLength {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeCaptionLength {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeCaptionLength"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeCaptionLength {}

impl PremiumLimitTypeCaptionLength {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeCaptionLengthBuilder {
    let inner = PremiumLimitTypeCaptionLength {
      td_name: "premiumLimitTypeCaptionLength".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeCaptionLengthBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeCaptionLengthBuilder {
  inner: PremiumLimitTypeCaptionLength,
}

impl RTDPremiumLimitTypeCaptionLengthBuilder {
  pub fn build(&self) -> PremiumLimitTypeCaptionLength {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeCaptionLength> for PremiumLimitTypeCaptionLength {
  fn as_ref(&self) -> &PremiumLimitTypeCaptionLength {
    self
  }
}

impl AsRef<PremiumLimitTypeCaptionLength> for RTDPremiumLimitTypeCaptionLengthBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeCaptionLength {
    &self.inner
  }
}

/// The maximum number of pinned and always included, or always excluded chats in a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeChatFolderChosenChatCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeChatFolderChosenChatCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeChatFolderChosenChatCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeChatFolderChosenChatCount {}

impl PremiumLimitTypeChatFolderChosenChatCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeChatFolderChosenChatCountBuilder {
    let inner = PremiumLimitTypeChatFolderChosenChatCount {
      td_name: "premiumLimitTypeChatFolderChosenChatCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeChatFolderChosenChatCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeChatFolderChosenChatCountBuilder {
  inner: PremiumLimitTypeChatFolderChosenChatCount,
}

impl RTDPremiumLimitTypeChatFolderChosenChatCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeChatFolderChosenChatCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeChatFolderChosenChatCount>
  for PremiumLimitTypeChatFolderChosenChatCount
{
  fn as_ref(&self) -> &PremiumLimitTypeChatFolderChosenChatCount {
    self
  }
}

impl AsRef<PremiumLimitTypeChatFolderChosenChatCount>
  for RTDPremiumLimitTypeChatFolderChosenChatCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypeChatFolderChosenChatCount {
    &self.inner
  }
}

/// The maximum number of chat folders
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeChatFolderCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeChatFolderCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeChatFolderCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeChatFolderCount {}

impl PremiumLimitTypeChatFolderCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeChatFolderCountBuilder {
    let inner = PremiumLimitTypeChatFolderCount {
      td_name: "premiumLimitTypeChatFolderCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeChatFolderCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeChatFolderCountBuilder {
  inner: PremiumLimitTypeChatFolderCount,
}

impl RTDPremiumLimitTypeChatFolderCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeChatFolderCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeChatFolderCount> for PremiumLimitTypeChatFolderCount {
  fn as_ref(&self) -> &PremiumLimitTypeChatFolderCount {
    self
  }
}

impl AsRef<PremiumLimitTypeChatFolderCount> for RTDPremiumLimitTypeChatFolderCountBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeChatFolderCount {
    &self.inner
  }
}

/// The maximum number of invite links for a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeChatFolderInviteLinkCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeChatFolderInviteLinkCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeChatFolderInviteLinkCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeChatFolderInviteLinkCount {}

impl PremiumLimitTypeChatFolderInviteLinkCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeChatFolderInviteLinkCountBuilder {
    let inner = PremiumLimitTypeChatFolderInviteLinkCount {
      td_name: "premiumLimitTypeChatFolderInviteLinkCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeChatFolderInviteLinkCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeChatFolderInviteLinkCountBuilder {
  inner: PremiumLimitTypeChatFolderInviteLinkCount,
}

impl RTDPremiumLimitTypeChatFolderInviteLinkCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeChatFolderInviteLinkCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeChatFolderInviteLinkCount>
  for PremiumLimitTypeChatFolderInviteLinkCount
{
  fn as_ref(&self) -> &PremiumLimitTypeChatFolderInviteLinkCount {
    self
  }
}

impl AsRef<PremiumLimitTypeChatFolderInviteLinkCount>
  for RTDPremiumLimitTypeChatFolderInviteLinkCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypeChatFolderInviteLinkCount {
    &self.inner
  }
}

/// The maximum number of created public chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeCreatedPublicChatCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeCreatedPublicChatCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeCreatedPublicChatCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeCreatedPublicChatCount {}

impl PremiumLimitTypeCreatedPublicChatCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeCreatedPublicChatCountBuilder {
    let inner = PremiumLimitTypeCreatedPublicChatCount {
      td_name: "premiumLimitTypeCreatedPublicChatCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeCreatedPublicChatCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeCreatedPublicChatCountBuilder {
  inner: PremiumLimitTypeCreatedPublicChatCount,
}

impl RTDPremiumLimitTypeCreatedPublicChatCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeCreatedPublicChatCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeCreatedPublicChatCount> for PremiumLimitTypeCreatedPublicChatCount {
  fn as_ref(&self) -> &PremiumLimitTypeCreatedPublicChatCount {
    self
  }
}

impl AsRef<PremiumLimitTypeCreatedPublicChatCount>
  for RTDPremiumLimitTypeCreatedPublicChatCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypeCreatedPublicChatCount {
    &self.inner
  }
}

/// The maximum number of added text composition styles
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeCustomTextCompositionStyleCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeCustomTextCompositionStyleCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeCustomTextCompositionStyleCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeCustomTextCompositionStyleCount {}

impl PremiumLimitTypeCustomTextCompositionStyleCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeCustomTextCompositionStyleCountBuilder {
    let inner = PremiumLimitTypeCustomTextCompositionStyleCount {
      td_name: "premiumLimitTypeCustomTextCompositionStyleCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeCustomTextCompositionStyleCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeCustomTextCompositionStyleCountBuilder {
  inner: PremiumLimitTypeCustomTextCompositionStyleCount,
}

impl RTDPremiumLimitTypeCustomTextCompositionStyleCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeCustomTextCompositionStyleCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeCustomTextCompositionStyleCount>
  for PremiumLimitTypeCustomTextCompositionStyleCount
{
  fn as_ref(&self) -> &PremiumLimitTypeCustomTextCompositionStyleCount {
    self
  }
}

impl AsRef<PremiumLimitTypeCustomTextCompositionStyleCount>
  for RTDPremiumLimitTypeCustomTextCompositionStyleCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypeCustomTextCompositionStyleCount {
    &self.inner
  }
}

/// The maximum number of favorite stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeFavoriteStickerCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeFavoriteStickerCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeFavoriteStickerCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeFavoriteStickerCount {}

impl PremiumLimitTypeFavoriteStickerCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeFavoriteStickerCountBuilder {
    let inner = PremiumLimitTypeFavoriteStickerCount {
      td_name: "premiumLimitTypeFavoriteStickerCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeFavoriteStickerCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeFavoriteStickerCountBuilder {
  inner: PremiumLimitTypeFavoriteStickerCount,
}

impl RTDPremiumLimitTypeFavoriteStickerCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeFavoriteStickerCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeFavoriteStickerCount> for PremiumLimitTypeFavoriteStickerCount {
  fn as_ref(&self) -> &PremiumLimitTypeFavoriteStickerCount {
    self
  }
}

impl AsRef<PremiumLimitTypeFavoriteStickerCount>
  for RTDPremiumLimitTypeFavoriteStickerCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypeFavoriteStickerCount {
    &self.inner
  }
}

/// The maximum length of text of sent messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeMessageTextLength {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeMessageTextLength {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeMessageTextLength"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeMessageTextLength {}

impl PremiumLimitTypeMessageTextLength {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeMessageTextLengthBuilder {
    let inner = PremiumLimitTypeMessageTextLength {
      td_name: "premiumLimitTypeMessageTextLength".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeMessageTextLengthBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeMessageTextLengthBuilder {
  inner: PremiumLimitTypeMessageTextLength,
}

impl RTDPremiumLimitTypeMessageTextLengthBuilder {
  pub fn build(&self) -> PremiumLimitTypeMessageTextLength {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeMessageTextLength> for PremiumLimitTypeMessageTextLength {
  fn as_ref(&self) -> &PremiumLimitTypeMessageTextLength {
    self
  }
}

impl AsRef<PremiumLimitTypeMessageTextLength> for RTDPremiumLimitTypeMessageTextLengthBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeMessageTextLength {
    &self.inner
  }
}

/// The maximum number of stories posted per month
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeMonthlyPostedStoryCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeMonthlyPostedStoryCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeMonthlyPostedStoryCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeMonthlyPostedStoryCount {}

impl PremiumLimitTypeMonthlyPostedStoryCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeMonthlyPostedStoryCountBuilder {
    let inner = PremiumLimitTypeMonthlyPostedStoryCount {
      td_name: "premiumLimitTypeMonthlyPostedStoryCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeMonthlyPostedStoryCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeMonthlyPostedStoryCountBuilder {
  inner: PremiumLimitTypeMonthlyPostedStoryCount,
}

impl RTDPremiumLimitTypeMonthlyPostedStoryCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeMonthlyPostedStoryCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeMonthlyPostedStoryCount> for PremiumLimitTypeMonthlyPostedStoryCount {
  fn as_ref(&self) -> &PremiumLimitTypeMonthlyPostedStoryCount {
    self
  }
}

impl AsRef<PremiumLimitTypeMonthlyPostedStoryCount>
  for RTDPremiumLimitTypeMonthlyPostedStoryCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypeMonthlyPostedStoryCount {
    &self.inner
  }
}

/// The maximum number of owned bots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeOwnedBotCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeOwnedBotCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeOwnedBotCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeOwnedBotCount {}

impl PremiumLimitTypeOwnedBotCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeOwnedBotCountBuilder {
    let inner = PremiumLimitTypeOwnedBotCount {
      td_name: "premiumLimitTypeOwnedBotCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeOwnedBotCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeOwnedBotCountBuilder {
  inner: PremiumLimitTypeOwnedBotCount,
}

impl RTDPremiumLimitTypeOwnedBotCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeOwnedBotCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeOwnedBotCount> for PremiumLimitTypeOwnedBotCount {
  fn as_ref(&self) -> &PremiumLimitTypeOwnedBotCount {
    self
  }
}

impl AsRef<PremiumLimitTypeOwnedBotCount> for RTDPremiumLimitTypeOwnedBotCountBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeOwnedBotCount {
    &self.inner
  }
}

/// The maximum number of pinned chats in the archive chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypePinnedArchivedChatCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypePinnedArchivedChatCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypePinnedArchivedChatCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypePinnedArchivedChatCount {}

impl PremiumLimitTypePinnedArchivedChatCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypePinnedArchivedChatCountBuilder {
    let inner = PremiumLimitTypePinnedArchivedChatCount {
      td_name: "premiumLimitTypePinnedArchivedChatCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypePinnedArchivedChatCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypePinnedArchivedChatCountBuilder {
  inner: PremiumLimitTypePinnedArchivedChatCount,
}

impl RTDPremiumLimitTypePinnedArchivedChatCountBuilder {
  pub fn build(&self) -> PremiumLimitTypePinnedArchivedChatCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypePinnedArchivedChatCount> for PremiumLimitTypePinnedArchivedChatCount {
  fn as_ref(&self) -> &PremiumLimitTypePinnedArchivedChatCount {
    self
  }
}

impl AsRef<PremiumLimitTypePinnedArchivedChatCount>
  for RTDPremiumLimitTypePinnedArchivedChatCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypePinnedArchivedChatCount {
    &self.inner
  }
}

/// The maximum number of pinned chats in the main chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypePinnedChatCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypePinnedChatCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypePinnedChatCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypePinnedChatCount {}

impl PremiumLimitTypePinnedChatCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypePinnedChatCountBuilder {
    let inner = PremiumLimitTypePinnedChatCount {
      td_name: "premiumLimitTypePinnedChatCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypePinnedChatCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypePinnedChatCountBuilder {
  inner: PremiumLimitTypePinnedChatCount,
}

impl RTDPremiumLimitTypePinnedChatCountBuilder {
  pub fn build(&self) -> PremiumLimitTypePinnedChatCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypePinnedChatCount> for PremiumLimitTypePinnedChatCount {
  fn as_ref(&self) -> &PremiumLimitTypePinnedChatCount {
    self
  }
}

impl AsRef<PremiumLimitTypePinnedChatCount> for RTDPremiumLimitTypePinnedChatCountBuilder {
  fn as_ref(&self) -> &PremiumLimitTypePinnedChatCount {
    &self.inner
  }
}

/// The maximum number of pinned Saved Messages topics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypePinnedSavedMessagesTopicCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypePinnedSavedMessagesTopicCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypePinnedSavedMessagesTopicCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypePinnedSavedMessagesTopicCount {}

impl PremiumLimitTypePinnedSavedMessagesTopicCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypePinnedSavedMessagesTopicCountBuilder {
    let inner = PremiumLimitTypePinnedSavedMessagesTopicCount {
      td_name: "premiumLimitTypePinnedSavedMessagesTopicCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypePinnedSavedMessagesTopicCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypePinnedSavedMessagesTopicCountBuilder {
  inner: PremiumLimitTypePinnedSavedMessagesTopicCount,
}

impl RTDPremiumLimitTypePinnedSavedMessagesTopicCountBuilder {
  pub fn build(&self) -> PremiumLimitTypePinnedSavedMessagesTopicCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypePinnedSavedMessagesTopicCount>
  for PremiumLimitTypePinnedSavedMessagesTopicCount
{
  fn as_ref(&self) -> &PremiumLimitTypePinnedSavedMessagesTopicCount {
    self
  }
}

impl AsRef<PremiumLimitTypePinnedSavedMessagesTopicCount>
  for RTDPremiumLimitTypePinnedSavedMessagesTopicCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypePinnedSavedMessagesTopicCount {
    &self.inner
  }
}

/// The maximum number of saved animations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeSavedAnimationCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeSavedAnimationCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeSavedAnimationCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeSavedAnimationCount {}

impl PremiumLimitTypeSavedAnimationCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeSavedAnimationCountBuilder {
    let inner = PremiumLimitTypeSavedAnimationCount {
      td_name: "premiumLimitTypeSavedAnimationCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeSavedAnimationCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeSavedAnimationCountBuilder {
  inner: PremiumLimitTypeSavedAnimationCount,
}

impl RTDPremiumLimitTypeSavedAnimationCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeSavedAnimationCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeSavedAnimationCount> for PremiumLimitTypeSavedAnimationCount {
  fn as_ref(&self) -> &PremiumLimitTypeSavedAnimationCount {
    self
  }
}

impl AsRef<PremiumLimitTypeSavedAnimationCount> for RTDPremiumLimitTypeSavedAnimationCountBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeSavedAnimationCount {
    &self.inner
  }
}

/// The maximum number of added shareable chat folders
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeShareableChatFolderCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeShareableChatFolderCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeShareableChatFolderCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeShareableChatFolderCount {}

impl PremiumLimitTypeShareableChatFolderCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeShareableChatFolderCountBuilder {
    let inner = PremiumLimitTypeShareableChatFolderCount {
      td_name: "premiumLimitTypeShareableChatFolderCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeShareableChatFolderCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeShareableChatFolderCountBuilder {
  inner: PremiumLimitTypeShareableChatFolderCount,
}

impl RTDPremiumLimitTypeShareableChatFolderCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeShareableChatFolderCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeShareableChatFolderCount> for PremiumLimitTypeShareableChatFolderCount {
  fn as_ref(&self) -> &PremiumLimitTypeShareableChatFolderCount {
    self
  }
}

impl AsRef<PremiumLimitTypeShareableChatFolderCount>
  for RTDPremiumLimitTypeShareableChatFolderCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypeShareableChatFolderCount {
    &self.inner
  }
}

/// The maximum number of received similar chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeSimilarChatCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeSimilarChatCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeSimilarChatCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeSimilarChatCount {}

impl PremiumLimitTypeSimilarChatCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeSimilarChatCountBuilder {
    let inner = PremiumLimitTypeSimilarChatCount {
      td_name: "premiumLimitTypeSimilarChatCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeSimilarChatCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeSimilarChatCountBuilder {
  inner: PremiumLimitTypeSimilarChatCount,
}

impl RTDPremiumLimitTypeSimilarChatCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeSimilarChatCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeSimilarChatCount> for PremiumLimitTypeSimilarChatCount {
  fn as_ref(&self) -> &PremiumLimitTypeSimilarChatCount {
    self
  }
}

impl AsRef<PremiumLimitTypeSimilarChatCount> for RTDPremiumLimitTypeSimilarChatCountBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeSimilarChatCount {
    &self.inner
  }
}

/// The maximum length of captions of posted stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeStoryCaptionLength {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeStoryCaptionLength {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeStoryCaptionLength"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeStoryCaptionLength {}

impl PremiumLimitTypeStoryCaptionLength {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeStoryCaptionLengthBuilder {
    let inner = PremiumLimitTypeStoryCaptionLength {
      td_name: "premiumLimitTypeStoryCaptionLength".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeStoryCaptionLengthBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeStoryCaptionLengthBuilder {
  inner: PremiumLimitTypeStoryCaptionLength,
}

impl RTDPremiumLimitTypeStoryCaptionLengthBuilder {
  pub fn build(&self) -> PremiumLimitTypeStoryCaptionLength {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeStoryCaptionLength> for PremiumLimitTypeStoryCaptionLength {
  fn as_ref(&self) -> &PremiumLimitTypeStoryCaptionLength {
    self
  }
}

impl AsRef<PremiumLimitTypeStoryCaptionLength> for RTDPremiumLimitTypeStoryCaptionLengthBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeStoryCaptionLength {
    &self.inner
  }
}

/// The maximum number of suggested reaction areas on a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeStorySuggestedReactionAreaCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeStorySuggestedReactionAreaCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeStorySuggestedReactionAreaCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeStorySuggestedReactionAreaCount {}

impl PremiumLimitTypeStorySuggestedReactionAreaCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeStorySuggestedReactionAreaCountBuilder {
    let inner = PremiumLimitTypeStorySuggestedReactionAreaCount {
      td_name: "premiumLimitTypeStorySuggestedReactionAreaCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeStorySuggestedReactionAreaCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeStorySuggestedReactionAreaCountBuilder {
  inner: PremiumLimitTypeStorySuggestedReactionAreaCount,
}

impl RTDPremiumLimitTypeStorySuggestedReactionAreaCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeStorySuggestedReactionAreaCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeStorySuggestedReactionAreaCount>
  for PremiumLimitTypeStorySuggestedReactionAreaCount
{
  fn as_ref(&self) -> &PremiumLimitTypeStorySuggestedReactionAreaCount {
    self
  }
}

impl AsRef<PremiumLimitTypeStorySuggestedReactionAreaCount>
  for RTDPremiumLimitTypeStorySuggestedReactionAreaCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypeStorySuggestedReactionAreaCount {
    &self.inner
  }
}

/// The maximum number of joined supergroups and channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeSupergroupCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeSupergroupCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeSupergroupCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeSupergroupCount {}

impl PremiumLimitTypeSupergroupCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeSupergroupCountBuilder {
    let inner = PremiumLimitTypeSupergroupCount {
      td_name: "premiumLimitTypeSupergroupCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeSupergroupCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeSupergroupCountBuilder {
  inner: PremiumLimitTypeSupergroupCount,
}

impl RTDPremiumLimitTypeSupergroupCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeSupergroupCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeSupergroupCount> for PremiumLimitTypeSupergroupCount {
  fn as_ref(&self) -> &PremiumLimitTypeSupergroupCount {
    self
  }
}

impl AsRef<PremiumLimitTypeSupergroupCount> for RTDPremiumLimitTypeSupergroupCountBuilder {
  fn as_ref(&self) -> &PremiumLimitTypeSupergroupCount {
    &self.inner
  }
}

/// The maximum number of stories posted per week
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeWeeklyPostedStoryCount {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumLimitTypeWeeklyPostedStoryCount {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumLimitTypeWeeklyPostedStoryCount"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumLimitType for PremiumLimitTypeWeeklyPostedStoryCount {}

impl PremiumLimitTypeWeeklyPostedStoryCount {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumLimitTypeWeeklyPostedStoryCountBuilder {
    let inner = PremiumLimitTypeWeeklyPostedStoryCount {
      td_name: "premiumLimitTypeWeeklyPostedStoryCount".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumLimitTypeWeeklyPostedStoryCountBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeWeeklyPostedStoryCountBuilder {
  inner: PremiumLimitTypeWeeklyPostedStoryCount,
}

impl RTDPremiumLimitTypeWeeklyPostedStoryCountBuilder {
  pub fn build(&self) -> PremiumLimitTypeWeeklyPostedStoryCount {
    self.inner.clone()
  }
}

impl AsRef<PremiumLimitTypeWeeklyPostedStoryCount> for PremiumLimitTypeWeeklyPostedStoryCount {
  fn as_ref(&self) -> &PremiumLimitTypeWeeklyPostedStoryCount {
    self
  }
}

impl AsRef<PremiumLimitTypeWeeklyPostedStoryCount>
  for RTDPremiumLimitTypeWeeklyPostedStoryCountBuilder
{
  fn as_ref(&self) -> &PremiumLimitTypeWeeklyPostedStoryCount {
    &self.inner
  }
}
