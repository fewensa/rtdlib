
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Describes a block of an instant view web page. 
#[typetag::serde(tag = "@struct")]
pub trait PageBlock: Object + RObject + Debug {}






impl PageBlock {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<PageBlock> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDPageBlockType {
  PageBlockAnchor,
  PageBlockAnimation,
  PageBlockAudio,
  PageBlockAuthorDate,
  PageBlockBlockQuote,
  PageBlockChatLink,
  PageBlockCollage,
  PageBlockCover,
  PageBlockDetails,
  PageBlockDivider,
  PageBlockEmbedded,
  PageBlockEmbeddedPost,
  PageBlockFooter,
  PageBlockHeader,
  PageBlockKicker,
  PageBlockList,
  PageBlockMap,
  PageBlockParagraph,
  PageBlockPhoto,
  PageBlockPreformatted,
  PageBlockPullQuote,
  PageBlockRelatedArticles,
  PageBlockSlideshow,
  PageBlockSubheader,
  PageBlockSubtitle,
  PageBlockTable,
  PageBlockTitle,
  PageBlockVideo,
  
}
impl RTDPageBlockType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDPageBlockType)(text.as_ref()) }
}



