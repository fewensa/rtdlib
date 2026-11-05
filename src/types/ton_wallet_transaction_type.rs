use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of transaction with a TON wallet
pub trait TDTonWalletTransactionType: Debug + RObject {}

/// Describes type of transaction with a TON wallet
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TonWalletTransactionType {
  #[doc(hidden)]
  _Default(()),
  /// Change of the wallet key
  KeyChange(TonWalletTransactionTypeKeyChange),
  /// A transfer of a TON NFT to or from the wallet
  NftTransfer(TonWalletTransactionTypeNftTransfer),
  /// The transaction is a deposit from an on-ramp provider
  OnRampDeposit(TonWalletTransactionTypeOnRampDeposit),
  /// A Telegram gift was minted as a TON NFT or was transferred from Telegram to the wallet or back; peer details aren't available for the transaction
  TelegramGiftTransfer(TonWalletTransactionTypeTelegramGiftTransfer),
  /// A transfer of TON Grams to or from the wallet
  Transfer(TonWalletTransactionTypeTransfer),
}

impl Default for TonWalletTransactionType {
  fn default() -> Self {
    TonWalletTransactionType::_Default(())
  }
}

impl<'de> Deserialize<'de> for TonWalletTransactionType {
  fn deserialize<D>(deserializer: D) -> Result<TonWalletTransactionType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TonWalletTransactionType,
      (tonWalletTransactionTypeKeyChange, KeyChange);
      (tonWalletTransactionTypeNftTransfer, NftTransfer);
      (tonWalletTransactionTypeOnRampDeposit, OnRampDeposit);
      (tonWalletTransactionTypeTelegramGiftTransfer, TelegramGiftTransfer);
      (tonWalletTransactionTypeTransfer, Transfer);

    )(deserializer)
  }
}

impl RObject for TonWalletTransactionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TonWalletTransactionType::KeyChange(t) => t.td_name(),
      TonWalletTransactionType::NftTransfer(t) => t.td_name(),
      TonWalletTransactionType::OnRampDeposit(t) => t.td_name(),
      TonWalletTransactionType::TelegramGiftTransfer(t) => t.td_name(),
      TonWalletTransactionType::Transfer(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TonWalletTransactionType::KeyChange(t) => t.extra(),
      TonWalletTransactionType::NftTransfer(t) => t.extra(),
      TonWalletTransactionType::OnRampDeposit(t) => t.extra(),
      TonWalletTransactionType::TelegramGiftTransfer(t) => t.extra(),
      TonWalletTransactionType::Transfer(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletTransactionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TonWalletTransactionType::_Default(_))
  }

  pub fn is_key_change(&self) -> bool {
    matches!(self, TonWalletTransactionType::KeyChange(_))
  }
  pub fn is_nft_transfer(&self) -> bool {
    matches!(self, TonWalletTransactionType::NftTransfer(_))
  }
  pub fn is_on_ramp_deposit(&self) -> bool {
    matches!(self, TonWalletTransactionType::OnRampDeposit(_))
  }
  pub fn is_telegram_gift_transfer(&self) -> bool {
    matches!(self, TonWalletTransactionType::TelegramGiftTransfer(_))
  }
  pub fn is_transfer(&self) -> bool {
    matches!(self, TonWalletTransactionType::Transfer(_))
  }

  pub fn on_key_change<F: FnOnce(&TonWalletTransactionTypeKeyChange)>(&self, fnc: F) -> &Self {
    if let TonWalletTransactionType::KeyChange(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_nft_transfer<F: FnOnce(&TonWalletTransactionTypeNftTransfer)>(&self, fnc: F) -> &Self {
    if let TonWalletTransactionType::NftTransfer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_on_ramp_deposit<F: FnOnce(&TonWalletTransactionTypeOnRampDeposit)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TonWalletTransactionType::OnRampDeposit(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_telegram_gift_transfer<F: FnOnce(&TonWalletTransactionTypeTelegramGiftTransfer)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let TonWalletTransactionType::TelegramGiftTransfer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_transfer<F: FnOnce(&TonWalletTransactionTypeTransfer)>(&self, fnc: F) -> &Self {
    if let TonWalletTransactionType::Transfer(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_key_change(&self) -> Option<&TonWalletTransactionTypeKeyChange> {
    if let TonWalletTransactionType::KeyChange(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_nft_transfer(&self) -> Option<&TonWalletTransactionTypeNftTransfer> {
    if let TonWalletTransactionType::NftTransfer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_on_ramp_deposit(&self) -> Option<&TonWalletTransactionTypeOnRampDeposit> {
    if let TonWalletTransactionType::OnRampDeposit(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_telegram_gift_transfer(&self) -> Option<&TonWalletTransactionTypeTelegramGiftTransfer> {
    if let TonWalletTransactionType::TelegramGiftTransfer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_transfer(&self) -> Option<&TonWalletTransactionTypeTransfer> {
    if let TonWalletTransactionType::Transfer(t) = self {
      return Some(t);
    }
    None
  }

  pub fn key_change<T: AsRef<TonWalletTransactionTypeKeyChange>>(t: T) -> Self {
    TonWalletTransactionType::KeyChange(t.as_ref().clone())
  }

  pub fn nft_transfer<T: AsRef<TonWalletTransactionTypeNftTransfer>>(t: T) -> Self {
    TonWalletTransactionType::NftTransfer(t.as_ref().clone())
  }

  pub fn on_ramp_deposit<T: AsRef<TonWalletTransactionTypeOnRampDeposit>>(t: T) -> Self {
    TonWalletTransactionType::OnRampDeposit(t.as_ref().clone())
  }

  pub fn telegram_gift_transfer<T: AsRef<TonWalletTransactionTypeTelegramGiftTransfer>>(
    t: T,
  ) -> Self {
    TonWalletTransactionType::TelegramGiftTransfer(t.as_ref().clone())
  }

  pub fn transfer<T: AsRef<TonWalletTransactionTypeTransfer>>(t: T) -> Self {
    TonWalletTransactionType::Transfer(t.as_ref().clone())
  }
}

impl AsRef<TonWalletTransactionType> for TonWalletTransactionType {
  fn as_ref(&self) -> &TonWalletTransactionType {
    self
  }
}

/// Change of the wallet key
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransactionTypeKeyChange {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TonWalletTransactionTypeKeyChange {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransactionTypeKeyChange"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonWalletTransactionType for TonWalletTransactionTypeKeyChange {}

impl TonWalletTransactionTypeKeyChange {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransactionTypeKeyChangeBuilder {
    let inner = TonWalletTransactionTypeKeyChange {
      td_name: "tonWalletTransactionTypeKeyChange".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTonWalletTransactionTypeKeyChangeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransactionTypeKeyChangeBuilder {
  inner: TonWalletTransactionTypeKeyChange,
}

impl RTDTonWalletTransactionTypeKeyChangeBuilder {
  pub fn build(&self) -> TonWalletTransactionTypeKeyChange {
    self.inner.clone()
  }
}

impl AsRef<TonWalletTransactionTypeKeyChange> for TonWalletTransactionTypeKeyChange {
  fn as_ref(&self) -> &TonWalletTransactionTypeKeyChange {
    self
  }
}

impl AsRef<TonWalletTransactionTypeKeyChange> for RTDTonWalletTransactionTypeKeyChangeBuilder {
  fn as_ref(&self) -> &TonWalletTransactionTypeKeyChange {
    &self.inner
  }
}

/// A transfer of a TON NFT to or from the wallet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransactionTypeNftTransfer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The transferred NFT
  nft: TonNft,
  /// True, if the transfer is outgoing
  is_outgoing: bool,
  /// Comment added to the transaction
  comment: String,
  /// True, if the comment is encrypted and must be decrypted using the user's private key
  is_comment_encrypted: bool,
}

impl RObject for TonWalletTransactionTypeNftTransfer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransactionTypeNftTransfer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonWalletTransactionType for TonWalletTransactionTypeNftTransfer {}

impl TonWalletTransactionTypeNftTransfer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransactionTypeNftTransferBuilder {
    let inner = TonWalletTransactionTypeNftTransfer {
      td_name: "tonWalletTransactionTypeNftTransfer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransactionTypeNftTransferBuilder { inner }
  }

  pub fn nft(&self) -> &TonNft {
    &self.nft
  }

  pub fn is_outgoing(&self) -> bool {
    self.is_outgoing
  }

  pub fn comment(&self) -> &String {
    &self.comment
  }

  pub fn is_comment_encrypted(&self) -> bool {
    self.is_comment_encrypted
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransactionTypeNftTransferBuilder {
  inner: TonWalletTransactionTypeNftTransfer,
}

impl RTDTonWalletTransactionTypeNftTransferBuilder {
  pub fn build(&self) -> TonWalletTransactionTypeNftTransfer {
    self.inner.clone()
  }

  pub fn nft<T: AsRef<TonNft>>(&mut self, nft: T) -> &mut Self {
    self.inner.nft = nft.as_ref().clone();
    self
  }

  pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
    self.inner.is_outgoing = is_outgoing;
    self
  }

  pub fn comment<T: AsRef<str>>(&mut self, comment: T) -> &mut Self {
    self.inner.comment = comment.as_ref().to_string();
    self
  }

  pub fn is_comment_encrypted(&mut self, is_comment_encrypted: bool) -> &mut Self {
    self.inner.is_comment_encrypted = is_comment_encrypted;
    self
  }
}

impl AsRef<TonWalletTransactionTypeNftTransfer> for TonWalletTransactionTypeNftTransfer {
  fn as_ref(&self) -> &TonWalletTransactionTypeNftTransfer {
    self
  }
}

impl AsRef<TonWalletTransactionTypeNftTransfer> for RTDTonWalletTransactionTypeNftTransferBuilder {
  fn as_ref(&self) -> &TonWalletTransactionTypeNftTransfer {
    &self.inner
  }
}

/// The transaction is a deposit from an on-ramp provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransactionTypeOnRampDeposit {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The amount of received cryptocurrency, in the smallest units of the cryptocurrency
  amount: i64,
  /// Name of the on-ramp provider
  provider_name: String,
}

impl RObject for TonWalletTransactionTypeOnRampDeposit {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransactionTypeOnRampDeposit"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonWalletTransactionType for TonWalletTransactionTypeOnRampDeposit {}

impl TonWalletTransactionTypeOnRampDeposit {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransactionTypeOnRampDepositBuilder {
    let inner = TonWalletTransactionTypeOnRampDeposit {
      td_name: "tonWalletTransactionTypeOnRampDeposit".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransactionTypeOnRampDepositBuilder { inner }
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn provider_name(&self) -> &String {
    &self.provider_name
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransactionTypeOnRampDepositBuilder {
  inner: TonWalletTransactionTypeOnRampDeposit,
}

impl RTDTonWalletTransactionTypeOnRampDepositBuilder {
  pub fn build(&self) -> TonWalletTransactionTypeOnRampDeposit {
    self.inner.clone()
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn provider_name<T: AsRef<str>>(&mut self, provider_name: T) -> &mut Self {
    self.inner.provider_name = provider_name.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletTransactionTypeOnRampDeposit> for TonWalletTransactionTypeOnRampDeposit {
  fn as_ref(&self) -> &TonWalletTransactionTypeOnRampDeposit {
    self
  }
}

impl AsRef<TonWalletTransactionTypeOnRampDeposit>
  for RTDTonWalletTransactionTypeOnRampDepositBuilder
{
  fn as_ref(&self) -> &TonWalletTransactionTypeOnRampDeposit {
    &self.inner
  }
}

/// A Telegram gift was minted as a TON NFT or was transferred from Telegram to the wallet or back; peer details aren't available for the transaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransactionTypeTelegramGiftTransfer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The minted or transferred NFT
  nft: TonNft,
  /// True, if the transfer is from the wallet to Telegram
  is_outgoing: bool,
  /// True, if the gift was displayed on the current user's profile page before or after the transfer
  is_saved: bool,
}

impl RObject for TonWalletTransactionTypeTelegramGiftTransfer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransactionTypeTelegramGiftTransfer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonWalletTransactionType for TonWalletTransactionTypeTelegramGiftTransfer {}

impl TonWalletTransactionTypeTelegramGiftTransfer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransactionTypeTelegramGiftTransferBuilder {
    let inner = TonWalletTransactionTypeTelegramGiftTransfer {
      td_name: "tonWalletTransactionTypeTelegramGiftTransfer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransactionTypeTelegramGiftTransferBuilder { inner }
  }

  pub fn nft(&self) -> &TonNft {
    &self.nft
  }

  pub fn is_outgoing(&self) -> bool {
    self.is_outgoing
  }

  pub fn is_saved(&self) -> bool {
    self.is_saved
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransactionTypeTelegramGiftTransferBuilder {
  inner: TonWalletTransactionTypeTelegramGiftTransfer,
}

impl RTDTonWalletTransactionTypeTelegramGiftTransferBuilder {
  pub fn build(&self) -> TonWalletTransactionTypeTelegramGiftTransfer {
    self.inner.clone()
  }

  pub fn nft<T: AsRef<TonNft>>(&mut self, nft: T) -> &mut Self {
    self.inner.nft = nft.as_ref().clone();
    self
  }

  pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
    self.inner.is_outgoing = is_outgoing;
    self
  }

  pub fn is_saved(&mut self, is_saved: bool) -> &mut Self {
    self.inner.is_saved = is_saved;
    self
  }
}

impl AsRef<TonWalletTransactionTypeTelegramGiftTransfer>
  for TonWalletTransactionTypeTelegramGiftTransfer
{
  fn as_ref(&self) -> &TonWalletTransactionTypeTelegramGiftTransfer {
    self
  }
}

impl AsRef<TonWalletTransactionTypeTelegramGiftTransfer>
  for RTDTonWalletTransactionTypeTelegramGiftTransferBuilder
{
  fn as_ref(&self) -> &TonWalletTransactionTypeTelegramGiftTransfer {
    &self.inner
  }
}

/// A transfer of TON Grams to or from the wallet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransactionTypeTransfer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The amount of received cryptocurrency, in the smallest units of the cryptocurrency; negative for outgoing transactions
  amount: i64,
  /// True, if the fee was paid by Telegram
  is_gasless: bool,
  /// Comment added to the transaction
  comment: String,
  /// True, if the comment is encrypted and must be decrypted using the user's private key
  is_comment_encrypted: bool,
}

impl RObject for TonWalletTransactionTypeTransfer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransactionTypeTransfer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTonWalletTransactionType for TonWalletTransactionTypeTransfer {}

impl TonWalletTransactionTypeTransfer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransactionTypeTransferBuilder {
    let inner = TonWalletTransactionTypeTransfer {
      td_name: "tonWalletTransactionTypeTransfer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransactionTypeTransferBuilder { inner }
  }

  pub fn amount(&self) -> i64 {
    self.amount
  }

  pub fn is_gasless(&self) -> bool {
    self.is_gasless
  }

  pub fn comment(&self) -> &String {
    &self.comment
  }

  pub fn is_comment_encrypted(&self) -> bool {
    self.is_comment_encrypted
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransactionTypeTransferBuilder {
  inner: TonWalletTransactionTypeTransfer,
}

impl RTDTonWalletTransactionTypeTransferBuilder {
  pub fn build(&self) -> TonWalletTransactionTypeTransfer {
    self.inner.clone()
  }

  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

  pub fn is_gasless(&mut self, is_gasless: bool) -> &mut Self {
    self.inner.is_gasless = is_gasless;
    self
  }

  pub fn comment<T: AsRef<str>>(&mut self, comment: T) -> &mut Self {
    self.inner.comment = comment.as_ref().to_string();
    self
  }

  pub fn is_comment_encrypted(&mut self, is_comment_encrypted: bool) -> &mut Self {
    self.inner.is_comment_encrypted = is_comment_encrypted;
    self
  }
}

impl AsRef<TonWalletTransactionTypeTransfer> for TonWalletTransactionTypeTransfer {
  fn as_ref(&self) -> &TonWalletTransactionTypeTransfer {
    self
  }
}

impl AsRef<TonWalletTransactionTypeTransfer> for RTDTonWalletTransactionTypeTransferBuilder {
  fn as_ref(&self) -> &TonWalletTransactionTypeTransfer {
    &self.inner
  }
}
