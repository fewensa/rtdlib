use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of transaction with Telegram Stars
pub trait TDStarTransactionType: Debug + RObject {}

/// Describes type of transaction with Telegram Stars
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StarTransactionType {
  #[doc(hidden)]
  _Default(()),
  /// The transaction is a receiving of a commission from an affiliate program; relevant for regular users, bots and channel chats only
  AffiliateProgramCommission(StarTransactionTypeAffiliateProgramCommission),
  /// The transaction is a deposit of Telegram Stars from App Store; relevant for regular users only
  AppStoreDeposit(StarTransactionTypeAppStoreDeposit),
  /// The transaction is a purchase of a product from a bot or a business account by the current user; relevant for regular users only
  BotInvoicePurchase(StarTransactionTypeBotInvoicePurchase),
  /// The transaction is a sale of a product by the bot; relevant for bots only
  BotInvoiceSale(StarTransactionTypeBotInvoiceSale),
  /// The transaction is a purchase of paid media from a bot or a business account by the current user; relevant for regular users only
  BotPaidMediaPurchase(StarTransactionTypeBotPaidMediaPurchase),
  /// The transaction is a sale of paid media by the bot or a business account managed by the bot; relevant for bots only
  BotPaidMediaSale(StarTransactionTypeBotPaidMediaSale),
  /// The transaction is a purchase of a subscription from a bot or a business account by the current user; relevant for regular users only
  BotSubscriptionPurchase(StarTransactionTypeBotSubscriptionPurchase),
  /// The transaction is a sale of a subscription by the bot; relevant for bots only
  BotSubscriptionSale(StarTransactionTypeBotSubscriptionSale),
  /// The transaction is a transfer of Telegram Stars from a business account; relevant for bots only
  BusinessBotTransferReceive(StarTransactionTypeBusinessBotTransferReceive),
  /// The transaction is a transfer of Telegram Stars to a business bot; relevant for regular users only
  BusinessBotTransferSend(StarTransactionTypeBusinessBotTransferSend),
  /// The transaction is a purchase of paid media from a channel by the current user; relevant for regular users only
  ChannelPaidMediaPurchase(StarTransactionTypeChannelPaidMediaPurchase),
  /// The transaction is a sale of paid media by the channel chat; relevant for channel chats only
  ChannelPaidMediaSale(StarTransactionTypeChannelPaidMediaSale),
  /// The transaction is a receiving of a paid reaction to a message by the channel chat; relevant for channel chats only
  ChannelPaidReactionReceive(StarTransactionTypeChannelPaidReactionReceive),
  /// The transaction is a sending of a paid reaction to a message in a channel chat by the current user; relevant for regular users only
  ChannelPaidReactionSend(StarTransactionTypeChannelPaidReactionSend),
  /// The transaction is a purchase of a subscription to a channel chat by the current user; relevant for regular users only
  ChannelSubscriptionPurchase(StarTransactionTypeChannelSubscriptionPurchase),
  /// The transaction is a sale of a subscription by the channel chat; relevant for channel chats only
  ChannelSubscriptionSale(StarTransactionTypeChannelSubscriptionSale),
  /// The transaction is a deposit of Telegram Stars from Fragment; relevant for regular users and bots only
  FragmentDeposit(StarTransactionTypeFragmentDeposit),
  /// The transaction is a withdrawal of earned Telegram Stars to Fragment; relevant for regular users, bots, supergroup and channel chats only
  FragmentWithdrawal(StarTransactionTypeFragmentWithdrawal),
  /// The transaction is a bid on a gift auction; relevant for regular users only
  GiftAuctionBid(StarTransactionTypeGiftAuctionBid),
  /// The transaction is a drop of original details of an upgraded gift; relevant for regular users only
  GiftOriginalDetailsDrop(StarTransactionTypeGiftOriginalDetailsDrop),
  /// The transaction is a purchase of a regular gift; relevant for regular users and bots only
  GiftPurchase(StarTransactionTypeGiftPurchase),
  /// The transaction is an offer of gift purchase; relevant for regular users only
  GiftPurchaseOffer(StarTransactionTypeGiftPurchaseOffer),
  /// The transaction is a sale of a received gift; relevant for regular users and channel chats only
  GiftSale(StarTransactionTypeGiftSale),
  /// The transaction is a transfer of an upgraded gift; relevant for regular users only
  GiftTransfer(StarTransactionTypeGiftTransfer),
  /// The transaction is an upgrade of a gift; relevant for regular users only
  GiftUpgrade(StarTransactionTypeGiftUpgrade),
  /// The transaction is a purchase of an upgrade of a gift owned by another user or channel; relevant for regular users only
  GiftUpgradePurchase(StarTransactionTypeGiftUpgradePurchase),
  /// The transaction is a deposit of Telegram Stars from a giveaway; relevant for regular users only
  GiveawayDeposit(StarTransactionTypeGiveawayDeposit),
  /// The transaction is a deposit of Telegram Stars from Google Play; relevant for regular users only
  GooglePlayDeposit(StarTransactionTypeGooglePlayDeposit),
  /// The transaction is a receiving of a paid group call message; relevant for regular users and channel chats only
  PaidGroupCallMessageReceive(StarTransactionTypePaidGroupCallMessageReceive),
  /// The transaction is a sending of a paid group call message; relevant for regular users only
  PaidGroupCallMessageSend(StarTransactionTypePaidGroupCallMessageSend),
  /// The transaction is a receiving of a paid group call reaction; relevant for regular users and channel chats only
  PaidGroupCallReactionReceive(StarTransactionTypePaidGroupCallReactionReceive),
  /// The transaction is a sending of a paid group call reaction; relevant for regular users only
  PaidGroupCallReactionSend(StarTransactionTypePaidGroupCallReactionSend),
  /// The transaction is a receiving of a paid message; relevant for regular users, supergroup and channel chats only
  PaidMessageReceive(StarTransactionTypePaidMessageReceive),
  /// The transaction is a sending of a paid message; relevant for regular users only
  PaidMessageSend(StarTransactionTypePaidMessageSend),
  /// The transaction is a deposit of Telegram Stars from the Premium bot; relevant for regular users only
  PremiumBotDeposit(StarTransactionTypePremiumBotDeposit),
  /// The transaction is a purchase of Telegram Premium subscription; relevant for regular users and bots only
  PremiumPurchase(StarTransactionTypePremiumPurchase),
  /// The transaction is a payment for search of posts in public Telegram channels; relevant for regular users only
  PublicPostSearch(StarTransactionTypePublicPostSearch),
  /// The transaction is a receiving of a payment for a suggested post by the channel chat; relevant for channel chats only
  SuggestedPostPaymentReceive(StarTransactionTypeSuggestedPostPaymentReceive),
  /// The transaction is a payment for a suggested post; relevant for regular users only
  SuggestedPostPaymentSend(StarTransactionTypeSuggestedPostPaymentSend),
  /// The transaction is a withdrawal of earned Telegram Stars to Telegram Ad platform; relevant for bots and channel chats only
  TelegramAdsWithdrawal(StarTransactionTypeTelegramAdsWithdrawal),
  /// The transaction is a payment for Telegram API usage; relevant for bots only
  TelegramApiUsage(StarTransactionTypeTelegramApiUsage),
  /// The transaction is a transaction of an unsupported type
  Unsupported(StarTransactionTypeUnsupported),
  /// The transaction is a purchase of an upgraded gift for some user or channel; relevant for regular users only
  UpgradedGiftPurchase(StarTransactionTypeUpgradedGiftPurchase),
  /// The transaction is a sale of an upgraded gift; relevant for regular users only
  UpgradedGiftSale(StarTransactionTypeUpgradedGiftSale),
  /// The transaction is a deposit of Telegram Stars by another user; relevant for regular users only
  UserDeposit(StarTransactionTypeUserDeposit),
}

impl Default for StarTransactionType {
  fn default() -> Self {
    StarTransactionType::_Default(())
  }
}

impl<'de> Deserialize<'de> for StarTransactionType {
  fn deserialize<D>(deserializer: D) -> Result<StarTransactionType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StarTransactionType,
      (starTransactionTypeAffiliateProgramCommission, AffiliateProgramCommission);
      (starTransactionTypeAppStoreDeposit, AppStoreDeposit);
      (starTransactionTypeBotInvoicePurchase, BotInvoicePurchase);
      (starTransactionTypeBotInvoiceSale, BotInvoiceSale);
      (starTransactionTypeBotPaidMediaPurchase, BotPaidMediaPurchase);
      (starTransactionTypeBotPaidMediaSale, BotPaidMediaSale);
      (starTransactionTypeBotSubscriptionPurchase, BotSubscriptionPurchase);
      (starTransactionTypeBotSubscriptionSale, BotSubscriptionSale);
      (starTransactionTypeBusinessBotTransferReceive, BusinessBotTransferReceive);
      (starTransactionTypeBusinessBotTransferSend, BusinessBotTransferSend);
      (starTransactionTypeChannelPaidMediaPurchase, ChannelPaidMediaPurchase);
      (starTransactionTypeChannelPaidMediaSale, ChannelPaidMediaSale);
      (starTransactionTypeChannelPaidReactionReceive, ChannelPaidReactionReceive);
      (starTransactionTypeChannelPaidReactionSend, ChannelPaidReactionSend);
      (starTransactionTypeChannelSubscriptionPurchase, ChannelSubscriptionPurchase);
      (starTransactionTypeChannelSubscriptionSale, ChannelSubscriptionSale);
      (starTransactionTypeFragmentDeposit, FragmentDeposit);
      (starTransactionTypeFragmentWithdrawal, FragmentWithdrawal);
      (starTransactionTypeGiftAuctionBid, GiftAuctionBid);
      (starTransactionTypeGiftOriginalDetailsDrop, GiftOriginalDetailsDrop);
      (starTransactionTypeGiftPurchase, GiftPurchase);
      (starTransactionTypeGiftPurchaseOffer, GiftPurchaseOffer);
      (starTransactionTypeGiftSale, GiftSale);
      (starTransactionTypeGiftTransfer, GiftTransfer);
      (starTransactionTypeGiftUpgrade, GiftUpgrade);
      (starTransactionTypeGiftUpgradePurchase, GiftUpgradePurchase);
      (starTransactionTypeGiveawayDeposit, GiveawayDeposit);
      (starTransactionTypeGooglePlayDeposit, GooglePlayDeposit);
      (starTransactionTypePaidGroupCallMessageReceive, PaidGroupCallMessageReceive);
      (starTransactionTypePaidGroupCallMessageSend, PaidGroupCallMessageSend);
      (starTransactionTypePaidGroupCallReactionReceive, PaidGroupCallReactionReceive);
      (starTransactionTypePaidGroupCallReactionSend, PaidGroupCallReactionSend);
      (starTransactionTypePaidMessageReceive, PaidMessageReceive);
      (starTransactionTypePaidMessageSend, PaidMessageSend);
      (starTransactionTypePremiumBotDeposit, PremiumBotDeposit);
      (starTransactionTypePremiumPurchase, PremiumPurchase);
      (starTransactionTypePublicPostSearch, PublicPostSearch);
      (starTransactionTypeSuggestedPostPaymentReceive, SuggestedPostPaymentReceive);
      (starTransactionTypeSuggestedPostPaymentSend, SuggestedPostPaymentSend);
      (starTransactionTypeTelegramAdsWithdrawal, TelegramAdsWithdrawal);
      (starTransactionTypeTelegramApiUsage, TelegramApiUsage);
      (starTransactionTypeUnsupported, Unsupported);
      (starTransactionTypeUpgradedGiftPurchase, UpgradedGiftPurchase);
      (starTransactionTypeUpgradedGiftSale, UpgradedGiftSale);
      (starTransactionTypeUserDeposit, UserDeposit);

    )(deserializer)
  }
}

impl RObject for StarTransactionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StarTransactionType::AffiliateProgramCommission(t) => t.td_name(),
      StarTransactionType::AppStoreDeposit(t) => t.td_name(),
      StarTransactionType::BotInvoicePurchase(t) => t.td_name(),
      StarTransactionType::BotInvoiceSale(t) => t.td_name(),
      StarTransactionType::BotPaidMediaPurchase(t) => t.td_name(),
      StarTransactionType::BotPaidMediaSale(t) => t.td_name(),
      StarTransactionType::BotSubscriptionPurchase(t) => t.td_name(),
      StarTransactionType::BotSubscriptionSale(t) => t.td_name(),
      StarTransactionType::BusinessBotTransferReceive(t) => t.td_name(),
      StarTransactionType::BusinessBotTransferSend(t) => t.td_name(),
      StarTransactionType::ChannelPaidMediaPurchase(t) => t.td_name(),
      StarTransactionType::ChannelPaidMediaSale(t) => t.td_name(),
      StarTransactionType::ChannelPaidReactionReceive(t) => t.td_name(),
      StarTransactionType::ChannelPaidReactionSend(t) => t.td_name(),
      StarTransactionType::ChannelSubscriptionPurchase(t) => t.td_name(),
      StarTransactionType::ChannelSubscriptionSale(t) => t.td_name(),
      StarTransactionType::FragmentDeposit(t) => t.td_name(),
      StarTransactionType::FragmentWithdrawal(t) => t.td_name(),
      StarTransactionType::GiftAuctionBid(t) => t.td_name(),
      StarTransactionType::GiftOriginalDetailsDrop(t) => t.td_name(),
      StarTransactionType::GiftPurchase(t) => t.td_name(),
      StarTransactionType::GiftPurchaseOffer(t) => t.td_name(),
      StarTransactionType::GiftSale(t) => t.td_name(),
      StarTransactionType::GiftTransfer(t) => t.td_name(),
      StarTransactionType::GiftUpgrade(t) => t.td_name(),
      StarTransactionType::GiftUpgradePurchase(t) => t.td_name(),
      StarTransactionType::GiveawayDeposit(t) => t.td_name(),
      StarTransactionType::GooglePlayDeposit(t) => t.td_name(),
      StarTransactionType::PaidGroupCallMessageReceive(t) => t.td_name(),
      StarTransactionType::PaidGroupCallMessageSend(t) => t.td_name(),
      StarTransactionType::PaidGroupCallReactionReceive(t) => t.td_name(),
      StarTransactionType::PaidGroupCallReactionSend(t) => t.td_name(),
      StarTransactionType::PaidMessageReceive(t) => t.td_name(),
      StarTransactionType::PaidMessageSend(t) => t.td_name(),
      StarTransactionType::PremiumBotDeposit(t) => t.td_name(),
      StarTransactionType::PremiumPurchase(t) => t.td_name(),
      StarTransactionType::PublicPostSearch(t) => t.td_name(),
      StarTransactionType::SuggestedPostPaymentReceive(t) => t.td_name(),
      StarTransactionType::SuggestedPostPaymentSend(t) => t.td_name(),
      StarTransactionType::TelegramAdsWithdrawal(t) => t.td_name(),
      StarTransactionType::TelegramApiUsage(t) => t.td_name(),
      StarTransactionType::Unsupported(t) => t.td_name(),
      StarTransactionType::UpgradedGiftPurchase(t) => t.td_name(),
      StarTransactionType::UpgradedGiftSale(t) => t.td_name(),
      StarTransactionType::UserDeposit(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StarTransactionType::AffiliateProgramCommission(t) => t.extra(),
      StarTransactionType::AppStoreDeposit(t) => t.extra(),
      StarTransactionType::BotInvoicePurchase(t) => t.extra(),
      StarTransactionType::BotInvoiceSale(t) => t.extra(),
      StarTransactionType::BotPaidMediaPurchase(t) => t.extra(),
      StarTransactionType::BotPaidMediaSale(t) => t.extra(),
      StarTransactionType::BotSubscriptionPurchase(t) => t.extra(),
      StarTransactionType::BotSubscriptionSale(t) => t.extra(),
      StarTransactionType::BusinessBotTransferReceive(t) => t.extra(),
      StarTransactionType::BusinessBotTransferSend(t) => t.extra(),
      StarTransactionType::ChannelPaidMediaPurchase(t) => t.extra(),
      StarTransactionType::ChannelPaidMediaSale(t) => t.extra(),
      StarTransactionType::ChannelPaidReactionReceive(t) => t.extra(),
      StarTransactionType::ChannelPaidReactionSend(t) => t.extra(),
      StarTransactionType::ChannelSubscriptionPurchase(t) => t.extra(),
      StarTransactionType::ChannelSubscriptionSale(t) => t.extra(),
      StarTransactionType::FragmentDeposit(t) => t.extra(),
      StarTransactionType::FragmentWithdrawal(t) => t.extra(),
      StarTransactionType::GiftAuctionBid(t) => t.extra(),
      StarTransactionType::GiftOriginalDetailsDrop(t) => t.extra(),
      StarTransactionType::GiftPurchase(t) => t.extra(),
      StarTransactionType::GiftPurchaseOffer(t) => t.extra(),
      StarTransactionType::GiftSale(t) => t.extra(),
      StarTransactionType::GiftTransfer(t) => t.extra(),
      StarTransactionType::GiftUpgrade(t) => t.extra(),
      StarTransactionType::GiftUpgradePurchase(t) => t.extra(),
      StarTransactionType::GiveawayDeposit(t) => t.extra(),
      StarTransactionType::GooglePlayDeposit(t) => t.extra(),
      StarTransactionType::PaidGroupCallMessageReceive(t) => t.extra(),
      StarTransactionType::PaidGroupCallMessageSend(t) => t.extra(),
      StarTransactionType::PaidGroupCallReactionReceive(t) => t.extra(),
      StarTransactionType::PaidGroupCallReactionSend(t) => t.extra(),
      StarTransactionType::PaidMessageReceive(t) => t.extra(),
      StarTransactionType::PaidMessageSend(t) => t.extra(),
      StarTransactionType::PremiumBotDeposit(t) => t.extra(),
      StarTransactionType::PremiumPurchase(t) => t.extra(),
      StarTransactionType::PublicPostSearch(t) => t.extra(),
      StarTransactionType::SuggestedPostPaymentReceive(t) => t.extra(),
      StarTransactionType::SuggestedPostPaymentSend(t) => t.extra(),
      StarTransactionType::TelegramAdsWithdrawal(t) => t.extra(),
      StarTransactionType::TelegramApiUsage(t) => t.extra(),
      StarTransactionType::Unsupported(t) => t.extra(),
      StarTransactionType::UpgradedGiftPurchase(t) => t.extra(),
      StarTransactionType::UpgradedGiftSale(t) => t.extra(),
      StarTransactionType::UserDeposit(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarTransactionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StarTransactionType::_Default(_))
  }

  pub fn is_affiliate_program_commission(&self) -> bool {
    matches!(self, StarTransactionType::AffiliateProgramCommission(_))
  }
  pub fn is_app_store_deposit(&self) -> bool {
    matches!(self, StarTransactionType::AppStoreDeposit(_))
  }
  pub fn is_bot_invoice_purchase(&self) -> bool {
    matches!(self, StarTransactionType::BotInvoicePurchase(_))
  }
  pub fn is_bot_invoice_sale(&self) -> bool {
    matches!(self, StarTransactionType::BotInvoiceSale(_))
  }
  pub fn is_bot_paid_media_purchase(&self) -> bool {
    matches!(self, StarTransactionType::BotPaidMediaPurchase(_))
  }
  pub fn is_bot_paid_media_sale(&self) -> bool {
    matches!(self, StarTransactionType::BotPaidMediaSale(_))
  }
  pub fn is_bot_subscription_purchase(&self) -> bool {
    matches!(self, StarTransactionType::BotSubscriptionPurchase(_))
  }
  pub fn is_bot_subscription_sale(&self) -> bool {
    matches!(self, StarTransactionType::BotSubscriptionSale(_))
  }
  pub fn is_business_bot_transfer_receive(&self) -> bool {
    matches!(self, StarTransactionType::BusinessBotTransferReceive(_))
  }
  pub fn is_business_bot_transfer_send(&self) -> bool {
    matches!(self, StarTransactionType::BusinessBotTransferSend(_))
  }
  pub fn is_channel_paid_media_purchase(&self) -> bool {
    matches!(self, StarTransactionType::ChannelPaidMediaPurchase(_))
  }
  pub fn is_channel_paid_media_sale(&self) -> bool {
    matches!(self, StarTransactionType::ChannelPaidMediaSale(_))
  }
  pub fn is_channel_paid_reaction_receive(&self) -> bool {
    matches!(self, StarTransactionType::ChannelPaidReactionReceive(_))
  }
  pub fn is_channel_paid_reaction_send(&self) -> bool {
    matches!(self, StarTransactionType::ChannelPaidReactionSend(_))
  }
  pub fn is_channel_subscription_purchase(&self) -> bool {
    matches!(self, StarTransactionType::ChannelSubscriptionPurchase(_))
  }
  pub fn is_channel_subscription_sale(&self) -> bool {
    matches!(self, StarTransactionType::ChannelSubscriptionSale(_))
  }
  pub fn is_fragment_deposit(&self) -> bool {
    matches!(self, StarTransactionType::FragmentDeposit(_))
  }
  pub fn is_fragment_withdrawal(&self) -> bool {
    matches!(self, StarTransactionType::FragmentWithdrawal(_))
  }
  pub fn is_gift_auction_bid(&self) -> bool {
    matches!(self, StarTransactionType::GiftAuctionBid(_))
  }
  pub fn is_gift_original_details_drop(&self) -> bool {
    matches!(self, StarTransactionType::GiftOriginalDetailsDrop(_))
  }
  pub fn is_gift_purchase(&self) -> bool {
    matches!(self, StarTransactionType::GiftPurchase(_))
  }
  pub fn is_gift_purchase_offer(&self) -> bool {
    matches!(self, StarTransactionType::GiftPurchaseOffer(_))
  }
  pub fn is_gift_sale(&self) -> bool {
    matches!(self, StarTransactionType::GiftSale(_))
  }
  pub fn is_gift_transfer(&self) -> bool {
    matches!(self, StarTransactionType::GiftTransfer(_))
  }
  pub fn is_gift_upgrade(&self) -> bool {
    matches!(self, StarTransactionType::GiftUpgrade(_))
  }
  pub fn is_gift_upgrade_purchase(&self) -> bool {
    matches!(self, StarTransactionType::GiftUpgradePurchase(_))
  }
  pub fn is_giveaway_deposit(&self) -> bool {
    matches!(self, StarTransactionType::GiveawayDeposit(_))
  }
  pub fn is_google_play_deposit(&self) -> bool {
    matches!(self, StarTransactionType::GooglePlayDeposit(_))
  }
  pub fn is_paid_group_call_message_receive(&self) -> bool {
    matches!(self, StarTransactionType::PaidGroupCallMessageReceive(_))
  }
  pub fn is_paid_group_call_message_send(&self) -> bool {
    matches!(self, StarTransactionType::PaidGroupCallMessageSend(_))
  }
  pub fn is_paid_group_call_reaction_receive(&self) -> bool {
    matches!(self, StarTransactionType::PaidGroupCallReactionReceive(_))
  }
  pub fn is_paid_group_call_reaction_send(&self) -> bool {
    matches!(self, StarTransactionType::PaidGroupCallReactionSend(_))
  }
  pub fn is_paid_message_receive(&self) -> bool {
    matches!(self, StarTransactionType::PaidMessageReceive(_))
  }
  pub fn is_paid_message_send(&self) -> bool {
    matches!(self, StarTransactionType::PaidMessageSend(_))
  }
  pub fn is_premium_bot_deposit(&self) -> bool {
    matches!(self, StarTransactionType::PremiumBotDeposit(_))
  }
  pub fn is_premium_purchase(&self) -> bool {
    matches!(self, StarTransactionType::PremiumPurchase(_))
  }
  pub fn is_public_post_search(&self) -> bool {
    matches!(self, StarTransactionType::PublicPostSearch(_))
  }
  pub fn is_suggested_post_payment_receive(&self) -> bool {
    matches!(self, StarTransactionType::SuggestedPostPaymentReceive(_))
  }
  pub fn is_suggested_post_payment_send(&self) -> bool {
    matches!(self, StarTransactionType::SuggestedPostPaymentSend(_))
  }
  pub fn is_telegram_ads_withdrawal(&self) -> bool {
    matches!(self, StarTransactionType::TelegramAdsWithdrawal(_))
  }
  pub fn is_telegram_api_usage(&self) -> bool {
    matches!(self, StarTransactionType::TelegramApiUsage(_))
  }
  pub fn is_unsupported(&self) -> bool {
    matches!(self, StarTransactionType::Unsupported(_))
  }
  pub fn is_upgraded_gift_purchase(&self) -> bool {
    matches!(self, StarTransactionType::UpgradedGiftPurchase(_))
  }
  pub fn is_upgraded_gift_sale(&self) -> bool {
    matches!(self, StarTransactionType::UpgradedGiftSale(_))
  }
  pub fn is_user_deposit(&self) -> bool {
    matches!(self, StarTransactionType::UserDeposit(_))
  }

  pub fn on_affiliate_program_commission<
    F: FnOnce(&StarTransactionTypeAffiliateProgramCommission),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::AffiliateProgramCommission(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_app_store_deposit<F: FnOnce(&StarTransactionTypeAppStoreDeposit)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::AppStoreDeposit(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_invoice_purchase<F: FnOnce(&StarTransactionTypeBotInvoicePurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::BotInvoicePurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_invoice_sale<F: FnOnce(&StarTransactionTypeBotInvoiceSale)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::BotInvoiceSale(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_paid_media_purchase<F: FnOnce(&StarTransactionTypeBotPaidMediaPurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::BotPaidMediaPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_paid_media_sale<F: FnOnce(&StarTransactionTypeBotPaidMediaSale)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::BotPaidMediaSale(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_subscription_purchase<F: FnOnce(&StarTransactionTypeBotSubscriptionPurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::BotSubscriptionPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_subscription_sale<F: FnOnce(&StarTransactionTypeBotSubscriptionSale)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::BotSubscriptionSale(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_business_bot_transfer_receive<
    F: FnOnce(&StarTransactionTypeBusinessBotTransferReceive),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::BusinessBotTransferReceive(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_business_bot_transfer_send<F: FnOnce(&StarTransactionTypeBusinessBotTransferSend)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::BusinessBotTransferSend(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel_paid_media_purchase<F: FnOnce(&StarTransactionTypeChannelPaidMediaPurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::ChannelPaidMediaPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel_paid_media_sale<F: FnOnce(&StarTransactionTypeChannelPaidMediaSale)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::ChannelPaidMediaSale(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel_paid_reaction_receive<
    F: FnOnce(&StarTransactionTypeChannelPaidReactionReceive),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::ChannelPaidReactionReceive(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel_paid_reaction_send<F: FnOnce(&StarTransactionTypeChannelPaidReactionSend)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::ChannelPaidReactionSend(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel_subscription_purchase<
    F: FnOnce(&StarTransactionTypeChannelSubscriptionPurchase),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::ChannelSubscriptionPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_channel_subscription_sale<F: FnOnce(&StarTransactionTypeChannelSubscriptionSale)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::ChannelSubscriptionSale(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fragment_deposit<F: FnOnce(&StarTransactionTypeFragmentDeposit)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::FragmentDeposit(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fragment_withdrawal<F: FnOnce(&StarTransactionTypeFragmentWithdrawal)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::FragmentWithdrawal(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_auction_bid<F: FnOnce(&StarTransactionTypeGiftAuctionBid)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::GiftAuctionBid(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_original_details_drop<F: FnOnce(&StarTransactionTypeGiftOriginalDetailsDrop)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::GiftOriginalDetailsDrop(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_purchase<F: FnOnce(&StarTransactionTypeGiftPurchase)>(&self, fnc: F) -> &Self {
    if let StarTransactionType::GiftPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_purchase_offer<F: FnOnce(&StarTransactionTypeGiftPurchaseOffer)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::GiftPurchaseOffer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_sale<F: FnOnce(&StarTransactionTypeGiftSale)>(&self, fnc: F) -> &Self {
    if let StarTransactionType::GiftSale(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_transfer<F: FnOnce(&StarTransactionTypeGiftTransfer)>(&self, fnc: F) -> &Self {
    if let StarTransactionType::GiftTransfer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_upgrade<F: FnOnce(&StarTransactionTypeGiftUpgrade)>(&self, fnc: F) -> &Self {
    if let StarTransactionType::GiftUpgrade(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_upgrade_purchase<F: FnOnce(&StarTransactionTypeGiftUpgradePurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::GiftUpgradePurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_giveaway_deposit<F: FnOnce(&StarTransactionTypeGiveawayDeposit)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::GiveawayDeposit(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_google_play_deposit<F: FnOnce(&StarTransactionTypeGooglePlayDeposit)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::GooglePlayDeposit(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid_group_call_message_receive<
    F: FnOnce(&StarTransactionTypePaidGroupCallMessageReceive),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::PaidGroupCallMessageReceive(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid_group_call_message_send<
    F: FnOnce(&StarTransactionTypePaidGroupCallMessageSend),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::PaidGroupCallMessageSend(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid_group_call_reaction_receive<
    F: FnOnce(&StarTransactionTypePaidGroupCallReactionReceive),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::PaidGroupCallReactionReceive(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid_group_call_reaction_send<
    F: FnOnce(&StarTransactionTypePaidGroupCallReactionSend),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::PaidGroupCallReactionSend(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid_message_receive<F: FnOnce(&StarTransactionTypePaidMessageReceive)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::PaidMessageReceive(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paid_message_send<F: FnOnce(&StarTransactionTypePaidMessageSend)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::PaidMessageSend(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_bot_deposit<F: FnOnce(&StarTransactionTypePremiumBotDeposit)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::PremiumBotDeposit(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_purchase<F: FnOnce(&StarTransactionTypePremiumPurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::PremiumPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_public_post_search<F: FnOnce(&StarTransactionTypePublicPostSearch)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::PublicPostSearch(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_suggested_post_payment_receive<
    F: FnOnce(&StarTransactionTypeSuggestedPostPaymentReceive),
  >(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::SuggestedPostPaymentReceive(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_suggested_post_payment_send<F: FnOnce(&StarTransactionTypeSuggestedPostPaymentSend)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::SuggestedPostPaymentSend(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_telegram_ads_withdrawal<F: FnOnce(&StarTransactionTypeTelegramAdsWithdrawal)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::TelegramAdsWithdrawal(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_telegram_api_usage<F: FnOnce(&StarTransactionTypeTelegramApiUsage)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::TelegramApiUsage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unsupported<F: FnOnce(&StarTransactionTypeUnsupported)>(&self, fnc: F) -> &Self {
    if let StarTransactionType::Unsupported(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift_purchase<F: FnOnce(&StarTransactionTypeUpgradedGiftPurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::UpgradedGiftPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift_sale<F: FnOnce(&StarTransactionTypeUpgradedGiftSale)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let StarTransactionType::UpgradedGiftSale(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_user_deposit<F: FnOnce(&StarTransactionTypeUserDeposit)>(&self, fnc: F) -> &Self {
    if let StarTransactionType::UserDeposit(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_affiliate_program_commission(
    &self,
  ) -> Option<&StarTransactionTypeAffiliateProgramCommission> {
    if let StarTransactionType::AffiliateProgramCommission(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_app_store_deposit(&self) -> Option<&StarTransactionTypeAppStoreDeposit> {
    if let StarTransactionType::AppStoreDeposit(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_invoice_purchase(&self) -> Option<&StarTransactionTypeBotInvoicePurchase> {
    if let StarTransactionType::BotInvoicePurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_invoice_sale(&self) -> Option<&StarTransactionTypeBotInvoiceSale> {
    if let StarTransactionType::BotInvoiceSale(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_paid_media_purchase(&self) -> Option<&StarTransactionTypeBotPaidMediaPurchase> {
    if let StarTransactionType::BotPaidMediaPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_paid_media_sale(&self) -> Option<&StarTransactionTypeBotPaidMediaSale> {
    if let StarTransactionType::BotPaidMediaSale(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_subscription_purchase(
    &self,
  ) -> Option<&StarTransactionTypeBotSubscriptionPurchase> {
    if let StarTransactionType::BotSubscriptionPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_subscription_sale(&self) -> Option<&StarTransactionTypeBotSubscriptionSale> {
    if let StarTransactionType::BotSubscriptionSale(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_business_bot_transfer_receive(
    &self,
  ) -> Option<&StarTransactionTypeBusinessBotTransferReceive> {
    if let StarTransactionType::BusinessBotTransferReceive(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_business_bot_transfer_send(
    &self,
  ) -> Option<&StarTransactionTypeBusinessBotTransferSend> {
    if let StarTransactionType::BusinessBotTransferSend(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel_paid_media_purchase(
    &self,
  ) -> Option<&StarTransactionTypeChannelPaidMediaPurchase> {
    if let StarTransactionType::ChannelPaidMediaPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel_paid_media_sale(&self) -> Option<&StarTransactionTypeChannelPaidMediaSale> {
    if let StarTransactionType::ChannelPaidMediaSale(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel_paid_reaction_receive(
    &self,
  ) -> Option<&StarTransactionTypeChannelPaidReactionReceive> {
    if let StarTransactionType::ChannelPaidReactionReceive(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel_paid_reaction_send(
    &self,
  ) -> Option<&StarTransactionTypeChannelPaidReactionSend> {
    if let StarTransactionType::ChannelPaidReactionSend(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel_subscription_purchase(
    &self,
  ) -> Option<&StarTransactionTypeChannelSubscriptionPurchase> {
    if let StarTransactionType::ChannelSubscriptionPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_channel_subscription_sale(
    &self,
  ) -> Option<&StarTransactionTypeChannelSubscriptionSale> {
    if let StarTransactionType::ChannelSubscriptionSale(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fragment_deposit(&self) -> Option<&StarTransactionTypeFragmentDeposit> {
    if let StarTransactionType::FragmentDeposit(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fragment_withdrawal(&self) -> Option<&StarTransactionTypeFragmentWithdrawal> {
    if let StarTransactionType::FragmentWithdrawal(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_auction_bid(&self) -> Option<&StarTransactionTypeGiftAuctionBid> {
    if let StarTransactionType::GiftAuctionBid(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_original_details_drop(
    &self,
  ) -> Option<&StarTransactionTypeGiftOriginalDetailsDrop> {
    if let StarTransactionType::GiftOriginalDetailsDrop(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_purchase(&self) -> Option<&StarTransactionTypeGiftPurchase> {
    if let StarTransactionType::GiftPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_purchase_offer(&self) -> Option<&StarTransactionTypeGiftPurchaseOffer> {
    if let StarTransactionType::GiftPurchaseOffer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_sale(&self) -> Option<&StarTransactionTypeGiftSale> {
    if let StarTransactionType::GiftSale(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_transfer(&self) -> Option<&StarTransactionTypeGiftTransfer> {
    if let StarTransactionType::GiftTransfer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_upgrade(&self) -> Option<&StarTransactionTypeGiftUpgrade> {
    if let StarTransactionType::GiftUpgrade(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_upgrade_purchase(&self) -> Option<&StarTransactionTypeGiftUpgradePurchase> {
    if let StarTransactionType::GiftUpgradePurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_giveaway_deposit(&self) -> Option<&StarTransactionTypeGiveawayDeposit> {
    if let StarTransactionType::GiveawayDeposit(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_google_play_deposit(&self) -> Option<&StarTransactionTypeGooglePlayDeposit> {
    if let StarTransactionType::GooglePlayDeposit(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid_group_call_message_receive(
    &self,
  ) -> Option<&StarTransactionTypePaidGroupCallMessageReceive> {
    if let StarTransactionType::PaidGroupCallMessageReceive(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid_group_call_message_send(
    &self,
  ) -> Option<&StarTransactionTypePaidGroupCallMessageSend> {
    if let StarTransactionType::PaidGroupCallMessageSend(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid_group_call_reaction_receive(
    &self,
  ) -> Option<&StarTransactionTypePaidGroupCallReactionReceive> {
    if let StarTransactionType::PaidGroupCallReactionReceive(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid_group_call_reaction_send(
    &self,
  ) -> Option<&StarTransactionTypePaidGroupCallReactionSend> {
    if let StarTransactionType::PaidGroupCallReactionSend(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid_message_receive(&self) -> Option<&StarTransactionTypePaidMessageReceive> {
    if let StarTransactionType::PaidMessageReceive(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paid_message_send(&self) -> Option<&StarTransactionTypePaidMessageSend> {
    if let StarTransactionType::PaidMessageSend(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_bot_deposit(&self) -> Option<&StarTransactionTypePremiumBotDeposit> {
    if let StarTransactionType::PremiumBotDeposit(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_purchase(&self) -> Option<&StarTransactionTypePremiumPurchase> {
    if let StarTransactionType::PremiumPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_public_post_search(&self) -> Option<&StarTransactionTypePublicPostSearch> {
    if let StarTransactionType::PublicPostSearch(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_suggested_post_payment_receive(
    &self,
  ) -> Option<&StarTransactionTypeSuggestedPostPaymentReceive> {
    if let StarTransactionType::SuggestedPostPaymentReceive(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_suggested_post_payment_send(
    &self,
  ) -> Option<&StarTransactionTypeSuggestedPostPaymentSend> {
    if let StarTransactionType::SuggestedPostPaymentSend(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_telegram_ads_withdrawal(&self) -> Option<&StarTransactionTypeTelegramAdsWithdrawal> {
    if let StarTransactionType::TelegramAdsWithdrawal(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_telegram_api_usage(&self) -> Option<&StarTransactionTypeTelegramApiUsage> {
    if let StarTransactionType::TelegramApiUsage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unsupported(&self) -> Option<&StarTransactionTypeUnsupported> {
    if let StarTransactionType::Unsupported(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift_purchase(&self) -> Option<&StarTransactionTypeUpgradedGiftPurchase> {
    if let StarTransactionType::UpgradedGiftPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift_sale(&self) -> Option<&StarTransactionTypeUpgradedGiftSale> {
    if let StarTransactionType::UpgradedGiftSale(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_user_deposit(&self) -> Option<&StarTransactionTypeUserDeposit> {
    if let StarTransactionType::UserDeposit(t) = self {
      return Some(t);
    }
    None
  }

  pub fn affiliate_program_commission<T: AsRef<StarTransactionTypeAffiliateProgramCommission>>(
    t: T,
  ) -> Self {
    StarTransactionType::AffiliateProgramCommission(t.as_ref().clone())
  }

  pub fn app_store_deposit<T: AsRef<StarTransactionTypeAppStoreDeposit>>(t: T) -> Self {
    StarTransactionType::AppStoreDeposit(t.as_ref().clone())
  }

  pub fn bot_invoice_purchase<T: AsRef<StarTransactionTypeBotInvoicePurchase>>(t: T) -> Self {
    StarTransactionType::BotInvoicePurchase(t.as_ref().clone())
  }

  pub fn bot_invoice_sale<T: AsRef<StarTransactionTypeBotInvoiceSale>>(t: T) -> Self {
    StarTransactionType::BotInvoiceSale(t.as_ref().clone())
  }

  pub fn bot_paid_media_purchase<T: AsRef<StarTransactionTypeBotPaidMediaPurchase>>(t: T) -> Self {
    StarTransactionType::BotPaidMediaPurchase(t.as_ref().clone())
  }

  pub fn bot_paid_media_sale<T: AsRef<StarTransactionTypeBotPaidMediaSale>>(t: T) -> Self {
    StarTransactionType::BotPaidMediaSale(t.as_ref().clone())
  }

  pub fn bot_subscription_purchase<T: AsRef<StarTransactionTypeBotSubscriptionPurchase>>(
    t: T,
  ) -> Self {
    StarTransactionType::BotSubscriptionPurchase(t.as_ref().clone())
  }

  pub fn bot_subscription_sale<T: AsRef<StarTransactionTypeBotSubscriptionSale>>(t: T) -> Self {
    StarTransactionType::BotSubscriptionSale(t.as_ref().clone())
  }

  pub fn business_bot_transfer_receive<T: AsRef<StarTransactionTypeBusinessBotTransferReceive>>(
    t: T,
  ) -> Self {
    StarTransactionType::BusinessBotTransferReceive(t.as_ref().clone())
  }

  pub fn business_bot_transfer_send<T: AsRef<StarTransactionTypeBusinessBotTransferSend>>(
    t: T,
  ) -> Self {
    StarTransactionType::BusinessBotTransferSend(t.as_ref().clone())
  }

  pub fn channel_paid_media_purchase<T: AsRef<StarTransactionTypeChannelPaidMediaPurchase>>(
    t: T,
  ) -> Self {
    StarTransactionType::ChannelPaidMediaPurchase(t.as_ref().clone())
  }

  pub fn channel_paid_media_sale<T: AsRef<StarTransactionTypeChannelPaidMediaSale>>(t: T) -> Self {
    StarTransactionType::ChannelPaidMediaSale(t.as_ref().clone())
  }

  pub fn channel_paid_reaction_receive<T: AsRef<StarTransactionTypeChannelPaidReactionReceive>>(
    t: T,
  ) -> Self {
    StarTransactionType::ChannelPaidReactionReceive(t.as_ref().clone())
  }

  pub fn channel_paid_reaction_send<T: AsRef<StarTransactionTypeChannelPaidReactionSend>>(
    t: T,
  ) -> Self {
    StarTransactionType::ChannelPaidReactionSend(t.as_ref().clone())
  }

  pub fn channel_subscription_purchase<T: AsRef<StarTransactionTypeChannelSubscriptionPurchase>>(
    t: T,
  ) -> Self {
    StarTransactionType::ChannelSubscriptionPurchase(t.as_ref().clone())
  }

  pub fn channel_subscription_sale<T: AsRef<StarTransactionTypeChannelSubscriptionSale>>(
    t: T,
  ) -> Self {
    StarTransactionType::ChannelSubscriptionSale(t.as_ref().clone())
  }

  pub fn fragment_deposit<T: AsRef<StarTransactionTypeFragmentDeposit>>(t: T) -> Self {
    StarTransactionType::FragmentDeposit(t.as_ref().clone())
  }

  pub fn fragment_withdrawal<T: AsRef<StarTransactionTypeFragmentWithdrawal>>(t: T) -> Self {
    StarTransactionType::FragmentWithdrawal(t.as_ref().clone())
  }

  pub fn gift_auction_bid<T: AsRef<StarTransactionTypeGiftAuctionBid>>(t: T) -> Self {
    StarTransactionType::GiftAuctionBid(t.as_ref().clone())
  }

  pub fn gift_original_details_drop<T: AsRef<StarTransactionTypeGiftOriginalDetailsDrop>>(
    t: T,
  ) -> Self {
    StarTransactionType::GiftOriginalDetailsDrop(t.as_ref().clone())
  }

  pub fn gift_purchase<T: AsRef<StarTransactionTypeGiftPurchase>>(t: T) -> Self {
    StarTransactionType::GiftPurchase(t.as_ref().clone())
  }

  pub fn gift_purchase_offer<T: AsRef<StarTransactionTypeGiftPurchaseOffer>>(t: T) -> Self {
    StarTransactionType::GiftPurchaseOffer(t.as_ref().clone())
  }

  pub fn gift_sale<T: AsRef<StarTransactionTypeGiftSale>>(t: T) -> Self {
    StarTransactionType::GiftSale(t.as_ref().clone())
  }

  pub fn gift_transfer<T: AsRef<StarTransactionTypeGiftTransfer>>(t: T) -> Self {
    StarTransactionType::GiftTransfer(t.as_ref().clone())
  }

  pub fn gift_upgrade<T: AsRef<StarTransactionTypeGiftUpgrade>>(t: T) -> Self {
    StarTransactionType::GiftUpgrade(t.as_ref().clone())
  }

  pub fn gift_upgrade_purchase<T: AsRef<StarTransactionTypeGiftUpgradePurchase>>(t: T) -> Self {
    StarTransactionType::GiftUpgradePurchase(t.as_ref().clone())
  }

  pub fn giveaway_deposit<T: AsRef<StarTransactionTypeGiveawayDeposit>>(t: T) -> Self {
    StarTransactionType::GiveawayDeposit(t.as_ref().clone())
  }

  pub fn google_play_deposit<T: AsRef<StarTransactionTypeGooglePlayDeposit>>(t: T) -> Self {
    StarTransactionType::GooglePlayDeposit(t.as_ref().clone())
  }

  pub fn paid_group_call_message_receive<
    T: AsRef<StarTransactionTypePaidGroupCallMessageReceive>,
  >(
    t: T,
  ) -> Self {
    StarTransactionType::PaidGroupCallMessageReceive(t.as_ref().clone())
  }

  pub fn paid_group_call_message_send<T: AsRef<StarTransactionTypePaidGroupCallMessageSend>>(
    t: T,
  ) -> Self {
    StarTransactionType::PaidGroupCallMessageSend(t.as_ref().clone())
  }

  pub fn paid_group_call_reaction_receive<
    T: AsRef<StarTransactionTypePaidGroupCallReactionReceive>,
  >(
    t: T,
  ) -> Self {
    StarTransactionType::PaidGroupCallReactionReceive(t.as_ref().clone())
  }

  pub fn paid_group_call_reaction_send<T: AsRef<StarTransactionTypePaidGroupCallReactionSend>>(
    t: T,
  ) -> Self {
    StarTransactionType::PaidGroupCallReactionSend(t.as_ref().clone())
  }

  pub fn paid_message_receive<T: AsRef<StarTransactionTypePaidMessageReceive>>(t: T) -> Self {
    StarTransactionType::PaidMessageReceive(t.as_ref().clone())
  }

  pub fn paid_message_send<T: AsRef<StarTransactionTypePaidMessageSend>>(t: T) -> Self {
    StarTransactionType::PaidMessageSend(t.as_ref().clone())
  }

  pub fn premium_bot_deposit<T: AsRef<StarTransactionTypePremiumBotDeposit>>(t: T) -> Self {
    StarTransactionType::PremiumBotDeposit(t.as_ref().clone())
  }

  pub fn premium_purchase<T: AsRef<StarTransactionTypePremiumPurchase>>(t: T) -> Self {
    StarTransactionType::PremiumPurchase(t.as_ref().clone())
  }

  pub fn public_post_search<T: AsRef<StarTransactionTypePublicPostSearch>>(t: T) -> Self {
    StarTransactionType::PublicPostSearch(t.as_ref().clone())
  }

  pub fn suggested_post_payment_receive<
    T: AsRef<StarTransactionTypeSuggestedPostPaymentReceive>,
  >(
    t: T,
  ) -> Self {
    StarTransactionType::SuggestedPostPaymentReceive(t.as_ref().clone())
  }

  pub fn suggested_post_payment_send<T: AsRef<StarTransactionTypeSuggestedPostPaymentSend>>(
    t: T,
  ) -> Self {
    StarTransactionType::SuggestedPostPaymentSend(t.as_ref().clone())
  }

  pub fn telegram_ads_withdrawal<T: AsRef<StarTransactionTypeTelegramAdsWithdrawal>>(t: T) -> Self {
    StarTransactionType::TelegramAdsWithdrawal(t.as_ref().clone())
  }

  pub fn telegram_api_usage<T: AsRef<StarTransactionTypeTelegramApiUsage>>(t: T) -> Self {
    StarTransactionType::TelegramApiUsage(t.as_ref().clone())
  }

  pub fn unsupported<T: AsRef<StarTransactionTypeUnsupported>>(t: T) -> Self {
    StarTransactionType::Unsupported(t.as_ref().clone())
  }

  pub fn upgraded_gift_purchase<T: AsRef<StarTransactionTypeUpgradedGiftPurchase>>(t: T) -> Self {
    StarTransactionType::UpgradedGiftPurchase(t.as_ref().clone())
  }

  pub fn upgraded_gift_sale<T: AsRef<StarTransactionTypeUpgradedGiftSale>>(t: T) -> Self {
    StarTransactionType::UpgradedGiftSale(t.as_ref().clone())
  }

  pub fn user_deposit<T: AsRef<StarTransactionTypeUserDeposit>>(t: T) -> Self {
    StarTransactionType::UserDeposit(t.as_ref().clone())
  }
}

impl AsRef<StarTransactionType> for StarTransactionType {
  fn as_ref(&self) -> &StarTransactionType {
    self
  }
}

/// The transaction is a receiving of a commission from an affiliate program; relevant for regular users, bots and channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeAffiliateProgramCommission {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that created the affiliate program
  chat_id: i64,
  /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the program owner
  commission_per_mille: i64,
}

impl RObject for StarTransactionTypeAffiliateProgramCommission {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeAffiliateProgramCommission"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeAffiliateProgramCommission {}

impl StarTransactionTypeAffiliateProgramCommission {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeAffiliateProgramCommissionBuilder {
    let inner = StarTransactionTypeAffiliateProgramCommission {
      td_name: "starTransactionTypeAffiliateProgramCommission".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeAffiliateProgramCommissionBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn commission_per_mille(&self) -> i64 {
    self.commission_per_mille
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeAffiliateProgramCommissionBuilder {
  inner: StarTransactionTypeAffiliateProgramCommission,
}

impl RTDStarTransactionTypeAffiliateProgramCommissionBuilder {
  pub fn build(&self) -> StarTransactionTypeAffiliateProgramCommission {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn commission_per_mille(&mut self, commission_per_mille: i64) -> &mut Self {
    self.inner.commission_per_mille = commission_per_mille;
    self
  }
}

impl AsRef<StarTransactionTypeAffiliateProgramCommission>
  for StarTransactionTypeAffiliateProgramCommission
{
  fn as_ref(&self) -> &StarTransactionTypeAffiliateProgramCommission {
    self
  }
}

impl AsRef<StarTransactionTypeAffiliateProgramCommission>
  for RTDStarTransactionTypeAffiliateProgramCommissionBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeAffiliateProgramCommission {
    &self.inner
  }
}

/// The transaction is a deposit of Telegram Stars from App Store; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeAppStoreDeposit {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StarTransactionTypeAppStoreDeposit {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeAppStoreDeposit"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeAppStoreDeposit {}

impl StarTransactionTypeAppStoreDeposit {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeAppStoreDepositBuilder {
    let inner = StarTransactionTypeAppStoreDeposit {
      td_name: "starTransactionTypeAppStoreDeposit".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStarTransactionTypeAppStoreDepositBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeAppStoreDepositBuilder {
  inner: StarTransactionTypeAppStoreDeposit,
}

impl RTDStarTransactionTypeAppStoreDepositBuilder {
  pub fn build(&self) -> StarTransactionTypeAppStoreDeposit {
    self.inner.clone()
  }
}

impl AsRef<StarTransactionTypeAppStoreDeposit> for StarTransactionTypeAppStoreDeposit {
  fn as_ref(&self) -> &StarTransactionTypeAppStoreDeposit {
    self
  }
}

impl AsRef<StarTransactionTypeAppStoreDeposit> for RTDStarTransactionTypeAppStoreDepositBuilder {
  fn as_ref(&self) -> &StarTransactionTypeAppStoreDeposit {
    &self.inner
  }
}

/// The transaction is a purchase of a product from a bot or a business account by the current user; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeBotInvoicePurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the bot or the business account user who created the invoice
  user_id: i64,
  /// Information about the bought product
  product_info: ProductInfo,
}

impl RObject for StarTransactionTypeBotInvoicePurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeBotInvoicePurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeBotInvoicePurchase {}

impl StarTransactionTypeBotInvoicePurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeBotInvoicePurchaseBuilder {
    let inner = StarTransactionTypeBotInvoicePurchase {
      td_name: "starTransactionTypeBotInvoicePurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeBotInvoicePurchaseBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn product_info(&self) -> &ProductInfo {
    &self.product_info
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeBotInvoicePurchaseBuilder {
  inner: StarTransactionTypeBotInvoicePurchase,
}

impl RTDStarTransactionTypeBotInvoicePurchaseBuilder {
  pub fn build(&self) -> StarTransactionTypeBotInvoicePurchase {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn product_info<T: AsRef<ProductInfo>>(&mut self, product_info: T) -> &mut Self {
    self.inner.product_info = product_info.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeBotInvoicePurchase> for StarTransactionTypeBotInvoicePurchase {
  fn as_ref(&self) -> &StarTransactionTypeBotInvoicePurchase {
    self
  }
}

impl AsRef<StarTransactionTypeBotInvoicePurchase>
  for RTDStarTransactionTypeBotInvoicePurchaseBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeBotInvoicePurchase {
    &self.inner
  }
}

/// The transaction is a sale of a product by the bot; relevant for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeBotInvoiceSale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who bought the product
  user_id: i64,
  /// Information about the bought product
  product_info: ProductInfo,
  /// Invoice payload
  invoice_payload: String,
  /// Information about the affiliate which received commission from the transaction; may be null if none
  affiliate: Option<AffiliateInfo>,
}

impl RObject for StarTransactionTypeBotInvoiceSale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeBotInvoiceSale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeBotInvoiceSale {}

impl StarTransactionTypeBotInvoiceSale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeBotInvoiceSaleBuilder {
    let inner = StarTransactionTypeBotInvoiceSale {
      td_name: "starTransactionTypeBotInvoiceSale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeBotInvoiceSaleBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn product_info(&self) -> &ProductInfo {
    &self.product_info
  }

  pub fn invoice_payload(&self) -> &String {
    &self.invoice_payload
  }

  pub fn affiliate(&self) -> &Option<AffiliateInfo> {
    &self.affiliate
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeBotInvoiceSaleBuilder {
  inner: StarTransactionTypeBotInvoiceSale,
}

impl RTDStarTransactionTypeBotInvoiceSaleBuilder {
  pub fn build(&self) -> StarTransactionTypeBotInvoiceSale {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn product_info<T: AsRef<ProductInfo>>(&mut self, product_info: T) -> &mut Self {
    self.inner.product_info = product_info.as_ref().clone();
    self
  }

  pub fn invoice_payload<T: AsRef<str>>(&mut self, invoice_payload: T) -> &mut Self {
    self.inner.invoice_payload = invoice_payload.as_ref().to_string();
    self
  }

  pub fn affiliate<T: AsRef<AffiliateInfo>>(&mut self, affiliate: T) -> &mut Self {
    self.inner.affiliate = Some(affiliate.as_ref().clone());
    self
  }
}

impl AsRef<StarTransactionTypeBotInvoiceSale> for StarTransactionTypeBotInvoiceSale {
  fn as_ref(&self) -> &StarTransactionTypeBotInvoiceSale {
    self
  }
}

impl AsRef<StarTransactionTypeBotInvoiceSale> for RTDStarTransactionTypeBotInvoiceSaleBuilder {
  fn as_ref(&self) -> &StarTransactionTypeBotInvoiceSale {
    &self.inner
  }
}

/// The transaction is a purchase of paid media from a bot or a business account by the current user; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeBotPaidMediaPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the bot or the business account user who sent the paid media
  user_id: i64,
  /// The bought media if the transaction wasn't refunded
  media: Vec<PaidMedia>,
}

impl RObject for StarTransactionTypeBotPaidMediaPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeBotPaidMediaPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeBotPaidMediaPurchase {}

impl StarTransactionTypeBotPaidMediaPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeBotPaidMediaPurchaseBuilder {
    let inner = StarTransactionTypeBotPaidMediaPurchase {
      td_name: "starTransactionTypeBotPaidMediaPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeBotPaidMediaPurchaseBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn media(&self) -> &Vec<PaidMedia> {
    &self.media
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeBotPaidMediaPurchaseBuilder {
  inner: StarTransactionTypeBotPaidMediaPurchase,
}

impl RTDStarTransactionTypeBotPaidMediaPurchaseBuilder {
  pub fn build(&self) -> StarTransactionTypeBotPaidMediaPurchase {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn media(&mut self, media: Vec<PaidMedia>) -> &mut Self {
    self.inner.media = media;
    self
  }
}

impl AsRef<StarTransactionTypeBotPaidMediaPurchase> for StarTransactionTypeBotPaidMediaPurchase {
  fn as_ref(&self) -> &StarTransactionTypeBotPaidMediaPurchase {
    self
  }
}

impl AsRef<StarTransactionTypeBotPaidMediaPurchase>
  for RTDStarTransactionTypeBotPaidMediaPurchaseBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeBotPaidMediaPurchase {
    &self.inner
  }
}

/// The transaction is a sale of paid media by the bot or a business account managed by the bot; relevant for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeBotPaidMediaSale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who bought the media
  user_id: i64,
  /// The bought media
  media: Vec<PaidMedia>,
  /// Bot-provided payload
  payload: String,
  /// Information about the affiliate which received commission from the transaction; may be null if none
  affiliate: Option<AffiliateInfo>,
}

impl RObject for StarTransactionTypeBotPaidMediaSale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeBotPaidMediaSale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeBotPaidMediaSale {}

impl StarTransactionTypeBotPaidMediaSale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeBotPaidMediaSaleBuilder {
    let inner = StarTransactionTypeBotPaidMediaSale {
      td_name: "starTransactionTypeBotPaidMediaSale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeBotPaidMediaSaleBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn media(&self) -> &Vec<PaidMedia> {
    &self.media
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }

  pub fn affiliate(&self) -> &Option<AffiliateInfo> {
    &self.affiliate
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeBotPaidMediaSaleBuilder {
  inner: StarTransactionTypeBotPaidMediaSale,
}

impl RTDStarTransactionTypeBotPaidMediaSaleBuilder {
  pub fn build(&self) -> StarTransactionTypeBotPaidMediaSale {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn media(&mut self, media: Vec<PaidMedia>) -> &mut Self {
    self.inner.media = media;
    self
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }

  pub fn affiliate<T: AsRef<AffiliateInfo>>(&mut self, affiliate: T) -> &mut Self {
    self.inner.affiliate = Some(affiliate.as_ref().clone());
    self
  }
}

impl AsRef<StarTransactionTypeBotPaidMediaSale> for StarTransactionTypeBotPaidMediaSale {
  fn as_ref(&self) -> &StarTransactionTypeBotPaidMediaSale {
    self
  }
}

impl AsRef<StarTransactionTypeBotPaidMediaSale> for RTDStarTransactionTypeBotPaidMediaSaleBuilder {
  fn as_ref(&self) -> &StarTransactionTypeBotPaidMediaSale {
    &self.inner
  }
}

/// The transaction is a purchase of a subscription from a bot or a business account by the current user; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeBotSubscriptionPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the bot or the business account user who created the subscription link
  user_id: i64,
  /// The number of seconds between consecutive Telegram Star debitings
  subscription_period: i64,
  /// Information about the bought subscription; may be null if no information about the subscription is available
  product_info: Option<ProductInfo>,
}

impl RObject for StarTransactionTypeBotSubscriptionPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeBotSubscriptionPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeBotSubscriptionPurchase {}

impl StarTransactionTypeBotSubscriptionPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeBotSubscriptionPurchaseBuilder {
    let inner = StarTransactionTypeBotSubscriptionPurchase {
      td_name: "starTransactionTypeBotSubscriptionPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeBotSubscriptionPurchaseBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn subscription_period(&self) -> i64 {
    self.subscription_period
  }

  pub fn product_info(&self) -> &Option<ProductInfo> {
    &self.product_info
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeBotSubscriptionPurchaseBuilder {
  inner: StarTransactionTypeBotSubscriptionPurchase,
}

impl RTDStarTransactionTypeBotSubscriptionPurchaseBuilder {
  pub fn build(&self) -> StarTransactionTypeBotSubscriptionPurchase {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn subscription_period(&mut self, subscription_period: i64) -> &mut Self {
    self.inner.subscription_period = subscription_period;
    self
  }

  pub fn product_info<T: AsRef<ProductInfo>>(&mut self, product_info: T) -> &mut Self {
    self.inner.product_info = Some(product_info.as_ref().clone());
    self
  }
}

impl AsRef<StarTransactionTypeBotSubscriptionPurchase>
  for StarTransactionTypeBotSubscriptionPurchase
{
  fn as_ref(&self) -> &StarTransactionTypeBotSubscriptionPurchase {
    self
  }
}

impl AsRef<StarTransactionTypeBotSubscriptionPurchase>
  for RTDStarTransactionTypeBotSubscriptionPurchaseBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeBotSubscriptionPurchase {
    &self.inner
  }
}

/// The transaction is a sale of a subscription by the bot; relevant for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeBotSubscriptionSale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who bought the subscription
  user_id: i64,
  /// The number of seconds between consecutive Telegram Star debitings
  subscription_period: i64,
  /// Information about the bought subscription; may be null if no information about the subscription is available
  product_info: Option<ProductInfo>,
  /// Invoice payload
  invoice_payload: String,
  /// Information about the affiliate which received commission from the transaction; may be null if none
  affiliate: Option<AffiliateInfo>,
}

impl RObject for StarTransactionTypeBotSubscriptionSale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeBotSubscriptionSale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeBotSubscriptionSale {}

impl StarTransactionTypeBotSubscriptionSale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeBotSubscriptionSaleBuilder {
    let inner = StarTransactionTypeBotSubscriptionSale {
      td_name: "starTransactionTypeBotSubscriptionSale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeBotSubscriptionSaleBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn subscription_period(&self) -> i64 {
    self.subscription_period
  }

  pub fn product_info(&self) -> &Option<ProductInfo> {
    &self.product_info
  }

  pub fn invoice_payload(&self) -> &String {
    &self.invoice_payload
  }

  pub fn affiliate(&self) -> &Option<AffiliateInfo> {
    &self.affiliate
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeBotSubscriptionSaleBuilder {
  inner: StarTransactionTypeBotSubscriptionSale,
}

impl RTDStarTransactionTypeBotSubscriptionSaleBuilder {
  pub fn build(&self) -> StarTransactionTypeBotSubscriptionSale {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn subscription_period(&mut self, subscription_period: i64) -> &mut Self {
    self.inner.subscription_period = subscription_period;
    self
  }

  pub fn product_info<T: AsRef<ProductInfo>>(&mut self, product_info: T) -> &mut Self {
    self.inner.product_info = Some(product_info.as_ref().clone());
    self
  }

  pub fn invoice_payload<T: AsRef<str>>(&mut self, invoice_payload: T) -> &mut Self {
    self.inner.invoice_payload = invoice_payload.as_ref().to_string();
    self
  }

  pub fn affiliate<T: AsRef<AffiliateInfo>>(&mut self, affiliate: T) -> &mut Self {
    self.inner.affiliate = Some(affiliate.as_ref().clone());
    self
  }
}

impl AsRef<StarTransactionTypeBotSubscriptionSale> for StarTransactionTypeBotSubscriptionSale {
  fn as_ref(&self) -> &StarTransactionTypeBotSubscriptionSale {
    self
  }
}

impl AsRef<StarTransactionTypeBotSubscriptionSale>
  for RTDStarTransactionTypeBotSubscriptionSaleBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeBotSubscriptionSale {
    &self.inner
  }
}

/// The transaction is a transfer of Telegram Stars from a business account; relevant for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeBusinessBotTransferReceive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who sent Telegram Stars
  user_id: i64,
}

impl RObject for StarTransactionTypeBusinessBotTransferReceive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeBusinessBotTransferReceive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeBusinessBotTransferReceive {}

impl StarTransactionTypeBusinessBotTransferReceive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeBusinessBotTransferReceiveBuilder {
    let inner = StarTransactionTypeBusinessBotTransferReceive {
      td_name: "starTransactionTypeBusinessBotTransferReceive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeBusinessBotTransferReceiveBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeBusinessBotTransferReceiveBuilder {
  inner: StarTransactionTypeBusinessBotTransferReceive,
}

impl RTDStarTransactionTypeBusinessBotTransferReceiveBuilder {
  pub fn build(&self) -> StarTransactionTypeBusinessBotTransferReceive {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }
}

impl AsRef<StarTransactionTypeBusinessBotTransferReceive>
  for StarTransactionTypeBusinessBotTransferReceive
{
  fn as_ref(&self) -> &StarTransactionTypeBusinessBotTransferReceive {
    self
  }
}

impl AsRef<StarTransactionTypeBusinessBotTransferReceive>
  for RTDStarTransactionTypeBusinessBotTransferReceiveBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeBusinessBotTransferReceive {
    &self.inner
  }
}

/// The transaction is a transfer of Telegram Stars to a business bot; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeBusinessBotTransferSend {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the bot that received Telegram Stars
  user_id: i64,
}

impl RObject for StarTransactionTypeBusinessBotTransferSend {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeBusinessBotTransferSend"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeBusinessBotTransferSend {}

impl StarTransactionTypeBusinessBotTransferSend {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeBusinessBotTransferSendBuilder {
    let inner = StarTransactionTypeBusinessBotTransferSend {
      td_name: "starTransactionTypeBusinessBotTransferSend".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeBusinessBotTransferSendBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeBusinessBotTransferSendBuilder {
  inner: StarTransactionTypeBusinessBotTransferSend,
}

impl RTDStarTransactionTypeBusinessBotTransferSendBuilder {
  pub fn build(&self) -> StarTransactionTypeBusinessBotTransferSend {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }
}

impl AsRef<StarTransactionTypeBusinessBotTransferSend>
  for StarTransactionTypeBusinessBotTransferSend
{
  fn as_ref(&self) -> &StarTransactionTypeBusinessBotTransferSend {
    self
  }
}

impl AsRef<StarTransactionTypeBusinessBotTransferSend>
  for RTDStarTransactionTypeBusinessBotTransferSendBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeBusinessBotTransferSend {
    &self.inner
  }
}

/// The transaction is a purchase of paid media from a channel by the current user; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeChannelPaidMediaPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the channel chat that sent the paid media
  chat_id: i64,
  /// Identifier of the corresponding message with paid media; may be 0 or an identifier of a deleted message
  message_id: i64,
  /// The bought media if the transaction wasn't refunded
  media: Vec<PaidMedia>,
}

impl RObject for StarTransactionTypeChannelPaidMediaPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeChannelPaidMediaPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeChannelPaidMediaPurchase {}

impl StarTransactionTypeChannelPaidMediaPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeChannelPaidMediaPurchaseBuilder {
    let inner = StarTransactionTypeChannelPaidMediaPurchase {
      td_name: "starTransactionTypeChannelPaidMediaPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeChannelPaidMediaPurchaseBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn media(&self) -> &Vec<PaidMedia> {
    &self.media
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeChannelPaidMediaPurchaseBuilder {
  inner: StarTransactionTypeChannelPaidMediaPurchase,
}

impl RTDStarTransactionTypeChannelPaidMediaPurchaseBuilder {
  pub fn build(&self) -> StarTransactionTypeChannelPaidMediaPurchase {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn media(&mut self, media: Vec<PaidMedia>) -> &mut Self {
    self.inner.media = media;
    self
  }
}

impl AsRef<StarTransactionTypeChannelPaidMediaPurchase>
  for StarTransactionTypeChannelPaidMediaPurchase
{
  fn as_ref(&self) -> &StarTransactionTypeChannelPaidMediaPurchase {
    self
  }
}

impl AsRef<StarTransactionTypeChannelPaidMediaPurchase>
  for RTDStarTransactionTypeChannelPaidMediaPurchaseBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeChannelPaidMediaPurchase {
    &self.inner
  }
}

/// The transaction is a sale of paid media by the channel chat; relevant for channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeChannelPaidMediaSale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who bought the media
  user_id: i64,
  /// Identifier of the corresponding message with paid media; may be 0 or an identifier of a deleted message
  message_id: i64,
  /// The bought media
  media: Vec<PaidMedia>,
}

impl RObject for StarTransactionTypeChannelPaidMediaSale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeChannelPaidMediaSale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeChannelPaidMediaSale {}

impl StarTransactionTypeChannelPaidMediaSale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeChannelPaidMediaSaleBuilder {
    let inner = StarTransactionTypeChannelPaidMediaSale {
      td_name: "starTransactionTypeChannelPaidMediaSale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeChannelPaidMediaSaleBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }

  pub fn media(&self) -> &Vec<PaidMedia> {
    &self.media
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeChannelPaidMediaSaleBuilder {
  inner: StarTransactionTypeChannelPaidMediaSale,
}

impl RTDStarTransactionTypeChannelPaidMediaSaleBuilder {
  pub fn build(&self) -> StarTransactionTypeChannelPaidMediaSale {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

  pub fn media(&mut self, media: Vec<PaidMedia>) -> &mut Self {
    self.inner.media = media;
    self
  }
}

impl AsRef<StarTransactionTypeChannelPaidMediaSale> for StarTransactionTypeChannelPaidMediaSale {
  fn as_ref(&self) -> &StarTransactionTypeChannelPaidMediaSale {
    self
  }
}

impl AsRef<StarTransactionTypeChannelPaidMediaSale>
  for RTDStarTransactionTypeChannelPaidMediaSaleBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeChannelPaidMediaSale {
    &self.inner
  }
}

/// The transaction is a receiving of a paid reaction to a message by the channel chat; relevant for channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeChannelPaidReactionReceive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who added the paid reaction
  user_id: i64,
  /// Identifier of the reacted message; may be 0 or an identifier of a deleted message
  message_id: i64,
}

impl RObject for StarTransactionTypeChannelPaidReactionReceive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeChannelPaidReactionReceive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeChannelPaidReactionReceive {}

impl StarTransactionTypeChannelPaidReactionReceive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeChannelPaidReactionReceiveBuilder {
    let inner = StarTransactionTypeChannelPaidReactionReceive {
      td_name: "starTransactionTypeChannelPaidReactionReceive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeChannelPaidReactionReceiveBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeChannelPaidReactionReceiveBuilder {
  inner: StarTransactionTypeChannelPaidReactionReceive,
}

impl RTDStarTransactionTypeChannelPaidReactionReceiveBuilder {
  pub fn build(&self) -> StarTransactionTypeChannelPaidReactionReceive {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }
}

impl AsRef<StarTransactionTypeChannelPaidReactionReceive>
  for StarTransactionTypeChannelPaidReactionReceive
{
  fn as_ref(&self) -> &StarTransactionTypeChannelPaidReactionReceive {
    self
  }
}

impl AsRef<StarTransactionTypeChannelPaidReactionReceive>
  for RTDStarTransactionTypeChannelPaidReactionReceiveBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeChannelPaidReactionReceive {
    &self.inner
  }
}

/// The transaction is a sending of a paid reaction to a message in a channel chat by the current user; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeChannelPaidReactionSend {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the channel chat
  chat_id: i64,
  /// Identifier of the reacted message; may be 0 or an identifier of a deleted message
  message_id: i64,
}

impl RObject for StarTransactionTypeChannelPaidReactionSend {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeChannelPaidReactionSend"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeChannelPaidReactionSend {}

impl StarTransactionTypeChannelPaidReactionSend {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeChannelPaidReactionSendBuilder {
    let inner = StarTransactionTypeChannelPaidReactionSend {
      td_name: "starTransactionTypeChannelPaidReactionSend".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeChannelPaidReactionSendBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_id(&self) -> i64 {
    self.message_id
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeChannelPaidReactionSendBuilder {
  inner: StarTransactionTypeChannelPaidReactionSend,
}

impl RTDStarTransactionTypeChannelPaidReactionSendBuilder {
  pub fn build(&self) -> StarTransactionTypeChannelPaidReactionSend {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }
}

impl AsRef<StarTransactionTypeChannelPaidReactionSend>
  for StarTransactionTypeChannelPaidReactionSend
{
  fn as_ref(&self) -> &StarTransactionTypeChannelPaidReactionSend {
    self
  }
}

impl AsRef<StarTransactionTypeChannelPaidReactionSend>
  for RTDStarTransactionTypeChannelPaidReactionSendBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeChannelPaidReactionSend {
    &self.inner
  }
}

/// The transaction is a purchase of a subscription to a channel chat by the current user; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeChannelSubscriptionPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the channel chat that created the subscription
  chat_id: i64,
  /// The number of seconds between consecutive Telegram Star debitings
  subscription_period: i64,
}

impl RObject for StarTransactionTypeChannelSubscriptionPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeChannelSubscriptionPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeChannelSubscriptionPurchase {}

impl StarTransactionTypeChannelSubscriptionPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeChannelSubscriptionPurchaseBuilder {
    let inner = StarTransactionTypeChannelSubscriptionPurchase {
      td_name: "starTransactionTypeChannelSubscriptionPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeChannelSubscriptionPurchaseBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn subscription_period(&self) -> i64 {
    self.subscription_period
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeChannelSubscriptionPurchaseBuilder {
  inner: StarTransactionTypeChannelSubscriptionPurchase,
}

impl RTDStarTransactionTypeChannelSubscriptionPurchaseBuilder {
  pub fn build(&self) -> StarTransactionTypeChannelSubscriptionPurchase {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn subscription_period(&mut self, subscription_period: i64) -> &mut Self {
    self.inner.subscription_period = subscription_period;
    self
  }
}

impl AsRef<StarTransactionTypeChannelSubscriptionPurchase>
  for StarTransactionTypeChannelSubscriptionPurchase
{
  fn as_ref(&self) -> &StarTransactionTypeChannelSubscriptionPurchase {
    self
  }
}

impl AsRef<StarTransactionTypeChannelSubscriptionPurchase>
  for RTDStarTransactionTypeChannelSubscriptionPurchaseBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeChannelSubscriptionPurchase {
    &self.inner
  }
}

/// The transaction is a sale of a subscription by the channel chat; relevant for channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeChannelSubscriptionSale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who bought the subscription
  user_id: i64,
  /// The number of seconds between consecutive Telegram Star debitings
  subscription_period: i64,
}

impl RObject for StarTransactionTypeChannelSubscriptionSale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeChannelSubscriptionSale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeChannelSubscriptionSale {}

impl StarTransactionTypeChannelSubscriptionSale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeChannelSubscriptionSaleBuilder {
    let inner = StarTransactionTypeChannelSubscriptionSale {
      td_name: "starTransactionTypeChannelSubscriptionSale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeChannelSubscriptionSaleBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn subscription_period(&self) -> i64 {
    self.subscription_period
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeChannelSubscriptionSaleBuilder {
  inner: StarTransactionTypeChannelSubscriptionSale,
}

impl RTDStarTransactionTypeChannelSubscriptionSaleBuilder {
  pub fn build(&self) -> StarTransactionTypeChannelSubscriptionSale {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn subscription_period(&mut self, subscription_period: i64) -> &mut Self {
    self.inner.subscription_period = subscription_period;
    self
  }
}

impl AsRef<StarTransactionTypeChannelSubscriptionSale>
  for StarTransactionTypeChannelSubscriptionSale
{
  fn as_ref(&self) -> &StarTransactionTypeChannelSubscriptionSale {
    self
  }
}

impl AsRef<StarTransactionTypeChannelSubscriptionSale>
  for RTDStarTransactionTypeChannelSubscriptionSaleBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeChannelSubscriptionSale {
    &self.inner
  }
}

/// The transaction is a deposit of Telegram Stars from Fragment; relevant for regular users and bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeFragmentDeposit {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StarTransactionTypeFragmentDeposit {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeFragmentDeposit"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeFragmentDeposit {}

impl StarTransactionTypeFragmentDeposit {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeFragmentDepositBuilder {
    let inner = StarTransactionTypeFragmentDeposit {
      td_name: "starTransactionTypeFragmentDeposit".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStarTransactionTypeFragmentDepositBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeFragmentDepositBuilder {
  inner: StarTransactionTypeFragmentDeposit,
}

impl RTDStarTransactionTypeFragmentDepositBuilder {
  pub fn build(&self) -> StarTransactionTypeFragmentDeposit {
    self.inner.clone()
  }
}

impl AsRef<StarTransactionTypeFragmentDeposit> for StarTransactionTypeFragmentDeposit {
  fn as_ref(&self) -> &StarTransactionTypeFragmentDeposit {
    self
  }
}

impl AsRef<StarTransactionTypeFragmentDeposit> for RTDStarTransactionTypeFragmentDepositBuilder {
  fn as_ref(&self) -> &StarTransactionTypeFragmentDeposit {
    &self.inner
  }
}

/// The transaction is a withdrawal of earned Telegram Stars to Fragment; relevant for regular users, bots, supergroup and channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeFragmentWithdrawal {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// State of the withdrawal; may be null for refunds from Fragment
  withdrawal_state: Option<RevenueWithdrawalState>,
}

impl RObject for StarTransactionTypeFragmentWithdrawal {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeFragmentWithdrawal"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeFragmentWithdrawal {}

impl StarTransactionTypeFragmentWithdrawal {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeFragmentWithdrawalBuilder {
    let inner = StarTransactionTypeFragmentWithdrawal {
      td_name: "starTransactionTypeFragmentWithdrawal".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeFragmentWithdrawalBuilder { inner }
  }

  pub fn withdrawal_state(&self) -> &Option<RevenueWithdrawalState> {
    &self.withdrawal_state
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeFragmentWithdrawalBuilder {
  inner: StarTransactionTypeFragmentWithdrawal,
}

impl RTDStarTransactionTypeFragmentWithdrawalBuilder {
  pub fn build(&self) -> StarTransactionTypeFragmentWithdrawal {
    self.inner.clone()
  }

  pub fn withdrawal_state<T: AsRef<RevenueWithdrawalState>>(
    &mut self,
    withdrawal_state: T,
  ) -> &mut Self {
    self.inner.withdrawal_state = Some(withdrawal_state.as_ref().clone());
    self
  }
}

impl AsRef<StarTransactionTypeFragmentWithdrawal> for StarTransactionTypeFragmentWithdrawal {
  fn as_ref(&self) -> &StarTransactionTypeFragmentWithdrawal {
    self
  }
}

impl AsRef<StarTransactionTypeFragmentWithdrawal>
  for RTDStarTransactionTypeFragmentWithdrawalBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeFragmentWithdrawal {
    &self.inner
  }
}

/// The transaction is a bid on a gift auction; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGiftAuctionBid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who will receive the gift
  owner_id: MessageSender,
  /// The gift
  gift: Gift,
}

impl RObject for StarTransactionTypeGiftAuctionBid {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGiftAuctionBid"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGiftAuctionBid {}

impl StarTransactionTypeGiftAuctionBid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGiftAuctionBidBuilder {
    let inner = StarTransactionTypeGiftAuctionBid {
      td_name: "starTransactionTypeGiftAuctionBid".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeGiftAuctionBidBuilder { inner }
  }

  pub fn owner_id(&self) -> &MessageSender {
    &self.owner_id
  }

  pub fn gift(&self) -> &Gift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGiftAuctionBidBuilder {
  inner: StarTransactionTypeGiftAuctionBid,
}

impl RTDStarTransactionTypeGiftAuctionBidBuilder {
  pub fn build(&self) -> StarTransactionTypeGiftAuctionBid {
    self.inner.clone()
  }

  pub fn owner_id<T: AsRef<MessageSender>>(&mut self, owner_id: T) -> &mut Self {
    self.inner.owner_id = owner_id.as_ref().clone();
    self
  }

  pub fn gift<T: AsRef<Gift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeGiftAuctionBid> for StarTransactionTypeGiftAuctionBid {
  fn as_ref(&self) -> &StarTransactionTypeGiftAuctionBid {
    self
  }
}

impl AsRef<StarTransactionTypeGiftAuctionBid> for RTDStarTransactionTypeGiftAuctionBidBuilder {
  fn as_ref(&self) -> &StarTransactionTypeGiftAuctionBid {
    &self.inner
  }
}

/// The transaction is a drop of original details of an upgraded gift; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGiftOriginalDetailsDrop {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user or the channel that owns the gift
  owner_id: MessageSender,
  /// The gift
  gift: UpgradedGift,
}

impl RObject for StarTransactionTypeGiftOriginalDetailsDrop {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGiftOriginalDetailsDrop"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGiftOriginalDetailsDrop {}

impl StarTransactionTypeGiftOriginalDetailsDrop {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGiftOriginalDetailsDropBuilder {
    let inner = StarTransactionTypeGiftOriginalDetailsDrop {
      td_name: "starTransactionTypeGiftOriginalDetailsDrop".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeGiftOriginalDetailsDropBuilder { inner }
  }

  pub fn owner_id(&self) -> &MessageSender {
    &self.owner_id
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGiftOriginalDetailsDropBuilder {
  inner: StarTransactionTypeGiftOriginalDetailsDrop,
}

impl RTDStarTransactionTypeGiftOriginalDetailsDropBuilder {
  pub fn build(&self) -> StarTransactionTypeGiftOriginalDetailsDrop {
    self.inner.clone()
  }

  pub fn owner_id<T: AsRef<MessageSender>>(&mut self, owner_id: T) -> &mut Self {
    self.inner.owner_id = owner_id.as_ref().clone();
    self
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeGiftOriginalDetailsDrop>
  for StarTransactionTypeGiftOriginalDetailsDrop
{
  fn as_ref(&self) -> &StarTransactionTypeGiftOriginalDetailsDrop {
    self
  }
}

impl AsRef<StarTransactionTypeGiftOriginalDetailsDrop>
  for RTDStarTransactionTypeGiftOriginalDetailsDropBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeGiftOriginalDetailsDrop {
    &self.inner
  }
}

/// The transaction is a purchase of a regular gift; relevant for regular users and bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGiftPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user or the channel that received the gift
  owner_id: MessageSender,
  /// The gift
  gift: Gift,
}

impl RObject for StarTransactionTypeGiftPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGiftPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGiftPurchase {}

impl StarTransactionTypeGiftPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGiftPurchaseBuilder {
    let inner = StarTransactionTypeGiftPurchase {
      td_name: "starTransactionTypeGiftPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeGiftPurchaseBuilder { inner }
  }

  pub fn owner_id(&self) -> &MessageSender {
    &self.owner_id
  }

  pub fn gift(&self) -> &Gift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGiftPurchaseBuilder {
  inner: StarTransactionTypeGiftPurchase,
}

impl RTDStarTransactionTypeGiftPurchaseBuilder {
  pub fn build(&self) -> StarTransactionTypeGiftPurchase {
    self.inner.clone()
  }

  pub fn owner_id<T: AsRef<MessageSender>>(&mut self, owner_id: T) -> &mut Self {
    self.inner.owner_id = owner_id.as_ref().clone();
    self
  }

  pub fn gift<T: AsRef<Gift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeGiftPurchase> for StarTransactionTypeGiftPurchase {
  fn as_ref(&self) -> &StarTransactionTypeGiftPurchase {
    self
  }
}

impl AsRef<StarTransactionTypeGiftPurchase> for RTDStarTransactionTypeGiftPurchaseBuilder {
  fn as_ref(&self) -> &StarTransactionTypeGiftPurchase {
    &self.inner
  }
}

/// The transaction is an offer of gift purchase; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGiftPurchaseOffer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: UpgradedGift,
}

impl RObject for StarTransactionTypeGiftPurchaseOffer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGiftPurchaseOffer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGiftPurchaseOffer {}

impl StarTransactionTypeGiftPurchaseOffer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGiftPurchaseOfferBuilder {
    let inner = StarTransactionTypeGiftPurchaseOffer {
      td_name: "starTransactionTypeGiftPurchaseOffer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeGiftPurchaseOfferBuilder { inner }
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGiftPurchaseOfferBuilder {
  inner: StarTransactionTypeGiftPurchaseOffer,
}

impl RTDStarTransactionTypeGiftPurchaseOfferBuilder {
  pub fn build(&self) -> StarTransactionTypeGiftPurchaseOffer {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeGiftPurchaseOffer> for StarTransactionTypeGiftPurchaseOffer {
  fn as_ref(&self) -> &StarTransactionTypeGiftPurchaseOffer {
    self
  }
}

impl AsRef<StarTransactionTypeGiftPurchaseOffer>
  for RTDStarTransactionTypeGiftPurchaseOfferBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeGiftPurchaseOffer {
    &self.inner
  }
}

/// The transaction is a sale of a received gift; relevant for regular users and channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGiftSale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who sent the gift
  user_id: i64,
  /// The gift
  gift: Gift,
}

impl RObject for StarTransactionTypeGiftSale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGiftSale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGiftSale {}

impl StarTransactionTypeGiftSale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGiftSaleBuilder {
    let inner = StarTransactionTypeGiftSale {
      td_name: "starTransactionTypeGiftSale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeGiftSaleBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn gift(&self) -> &Gift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGiftSaleBuilder {
  inner: StarTransactionTypeGiftSale,
}

impl RTDStarTransactionTypeGiftSaleBuilder {
  pub fn build(&self) -> StarTransactionTypeGiftSale {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn gift<T: AsRef<Gift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeGiftSale> for StarTransactionTypeGiftSale {
  fn as_ref(&self) -> &StarTransactionTypeGiftSale {
    self
  }
}

impl AsRef<StarTransactionTypeGiftSale> for RTDStarTransactionTypeGiftSaleBuilder {
  fn as_ref(&self) -> &StarTransactionTypeGiftSale {
    &self.inner
  }
}

/// The transaction is a transfer of an upgraded gift; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGiftTransfer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user or the channel that received the gift
  owner_id: MessageSender,
  /// The gift
  gift: UpgradedGift,
}

impl RObject for StarTransactionTypeGiftTransfer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGiftTransfer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGiftTransfer {}

impl StarTransactionTypeGiftTransfer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGiftTransferBuilder {
    let inner = StarTransactionTypeGiftTransfer {
      td_name: "starTransactionTypeGiftTransfer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeGiftTransferBuilder { inner }
  }

  pub fn owner_id(&self) -> &MessageSender {
    &self.owner_id
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGiftTransferBuilder {
  inner: StarTransactionTypeGiftTransfer,
}

impl RTDStarTransactionTypeGiftTransferBuilder {
  pub fn build(&self) -> StarTransactionTypeGiftTransfer {
    self.inner.clone()
  }

  pub fn owner_id<T: AsRef<MessageSender>>(&mut self, owner_id: T) -> &mut Self {
    self.inner.owner_id = owner_id.as_ref().clone();
    self
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeGiftTransfer> for StarTransactionTypeGiftTransfer {
  fn as_ref(&self) -> &StarTransactionTypeGiftTransfer {
    self
  }
}

impl AsRef<StarTransactionTypeGiftTransfer> for RTDStarTransactionTypeGiftTransferBuilder {
  fn as_ref(&self) -> &StarTransactionTypeGiftTransfer {
    &self.inner
  }
}

/// The transaction is an upgrade of a gift; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGiftUpgrade {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who initially sent the gift
  user_id: i64,
  /// The upgraded gift
  gift: UpgradedGift,
}

impl RObject for StarTransactionTypeGiftUpgrade {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGiftUpgrade"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGiftUpgrade {}

impl StarTransactionTypeGiftUpgrade {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGiftUpgradeBuilder {
    let inner = StarTransactionTypeGiftUpgrade {
      td_name: "starTransactionTypeGiftUpgrade".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeGiftUpgradeBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGiftUpgradeBuilder {
  inner: StarTransactionTypeGiftUpgrade,
}

impl RTDStarTransactionTypeGiftUpgradeBuilder {
  pub fn build(&self) -> StarTransactionTypeGiftUpgrade {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeGiftUpgrade> for StarTransactionTypeGiftUpgrade {
  fn as_ref(&self) -> &StarTransactionTypeGiftUpgrade {
    self
  }
}

impl AsRef<StarTransactionTypeGiftUpgrade> for RTDStarTransactionTypeGiftUpgradeBuilder {
  fn as_ref(&self) -> &StarTransactionTypeGiftUpgrade {
    &self.inner
  }
}

/// The transaction is a purchase of an upgrade of a gift owned by another user or channel; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGiftUpgradePurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Owner of the upgraded gift
  owner_id: MessageSender,
  /// The gift
  gift: Gift,
}

impl RObject for StarTransactionTypeGiftUpgradePurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGiftUpgradePurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGiftUpgradePurchase {}

impl StarTransactionTypeGiftUpgradePurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGiftUpgradePurchaseBuilder {
    let inner = StarTransactionTypeGiftUpgradePurchase {
      td_name: "starTransactionTypeGiftUpgradePurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeGiftUpgradePurchaseBuilder { inner }
  }

  pub fn owner_id(&self) -> &MessageSender {
    &self.owner_id
  }

  pub fn gift(&self) -> &Gift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGiftUpgradePurchaseBuilder {
  inner: StarTransactionTypeGiftUpgradePurchase,
}

impl RTDStarTransactionTypeGiftUpgradePurchaseBuilder {
  pub fn build(&self) -> StarTransactionTypeGiftUpgradePurchase {
    self.inner.clone()
  }

  pub fn owner_id<T: AsRef<MessageSender>>(&mut self, owner_id: T) -> &mut Self {
    self.inner.owner_id = owner_id.as_ref().clone();
    self
  }

  pub fn gift<T: AsRef<Gift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeGiftUpgradePurchase> for StarTransactionTypeGiftUpgradePurchase {
  fn as_ref(&self) -> &StarTransactionTypeGiftUpgradePurchase {
    self
  }
}

impl AsRef<StarTransactionTypeGiftUpgradePurchase>
  for RTDStarTransactionTypeGiftUpgradePurchaseBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeGiftUpgradePurchase {
    &self.inner
  }
}

/// The transaction is a deposit of Telegram Stars from a giveaway; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGiveawayDeposit {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of a supergroup or a channel chat that created the giveaway
  chat_id: i64,
  /// Identifier of the message with the giveaway; may be 0 or an identifier of a deleted message
  giveaway_message_id: i64,
}

impl RObject for StarTransactionTypeGiveawayDeposit {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGiveawayDeposit"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGiveawayDeposit {}

impl StarTransactionTypeGiveawayDeposit {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGiveawayDepositBuilder {
    let inner = StarTransactionTypeGiveawayDeposit {
      td_name: "starTransactionTypeGiveawayDeposit".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeGiveawayDepositBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn giveaway_message_id(&self) -> i64 {
    self.giveaway_message_id
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGiveawayDepositBuilder {
  inner: StarTransactionTypeGiveawayDeposit,
}

impl RTDStarTransactionTypeGiveawayDepositBuilder {
  pub fn build(&self) -> StarTransactionTypeGiveawayDeposit {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn giveaway_message_id(&mut self, giveaway_message_id: i64) -> &mut Self {
    self.inner.giveaway_message_id = giveaway_message_id;
    self
  }
}

impl AsRef<StarTransactionTypeGiveawayDeposit> for StarTransactionTypeGiveawayDeposit {
  fn as_ref(&self) -> &StarTransactionTypeGiveawayDeposit {
    self
  }
}

impl AsRef<StarTransactionTypeGiveawayDeposit> for RTDStarTransactionTypeGiveawayDepositBuilder {
  fn as_ref(&self) -> &StarTransactionTypeGiveawayDeposit {
    &self.inner
  }
}

/// The transaction is a deposit of Telegram Stars from Google Play; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeGooglePlayDeposit {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StarTransactionTypeGooglePlayDeposit {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeGooglePlayDeposit"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeGooglePlayDeposit {}

impl StarTransactionTypeGooglePlayDeposit {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeGooglePlayDepositBuilder {
    let inner = StarTransactionTypeGooglePlayDeposit {
      td_name: "starTransactionTypeGooglePlayDeposit".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStarTransactionTypeGooglePlayDepositBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeGooglePlayDepositBuilder {
  inner: StarTransactionTypeGooglePlayDeposit,
}

impl RTDStarTransactionTypeGooglePlayDepositBuilder {
  pub fn build(&self) -> StarTransactionTypeGooglePlayDeposit {
    self.inner.clone()
  }
}

impl AsRef<StarTransactionTypeGooglePlayDeposit> for StarTransactionTypeGooglePlayDeposit {
  fn as_ref(&self) -> &StarTransactionTypeGooglePlayDeposit {
    self
  }
}

impl AsRef<StarTransactionTypeGooglePlayDeposit>
  for RTDStarTransactionTypeGooglePlayDepositBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeGooglePlayDeposit {
    &self.inner
  }
}

/// The transaction is a receiving of a paid group call message; relevant for regular users and channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypePaidGroupCallMessageReceive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the sender of the message
  sender_id: MessageSender,
  /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars paid for message sending
  commission_per_mille: i64,
  /// The Telegram Star amount that was received by Telegram; can be negative for refunds
  commission_star_amount: StarAmount,
}

impl RObject for StarTransactionTypePaidGroupCallMessageReceive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypePaidGroupCallMessageReceive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypePaidGroupCallMessageReceive {}

impl StarTransactionTypePaidGroupCallMessageReceive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypePaidGroupCallMessageReceiveBuilder {
    let inner = StarTransactionTypePaidGroupCallMessageReceive {
      td_name: "starTransactionTypePaidGroupCallMessageReceive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypePaidGroupCallMessageReceiveBuilder { inner }
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn commission_per_mille(&self) -> i64 {
    self.commission_per_mille
  }

  pub fn commission_star_amount(&self) -> &StarAmount {
    &self.commission_star_amount
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypePaidGroupCallMessageReceiveBuilder {
  inner: StarTransactionTypePaidGroupCallMessageReceive,
}

impl RTDStarTransactionTypePaidGroupCallMessageReceiveBuilder {
  pub fn build(&self) -> StarTransactionTypePaidGroupCallMessageReceive {
    self.inner.clone()
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = sender_id.as_ref().clone();
    self
  }

  pub fn commission_per_mille(&mut self, commission_per_mille: i64) -> &mut Self {
    self.inner.commission_per_mille = commission_per_mille;
    self
  }

  pub fn commission_star_amount<T: AsRef<StarAmount>>(
    &mut self,
    commission_star_amount: T,
  ) -> &mut Self {
    self.inner.commission_star_amount = commission_star_amount.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypePaidGroupCallMessageReceive>
  for StarTransactionTypePaidGroupCallMessageReceive
{
  fn as_ref(&self) -> &StarTransactionTypePaidGroupCallMessageReceive {
    self
  }
}

impl AsRef<StarTransactionTypePaidGroupCallMessageReceive>
  for RTDStarTransactionTypePaidGroupCallMessageReceiveBuilder
{
  fn as_ref(&self) -> &StarTransactionTypePaidGroupCallMessageReceive {
    &self.inner
  }
}

/// The transaction is a sending of a paid group call message; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypePaidGroupCallMessageSend {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that received the payment
  chat_id: i64,
}

impl RObject for StarTransactionTypePaidGroupCallMessageSend {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypePaidGroupCallMessageSend"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypePaidGroupCallMessageSend {}

impl StarTransactionTypePaidGroupCallMessageSend {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypePaidGroupCallMessageSendBuilder {
    let inner = StarTransactionTypePaidGroupCallMessageSend {
      td_name: "starTransactionTypePaidGroupCallMessageSend".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypePaidGroupCallMessageSendBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypePaidGroupCallMessageSendBuilder {
  inner: StarTransactionTypePaidGroupCallMessageSend,
}

impl RTDStarTransactionTypePaidGroupCallMessageSendBuilder {
  pub fn build(&self) -> StarTransactionTypePaidGroupCallMessageSend {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<StarTransactionTypePaidGroupCallMessageSend>
  for StarTransactionTypePaidGroupCallMessageSend
{
  fn as_ref(&self) -> &StarTransactionTypePaidGroupCallMessageSend {
    self
  }
}

impl AsRef<StarTransactionTypePaidGroupCallMessageSend>
  for RTDStarTransactionTypePaidGroupCallMessageSendBuilder
{
  fn as_ref(&self) -> &StarTransactionTypePaidGroupCallMessageSend {
    &self.inner
  }
}

/// The transaction is a receiving of a paid group call reaction; relevant for regular users and channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypePaidGroupCallReactionReceive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the sender of the reaction
  sender_id: MessageSender,
  /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars paid for reaction sending
  commission_per_mille: i64,
  /// The Telegram Star amount that was received by Telegram; can be negative for refunds
  commission_star_amount: StarAmount,
}

impl RObject for StarTransactionTypePaidGroupCallReactionReceive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypePaidGroupCallReactionReceive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypePaidGroupCallReactionReceive {}

impl StarTransactionTypePaidGroupCallReactionReceive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypePaidGroupCallReactionReceiveBuilder {
    let inner = StarTransactionTypePaidGroupCallReactionReceive {
      td_name: "starTransactionTypePaidGroupCallReactionReceive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypePaidGroupCallReactionReceiveBuilder { inner }
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn commission_per_mille(&self) -> i64 {
    self.commission_per_mille
  }

  pub fn commission_star_amount(&self) -> &StarAmount {
    &self.commission_star_amount
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypePaidGroupCallReactionReceiveBuilder {
  inner: StarTransactionTypePaidGroupCallReactionReceive,
}

impl RTDStarTransactionTypePaidGroupCallReactionReceiveBuilder {
  pub fn build(&self) -> StarTransactionTypePaidGroupCallReactionReceive {
    self.inner.clone()
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = sender_id.as_ref().clone();
    self
  }

  pub fn commission_per_mille(&mut self, commission_per_mille: i64) -> &mut Self {
    self.inner.commission_per_mille = commission_per_mille;
    self
  }

  pub fn commission_star_amount<T: AsRef<StarAmount>>(
    &mut self,
    commission_star_amount: T,
  ) -> &mut Self {
    self.inner.commission_star_amount = commission_star_amount.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypePaidGroupCallReactionReceive>
  for StarTransactionTypePaidGroupCallReactionReceive
{
  fn as_ref(&self) -> &StarTransactionTypePaidGroupCallReactionReceive {
    self
  }
}

impl AsRef<StarTransactionTypePaidGroupCallReactionReceive>
  for RTDStarTransactionTypePaidGroupCallReactionReceiveBuilder
{
  fn as_ref(&self) -> &StarTransactionTypePaidGroupCallReactionReceive {
    &self.inner
  }
}

/// The transaction is a sending of a paid group call reaction; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypePaidGroupCallReactionSend {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that received the payment
  chat_id: i64,
}

impl RObject for StarTransactionTypePaidGroupCallReactionSend {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypePaidGroupCallReactionSend"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypePaidGroupCallReactionSend {}

impl StarTransactionTypePaidGroupCallReactionSend {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypePaidGroupCallReactionSendBuilder {
    let inner = StarTransactionTypePaidGroupCallReactionSend {
      td_name: "starTransactionTypePaidGroupCallReactionSend".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypePaidGroupCallReactionSendBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypePaidGroupCallReactionSendBuilder {
  inner: StarTransactionTypePaidGroupCallReactionSend,
}

impl RTDStarTransactionTypePaidGroupCallReactionSendBuilder {
  pub fn build(&self) -> StarTransactionTypePaidGroupCallReactionSend {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<StarTransactionTypePaidGroupCallReactionSend>
  for StarTransactionTypePaidGroupCallReactionSend
{
  fn as_ref(&self) -> &StarTransactionTypePaidGroupCallReactionSend {
    self
  }
}

impl AsRef<StarTransactionTypePaidGroupCallReactionSend>
  for RTDStarTransactionTypePaidGroupCallReactionSendBuilder
{
  fn as_ref(&self) -> &StarTransactionTypePaidGroupCallReactionSend {
    &self.inner
  }
}

/// The transaction is a receiving of a paid message; relevant for regular users, supergroup and channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypePaidMessageReceive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the sender of the message
  sender_id: MessageSender,
  /// Number of received paid messages
  message_count: i64,
  /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars paid for message sending
  commission_per_mille: i64,
  /// The Telegram Star amount that was received by Telegram; can be negative for refunds
  commission_star_amount: StarAmount,
}

impl RObject for StarTransactionTypePaidMessageReceive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypePaidMessageReceive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypePaidMessageReceive {}

impl StarTransactionTypePaidMessageReceive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypePaidMessageReceiveBuilder {
    let inner = StarTransactionTypePaidMessageReceive {
      td_name: "starTransactionTypePaidMessageReceive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypePaidMessageReceiveBuilder { inner }
  }

  pub fn sender_id(&self) -> &MessageSender {
    &self.sender_id
  }

  pub fn message_count(&self) -> i64 {
    self.message_count
  }

  pub fn commission_per_mille(&self) -> i64 {
    self.commission_per_mille
  }

  pub fn commission_star_amount(&self) -> &StarAmount {
    &self.commission_star_amount
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypePaidMessageReceiveBuilder {
  inner: StarTransactionTypePaidMessageReceive,
}

impl RTDStarTransactionTypePaidMessageReceiveBuilder {
  pub fn build(&self) -> StarTransactionTypePaidMessageReceive {
    self.inner.clone()
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = sender_id.as_ref().clone();
    self
  }

  pub fn message_count(&mut self, message_count: i64) -> &mut Self {
    self.inner.message_count = message_count;
    self
  }

  pub fn commission_per_mille(&mut self, commission_per_mille: i64) -> &mut Self {
    self.inner.commission_per_mille = commission_per_mille;
    self
  }

  pub fn commission_star_amount<T: AsRef<StarAmount>>(
    &mut self,
    commission_star_amount: T,
  ) -> &mut Self {
    self.inner.commission_star_amount = commission_star_amount.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypePaidMessageReceive> for StarTransactionTypePaidMessageReceive {
  fn as_ref(&self) -> &StarTransactionTypePaidMessageReceive {
    self
  }
}

impl AsRef<StarTransactionTypePaidMessageReceive>
  for RTDStarTransactionTypePaidMessageReceiveBuilder
{
  fn as_ref(&self) -> &StarTransactionTypePaidMessageReceive {
    &self.inner
  }
}

/// The transaction is a sending of a paid message; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypePaidMessageSend {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat that received the payment
  chat_id: i64,
  /// Number of sent paid messages
  message_count: i64,
}

impl RObject for StarTransactionTypePaidMessageSend {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypePaidMessageSend"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypePaidMessageSend {}

impl StarTransactionTypePaidMessageSend {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypePaidMessageSendBuilder {
    let inner = StarTransactionTypePaidMessageSend {
      td_name: "starTransactionTypePaidMessageSend".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypePaidMessageSendBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn message_count(&self) -> i64 {
    self.message_count
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypePaidMessageSendBuilder {
  inner: StarTransactionTypePaidMessageSend,
}

impl RTDStarTransactionTypePaidMessageSendBuilder {
  pub fn build(&self) -> StarTransactionTypePaidMessageSend {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn message_count(&mut self, message_count: i64) -> &mut Self {
    self.inner.message_count = message_count;
    self
  }
}

impl AsRef<StarTransactionTypePaidMessageSend> for StarTransactionTypePaidMessageSend {
  fn as_ref(&self) -> &StarTransactionTypePaidMessageSend {
    self
  }
}

impl AsRef<StarTransactionTypePaidMessageSend> for RTDStarTransactionTypePaidMessageSendBuilder {
  fn as_ref(&self) -> &StarTransactionTypePaidMessageSend {
    &self.inner
  }
}

/// The transaction is a deposit of Telegram Stars from the Premium bot; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypePremiumBotDeposit {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StarTransactionTypePremiumBotDeposit {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypePremiumBotDeposit"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypePremiumBotDeposit {}

impl StarTransactionTypePremiumBotDeposit {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypePremiumBotDepositBuilder {
    let inner = StarTransactionTypePremiumBotDeposit {
      td_name: "starTransactionTypePremiumBotDeposit".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStarTransactionTypePremiumBotDepositBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypePremiumBotDepositBuilder {
  inner: StarTransactionTypePremiumBotDeposit,
}

impl RTDStarTransactionTypePremiumBotDepositBuilder {
  pub fn build(&self) -> StarTransactionTypePremiumBotDeposit {
    self.inner.clone()
  }
}

impl AsRef<StarTransactionTypePremiumBotDeposit> for StarTransactionTypePremiumBotDeposit {
  fn as_ref(&self) -> &StarTransactionTypePremiumBotDeposit {
    self
  }
}

impl AsRef<StarTransactionTypePremiumBotDeposit>
  for RTDStarTransactionTypePremiumBotDepositBuilder
{
  fn as_ref(&self) -> &StarTransactionTypePremiumBotDeposit {
    &self.inner
  }
}

/// The transaction is a purchase of Telegram Premium subscription; relevant for regular users and bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypePremiumPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who received the Telegram Premium subscription
  user_id: i64,
  /// Number of months the Telegram Premium subscription will be active
  month_count: i64,
  /// A sticker to be shown in the transaction information; may be null if unknown
  sticker: Option<Sticker>,
}

impl RObject for StarTransactionTypePremiumPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypePremiumPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypePremiumPurchase {}

impl StarTransactionTypePremiumPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypePremiumPurchaseBuilder {
    let inner = StarTransactionTypePremiumPurchase {
      td_name: "starTransactionTypePremiumPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypePremiumPurchaseBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }

  pub fn sticker(&self) -> &Option<Sticker> {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypePremiumPurchaseBuilder {
  inner: StarTransactionTypePremiumPurchase,
}

impl RTDStarTransactionTypePremiumPurchaseBuilder {
  pub fn build(&self) -> StarTransactionTypePremiumPurchase {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn month_count(&mut self, month_count: i64) -> &mut Self {
    self.inner.month_count = month_count;
    self
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }
}

impl AsRef<StarTransactionTypePremiumPurchase> for StarTransactionTypePremiumPurchase {
  fn as_ref(&self) -> &StarTransactionTypePremiumPurchase {
    self
  }
}

impl AsRef<StarTransactionTypePremiumPurchase> for RTDStarTransactionTypePremiumPurchaseBuilder {
  fn as_ref(&self) -> &StarTransactionTypePremiumPurchase {
    &self.inner
  }
}

/// The transaction is a payment for search of posts in public Telegram channels; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypePublicPostSearch {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StarTransactionTypePublicPostSearch {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypePublicPostSearch"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypePublicPostSearch {}

impl StarTransactionTypePublicPostSearch {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypePublicPostSearchBuilder {
    let inner = StarTransactionTypePublicPostSearch {
      td_name: "starTransactionTypePublicPostSearch".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStarTransactionTypePublicPostSearchBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypePublicPostSearchBuilder {
  inner: StarTransactionTypePublicPostSearch,
}

impl RTDStarTransactionTypePublicPostSearchBuilder {
  pub fn build(&self) -> StarTransactionTypePublicPostSearch {
    self.inner.clone()
  }
}

impl AsRef<StarTransactionTypePublicPostSearch> for StarTransactionTypePublicPostSearch {
  fn as_ref(&self) -> &StarTransactionTypePublicPostSearch {
    self
  }
}

impl AsRef<StarTransactionTypePublicPostSearch> for RTDStarTransactionTypePublicPostSearchBuilder {
  fn as_ref(&self) -> &StarTransactionTypePublicPostSearch {
    &self.inner
  }
}

/// The transaction is a receiving of a payment for a suggested post by the channel chat; relevant for channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeSuggestedPostPaymentReceive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who paid for the suggested post
  user_id: i64,
}

impl RObject for StarTransactionTypeSuggestedPostPaymentReceive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeSuggestedPostPaymentReceive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeSuggestedPostPaymentReceive {}

impl StarTransactionTypeSuggestedPostPaymentReceive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeSuggestedPostPaymentReceiveBuilder {
    let inner = StarTransactionTypeSuggestedPostPaymentReceive {
      td_name: "starTransactionTypeSuggestedPostPaymentReceive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeSuggestedPostPaymentReceiveBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeSuggestedPostPaymentReceiveBuilder {
  inner: StarTransactionTypeSuggestedPostPaymentReceive,
}

impl RTDStarTransactionTypeSuggestedPostPaymentReceiveBuilder {
  pub fn build(&self) -> StarTransactionTypeSuggestedPostPaymentReceive {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }
}

impl AsRef<StarTransactionTypeSuggestedPostPaymentReceive>
  for StarTransactionTypeSuggestedPostPaymentReceive
{
  fn as_ref(&self) -> &StarTransactionTypeSuggestedPostPaymentReceive {
    self
  }
}

impl AsRef<StarTransactionTypeSuggestedPostPaymentReceive>
  for RTDStarTransactionTypeSuggestedPostPaymentReceiveBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeSuggestedPostPaymentReceive {
    &self.inner
  }
}

/// The transaction is a payment for a suggested post; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeSuggestedPostPaymentSend {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the channel chat that posted the post
  chat_id: i64,
}

impl RObject for StarTransactionTypeSuggestedPostPaymentSend {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeSuggestedPostPaymentSend"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeSuggestedPostPaymentSend {}

impl StarTransactionTypeSuggestedPostPaymentSend {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeSuggestedPostPaymentSendBuilder {
    let inner = StarTransactionTypeSuggestedPostPaymentSend {
      td_name: "starTransactionTypeSuggestedPostPaymentSend".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeSuggestedPostPaymentSendBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeSuggestedPostPaymentSendBuilder {
  inner: StarTransactionTypeSuggestedPostPaymentSend,
}

impl RTDStarTransactionTypeSuggestedPostPaymentSendBuilder {
  pub fn build(&self) -> StarTransactionTypeSuggestedPostPaymentSend {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<StarTransactionTypeSuggestedPostPaymentSend>
  for StarTransactionTypeSuggestedPostPaymentSend
{
  fn as_ref(&self) -> &StarTransactionTypeSuggestedPostPaymentSend {
    self
  }
}

impl AsRef<StarTransactionTypeSuggestedPostPaymentSend>
  for RTDStarTransactionTypeSuggestedPostPaymentSendBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeSuggestedPostPaymentSend {
    &self.inner
  }
}

/// The transaction is a withdrawal of earned Telegram Stars to Telegram Ad platform; relevant for bots and channel chats only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeTelegramAdsWithdrawal {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StarTransactionTypeTelegramAdsWithdrawal {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeTelegramAdsWithdrawal"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeTelegramAdsWithdrawal {}

impl StarTransactionTypeTelegramAdsWithdrawal {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeTelegramAdsWithdrawalBuilder {
    let inner = StarTransactionTypeTelegramAdsWithdrawal {
      td_name: "starTransactionTypeTelegramAdsWithdrawal".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStarTransactionTypeTelegramAdsWithdrawalBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeTelegramAdsWithdrawalBuilder {
  inner: StarTransactionTypeTelegramAdsWithdrawal,
}

impl RTDStarTransactionTypeTelegramAdsWithdrawalBuilder {
  pub fn build(&self) -> StarTransactionTypeTelegramAdsWithdrawal {
    self.inner.clone()
  }
}

impl AsRef<StarTransactionTypeTelegramAdsWithdrawal> for StarTransactionTypeTelegramAdsWithdrawal {
  fn as_ref(&self) -> &StarTransactionTypeTelegramAdsWithdrawal {
    self
  }
}

impl AsRef<StarTransactionTypeTelegramAdsWithdrawal>
  for RTDStarTransactionTypeTelegramAdsWithdrawalBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeTelegramAdsWithdrawal {
    &self.inner
  }
}

/// The transaction is a payment for Telegram API usage; relevant for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeTelegramApiUsage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of billed requests
  request_count: i64,
}

impl RObject for StarTransactionTypeTelegramApiUsage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeTelegramApiUsage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeTelegramApiUsage {}

impl StarTransactionTypeTelegramApiUsage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeTelegramApiUsageBuilder {
    let inner = StarTransactionTypeTelegramApiUsage {
      td_name: "starTransactionTypeTelegramApiUsage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeTelegramApiUsageBuilder { inner }
  }

  pub fn request_count(&self) -> i64 {
    self.request_count
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeTelegramApiUsageBuilder {
  inner: StarTransactionTypeTelegramApiUsage,
}

impl RTDStarTransactionTypeTelegramApiUsageBuilder {
  pub fn build(&self) -> StarTransactionTypeTelegramApiUsage {
    self.inner.clone()
  }

  pub fn request_count(&mut self, request_count: i64) -> &mut Self {
    self.inner.request_count = request_count;
    self
  }
}

impl AsRef<StarTransactionTypeTelegramApiUsage> for StarTransactionTypeTelegramApiUsage {
  fn as_ref(&self) -> &StarTransactionTypeTelegramApiUsage {
    self
  }
}

impl AsRef<StarTransactionTypeTelegramApiUsage> for RTDStarTransactionTypeTelegramApiUsageBuilder {
  fn as_ref(&self) -> &StarTransactionTypeTelegramApiUsage {
    &self.inner
  }
}

/// The transaction is a transaction of an unsupported type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for StarTransactionTypeUnsupported {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeUnsupported"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeUnsupported {}

impl StarTransactionTypeUnsupported {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeUnsupportedBuilder {
    let inner = StarTransactionTypeUnsupported {
      td_name: "starTransactionTypeUnsupported".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDStarTransactionTypeUnsupportedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeUnsupportedBuilder {
  inner: StarTransactionTypeUnsupported,
}

impl RTDStarTransactionTypeUnsupportedBuilder {
  pub fn build(&self) -> StarTransactionTypeUnsupported {
    self.inner.clone()
  }
}

impl AsRef<StarTransactionTypeUnsupported> for StarTransactionTypeUnsupported {
  fn as_ref(&self) -> &StarTransactionTypeUnsupported {
    self
  }
}

impl AsRef<StarTransactionTypeUnsupported> for RTDStarTransactionTypeUnsupportedBuilder {
  fn as_ref(&self) -> &StarTransactionTypeUnsupported {
    &self.inner
  }
}

/// The transaction is a purchase of an upgraded gift for some user or channel; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeUpgradedGiftPurchase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who sold the gift
  user_id: i64,
  /// The gift
  gift: UpgradedGift,
}

impl RObject for StarTransactionTypeUpgradedGiftPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeUpgradedGiftPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeUpgradedGiftPurchase {}

impl StarTransactionTypeUpgradedGiftPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeUpgradedGiftPurchaseBuilder {
    let inner = StarTransactionTypeUpgradedGiftPurchase {
      td_name: "starTransactionTypeUpgradedGiftPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeUpgradedGiftPurchaseBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeUpgradedGiftPurchaseBuilder {
  inner: StarTransactionTypeUpgradedGiftPurchase,
}

impl RTDStarTransactionTypeUpgradedGiftPurchaseBuilder {
  pub fn build(&self) -> StarTransactionTypeUpgradedGiftPurchase {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<StarTransactionTypeUpgradedGiftPurchase> for StarTransactionTypeUpgradedGiftPurchase {
  fn as_ref(&self) -> &StarTransactionTypeUpgradedGiftPurchase {
    self
  }
}

impl AsRef<StarTransactionTypeUpgradedGiftPurchase>
  for RTDStarTransactionTypeUpgradedGiftPurchaseBuilder
{
  fn as_ref(&self) -> &StarTransactionTypeUpgradedGiftPurchase {
    &self.inner
  }
}

/// The transaction is a sale of an upgraded gift; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeUpgradedGiftSale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who bought the gift
  user_id: i64,
  /// The gift
  gift: UpgradedGift,
  /// The number of Telegram Stars received by the Telegram for each 1000 Telegram Stars received by the seller of the gift
  commission_per_mille: i64,
  /// The Telegram Star amount that was received by Telegram; can be negative for refunds
  commission_star_amount: StarAmount,
  /// True, if the gift was sold through a purchase offer
  via_offer: bool,
}

impl RObject for StarTransactionTypeUpgradedGiftSale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeUpgradedGiftSale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeUpgradedGiftSale {}

impl StarTransactionTypeUpgradedGiftSale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeUpgradedGiftSaleBuilder {
    let inner = StarTransactionTypeUpgradedGiftSale {
      td_name: "starTransactionTypeUpgradedGiftSale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeUpgradedGiftSaleBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }

  pub fn commission_per_mille(&self) -> i64 {
    self.commission_per_mille
  }

  pub fn commission_star_amount(&self) -> &StarAmount {
    &self.commission_star_amount
  }

  pub fn via_offer(&self) -> bool {
    self.via_offer
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeUpgradedGiftSaleBuilder {
  inner: StarTransactionTypeUpgradedGiftSale,
}

impl RTDStarTransactionTypeUpgradedGiftSaleBuilder {
  pub fn build(&self) -> StarTransactionTypeUpgradedGiftSale {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }

  pub fn commission_per_mille(&mut self, commission_per_mille: i64) -> &mut Self {
    self.inner.commission_per_mille = commission_per_mille;
    self
  }

  pub fn commission_star_amount<T: AsRef<StarAmount>>(
    &mut self,
    commission_star_amount: T,
  ) -> &mut Self {
    self.inner.commission_star_amount = commission_star_amount.as_ref().clone();
    self
  }

  pub fn via_offer(&mut self, via_offer: bool) -> &mut Self {
    self.inner.via_offer = via_offer;
    self
  }
}

impl AsRef<StarTransactionTypeUpgradedGiftSale> for StarTransactionTypeUpgradedGiftSale {
  fn as_ref(&self) -> &StarTransactionTypeUpgradedGiftSale {
    self
  }
}

impl AsRef<StarTransactionTypeUpgradedGiftSale> for RTDStarTransactionTypeUpgradedGiftSaleBuilder {
  fn as_ref(&self) -> &StarTransactionTypeUpgradedGiftSale {
    &self.inner
  }
}

/// The transaction is a deposit of Telegram Stars by another user; relevant for regular users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactionTypeUserDeposit {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user who gifted Telegram Stars; 0 if the user was anonymous
  user_id: i64,
  /// The sticker to be shown in the transaction information; may be null if unknown
  sticker: Option<Sticker>,
}

impl RObject for StarTransactionTypeUserDeposit {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactionTypeUserDeposit"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStarTransactionType for StarTransactionTypeUserDeposit {}

impl StarTransactionTypeUserDeposit {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionTypeUserDepositBuilder {
    let inner = StarTransactionTypeUserDeposit {
      td_name: "starTransactionTypeUserDeposit".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionTypeUserDepositBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn sticker(&self) -> &Option<Sticker> {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionTypeUserDepositBuilder {
  inner: StarTransactionTypeUserDeposit,
}

impl RTDStarTransactionTypeUserDepositBuilder {
  pub fn build(&self) -> StarTransactionTypeUserDeposit {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }
}

impl AsRef<StarTransactionTypeUserDeposit> for StarTransactionTypeUserDeposit {
  fn as_ref(&self) -> &StarTransactionTypeUserDeposit {
    self
  }
}

impl AsRef<StarTransactionTypeUserDeposit> for RTDStarTransactionTypeUserDepositBuilder {
  fn as_ref(&self) -> &StarTransactionTypeUserDeposit {
    &self.inner
  }
}
