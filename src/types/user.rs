
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a user. 
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // user
  /// User identifier.
  id: Option<i32>,
  /// First name of the user.
  first_name: Option<String>,
  /// Last name of the user.
  last_name: Option<String>,
  /// Username of the user.
  username: Option<String>,
  /// Phone number of the user.
  phone_number: Option<String>,
  /// Current online status of the user.
  status: Option<Box<UserStatus>>,
  /// Profile photo of the user; may be null.
  profile_photo: Option<ProfilePhoto>,
  /// Relationship from the current user to the other user.
  outgoing_link: Option<Box<LinkState>>,
  /// Relationship from the other user to the current user.
  incoming_link: Option<Box<LinkState>>,
  /// True, if the user is verified.
  is_verified: Option<bool>,
  /// True, if the user is Telegram support account.
  is_support: Option<bool>,
  /// If non-empty, it contains the reason why access to this user must be restricted. The format of the string is "{type}: {description}". {type} contains the type of the restriction and at least one of the suffixes "-all", "-ios", "-android", or "-wp", which describe the platforms on which access should be restricted. (For example, "terms-ios-android". {description} contains a human-readable description of the restriction, which can be shown to the user.)
  restriction_reason: Option<String>,
  /// If false, the user is inaccessible, and the only information known about the user is inside this class. It can't be passed to any method except GetUser.
  have_access: Option<bool>,
  /// Type of the user.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<UserType>>,
  /// IETF language tag of the user's language; only available to bots.
  language_code: Option<String>,
  
}


impl Clone for User {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for User {}
impl RObject for User {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "user" }
  fn td_type(&self) -> RTDType { RTDType::User }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl User {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "user".to_string(),
      id: None,
      first_name: None,
      last_name: None,
      username: None,
      phone_number: None,
      status: None,
      profile_photo: None,
      outgoing_link: None,
      incoming_link: None,
      is_verified: None,
      is_support: None,
      restriction_reason: None,
      have_access: None,
      type_: None,
      language_code: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn first_name(&self) -> Option<String> { self.first_name.clone() }
  #[doc(hidden)] pub fn _set_first_name(&mut self, first_name: String) -> &mut Self { self.first_name = Some(first_name); self }
  
  pub fn last_name(&self) -> Option<String> { self.last_name.clone() }
  #[doc(hidden)] pub fn _set_last_name(&mut self, last_name: String) -> &mut Self { self.last_name = Some(last_name); self }
  
  pub fn username(&self) -> Option<String> { self.username.clone() }
  #[doc(hidden)] pub fn _set_username(&mut self, username: String) -> &mut Self { self.username = Some(username); self }
  
  pub fn phone_number(&self) -> Option<String> { self.phone_number.clone() }
  #[doc(hidden)] pub fn _set_phone_number(&mut self, phone_number: String) -> &mut Self { self.phone_number = Some(phone_number); self }
  
  pub fn status(&self) -> Option<Box<UserStatus>> { self.status.clone() }
  #[doc(hidden)] pub fn _set_status(&mut self, status: Box<UserStatus>) -> &mut Self { self.status = Some(status); self }
  
  pub fn profile_photo(&self) -> Option<ProfilePhoto> { self.profile_photo.clone() }
  #[doc(hidden)] pub fn _set_profile_photo(&mut self, profile_photo: ProfilePhoto) -> &mut Self { self.profile_photo = Some(profile_photo); self }
  
  pub fn outgoing_link(&self) -> Option<Box<LinkState>> { self.outgoing_link.clone() }
  #[doc(hidden)] pub fn _set_outgoing_link(&mut self, outgoing_link: Box<LinkState>) -> &mut Self { self.outgoing_link = Some(outgoing_link); self }
  
  pub fn incoming_link(&self) -> Option<Box<LinkState>> { self.incoming_link.clone() }
  #[doc(hidden)] pub fn _set_incoming_link(&mut self, incoming_link: Box<LinkState>) -> &mut Self { self.incoming_link = Some(incoming_link); self }
  
  pub fn is_verified(&self) -> Option<bool> { self.is_verified.clone() }
  #[doc(hidden)] pub fn _set_is_verified(&mut self, is_verified: bool) -> &mut Self { self.is_verified = Some(is_verified); self }
  
  pub fn is_support(&self) -> Option<bool> { self.is_support.clone() }
  #[doc(hidden)] pub fn _set_is_support(&mut self, is_support: bool) -> &mut Self { self.is_support = Some(is_support); self }
  
  pub fn restriction_reason(&self) -> Option<String> { self.restriction_reason.clone() }
  #[doc(hidden)] pub fn _set_restriction_reason(&mut self, restriction_reason: String) -> &mut Self { self.restriction_reason = Some(restriction_reason); self }
  
  pub fn have_access(&self) -> Option<bool> { self.have_access.clone() }
  #[doc(hidden)] pub fn _set_have_access(&mut self, have_access: bool) -> &mut Self { self.have_access = Some(have_access); self }
  
  pub fn type_(&self) -> Option<Box<UserType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<UserType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn language_code(&self) -> Option<String> { self.language_code.clone() }
  #[doc(hidden)] pub fn _set_language_code(&mut self, language_code: String) -> &mut Self { self.language_code = Some(language_code); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



