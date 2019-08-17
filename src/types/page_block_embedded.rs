
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An embedded web page. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockEmbedded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockEmbedded
  /// Web page URL, if available.
  url: Option<String>,
  /// HTML-markup of the embedded page.
  html: Option<String>,
  /// Poster photo, if available; may be null.
  poster_photo: Option<Photo>,
  /// Block width, 0 if unknown.
  width: Option<i32>,
  /// Block height, 0 if unknown.
  height: Option<i32>,
  /// Block caption.
  caption: Option<PageBlockCaption>,
  /// True, if the block should be full width.
  is_full_width: Option<bool>,
  /// True, if scrolling should be allowed.
  allow_scrolling: Option<bool>,
  
}



impl Object for PageBlockEmbedded {}
impl RObject for PageBlockEmbedded {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockEmbedded" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockEmbedded }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockEmbedded {}


impl PageBlockEmbedded {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockEmbedded".to_string(),
      url: None,
      html: None,
      poster_photo: None,
      width: None,
      height: None,
      caption: None,
      is_full_width: None,
      allow_scrolling: None,
      
    }
  }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn html(&self) -> Option<String> { self.html.clone() }
  #[doc(hidden)] pub fn _set_html(&mut self, html: String) -> &mut Self { self.html = Some(html); self }
  
  pub fn poster_photo(&self) -> Option<Photo> { self.poster_photo.clone() }
  #[doc(hidden)] pub fn _set_poster_photo(&mut self, poster_photo: Photo) -> &mut Self { self.poster_photo = Some(poster_photo); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn caption(&self) -> Option<PageBlockCaption> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: PageBlockCaption) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn is_full_width(&self) -> Option<bool> { self.is_full_width.clone() }
  #[doc(hidden)] pub fn _set_is_full_width(&mut self, is_full_width: bool) -> &mut Self { self.is_full_width = Some(is_full_width); self }
  
  pub fn allow_scrolling(&self) -> Option<bool> { self.allow_scrolling.clone() }
  #[doc(hidden)] pub fn _set_allow_scrolling(&mut self, allow_scrolling: bool) -> &mut Self { self.allow_scrolling = Some(allow_scrolling); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



