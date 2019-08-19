
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's bank statement. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementBankStatement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementBankStatement
  /// The bank statement to be saved.
  bank_statement: Option<InputPersonalDocument>,
  
}



impl Object for InputPassportElementBankStatement {}
impl RObject for InputPassportElementBankStatement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementBankStatement" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementBankStatement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementBankStatement {}


impl InputPassportElementBankStatement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementBankStatement".to_string(),
      bank_statement: None,
      
    }
  }
  
  pub fn bank_statement(&self) -> Option<InputPersonalDocument> { self.bank_statement.clone() }
  #[doc(hidden)] pub fn _set_bank_statement(&mut self, bank_statement: InputPersonalDocument) -> &mut Self { self.bank_statement = Some(bank_statement); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



