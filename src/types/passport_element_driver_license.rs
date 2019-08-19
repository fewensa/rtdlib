
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's driver license. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementDriverLicense {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementDriverLicense
  /// Driver license.
  driver_license: Option<IdentityDocument>,
  
}



impl Object for PassportElementDriverLicense {}
impl RObject for PassportElementDriverLicense {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementDriverLicense" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementDriverLicense }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementDriverLicense {}


impl PassportElementDriverLicense {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementDriverLicense".to_string(),
      driver_license: None,
      
    }
  }
  
  pub fn driver_license(&self) -> Option<IdentityDocument> { self.driver_license.clone() }
  #[doc(hidden)] pub fn _set_driver_license(&mut self, driver_license: IdentityDocument) -> &mut Self { self.driver_license = Some(driver_license); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



