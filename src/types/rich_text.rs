
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a text object inside an instant-view web page
pub trait TDRichText: Debug + RObject {}

/// Describes a text object inside an instant-view web page
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RichText {
  #[doc(hidden)] _Default(()),
  /// A bold rich text
  Bold(RichTextBold),
  /// A rich text email link
  EmailAddress(RichTextEmailAddress),
  /// A fixed-width rich text
  Fixed(RichTextFixed),
  /// An italicized rich text
  Italic(RichTextItalic),
  /// A plain text
  Plain(RichTextPlain),
  /// A strike-through rich text
  Strikethrough(RichTextStrikethrough),
  /// An underlined rich text
  Underline(RichTextUnderline),
  /// A rich text URL link
  Url(RichTextUrl),
  /// A concatenation of rich texts
  RichTexts(RichTexts),

}

impl Default for RichText {
  fn default() -> Self { RichText::_Default(()) }
}

impl<'de> Deserialize<'de> for RichText {
  fn deserialize<D>(deserializer: D) -> Result<RichText, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      RichText,
      (richTextBold, Bold);
      (richTextEmailAddress, EmailAddress);
      (richTextFixed, Fixed);
      (richTextItalic, Italic);
      (richTextPlain, Plain);
      (richTextStrikethrough, Strikethrough);
      (richTextUnderline, Underline);
      (richTextUrl, Url);
      (richTexts, RichTexts);

    )(deserializer)
  }
}

impl RObject for RichText {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      RichText::Bold(t) => t.td_name(),
      RichText::EmailAddress(t) => t.td_name(),
      RichText::Fixed(t) => t.td_name(),
      RichText::Italic(t) => t.td_name(),
      RichText::Plain(t) => t.td_name(),
      RichText::Strikethrough(t) => t.td_name(),
      RichText::Underline(t) => t.td_name(),
      RichText::Url(t) => t.td_name(),
      RichText::RichTexts(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      RichText::Bold(t) => t.extra(),
      RichText::EmailAddress(t) => t.extra(),
      RichText::Fixed(t) => t.extra(),
      RichText::Italic(t) => t.extra(),
      RichText::Plain(t) => t.extra(),
      RichText::Strikethrough(t) => t.extra(),
      RichText::Underline(t) => t.extra(),
      RichText::Url(t) => t.extra(),
      RichText::RichTexts(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl RichText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let RichText::_Default(_) = self { true } else { false } }

  pub fn is_bold(&self) -> bool { if let RichText::Bold(_) = self { true } else { false } }
  pub fn is_email_address(&self) -> bool { if let RichText::EmailAddress(_) = self { true } else { false } }
  pub fn is_fixed(&self) -> bool { if let RichText::Fixed(_) = self { true } else { false } }
  pub fn is_italic(&self) -> bool { if let RichText::Italic(_) = self { true } else { false } }
  pub fn is_plain(&self) -> bool { if let RichText::Plain(_) = self { true } else { false } }
  pub fn is_strikethrough(&self) -> bool { if let RichText::Strikethrough(_) = self { true } else { false } }
  pub fn is_underline(&self) -> bool { if let RichText::Underline(_) = self { true } else { false } }
  pub fn is_url(&self) -> bool { if let RichText::Url(_) = self { true } else { false } }
  pub fn is_rich_texts(&self) -> bool { if let RichText::RichTexts(_) = self { true } else { false } }

  pub fn on_bold<F: FnOnce(&RichTextBold)>(&self, fnc: F) -> &Self { if let RichText::Bold(t) = self { fnc(t) }; self }
  pub fn on_email_address<F: FnOnce(&RichTextEmailAddress)>(&self, fnc: F) -> &Self { if let RichText::EmailAddress(t) = self { fnc(t) }; self }
  pub fn on_fixed<F: FnOnce(&RichTextFixed)>(&self, fnc: F) -> &Self { if let RichText::Fixed(t) = self { fnc(t) }; self }
  pub fn on_italic<F: FnOnce(&RichTextItalic)>(&self, fnc: F) -> &Self { if let RichText::Italic(t) = self { fnc(t) }; self }
  pub fn on_plain<F: FnOnce(&RichTextPlain)>(&self, fnc: F) -> &Self { if let RichText::Plain(t) = self { fnc(t) }; self }
  pub fn on_strikethrough<F: FnOnce(&RichTextStrikethrough)>(&self, fnc: F) -> &Self { if let RichText::Strikethrough(t) = self { fnc(t) }; self }
  pub fn on_underline<F: FnOnce(&RichTextUnderline)>(&self, fnc: F) -> &Self { if let RichText::Underline(t) = self { fnc(t) }; self }
  pub fn on_url<F: FnOnce(&RichTextUrl)>(&self, fnc: F) -> &Self { if let RichText::Url(t) = self { fnc(t) }; self }
  pub fn on_rich_texts<F: FnOnce(&RichTexts)>(&self, fnc: F) -> &Self { if let RichText::RichTexts(t) = self { fnc(t) }; self }

  pub fn as_bold(&self) -> Option<&RichTextBold> { if let RichText::Bold(t) = self { return Some(t) } None }
  pub fn as_email_address(&self) -> Option<&RichTextEmailAddress> { if let RichText::EmailAddress(t) = self { return Some(t) } None }
  pub fn as_fixed(&self) -> Option<&RichTextFixed> { if let RichText::Fixed(t) = self { return Some(t) } None }
  pub fn as_italic(&self) -> Option<&RichTextItalic> { if let RichText::Italic(t) = self { return Some(t) } None }
  pub fn as_plain(&self) -> Option<&RichTextPlain> { if let RichText::Plain(t) = self { return Some(t) } None }
  pub fn as_strikethrough(&self) -> Option<&RichTextStrikethrough> { if let RichText::Strikethrough(t) = self { return Some(t) } None }
  pub fn as_underline(&self) -> Option<&RichTextUnderline> { if let RichText::Underline(t) = self { return Some(t) } None }
  pub fn as_url(&self) -> Option<&RichTextUrl> { if let RichText::Url(t) = self { return Some(t) } None }
  pub fn as_rich_texts(&self) -> Option<&RichTexts> { if let RichText::RichTexts(t) = self { return Some(t) } None }



  pub fn bold<T: AsRef<RichTextBold>>(t: T) -> Self { RichText::Bold(t.as_ref().clone()) }

  pub fn email_address<T: AsRef<RichTextEmailAddress>>(t: T) -> Self { RichText::EmailAddress(t.as_ref().clone()) }

  pub fn fixed<T: AsRef<RichTextFixed>>(t: T) -> Self { RichText::Fixed(t.as_ref().clone()) }

  pub fn italic<T: AsRef<RichTextItalic>>(t: T) -> Self { RichText::Italic(t.as_ref().clone()) }

  pub fn plain<T: AsRef<RichTextPlain>>(t: T) -> Self { RichText::Plain(t.as_ref().clone()) }

  pub fn strikethrough<T: AsRef<RichTextStrikethrough>>(t: T) -> Self { RichText::Strikethrough(t.as_ref().clone()) }

  pub fn underline<T: AsRef<RichTextUnderline>>(t: T) -> Self { RichText::Underline(t.as_ref().clone()) }

  pub fn url<T: AsRef<RichTextUrl>>(t: T) -> Self { RichText::Url(t.as_ref().clone()) }

  pub fn rich_texts<T: AsRef<RichTexts>>(t: T) -> Self { RichText::RichTexts(t.as_ref().clone()) }

}

impl AsRef<RichText> for RichText {
  fn as_ref(&self) -> &RichText { self }
}







/// A bold rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextBold {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  
}

impl RObject for RichTextBold {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextBold" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDRichText for RichTextBold {}



impl RichTextBold {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRichTextBoldBuilder {
    let mut inner = RichTextBold::default();
    inner.td_name = "richTextBold".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRichTextBoldBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> { &self.text }

}

#[doc(hidden)]
pub struct RTDRichTextBoldBuilder {
  inner: RichTextBold
}

impl RTDRichTextBoldBuilder {
  pub fn build(&self) -> RichTextBold { self.inner.clone() }

   
  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

}

impl AsRef<RichTextBold> for RichTextBold {
  fn as_ref(&self) -> &RichTextBold { self }
}

impl AsRef<RichTextBold> for RTDRichTextBoldBuilder {
  fn as_ref(&self) -> &RichTextBold { &self.inner }
}







/// A rich text email link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// Email address
  email_address: String,
  
}

impl RObject for RichTextEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextEmailAddress" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDRichText for RichTextEmailAddress {}



impl RichTextEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRichTextEmailAddressBuilder {
    let mut inner = RichTextEmailAddress::default();
    inner.td_name = "richTextEmailAddress".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRichTextEmailAddressBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> { &self.text }

  pub fn email_address(&self) -> &String { &self.email_address }

}

#[doc(hidden)]
pub struct RTDRichTextEmailAddressBuilder {
  inner: RichTextEmailAddress
}

impl RTDRichTextEmailAddressBuilder {
  pub fn build(&self) -> RichTextEmailAddress { self.inner.clone() }

   
  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
    self.inner.email_address = email_address.as_ref().to_string();
    self
  }

}

impl AsRef<RichTextEmailAddress> for RichTextEmailAddress {
  fn as_ref(&self) -> &RichTextEmailAddress { self }
}

impl AsRef<RichTextEmailAddress> for RTDRichTextEmailAddressBuilder {
  fn as_ref(&self) -> &RichTextEmailAddress { &self.inner }
}







/// A fixed-width rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextFixed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  
}

impl RObject for RichTextFixed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextFixed" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDRichText for RichTextFixed {}



impl RichTextFixed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRichTextFixedBuilder {
    let mut inner = RichTextFixed::default();
    inner.td_name = "richTextFixed".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRichTextFixedBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> { &self.text }

}

#[doc(hidden)]
pub struct RTDRichTextFixedBuilder {
  inner: RichTextFixed
}

impl RTDRichTextFixedBuilder {
  pub fn build(&self) -> RichTextFixed { self.inner.clone() }

   
  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

}

impl AsRef<RichTextFixed> for RichTextFixed {
  fn as_ref(&self) -> &RichTextFixed { self }
}

impl AsRef<RichTextFixed> for RTDRichTextFixedBuilder {
  fn as_ref(&self) -> &RichTextFixed { &self.inner }
}







/// An italicized rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextItalic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  
}

impl RObject for RichTextItalic {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextItalic" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDRichText for RichTextItalic {}



impl RichTextItalic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRichTextItalicBuilder {
    let mut inner = RichTextItalic::default();
    inner.td_name = "richTextItalic".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRichTextItalicBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> { &self.text }

}

#[doc(hidden)]
pub struct RTDRichTextItalicBuilder {
  inner: RichTextItalic
}

impl RTDRichTextItalicBuilder {
  pub fn build(&self) -> RichTextItalic { self.inner.clone() }

   
  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

}

impl AsRef<RichTextItalic> for RichTextItalic {
  fn as_ref(&self) -> &RichTextItalic { self }
}

impl AsRef<RichTextItalic> for RTDRichTextItalicBuilder {
  fn as_ref(&self) -> &RichTextItalic { &self.inner }
}







/// A plain text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextPlain {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  
}

impl RObject for RichTextPlain {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextPlain" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDRichText for RichTextPlain {}



impl RichTextPlain {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRichTextPlainBuilder {
    let mut inner = RichTextPlain::default();
    inner.td_name = "richTextPlain".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRichTextPlainBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> { &self.text }

}

#[doc(hidden)]
pub struct RTDRichTextPlainBuilder {
  inner: RichTextPlain
}

impl RTDRichTextPlainBuilder {
  pub fn build(&self) -> RichTextPlain { self.inner.clone() }

   
  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

}

impl AsRef<RichTextPlain> for RichTextPlain {
  fn as_ref(&self) -> &RichTextPlain { self }
}

impl AsRef<RichTextPlain> for RTDRichTextPlainBuilder {
  fn as_ref(&self) -> &RichTextPlain { &self.inner }
}







/// A strike-through rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextStrikethrough {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  
}

impl RObject for RichTextStrikethrough {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextStrikethrough" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDRichText for RichTextStrikethrough {}



impl RichTextStrikethrough {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRichTextStrikethroughBuilder {
    let mut inner = RichTextStrikethrough::default();
    inner.td_name = "richTextStrikethrough".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRichTextStrikethroughBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> { &self.text }

}

#[doc(hidden)]
pub struct RTDRichTextStrikethroughBuilder {
  inner: RichTextStrikethrough
}

impl RTDRichTextStrikethroughBuilder {
  pub fn build(&self) -> RichTextStrikethrough { self.inner.clone() }

   
  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

}

impl AsRef<RichTextStrikethrough> for RichTextStrikethrough {
  fn as_ref(&self) -> &RichTextStrikethrough { self }
}

impl AsRef<RichTextStrikethrough> for RTDRichTextStrikethroughBuilder {
  fn as_ref(&self) -> &RichTextStrikethrough { &self.inner }
}







/// An underlined rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextUnderline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  
}

impl RObject for RichTextUnderline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextUnderline" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDRichText for RichTextUnderline {}



impl RichTextUnderline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRichTextUnderlineBuilder {
    let mut inner = RichTextUnderline::default();
    inner.td_name = "richTextUnderline".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRichTextUnderlineBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> { &self.text }

}

#[doc(hidden)]
pub struct RTDRichTextUnderlineBuilder {
  inner: RichTextUnderline
}

impl RTDRichTextUnderlineBuilder {
  pub fn build(&self) -> RichTextUnderline { self.inner.clone() }

   
  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

}

impl AsRef<RichTextUnderline> for RichTextUnderline {
  fn as_ref(&self) -> &RichTextUnderline { self }
}

impl AsRef<RichTextUnderline> for RTDRichTextUnderlineBuilder {
  fn as_ref(&self) -> &RichTextUnderline { &self.inner }
}







/// A rich text URL link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// URL
  url: String,
  
}

impl RObject for RichTextUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextUrl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDRichText for RichTextUrl {}



impl RichTextUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRichTextUrlBuilder {
    let mut inner = RichTextUrl::default();
    inner.td_name = "richTextUrl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRichTextUrlBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> { &self.text }

  pub fn url(&self) -> &String { &self.url }

}

#[doc(hidden)]
pub struct RTDRichTextUrlBuilder {
  inner: RichTextUrl
}

impl RTDRichTextUrlBuilder {
  pub fn build(&self) -> RichTextUrl { self.inner.clone() }

   
  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

}

impl AsRef<RichTextUrl> for RichTextUrl {
  fn as_ref(&self) -> &RichTextUrl { self }
}

impl AsRef<RichTextUrl> for RTDRichTextUrlBuilder {
  fn as_ref(&self) -> &RichTextUrl { &self.inner }
}







/// A concatenation of rich texts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTexts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Texts
  texts: Vec<RichText>,
  
}

impl RObject for RichTexts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTexts" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDRichText for RichTexts {}



impl RichTexts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRichTextsBuilder {
    let mut inner = RichTexts::default();
    inner.td_name = "richTexts".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDRichTextsBuilder { inner }
  }

  pub fn texts(&self) -> &Vec<RichText> { &self.texts }

}

#[doc(hidden)]
pub struct RTDRichTextsBuilder {
  inner: RichTexts
}

impl RTDRichTextsBuilder {
  pub fn build(&self) -> RichTexts { self.inner.clone() }

   
  pub fn texts(&mut self, texts: Vec<RichText>) -> &mut Self {
    self.inner.texts = texts;
    self
  }

}

impl AsRef<RichTexts> for RichTexts {
  fn as_ref(&self) -> &RichTexts { self }
}

impl AsRef<RichTexts> for RTDRichTextsBuilder {
  fn as_ref(&self) -> &RichTexts { &self.inner }
}



