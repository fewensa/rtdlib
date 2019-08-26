
use crate::types::*;
use crate::errors::*;




/// Contains a list of wallpapers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Wallpapers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// A list of wallpapers
  wallpapers: Vec<Wallpaper>,
  
}

impl RObject for Wallpapers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "wallpapers" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Wallpapers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDWallpapersBuilder {
    let mut inner = Wallpapers::default();
    inner.td_name = "wallpapers".to_string();
    RTDWallpapersBuilder { inner }
  }

  pub fn wallpapers(&self) -> &Vec<Wallpaper> { &self.wallpapers }

}

#[doc(hidden)]
pub struct RTDWallpapersBuilder {
  inner: Wallpapers
}

impl RTDWallpapersBuilder {
  pub fn build(&self) -> Wallpapers { self.inner.clone() }

   
  pub fn wallpapers(&mut self, wallpapers: Vec<Wallpaper>) -> &mut Self {
    self.inner.wallpapers = wallpapers;
    self
  }

}

impl AsRef<Wallpapers> for Wallpapers {
  fn as_ref(&self) -> &Wallpapers { self }
}

impl AsRef<Wallpapers> for RTDWallpapersBuilder {
  fn as_ref(&self) -> &Wallpapers { &self.inner }
}



