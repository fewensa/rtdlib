
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's utility bill. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementUtilityBill {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementUtilityBill
  /// Utility bill.
  utility_bill: Option<PersonalDocument>,
  
}



impl Object for PassportElementUtilityBill {}
impl RObject for PassportElementUtilityBill {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementUtilityBill" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementUtilityBill }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementUtilityBill {}


impl PassportElementUtilityBill {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementUtilityBill".to_string(),
      utility_bill: None,
      
    }
  }
  
  pub fn utility_bill(&self) -> Option<PersonalDocument> { self.utility_bill.clone() }
  #[doc(hidden)] pub fn _set_utility_bill(&mut self, utility_bill: PersonalDocument) -> &mut Self { self.utility_bill = Some(utility_bill); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



