use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes position of a rectangle area on a story media or a TON wallet card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AreaPosition {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The abscissa of the rectangle's center, as a percentage of the media or the card width
  x_percentage: f32,
  /// The ordinate of the rectangle's center, as a percentage of the media or the card height
  y_percentage: f32,
  /// The width of the rectangle, as a percentage of the media or the card width
  width_percentage: f32,
  /// The height of the rectangle, as a percentage of the media or the card height
  height_percentage: f32,
  /// Clockwise rotation angle of the rectangle, in degrees; 0-360
  rotation_angle: f32,
  /// The radius of the rectangle corner rounding, as a percentage of the media or the card width
  corner_radius_percentage: f32,
}

impl RObject for AreaPosition {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "areaPosition"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AreaPosition {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAreaPositionBuilder {
    let inner = AreaPosition {
      td_name: "areaPosition".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAreaPositionBuilder { inner }
  }

  pub fn x_percentage(&self) -> f32 {
    self.x_percentage
  }

  pub fn y_percentage(&self) -> f32 {
    self.y_percentage
  }

  pub fn width_percentage(&self) -> f32 {
    self.width_percentage
  }

  pub fn height_percentage(&self) -> f32 {
    self.height_percentage
  }

  pub fn rotation_angle(&self) -> f32 {
    self.rotation_angle
  }

  pub fn corner_radius_percentage(&self) -> f32 {
    self.corner_radius_percentage
  }
}

#[doc(hidden)]
pub struct RTDAreaPositionBuilder {
  inner: AreaPosition,
}

impl RTDAreaPositionBuilder {
  pub fn build(&self) -> AreaPosition {
    self.inner.clone()
  }

  pub fn x_percentage(&mut self, x_percentage: f32) -> &mut Self {
    self.inner.x_percentage = x_percentage;
    self
  }

  pub fn y_percentage(&mut self, y_percentage: f32) -> &mut Self {
    self.inner.y_percentage = y_percentage;
    self
  }

  pub fn width_percentage(&mut self, width_percentage: f32) -> &mut Self {
    self.inner.width_percentage = width_percentage;
    self
  }

  pub fn height_percentage(&mut self, height_percentage: f32) -> &mut Self {
    self.inner.height_percentage = height_percentage;
    self
  }

  pub fn rotation_angle(&mut self, rotation_angle: f32) -> &mut Self {
    self.inner.rotation_angle = rotation_angle;
    self
  }

  pub fn corner_radius_percentage(&mut self, corner_radius_percentage: f32) -> &mut Self {
    self.inner.corner_radius_percentage = corner_radius_percentage;
    self
  }
}

impl AsRef<AreaPosition> for AreaPosition {
  fn as_ref(&self) -> &AreaPosition {
    self
  }
}

impl AsRef<AreaPosition> for RTDAreaPositionBuilder {
  fn as_ref(&self) -> &AreaPosition {
    &self.inner
  }
}
