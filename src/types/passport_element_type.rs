
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Contains the type of a Telegram Passport element. 
#[typetag::serde(tag = "@struct")]
pub trait PassportElementType: Object + RObject + Debug {}






impl PassportElementType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<PassportElementType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDPassportElementTypeType {
  PassportElementTypeAddress,
  PassportElementTypeBankStatement,
  PassportElementTypeDriverLicense,
  PassportElementTypeEmailAddress,
  PassportElementTypeIdentityCard,
  PassportElementTypeInternalPassport,
  PassportElementTypePassport,
  PassportElementTypePassportRegistration,
  PassportElementTypePersonalDetails,
  PassportElementTypePhoneNumber,
  PassportElementTypeRentalAgreement,
  PassportElementTypeTemporaryRegistration,
  PassportElementTypeUtilityBill,
  
}
impl RTDPassportElementTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDPassportElementTypeType)(text.as_ref()) }
}



