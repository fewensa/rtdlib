use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes origin from which the upgraded gift was obtained
pub trait TDUpgradedGiftOrigin: Debug + RObject {}

/// Describes origin from which the upgraded gift was obtained
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UpgradedGiftOrigin {
  #[doc(hidden)]
  _Default(()),
  /// The gift was assigned from blockchain and isn't owned by the current user. The gift can't be transferred, resold, or withdrawn to blockchain
  Blockchain(UpgradedGiftOriginBlockchain),
  /// The gift was crafted from other gifts
  Craft(UpgradedGiftOriginCraft),
  /// The gift was bought through an offer
  Offer(UpgradedGiftOriginOffer),
  /// The sender or receiver of the message has paid for upgrade of the gift, which has been completed
  PrepaidUpgrade(UpgradedGiftOriginPrepaidUpgrade),
  /// The gift was bought from another user
  Resale(UpgradedGiftOriginResale),
  /// The gift was transferred from another owner
  Transfer(UpgradedGiftOriginTransfer),
  /// The gift was obtained by upgrading of a previously received gift
  Upgrade(UpgradedGiftOriginUpgrade),
}

impl Default for UpgradedGiftOrigin {
  fn default() -> Self {
    UpgradedGiftOrigin::_Default(())
  }
}

impl<'de> Deserialize<'de> for UpgradedGiftOrigin {
  fn deserialize<D>(deserializer: D) -> Result<UpgradedGiftOrigin, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      UpgradedGiftOrigin,
      (upgradedGiftOriginBlockchain, Blockchain);
      (upgradedGiftOriginCraft, Craft);
      (upgradedGiftOriginOffer, Offer);
      (upgradedGiftOriginPrepaidUpgrade, PrepaidUpgrade);
      (upgradedGiftOriginResale, Resale);
      (upgradedGiftOriginTransfer, Transfer);
      (upgradedGiftOriginUpgrade, Upgrade);

    )(deserializer)
  }
}

impl RObject for UpgradedGiftOrigin {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      UpgradedGiftOrigin::Blockchain(t) => t.td_name(),
      UpgradedGiftOrigin::Craft(t) => t.td_name(),
      UpgradedGiftOrigin::Offer(t) => t.td_name(),
      UpgradedGiftOrigin::PrepaidUpgrade(t) => t.td_name(),
      UpgradedGiftOrigin::Resale(t) => t.td_name(),
      UpgradedGiftOrigin::Transfer(t) => t.td_name(),
      UpgradedGiftOrigin::Upgrade(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      UpgradedGiftOrigin::Blockchain(t) => t.extra(),
      UpgradedGiftOrigin::Craft(t) => t.extra(),
      UpgradedGiftOrigin::Offer(t) => t.extra(),
      UpgradedGiftOrigin::PrepaidUpgrade(t) => t.extra(),
      UpgradedGiftOrigin::Resale(t) => t.extra(),
      UpgradedGiftOrigin::Transfer(t) => t.extra(),
      UpgradedGiftOrigin::Upgrade(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftOrigin {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, UpgradedGiftOrigin::_Default(_))
  }

  pub fn is_blockchain(&self) -> bool {
    matches!(self, UpgradedGiftOrigin::Blockchain(_))
  }
  pub fn is_craft(&self) -> bool {
    matches!(self, UpgradedGiftOrigin::Craft(_))
  }
  pub fn is_offer(&self) -> bool {
    matches!(self, UpgradedGiftOrigin::Offer(_))
  }
  pub fn is_prepaid_upgrade(&self) -> bool {
    matches!(self, UpgradedGiftOrigin::PrepaidUpgrade(_))
  }
  pub fn is_resale(&self) -> bool {
    matches!(self, UpgradedGiftOrigin::Resale(_))
  }
  pub fn is_transfer(&self) -> bool {
    matches!(self, UpgradedGiftOrigin::Transfer(_))
  }
  pub fn is_upgrade(&self) -> bool {
    matches!(self, UpgradedGiftOrigin::Upgrade(_))
  }

  pub fn on_blockchain<F: FnOnce(&UpgradedGiftOriginBlockchain)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftOrigin::Blockchain(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_craft<F: FnOnce(&UpgradedGiftOriginCraft)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftOrigin::Craft(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_offer<F: FnOnce(&UpgradedGiftOriginOffer)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftOrigin::Offer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_prepaid_upgrade<F: FnOnce(&UpgradedGiftOriginPrepaidUpgrade)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftOrigin::PrepaidUpgrade(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_resale<F: FnOnce(&UpgradedGiftOriginResale)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftOrigin::Resale(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_transfer<F: FnOnce(&UpgradedGiftOriginTransfer)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftOrigin::Transfer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_upgrade<F: FnOnce(&UpgradedGiftOriginUpgrade)>(&self, fnc: F) -> &Self {
    if let UpgradedGiftOrigin::Upgrade(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_blockchain(&self) -> Option<&UpgradedGiftOriginBlockchain> {
    if let UpgradedGiftOrigin::Blockchain(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_craft(&self) -> Option<&UpgradedGiftOriginCraft> {
    if let UpgradedGiftOrigin::Craft(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_offer(&self) -> Option<&UpgradedGiftOriginOffer> {
    if let UpgradedGiftOrigin::Offer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_prepaid_upgrade(&self) -> Option<&UpgradedGiftOriginPrepaidUpgrade> {
    if let UpgradedGiftOrigin::PrepaidUpgrade(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_resale(&self) -> Option<&UpgradedGiftOriginResale> {
    if let UpgradedGiftOrigin::Resale(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_transfer(&self) -> Option<&UpgradedGiftOriginTransfer> {
    if let UpgradedGiftOrigin::Transfer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_upgrade(&self) -> Option<&UpgradedGiftOriginUpgrade> {
    if let UpgradedGiftOrigin::Upgrade(t) = self {
      return Some(t);
    }
    None
  }

  pub fn blockchain<T: AsRef<UpgradedGiftOriginBlockchain>>(t: T) -> Self {
    UpgradedGiftOrigin::Blockchain(t.as_ref().clone())
  }

  pub fn craft<T: AsRef<UpgradedGiftOriginCraft>>(t: T) -> Self {
    UpgradedGiftOrigin::Craft(t.as_ref().clone())
  }

  pub fn offer<T: AsRef<UpgradedGiftOriginOffer>>(t: T) -> Self {
    UpgradedGiftOrigin::Offer(t.as_ref().clone())
  }

  pub fn prepaid_upgrade<T: AsRef<UpgradedGiftOriginPrepaidUpgrade>>(t: T) -> Self {
    UpgradedGiftOrigin::PrepaidUpgrade(t.as_ref().clone())
  }

  pub fn resale<T: AsRef<UpgradedGiftOriginResale>>(t: T) -> Self {
    UpgradedGiftOrigin::Resale(t.as_ref().clone())
  }

  pub fn transfer<T: AsRef<UpgradedGiftOriginTransfer>>(t: T) -> Self {
    UpgradedGiftOrigin::Transfer(t.as_ref().clone())
  }

  pub fn upgrade<T: AsRef<UpgradedGiftOriginUpgrade>>(t: T) -> Self {
    UpgradedGiftOrigin::Upgrade(t.as_ref().clone())
  }
}

impl AsRef<UpgradedGiftOrigin> for UpgradedGiftOrigin {
  fn as_ref(&self) -> &UpgradedGiftOrigin {
    self
  }
}

/// The gift was assigned from blockchain and isn't owned by the current user. The gift can't be transferred, resold, or withdrawn to blockchain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftOriginBlockchain {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UpgradedGiftOriginBlockchain {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftOriginBlockchain"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftOrigin for UpgradedGiftOriginBlockchain {}

impl UpgradedGiftOriginBlockchain {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftOriginBlockchainBuilder {
    let inner = UpgradedGiftOriginBlockchain {
      td_name: "upgradedGiftOriginBlockchain".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUpgradedGiftOriginBlockchainBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftOriginBlockchainBuilder {
  inner: UpgradedGiftOriginBlockchain,
}

impl RTDUpgradedGiftOriginBlockchainBuilder {
  pub fn build(&self) -> UpgradedGiftOriginBlockchain {
    self.inner.clone()
  }
}

impl AsRef<UpgradedGiftOriginBlockchain> for UpgradedGiftOriginBlockchain {
  fn as_ref(&self) -> &UpgradedGiftOriginBlockchain {
    self
  }
}

impl AsRef<UpgradedGiftOriginBlockchain> for RTDUpgradedGiftOriginBlockchainBuilder {
  fn as_ref(&self) -> &UpgradedGiftOriginBlockchain {
    &self.inner
  }
}

/// The gift was crafted from other gifts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftOriginCraft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UpgradedGiftOriginCraft {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftOriginCraft"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftOrigin for UpgradedGiftOriginCraft {}

impl UpgradedGiftOriginCraft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftOriginCraftBuilder {
    let inner = UpgradedGiftOriginCraft {
      td_name: "upgradedGiftOriginCraft".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUpgradedGiftOriginCraftBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftOriginCraftBuilder {
  inner: UpgradedGiftOriginCraft,
}

impl RTDUpgradedGiftOriginCraftBuilder {
  pub fn build(&self) -> UpgradedGiftOriginCraft {
    self.inner.clone()
  }
}

impl AsRef<UpgradedGiftOriginCraft> for UpgradedGiftOriginCraft {
  fn as_ref(&self) -> &UpgradedGiftOriginCraft {
    self
  }
}

impl AsRef<UpgradedGiftOriginCraft> for RTDUpgradedGiftOriginCraftBuilder {
  fn as_ref(&self) -> &UpgradedGiftOriginCraft {
    &self.inner
  }
}

/// The gift was bought through an offer
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftOriginOffer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Price paid for the gift
  price: GiftResalePrice,
}

impl RObject for UpgradedGiftOriginOffer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftOriginOffer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftOrigin for UpgradedGiftOriginOffer {}

impl UpgradedGiftOriginOffer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftOriginOfferBuilder {
    let inner = UpgradedGiftOriginOffer {
      td_name: "upgradedGiftOriginOffer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftOriginOfferBuilder { inner }
  }

  pub fn price(&self) -> &GiftResalePrice {
    &self.price
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftOriginOfferBuilder {
  inner: UpgradedGiftOriginOffer,
}

impl RTDUpgradedGiftOriginOfferBuilder {
  pub fn build(&self) -> UpgradedGiftOriginOffer {
    self.inner.clone()
  }

  pub fn price<T: AsRef<GiftResalePrice>>(&mut self, price: T) -> &mut Self {
    self.inner.price = price.as_ref().clone();
    self
  }
}

impl AsRef<UpgradedGiftOriginOffer> for UpgradedGiftOriginOffer {
  fn as_ref(&self) -> &UpgradedGiftOriginOffer {
    self
  }
}

impl AsRef<UpgradedGiftOriginOffer> for RTDUpgradedGiftOriginOfferBuilder {
  fn as_ref(&self) -> &UpgradedGiftOriginOffer {
    &self.inner
  }
}

/// The sender or receiver of the message has paid for upgrade of the gift, which has been completed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftOriginPrepaidUpgrade {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UpgradedGiftOriginPrepaidUpgrade {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftOriginPrepaidUpgrade"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftOrigin for UpgradedGiftOriginPrepaidUpgrade {}

impl UpgradedGiftOriginPrepaidUpgrade {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftOriginPrepaidUpgradeBuilder {
    let inner = UpgradedGiftOriginPrepaidUpgrade {
      td_name: "upgradedGiftOriginPrepaidUpgrade".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUpgradedGiftOriginPrepaidUpgradeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftOriginPrepaidUpgradeBuilder {
  inner: UpgradedGiftOriginPrepaidUpgrade,
}

impl RTDUpgradedGiftOriginPrepaidUpgradeBuilder {
  pub fn build(&self) -> UpgradedGiftOriginPrepaidUpgrade {
    self.inner.clone()
  }
}

impl AsRef<UpgradedGiftOriginPrepaidUpgrade> for UpgradedGiftOriginPrepaidUpgrade {
  fn as_ref(&self) -> &UpgradedGiftOriginPrepaidUpgrade {
    self
  }
}

impl AsRef<UpgradedGiftOriginPrepaidUpgrade> for RTDUpgradedGiftOriginPrepaidUpgradeBuilder {
  fn as_ref(&self) -> &UpgradedGiftOriginPrepaidUpgrade {
    &self.inner
  }
}

/// The gift was bought from another user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftOriginResale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Price paid for the gift
  price: GiftResalePrice,
}

impl RObject for UpgradedGiftOriginResale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftOriginResale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftOrigin for UpgradedGiftOriginResale {}

impl UpgradedGiftOriginResale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftOriginResaleBuilder {
    let inner = UpgradedGiftOriginResale {
      td_name: "upgradedGiftOriginResale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftOriginResaleBuilder { inner }
  }

  pub fn price(&self) -> &GiftResalePrice {
    &self.price
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftOriginResaleBuilder {
  inner: UpgradedGiftOriginResale,
}

impl RTDUpgradedGiftOriginResaleBuilder {
  pub fn build(&self) -> UpgradedGiftOriginResale {
    self.inner.clone()
  }

  pub fn price<T: AsRef<GiftResalePrice>>(&mut self, price: T) -> &mut Self {
    self.inner.price = price.as_ref().clone();
    self
  }
}

impl AsRef<UpgradedGiftOriginResale> for UpgradedGiftOriginResale {
  fn as_ref(&self) -> &UpgradedGiftOriginResale {
    self
  }
}

impl AsRef<UpgradedGiftOriginResale> for RTDUpgradedGiftOriginResaleBuilder {
  fn as_ref(&self) -> &UpgradedGiftOriginResale {
    &self.inner
  }
}

/// The gift was transferred from another owner
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftOriginTransfer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for UpgradedGiftOriginTransfer {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftOriginTransfer"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftOrigin for UpgradedGiftOriginTransfer {}

impl UpgradedGiftOriginTransfer {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftOriginTransferBuilder {
    let inner = UpgradedGiftOriginTransfer {
      td_name: "upgradedGiftOriginTransfer".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDUpgradedGiftOriginTransferBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftOriginTransferBuilder {
  inner: UpgradedGiftOriginTransfer,
}

impl RTDUpgradedGiftOriginTransferBuilder {
  pub fn build(&self) -> UpgradedGiftOriginTransfer {
    self.inner.clone()
  }
}

impl AsRef<UpgradedGiftOriginTransfer> for UpgradedGiftOriginTransfer {
  fn as_ref(&self) -> &UpgradedGiftOriginTransfer {
    self
  }
}

impl AsRef<UpgradedGiftOriginTransfer> for RTDUpgradedGiftOriginTransferBuilder {
  fn as_ref(&self) -> &UpgradedGiftOriginTransfer {
    &self.inner
  }
}

/// The gift was obtained by upgrading of a previously received gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftOriginUpgrade {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the message with the regular gift that was upgraded; may be 0 or an identifier of a deleted message
  gift_message_id: i64,
}

impl RObject for UpgradedGiftOriginUpgrade {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftOriginUpgrade"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDUpgradedGiftOrigin for UpgradedGiftOriginUpgrade {}

impl UpgradedGiftOriginUpgrade {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftOriginUpgradeBuilder {
    let inner = UpgradedGiftOriginUpgrade {
      td_name: "upgradedGiftOriginUpgrade".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftOriginUpgradeBuilder { inner }
  }

  pub fn gift_message_id(&self) -> i64 {
    self.gift_message_id
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftOriginUpgradeBuilder {
  inner: UpgradedGiftOriginUpgrade,
}

impl RTDUpgradedGiftOriginUpgradeBuilder {
  pub fn build(&self) -> UpgradedGiftOriginUpgrade {
    self.inner.clone()
  }

  pub fn gift_message_id(&mut self, gift_message_id: i64) -> &mut Self {
    self.inner.gift_message_id = gift_message_id;
    self
  }
}

impl AsRef<UpgradedGiftOriginUpgrade> for UpgradedGiftOriginUpgrade {
  fn as_ref(&self) -> &UpgradedGiftOriginUpgrade {
    self
  }
}

impl AsRef<UpgradedGiftOriginUpgrade> for RTDUpgradedGiftOriginUpgradeBuilder {
  fn as_ref(&self) -> &UpgradedGiftOriginUpgrade {
    &self.inner
  }
}
