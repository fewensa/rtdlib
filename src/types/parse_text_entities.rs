
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Parses Bold, Italic, Code, Pre, PreCode and TextUrl entities contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseTextEntities {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // parseTextEntities
  /// The text which should be parsed.
  text: Option<String>,
  /// Text parse mode.
  parse_mode: Option<Box<TextParseMode>>,
  
}


impl Clone for ParseTextEntities {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for ParseTextEntities {}
impl RObject for ParseTextEntities {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "parseTextEntities" }
  fn td_type(&self) -> RTDType { RTDType::ParseTextEntities }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ParseTextEntities {}


impl ParseTextEntities {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "parseTextEntities".to_string(),
      text: None,
      parse_mode: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn parse_mode(&self) -> Option<Box<TextParseMode>> { self.parse_mode.clone() }
  #[doc(hidden)] pub fn _set_parse_mode(&mut self, parse_mode: Box<TextParseMode>) -> &mut Self { self.parse_mode = Some(parse_mode); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



