
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's utility bill. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementUtilityBill {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementUtilityBill
  /// The utility bill to be saved.
  utility_bill: Option<InputPersonalDocument>,
  
}



impl Object for InputPassportElementUtilityBill {}
impl RObject for InputPassportElementUtilityBill {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementUtilityBill" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementUtilityBill }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementUtilityBill {}


impl InputPassportElementUtilityBill {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementUtilityBill".to_string(),
      utility_bill: None,
      
    }
  }
  
  pub fn utility_bill(&self) -> Option<InputPersonalDocument> { self.utility_bill.clone() }
  #[doc(hidden)] pub fn _set_utility_bill(&mut self, utility_bill: InputPersonalDocument) -> &mut Self { self.utility_bill = Some(utility_bill); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



