
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's bank statement. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementBankStatement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementBankStatement
  /// Bank statement.
  bank_statement: Option<PersonalDocument>,
  
}



impl Object for PassportElementBankStatement {}
impl RObject for PassportElementBankStatement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementBankStatement" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementBankStatement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementBankStatement {}


impl PassportElementBankStatement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementBankStatement".to_string(),
      bank_statement: None,
      
    }
  }
  
  pub fn bank_statement(&self) -> Option<PersonalDocument> { self.bank_statement.clone() }
  #[doc(hidden)] pub fn _set_bank_statement(&mut self, bank_statement: PersonalDocument) -> &mut Self { self.bank_statement = Some(bank_statement); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



