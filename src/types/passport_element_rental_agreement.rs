
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's rental agreement. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementRentalAgreement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementRentalAgreement
  /// Rental agreement.
  rental_agreement: Option<PersonalDocument>,
  
}



impl Object for PassportElementRentalAgreement {}
impl RObject for PassportElementRentalAgreement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementRentalAgreement" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementRentalAgreement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementRentalAgreement {}


impl PassportElementRentalAgreement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementRentalAgreement".to_string(),
      rental_agreement: None,
      
    }
  }
  
  pub fn rental_agreement(&self) -> Option<PersonalDocument> { self.rental_agreement.clone() }
  #[doc(hidden)] pub fn _set_rental_agreement(&mut self, rental_agreement: PersonalDocument) -> &mut Self { self.rental_agreement = Some(rental_agreement); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



