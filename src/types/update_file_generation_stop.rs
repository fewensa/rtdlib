
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// File generation is no longer needed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFileGenerationStop {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateFileGenerationStop
  /// Unique identifier for the generation process.
  generation_id: Option<i64>,
  
}



impl Object for UpdateFileGenerationStop {}
impl RObject for UpdateFileGenerationStop {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateFileGenerationStop" }
  fn td_type(&self) -> RTDType { RTDType::UpdateFileGenerationStop }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateFileGenerationStop {}


impl UpdateFileGenerationStop {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateFileGenerationStop".to_string(),
      generation_id: None,
      
    }
  }
  
  pub fn generation_id(&self) -> Option<i64> { self.generation_id.clone() }
  #[doc(hidden)] pub fn _set_generation_id(&mut self, generation_id: i64) -> &mut Self { self.generation_id = Some(generation_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



