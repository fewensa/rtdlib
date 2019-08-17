
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A text message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageText
  /// Formatted text to be sent; 1-GetOption("message_text_length_max") characters. Only Bold, Italic, Code, Pre, PreCode and TextUrl entities are allowed to be specified manually.
  text: Option<FormattedText>,
  /// True, if rich web page previews for URLs in the message text should be disabled.
  disable_web_page_preview: Option<bool>,
  /// True, if a chat message draft should be deleted.
  clear_draft: Option<bool>,
  
}



impl Object for InputMessageText {}
impl RObject for InputMessageText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageText" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageText }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageText {}


impl InputMessageText {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageText".to_string(),
      text: None,
      disable_web_page_preview: None,
      clear_draft: None,
      
    }
  }
  
  pub fn text(&self) -> Option<FormattedText> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: FormattedText) -> &mut Self { self.text = Some(text); self }
  
  pub fn disable_web_page_preview(&self) -> Option<bool> { self.disable_web_page_preview.clone() }
  #[doc(hidden)] pub fn _set_disable_web_page_preview(&mut self, disable_web_page_preview: bool) -> &mut Self { self.disable_web_page_preview = Some(disable_web_page_preview); self }
  
  pub fn clear_draft(&self) -> Option<bool> { self.clear_draft.clone() }
  #[doc(hidden)] pub fn _set_clear_draft(&mut self, clear_draft: bool) -> &mut Self { self.clear_draft = Some(clear_draft); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



