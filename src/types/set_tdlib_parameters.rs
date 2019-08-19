
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the parameters for TDLib initialization. Works only when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTdlibParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setTdlibParameters
  /// Parameters.
  parameters: Option<TdlibParameters>,
  
}



impl Object for SetTdlibParameters {}
impl RObject for SetTdlibParameters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setTdlibParameters" }
  fn td_type(&self) -> RTDType { RTDType::SetTdlibParameters }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetTdlibParameters {}


impl SetTdlibParameters {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setTdlibParameters".to_string(),
      parameters: None,
      
    }
  }
  
  pub fn parameters(&self) -> Option<TdlibParameters> { self.parameters.clone() }
  #[doc(hidden)] pub fn _set_parameters(&mut self, parameters: TdlibParameters) -> &mut Self { self.parameters = Some(parameters); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



