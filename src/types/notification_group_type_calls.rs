
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A group containing notifications of type 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationGroupTypeCalls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationGroupTypeCalls
  
}



impl Object for NotificationGroupTypeCalls {}
impl RObject for NotificationGroupTypeCalls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationGroupTypeCalls" }
  fn td_type(&self) -> RTDType { RTDType::NotificationGroupTypeCalls }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationGroupType for NotificationGroupTypeCalls {}


impl NotificationGroupTypeCalls {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationGroupTypeCalls".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



