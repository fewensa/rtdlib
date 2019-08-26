
use crate::types::*;
use crate::errors::*;




/// Contains information about a wallpaper
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Wallpaper {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Unique persistent wallpaper identifier
  id: i64,
  /// Available variants of the wallpaper in different sizes. These photos can only be downloaded; they can't be sent in a message
  sizes: Vec<PhotoSize>,
  /// Main color of the wallpaper in RGB24 format; should be treated as background color if no photos are specified
  color: i64,
  
}

impl RObject for Wallpaper {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "wallpaper" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Wallpaper {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDWallpaperBuilder {
    let mut inner = Wallpaper::default();
    inner.td_name = "wallpaper".to_string();
    RTDWallpaperBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn sizes(&self) -> &Vec<PhotoSize> { &self.sizes }

  pub fn color(&self) -> i64 { self.color }

}

#[doc(hidden)]
pub struct RTDWallpaperBuilder {
  inner: Wallpaper
}

impl RTDWallpaperBuilder {
  pub fn build(&self) -> Wallpaper { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn sizes(&mut self, sizes: Vec<PhotoSize>) -> &mut Self {
    self.inner.sizes = sizes;
    self
  }

   
  pub fn color(&mut self, color: i64) -> &mut Self {
    self.inner.color = color;
    self
  }

}

impl AsRef<Wallpaper> for Wallpaper {
  fn as_ref(&self) -> &Wallpaper { self }
}

impl AsRef<Wallpaper> for RTDWallpaperBuilder {
  fn as_ref(&self) -> &Wallpaper { &self.inner }
}



