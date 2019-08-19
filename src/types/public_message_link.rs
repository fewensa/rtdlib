
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a public HTTPS link to a message in a public supergroup or channel. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicMessageLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // publicMessageLink
  /// Message link.
  link: Option<String>,
  /// HTML-code for embedding the message.
  html: Option<String>,
  
}



impl Object for PublicMessageLink {}
impl RObject for PublicMessageLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "publicMessageLink" }
  fn td_type(&self) -> RTDType { RTDType::PublicMessageLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PublicMessageLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "publicMessageLink".to_string(),
      link: None,
      html: None,
      
    }
  }
  
  pub fn link(&self) -> Option<String> { self.link.clone() }
  #[doc(hidden)] pub fn _set_link(&mut self, link: String) -> &mut Self { self.link = Some(link); self }
  
  pub fn html(&self) -> Option<String> { self.html.clone() }
  #[doc(hidden)] pub fn _set_html(&mut self, html: String) -> &mut Self { self.html = Some(html); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



