
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains information about a Telegram Passport element. 
#[typetag::serde(tag = "@struct")]
pub trait PassportElement: Object + RObject + Debug {}






impl PassportElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<PassportElement> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDPassportElementType {
  PassportElementAddress,
  PassportElementBankStatement,
  PassportElementDriverLicense,
  PassportElementEmailAddress,
  PassportElementIdentityCard,
  PassportElementInternalPassport,
  PassportElementPassport,
  PassportElementPassportRegistration,
  PassportElementPersonalDetails,
  PassportElementPhoneNumber,
  PassportElementRentalAgreement,
  PassportElementTemporaryRegistration,
  PassportElementUtilityBill,
  
}
impl RTDPassportElementType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDPassportElementType)(text.as_ref()) }
}



