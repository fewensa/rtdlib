
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains information about the payment method chosen by the user. 
#[typetag::serde(tag = "@struct")]
pub trait InputCredentials: Object + RObject + Debug {}






impl InputCredentials {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<InputCredentials> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDInputCredentialsType {
  InputCredentialsAndroidPay,
  InputCredentialsApplePay,
  InputCredentialsNew,
  InputCredentialsSaved,
  
}
impl RTDInputCredentialsType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDInputCredentialsType)(text.as_ref()) }
}



