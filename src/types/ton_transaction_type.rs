use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of transaction with TON Grams
pub trait TDTonTransactionType: Debug + RObject {}

/// Describes type of transaction with TON Grams
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TonTransactionType {
  #[doc(hidden)]
  _Default(()),
  /// The transaction is a deposit of Grams from Fragment
  FragmentDeposit(TonTransactionTypeFragmentDeposit),
  /// The transaction is a withdrawal of earned Grams to Fragment
  FragmentWithdrawal(TonTransactionTypeFragmentWithdrawal),
  /// The transaction is an offer of gift purchase
  GiftPurchaseOffer(TonTransactionTypeGiftPurchaseOffer),
  /// The transaction is a payment for successful stake dice throw
  StakeDicePayout(TonTransactionTypeStakeDicePayout),
  /// The transaction is a payment for stake dice throw
  StakeDiceStake(TonTransactionTypeStakeDiceStake),
  /// The transaction is a payment for a suggested post
  SuggestedPostPayment(TonTransactionTypeSuggestedPostPayment),
  /// The transaction is a transaction of an unsupported type
  Unsupported(TonTransactionTypeUnsupported),
  /// The transaction is a purchase of an upgraded gift for some user or channel
  UpgradedGiftPurchase(TonTransactionTypeUpgradedGiftPurchase),
  /// The transaction is a sale of an upgraded gift
  UpgradedGiftSale(TonTransactionTypeUpgradedGiftSale),
}

impl Default for TonTransactionType {
  fn default() -> Self {
    TonTransactionType::_Default(())
  }
}

impl<'de> Deserialize<'de> for TonTransactionType {
  fn deserialize<D>(deserializer: D) -> Result<TonTransactionType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TonTransactionType,
      (tonTransactionTypeFragmentDeposit, FragmentDeposit);
      (tonTransactionTypeFragmentWithdrawal, FragmentWithdrawal);
      (tonTransactionTypeGiftPurchaseOffer, GiftPurchaseOffer);
      (tonTransactionTypeStakeDicePayout, StakeDicePayout);
      (tonTransactionTypeStakeDiceStake, StakeDiceStake);
      (tonTransactionTypeSuggestedPostPayment, SuggestedPostPayment);
      (tonTransactionTypeUnsupported, Unsupported);
      (tonTransactionTypeUpgradedGiftPurchase, UpgradedGiftPurchase);
      (tonTransactionTypeUpgradedGiftSale, UpgradedGiftSale);

    )(deserializer)
  }
}

impl RObject for TonTransactionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TonTransactionType::FragmentDeposit(t) => t.td_name(),
      TonTransactionType::FragmentWithdrawal(t) => t.td_name(),
      TonTransactionType::GiftPurchaseOffer(t) => t.td_name(),
      TonTransactionType::StakeDicePayout(t) => t.td_name(),
      TonTransactionType::StakeDiceStake(t) => t.td_name(),
      TonTransactionType::SuggestedPostPayment(t) => t.td_name(),
      TonTransactionType::Unsupported(t) => t.td_name(),
      TonTransactionType::UpgradedGiftPurchase(t) => t.td_name(),
      TonTransactionType::UpgradedGiftSale(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TonTransactionType::FragmentDeposit(t) => t.extra(),
      TonTransactionType::FragmentWithdrawal(t) => t.extra(),
      TonTransactionType::GiftPurchaseOffer(t) => t.extra(),
      TonTransactionType::StakeDicePayout(t) => t.extra(),
      TonTransactionType::StakeDiceStake(t) => t.extra(),
      TonTransactionType::SuggestedPostPayment(t) => t.extra(),
      TonTransactionType::Unsupported(t) => t.extra(),
      TonTransactionType::UpgradedGiftPurchase(t) => t.extra(),
      TonTransactionType::UpgradedGiftSale(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonTransactionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TonTransactionType::_Default(_))
  }

  pub fn is_fragment_deposit(&self) -> bool {
    matches!(self, TonTransactionType::FragmentDeposit(_))
  }
  pub fn is_fragment_withdrawal(&self) -> bool {
    matches!(self, TonTransactionType::FragmentWithdrawal(_))
  }
  pub fn is_gift_purchase_offer(&self) -> bool {
    matches!(self, TonTransactionType::GiftPurchaseOffer(_))
  }
  pub fn is_stake_dice_payout(&self) -> bool {
    matches!(self, TonTransactionType::StakeDicePayout(_))
  }
  pub fn is_stake_dice_stake(&self) -> bool {
    matches!(self, TonTransactionType::StakeDiceStake(_))
  }
  pub fn is_suggested_post_payment(&self) -> bool {
    matches!(self, TonTransactionType::SuggestedPostPayment(_))
  }
  pub fn is_unsupported(&self) -> bool {
    matches!(self, TonTransactionType::Unsupported(_))
  }
  pub fn is_upgraded_gift_purchase(&self) -> bool {
    matches!(self, TonTransactionType::UpgradedGiftPurchase(_))
  }
  pub fn is_upgraded_gift_sale(&self) -> bool {
    matches!(self, TonTransactionType::UpgradedGiftSale(_))
  }

  pub fn on_fragment_deposit<F: FnOnce(&TonTransactionTypeFragmentDeposit)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TonTransactionType::FragmentDeposit(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fragment_withdrawal<F: FnOnce(&TonTransactionTypeFragmentWithdrawal)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TonTransactionType::FragmentWithdrawal(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_gift_purchase_offer<F: FnOnce(&TonTransactionTypeGiftPurchaseOffer)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TonTransactionType::GiftPurchaseOffer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stake_dice_payout<F: FnOnce(&TonTransactionTypeStakeDicePayout)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TonTransactionType::StakeDicePayout(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stake_dice_stake<F: FnOnce(&TonTransactionTypeStakeDiceStake)>(&self, fnc: F) -> &Self {
    if let TonTransactionType::StakeDiceStake(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_suggested_post_payment<F: FnOnce(&TonTransactionTypeSuggestedPostPayment)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TonTransactionType::SuggestedPostPayment(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unsupported<F: FnOnce(&TonTransactionTypeUnsupported)>(&self, fnc: F) -> &Self {
    if let TonTransactionType::Unsupported(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift_purchase<F: FnOnce(&TonTransactionTypeUpgradedGiftPurchase)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TonTransactionType::UpgradedGiftPurchase(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgraded_gift_sale<F: FnOnce(&TonTransactionTypeUpgradedGiftSale)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TonTransactionType::UpgradedGiftSale(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_fragment_deposit(&self) -> Option<&TonTransactionTypeFragmentDeposit> {
    if let TonTransactionType::FragmentDeposit(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fragment_withdrawal(&self) -> Option<&TonTransactionTypeFragmentWithdrawal> {
    if let TonTransactionType::FragmentWithdrawal(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_gift_purchase_offer(&self) -> Option<&TonTransactionTypeGiftPurchaseOffer> {
    if let TonTransactionType::GiftPurchaseOffer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stake_dice_payout(&self) -> Option<&TonTransactionTypeStakeDicePayout> {
    if let TonTransactionType::StakeDicePayout(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stake_dice_stake(&self) -> Option<&TonTransactionTypeStakeDiceStake> {
    if let TonTransactionType::StakeDiceStake(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_suggested_post_payment(&self) -> Option<&TonTransactionTypeSuggestedPostPayment> {
    if let TonTransactionType::SuggestedPostPayment(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unsupported(&self) -> Option<&TonTransactionTypeUnsupported> {
    if let TonTransactionType::Unsupported(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift_purchase(&self) -> Option<&TonTransactionTypeUpgradedGiftPurchase> {
    if let TonTransactionType::UpgradedGiftPurchase(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgraded_gift_sale(&self) -> Option<&TonTransactionTypeUpgradedGiftSale> {
    if let TonTransactionType::UpgradedGiftSale(t) = self {
      return Some(t);
    }
    None
  }

  pub fn fragment_deposit<T: AsRef<TonTransactionTypeFragmentDeposit>>(t: T) -> Self {
    TonTransactionType::FragmentDeposit(t.as_ref().clone())
  }

  pub fn fragment_withdrawal<T: AsRef<TonTransactionTypeFragmentWithdrawal>>(t: T) -> Self {
    TonTransactionType::FragmentWithdrawal(t.as_ref().clone())
  }

  pub fn gift_purchase_offer<T: AsRef<TonTransactionTypeGiftPurchaseOffer>>(t: T) -> Self {
    TonTransactionType::GiftPurchaseOffer(t.as_ref().clone())
  }

  pub fn stake_dice_payout<T: AsRef<TonTransactionTypeStakeDicePayout>>(t: T) -> Self {
    TonTransactionType::StakeDicePayout(t.as_ref().clone())
  }

  pub fn stake_dice_stake<T: AsRef<TonTransactionTypeStakeDiceStake>>(t: T) -> Self {
    TonTransactionType::StakeDiceStake(t.as_ref().clone())
  }

  pub fn suggested_post_payment<T: AsRef<TonTransactionTypeSuggestedPostPayment>>(t: T) -> Self {
    TonTransactionType::SuggestedPostPayment(t.as_ref().clone())
  }

  pub fn unsupported<T: AsRef<TonTransactionTypeUnsupported>>(t: T) -> Self {
    TonTransactionType::Unsupported(t.as_ref().clone())
  }

  pub fn upgraded_gift_purchase<T: AsRef<TonTransactionTypeUpgradedGiftPurchase>>(t: T) -> Self {
    TonTransactionType::UpgradedGiftPurchase(t.as_ref().clone())
  }

  pub fn upgraded_gift_sale<T: AsRef<TonTransactionTypeUpgradedGiftSale>>(t: T) -> Self {
    TonTransactionType::UpgradedGiftSale(t.as_ref().clone())
  }
}

impl AsRef<TonTransactionType> for TonTransactionType {
  fn as_ref(&self) -> &TonTransactionType {
    self
  }
}

/// The transaction is a deposit of Grams from Fragment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransactionTypeFragmentDeposit {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the transaction is a gift from another user
  is_gift: bool,
  /// The sticker to be shown in the transaction information; may be null if unknown
  sticker: Option<Sticker>,
}

impl RObject for TonTransactionTypeFragmentDeposit {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransactionTypeFragmentDeposit"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonTransactionType for TonTransactionTypeFragmentDeposit {}

impl TonTransactionTypeFragmentDeposit {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionTypeFragmentDepositBuilder {
    let inner = TonTransactionTypeFragmentDeposit {
      td_name: "tonTransactionTypeFragmentDeposit".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonTransactionTypeFragmentDepositBuilder { inner }
  }

  pub fn is_gift(&self) -> bool {
    self.is_gift
  }

  pub fn sticker(&self) -> &Option<Sticker> {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionTypeFragmentDepositBuilder {
  inner: TonTransactionTypeFragmentDeposit,
}

impl RTDTonTransactionTypeFragmentDepositBuilder {
  pub fn build(&self) -> TonTransactionTypeFragmentDeposit {
    self.inner.clone()
  }

  pub fn is_gift(&mut self, is_gift: bool) -> &mut Self {
    self.inner.is_gift = is_gift;
    self
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }
}

impl AsRef<TonTransactionTypeFragmentDeposit> for TonTransactionTypeFragmentDeposit {
  fn as_ref(&self) -> &TonTransactionTypeFragmentDeposit {
    self
  }
}

impl AsRef<TonTransactionTypeFragmentDeposit> for RTDTonTransactionTypeFragmentDepositBuilder {
  fn as_ref(&self) -> &TonTransactionTypeFragmentDeposit {
    &self.inner
  }
}

/// The transaction is a withdrawal of earned Grams to Fragment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransactionTypeFragmentWithdrawal {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// State of the withdrawal; may be null for refunds from Fragment
  withdrawal_state: Option<RevenueWithdrawalState>,
}

impl RObject for TonTransactionTypeFragmentWithdrawal {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransactionTypeFragmentWithdrawal"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonTransactionType for TonTransactionTypeFragmentWithdrawal {}

impl TonTransactionTypeFragmentWithdrawal {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionTypeFragmentWithdrawalBuilder {
    let inner = TonTransactionTypeFragmentWithdrawal {
      td_name: "tonTransactionTypeFragmentWithdrawal".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonTransactionTypeFragmentWithdrawalBuilder { inner }
  }

  pub fn withdrawal_state(&self) -> &Option<RevenueWithdrawalState> {
    &self.withdrawal_state
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionTypeFragmentWithdrawalBuilder {
  inner: TonTransactionTypeFragmentWithdrawal,
}

impl RTDTonTransactionTypeFragmentWithdrawalBuilder {
  pub fn build(&self) -> TonTransactionTypeFragmentWithdrawal {
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

impl AsRef<TonTransactionTypeFragmentWithdrawal> for TonTransactionTypeFragmentWithdrawal {
  fn as_ref(&self) -> &TonTransactionTypeFragmentWithdrawal {
    self
  }
}

impl AsRef<TonTransactionTypeFragmentWithdrawal>
  for RTDTonTransactionTypeFragmentWithdrawalBuilder
{
  fn as_ref(&self) -> &TonTransactionTypeFragmentWithdrawal {
    &self.inner
  }
}

/// The transaction is an offer of gift purchase
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransactionTypeGiftPurchaseOffer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: UpgradedGift,
}

impl RObject for TonTransactionTypeGiftPurchaseOffer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransactionTypeGiftPurchaseOffer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonTransactionType for TonTransactionTypeGiftPurchaseOffer {}

impl TonTransactionTypeGiftPurchaseOffer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionTypeGiftPurchaseOfferBuilder {
    let inner = TonTransactionTypeGiftPurchaseOffer {
      td_name: "tonTransactionTypeGiftPurchaseOffer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonTransactionTypeGiftPurchaseOfferBuilder { inner }
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionTypeGiftPurchaseOfferBuilder {
  inner: TonTransactionTypeGiftPurchaseOffer,
}

impl RTDTonTransactionTypeGiftPurchaseOfferBuilder {
  pub fn build(&self) -> TonTransactionTypeGiftPurchaseOffer {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }
}

impl AsRef<TonTransactionTypeGiftPurchaseOffer> for TonTransactionTypeGiftPurchaseOffer {
  fn as_ref(&self) -> &TonTransactionTypeGiftPurchaseOffer {
    self
  }
}

impl AsRef<TonTransactionTypeGiftPurchaseOffer> for RTDTonTransactionTypeGiftPurchaseOfferBuilder {
  fn as_ref(&self) -> &TonTransactionTypeGiftPurchaseOffer {
    &self.inner
  }
}

/// The transaction is a payment for successful stake dice throw
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransactionTypeStakeDicePayout {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonTransactionTypeStakeDicePayout {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransactionTypeStakeDicePayout"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonTransactionType for TonTransactionTypeStakeDicePayout {}

impl TonTransactionTypeStakeDicePayout {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionTypeStakeDicePayoutBuilder {
    let inner = TonTransactionTypeStakeDicePayout {
      td_name: "tonTransactionTypeStakeDicePayout".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonTransactionTypeStakeDicePayoutBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionTypeStakeDicePayoutBuilder {
  inner: TonTransactionTypeStakeDicePayout,
}

impl RTDTonTransactionTypeStakeDicePayoutBuilder {
  pub fn build(&self) -> TonTransactionTypeStakeDicePayout {
    self.inner.clone()
  }
}

impl AsRef<TonTransactionTypeStakeDicePayout> for TonTransactionTypeStakeDicePayout {
  fn as_ref(&self) -> &TonTransactionTypeStakeDicePayout {
    self
  }
}

impl AsRef<TonTransactionTypeStakeDicePayout> for RTDTonTransactionTypeStakeDicePayoutBuilder {
  fn as_ref(&self) -> &TonTransactionTypeStakeDicePayout {
    &self.inner
  }
}

/// The transaction is a payment for stake dice throw
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransactionTypeStakeDiceStake {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonTransactionTypeStakeDiceStake {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransactionTypeStakeDiceStake"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonTransactionType for TonTransactionTypeStakeDiceStake {}

impl TonTransactionTypeStakeDiceStake {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionTypeStakeDiceStakeBuilder {
    let inner = TonTransactionTypeStakeDiceStake {
      td_name: "tonTransactionTypeStakeDiceStake".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonTransactionTypeStakeDiceStakeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionTypeStakeDiceStakeBuilder {
  inner: TonTransactionTypeStakeDiceStake,
}

impl RTDTonTransactionTypeStakeDiceStakeBuilder {
  pub fn build(&self) -> TonTransactionTypeStakeDiceStake {
    self.inner.clone()
  }
}

impl AsRef<TonTransactionTypeStakeDiceStake> for TonTransactionTypeStakeDiceStake {
  fn as_ref(&self) -> &TonTransactionTypeStakeDiceStake {
    self
  }
}

impl AsRef<TonTransactionTypeStakeDiceStake> for RTDTonTransactionTypeStakeDiceStakeBuilder {
  fn as_ref(&self) -> &TonTransactionTypeStakeDiceStake {
    &self.inner
  }
}

/// The transaction is a payment for a suggested post
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransactionTypeSuggestedPostPayment {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the channel chat that posted the post
  chat_id: i64,
}

impl RObject for TonTransactionTypeSuggestedPostPayment {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransactionTypeSuggestedPostPayment"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonTransactionType for TonTransactionTypeSuggestedPostPayment {}

impl TonTransactionTypeSuggestedPostPayment {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionTypeSuggestedPostPaymentBuilder {
    let inner = TonTransactionTypeSuggestedPostPayment {
      td_name: "tonTransactionTypeSuggestedPostPayment".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonTransactionTypeSuggestedPostPaymentBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionTypeSuggestedPostPaymentBuilder {
  inner: TonTransactionTypeSuggestedPostPayment,
}

impl RTDTonTransactionTypeSuggestedPostPaymentBuilder {
  pub fn build(&self) -> TonTransactionTypeSuggestedPostPayment {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<TonTransactionTypeSuggestedPostPayment> for TonTransactionTypeSuggestedPostPayment {
  fn as_ref(&self) -> &TonTransactionTypeSuggestedPostPayment {
    self
  }
}

impl AsRef<TonTransactionTypeSuggestedPostPayment>
  for RTDTonTransactionTypeSuggestedPostPaymentBuilder
{
  fn as_ref(&self) -> &TonTransactionTypeSuggestedPostPayment {
    &self.inner
  }
}

/// The transaction is a transaction of an unsupported type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransactionTypeUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonTransactionTypeUnsupported {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransactionTypeUnsupported"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonTransactionType for TonTransactionTypeUnsupported {}

impl TonTransactionTypeUnsupported {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionTypeUnsupportedBuilder {
    let inner = TonTransactionTypeUnsupported {
      td_name: "tonTransactionTypeUnsupported".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonTransactionTypeUnsupportedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionTypeUnsupportedBuilder {
  inner: TonTransactionTypeUnsupported,
}

impl RTDTonTransactionTypeUnsupportedBuilder {
  pub fn build(&self) -> TonTransactionTypeUnsupported {
    self.inner.clone()
  }
}

impl AsRef<TonTransactionTypeUnsupported> for TonTransactionTypeUnsupported {
  fn as_ref(&self) -> &TonTransactionTypeUnsupported {
    self
  }
}

impl AsRef<TonTransactionTypeUnsupported> for RTDTonTransactionTypeUnsupportedBuilder {
  fn as_ref(&self) -> &TonTransactionTypeUnsupported {
    &self.inner
  }
}

/// The transaction is a purchase of an upgraded gift for some user or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransactionTypeUpgradedGiftPurchase {
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

impl RObject for TonTransactionTypeUpgradedGiftPurchase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransactionTypeUpgradedGiftPurchase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonTransactionType for TonTransactionTypeUpgradedGiftPurchase {}

impl TonTransactionTypeUpgradedGiftPurchase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionTypeUpgradedGiftPurchaseBuilder {
    let inner = TonTransactionTypeUpgradedGiftPurchase {
      td_name: "tonTransactionTypeUpgradedGiftPurchase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonTransactionTypeUpgradedGiftPurchaseBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionTypeUpgradedGiftPurchaseBuilder {
  inner: TonTransactionTypeUpgradedGiftPurchase,
}

impl RTDTonTransactionTypeUpgradedGiftPurchaseBuilder {
  pub fn build(&self) -> TonTransactionTypeUpgradedGiftPurchase {
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

impl AsRef<TonTransactionTypeUpgradedGiftPurchase> for TonTransactionTypeUpgradedGiftPurchase {
  fn as_ref(&self) -> &TonTransactionTypeUpgradedGiftPurchase {
    self
  }
}

impl AsRef<TonTransactionTypeUpgradedGiftPurchase>
  for RTDTonTransactionTypeUpgradedGiftPurchaseBuilder
{
  fn as_ref(&self) -> &TonTransactionTypeUpgradedGiftPurchase {
    &self.inner
  }
}

/// The transaction is a sale of an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransactionTypeUpgradedGiftSale {
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
  /// The number of Grams received by the Telegram for each 1000 Grams received by the seller of the gift
  commission_per_mille: i64,
  /// The Gram amount that was received by the Telegram; in the smallest units of the currency
  commission_gram_amount: i64,
  /// True, if the gift was sold through a purchase offer
  via_offer: bool,
}

impl RObject for TonTransactionTypeUpgradedGiftSale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransactionTypeUpgradedGiftSale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonTransactionType for TonTransactionTypeUpgradedGiftSale {}

impl TonTransactionTypeUpgradedGiftSale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionTypeUpgradedGiftSaleBuilder {
    let inner = TonTransactionTypeUpgradedGiftSale {
      td_name: "tonTransactionTypeUpgradedGiftSale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonTransactionTypeUpgradedGiftSaleBuilder { inner }
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

  pub fn commission_gram_amount(&self) -> i64 {
    self.commission_gram_amount
  }

  pub fn via_offer(&self) -> bool {
    self.via_offer
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionTypeUpgradedGiftSaleBuilder {
  inner: TonTransactionTypeUpgradedGiftSale,
}

impl RTDTonTransactionTypeUpgradedGiftSaleBuilder {
  pub fn build(&self) -> TonTransactionTypeUpgradedGiftSale {
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

  pub fn commission_gram_amount(&mut self, commission_gram_amount: i64) -> &mut Self {
    self.inner.commission_gram_amount = commission_gram_amount;
    self
  }

  pub fn via_offer(&mut self, via_offer: bool) -> &mut Self {
    self.inner.via_offer = via_offer;
    self
  }
}

impl AsRef<TonTransactionTypeUpgradedGiftSale> for TonTransactionTypeUpgradedGiftSale {
  fn as_ref(&self) -> &TonTransactionTypeUpgradedGiftSale {
    self
  }
}

impl AsRef<TonTransactionTypeUpgradedGiftSale> for RTDTonTransactionTypeUpgradedGiftSaleBuilder {
  fn as_ref(&self) -> &TonTransactionTypeUpgradedGiftSale {
    &self.inner
  }
}
