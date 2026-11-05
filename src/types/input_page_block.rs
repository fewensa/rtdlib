use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a block of a rich message to send
pub trait TDInputPageBlock: Debug + RObject {}

/// Describes a block of a rich message to send
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputPageBlock {
  #[doc(hidden)]
  _Default(()),
  /// An invisible anchor
  Anchor(InputPageBlockAnchor),
  /// An animation
  Animation(InputPageBlockAnimation),
  /// An audio file
  Audio(InputPageBlockAudio),
  /// A block quote
  BlockQuote(InputPageBlockBlockQuote),
  /// A list of buttons shown in a row
  ButtonRow(InputPageBlockButtonRow),
  /// A collage
  Collage(InputPageBlockCollage),
  /// A collapsible block
  Details(InputPageBlockDetails),
  /// An empty block separating the page
  Divider(InputPageBlockDivider),
  /// A general file
  Document(InputPageBlockDocument),
  /// An expandable block quote
  ExpandableBlockQuote(InputPageBlockExpandableBlockQuote),
  /// The footer of the page
  Footer(InputPageBlockFooter),
  /// A list of data blocks
  List(InputPageBlockList),
  /// A map. The map's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
  Map(InputPageBlockMap),
  /// A mathematical expression
  MathematicalExpression(InputPageBlockMathematicalExpression),
  /// A text paragraph
  Paragraph(InputPageBlockParagraph),
  /// A photo
  Photo(InputPageBlockPhoto),
  /// A preformatted text paragraph
  Preformatted(InputPageBlockPreformatted),
  /// A pull quote
  PullQuote(InputPageBlockPullQuote),
  /// A section heading
  SectionHeading(InputPageBlockSectionHeading),
  /// A slideshow
  Slideshow(InputPageBlockSlideshow),
  /// A table
  Table(InputPageBlockTable),
  /// A "Thinking..." placeholder; for pending rich messages only; for bots only
  Thinking(InputPageBlockThinking),
  /// A video
  Video(InputPageBlockVideo),
  /// A voice note
  VoiceNote(InputPageBlockVoiceNote),
}

impl Default for InputPageBlock {
  fn default() -> Self {
    InputPageBlock::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputPageBlock {
  fn deserialize<D>(deserializer: D) -> Result<InputPageBlock, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputPageBlock,
      (inputPageBlockAnchor, Anchor);
      (inputPageBlockAnimation, Animation);
      (inputPageBlockAudio, Audio);
      (inputPageBlockBlockQuote, BlockQuote);
      (inputPageBlockButtonRow, ButtonRow);
      (inputPageBlockCollage, Collage);
      (inputPageBlockDetails, Details);
      (inputPageBlockDivider, Divider);
      (inputPageBlockDocument, Document);
      (inputPageBlockExpandableBlockQuote, ExpandableBlockQuote);
      (inputPageBlockFooter, Footer);
      (inputPageBlockList, List);
      (inputPageBlockMap, Map);
      (inputPageBlockMathematicalExpression, MathematicalExpression);
      (inputPageBlockParagraph, Paragraph);
      (inputPageBlockPhoto, Photo);
      (inputPageBlockPreformatted, Preformatted);
      (inputPageBlockPullQuote, PullQuote);
      (inputPageBlockSectionHeading, SectionHeading);
      (inputPageBlockSlideshow, Slideshow);
      (inputPageBlockTable, Table);
      (inputPageBlockThinking, Thinking);
      (inputPageBlockVideo, Video);
      (inputPageBlockVoiceNote, VoiceNote);

    )(deserializer)
  }
}

impl RObject for InputPageBlock {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputPageBlock::Anchor(t) => t.td_name(),
      InputPageBlock::Animation(t) => t.td_name(),
      InputPageBlock::Audio(t) => t.td_name(),
      InputPageBlock::BlockQuote(t) => t.td_name(),
      InputPageBlock::ButtonRow(t) => t.td_name(),
      InputPageBlock::Collage(t) => t.td_name(),
      InputPageBlock::Details(t) => t.td_name(),
      InputPageBlock::Divider(t) => t.td_name(),
      InputPageBlock::Document(t) => t.td_name(),
      InputPageBlock::ExpandableBlockQuote(t) => t.td_name(),
      InputPageBlock::Footer(t) => t.td_name(),
      InputPageBlock::List(t) => t.td_name(),
      InputPageBlock::Map(t) => t.td_name(),
      InputPageBlock::MathematicalExpression(t) => t.td_name(),
      InputPageBlock::Paragraph(t) => t.td_name(),
      InputPageBlock::Photo(t) => t.td_name(),
      InputPageBlock::Preformatted(t) => t.td_name(),
      InputPageBlock::PullQuote(t) => t.td_name(),
      InputPageBlock::SectionHeading(t) => t.td_name(),
      InputPageBlock::Slideshow(t) => t.td_name(),
      InputPageBlock::Table(t) => t.td_name(),
      InputPageBlock::Thinking(t) => t.td_name(),
      InputPageBlock::Video(t) => t.td_name(),
      InputPageBlock::VoiceNote(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputPageBlock::Anchor(t) => t.extra(),
      InputPageBlock::Animation(t) => t.extra(),
      InputPageBlock::Audio(t) => t.extra(),
      InputPageBlock::BlockQuote(t) => t.extra(),
      InputPageBlock::ButtonRow(t) => t.extra(),
      InputPageBlock::Collage(t) => t.extra(),
      InputPageBlock::Details(t) => t.extra(),
      InputPageBlock::Divider(t) => t.extra(),
      InputPageBlock::Document(t) => t.extra(),
      InputPageBlock::ExpandableBlockQuote(t) => t.extra(),
      InputPageBlock::Footer(t) => t.extra(),
      InputPageBlock::List(t) => t.extra(),
      InputPageBlock::Map(t) => t.extra(),
      InputPageBlock::MathematicalExpression(t) => t.extra(),
      InputPageBlock::Paragraph(t) => t.extra(),
      InputPageBlock::Photo(t) => t.extra(),
      InputPageBlock::Preformatted(t) => t.extra(),
      InputPageBlock::PullQuote(t) => t.extra(),
      InputPageBlock::SectionHeading(t) => t.extra(),
      InputPageBlock::Slideshow(t) => t.extra(),
      InputPageBlock::Table(t) => t.extra(),
      InputPageBlock::Thinking(t) => t.extra(),
      InputPageBlock::Video(t) => t.extra(),
      InputPageBlock::VoiceNote(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputPageBlock {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputPageBlock::_Default(_))
  }

  pub fn is_anchor(&self) -> bool {
    matches!(self, InputPageBlock::Anchor(_))
  }
  pub fn is_animation(&self) -> bool {
    matches!(self, InputPageBlock::Animation(_))
  }
  pub fn is_audio(&self) -> bool {
    matches!(self, InputPageBlock::Audio(_))
  }
  pub fn is_block_quote(&self) -> bool {
    matches!(self, InputPageBlock::BlockQuote(_))
  }
  pub fn is_button_row(&self) -> bool {
    matches!(self, InputPageBlock::ButtonRow(_))
  }
  pub fn is_collage(&self) -> bool {
    matches!(self, InputPageBlock::Collage(_))
  }
  pub fn is_details(&self) -> bool {
    matches!(self, InputPageBlock::Details(_))
  }
  pub fn is_divider(&self) -> bool {
    matches!(self, InputPageBlock::Divider(_))
  }
  pub fn is_document(&self) -> bool {
    matches!(self, InputPageBlock::Document(_))
  }
  pub fn is_expandable_block_quote(&self) -> bool {
    matches!(self, InputPageBlock::ExpandableBlockQuote(_))
  }
  pub fn is_footer(&self) -> bool {
    matches!(self, InputPageBlock::Footer(_))
  }
  pub fn is_list(&self) -> bool {
    matches!(self, InputPageBlock::List(_))
  }
  pub fn is_map(&self) -> bool {
    matches!(self, InputPageBlock::Map(_))
  }
  pub fn is_mathematical_expression(&self) -> bool {
    matches!(self, InputPageBlock::MathematicalExpression(_))
  }
  pub fn is_paragraph(&self) -> bool {
    matches!(self, InputPageBlock::Paragraph(_))
  }
  pub fn is_photo(&self) -> bool {
    matches!(self, InputPageBlock::Photo(_))
  }
  pub fn is_preformatted(&self) -> bool {
    matches!(self, InputPageBlock::Preformatted(_))
  }
  pub fn is_pull_quote(&self) -> bool {
    matches!(self, InputPageBlock::PullQuote(_))
  }
  pub fn is_section_heading(&self) -> bool {
    matches!(self, InputPageBlock::SectionHeading(_))
  }
  pub fn is_slideshow(&self) -> bool {
    matches!(self, InputPageBlock::Slideshow(_))
  }
  pub fn is_table(&self) -> bool {
    matches!(self, InputPageBlock::Table(_))
  }
  pub fn is_thinking(&self) -> bool {
    matches!(self, InputPageBlock::Thinking(_))
  }
  pub fn is_video(&self) -> bool {
    matches!(self, InputPageBlock::Video(_))
  }
  pub fn is_voice_note(&self) -> bool {
    matches!(self, InputPageBlock::VoiceNote(_))
  }

  pub fn on_anchor<F: FnOnce(&InputPageBlockAnchor)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Anchor(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_animation<F: FnOnce(&InputPageBlockAnimation)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Animation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_audio<F: FnOnce(&InputPageBlockAudio)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Audio(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_block_quote<F: FnOnce(&InputPageBlockBlockQuote)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::BlockQuote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_button_row<F: FnOnce(&InputPageBlockButtonRow)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::ButtonRow(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_collage<F: FnOnce(&InputPageBlockCollage)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Collage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_details<F: FnOnce(&InputPageBlockDetails)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Details(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_divider<F: FnOnce(&InputPageBlockDivider)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Divider(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_document<F: FnOnce(&InputPageBlockDocument)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Document(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_expandable_block_quote<F: FnOnce(&InputPageBlockExpandableBlockQuote)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InputPageBlock::ExpandableBlockQuote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_footer<F: FnOnce(&InputPageBlockFooter)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Footer(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_list<F: FnOnce(&InputPageBlockList)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::List(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_map<F: FnOnce(&InputPageBlockMap)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Map(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_mathematical_expression<F: FnOnce(&InputPageBlockMathematicalExpression)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let InputPageBlock::MathematicalExpression(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_paragraph<F: FnOnce(&InputPageBlockParagraph)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Paragraph(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_photo<F: FnOnce(&InputPageBlockPhoto)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Photo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_preformatted<F: FnOnce(&InputPageBlockPreformatted)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Preformatted(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pull_quote<F: FnOnce(&InputPageBlockPullQuote)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::PullQuote(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_section_heading<F: FnOnce(&InputPageBlockSectionHeading)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::SectionHeading(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_slideshow<F: FnOnce(&InputPageBlockSlideshow)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Slideshow(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_table<F: FnOnce(&InputPageBlockTable)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Table(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_thinking<F: FnOnce(&InputPageBlockThinking)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Thinking(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video<F: FnOnce(&InputPageBlockVideo)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::Video(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_voice_note<F: FnOnce(&InputPageBlockVoiceNote)>(&self, fnc: F) -> &Self {
    if let InputPageBlock::VoiceNote(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_anchor(&self) -> Option<&InputPageBlockAnchor> {
    if let InputPageBlock::Anchor(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_animation(&self) -> Option<&InputPageBlockAnimation> {
    if let InputPageBlock::Animation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_audio(&self) -> Option<&InputPageBlockAudio> {
    if let InputPageBlock::Audio(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_block_quote(&self) -> Option<&InputPageBlockBlockQuote> {
    if let InputPageBlock::BlockQuote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_button_row(&self) -> Option<&InputPageBlockButtonRow> {
    if let InputPageBlock::ButtonRow(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_collage(&self) -> Option<&InputPageBlockCollage> {
    if let InputPageBlock::Collage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_details(&self) -> Option<&InputPageBlockDetails> {
    if let InputPageBlock::Details(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_divider(&self) -> Option<&InputPageBlockDivider> {
    if let InputPageBlock::Divider(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_document(&self) -> Option<&InputPageBlockDocument> {
    if let InputPageBlock::Document(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_expandable_block_quote(&self) -> Option<&InputPageBlockExpandableBlockQuote> {
    if let InputPageBlock::ExpandableBlockQuote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_footer(&self) -> Option<&InputPageBlockFooter> {
    if let InputPageBlock::Footer(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_list(&self) -> Option<&InputPageBlockList> {
    if let InputPageBlock::List(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_map(&self) -> Option<&InputPageBlockMap> {
    if let InputPageBlock::Map(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_mathematical_expression(&self) -> Option<&InputPageBlockMathematicalExpression> {
    if let InputPageBlock::MathematicalExpression(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_paragraph(&self) -> Option<&InputPageBlockParagraph> {
    if let InputPageBlock::Paragraph(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_photo(&self) -> Option<&InputPageBlockPhoto> {
    if let InputPageBlock::Photo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_preformatted(&self) -> Option<&InputPageBlockPreformatted> {
    if let InputPageBlock::Preformatted(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pull_quote(&self) -> Option<&InputPageBlockPullQuote> {
    if let InputPageBlock::PullQuote(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_section_heading(&self) -> Option<&InputPageBlockSectionHeading> {
    if let InputPageBlock::SectionHeading(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_slideshow(&self) -> Option<&InputPageBlockSlideshow> {
    if let InputPageBlock::Slideshow(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_table(&self) -> Option<&InputPageBlockTable> {
    if let InputPageBlock::Table(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_thinking(&self) -> Option<&InputPageBlockThinking> {
    if let InputPageBlock::Thinking(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video(&self) -> Option<&InputPageBlockVideo> {
    if let InputPageBlock::Video(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_voice_note(&self) -> Option<&InputPageBlockVoiceNote> {
    if let InputPageBlock::VoiceNote(t) = self {
      return Some(t);
    }
    None
  }

  pub fn anchor<T: AsRef<InputPageBlockAnchor>>(t: T) -> Self {
    InputPageBlock::Anchor(t.as_ref().clone())
  }

  pub fn animation<T: AsRef<InputPageBlockAnimation>>(t: T) -> Self {
    InputPageBlock::Animation(t.as_ref().clone())
  }

  pub fn audio<T: AsRef<InputPageBlockAudio>>(t: T) -> Self {
    InputPageBlock::Audio(t.as_ref().clone())
  }

  pub fn block_quote<T: AsRef<InputPageBlockBlockQuote>>(t: T) -> Self {
    InputPageBlock::BlockQuote(t.as_ref().clone())
  }

  pub fn button_row<T: AsRef<InputPageBlockButtonRow>>(t: T) -> Self {
    InputPageBlock::ButtonRow(t.as_ref().clone())
  }

  pub fn collage<T: AsRef<InputPageBlockCollage>>(t: T) -> Self {
    InputPageBlock::Collage(t.as_ref().clone())
  }

  pub fn details<T: AsRef<InputPageBlockDetails>>(t: T) -> Self {
    InputPageBlock::Details(t.as_ref().clone())
  }

  pub fn divider<T: AsRef<InputPageBlockDivider>>(t: T) -> Self {
    InputPageBlock::Divider(t.as_ref().clone())
  }

  pub fn document<T: AsRef<InputPageBlockDocument>>(t: T) -> Self {
    InputPageBlock::Document(t.as_ref().clone())
  }

  pub fn expandable_block_quote<T: AsRef<InputPageBlockExpandableBlockQuote>>(t: T) -> Self {
    InputPageBlock::ExpandableBlockQuote(t.as_ref().clone())
  }

  pub fn footer<T: AsRef<InputPageBlockFooter>>(t: T) -> Self {
    InputPageBlock::Footer(t.as_ref().clone())
  }

  pub fn list<T: AsRef<InputPageBlockList>>(t: T) -> Self {
    InputPageBlock::List(t.as_ref().clone())
  }

  pub fn map<T: AsRef<InputPageBlockMap>>(t: T) -> Self {
    InputPageBlock::Map(t.as_ref().clone())
  }

  pub fn mathematical_expression<T: AsRef<InputPageBlockMathematicalExpression>>(t: T) -> Self {
    InputPageBlock::MathematicalExpression(t.as_ref().clone())
  }

  pub fn paragraph<T: AsRef<InputPageBlockParagraph>>(t: T) -> Self {
    InputPageBlock::Paragraph(t.as_ref().clone())
  }

  pub fn photo<T: AsRef<InputPageBlockPhoto>>(t: T) -> Self {
    InputPageBlock::Photo(t.as_ref().clone())
  }

  pub fn preformatted<T: AsRef<InputPageBlockPreformatted>>(t: T) -> Self {
    InputPageBlock::Preformatted(t.as_ref().clone())
  }

  pub fn pull_quote<T: AsRef<InputPageBlockPullQuote>>(t: T) -> Self {
    InputPageBlock::PullQuote(t.as_ref().clone())
  }

  pub fn section_heading<T: AsRef<InputPageBlockSectionHeading>>(t: T) -> Self {
    InputPageBlock::SectionHeading(t.as_ref().clone())
  }

  pub fn slideshow<T: AsRef<InputPageBlockSlideshow>>(t: T) -> Self {
    InputPageBlock::Slideshow(t.as_ref().clone())
  }

  pub fn table<T: AsRef<InputPageBlockTable>>(t: T) -> Self {
    InputPageBlock::Table(t.as_ref().clone())
  }

  pub fn thinking<T: AsRef<InputPageBlockThinking>>(t: T) -> Self {
    InputPageBlock::Thinking(t.as_ref().clone())
  }

  pub fn video<T: AsRef<InputPageBlockVideo>>(t: T) -> Self {
    InputPageBlock::Video(t.as_ref().clone())
  }

  pub fn voice_note<T: AsRef<InputPageBlockVoiceNote>>(t: T) -> Self {
    InputPageBlock::VoiceNote(t.as_ref().clone())
  }
}

impl AsRef<InputPageBlock> for InputPageBlock {
  fn as_ref(&self) -> &InputPageBlock {
    self
  }
}

/// An invisible anchor
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockAnchor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the anchor
  name: String,
}

impl RObject for InputPageBlockAnchor {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockAnchor"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockAnchor {}

impl InputPageBlockAnchor {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockAnchorBuilder {
    let inner = InputPageBlockAnchor {
      td_name: "inputPageBlockAnchor".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockAnchorBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockAnchorBuilder {
  inner: InputPageBlockAnchor,
}

impl RTDInputPageBlockAnchorBuilder {
  pub fn build(&self) -> InputPageBlockAnchor {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }
}

impl AsRef<InputPageBlockAnchor> for InputPageBlockAnchor {
  fn as_ref(&self) -> &InputPageBlockAnchor {
    self
  }
}

impl AsRef<InputPageBlockAnchor> for RTDInputPageBlockAnchorBuilder {
  fn as_ref(&self) -> &InputPageBlockAnchor {
    &self.inner
  }
}

/// An animation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The animation to be sent
  animation: InputAnimation,
  /// Animation caption; pass null if none
  caption: PageBlockCaption,
  /// True, if the animation preview must be covered by a spoiler animation
  has_spoiler: bool,
}

impl RObject for InputPageBlockAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockAnimation {}

impl InputPageBlockAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockAnimationBuilder {
    let inner = InputPageBlockAnimation {
      td_name: "inputPageBlockAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &InputAnimation {
    &self.animation
  }

  pub fn caption(&self) -> &PageBlockCaption {
    &self.caption
  }

  pub fn has_spoiler(&self) -> bool {
    self.has_spoiler
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockAnimationBuilder {
  inner: InputPageBlockAnimation,
}

impl RTDInputPageBlockAnimationBuilder {
  pub fn build(&self) -> InputPageBlockAnimation {
    self.inner.clone()
  }

  pub fn animation<T: AsRef<InputAnimation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn has_spoiler(&mut self, has_spoiler: bool) -> &mut Self {
    self.inner.has_spoiler = has_spoiler;
    self
  }
}

impl AsRef<InputPageBlockAnimation> for InputPageBlockAnimation {
  fn as_ref(&self) -> &InputPageBlockAnimation {
    self
  }
}

impl AsRef<InputPageBlockAnimation> for RTDInputPageBlockAnimationBuilder {
  fn as_ref(&self) -> &InputPageBlockAnimation {
    &self.inner
  }
}

/// An audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The audio to be sent
  audio: InputAudio,
  /// Audio file caption; pass null if none
  caption: PageBlockCaption,
}

impl RObject for InputPageBlockAudio {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockAudio"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockAudio {}

impl InputPageBlockAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockAudioBuilder {
    let inner = InputPageBlockAudio {
      td_name: "inputPageBlockAudio".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockAudioBuilder { inner }
  }

  pub fn audio(&self) -> &InputAudio {
    &self.audio
  }

  pub fn caption(&self) -> &PageBlockCaption {
    &self.caption
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockAudioBuilder {
  inner: InputPageBlockAudio,
}

impl RTDInputPageBlockAudioBuilder {
  pub fn build(&self) -> InputPageBlockAudio {
    self.inner.clone()
  }

  pub fn audio<T: AsRef<InputAudio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockAudio> for InputPageBlockAudio {
  fn as_ref(&self) -> &InputPageBlockAudio {
    self
  }
}

impl AsRef<InputPageBlockAudio> for RTDInputPageBlockAudioBuilder {
  fn as_ref(&self) -> &InputPageBlockAudio {
    &self.inner
  }
}

/// A block quote
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockBlockQuote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Quote blocks
  blocks: Vec<InputPageBlock>,
  /// Quote credit; pass null if none
  credit: RichText,
}

impl RObject for InputPageBlockBlockQuote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockBlockQuote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockBlockQuote {}

impl InputPageBlockBlockQuote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockBlockQuoteBuilder {
    let inner = InputPageBlockBlockQuote {
      td_name: "inputPageBlockBlockQuote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockBlockQuoteBuilder { inner }
  }

  pub fn blocks(&self) -> &Vec<InputPageBlock> {
    &self.blocks
  }

  pub fn credit(&self) -> &RichText {
    &self.credit
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockBlockQuoteBuilder {
  inner: InputPageBlockBlockQuote,
}

impl RTDInputPageBlockBlockQuoteBuilder {
  pub fn build(&self) -> InputPageBlockBlockQuote {
    self.inner.clone()
  }

  pub fn blocks(&mut self, blocks: Vec<InputPageBlock>) -> &mut Self {
    self.inner.blocks = blocks;
    self
  }

  pub fn credit<T: AsRef<RichText>>(&mut self, credit: T) -> &mut Self {
    self.inner.credit = credit.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockBlockQuote> for InputPageBlockBlockQuote {
  fn as_ref(&self) -> &InputPageBlockBlockQuote {
    self
  }
}

impl AsRef<InputPageBlockBlockQuote> for RTDInputPageBlockBlockQuoteBuilder {
  fn as_ref(&self) -> &InputPageBlockBlockQuote {
    &self.inner
  }
}

/// A list of buttons shown in a row
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockButtonRow {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The buttons
  buttons: Vec<InlineButton>,
  /// Horizontal alignment of the buttons; pass null if the buttons must be shown full-width
  align: PageBlockHorizontalAlignment,
}

impl RObject for InputPageBlockButtonRow {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockButtonRow"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockButtonRow {}

impl InputPageBlockButtonRow {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockButtonRowBuilder {
    let inner = InputPageBlockButtonRow {
      td_name: "inputPageBlockButtonRow".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockButtonRowBuilder { inner }
  }

  pub fn buttons(&self) -> &Vec<InlineButton> {
    &self.buttons
  }

  pub fn align(&self) -> &PageBlockHorizontalAlignment {
    &self.align
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockButtonRowBuilder {
  inner: InputPageBlockButtonRow,
}

impl RTDInputPageBlockButtonRowBuilder {
  pub fn build(&self) -> InputPageBlockButtonRow {
    self.inner.clone()
  }

  pub fn buttons(&mut self, buttons: Vec<InlineButton>) -> &mut Self {
    self.inner.buttons = buttons;
    self
  }

  pub fn align<T: AsRef<PageBlockHorizontalAlignment>>(&mut self, align: T) -> &mut Self {
    self.inner.align = align.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockButtonRow> for InputPageBlockButtonRow {
  fn as_ref(&self) -> &InputPageBlockButtonRow {
    self
  }
}

impl AsRef<InputPageBlockButtonRow> for RTDInputPageBlockButtonRowBuilder {
  fn as_ref(&self) -> &InputPageBlockButtonRow {
    &self.inner
  }
}

/// A collage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockCollage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Collage item contents
  blocks: Vec<InputPageBlock>,
  /// Block caption; pass null if none
  caption: PageBlockCaption,
}

impl RObject for InputPageBlockCollage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockCollage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockCollage {}

impl InputPageBlockCollage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockCollageBuilder {
    let inner = InputPageBlockCollage {
      td_name: "inputPageBlockCollage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockCollageBuilder { inner }
  }

  pub fn blocks(&self) -> &Vec<InputPageBlock> {
    &self.blocks
  }

  pub fn caption(&self) -> &PageBlockCaption {
    &self.caption
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockCollageBuilder {
  inner: InputPageBlockCollage,
}

impl RTDInputPageBlockCollageBuilder {
  pub fn build(&self) -> InputPageBlockCollage {
    self.inner.clone()
  }

  pub fn blocks(&mut self, blocks: Vec<InputPageBlock>) -> &mut Self {
    self.inner.blocks = blocks;
    self
  }

  pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockCollage> for InputPageBlockCollage {
  fn as_ref(&self) -> &InputPageBlockCollage {
    self
  }
}

impl AsRef<InputPageBlockCollage> for RTDInputPageBlockCollageBuilder {
  fn as_ref(&self) -> &InputPageBlockCollage {
    &self.inner
  }
}

/// A collapsible block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Always visible heading for the block
  header: RichText,
  /// Block contents
  blocks: Vec<InputPageBlock>,
  /// True, if the block is open by default
  is_open: bool,
}

impl RObject for InputPageBlockDetails {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockDetails"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockDetails {}

impl InputPageBlockDetails {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockDetailsBuilder {
    let inner = InputPageBlockDetails {
      td_name: "inputPageBlockDetails".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockDetailsBuilder { inner }
  }

  pub fn header(&self) -> &RichText {
    &self.header
  }

  pub fn blocks(&self) -> &Vec<InputPageBlock> {
    &self.blocks
  }

  pub fn is_open(&self) -> bool {
    self.is_open
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockDetailsBuilder {
  inner: InputPageBlockDetails,
}

impl RTDInputPageBlockDetailsBuilder {
  pub fn build(&self) -> InputPageBlockDetails {
    self.inner.clone()
  }

  pub fn header<T: AsRef<RichText>>(&mut self, header: T) -> &mut Self {
    self.inner.header = header.as_ref().clone();
    self
  }

  pub fn blocks(&mut self, blocks: Vec<InputPageBlock>) -> &mut Self {
    self.inner.blocks = blocks;
    self
  }

  pub fn is_open(&mut self, is_open: bool) -> &mut Self {
    self.inner.is_open = is_open;
    self
  }
}

impl AsRef<InputPageBlockDetails> for InputPageBlockDetails {
  fn as_ref(&self) -> &InputPageBlockDetails {
    self
  }
}

impl AsRef<InputPageBlockDetails> for RTDInputPageBlockDetailsBuilder {
  fn as_ref(&self) -> &InputPageBlockDetails {
    &self.inner
  }
}

/// An empty block separating the page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockDivider {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for InputPageBlockDivider {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockDivider"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockDivider {}

impl InputPageBlockDivider {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockDividerBuilder {
    let inner = InputPageBlockDivider {
      td_name: "inputPageBlockDivider".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDInputPageBlockDividerBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockDividerBuilder {
  inner: InputPageBlockDivider,
}

impl RTDInputPageBlockDividerBuilder {
  pub fn build(&self) -> InputPageBlockDivider {
    self.inner.clone()
  }
}

impl AsRef<InputPageBlockDivider> for InputPageBlockDivider {
  fn as_ref(&self) -> &InputPageBlockDivider {
    self
  }
}

impl AsRef<InputPageBlockDivider> for RTDInputPageBlockDividerBuilder {
  fn as_ref(&self) -> &InputPageBlockDivider {
    &self.inner
  }
}

/// A general file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The file to be sent
  document: InputDocument,
  /// File caption; pass null if none
  caption: PageBlockCaption,
}

impl RObject for InputPageBlockDocument {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockDocument"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockDocument {}

impl InputPageBlockDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockDocumentBuilder {
    let inner = InputPageBlockDocument {
      td_name: "inputPageBlockDocument".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockDocumentBuilder { inner }
  }

  pub fn document(&self) -> &InputDocument {
    &self.document
  }

  pub fn caption(&self) -> &PageBlockCaption {
    &self.caption
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockDocumentBuilder {
  inner: InputPageBlockDocument,
}

impl RTDInputPageBlockDocumentBuilder {
  pub fn build(&self) -> InputPageBlockDocument {
    self.inner.clone()
  }

  pub fn document<T: AsRef<InputDocument>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockDocument> for InputPageBlockDocument {
  fn as_ref(&self) -> &InputPageBlockDocument {
    self
  }
}

impl AsRef<InputPageBlockDocument> for RTDInputPageBlockDocumentBuilder {
  fn as_ref(&self) -> &InputPageBlockDocument {
    &self.inner
  }
}

/// An expandable block quote
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockExpandableBlockQuote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Quote text
  text: RichText,
  /// Quote credit; pass null if none
  credit: RichText,
}

impl RObject for InputPageBlockExpandableBlockQuote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockExpandableBlockQuote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockExpandableBlockQuote {}

impl InputPageBlockExpandableBlockQuote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockExpandableBlockQuoteBuilder {
    let inner = InputPageBlockExpandableBlockQuote {
      td_name: "inputPageBlockExpandableBlockQuote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockExpandableBlockQuoteBuilder { inner }
  }

  pub fn text(&self) -> &RichText {
    &self.text
  }

  pub fn credit(&self) -> &RichText {
    &self.credit
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockExpandableBlockQuoteBuilder {
  inner: InputPageBlockExpandableBlockQuote,
}

impl RTDInputPageBlockExpandableBlockQuoteBuilder {
  pub fn build(&self) -> InputPageBlockExpandableBlockQuote {
    self.inner.clone()
  }

  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn credit<T: AsRef<RichText>>(&mut self, credit: T) -> &mut Self {
    self.inner.credit = credit.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockExpandableBlockQuote> for InputPageBlockExpandableBlockQuote {
  fn as_ref(&self) -> &InputPageBlockExpandableBlockQuote {
    self
  }
}

impl AsRef<InputPageBlockExpandableBlockQuote> for RTDInputPageBlockExpandableBlockQuoteBuilder {
  fn as_ref(&self) -> &InputPageBlockExpandableBlockQuote {
    &self.inner
  }
}

/// The footer of the page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockFooter {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Footer
  footer: RichText,
}

impl RObject for InputPageBlockFooter {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockFooter"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockFooter {}

impl InputPageBlockFooter {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockFooterBuilder {
    let inner = InputPageBlockFooter {
      td_name: "inputPageBlockFooter".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockFooterBuilder { inner }
  }

  pub fn footer(&self) -> &RichText {
    &self.footer
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockFooterBuilder {
  inner: InputPageBlockFooter,
}

impl RTDInputPageBlockFooterBuilder {
  pub fn build(&self) -> InputPageBlockFooter {
    self.inner.clone()
  }

  pub fn footer<T: AsRef<RichText>>(&mut self, footer: T) -> &mut Self {
    self.inner.footer = footer.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockFooter> for InputPageBlockFooter {
  fn as_ref(&self) -> &InputPageBlockFooter {
    self
  }
}

impl AsRef<InputPageBlockFooter> for RTDInputPageBlockFooterBuilder {
  fn as_ref(&self) -> &InputPageBlockFooter {
    &self.inner
  }
}

/// A list of data blocks
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockList {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The items of the list
  items: Vec<InputPageBlockListItem>,
}

impl RObject for InputPageBlockList {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockList"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockList {}

impl InputPageBlockList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockListBuilder {
    let inner = InputPageBlockList {
      td_name: "inputPageBlockList".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockListBuilder { inner }
  }

  pub fn items(&self) -> &Vec<InputPageBlockListItem> {
    &self.items
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockListBuilder {
  inner: InputPageBlockList,
}

impl RTDInputPageBlockListBuilder {
  pub fn build(&self) -> InputPageBlockList {
    self.inner.clone()
  }

  pub fn items(&mut self, items: Vec<InputPageBlockListItem>) -> &mut Self {
    self.inner.items = items;
    self
  }
}

impl AsRef<InputPageBlockList> for InputPageBlockList {
  fn as_ref(&self) -> &InputPageBlockList {
    self
  }
}

impl AsRef<InputPageBlockList> for RTDInputPageBlockListBuilder {
  fn as_ref(&self) -> &InputPageBlockList {
    &self.inner
  }
}

/// A map. The map's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockMap {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Location of the map center
  location: Location,
  /// Map zoom level; 0-24
  zoom: i64,
  /// Map width; 0-10000
  width: i64,
  /// Map height; 0-10000
  height: i64,
  /// Block caption; pass null if none
  caption: PageBlockCaption,
}

impl RObject for InputPageBlockMap {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockMap"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockMap {}

impl InputPageBlockMap {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockMapBuilder {
    let inner = InputPageBlockMap {
      td_name: "inputPageBlockMap".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockMapBuilder { inner }
  }

  pub fn location(&self) -> &Location {
    &self.location
  }

  pub fn zoom(&self) -> i64 {
    self.zoom
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }

  pub fn caption(&self) -> &PageBlockCaption {
    &self.caption
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockMapBuilder {
  inner: InputPageBlockMap,
}

impl RTDInputPageBlockMapBuilder {
  pub fn build(&self) -> InputPageBlockMap {
    self.inner.clone()
  }

  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

  pub fn zoom(&mut self, zoom: i64) -> &mut Self {
    self.inner.zoom = zoom;
    self
  }

  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

  pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockMap> for InputPageBlockMap {
  fn as_ref(&self) -> &InputPageBlockMap {
    self
  }
}

impl AsRef<InputPageBlockMap> for RTDInputPageBlockMapBuilder {
  fn as_ref(&self) -> &InputPageBlockMap {
    &self.inner
  }
}

/// A mathematical expression
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockMathematicalExpression {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The expression in LaTeX format
  expression: String,
}

impl RObject for InputPageBlockMathematicalExpression {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockMathematicalExpression"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockMathematicalExpression {}

impl InputPageBlockMathematicalExpression {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockMathematicalExpressionBuilder {
    let inner = InputPageBlockMathematicalExpression {
      td_name: "inputPageBlockMathematicalExpression".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockMathematicalExpressionBuilder { inner }
  }

  pub fn expression(&self) -> &String {
    &self.expression
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockMathematicalExpressionBuilder {
  inner: InputPageBlockMathematicalExpression,
}

impl RTDInputPageBlockMathematicalExpressionBuilder {
  pub fn build(&self) -> InputPageBlockMathematicalExpression {
    self.inner.clone()
  }

  pub fn expression<T: AsRef<str>>(&mut self, expression: T) -> &mut Self {
    self.inner.expression = expression.as_ref().to_string();
    self
  }
}

impl AsRef<InputPageBlockMathematicalExpression> for InputPageBlockMathematicalExpression {
  fn as_ref(&self) -> &InputPageBlockMathematicalExpression {
    self
  }
}

impl AsRef<InputPageBlockMathematicalExpression>
  for RTDInputPageBlockMathematicalExpressionBuilder
{
  fn as_ref(&self) -> &InputPageBlockMathematicalExpression {
    &self.inner
  }
}

/// A text paragraph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockParagraph {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Paragraph text
  text: RichText,
}

impl RObject for InputPageBlockParagraph {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockParagraph"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockParagraph {}

impl InputPageBlockParagraph {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockParagraphBuilder {
    let inner = InputPageBlockParagraph {
      td_name: "inputPageBlockParagraph".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockParagraphBuilder { inner }
  }

  pub fn text(&self) -> &RichText {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockParagraphBuilder {
  inner: InputPageBlockParagraph,
}

impl RTDInputPageBlockParagraphBuilder {
  pub fn build(&self) -> InputPageBlockParagraph {
    self.inner.clone()
  }

  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockParagraph> for InputPageBlockParagraph {
  fn as_ref(&self) -> &InputPageBlockParagraph {
    self
  }
}

impl AsRef<InputPageBlockParagraph> for RTDInputPageBlockParagraphBuilder {
  fn as_ref(&self) -> &InputPageBlockParagraph {
    &self.inner
  }
}

/// A photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The photo to be sent
  photo: InputPhoto,
  /// Photo caption; pass null if none
  caption: PageBlockCaption,
  /// True, if the photo preview must be covered by a spoiler animation
  has_spoiler: bool,
}

impl RObject for InputPageBlockPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockPhoto {}

impl InputPageBlockPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockPhotoBuilder {
    let inner = InputPageBlockPhoto {
      td_name: "inputPageBlockPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &InputPhoto {
    &self.photo
  }

  pub fn caption(&self) -> &PageBlockCaption {
    &self.caption
  }

  pub fn has_spoiler(&self) -> bool {
    self.has_spoiler
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockPhotoBuilder {
  inner: InputPageBlockPhoto,
}

impl RTDInputPageBlockPhotoBuilder {
  pub fn build(&self) -> InputPageBlockPhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<InputPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn has_spoiler(&mut self, has_spoiler: bool) -> &mut Self {
    self.inner.has_spoiler = has_spoiler;
    self
  }
}

impl AsRef<InputPageBlockPhoto> for InputPageBlockPhoto {
  fn as_ref(&self) -> &InputPageBlockPhoto {
    self
  }
}

impl AsRef<InputPageBlockPhoto> for RTDInputPageBlockPhotoBuilder {
  fn as_ref(&self) -> &InputPageBlockPhoto {
    &self.inner
  }
}

/// A preformatted text paragraph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockPreformatted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Paragraph text
  text: RichText,
  /// Programming language for which the text needs to be formatted
  language: String,
}

impl RObject for InputPageBlockPreformatted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockPreformatted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockPreformatted {}

impl InputPageBlockPreformatted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockPreformattedBuilder {
    let inner = InputPageBlockPreformatted {
      td_name: "inputPageBlockPreformatted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockPreformattedBuilder { inner }
  }

  pub fn text(&self) -> &RichText {
    &self.text
  }

  pub fn language(&self) -> &String {
    &self.language
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockPreformattedBuilder {
  inner: InputPageBlockPreformatted,
}

impl RTDInputPageBlockPreformattedBuilder {
  pub fn build(&self) -> InputPageBlockPreformatted {
    self.inner.clone()
  }

  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn language<T: AsRef<str>>(&mut self, language: T) -> &mut Self {
    self.inner.language = language.as_ref().to_string();
    self
  }
}

impl AsRef<InputPageBlockPreformatted> for InputPageBlockPreformatted {
  fn as_ref(&self) -> &InputPageBlockPreformatted {
    self
  }
}

impl AsRef<InputPageBlockPreformatted> for RTDInputPageBlockPreformattedBuilder {
  fn as_ref(&self) -> &InputPageBlockPreformatted {
    &self.inner
  }
}

/// A pull quote
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockPullQuote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Quote text
  text: RichText,
  /// Quote credit; pass null if none
  credit: RichText,
}

impl RObject for InputPageBlockPullQuote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockPullQuote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockPullQuote {}

impl InputPageBlockPullQuote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockPullQuoteBuilder {
    let inner = InputPageBlockPullQuote {
      td_name: "inputPageBlockPullQuote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockPullQuoteBuilder { inner }
  }

  pub fn text(&self) -> &RichText {
    &self.text
  }

  pub fn credit(&self) -> &RichText {
    &self.credit
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockPullQuoteBuilder {
  inner: InputPageBlockPullQuote,
}

impl RTDInputPageBlockPullQuoteBuilder {
  pub fn build(&self) -> InputPageBlockPullQuote {
    self.inner.clone()
  }

  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn credit<T: AsRef<RichText>>(&mut self, credit: T) -> &mut Self {
    self.inner.credit = credit.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockPullQuote> for InputPageBlockPullQuote {
  fn as_ref(&self) -> &InputPageBlockPullQuote {
    self
  }
}

impl AsRef<InputPageBlockPullQuote> for RTDInputPageBlockPullQuoteBuilder {
  fn as_ref(&self) -> &InputPageBlockPullQuote {
    &self.inner
  }
}

/// A section heading
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockSectionHeading {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the section heading
  text: RichText,
  /// Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
  size: i64,
}

impl RObject for InputPageBlockSectionHeading {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockSectionHeading"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockSectionHeading {}

impl InputPageBlockSectionHeading {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockSectionHeadingBuilder {
    let inner = InputPageBlockSectionHeading {
      td_name: "inputPageBlockSectionHeading".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockSectionHeadingBuilder { inner }
  }

  pub fn text(&self) -> &RichText {
    &self.text
  }

  pub fn size(&self) -> i64 {
    self.size
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockSectionHeadingBuilder {
  inner: InputPageBlockSectionHeading,
}

impl RTDInputPageBlockSectionHeadingBuilder {
  pub fn build(&self) -> InputPageBlockSectionHeading {
    self.inner.clone()
  }

  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn size(&mut self, size: i64) -> &mut Self {
    self.inner.size = size;
    self
  }
}

impl AsRef<InputPageBlockSectionHeading> for InputPageBlockSectionHeading {
  fn as_ref(&self) -> &InputPageBlockSectionHeading {
    self
  }
}

impl AsRef<InputPageBlockSectionHeading> for RTDInputPageBlockSectionHeadingBuilder {
  fn as_ref(&self) -> &InputPageBlockSectionHeading {
    &self.inner
  }
}

/// A slideshow
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockSlideshow {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Slideshow item contents
  blocks: Vec<InputPageBlock>,
  /// Block caption; pass null if none
  caption: PageBlockCaption,
}

impl RObject for InputPageBlockSlideshow {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockSlideshow"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockSlideshow {}

impl InputPageBlockSlideshow {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockSlideshowBuilder {
    let inner = InputPageBlockSlideshow {
      td_name: "inputPageBlockSlideshow".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockSlideshowBuilder { inner }
  }

  pub fn blocks(&self) -> &Vec<InputPageBlock> {
    &self.blocks
  }

  pub fn caption(&self) -> &PageBlockCaption {
    &self.caption
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockSlideshowBuilder {
  inner: InputPageBlockSlideshow,
}

impl RTDInputPageBlockSlideshowBuilder {
  pub fn build(&self) -> InputPageBlockSlideshow {
    self.inner.clone()
  }

  pub fn blocks(&mut self, blocks: Vec<InputPageBlock>) -> &mut Self {
    self.inner.blocks = blocks;
    self
  }

  pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockSlideshow> for InputPageBlockSlideshow {
  fn as_ref(&self) -> &InputPageBlockSlideshow {
    self
  }
}

impl AsRef<InputPageBlockSlideshow> for RTDInputPageBlockSlideshowBuilder {
  fn as_ref(&self) -> &InputPageBlockSlideshow {
    &self.inner
  }
}

/// A table
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockTable {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Table caption
  caption: RichText,
  /// Table cells
  cells: Vec<Vec<PageBlockTableCell>>,
  /// Pass true if the table is bordered
  is_bordered: bool,
  /// Pass true if the table is striped
  is_striped: bool,
  /// Pass true if table cells must have smaller indents
  is_compact: bool,
}

impl RObject for InputPageBlockTable {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockTable"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockTable {}

impl InputPageBlockTable {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockTableBuilder {
    let inner = InputPageBlockTable {
      td_name: "inputPageBlockTable".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockTableBuilder { inner }
  }

  pub fn caption(&self) -> &RichText {
    &self.caption
  }

  pub fn cells(&self) -> &Vec<Vec<PageBlockTableCell>> {
    &self.cells
  }

  pub fn is_bordered(&self) -> bool {
    self.is_bordered
  }

  pub fn is_striped(&self) -> bool {
    self.is_striped
  }

  pub fn is_compact(&self) -> bool {
    self.is_compact
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockTableBuilder {
  inner: InputPageBlockTable,
}

impl RTDInputPageBlockTableBuilder {
  pub fn build(&self) -> InputPageBlockTable {
    self.inner.clone()
  }

  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn cells(&mut self, cells: Vec<Vec<PageBlockTableCell>>) -> &mut Self {
    self.inner.cells = cells;
    self
  }

  pub fn is_bordered(&mut self, is_bordered: bool) -> &mut Self {
    self.inner.is_bordered = is_bordered;
    self
  }

  pub fn is_striped(&mut self, is_striped: bool) -> &mut Self {
    self.inner.is_striped = is_striped;
    self
  }

  pub fn is_compact(&mut self, is_compact: bool) -> &mut Self {
    self.inner.is_compact = is_compact;
    self
  }
}

impl AsRef<InputPageBlockTable> for InputPageBlockTable {
  fn as_ref(&self) -> &InputPageBlockTable {
    self
  }
}

impl AsRef<InputPageBlockTable> for RTDInputPageBlockTableBuilder {
  fn as_ref(&self) -> &InputPageBlockTable {
    &self.inner
  }
}

/// A "Thinking..." placeholder; for pending rich messages only; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockThinking {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the placeholder
  text: RichText,
}

impl RObject for InputPageBlockThinking {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockThinking"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockThinking {}

impl InputPageBlockThinking {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockThinkingBuilder {
    let inner = InputPageBlockThinking {
      td_name: "inputPageBlockThinking".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockThinkingBuilder { inner }
  }

  pub fn text(&self) -> &RichText {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockThinkingBuilder {
  inner: InputPageBlockThinking,
}

impl RTDInputPageBlockThinkingBuilder {
  pub fn build(&self) -> InputPageBlockThinking {
    self.inner.clone()
  }

  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockThinking> for InputPageBlockThinking {
  fn as_ref(&self) -> &InputPageBlockThinking {
    self
  }
}

impl AsRef<InputPageBlockThinking> for RTDInputPageBlockThinkingBuilder {
  fn as_ref(&self) -> &InputPageBlockThinking {
    &self.inner
  }
}

/// A video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The video to be sent
  video: InputVideo,
  /// Video caption; pass null if none
  caption: PageBlockCaption,
  /// True, if the video preview must be covered by a spoiler animation
  has_spoiler: bool,
}

impl RObject for InputPageBlockVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockVideo {}

impl InputPageBlockVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockVideoBuilder {
    let inner = InputPageBlockVideo {
      td_name: "inputPageBlockVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockVideoBuilder { inner }
  }

  pub fn video(&self) -> &InputVideo {
    &self.video
  }

  pub fn caption(&self) -> &PageBlockCaption {
    &self.caption
  }

  pub fn has_spoiler(&self) -> bool {
    self.has_spoiler
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockVideoBuilder {
  inner: InputPageBlockVideo,
}

impl RTDInputPageBlockVideoBuilder {
  pub fn build(&self) -> InputPageBlockVideo {
    self.inner.clone()
  }

  pub fn video<T: AsRef<InputVideo>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

  pub fn has_spoiler(&mut self, has_spoiler: bool) -> &mut Self {
    self.inner.has_spoiler = has_spoiler;
    self
  }
}

impl AsRef<InputPageBlockVideo> for InputPageBlockVideo {
  fn as_ref(&self) -> &InputPageBlockVideo {
    self
  }
}

impl AsRef<InputPageBlockVideo> for RTDInputPageBlockVideoBuilder {
  fn as_ref(&self) -> &InputPageBlockVideo {
    &self.inner
  }
}

/// A voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The voice note to be sent
  voice_note: InputVoiceNote,
  /// Voice note caption; pass null if none
  caption: PageBlockCaption,
}

impl RObject for InputPageBlockVoiceNote {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockVoiceNote"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPageBlock for InputPageBlockVoiceNote {}

impl InputPageBlockVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockVoiceNoteBuilder {
    let inner = InputPageBlockVoiceNote {
      td_name: "inputPageBlockVoiceNote".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockVoiceNoteBuilder { inner }
  }

  pub fn voice_note(&self) -> &InputVoiceNote {
    &self.voice_note
  }

  pub fn caption(&self) -> &PageBlockCaption {
    &self.caption
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockVoiceNoteBuilder {
  inner: InputPageBlockVoiceNote,
}

impl RTDInputPageBlockVoiceNoteBuilder {
  pub fn build(&self) -> InputPageBlockVoiceNote {
    self.inner.clone()
  }

  pub fn voice_note<T: AsRef<InputVoiceNote>>(&mut self, voice_note: T) -> &mut Self {
    self.inner.voice_note = voice_note.as_ref().clone();
    self
  }

  pub fn caption<T: AsRef<PageBlockCaption>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }
}

impl AsRef<InputPageBlockVoiceNote> for InputPageBlockVoiceNote {
  fn as_ref(&self) -> &InputPageBlockVoiceNote {
    self
  }
}

impl AsRef<InputPageBlockVoiceNote> for RTDInputPageBlockVoiceNoteBuilder {
  fn as_ref(&self) -> &InputPageBlockVoiceNote {
    &self.inner
  }
}
