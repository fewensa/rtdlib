use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an upgraded gift that can be transferred to another owner or transferred to the TON blockchain as an NFT
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGift {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the gift
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Unique identifier of the regular gift from which the gift was upgraded; may be 0 for short period of time for old gifts from database
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  regular_gift_id: isize,
  /// Identifier of the chat that published the gift; 0 if none
  publisher_chat_id: i64,
  /// The title of the upgraded gift
  title: String,
  /// Unique name of the upgraded gift that can be used with internalLinkTypeUpgradedGift or sendResoldGift
  name: String,
  /// Unique number of the upgraded gift among gifts upgraded from the same gift
  number: i64,
  /// Total number of gifts that were upgraded from the same gift
  total_upgraded_count: i64,
  /// The maximum number of gifts that can be upgraded from the same gift
  max_upgraded_count: i64,
  /// True, if the gift was used to craft another gift
  is_burned: bool,
  /// True, if the gift was crafted from other gifts
  is_crafted: bool,
  /// True, if the original gift could have been bought only by Telegram Premium subscribers
  is_premium: bool,
  /// True, if the gift can be used to set a theme in a chat
  is_theme_available: bool,
  /// Identifier of the chat for which the gift is used to set a theme; 0 if none or the gift isn't owned by the current user
  used_theme_chat_id: i64,
  /// Identifier of the user or the chat to which the upgraded gift was assigned from blockchain; may be null if none or unknown
  host_id: Option<MessageSender>,
  /// Identifier of the user or the chat that owns the upgraded gift; may be null if none or unknown
  owner_id: Option<MessageSender>,
  /// Address of the gift NFT owner in TON blockchain; may be empty if none. Append the address to getOption("ton_blockchain_explorer_url") to get a link with information about the address
  owner_address: String,
  /// Name of the owner for the case when owner identifier and address aren't known
  owner_name: String,
  /// Address of the gift NFT in TON blockchain; may be empty if none. Append the address to getOption("ton_blockchain_explorer_url") to get a link with information about the address
  gift_address: String,
  /// True, if the gift NFT can be used as background_nft_address in setTonWalletCardStickers
  can_be_ton_wallet_card_background: bool,
  /// Model of the upgraded gift
  model: UpgradedGiftModel,
  /// Symbol of the upgraded gift
  symbol: UpgradedGiftSymbol,
  /// Backdrop of the upgraded gift
  backdrop: UpgradedGiftBackdrop,
  /// Information about the originally sent gift; may be null if unknown
  original_details: Option<UpgradedGiftOriginalDetails>,
  /// Colors that can be set for user's name, background of empty chat photo, replies to messages and link previews; may be null if none or unknown
  colors: Option<UpgradedGiftColors>,
  /// Resale parameters of the gift; may be null if resale isn't possible
  resale_parameters: Option<GiftResaleParameters>,
  /// True, if an offer to purchase the gift can be sent using sendGiftPurchaseOffer
  can_send_purchase_offer: bool,
  /// Probability that the gift adds to the chance of successful crafting of a new gift; 0 if the gift can't be used for crafting
  craft_probability_per_mille: i64,
  /// ISO 4217 currency code of the currency in which value of the gift is represented; may be empty if unavailable
  value_currency: String,
  /// Estimated value of the gift; in the smallest units of the currency; 0 if unavailable
  value_amount: i64,
  /// Estimated value of the gift in USD; in USD cents; 0 if unavailable
  value_usd_amount: i64,
}

impl RObject for UpgradedGift {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGift"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGift {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftBuilder {
    let inner = UpgradedGift {
      td_name: "upgradedGift".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn regular_gift_id(&self) -> isize {
    self.regular_gift_id
  }

  pub fn publisher_chat_id(&self) -> i64 {
    self.publisher_chat_id
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn number(&self) -> i64 {
    self.number
  }

  pub fn total_upgraded_count(&self) -> i64 {
    self.total_upgraded_count
  }

  pub fn max_upgraded_count(&self) -> i64 {
    self.max_upgraded_count
  }

  pub fn is_burned(&self) -> bool {
    self.is_burned
  }

  pub fn is_crafted(&self) -> bool {
    self.is_crafted
  }

  pub fn is_premium(&self) -> bool {
    self.is_premium
  }

  pub fn is_theme_available(&self) -> bool {
    self.is_theme_available
  }

  pub fn used_theme_chat_id(&self) -> i64 {
    self.used_theme_chat_id
  }

  pub fn host_id(&self) -> &Option<MessageSender> {
    &self.host_id
  }

  pub fn owner_id(&self) -> &Option<MessageSender> {
    &self.owner_id
  }

  pub fn owner_address(&self) -> &String {
    &self.owner_address
  }

  pub fn owner_name(&self) -> &String {
    &self.owner_name
  }

  pub fn gift_address(&self) -> &String {
    &self.gift_address
  }

  pub fn can_be_ton_wallet_card_background(&self) -> bool {
    self.can_be_ton_wallet_card_background
  }

  pub fn model(&self) -> &UpgradedGiftModel {
    &self.model
  }

  pub fn symbol(&self) -> &UpgradedGiftSymbol {
    &self.symbol
  }

  pub fn backdrop(&self) -> &UpgradedGiftBackdrop {
    &self.backdrop
  }

  pub fn original_details(&self) -> &Option<UpgradedGiftOriginalDetails> {
    &self.original_details
  }

  pub fn colors(&self) -> &Option<UpgradedGiftColors> {
    &self.colors
  }

  pub fn resale_parameters(&self) -> &Option<GiftResaleParameters> {
    &self.resale_parameters
  }

  pub fn can_send_purchase_offer(&self) -> bool {
    self.can_send_purchase_offer
  }

  pub fn craft_probability_per_mille(&self) -> i64 {
    self.craft_probability_per_mille
  }

  pub fn value_currency(&self) -> &String {
    &self.value_currency
  }

  pub fn value_amount(&self) -> i64 {
    self.value_amount
  }

  pub fn value_usd_amount(&self) -> i64 {
    self.value_usd_amount
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftBuilder {
  inner: UpgradedGift,
}

impl RTDUpgradedGiftBuilder {
  pub fn build(&self) -> UpgradedGift {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn regular_gift_id(&mut self, regular_gift_id: isize) -> &mut Self {
    self.inner.regular_gift_id = regular_gift_id;
    self
  }

  pub fn publisher_chat_id(&mut self, publisher_chat_id: i64) -> &mut Self {
    self.inner.publisher_chat_id = publisher_chat_id;
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn number(&mut self, number: i64) -> &mut Self {
    self.inner.number = number;
    self
  }

  pub fn total_upgraded_count(&mut self, total_upgraded_count: i64) -> &mut Self {
    self.inner.total_upgraded_count = total_upgraded_count;
    self
  }

  pub fn max_upgraded_count(&mut self, max_upgraded_count: i64) -> &mut Self {
    self.inner.max_upgraded_count = max_upgraded_count;
    self
  }

  pub fn is_burned(&mut self, is_burned: bool) -> &mut Self {
    self.inner.is_burned = is_burned;
    self
  }

  pub fn is_crafted(&mut self, is_crafted: bool) -> &mut Self {
    self.inner.is_crafted = is_crafted;
    self
  }

  pub fn is_premium(&mut self, is_premium: bool) -> &mut Self {
    self.inner.is_premium = is_premium;
    self
  }

  pub fn is_theme_available(&mut self, is_theme_available: bool) -> &mut Self {
    self.inner.is_theme_available = is_theme_available;
    self
  }

  pub fn used_theme_chat_id(&mut self, used_theme_chat_id: i64) -> &mut Self {
    self.inner.used_theme_chat_id = used_theme_chat_id;
    self
  }

  pub fn host_id<T: AsRef<MessageSender>>(&mut self, host_id: T) -> &mut Self {
    self.inner.host_id = Some(host_id.as_ref().clone());
    self
  }

  pub fn owner_id<T: AsRef<MessageSender>>(&mut self, owner_id: T) -> &mut Self {
    self.inner.owner_id = Some(owner_id.as_ref().clone());
    self
  }

  pub fn owner_address<T: AsRef<str>>(&mut self, owner_address: T) -> &mut Self {
    self.inner.owner_address = owner_address.as_ref().to_string();
    self
  }

  pub fn owner_name<T: AsRef<str>>(&mut self, owner_name: T) -> &mut Self {
    self.inner.owner_name = owner_name.as_ref().to_string();
    self
  }

  pub fn gift_address<T: AsRef<str>>(&mut self, gift_address: T) -> &mut Self {
    self.inner.gift_address = gift_address.as_ref().to_string();
    self
  }

  pub fn can_be_ton_wallet_card_background(
    &mut self,
    can_be_ton_wallet_card_background: bool,
  ) -> &mut Self {
    self.inner.can_be_ton_wallet_card_background = can_be_ton_wallet_card_background;
    self
  }

  pub fn model<T: AsRef<UpgradedGiftModel>>(&mut self, model: T) -> &mut Self {
    self.inner.model = model.as_ref().clone();
    self
  }

  pub fn symbol<T: AsRef<UpgradedGiftSymbol>>(&mut self, symbol: T) -> &mut Self {
    self.inner.symbol = symbol.as_ref().clone();
    self
  }

  pub fn backdrop<T: AsRef<UpgradedGiftBackdrop>>(&mut self, backdrop: T) -> &mut Self {
    self.inner.backdrop = backdrop.as_ref().clone();
    self
  }

  pub fn original_details<T: AsRef<UpgradedGiftOriginalDetails>>(
    &mut self,
    original_details: T,
  ) -> &mut Self {
    self.inner.original_details = Some(original_details.as_ref().clone());
    self
  }

  pub fn colors<T: AsRef<UpgradedGiftColors>>(&mut self, colors: T) -> &mut Self {
    self.inner.colors = Some(colors.as_ref().clone());
    self
  }

  pub fn resale_parameters<T: AsRef<GiftResaleParameters>>(
    &mut self,
    resale_parameters: T,
  ) -> &mut Self {
    self.inner.resale_parameters = Some(resale_parameters.as_ref().clone());
    self
  }

  pub fn can_send_purchase_offer(&mut self, can_send_purchase_offer: bool) -> &mut Self {
    self.inner.can_send_purchase_offer = can_send_purchase_offer;
    self
  }

  pub fn craft_probability_per_mille(&mut self, craft_probability_per_mille: i64) -> &mut Self {
    self.inner.craft_probability_per_mille = craft_probability_per_mille;
    self
  }

  pub fn value_currency<T: AsRef<str>>(&mut self, value_currency: T) -> &mut Self {
    self.inner.value_currency = value_currency.as_ref().to_string();
    self
  }

  pub fn value_amount(&mut self, value_amount: i64) -> &mut Self {
    self.inner.value_amount = value_amount;
    self
  }

  pub fn value_usd_amount(&mut self, value_usd_amount: i64) -> &mut Self {
    self.inner.value_usd_amount = value_usd_amount;
    self
  }
}

impl AsRef<UpgradedGift> for UpgradedGift {
  fn as_ref(&self) -> &UpgradedGift {
    self
  }
}

impl AsRef<UpgradedGift> for RTDUpgradedGiftBuilder {
  fn as_ref(&self) -> &UpgradedGift {
    &self.inner
  }
}
