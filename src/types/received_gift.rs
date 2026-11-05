use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a gift received by a user or a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReceivedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the received gift for the current user; only for the receiver of the gift
  received_gift_id: String,
  /// Identifier of a user or a chat that sent the gift; may be null if unknown
  sender_id: Option<MessageSender>,
  /// Message added to the gift
  text: FormattedText,
  /// Unique number of the gift among gifts upgraded from the same gift after upgrade; 0 if yet unassigned
  unique_gift_number: i64,
  /// True, if the sender and gift text are shown only to the gift receiver; otherwise, everyone is able to see them
  is_private: bool,
  /// True, if the gift is displayed on the chat's profile page; only for the receiver of the gift
  is_saved: bool,
  /// True, if the gift is pinned to the top of the chat's profile page
  is_pinned: bool,
  /// True, if the gift is a regular gift that can be upgraded to a unique gift; only for the receiver of the gift
  can_be_upgraded: bool,
  /// True, if the gift is an upgraded gift that can be transferred to another owner; only for the receiver of the gift
  can_be_transferred: bool,
  /// True, if the gift is a regular gift that can be deleted using deleteGift; only for the receiver of the gift
  can_be_deleted: bool,
  /// True, if the gift was refunded and isn't available anymore
  was_refunded: bool,
  /// Point in time (Unix timestamp) when the gift was sent
  date: i64,
  /// The gift
  gift: SentGift,
  /// Identifiers of collections to which the gift is added; only for the receiver of the gift
  collection_ids: Vec<i64>,
  /// Number of Telegram Stars that can be claimed by the receiver instead of the regular gift; 0 if the gift can't be sold by the current user
  sell_star_count: i64,
  /// Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift
  prepaid_upgrade_star_count: i64,
  /// True, if the upgrade was bought after the gift was sent. In this case, prepaid upgrade cost must not be added to the gift cost
  is_upgrade_separate: bool,
  /// Number of Telegram Stars that must be paid to transfer the upgraded gift; only for the receiver of the gift
  transfer_star_count: i64,
  /// Number of Telegram Stars that must be paid to drop original details of the upgraded gift; 0 if not available; only for the receiver of the gift
  drop_original_details_star_count: i64,
  /// Point in time (Unix timestamp) when the gift can be transferred to another owner; can be in the past; 0 if the gift can be transferred immediately or transfer isn't possible; only for the receiver of the gift
  next_transfer_date: i64,
  /// Point in time (Unix timestamp) when the gift can be resold to another user; can be in the past; 0 if the gift can't be resold; only for the receiver of the gift
  next_resale_date: i64,
  /// Point in time (Unix timestamp) when the upgraded gift can be transferred to the TON blockchain as an NFT; can be in the past; 0 if NFT export isn't possible; only for the receiver of the gift
  export_date: i64,
  /// If non-empty, then the user can pay for an upgrade of the gift using buyGiftUpgrade
  prepaid_upgrade_hash: String,
  /// Point in time (Unix timestamp) when the gift can be used to craft another gift; can be in the past; only for the receiver of the gift
  craft_date: i64,
}

impl RObject for ReceivedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "receivedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReceivedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReceivedGiftBuilder {
    let inner = ReceivedGift {
      td_name: "receivedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReceivedGiftBuilder { inner }
  }

  pub fn received_gift_id(&self) -> &String {
    &self.received_gift_id
  }

  pub fn sender_id(&self) -> &Option<MessageSender> {
    &self.sender_id
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn unique_gift_number(&self) -> i64 {
    self.unique_gift_number
  }

  pub fn is_private(&self) -> bool {
    self.is_private
  }

  pub fn is_saved(&self) -> bool {
    self.is_saved
  }

  pub fn is_pinned(&self) -> bool {
    self.is_pinned
  }

  pub fn can_be_upgraded(&self) -> bool {
    self.can_be_upgraded
  }

  pub fn can_be_transferred(&self) -> bool {
    self.can_be_transferred
  }

  pub fn can_be_deleted(&self) -> bool {
    self.can_be_deleted
  }

  pub fn was_refunded(&self) -> bool {
    self.was_refunded
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn gift(&self) -> &SentGift {
    &self.gift
  }

  pub fn collection_ids(&self) -> &Vec<i64> {
    &self.collection_ids
  }

  pub fn sell_star_count(&self) -> i64 {
    self.sell_star_count
  }

  pub fn prepaid_upgrade_star_count(&self) -> i64 {
    self.prepaid_upgrade_star_count
  }

  pub fn is_upgrade_separate(&self) -> bool {
    self.is_upgrade_separate
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

  pub fn prepaid_upgrade_hash(&self) -> &String {
    &self.prepaid_upgrade_hash
  }

  pub fn craft_date(&self) -> i64 {
    self.craft_date
  }
}

#[doc(hidden)]
pub struct RTDReceivedGiftBuilder {
  inner: ReceivedGift,
}

impl RTDReceivedGiftBuilder {
  pub fn build(&self) -> ReceivedGift {
    self.inner.clone()
  }

  pub fn received_gift_id<T: AsRef<str>>(&mut self, received_gift_id: T) -> &mut Self {
    self.inner.received_gift_id = received_gift_id.as_ref().to_string();
    self
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = Some(sender_id.as_ref().clone());
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn unique_gift_number(&mut self, unique_gift_number: i64) -> &mut Self {
    self.inner.unique_gift_number = unique_gift_number;
    self
  }

  pub fn is_private(&mut self, is_private: bool) -> &mut Self {
    self.inner.is_private = is_private;
    self
  }

  pub fn is_saved(&mut self, is_saved: bool) -> &mut Self {
    self.inner.is_saved = is_saved;
    self
  }

  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }

  pub fn can_be_upgraded(&mut self, can_be_upgraded: bool) -> &mut Self {
    self.inner.can_be_upgraded = can_be_upgraded;
    self
  }

  pub fn can_be_transferred(&mut self, can_be_transferred: bool) -> &mut Self {
    self.inner.can_be_transferred = can_be_transferred;
    self
  }

  pub fn can_be_deleted(&mut self, can_be_deleted: bool) -> &mut Self {
    self.inner.can_be_deleted = can_be_deleted;
    self
  }

  pub fn was_refunded(&mut self, was_refunded: bool) -> &mut Self {
    self.inner.was_refunded = was_refunded;
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn gift<T: AsRef<SentGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }

  pub fn collection_ids(&mut self, collection_ids: Vec<i64>) -> &mut Self {
    self.inner.collection_ids = collection_ids;
    self
  }

  pub fn sell_star_count(&mut self, sell_star_count: i64) -> &mut Self {
    self.inner.sell_star_count = sell_star_count;
    self
  }

  pub fn prepaid_upgrade_star_count(&mut self, prepaid_upgrade_star_count: i64) -> &mut Self {
    self.inner.prepaid_upgrade_star_count = prepaid_upgrade_star_count;
    self
  }

  pub fn is_upgrade_separate(&mut self, is_upgrade_separate: bool) -> &mut Self {
    self.inner.is_upgrade_separate = is_upgrade_separate;
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

  pub fn prepaid_upgrade_hash<T: AsRef<str>>(&mut self, prepaid_upgrade_hash: T) -> &mut Self {
    self.inner.prepaid_upgrade_hash = prepaid_upgrade_hash.as_ref().to_string();
    self
  }

  pub fn craft_date(&mut self, craft_date: i64) -> &mut Self {
    self.inner.craft_date = craft_date;
    self
  }
}

impl AsRef<ReceivedGift> for ReceivedGift {
  fn as_ref(&self) -> &ReceivedGift {
    self
  }
}

impl AsRef<ReceivedGift> for RTDReceivedGiftBuilder {
  fn as_ref(&self) -> &ReceivedGift {
    &self.inner
  }
}
