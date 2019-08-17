
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets new log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Serialize, Deserialize)]
pub struct SetLogStream {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setLogStream
  /// New log stream.
  log_stream: Option<Box<LogStream>>,
  
}


impl Clone for SetLogStream {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetLogStream {}
impl RObject for SetLogStream {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setLogStream" }
  fn td_type(&self) -> RTDType { RTDType::SetLogStream }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetLogStream {}


impl SetLogStream {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setLogStream".to_string(),
      log_stream: None,
      
    }
  }
  
  pub fn log_stream(&self) -> Option<Box<LogStream>> { self.log_stream.clone() }
  #[doc(hidden)] pub fn _set_log_stream(&mut self, log_stream: Box<LogStream>) -> &mut Self { self.log_stream = Some(log_stream); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



