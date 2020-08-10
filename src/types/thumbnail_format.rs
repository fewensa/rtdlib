
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes format of the thumbnail
pub trait TDThumbnailFormat: Debug + RObject {}

/// Describes format of the thumbnail
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ThumbnailFormat {
  #[doc(hidden)] _Default(()),
  /// The thumbnail is in static GIF format. It will be used only for some bot inline results
  Gif(ThumbnailFormatGif),
  /// The thumbnail is in JPEG format
  Jpeg(ThumbnailFormatJpeg),
  /// The thumbnail is in MPEG4 format. It will be used only for some animations and videos
  Mpeg4(ThumbnailFormatMpeg4),
  /// The thumbnail is in PNG format. It will be used only for background patterns
  Png(ThumbnailFormatPng),
  /// The thumbnail is in TGS format. It will be used only for animated sticker sets
  Tgs(ThumbnailFormatTgs),
  /// The thumbnail is in WEBP format. It will be used only for some stickers
  Webp(ThumbnailFormatWebp),

}

impl Default for ThumbnailFormat {
  fn default() -> Self { ThumbnailFormat::_Default(()) }
}

impl<'de> Deserialize<'de> for ThumbnailFormat {
  fn deserialize<D>(deserializer: D) -> Result<ThumbnailFormat, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ThumbnailFormat,
      (thumbnailFormatGif, Gif);
      (thumbnailFormatJpeg, Jpeg);
      (thumbnailFormatMpeg4, Mpeg4);
      (thumbnailFormatPng, Png);
      (thumbnailFormatTgs, Tgs);
      (thumbnailFormatWebp, Webp);

    )(deserializer)
  }
}

impl RObject for ThumbnailFormat {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ThumbnailFormat::Gif(t) => t.td_name(),
      ThumbnailFormat::Jpeg(t) => t.td_name(),
      ThumbnailFormat::Mpeg4(t) => t.td_name(),
      ThumbnailFormat::Png(t) => t.td_name(),
      ThumbnailFormat::Tgs(t) => t.td_name(),
      ThumbnailFormat::Webp(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ThumbnailFormat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ThumbnailFormat::_Default(_) = self { true } else { false } }

  pub fn is_gif(&self) -> bool { if let ThumbnailFormat::Gif(_) = self { true } else { false } }
  pub fn is_jpeg(&self) -> bool { if let ThumbnailFormat::Jpeg(_) = self { true } else { false } }
  pub fn is_mpeg4(&self) -> bool { if let ThumbnailFormat::Mpeg4(_) = self { true } else { false } }
  pub fn is_png(&self) -> bool { if let ThumbnailFormat::Png(_) = self { true } else { false } }
  pub fn is_tgs(&self) -> bool { if let ThumbnailFormat::Tgs(_) = self { true } else { false } }
  pub fn is_webp(&self) -> bool { if let ThumbnailFormat::Webp(_) = self { true } else { false } }

  pub fn on_gif<F: FnOnce(&ThumbnailFormatGif)>(&self, fnc: F) -> &Self { if let ThumbnailFormat::Gif(t) = self { fnc(t) }; self }
  pub fn on_jpeg<F: FnOnce(&ThumbnailFormatJpeg)>(&self, fnc: F) -> &Self { if let ThumbnailFormat::Jpeg(t) = self { fnc(t) }; self }
  pub fn on_mpeg4<F: FnOnce(&ThumbnailFormatMpeg4)>(&self, fnc: F) -> &Self { if let ThumbnailFormat::Mpeg4(t) = self { fnc(t) }; self }
  pub fn on_png<F: FnOnce(&ThumbnailFormatPng)>(&self, fnc: F) -> &Self { if let ThumbnailFormat::Png(t) = self { fnc(t) }; self }
  pub fn on_tgs<F: FnOnce(&ThumbnailFormatTgs)>(&self, fnc: F) -> &Self { if let ThumbnailFormat::Tgs(t) = self { fnc(t) }; self }
  pub fn on_webp<F: FnOnce(&ThumbnailFormatWebp)>(&self, fnc: F) -> &Self { if let ThumbnailFormat::Webp(t) = self { fnc(t) }; self }

  pub fn as_gif(&self) -> Option<&ThumbnailFormatGif> { if let ThumbnailFormat::Gif(t) = self { return Some(t) } None }
  pub fn as_jpeg(&self) -> Option<&ThumbnailFormatJpeg> { if let ThumbnailFormat::Jpeg(t) = self { return Some(t) } None }
  pub fn as_mpeg4(&self) -> Option<&ThumbnailFormatMpeg4> { if let ThumbnailFormat::Mpeg4(t) = self { return Some(t) } None }
  pub fn as_png(&self) -> Option<&ThumbnailFormatPng> { if let ThumbnailFormat::Png(t) = self { return Some(t) } None }
  pub fn as_tgs(&self) -> Option<&ThumbnailFormatTgs> { if let ThumbnailFormat::Tgs(t) = self { return Some(t) } None }
  pub fn as_webp(&self) -> Option<&ThumbnailFormatWebp> { if let ThumbnailFormat::Webp(t) = self { return Some(t) } None }



  pub fn gif<T: AsRef<ThumbnailFormatGif>>(t: T) -> Self { ThumbnailFormat::Gif(t.as_ref().clone()) }

  pub fn jpeg<T: AsRef<ThumbnailFormatJpeg>>(t: T) -> Self { ThumbnailFormat::Jpeg(t.as_ref().clone()) }

  pub fn mpeg4<T: AsRef<ThumbnailFormatMpeg4>>(t: T) -> Self { ThumbnailFormat::Mpeg4(t.as_ref().clone()) }

  pub fn png<T: AsRef<ThumbnailFormatPng>>(t: T) -> Self { ThumbnailFormat::Png(t.as_ref().clone()) }

  pub fn tgs<T: AsRef<ThumbnailFormatTgs>>(t: T) -> Self { ThumbnailFormat::Tgs(t.as_ref().clone()) }

  pub fn webp<T: AsRef<ThumbnailFormatWebp>>(t: T) -> Self { ThumbnailFormat::Webp(t.as_ref().clone()) }

}

impl AsRef<ThumbnailFormat> for ThumbnailFormat {
  fn as_ref(&self) -> &ThumbnailFormat { self }
}







/// The thumbnail is in static GIF format. It will be used only for some bot inline results
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatGif {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ThumbnailFormatGif {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "thumbnailFormatGif" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDThumbnailFormat for ThumbnailFormatGif {}



impl ThumbnailFormatGif {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDThumbnailFormatGifBuilder {
    let mut inner = ThumbnailFormatGif::default();
    inner.td_name = "thumbnailFormatGif".to_string();
    RTDThumbnailFormatGifBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDThumbnailFormatGifBuilder {
  inner: ThumbnailFormatGif
}

impl RTDThumbnailFormatGifBuilder {
  pub fn build(&self) -> ThumbnailFormatGif { self.inner.clone() }

}

impl AsRef<ThumbnailFormatGif> for ThumbnailFormatGif {
  fn as_ref(&self) -> &ThumbnailFormatGif { self }
}

impl AsRef<ThumbnailFormatGif> for RTDThumbnailFormatGifBuilder {
  fn as_ref(&self) -> &ThumbnailFormatGif { &self.inner }
}







/// The thumbnail is in JPEG format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatJpeg {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ThumbnailFormatJpeg {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "thumbnailFormatJpeg" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDThumbnailFormat for ThumbnailFormatJpeg {}



impl ThumbnailFormatJpeg {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDThumbnailFormatJpegBuilder {
    let mut inner = ThumbnailFormatJpeg::default();
    inner.td_name = "thumbnailFormatJpeg".to_string();
    RTDThumbnailFormatJpegBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDThumbnailFormatJpegBuilder {
  inner: ThumbnailFormatJpeg
}

impl RTDThumbnailFormatJpegBuilder {
  pub fn build(&self) -> ThumbnailFormatJpeg { self.inner.clone() }

}

impl AsRef<ThumbnailFormatJpeg> for ThumbnailFormatJpeg {
  fn as_ref(&self) -> &ThumbnailFormatJpeg { self }
}

impl AsRef<ThumbnailFormatJpeg> for RTDThumbnailFormatJpegBuilder {
  fn as_ref(&self) -> &ThumbnailFormatJpeg { &self.inner }
}







/// The thumbnail is in MPEG4 format. It will be used only for some animations and videos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatMpeg4 {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ThumbnailFormatMpeg4 {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "thumbnailFormatMpeg4" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDThumbnailFormat for ThumbnailFormatMpeg4 {}



impl ThumbnailFormatMpeg4 {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDThumbnailFormatMpeg4Builder {
    let mut inner = ThumbnailFormatMpeg4::default();
    inner.td_name = "thumbnailFormatMpeg4".to_string();
    RTDThumbnailFormatMpeg4Builder { inner }
  }

}

#[doc(hidden)]
pub struct RTDThumbnailFormatMpeg4Builder {
  inner: ThumbnailFormatMpeg4
}

impl RTDThumbnailFormatMpeg4Builder {
  pub fn build(&self) -> ThumbnailFormatMpeg4 { self.inner.clone() }

}

impl AsRef<ThumbnailFormatMpeg4> for ThumbnailFormatMpeg4 {
  fn as_ref(&self) -> &ThumbnailFormatMpeg4 { self }
}

impl AsRef<ThumbnailFormatMpeg4> for RTDThumbnailFormatMpeg4Builder {
  fn as_ref(&self) -> &ThumbnailFormatMpeg4 { &self.inner }
}







/// The thumbnail is in PNG format. It will be used only for background patterns
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatPng {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ThumbnailFormatPng {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "thumbnailFormatPng" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDThumbnailFormat for ThumbnailFormatPng {}



impl ThumbnailFormatPng {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDThumbnailFormatPngBuilder {
    let mut inner = ThumbnailFormatPng::default();
    inner.td_name = "thumbnailFormatPng".to_string();
    RTDThumbnailFormatPngBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDThumbnailFormatPngBuilder {
  inner: ThumbnailFormatPng
}

impl RTDThumbnailFormatPngBuilder {
  pub fn build(&self) -> ThumbnailFormatPng { self.inner.clone() }

}

impl AsRef<ThumbnailFormatPng> for ThumbnailFormatPng {
  fn as_ref(&self) -> &ThumbnailFormatPng { self }
}

impl AsRef<ThumbnailFormatPng> for RTDThumbnailFormatPngBuilder {
  fn as_ref(&self) -> &ThumbnailFormatPng { &self.inner }
}







/// The thumbnail is in TGS format. It will be used only for animated sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatTgs {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ThumbnailFormatTgs {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "thumbnailFormatTgs" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDThumbnailFormat for ThumbnailFormatTgs {}



impl ThumbnailFormatTgs {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDThumbnailFormatTgsBuilder {
    let mut inner = ThumbnailFormatTgs::default();
    inner.td_name = "thumbnailFormatTgs".to_string();
    RTDThumbnailFormatTgsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDThumbnailFormatTgsBuilder {
  inner: ThumbnailFormatTgs
}

impl RTDThumbnailFormatTgsBuilder {
  pub fn build(&self) -> ThumbnailFormatTgs { self.inner.clone() }

}

impl AsRef<ThumbnailFormatTgs> for ThumbnailFormatTgs {
  fn as_ref(&self) -> &ThumbnailFormatTgs { self }
}

impl AsRef<ThumbnailFormatTgs> for RTDThumbnailFormatTgsBuilder {
  fn as_ref(&self) -> &ThumbnailFormatTgs { &self.inner }
}







/// The thumbnail is in WEBP format. It will be used only for some stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThumbnailFormatWebp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ThumbnailFormatWebp {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "thumbnailFormatWebp" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDThumbnailFormat for ThumbnailFormatWebp {}



impl ThumbnailFormatWebp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDThumbnailFormatWebpBuilder {
    let mut inner = ThumbnailFormatWebp::default();
    inner.td_name = "thumbnailFormatWebp".to_string();
    RTDThumbnailFormatWebpBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDThumbnailFormatWebpBuilder {
  inner: ThumbnailFormatWebp
}

impl RTDThumbnailFormatWebpBuilder {
  pub fn build(&self) -> ThumbnailFormatWebp { self.inner.clone() }

}

impl AsRef<ThumbnailFormatWebp> for ThumbnailFormatWebp {
  fn as_ref(&self) -> &ThumbnailFormatWebp { self }
}

impl AsRef<ThumbnailFormatWebp> for RTDThumbnailFormatWebpBuilder {
  fn as_ref(&self) -> &ThumbnailFormatWebp { &self.inner }
}



