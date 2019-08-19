
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A text message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageText
  /// Text of the message.
  text: Option<FormattedText>,
  /// A preview of the web page that's mentioned in the text; may be null.
  web_page: Option<WebPage>,
  
}



impl Object for MessageText {}
impl RObject for MessageText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageText" }
  fn td_type(&self) -> RTDType { RTDType::MessageText }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageText {}


impl MessageText {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageText".to_string(),
      text: None,
      web_page: None,
      
    }
  }
  
  pub fn text(&self) -> Option<FormattedText> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: FormattedText) -> &mut Self { self.text = Some(text); self }
  
  pub fn web_page(&self) -> Option<WebPage> { self.web_page.clone() }
  #[doc(hidden)] pub fn _set_web_page(&mut self, web_page: WebPage) -> &mut Self { self.web_page = Some(web_page); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



