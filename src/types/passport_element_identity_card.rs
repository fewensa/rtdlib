
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element containing the user's identity card. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportElementIdentityCard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportElementIdentityCard
  /// Identity card.
  identity_card: Option<IdentityDocument>,
  
}



impl Object for PassportElementIdentityCard {}
impl RObject for PassportElementIdentityCard {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementIdentityCard" }
  fn td_type(&self) -> RTDType { RTDType::PassportElementIdentityCard }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PassportElement for PassportElementIdentityCard {}


impl PassportElementIdentityCard {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportElementIdentityCard".to_string(),
      identity_card: None,
      
    }
  }
  
  pub fn identity_card(&self) -> Option<IdentityDocument> { self.identity_card.clone() }
  #[doc(hidden)] pub fn _set_identity_card(&mut self, identity_card: IdentityDocument) -> &mut Self { self.identity_card = Some(identity_card); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



