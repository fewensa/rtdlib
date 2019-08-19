
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A Telegram Passport element to be saved containing the user's identity card. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPassportElementIdentityCard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputPassportElementIdentityCard
  /// The identity card to be saved.
  identity_card: Option<InputIdentityDocument>,
  
}



impl Object for InputPassportElementIdentityCard {}
impl RObject for InputPassportElementIdentityCard {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementIdentityCard" }
  fn td_type(&self) -> RTDType { RTDType::InputPassportElementIdentityCard }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputPassportElement for InputPassportElementIdentityCard {}


impl InputPassportElementIdentityCard {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputPassportElementIdentityCard".to_string(),
      identity_card: None,
      
    }
  }
  
  pub fn identity_card(&self) -> Option<InputIdentityDocument> { self.identity_card.clone() }
  #[doc(hidden)] pub fn _set_identity_card(&mut self, identity_card: InputIdentityDocument) -> &mut Self { self.identity_card = Some(identity_card); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



