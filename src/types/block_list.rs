use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of block list
pub trait TDBlockList: Debug + RObject {}

/// Describes type of block list
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BlockList {
  #[doc(hidden)]
  _Default(()),
  /// The main block list that disallows writing messages to the current user, receiving their status and photo, viewing of stories, and some other actions
  Main(BlockListMain),
  /// The block list that disallows viewing of stories of the current user
  Stories(BlockListStories),
}

impl Default for BlockList {
  fn default() -> Self {
    BlockList::_Default(())
  }
}

impl<'de> Deserialize<'de> for BlockList {
  fn deserialize<D>(deserializer: D) -> Result<BlockList, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      BlockList,
      (blockListMain, Main);
      (blockListStories, Stories);

    )(deserializer)
  }
}

impl RObject for BlockList {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      BlockList::Main(t) => t.td_name(),
      BlockList::Stories(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      BlockList::Main(t) => t.extra(),
      BlockList::Stories(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BlockList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, BlockList::_Default(_))
  }

  pub fn is_main(&self) -> bool {
    matches!(self, BlockList::Main(_))
  }
  pub fn is_stories(&self) -> bool {
    matches!(self, BlockList::Stories(_))
  }

  pub fn on_main<F: FnOnce(&BlockListMain)>(&self, fnc: F) -> &Self {
    if let BlockList::Main(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stories<F: FnOnce(&BlockListStories)>(&self, fnc: F) -> &Self {
    if let BlockList::Stories(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_main(&self) -> Option<&BlockListMain> {
    if let BlockList::Main(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stories(&self) -> Option<&BlockListStories> {
    if let BlockList::Stories(t) = self {
      return Some(t);
    }
    None
  }

  pub fn main<T: AsRef<BlockListMain>>(t: T) -> Self {
    BlockList::Main(t.as_ref().clone())
  }

  pub fn stories<T: AsRef<BlockListStories>>(t: T) -> Self {
    BlockList::Stories(t.as_ref().clone())
  }
}

impl AsRef<BlockList> for BlockList {
  fn as_ref(&self) -> &BlockList {
    self
  }
}

/// The main block list that disallows writing messages to the current user, receiving their status and photo, viewing of stories, and some other actions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockListMain {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BlockListMain {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "blockListMain"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBlockList for BlockListMain {}

impl BlockListMain {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBlockListMainBuilder {
    let inner = BlockListMain {
      td_name: "blockListMain".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBlockListMainBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBlockListMainBuilder {
  inner: BlockListMain,
}

impl RTDBlockListMainBuilder {
  pub fn build(&self) -> BlockListMain {
    self.inner.clone()
  }
}

impl AsRef<BlockListMain> for BlockListMain {
  fn as_ref(&self) -> &BlockListMain {
    self
  }
}

impl AsRef<BlockListMain> for RTDBlockListMainBuilder {
  fn as_ref(&self) -> &BlockListMain {
    &self.inner
  }
}

/// The block list that disallows viewing of stories of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockListStories {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BlockListStories {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "blockListStories"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBlockList for BlockListStories {}

impl BlockListStories {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBlockListStoriesBuilder {
    let inner = BlockListStories {
      td_name: "blockListStories".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBlockListStoriesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBlockListStoriesBuilder {
  inner: BlockListStories,
}

impl RTDBlockListStoriesBuilder {
  pub fn build(&self) -> BlockListStories {
    self.inner.clone()
  }
}

impl AsRef<BlockListStories> for BlockListStories {
  fn as_ref(&self) -> &BlockListStories {
    self
  }
}

impl AsRef<BlockListStories> for RTDBlockListStoriesBuilder {
  fn as_ref(&self) -> &BlockListStories {
    &self.inner
  }
}
