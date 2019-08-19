
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's rental agreement. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementRentalAgreement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementRentalAgreement
  /// The rental agreement to be saved.
  rental_agreement: Option<InputPersonalDocument>,
  
}



impl Object for InputPassportElementRentalAgreement {}
impl RObject for InputPassportElementRentalAgreement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementRentalAgreement" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementRentalAgreement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementRentalAgreement {}


impl InputPassportElementRentalAgreement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementRentalAgreement".to_string(),
      rental_agreement: None,
      
    }
  }
  
  pub fn rental_agreement(&self) -> Option<InputPersonalDocument> { self.rental_agreement.clone() }
  #[doc(hidden)] pub fn _set_rental_agreement(&mut self, rental_agreement: InputPersonalDocument) -> &mut Self { self.rental_agreement = Some(rental_agreement); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



