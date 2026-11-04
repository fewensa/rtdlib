use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents an NFT in the TON blockchain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonNft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Address of the collection to which the NFT belongs; may be empty if unknown
  collection_address: String,
  /// Address of the NFT
  address: String,
  /// Address of the owner of the NFT
  owner_address: String,
  /// Index of the NFT among all NFTs from the collection as a decimal number
  index: String,
  /// Name of the NFT
  name: String,
  /// Represents an NFT in the TON blockchain
  description: String,
  /// Image of the NFT; may be null if none
  image: Option<PhotoSize>,
  /// Thumbnail of the NFT; may be null if none
  thumbnail: Option<PhotoSize>,
  /// Content of the NFT; may be null if none
  content: Option<Document>,
  /// Sticker of the NFT; may be null if none
  sticker: Option<Sticker>,
  /// Attributes of the NFT
  attributes: Vec<TonNftAttribute>,
  /// Additional data about the NFT; may be null if none
  #[serde(rename(serialize = "extra", deserialize = "extra"))]
  extra_: Option<JsonValue>,
  /// The upgraded gift corresponding to the NFT; may be null if none. Information about the owner of the gift isn't available. Original details of the gift are available only to the sender and the receiver of the original gift
  gift: Option<UpgradedGift>,
}

impl RObject for TonNft {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonNft"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonNft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonNftBuilder {
    let inner = TonNft {
      td_name: "tonNft".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonNftBuilder { inner }
  }

  pub fn collection_address(&self) -> &String {
    &self.collection_address
  }

  pub fn address(&self) -> &String {
    &self.address
  }

  pub fn owner_address(&self) -> &String {
    &self.owner_address
  }

  pub fn index(&self) -> &String {
    &self.index
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn description(&self) -> &String {
    &self.description
  }

  pub fn image(&self) -> &Option<PhotoSize> {
    &self.image
  }

  pub fn thumbnail(&self) -> &Option<PhotoSize> {
    &self.thumbnail
  }

  pub fn content(&self) -> &Option<Document> {
    &self.content
  }

  pub fn sticker(&self) -> &Option<Sticker> {
    &self.sticker
  }

  pub fn attributes(&self) -> &Vec<TonNftAttribute> {
    &self.attributes
  }

  pub fn extra_(&self) -> &Option<JsonValue> {
    &self.extra_
  }

  pub fn gift(&self) -> &Option<UpgradedGift> {
    &self.gift
  }
}

#[doc(hidden)]
pub struct RTDTonNftBuilder {
  inner: TonNft,
}

impl RTDTonNftBuilder {
  pub fn build(&self) -> TonNft {
    self.inner.clone()
  }

  pub fn collection_address<T: AsRef<str>>(&mut self, collection_address: T) -> &mut Self {
    self.inner.collection_address = collection_address.as_ref().to_string();
    self
  }

  pub fn address<T: AsRef<str>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().to_string();
    self
  }

  pub fn owner_address<T: AsRef<str>>(&mut self, owner_address: T) -> &mut Self {
    self.inner.owner_address = owner_address.as_ref().to_string();
    self
  }

  pub fn index<T: AsRef<str>>(&mut self, index: T) -> &mut Self {
    self.inner.index = index.as_ref().to_string();
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

  pub fn image<T: AsRef<PhotoSize>>(&mut self, image: T) -> &mut Self {
    self.inner.image = Some(image.as_ref().clone());
    self
  }

  pub fn thumbnail<T: AsRef<PhotoSize>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

  pub fn content<T: AsRef<Document>>(&mut self, content: T) -> &mut Self {
    self.inner.content = Some(content.as_ref().clone());
    self
  }

  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }

  pub fn attributes(&mut self, attributes: Vec<TonNftAttribute>) -> &mut Self {
    self.inner.attributes = attributes;
    self
  }

  pub fn extra_<T: AsRef<JsonValue>>(&mut self, extra_: T) -> &mut Self {
    self.inner.extra_ = Some(extra_.as_ref().clone());
    self
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = Some(gift.as_ref().clone());
    self
  }
}

impl AsRef<TonNft> for TonNft {
  fn as_ref(&self) -> &TonNft {
    self
  }
}

impl AsRef<TonNft> for RTDTonNftBuilder {
  fn as_ref(&self) -> &TonNft {
    &self.inner
  }
}
