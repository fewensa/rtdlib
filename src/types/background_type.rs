
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a type of a background
pub trait TDBackgroundType: Debug + RObject {}

/// Describes a type of a background
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BackgroundType {
  #[doc(hidden)] _Default(()),
  /// A PNG pattern to be combined with the color chosen by the user
  Pattern(BackgroundTypePattern),
  /// A solid background
  Solid(BackgroundTypeSolid),
  /// A wallpaper in JPEG format
  Wallpaper(BackgroundTypeWallpaper),

}

impl Default for BackgroundType {
  fn default() -> Self { BackgroundType::_Default(()) }
}

impl<'de> Deserialize<'de> for BackgroundType {
  fn deserialize<D>(deserializer: D) -> Result<BackgroundType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      BackgroundType,
      (backgroundTypePattern, Pattern);
      (backgroundTypeSolid, Solid);
      (backgroundTypeWallpaper, Wallpaper);

    )(deserializer)
  }
}

impl RObject for BackgroundType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      BackgroundType::Pattern(t) => t.td_name(),
      BackgroundType::Solid(t) => t.td_name(),
      BackgroundType::Wallpaper(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl BackgroundType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let BackgroundType::_Default(_) = self { true } else { false } }

  pub fn is_pattern(&self) -> bool { if let BackgroundType::Pattern(_) = self { true } else { false } }
  pub fn is_solid(&self) -> bool { if let BackgroundType::Solid(_) = self { true } else { false } }
  pub fn is_wallpaper(&self) -> bool { if let BackgroundType::Wallpaper(_) = self { true } else { false } }

  pub fn on_pattern<F: FnOnce(&BackgroundTypePattern)>(&self, fnc: F) -> &Self { if let BackgroundType::Pattern(t) = self { fnc(t) }; self }
  pub fn on_solid<F: FnOnce(&BackgroundTypeSolid)>(&self, fnc: F) -> &Self { if let BackgroundType::Solid(t) = self { fnc(t) }; self }
  pub fn on_wallpaper<F: FnOnce(&BackgroundTypeWallpaper)>(&self, fnc: F) -> &Self { if let BackgroundType::Wallpaper(t) = self { fnc(t) }; self }

  pub fn as_pattern(&self) -> Option<&BackgroundTypePattern> { if let BackgroundType::Pattern(t) = self { return Some(t) } None }
  pub fn as_solid(&self) -> Option<&BackgroundTypeSolid> { if let BackgroundType::Solid(t) = self { return Some(t) } None }
  pub fn as_wallpaper(&self) -> Option<&BackgroundTypeWallpaper> { if let BackgroundType::Wallpaper(t) = self { return Some(t) } None }



  pub fn pattern<T: AsRef<BackgroundTypePattern>>(t: T) -> Self { BackgroundType::Pattern(t.as_ref().clone()) }

  pub fn solid<T: AsRef<BackgroundTypeSolid>>(t: T) -> Self { BackgroundType::Solid(t.as_ref().clone()) }

  pub fn wallpaper<T: AsRef<BackgroundTypeWallpaper>>(t: T) -> Self { BackgroundType::Wallpaper(t.as_ref().clone()) }

}

impl AsRef<BackgroundType> for BackgroundType {
  fn as_ref(&self) -> &BackgroundType { self }
}







/// A PNG pattern to be combined with the color chosen by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundTypePattern {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// True, if the background needs to be slightly moved when device is rotated
  is_moving: bool,
  /// Main color of the background in RGB24 format
  color: i64,
  /// Intensity of the pattern when it is shown above the main background color, 0-100
  intensity: i64,
  
}

impl RObject for BackgroundTypePattern {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "backgroundTypePattern" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBackgroundType for BackgroundTypePattern {}



impl BackgroundTypePattern {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundTypePatternBuilder {
    let mut inner = BackgroundTypePattern::default();
    inner.td_name = "backgroundTypePattern".to_string();
    RTDBackgroundTypePatternBuilder { inner }
  }

  pub fn is_moving(&self) -> bool { self.is_moving }

  pub fn color(&self) -> i64 { self.color }

  pub fn intensity(&self) -> i64 { self.intensity }

}

#[doc(hidden)]
pub struct RTDBackgroundTypePatternBuilder {
  inner: BackgroundTypePattern
}

impl RTDBackgroundTypePatternBuilder {
  pub fn build(&self) -> BackgroundTypePattern { self.inner.clone() }

   
  pub fn is_moving(&mut self, is_moving: bool) -> &mut Self {
    self.inner.is_moving = is_moving;
    self
  }

   
  pub fn color(&mut self, color: i64) -> &mut Self {
    self.inner.color = color;
    self
  }

   
  pub fn intensity(&mut self, intensity: i64) -> &mut Self {
    self.inner.intensity = intensity;
    self
  }

}

impl AsRef<BackgroundTypePattern> for BackgroundTypePattern {
  fn as_ref(&self) -> &BackgroundTypePattern { self }
}

impl AsRef<BackgroundTypePattern> for RTDBackgroundTypePatternBuilder {
  fn as_ref(&self) -> &BackgroundTypePattern { &self.inner }
}







/// A solid background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundTypeSolid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// A color of the background in RGB24 format
  color: i64,
  
}

impl RObject for BackgroundTypeSolid {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "backgroundTypeSolid" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBackgroundType for BackgroundTypeSolid {}



impl BackgroundTypeSolid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundTypeSolidBuilder {
    let mut inner = BackgroundTypeSolid::default();
    inner.td_name = "backgroundTypeSolid".to_string();
    RTDBackgroundTypeSolidBuilder { inner }
  }

  pub fn color(&self) -> i64 { self.color }

}

#[doc(hidden)]
pub struct RTDBackgroundTypeSolidBuilder {
  inner: BackgroundTypeSolid
}

impl RTDBackgroundTypeSolidBuilder {
  pub fn build(&self) -> BackgroundTypeSolid { self.inner.clone() }

   
  pub fn color(&mut self, color: i64) -> &mut Self {
    self.inner.color = color;
    self
  }

}

impl AsRef<BackgroundTypeSolid> for BackgroundTypeSolid {
  fn as_ref(&self) -> &BackgroundTypeSolid { self }
}

impl AsRef<BackgroundTypeSolid> for RTDBackgroundTypeSolidBuilder {
  fn as_ref(&self) -> &BackgroundTypeSolid { &self.inner }
}







/// A wallpaper in JPEG format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundTypeWallpaper {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// True, if the wallpaper must be downscaled to fit in 450x450 square and then box-blurred with radius 12
  is_blurred: bool,
  /// True, if the background needs to be slightly moved when device is rotated
  is_moving: bool,
  
}

impl RObject for BackgroundTypeWallpaper {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "backgroundTypeWallpaper" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBackgroundType for BackgroundTypeWallpaper {}



impl BackgroundTypeWallpaper {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundTypeWallpaperBuilder {
    let mut inner = BackgroundTypeWallpaper::default();
    inner.td_name = "backgroundTypeWallpaper".to_string();
    RTDBackgroundTypeWallpaperBuilder { inner }
  }

  pub fn is_blurred(&self) -> bool { self.is_blurred }

  pub fn is_moving(&self) -> bool { self.is_moving }

}

#[doc(hidden)]
pub struct RTDBackgroundTypeWallpaperBuilder {
  inner: BackgroundTypeWallpaper
}

impl RTDBackgroundTypeWallpaperBuilder {
  pub fn build(&self) -> BackgroundTypeWallpaper { self.inner.clone() }

   
  pub fn is_blurred(&mut self, is_blurred: bool) -> &mut Self {
    self.inner.is_blurred = is_blurred;
    self
  }

   
  pub fn is_moving(&mut self, is_moving: bool) -> &mut Self {
    self.inner.is_moving = is_moving;
    self
  }

}

impl AsRef<BackgroundTypeWallpaper> for BackgroundTypeWallpaper {
  fn as_ref(&self) -> &BackgroundTypeWallpaper { self }
}

impl AsRef<BackgroundTypeWallpaper> for RTDBackgroundTypeWallpaperBuilder {
  fn as_ref(&self) -> &BackgroundTypeWallpaper { &self.inner }
}



