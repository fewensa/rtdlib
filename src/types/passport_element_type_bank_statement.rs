
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's bank statement. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementTypeBankStatement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementTypeBankStatement
  
}



impl Object for PassportElementTypeBankStatement {}
impl RObject for PassportElementTypeBankStatement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementTypeBankStatement" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementTypeBankStatement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElementType for PassportElementTypeBankStatement {}


impl PassportElementTypeBankStatement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementTypeBankStatement".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



