
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains information about a Telegram Passport element to be saved. 
#[typetag::serde(tag = "@struct")]
pub trait InputPassportElement: Object + RObject + Debug {}






impl InputPassportElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<InputPassportElement> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDInputPassportElementType {
  InputPassportElementAddress,
  InputPassportElementBankStatement,
  InputPassportElementDriverLicense,
  InputPassportElementEmailAddress,
  InputPassportElementIdentityCard,
  InputPassportElementInternalPassport,
  InputPassportElementPassport,
  InputPassportElementPassportRegistration,
  InputPassportElementPersonalDetails,
  InputPassportElementPhoneNumber,
  InputPassportElementRentalAgreement,
  InputPassportElementTemporaryRegistration,
  InputPassportElementUtilityBill,
  
}
impl RTDInputPassportElementType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDInputPassportElementType)(text.as_ref()) }
}



