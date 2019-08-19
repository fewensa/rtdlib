
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A group containing notifications of type 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationGroupTypeMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationGroupTypeMessages
  
}



impl Object for NotificationGroupTypeMessages {}
impl RObject for NotificationGroupTypeMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationGroupTypeMessages" }
  fn td_type(&self) -> RTDType { RTDType::NotificationGroupTypeMessages }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationGroupType for NotificationGroupTypeMessages {}


impl NotificationGroupTypeMessages {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationGroupTypeMessages".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



