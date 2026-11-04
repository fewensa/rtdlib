use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains result of gift upgrading
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeGiftResult {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The upgraded gift
  gift: UpgradedGift,
  /// Unique identifier of the received gift for the current user
  received_gift_id: String,
  /// True, if the gift is displayed on the user's or the channel's profile page
  is_saved: bool,
  /// True, if the gift can be transferred to another owner
  can_be_transferred: bool,
  /// Number of Telegram Stars that must be paid to transfer the upgraded gift
  transfer_star_count: i64,
  /// Number of Telegram Stars that must be paid to drop original details of the upgraded gift; 0 if not available
  drop_original_details_star_count: i64,
  /// Point in time (Unix timestamp) when the gift can be transferred to another owner; can be in the past; 0 if the gift can be transferred immediately or transfer isn't possible
  next_transfer_date: i64,
  /// Point in time (Unix timestamp) when the gift can be resold to another user; can be in the past; 0 if the gift can't be resold; only for the receiver of the gift
  next_resale_date: i64,
  /// Point in time (Unix timestamp) when the gift can be transferred to the TON blockchain as an NFT; can be in the past
  export_date: i64,
}

impl RObject for UpgradeGiftResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradeGiftResult"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradeGiftResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradeGiftResultBuilder {
    let inner = UpgradeGiftResult {
      td_name: "upgradeGiftResult".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradeGiftResultBuilder { inner }
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }

  pub fn received_gift_id(&self) -> &String {
    &self.received_gift_id
  }

  pub fn is_saved(&self) -> bool {
    self.is_saved
  }

  pub fn can_be_transferred(&self) -> bool {
    self.can_be_transferred
  }

  pub fn transfer_star_count(&self) -> i64 {
    self.transfer_star_count
  }

  pub fn drop_original_details_star_count(&self) -> i64 {
    self.drop_original_details_star_count
  }

  pub fn next_transfer_date(&self) -> i64 {
    self.next_transfer_date
  }

  pub fn next_resale_date(&self) -> i64 {
    self.next_resale_date
  }

  pub fn export_date(&self) -> i64 {
    self.export_date
  }
}

#[doc(hidden)]
pub struct RTDUpgradeGiftResultBuilder {
  inner: UpgradeGiftResult,
}

impl RTDUpgradeGiftResultBuilder {
  pub fn build(&self) -> UpgradeGiftResult {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }

  pub fn received_gift_id<T: AsRef<str>>(&mut self, received_gift_id: T) -> &mut Self {
    self.inner.received_gift_id = received_gift_id.as_ref().to_string();
    self
  }

  pub fn is_saved(&mut self, is_saved: bool) -> &mut Self {
    self.inner.is_saved = is_saved;
    self
  }

  pub fn can_be_transferred(&mut self, can_be_transferred: bool) -> &mut Self {
    self.inner.can_be_transferred = can_be_transferred;
    self
  }

  pub fn transfer_star_count(&mut self, transfer_star_count: i64) -> &mut Self {
    self.inner.transfer_star_count = transfer_star_count;
    self
  }

  pub fn drop_original_details_star_count(
    &mut self,
    drop_original_details_star_count: i64,
  ) -> &mut Self {
    self.inner.drop_original_details_star_count = drop_original_details_star_count;
    self
  }

  pub fn next_transfer_date(&mut self, next_transfer_date: i64) -> &mut Self {
    self.inner.next_transfer_date = next_transfer_date;
    self
  }

  pub fn next_resale_date(&mut self, next_resale_date: i64) -> &mut Self {
    self.inner.next_resale_date = next_resale_date;
    self
  }

  pub fn export_date(&mut self, export_date: i64) -> &mut Self {
    self.inner.export_date = export_date;
    self
  }
}

impl AsRef<UpgradeGiftResult> for UpgradeGiftResult {
  fn as_ref(&self) -> &UpgradeGiftResult {
    self
  }
}

impl AsRef<UpgradeGiftResult> for RTDUpgradeGiftResultBuilder {
  fn as_ref(&self) -> &UpgradeGiftResult {
    &self.inner
  }
}
