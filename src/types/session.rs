
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about one session in a Telegram application used by the current user. Sessions should be shown to the user in the returned order. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // session
  /// Session identifier.
  id: Option<i64>,
  /// True, if this session is the current session.
  is_current: Option<bool>,
  /// True, if a password is needed to complete authorization of the session.
  is_password_pending: Option<bool>,
  /// Telegram API identifier, as provided by the application.
  api_id: Option<i32>,
  /// Name of the application, as provided by the application.
  application_name: Option<String>,
  /// The version of the application, as provided by the application.
  application_version: Option<String>,
  /// True, if the application is an official application or uses the api_id of an official application.
  is_official_application: Option<bool>,
  /// Model of the device the application has been run or is running on, as provided by the application.
  device_model: Option<String>,
  /// Operating system the application has been run or is running on, as provided by the application.
  platform: Option<String>,
  /// Version of the operating system the application has been run or is running on, as provided by the application.
  system_version: Option<String>,
  /// Point in time (Unix timestamp) when the user has logged in.
  log_in_date: Option<i32>,
  /// Point in time (Unix timestamp) when the session was last used.
  last_active_date: Option<i32>,
  /// IP address from which the session was created, in human-readable format.
  ip: Option<String>,
  /// A two-letter country code for the country from which the session was created, based on the IP address.
  country: Option<String>,
  /// Region code from which the session was created, based on the IP address.
  region: Option<String>,
  
}



impl Object for Session {}
impl RObject for Session {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "session" }
  fn td_type(&self) -> RTDType { RTDType::Session }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Session {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "session".to_string(),
      id: None,
      is_current: None,
      is_password_pending: None,
      api_id: None,
      application_name: None,
      application_version: None,
      is_official_application: None,
      device_model: None,
      platform: None,
      system_version: None,
      log_in_date: None,
      last_active_date: None,
      ip: None,
      country: None,
      region: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn is_current(&self) -> Option<bool> { self.is_current.clone() }
  #[doc(hidden)] pub fn _set_is_current(&mut self, is_current: bool) -> &mut Self { self.is_current = Some(is_current); self }
  
  pub fn is_password_pending(&self) -> Option<bool> { self.is_password_pending.clone() }
  #[doc(hidden)] pub fn _set_is_password_pending(&mut self, is_password_pending: bool) -> &mut Self { self.is_password_pending = Some(is_password_pending); self }
  
  pub fn api_id(&self) -> Option<i32> { self.api_id.clone() }
  #[doc(hidden)] pub fn _set_api_id(&mut self, api_id: i32) -> &mut Self { self.api_id = Some(api_id); self }
  
  pub fn application_name(&self) -> Option<String> { self.application_name.clone() }
  #[doc(hidden)] pub fn _set_application_name(&mut self, application_name: String) -> &mut Self { self.application_name = Some(application_name); self }
  
  pub fn application_version(&self) -> Option<String> { self.application_version.clone() }
  #[doc(hidden)] pub fn _set_application_version(&mut self, application_version: String) -> &mut Self { self.application_version = Some(application_version); self }
  
  pub fn is_official_application(&self) -> Option<bool> { self.is_official_application.clone() }
  #[doc(hidden)] pub fn _set_is_official_application(&mut self, is_official_application: bool) -> &mut Self { self.is_official_application = Some(is_official_application); self }
  
  pub fn device_model(&self) -> Option<String> { self.device_model.clone() }
  #[doc(hidden)] pub fn _set_device_model(&mut self, device_model: String) -> &mut Self { self.device_model = Some(device_model); self }
  
  pub fn platform(&self) -> Option<String> { self.platform.clone() }
  #[doc(hidden)] pub fn _set_platform(&mut self, platform: String) -> &mut Self { self.platform = Some(platform); self }
  
  pub fn system_version(&self) -> Option<String> { self.system_version.clone() }
  #[doc(hidden)] pub fn _set_system_version(&mut self, system_version: String) -> &mut Self { self.system_version = Some(system_version); self }
  
  pub fn log_in_date(&self) -> Option<i32> { self.log_in_date.clone() }
  #[doc(hidden)] pub fn _set_log_in_date(&mut self, log_in_date: i32) -> &mut Self { self.log_in_date = Some(log_in_date); self }
  
  pub fn last_active_date(&self) -> Option<i32> { self.last_active_date.clone() }
  #[doc(hidden)] pub fn _set_last_active_date(&mut self, last_active_date: i32) -> &mut Self { self.last_active_date = Some(last_active_date); self }
  
  pub fn ip(&self) -> Option<String> { self.ip.clone() }
  #[doc(hidden)] pub fn _set_ip(&mut self, ip: String) -> &mut Self { self.ip = Some(ip); self }
  
  pub fn country(&self) -> Option<String> { self.country.clone() }
  #[doc(hidden)] pub fn _set_country(&mut self, country: String) -> &mut Self { self.country = Some(country); self }
  
  pub fn region(&self) -> Option<String> { self.region.clone() }
  #[doc(hidden)] pub fn _set_region(&mut self, region: String) -> &mut Self { self.region = Some(region); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



