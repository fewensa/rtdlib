
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a vector path command
pub trait TDVectorPathCommand: Debug + RObject {}

/// Represents a vector path command
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum VectorPathCommand {
  #[doc(hidden)] _Default(()),
  /// A cubic Bézier curve to a given point
  CubicBezierCurve(VectorPathCommandCubicBezierCurve),
  /// A straight line to a given point
  Line(VectorPathCommandLine),

}

impl Default for VectorPathCommand {
  fn default() -> Self { VectorPathCommand::_Default(()) }
}

impl<'de> Deserialize<'de> for VectorPathCommand {
  fn deserialize<D>(deserializer: D) -> Result<VectorPathCommand, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      VectorPathCommand,
      (vectorPathCommandCubicBezierCurve, CubicBezierCurve);
      (vectorPathCommandLine, Line);

    )(deserializer)
  }
}

impl RObject for VectorPathCommand {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      VectorPathCommand::CubicBezierCurve(t) => t.td_name(),
      VectorPathCommand::Line(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      VectorPathCommand::CubicBezierCurve(t) => t.extra(),
      VectorPathCommand::Line(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl VectorPathCommand {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let VectorPathCommand::_Default(_) = self { true } else { false } }

  pub fn is_cubic_bezier_curve(&self) -> bool { if let VectorPathCommand::CubicBezierCurve(_) = self { true } else { false } }
  pub fn is_line(&self) -> bool { if let VectorPathCommand::Line(_) = self { true } else { false } }

  pub fn on_cubic_bezier_curve<F: FnOnce(&VectorPathCommandCubicBezierCurve)>(&self, fnc: F) -> &Self { if let VectorPathCommand::CubicBezierCurve(t) = self { fnc(t) }; self }
  pub fn on_line<F: FnOnce(&VectorPathCommandLine)>(&self, fnc: F) -> &Self { if let VectorPathCommand::Line(t) = self { fnc(t) }; self }

  pub fn as_cubic_bezier_curve(&self) -> Option<&VectorPathCommandCubicBezierCurve> { if let VectorPathCommand::CubicBezierCurve(t) = self { return Some(t) } None }
  pub fn as_line(&self) -> Option<&VectorPathCommandLine> { if let VectorPathCommand::Line(t) = self { return Some(t) } None }



  pub fn cubic_bezier_curve<T: AsRef<VectorPathCommandCubicBezierCurve>>(t: T) -> Self { VectorPathCommand::CubicBezierCurve(t.as_ref().clone()) }

  pub fn line<T: AsRef<VectorPathCommandLine>>(t: T) -> Self { VectorPathCommand::Line(t.as_ref().clone()) }

}

impl AsRef<VectorPathCommand> for VectorPathCommand {
  fn as_ref(&self) -> &VectorPathCommand { self }
}







/// A cubic Bézier curve to a given point
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorPathCommandCubicBezierCurve {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The start control point of the curve
  start_control_point: Point,
  /// The end control point of the curve
  end_control_point: Point,
  /// The end point of the curve
  end_point: Point,
  
}

impl RObject for VectorPathCommandCubicBezierCurve {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "vectorPathCommandCubicBezierCurve" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDVectorPathCommand for VectorPathCommandCubicBezierCurve {}



impl VectorPathCommandCubicBezierCurve {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDVectorPathCommandCubicBezierCurveBuilder {
    let mut inner = VectorPathCommandCubicBezierCurve::default();
    inner.td_name = "vectorPathCommandCubicBezierCurve".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDVectorPathCommandCubicBezierCurveBuilder { inner }
  }

  pub fn start_control_point(&self) -> &Point { &self.start_control_point }

  pub fn end_control_point(&self) -> &Point { &self.end_control_point }

  pub fn end_point(&self) -> &Point { &self.end_point }

}

#[doc(hidden)]
pub struct RTDVectorPathCommandCubicBezierCurveBuilder {
  inner: VectorPathCommandCubicBezierCurve
}

impl RTDVectorPathCommandCubicBezierCurveBuilder {
  pub fn build(&self) -> VectorPathCommandCubicBezierCurve { self.inner.clone() }

   
  pub fn start_control_point<T: AsRef<Point>>(&mut self, start_control_point: T) -> &mut Self {
    self.inner.start_control_point = start_control_point.as_ref().clone();
    self
  }

   
  pub fn end_control_point<T: AsRef<Point>>(&mut self, end_control_point: T) -> &mut Self {
    self.inner.end_control_point = end_control_point.as_ref().clone();
    self
  }

   
  pub fn end_point<T: AsRef<Point>>(&mut self, end_point: T) -> &mut Self {
    self.inner.end_point = end_point.as_ref().clone();
    self
  }

}

impl AsRef<VectorPathCommandCubicBezierCurve> for VectorPathCommandCubicBezierCurve {
  fn as_ref(&self) -> &VectorPathCommandCubicBezierCurve { self }
}

impl AsRef<VectorPathCommandCubicBezierCurve> for RTDVectorPathCommandCubicBezierCurveBuilder {
  fn as_ref(&self) -> &VectorPathCommandCubicBezierCurve { &self.inner }
}







/// A straight line to a given point
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorPathCommandLine {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The end point of the straight line
  end_point: Point,
  
}

impl RObject for VectorPathCommandLine {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "vectorPathCommandLine" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDVectorPathCommand for VectorPathCommandLine {}



impl VectorPathCommandLine {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDVectorPathCommandLineBuilder {
    let mut inner = VectorPathCommandLine::default();
    inner.td_name = "vectorPathCommandLine".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDVectorPathCommandLineBuilder { inner }
  }

  pub fn end_point(&self) -> &Point { &self.end_point }

}

#[doc(hidden)]
pub struct RTDVectorPathCommandLineBuilder {
  inner: VectorPathCommandLine
}

impl RTDVectorPathCommandLineBuilder {
  pub fn build(&self) -> VectorPathCommandLine { self.inner.clone() }

   
  pub fn end_point<T: AsRef<Point>>(&mut self, end_point: T) -> &mut Self {
    self.inner.end_point = end_point.as_ref().clone();
    self
  }

}

impl AsRef<VectorPathCommandLine> for VectorPathCommandLine {
  fn as_ref(&self) -> &VectorPathCommandLine { self }
}

impl AsRef<VectorPathCommandLine> for RTDVectorPathCommandLineBuilder {
  fn as_ref(&self) -> &VectorPathCommandLine { &self.inner }
}



