
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Position on a photo where a mask should be placed. 
#[derive(Debug, Serialize, Deserialize)]
pub struct MaskPosition {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // maskPosition
  /// Part of the face, relative to which the mask should be placed.
  point: Option<Box<MaskPoint>>,
  /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. (For example, -1.0 will place the mask just to the left of the default mask position.)
  x_shift: Option<f64>,
  /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. (For example, 1.0 will place the mask just below the default mask position.)
  y_shift: Option<f64>,
  /// Mask scaling coefficient. (For example, 2.0 means a doubled size.)
  scale: Option<f64>,
  
}


impl Clone for MaskPosition {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for MaskPosition {}
impl RObject for MaskPosition {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "maskPosition" }
  fn td_type(&self) -> RTDType { RTDType::MaskPosition }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl MaskPosition {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "maskPosition".to_string(),
      point: None,
      x_shift: None,
      y_shift: None,
      scale: None,
      
    }
  }
  
  pub fn point(&self) -> Option<Box<MaskPoint>> { self.point.clone() }
  #[doc(hidden)] pub fn _set_point(&mut self, point: Box<MaskPoint>) -> &mut Self { self.point = Some(point); self }
  
  pub fn x_shift(&self) -> Option<f64> { self.x_shift.clone() }
  #[doc(hidden)] pub fn _set_x_shift(&mut self, x_shift: f64) -> &mut Self { self.x_shift = Some(x_shift); self }
  
  pub fn y_shift(&self) -> Option<f64> { self.y_shift.clone() }
  #[doc(hidden)] pub fn _set_y_shift(&mut self, y_shift: f64) -> &mut Self { self.y_shift = Some(y_shift); self }
  
  pub fn scale(&self) -> Option<f64> { self.scale.clone() }
  #[doc(hidden)] pub fn _set_scale(&mut self, scale: f64) -> &mut Self { self.scale = Some(scale); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



