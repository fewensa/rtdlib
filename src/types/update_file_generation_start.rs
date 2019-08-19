
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The file generation process needs to be started by the client. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFileGenerationStart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateFileGenerationStart
  /// Unique identifier for the generation process.
  generation_id: Option<i64>,
  /// The path to a file from which a new file is generated; may be empty.
  original_path: Option<String>,
  /// The path to a file that should be created and where the new file should be generated.
  destination_path: Option<String>,
  /// String specifying the conversion applied to the original file. If conversion is "#url#" than original_path contains an HTTP/HTTPS URL of a file, which should be downloaded by the client.
  conversion: Option<String>,
  
}



impl Object for UpdateFileGenerationStart {}
impl RObject for UpdateFileGenerationStart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateFileGenerationStart" }
  fn td_type(&self) -> RTDType { RTDType::UpdateFileGenerationStart }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateFileGenerationStart {}


impl UpdateFileGenerationStart {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateFileGenerationStart".to_string(),
      generation_id: None,
      original_path: None,
      destination_path: None,
      conversion: None,
      
    }
  }
  
  pub fn generation_id(&self) -> Option<i64> { self.generation_id.clone() }
  #[doc(hidden)] pub fn _set_generation_id(&mut self, generation_id: i64) -> &mut Self { self.generation_id = Some(generation_id); self }
  
  pub fn original_path(&self) -> Option<String> { self.original_path.clone() }
  #[doc(hidden)] pub fn _set_original_path(&mut self, original_path: String) -> &mut Self { self.original_path = Some(original_path); self }
  
  pub fn destination_path(&self) -> Option<String> { self.destination_path.clone() }
  #[doc(hidden)] pub fn _set_destination_path(&mut self, destination_path: String) -> &mut Self { self.destination_path = Some(destination_path); self }
  
  pub fn conversion(&self) -> Option<String> { self.conversion.clone() }
  #[doc(hidden)] pub fn _set_conversion(&mut self, conversion: String) -> &mut Self { self.conversion = Some(conversion); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



