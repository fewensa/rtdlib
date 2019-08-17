
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A group containing notifications of type 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationGroupTypeMentions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // notificationGroupTypeMentions
  
}



impl Object for NotificationGroupTypeMentions {}
impl RObject for NotificationGroupTypeMentions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationGroupTypeMentions" }
  fn td_type(&self) -> RTDType { RTDType::NotificationGroupTypeMentions }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl NotificationGroupType for NotificationGroupTypeMentions {}


impl NotificationGroupTypeMentions {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "notificationGroupTypeMentions".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



