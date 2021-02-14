
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a block of an instant view web page
pub trait TDPageBlock: Debug + RObject {}

/// Describes a block of an instant view web page
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageBlock {
  #[doc(hidden)] _Default(()),
  /// An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor
  Anchor(PageBlockAnchor),
  /// An animation
  Animation(PageBlockAnimation),
  /// An audio file
  Audio(PageBlockAudio),
  /// The author and publishing date of a page
  AuthorDate(PageBlockAuthorDate),
  /// A block quote
  BlockQuote(PageBlockBlockQuote),
  /// A link to a chat
  ChatLink(PageBlockChatLink),
  /// A collage
  Collage(PageBlockCollage),
  /// A page cover
  Cover(PageBlockCover),
  /// An empty block separating a page
  Divider(PageBlockDivider),
  /// An embedded web page
  Embedded(PageBlockEmbedded),
  /// An embedded post
  EmbeddedPost(PageBlockEmbeddedPost),
  /// The footer of a page
  Footer(PageBlockFooter),
  /// A header
  Header(PageBlockHeader),
  /// A list of texts
  List(PageBlockList),
  /// A text paragraph
  Paragraph(PageBlockParagraph),
  /// A photo
  Photo(PageBlockPhoto),
  /// A preformatted text paragraph
  Preformatted(PageBlockPreformatted),
  /// A pull quote
  PullQuote(PageBlockPullQuote),
  /// A slideshow
  Slideshow(PageBlockSlideshow),
  /// A subheader
  Subheader(PageBlockSubheader),
  /// The subtitle of a page
  Subtitle(PageBlockSubtitle),
  /// The title of a page
  Title(PageBlockTitle),
  /// A video
  Video(PageBlockVideo),

}

impl Default for PageBlock {
  fn default() -> Self { PageBlock::_Default(()) }
}

impl<'de> Deserialize<'de> for PageBlock {
  fn deserialize<D>(deserializer: D) -> Result<PageBlock, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PageBlock,
      (pageBlockAnchor, Anchor);
      (pageBlockAnimation, Animation);
      (pageBlockAudio, Audio);
      (pageBlockAuthorDate, AuthorDate);
      (pageBlockBlockQuote, BlockQuote);
      (pageBlockChatLink, ChatLink);
      (pageBlockCollage, Collage);
      (pageBlockCover, Cover);
      (pageBlockDivider, Divider);
      (pageBlockEmbedded, Embedded);
      (pageBlockEmbeddedPost, EmbeddedPost);
      (pageBlockFooter, Footer);
      (pageBlockHeader, Header);
      (pageBlockList, List);
      (pageBlockParagraph, Paragraph);
      (pageBlockPhoto, Photo);
      (pageBlockPreformatted, Preformatted);
      (pageBlockPullQuote, PullQuote);
      (pageBlockSlideshow, Slideshow);
      (pageBlockSubheader, Subheader);
      (pageBlockSubtitle, Subtitle);
      (pageBlockTitle, Title);
      (pageBlockVideo, Video);

    )(deserializer)
  }
}

impl RObject for PageBlock {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      PageBlock::Anchor(t) => t.td_name(),
      PageBlock::Animation(t) => t.td_name(),
      PageBlock::Audio(t) => t.td_name(),
      PageBlock::AuthorDate(t) => t.td_name(),
      PageBlock::BlockQuote(t) => t.td_name(),
      PageBlock::ChatLink(t) => t.td_name(),
      PageBlock::Collage(t) => t.td_name(),
      PageBlock::Cover(t) => t.td_name(),
      PageBlock::Divider(t) => t.td_name(),
      PageBlock::Embedded(t) => t.td_name(),
      PageBlock::EmbeddedPost(t) => t.td_name(),
      PageBlock::Footer(t) => t.td_name(),
      PageBlock::Header(t) => t.td_name(),
      PageBlock::List(t) => t.td_name(),
      PageBlock::Paragraph(t) => t.td_name(),
      PageBlock::Photo(t) => t.td_name(),
      PageBlock::Preformatted(t) => t.td_name(),
      PageBlock::PullQuote(t) => t.td_name(),
      PageBlock::Slideshow(t) => t.td_name(),
      PageBlock::Subheader(t) => t.td_name(),
      PageBlock::Subtitle(t) => t.td_name(),
      PageBlock::Title(t) => t.td_name(),
      PageBlock::Video(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      PageBlock::Anchor(t) => t.extra(),
      PageBlock::Animation(t) => t.extra(),
      PageBlock::Audio(t) => t.extra(),
      PageBlock::AuthorDate(t) => t.extra(),
      PageBlock::BlockQuote(t) => t.extra(),
      PageBlock::ChatLink(t) => t.extra(),
      PageBlock::Collage(t) => t.extra(),
      PageBlock::Cover(t) => t.extra(),
      PageBlock::Divider(t) => t.extra(),
      PageBlock::Embedded(t) => t.extra(),
      PageBlock::EmbeddedPost(t) => t.extra(),
      PageBlock::Footer(t) => t.extra(),
      PageBlock::Header(t) => t.extra(),
      PageBlock::List(t) => t.extra(),
      PageBlock::Paragraph(t) => t.extra(),
      PageBlock::Photo(t) => t.extra(),
      PageBlock::Preformatted(t) => t.extra(),
      PageBlock::PullQuote(t) => t.extra(),
      PageBlock::Slideshow(t) => t.extra(),
      PageBlock::Subheader(t) => t.extra(),
      PageBlock::Subtitle(t) => t.extra(),
      PageBlock::Title(t) => t.extra(),
      PageBlock::Video(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl PageBlock {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let PageBlock::_Default(_) = self { true } else { false } }

  pub fn is_anchor(&self) -> bool { if let PageBlock::Anchor(_) = self { true } else { false } }
  pub fn is_animation(&self) -> bool { if let PageBlock::Animation(_) = self { true } else { false } }
  pub fn is_audio(&self) -> bool { if let PageBlock::Audio(_) = self { true } else { false } }
  pub fn is_author_date(&self) -> bool { if let PageBlock::AuthorDate(_) = self { true } else { false } }
  pub fn is_block_quote(&self) -> bool { if let PageBlock::BlockQuote(_) = self { true } else { false } }
  pub fn is_chat_link(&self) -> bool { if let PageBlock::ChatLink(_) = self { true } else { false } }
  pub fn is_collage(&self) -> bool { if let PageBlock::Collage(_) = self { true } else { false } }
  pub fn is_cover(&self) -> bool { if let PageBlock::Cover(_) = self { true } else { false } }
  pub fn is_divider(&self) -> bool { if let PageBlock::Divider(_) = self { true } else { false } }
  pub fn is_embedded(&self) -> bool { if let PageBlock::Embedded(_) = self { true } else { false } }
  pub fn is_embedded_post(&self) -> bool { if let PageBlock::EmbeddedPost(_) = self { true } else { false } }
  pub fn is_footer(&self) -> bool { if let PageBlock::Footer(_) = self { true } else { false } }
  pub fn is_header(&self) -> bool { if let PageBlock::Header(_) = self { true } else { false } }
  pub fn is_list(&self) -> bool { if let PageBlock::List(_) = self { true } else { false } }
  pub fn is_paragraph(&self) -> bool { if let PageBlock::Paragraph(_) = self { true } else { false } }
  pub fn is_photo(&self) -> bool { if let PageBlock::Photo(_) = self { true } else { false } }
  pub fn is_preformatted(&self) -> bool { if let PageBlock::Preformatted(_) = self { true } else { false } }
  pub fn is_pull_quote(&self) -> bool { if let PageBlock::PullQuote(_) = self { true } else { false } }
  pub fn is_slideshow(&self) -> bool { if let PageBlock::Slideshow(_) = self { true } else { false } }
  pub fn is_subheader(&self) -> bool { if let PageBlock::Subheader(_) = self { true } else { false } }
  pub fn is_subtitle(&self) -> bool { if let PageBlock::Subtitle(_) = self { true } else { false } }
  pub fn is_title(&self) -> bool { if let PageBlock::Title(_) = self { true } else { false } }
  pub fn is_video(&self) -> bool { if let PageBlock::Video(_) = self { true } else { false } }

  pub fn on_anchor<F: FnOnce(&PageBlockAnchor)>(&self, fnc: F) -> &Self { if let PageBlock::Anchor(t) = self { fnc(t) }; self }
  pub fn on_animation<F: FnOnce(&PageBlockAnimation)>(&self, fnc: F) -> &Self { if let PageBlock::Animation(t) = self { fnc(t) }; self }
  pub fn on_audio<F: FnOnce(&PageBlockAudio)>(&self, fnc: F) -> &Self { if let PageBlock::Audio(t) = self { fnc(t) }; self }
  pub fn on_author_date<F: FnOnce(&PageBlockAuthorDate)>(&self, fnc: F) -> &Self { if let PageBlock::AuthorDate(t) = self { fnc(t) }; self }
  pub fn on_block_quote<F: FnOnce(&PageBlockBlockQuote)>(&self, fnc: F) -> &Self { if let PageBlock::BlockQuote(t) = self { fnc(t) }; self }
  pub fn on_chat_link<F: FnOnce(&PageBlockChatLink)>(&self, fnc: F) -> &Self { if let PageBlock::ChatLink(t) = self { fnc(t) }; self }
  pub fn on_collage<F: FnOnce(&PageBlockCollage)>(&self, fnc: F) -> &Self { if let PageBlock::Collage(t) = self { fnc(t) }; self }
  pub fn on_cover<F: FnOnce(&PageBlockCover)>(&self, fnc: F) -> &Self { if let PageBlock::Cover(t) = self { fnc(t) }; self }
  pub fn on_divider<F: FnOnce(&PageBlockDivider)>(&self, fnc: F) -> &Self { if let PageBlock::Divider(t) = self { fnc(t) }; self }
  pub fn on_embedded<F: FnOnce(&PageBlockEmbedded)>(&self, fnc: F) -> &Self { if let PageBlock::Embedded(t) = self { fnc(t) }; self }
  pub fn on_embedded_post<F: FnOnce(&PageBlockEmbeddedPost)>(&self, fnc: F) -> &Self { if let PageBlock::EmbeddedPost(t) = self { fnc(t) }; self }
  pub fn on_footer<F: FnOnce(&PageBlockFooter)>(&self, fnc: F) -> &Self { if let PageBlock::Footer(t) = self { fnc(t) }; self }
  pub fn on_header<F: FnOnce(&PageBlockHeader)>(&self, fnc: F) -> &Self { if let PageBlock::Header(t) = self { fnc(t) }; self }
  pub fn on_list<F: FnOnce(&PageBlockList)>(&self, fnc: F) -> &Self { if let PageBlock::List(t) = self { fnc(t) }; self }
  pub fn on_paragraph<F: FnOnce(&PageBlockParagraph)>(&self, fnc: F) -> &Self { if let PageBlock::Paragraph(t) = self { fnc(t) }; self }
  pub fn on_photo<F: FnOnce(&PageBlockPhoto)>(&self, fnc: F) -> &Self { if let PageBlock::Photo(t) = self { fnc(t) }; self }
  pub fn on_preformatted<F: FnOnce(&PageBlockPreformatted)>(&self, fnc: F) -> &Self { if let PageBlock::Preformatted(t) = self { fnc(t) }; self }
  pub fn on_pull_quote<F: FnOnce(&PageBlockPullQuote)>(&self, fnc: F) -> &Self { if let PageBlock::PullQuote(t) = self { fnc(t) }; self }
  pub fn on_slideshow<F: FnOnce(&PageBlockSlideshow)>(&self, fnc: F) -> &Self { if let PageBlock::Slideshow(t) = self { fnc(t) }; self }
  pub fn on_subheader<F: FnOnce(&PageBlockSubheader)>(&self, fnc: F) -> &Self { if let PageBlock::Subheader(t) = self { fnc(t) }; self }
  pub fn on_subtitle<F: FnOnce(&PageBlockSubtitle)>(&self, fnc: F) -> &Self { if let PageBlock::Subtitle(t) = self { fnc(t) }; self }
  pub fn on_title<F: FnOnce(&PageBlockTitle)>(&self, fnc: F) -> &Self { if let PageBlock::Title(t) = self { fnc(t) }; self }
  pub fn on_video<F: FnOnce(&PageBlockVideo)>(&self, fnc: F) -> &Self { if let PageBlock::Video(t) = self { fnc(t) }; self }

  pub fn as_anchor(&self) -> Option<&PageBlockAnchor> { if let PageBlock::Anchor(t) = self { return Some(t) } None }
  pub fn as_animation(&self) -> Option<&PageBlockAnimation> { if let PageBlock::Animation(t) = self { return Some(t) } None }
  pub fn as_audio(&self) -> Option<&PageBlockAudio> { if let PageBlock::Audio(t) = self { return Some(t) } None }
  pub fn as_author_date(&self) -> Option<&PageBlockAuthorDate> { if let PageBlock::AuthorDate(t) = self { return Some(t) } None }
  pub fn as_block_quote(&self) -> Option<&PageBlockBlockQuote> { if let PageBlock::BlockQuote(t) = self { return Some(t) } None }
  pub fn as_chat_link(&self) -> Option<&PageBlockChatLink> { if let PageBlock::ChatLink(t) = self { return Some(t) } None }
  pub fn as_collage(&self) -> Option<&PageBlockCollage> { if let PageBlock::Collage(t) = self { return Some(t) } None }
  pub fn as_cover(&self) -> Option<&PageBlockCover> { if let PageBlock::Cover(t) = self { return Some(t) } None }
  pub fn as_divider(&self) -> Option<&PageBlockDivider> { if let PageBlock::Divider(t) = self { return Some(t) } None }
  pub fn as_embedded(&self) -> Option<&PageBlockEmbedded> { if let PageBlock::Embedded(t) = self { return Some(t) } None }
  pub fn as_embedded_post(&self) -> Option<&PageBlockEmbeddedPost> { if let PageBlock::EmbeddedPost(t) = self { return Some(t) } None }
  pub fn as_footer(&self) -> Option<&PageBlockFooter> { if let PageBlock::Footer(t) = self { return Some(t) } None }
  pub fn as_header(&self) -> Option<&PageBlockHeader> { if let PageBlock::Header(t) = self { return Some(t) } None }
  pub fn as_list(&self) -> Option<&PageBlockList> { if let PageBlock::List(t) = self { return Some(t) } None }
  pub fn as_paragraph(&self) -> Option<&PageBlockParagraph> { if let PageBlock::Paragraph(t) = self { return Some(t) } None }
  pub fn as_photo(&self) -> Option<&PageBlockPhoto> { if let PageBlock::Photo(t) = self { return Some(t) } None }
  pub fn as_preformatted(&self) -> Option<&PageBlockPreformatted> { if let PageBlock::Preformatted(t) = self { return Some(t) } None }
  pub fn as_pull_quote(&self) -> Option<&PageBlockPullQuote> { if let PageBlock::PullQuote(t) = self { return Some(t) } None }
  pub fn as_slideshow(&self) -> Option<&PageBlockSlideshow> { if let PageBlock::Slideshow(t) = self { return Some(t) } None }
  pub fn as_subheader(&self) -> Option<&PageBlockSubheader> { if let PageBlock::Subheader(t) = self { return Some(t) } None }
  pub fn as_subtitle(&self) -> Option<&PageBlockSubtitle> { if let PageBlock::Subtitle(t) = self { return Some(t) } None }
  pub fn as_title(&self) -> Option<&PageBlockTitle> { if let PageBlock::Title(t) = self { return Some(t) } None }
  pub fn as_video(&self) -> Option<&PageBlockVideo> { if let PageBlock::Video(t) = self { return Some(t) } None }



  pub fn anchor<T: AsRef<PageBlockAnchor>>(t: T) -> Self { PageBlock::Anchor(t.as_ref().clone()) }

  pub fn animation<T: AsRef<PageBlockAnimation>>(t: T) -> Self { PageBlock::Animation(t.as_ref().clone()) }

  pub fn audio<T: AsRef<PageBlockAudio>>(t: T) -> Self { PageBlock::Audio(t.as_ref().clone()) }

  pub fn author_date<T: AsRef<PageBlockAuthorDate>>(t: T) -> Self { PageBlock::AuthorDate(t.as_ref().clone()) }

  pub fn block_quote<T: AsRef<PageBlockBlockQuote>>(t: T) -> Self { PageBlock::BlockQuote(t.as_ref().clone()) }

  pub fn chat_link<T: AsRef<PageBlockChatLink>>(t: T) -> Self { PageBlock::ChatLink(t.as_ref().clone()) }

  pub fn collage<T: AsRef<PageBlockCollage>>(t: T) -> Self { PageBlock::Collage(t.as_ref().clone()) }

  pub fn cover<T: AsRef<PageBlockCover>>(t: T) -> Self { PageBlock::Cover(t.as_ref().clone()) }

  pub fn divider<T: AsRef<PageBlockDivider>>(t: T) -> Self { PageBlock::Divider(t.as_ref().clone()) }

  pub fn embedded<T: AsRef<PageBlockEmbedded>>(t: T) -> Self { PageBlock::Embedded(t.as_ref().clone()) }

  pub fn embedded_post<T: AsRef<PageBlockEmbeddedPost>>(t: T) -> Self { PageBlock::EmbeddedPost(t.as_ref().clone()) }

  pub fn footer<T: AsRef<PageBlockFooter>>(t: T) -> Self { PageBlock::Footer(t.as_ref().clone()) }

  pub fn header<T: AsRef<PageBlockHeader>>(t: T) -> Self { PageBlock::Header(t.as_ref().clone()) }

  pub fn list<T: AsRef<PageBlockList>>(t: T) -> Self { PageBlock::List(t.as_ref().clone()) }

  pub fn paragraph<T: AsRef<PageBlockParagraph>>(t: T) -> Self { PageBlock::Paragraph(t.as_ref().clone()) }

  pub fn photo<T: AsRef<PageBlockPhoto>>(t: T) -> Self { PageBlock::Photo(t.as_ref().clone()) }

  pub fn preformatted<T: AsRef<PageBlockPreformatted>>(t: T) -> Self { PageBlock::Preformatted(t.as_ref().clone()) }

  pub fn pull_quote<T: AsRef<PageBlockPullQuote>>(t: T) -> Self { PageBlock::PullQuote(t.as_ref().clone()) }

  pub fn slideshow<T: AsRef<PageBlockSlideshow>>(t: T) -> Self { PageBlock::Slideshow(t.as_ref().clone()) }

  pub fn subheader<T: AsRef<PageBlockSubheader>>(t: T) -> Self { PageBlock::Subheader(t.as_ref().clone()) }

  pub fn subtitle<T: AsRef<PageBlockSubtitle>>(t: T) -> Self { PageBlock::Subtitle(t.as_ref().clone()) }

  pub fn title<T: AsRef<PageBlockTitle>>(t: T) -> Self { PageBlock::Title(t.as_ref().clone()) }

  pub fn video<T: AsRef<PageBlockVideo>>(t: T) -> Self { PageBlock::Video(t.as_ref().clone()) }

}

impl AsRef<PageBlock> for PageBlock {
  fn as_ref(&self) -> &PageBlock { self }
}







/// An invisible anchor on a page, which can be used in a URL to open the page from the specified anchor
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockAnchor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the anchor
  name: String,
  
}

impl RObject for PageBlockAnchor {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockAnchor" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockAnchor {}



impl PageBlockAnchor {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockAnchorBuilder {
    let mut inner = PageBlockAnchor::default();
    inner.td_name = "pageBlockAnchor".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockAnchorBuilder { inner }
  }

  pub fn name(&self) -> &String { &self.name }

}

#[doc(hidden)]
pub struct RTDPageBlockAnchorBuilder {
  inner: PageBlockAnchor
}

impl RTDPageBlockAnchorBuilder {
  pub fn build(&self) -> PageBlockAnchor { self.inner.clone() }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

}

impl AsRef<PageBlockAnchor> for PageBlockAnchor {
  fn as_ref(&self) -> &PageBlockAnchor { self }
}

impl AsRef<PageBlockAnchor> for RTDPageBlockAnchorBuilder {
  fn as_ref(&self) -> &PageBlockAnchor { &self.inner }
}







/// An animation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Animation file; may be null
  animation: Option<Animation>,
  /// Animation caption
  caption: RichText,
  /// True, if the animation should be played automatically
  need_autoplay: bool,
  
}

impl RObject for PageBlockAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockAnimation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockAnimation {}



impl PageBlockAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockAnimationBuilder {
    let mut inner = PageBlockAnimation::default();
    inner.td_name = "pageBlockAnimation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &Option<Animation> { &self.animation }

  pub fn caption(&self) -> &RichText { &self.caption }

  pub fn need_autoplay(&self) -> bool { self.need_autoplay }

}

#[doc(hidden)]
pub struct RTDPageBlockAnimationBuilder {
  inner: PageBlockAnimation
}

impl RTDPageBlockAnimationBuilder {
  pub fn build(&self) -> PageBlockAnimation { self.inner.clone() }

   
  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = Some(animation.as_ref().clone());
    self
  }

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

   
  pub fn need_autoplay(&mut self, need_autoplay: bool) -> &mut Self {
    self.inner.need_autoplay = need_autoplay;
    self
  }

}

impl AsRef<PageBlockAnimation> for PageBlockAnimation {
  fn as_ref(&self) -> &PageBlockAnimation { self }
}

impl AsRef<PageBlockAnimation> for RTDPageBlockAnimationBuilder {
  fn as_ref(&self) -> &PageBlockAnimation { &self.inner }
}







/// An audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Audio file; may be null
  audio: Option<Audio>,
  /// Audio file caption
  caption: RichText,
  
}

impl RObject for PageBlockAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockAudio" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockAudio {}



impl PageBlockAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockAudioBuilder {
    let mut inner = PageBlockAudio::default();
    inner.td_name = "pageBlockAudio".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockAudioBuilder { inner }
  }

  pub fn audio(&self) -> &Option<Audio> { &self.audio }

  pub fn caption(&self) -> &RichText { &self.caption }

}

#[doc(hidden)]
pub struct RTDPageBlockAudioBuilder {
  inner: PageBlockAudio
}

impl RTDPageBlockAudioBuilder {
  pub fn build(&self) -> PageBlockAudio { self.inner.clone() }

   
  pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = Some(audio.as_ref().clone());
    self
  }

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockAudio> for PageBlockAudio {
  fn as_ref(&self) -> &PageBlockAudio { self }
}

impl AsRef<PageBlockAudio> for RTDPageBlockAudioBuilder {
  fn as_ref(&self) -> &PageBlockAudio { &self.inner }
}







/// The author and publishing date of a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockAuthorDate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Author
  author: RichText,
  /// Point in time (Unix timestamp) when the article was published; 0 if unknown
  publish_date: i64,
  
}

impl RObject for PageBlockAuthorDate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockAuthorDate" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockAuthorDate {}



impl PageBlockAuthorDate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockAuthorDateBuilder {
    let mut inner = PageBlockAuthorDate::default();
    inner.td_name = "pageBlockAuthorDate".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockAuthorDateBuilder { inner }
  }

  pub fn author(&self) -> &RichText { &self.author }

  pub fn publish_date(&self) -> i64 { self.publish_date }

}

#[doc(hidden)]
pub struct RTDPageBlockAuthorDateBuilder {
  inner: PageBlockAuthorDate
}

impl RTDPageBlockAuthorDateBuilder {
  pub fn build(&self) -> PageBlockAuthorDate { self.inner.clone() }

   
  pub fn author<T: AsRef<RichText>>(&mut self, author: T) -> &mut Self {
    self.inner.author = author.as_ref().clone();
    self
  }

   
  pub fn publish_date(&mut self, publish_date: i64) -> &mut Self {
    self.inner.publish_date = publish_date;
    self
  }

}

impl AsRef<PageBlockAuthorDate> for PageBlockAuthorDate {
  fn as_ref(&self) -> &PageBlockAuthorDate { self }
}

impl AsRef<PageBlockAuthorDate> for RTDPageBlockAuthorDateBuilder {
  fn as_ref(&self) -> &PageBlockAuthorDate { &self.inner }
}







/// A block quote
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockBlockQuote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Quote text
  text: RichText,
  /// Quote caption
  caption: RichText,
  
}

impl RObject for PageBlockBlockQuote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockBlockQuote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockBlockQuote {}



impl PageBlockBlockQuote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockBlockQuoteBuilder {
    let mut inner = PageBlockBlockQuote::default();
    inner.td_name = "pageBlockBlockQuote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockBlockQuoteBuilder { inner }
  }

  pub fn text(&self) -> &RichText { &self.text }

  pub fn caption(&self) -> &RichText { &self.caption }

}

#[doc(hidden)]
pub struct RTDPageBlockBlockQuoteBuilder {
  inner: PageBlockBlockQuote
}

impl RTDPageBlockBlockQuoteBuilder {
  pub fn build(&self) -> PageBlockBlockQuote { self.inner.clone() }

   
  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockBlockQuote> for PageBlockBlockQuote {
  fn as_ref(&self) -> &PageBlockBlockQuote { self }
}

impl AsRef<PageBlockBlockQuote> for RTDPageBlockBlockQuoteBuilder {
  fn as_ref(&self) -> &PageBlockBlockQuote { &self.inner }
}







/// A link to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockChatLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat title
  title: String,
  /// Chat photo; may be null
  photo: Option<ChatPhoto>,
  /// Chat username, by which all other information about the chat should be resolved
  username: String,
  
}

impl RObject for PageBlockChatLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockChatLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockChatLink {}



impl PageBlockChatLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockChatLinkBuilder {
    let mut inner = PageBlockChatLink::default();
    inner.td_name = "pageBlockChatLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockChatLinkBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

  pub fn photo(&self) -> &Option<ChatPhoto> { &self.photo }

  pub fn username(&self) -> &String { &self.username }

}

#[doc(hidden)]
pub struct RTDPageBlockChatLinkBuilder {
  inner: PageBlockChatLink
}

impl RTDPageBlockChatLinkBuilder {
  pub fn build(&self) -> PageBlockChatLink { self.inner.clone() }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

   
  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

}

impl AsRef<PageBlockChatLink> for PageBlockChatLink {
  fn as_ref(&self) -> &PageBlockChatLink { self }
}

impl AsRef<PageBlockChatLink> for RTDPageBlockChatLinkBuilder {
  fn as_ref(&self) -> &PageBlockChatLink { &self.inner }
}







/// A collage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockCollage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Collage item contents
  page_blocks: Vec<PageBlock>,
  /// Block caption
  caption: RichText,
  
}

impl RObject for PageBlockCollage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockCollage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockCollage {}



impl PageBlockCollage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockCollageBuilder {
    let mut inner = PageBlockCollage::default();
    inner.td_name = "pageBlockCollage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockCollageBuilder { inner }
  }

  pub fn page_blocks(&self) -> &Vec<PageBlock> { &self.page_blocks }

  pub fn caption(&self) -> &RichText { &self.caption }

}

#[doc(hidden)]
pub struct RTDPageBlockCollageBuilder {
  inner: PageBlockCollage
}

impl RTDPageBlockCollageBuilder {
  pub fn build(&self) -> PageBlockCollage { self.inner.clone() }

   
  pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
    self.inner.page_blocks = page_blocks;
    self
  }

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockCollage> for PageBlockCollage {
  fn as_ref(&self) -> &PageBlockCollage { self }
}

impl AsRef<PageBlockCollage> for RTDPageBlockCollageBuilder {
  fn as_ref(&self) -> &PageBlockCollage { &self.inner }
}







/// A page cover
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockCover {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Cover
  cover: Box<PageBlock>,
  
}

impl RObject for PageBlockCover {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockCover" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockCover {}



impl PageBlockCover {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockCoverBuilder {
    let mut inner = PageBlockCover::default();
    inner.td_name = "pageBlockCover".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockCoverBuilder { inner }
  }

  pub fn cover(&self) -> &Box<PageBlock> { &self.cover }

}

#[doc(hidden)]
pub struct RTDPageBlockCoverBuilder {
  inner: PageBlockCover
}

impl RTDPageBlockCoverBuilder {
  pub fn build(&self) -> PageBlockCover { self.inner.clone() }

   
  pub fn cover<T: AsRef<Box<PageBlock>>>(&mut self, cover: T) -> &mut Self {
    self.inner.cover = cover.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockCover> for PageBlockCover {
  fn as_ref(&self) -> &PageBlockCover { self }
}

impl AsRef<PageBlockCover> for RTDPageBlockCoverBuilder {
  fn as_ref(&self) -> &PageBlockCover { &self.inner }
}







/// An empty block separating a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockDivider {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for PageBlockDivider {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockDivider" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockDivider {}



impl PageBlockDivider {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockDividerBuilder {
    let mut inner = PageBlockDivider::default();
    inner.td_name = "pageBlockDivider".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockDividerBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPageBlockDividerBuilder {
  inner: PageBlockDivider
}

impl RTDPageBlockDividerBuilder {
  pub fn build(&self) -> PageBlockDivider { self.inner.clone() }

}

impl AsRef<PageBlockDivider> for PageBlockDivider {
  fn as_ref(&self) -> &PageBlockDivider { self }
}

impl AsRef<PageBlockDivider> for RTDPageBlockDividerBuilder {
  fn as_ref(&self) -> &PageBlockDivider { &self.inner }
}







/// An embedded web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockEmbedded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Web page URL, if available
  url: String,
  /// HTML-markup of the embedded page
  html: String,
  /// Poster photo, if available; may be null
  poster_photo: Option<Photo>,
  /// Block width
  width: i64,
  /// Block height
  height: i64,
  /// Block caption
  caption: RichText,
  /// True, if the block should be full width
  is_full_width: bool,
  /// True, if scrolling should be allowed
  allow_scrolling: bool,
  
}

impl RObject for PageBlockEmbedded {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockEmbedded" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockEmbedded {}



impl PageBlockEmbedded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockEmbeddedBuilder {
    let mut inner = PageBlockEmbedded::default();
    inner.td_name = "pageBlockEmbedded".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockEmbeddedBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

  pub fn html(&self) -> &String { &self.html }

  pub fn poster_photo(&self) -> &Option<Photo> { &self.poster_photo }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn caption(&self) -> &RichText { &self.caption }

  pub fn is_full_width(&self) -> bool { self.is_full_width }

  pub fn allow_scrolling(&self) -> bool { self.allow_scrolling }

}

#[doc(hidden)]
pub struct RTDPageBlockEmbeddedBuilder {
  inner: PageBlockEmbedded
}

impl RTDPageBlockEmbeddedBuilder {
  pub fn build(&self) -> PageBlockEmbedded { self.inner.clone() }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn html<T: AsRef<str>>(&mut self, html: T) -> &mut Self {
    self.inner.html = html.as_ref().to_string();
    self
  }

   
  pub fn poster_photo<T: AsRef<Photo>>(&mut self, poster_photo: T) -> &mut Self {
    self.inner.poster_photo = Some(poster_photo.as_ref().clone());
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

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

   
  pub fn is_full_width(&mut self, is_full_width: bool) -> &mut Self {
    self.inner.is_full_width = is_full_width;
    self
  }

   
  pub fn allow_scrolling(&mut self, allow_scrolling: bool) -> &mut Self {
    self.inner.allow_scrolling = allow_scrolling;
    self
  }

}

impl AsRef<PageBlockEmbedded> for PageBlockEmbedded {
  fn as_ref(&self) -> &PageBlockEmbedded { self }
}

impl AsRef<PageBlockEmbedded> for RTDPageBlockEmbeddedBuilder {
  fn as_ref(&self) -> &PageBlockEmbedded { &self.inner }
}







/// An embedded post
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockEmbeddedPost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Web page URL
  url: String,
  /// Post author
  author: String,
  /// Post author photo
  author_photo: Photo,
  /// Point in time (Unix timestamp) when the post was created; 0 if unknown
  date: i64,
  /// Post content
  page_blocks: Vec<PageBlock>,
  /// Post caption
  caption: RichText,
  
}

impl RObject for PageBlockEmbeddedPost {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockEmbeddedPost" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockEmbeddedPost {}



impl PageBlockEmbeddedPost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockEmbeddedPostBuilder {
    let mut inner = PageBlockEmbeddedPost::default();
    inner.td_name = "pageBlockEmbeddedPost".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockEmbeddedPostBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

  pub fn author(&self) -> &String { &self.author }

  pub fn author_photo(&self) -> &Photo { &self.author_photo }

  pub fn date(&self) -> i64 { self.date }

  pub fn page_blocks(&self) -> &Vec<PageBlock> { &self.page_blocks }

  pub fn caption(&self) -> &RichText { &self.caption }

}

#[doc(hidden)]
pub struct RTDPageBlockEmbeddedPostBuilder {
  inner: PageBlockEmbeddedPost
}

impl RTDPageBlockEmbeddedPostBuilder {
  pub fn build(&self) -> PageBlockEmbeddedPost { self.inner.clone() }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn author<T: AsRef<str>>(&mut self, author: T) -> &mut Self {
    self.inner.author = author.as_ref().to_string();
    self
  }

   
  pub fn author_photo<T: AsRef<Photo>>(&mut self, author_photo: T) -> &mut Self {
    self.inner.author_photo = author_photo.as_ref().clone();
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
    self.inner.page_blocks = page_blocks;
    self
  }

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockEmbeddedPost> for PageBlockEmbeddedPost {
  fn as_ref(&self) -> &PageBlockEmbeddedPost { self }
}

impl AsRef<PageBlockEmbeddedPost> for RTDPageBlockEmbeddedPostBuilder {
  fn as_ref(&self) -> &PageBlockEmbeddedPost { &self.inner }
}







/// The footer of a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockFooter {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Footer
  footer: RichText,
  
}

impl RObject for PageBlockFooter {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockFooter" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockFooter {}



impl PageBlockFooter {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockFooterBuilder {
    let mut inner = PageBlockFooter::default();
    inner.td_name = "pageBlockFooter".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockFooterBuilder { inner }
  }

  pub fn footer(&self) -> &RichText { &self.footer }

}

#[doc(hidden)]
pub struct RTDPageBlockFooterBuilder {
  inner: PageBlockFooter
}

impl RTDPageBlockFooterBuilder {
  pub fn build(&self) -> PageBlockFooter { self.inner.clone() }

   
  pub fn footer<T: AsRef<RichText>>(&mut self, footer: T) -> &mut Self {
    self.inner.footer = footer.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockFooter> for PageBlockFooter {
  fn as_ref(&self) -> &PageBlockFooter { self }
}

impl AsRef<PageBlockFooter> for RTDPageBlockFooterBuilder {
  fn as_ref(&self) -> &PageBlockFooter { &self.inner }
}







/// A header
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockHeader {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Header
  header: RichText,
  
}

impl RObject for PageBlockHeader {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockHeader" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockHeader {}



impl PageBlockHeader {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockHeaderBuilder {
    let mut inner = PageBlockHeader::default();
    inner.td_name = "pageBlockHeader".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockHeaderBuilder { inner }
  }

  pub fn header(&self) -> &RichText { &self.header }

}

#[doc(hidden)]
pub struct RTDPageBlockHeaderBuilder {
  inner: PageBlockHeader
}

impl RTDPageBlockHeaderBuilder {
  pub fn build(&self) -> PageBlockHeader { self.inner.clone() }

   
  pub fn header<T: AsRef<RichText>>(&mut self, header: T) -> &mut Self {
    self.inner.header = header.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockHeader> for PageBlockHeader {
  fn as_ref(&self) -> &PageBlockHeader { self }
}

impl AsRef<PageBlockHeader> for RTDPageBlockHeaderBuilder {
  fn as_ref(&self) -> &PageBlockHeader { &self.inner }
}







/// A list of texts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockList {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Texts
  items: Vec<RichText>,
  /// True, if the items should be marked with numbers
  is_ordered: bool,
  
}

impl RObject for PageBlockList {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockList" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockList {}



impl PageBlockList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockListBuilder {
    let mut inner = PageBlockList::default();
    inner.td_name = "pageBlockList".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockListBuilder { inner }
  }

  pub fn items(&self) -> &Vec<RichText> { &self.items }

  pub fn is_ordered(&self) -> bool { self.is_ordered }

}

#[doc(hidden)]
pub struct RTDPageBlockListBuilder {
  inner: PageBlockList
}

impl RTDPageBlockListBuilder {
  pub fn build(&self) -> PageBlockList { self.inner.clone() }

   
  pub fn items(&mut self, items: Vec<RichText>) -> &mut Self {
    self.inner.items = items;
    self
  }

   
  pub fn is_ordered(&mut self, is_ordered: bool) -> &mut Self {
    self.inner.is_ordered = is_ordered;
    self
  }

}

impl AsRef<PageBlockList> for PageBlockList {
  fn as_ref(&self) -> &PageBlockList { self }
}

impl AsRef<PageBlockList> for RTDPageBlockListBuilder {
  fn as_ref(&self) -> &PageBlockList { &self.inner }
}







/// A text paragraph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockParagraph {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Paragraph text
  text: RichText,
  
}

impl RObject for PageBlockParagraph {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockParagraph" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockParagraph {}



impl PageBlockParagraph {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockParagraphBuilder {
    let mut inner = PageBlockParagraph::default();
    inner.td_name = "pageBlockParagraph".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockParagraphBuilder { inner }
  }

  pub fn text(&self) -> &RichText { &self.text }

}

#[doc(hidden)]
pub struct RTDPageBlockParagraphBuilder {
  inner: PageBlockParagraph
}

impl RTDPageBlockParagraphBuilder {
  pub fn build(&self) -> PageBlockParagraph { self.inner.clone() }

   
  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockParagraph> for PageBlockParagraph {
  fn as_ref(&self) -> &PageBlockParagraph { self }
}

impl AsRef<PageBlockParagraph> for RTDPageBlockParagraphBuilder {
  fn as_ref(&self) -> &PageBlockParagraph { &self.inner }
}







/// A photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo file; may be null
  photo: Option<Photo>,
  /// Photo caption
  caption: RichText,
  
}

impl RObject for PageBlockPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockPhoto" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockPhoto {}



impl PageBlockPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockPhotoBuilder {
    let mut inner = PageBlockPhoto::default();
    inner.td_name = "pageBlockPhoto".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Option<Photo> { &self.photo }

  pub fn caption(&self) -> &RichText { &self.caption }

}

#[doc(hidden)]
pub struct RTDPageBlockPhotoBuilder {
  inner: PageBlockPhoto
}

impl RTDPageBlockPhotoBuilder {
  pub fn build(&self) -> PageBlockPhoto { self.inner.clone() }

   
  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockPhoto> for PageBlockPhoto {
  fn as_ref(&self) -> &PageBlockPhoto { self }
}

impl AsRef<PageBlockPhoto> for RTDPageBlockPhotoBuilder {
  fn as_ref(&self) -> &PageBlockPhoto { &self.inner }
}







/// A preformatted text paragraph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockPreformatted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Paragraph text
  text: RichText,
  /// Programming language for which the text should be formatted
  language: String,
  
}

impl RObject for PageBlockPreformatted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockPreformatted" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockPreformatted {}



impl PageBlockPreformatted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockPreformattedBuilder {
    let mut inner = PageBlockPreformatted::default();
    inner.td_name = "pageBlockPreformatted".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockPreformattedBuilder { inner }
  }

  pub fn text(&self) -> &RichText { &self.text }

  pub fn language(&self) -> &String { &self.language }

}

#[doc(hidden)]
pub struct RTDPageBlockPreformattedBuilder {
  inner: PageBlockPreformatted
}

impl RTDPageBlockPreformattedBuilder {
  pub fn build(&self) -> PageBlockPreformatted { self.inner.clone() }

   
  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn language<T: AsRef<str>>(&mut self, language: T) -> &mut Self {
    self.inner.language = language.as_ref().to_string();
    self
  }

}

impl AsRef<PageBlockPreformatted> for PageBlockPreformatted {
  fn as_ref(&self) -> &PageBlockPreformatted { self }
}

impl AsRef<PageBlockPreformatted> for RTDPageBlockPreformattedBuilder {
  fn as_ref(&self) -> &PageBlockPreformatted { &self.inner }
}







/// A pull quote
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockPullQuote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Quote text
  text: RichText,
  /// Quote caption
  caption: RichText,
  
}

impl RObject for PageBlockPullQuote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockPullQuote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockPullQuote {}



impl PageBlockPullQuote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockPullQuoteBuilder {
    let mut inner = PageBlockPullQuote::default();
    inner.td_name = "pageBlockPullQuote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockPullQuoteBuilder { inner }
  }

  pub fn text(&self) -> &RichText { &self.text }

  pub fn caption(&self) -> &RichText { &self.caption }

}

#[doc(hidden)]
pub struct RTDPageBlockPullQuoteBuilder {
  inner: PageBlockPullQuote
}

impl RTDPageBlockPullQuoteBuilder {
  pub fn build(&self) -> PageBlockPullQuote { self.inner.clone() }

   
  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockPullQuote> for PageBlockPullQuote {
  fn as_ref(&self) -> &PageBlockPullQuote { self }
}

impl AsRef<PageBlockPullQuote> for RTDPageBlockPullQuoteBuilder {
  fn as_ref(&self) -> &PageBlockPullQuote { &self.inner }
}







/// A slideshow
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockSlideshow {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Slideshow item contents
  page_blocks: Vec<PageBlock>,
  /// Block caption
  caption: RichText,
  
}

impl RObject for PageBlockSlideshow {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockSlideshow" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockSlideshow {}



impl PageBlockSlideshow {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockSlideshowBuilder {
    let mut inner = PageBlockSlideshow::default();
    inner.td_name = "pageBlockSlideshow".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockSlideshowBuilder { inner }
  }

  pub fn page_blocks(&self) -> &Vec<PageBlock> { &self.page_blocks }

  pub fn caption(&self) -> &RichText { &self.caption }

}

#[doc(hidden)]
pub struct RTDPageBlockSlideshowBuilder {
  inner: PageBlockSlideshow
}

impl RTDPageBlockSlideshowBuilder {
  pub fn build(&self) -> PageBlockSlideshow { self.inner.clone() }

   
  pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
    self.inner.page_blocks = page_blocks;
    self
  }

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockSlideshow> for PageBlockSlideshow {
  fn as_ref(&self) -> &PageBlockSlideshow { self }
}

impl AsRef<PageBlockSlideshow> for RTDPageBlockSlideshowBuilder {
  fn as_ref(&self) -> &PageBlockSlideshow { &self.inner }
}







/// A subheader
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockSubheader {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subheader
  subheader: RichText,
  
}

impl RObject for PageBlockSubheader {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockSubheader" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockSubheader {}



impl PageBlockSubheader {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockSubheaderBuilder {
    let mut inner = PageBlockSubheader::default();
    inner.td_name = "pageBlockSubheader".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockSubheaderBuilder { inner }
  }

  pub fn subheader(&self) -> &RichText { &self.subheader }

}

#[doc(hidden)]
pub struct RTDPageBlockSubheaderBuilder {
  inner: PageBlockSubheader
}

impl RTDPageBlockSubheaderBuilder {
  pub fn build(&self) -> PageBlockSubheader { self.inner.clone() }

   
  pub fn subheader<T: AsRef<RichText>>(&mut self, subheader: T) -> &mut Self {
    self.inner.subheader = subheader.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockSubheader> for PageBlockSubheader {
  fn as_ref(&self) -> &PageBlockSubheader { self }
}

impl AsRef<PageBlockSubheader> for RTDPageBlockSubheaderBuilder {
  fn as_ref(&self) -> &PageBlockSubheader { &self.inner }
}







/// The subtitle of a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockSubtitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Subtitle
  subtitle: RichText,
  
}

impl RObject for PageBlockSubtitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockSubtitle" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockSubtitle {}



impl PageBlockSubtitle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockSubtitleBuilder {
    let mut inner = PageBlockSubtitle::default();
    inner.td_name = "pageBlockSubtitle".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockSubtitleBuilder { inner }
  }

  pub fn subtitle(&self) -> &RichText { &self.subtitle }

}

#[doc(hidden)]
pub struct RTDPageBlockSubtitleBuilder {
  inner: PageBlockSubtitle
}

impl RTDPageBlockSubtitleBuilder {
  pub fn build(&self) -> PageBlockSubtitle { self.inner.clone() }

   
  pub fn subtitle<T: AsRef<RichText>>(&mut self, subtitle: T) -> &mut Self {
    self.inner.subtitle = subtitle.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockSubtitle> for PageBlockSubtitle {
  fn as_ref(&self) -> &PageBlockSubtitle { self }
}

impl AsRef<PageBlockSubtitle> for RTDPageBlockSubtitleBuilder {
  fn as_ref(&self) -> &PageBlockSubtitle { &self.inner }
}







/// The title of a page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Title
  title: RichText,
  
}

impl RObject for PageBlockTitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockTitle" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockTitle {}



impl PageBlockTitle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockTitleBuilder {
    let mut inner = PageBlockTitle::default();
    inner.td_name = "pageBlockTitle".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockTitleBuilder { inner }
  }

  pub fn title(&self) -> &RichText { &self.title }

}

#[doc(hidden)]
pub struct RTDPageBlockTitleBuilder {
  inner: PageBlockTitle
}

impl RTDPageBlockTitleBuilder {
  pub fn build(&self) -> PageBlockTitle { self.inner.clone() }

   
  pub fn title<T: AsRef<RichText>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockTitle> for PageBlockTitle {
  fn as_ref(&self) -> &PageBlockTitle { self }
}

impl AsRef<PageBlockTitle> for RTDPageBlockTitleBuilder {
  fn as_ref(&self) -> &PageBlockTitle { &self.inner }
}







/// A video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Video file; may be null
  video: Option<Video>,
  /// Video caption
  caption: RichText,
  /// True, if the video should be played automatically
  need_autoplay: bool,
  /// True, if the video should be looped
  is_looped: bool,
  
}

impl RObject for PageBlockVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockVideo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlock for PageBlockVideo {}



impl PageBlockVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockVideoBuilder {
    let mut inner = PageBlockVideo::default();
    inner.td_name = "pageBlockVideo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockVideoBuilder { inner }
  }

  pub fn video(&self) -> &Option<Video> { &self.video }

  pub fn caption(&self) -> &RichText { &self.caption }

  pub fn need_autoplay(&self) -> bool { self.need_autoplay }

  pub fn is_looped(&self) -> bool { self.is_looped }

}

#[doc(hidden)]
pub struct RTDPageBlockVideoBuilder {
  inner: PageBlockVideo
}

impl RTDPageBlockVideoBuilder {
  pub fn build(&self) -> PageBlockVideo { self.inner.clone() }

   
  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = Some(video.as_ref().clone());
    self
  }

   
  pub fn caption<T: AsRef<RichText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

   
  pub fn need_autoplay(&mut self, need_autoplay: bool) -> &mut Self {
    self.inner.need_autoplay = need_autoplay;
    self
  }

   
  pub fn is_looped(&mut self, is_looped: bool) -> &mut Self {
    self.inner.is_looped = is_looped;
    self
  }

}

impl AsRef<PageBlockVideo> for PageBlockVideo {
  fn as_ref(&self) -> &PageBlockVideo { self }
}

impl AsRef<PageBlockVideo> for RTDPageBlockVideoBuilder {
  fn as_ref(&self) -> &PageBlockVideo { &self.inner }
}



