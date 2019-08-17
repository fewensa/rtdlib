
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockPhoto
  /// Photo file; may be null.
  photo: Option<Photo>,
  /// Photo caption.
  caption: Option<PageBlockCaption>,
  /// URL that needs to be opened when the photo is clicked.
  url: Option<String>,
  
}



impl Object for PageBlockPhoto {}
impl RObject for PageBlockPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockPhoto" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockPhoto {}


impl PageBlockPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockPhoto".to_string(),
      photo: None,
      caption: None,
      url: None,
      
    }
  }
  
  pub fn photo(&self) -> Option<Photo> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Photo) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn caption(&self) -> Option<PageBlockCaption> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: PageBlockCaption) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



