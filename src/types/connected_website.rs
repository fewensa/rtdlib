
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about one website the current user is logged in with Telegram. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedWebsite {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // connectedWebsite
  /// Website identifier.
  id: Option<i64>,
  /// The domain name of the website.
  domain_name: Option<String>,
  /// User identifier of a bot linked with the website.
  bot_user_id: Option<i32>,
  /// The version of a browser used to log in.
  browser: Option<String>,
  /// Operating system the browser is running on.
  platform: Option<String>,
  /// Point in time (Unix timestamp) when the user was logged in.
  log_in_date: Option<i32>,
  /// Point in time (Unix timestamp) when obtained authorization was last used.
  last_active_date: Option<i32>,
  /// IP address from which the user was logged in, in human-readable format.
  ip: Option<String>,
  /// Human-readable description of a country and a region, from which the user was logged in, based on the IP address.
  location: Option<String>,
  
}



impl Object for ConnectedWebsite {}
impl RObject for ConnectedWebsite {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectedWebsite" }
  fn td_type(&self) -> RTDType { RTDType::ConnectedWebsite }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ConnectedWebsite {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "connectedWebsite".to_string(),
      id: None,
      domain_name: None,
      bot_user_id: None,
      browser: None,
      platform: None,
      log_in_date: None,
      last_active_date: None,
      ip: None,
      location: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn domain_name(&self) -> Option<String> { self.domain_name.clone() }
  #[doc(hidden)] pub fn _set_domain_name(&mut self, domain_name: String) -> &mut Self { self.domain_name = Some(domain_name); self }
  
  pub fn bot_user_id(&self) -> Option<i32> { self.bot_user_id.clone() }
  #[doc(hidden)] pub fn _set_bot_user_id(&mut self, bot_user_id: i32) -> &mut Self { self.bot_user_id = Some(bot_user_id); self }
  
  pub fn browser(&self) -> Option<String> { self.browser.clone() }
  #[doc(hidden)] pub fn _set_browser(&mut self, browser: String) -> &mut Self { self.browser = Some(browser); self }
  
  pub fn platform(&self) -> Option<String> { self.platform.clone() }
  #[doc(hidden)] pub fn _set_platform(&mut self, platform: String) -> &mut Self { self.platform = Some(platform); self }
  
  pub fn log_in_date(&self) -> Option<i32> { self.log_in_date.clone() }
  #[doc(hidden)] pub fn _set_log_in_date(&mut self, log_in_date: i32) -> &mut Self { self.log_in_date = Some(log_in_date); self }
  
  pub fn last_active_date(&self) -> Option<i32> { self.last_active_date.clone() }
  #[doc(hidden)] pub fn _set_last_active_date(&mut self, last_active_date: i32) -> &mut Self { self.last_active_date = Some(last_active_date); self }
  
  pub fn ip(&self) -> Option<String> { self.ip.clone() }
  #[doc(hidden)] pub fn _set_ip(&mut self, ip: String) -> &mut Self { self.ip = Some(ip); self }
  
  pub fn location(&self) -> Option<String> { self.location.clone() }
  #[doc(hidden)] pub fn _set_location(&mut self, location: String) -> &mut Self { self.location = Some(location); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



