
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a date according to the Gregorian calendar. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Date {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // date
  /// Day of the month, 1-31.
  day: Option<i32>,
  /// Month, 1-12.
  month: Option<i32>,
  /// Year, 1-9999.
  year: Option<i32>,
  
}



impl Object for Date {}
impl RObject for Date {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "date" }
  fn td_type(&self) -> RTDType { RTDType::Date }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Date {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "date".to_string(),
      day: None,
      month: None,
      year: None,
      
    }
  }
  
  pub fn day(&self) -> Option<i32> { self.day.clone() }
  #[doc(hidden)] pub fn _set_day(&mut self, day: i32) -> &mut Self { self.day = Some(day); self }
  
  pub fn month(&self) -> Option<i32> { self.month.clone() }
  #[doc(hidden)] pub fn _set_month(&mut self, month: i32) -> &mut Self { self.month = Some(month); self }
  
  pub fn year(&self) -> Option<i32> { self.year.clone() }
  #[doc(hidden)] pub fn _set_year(&mut self, year: i32) -> &mut Self { self.year = Some(year); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



