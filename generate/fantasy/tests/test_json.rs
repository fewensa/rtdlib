#[macro_use]
extern crate serde_derive;

use serde::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthorizationState {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  authorization_state: AuthorizationState,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AuthorizationState {
  Closed             (AuthorizationStateClosed               ),
  Closing            (AuthorizationStateClosing              ),
  LoggingOut         (AuthorizationStateLoggingOut           ),
  Ready              (AuthorizationStateReady                ),
  WaitCode           (AuthorizationStateWaitCode             ),
  WaitEncryptionKey  (AuthorizationStateWaitEncryptionKey    ),
  WaitPassword       (AuthorizationStateWaitPassword         ),
  WaitPhoneNumber    (AuthorizationStateWaitPhoneNumber      ),
  WaitTdlibParameters(AuthorizationStateWaitTdlibParameters  ),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateClosed {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateClosing {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateLoggingOut {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateReady {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitCode {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// True, if the user is already registered.
  is_registered: Option<bool>,
//  /// Telegram terms of service, which should be accepted before user can continue registration; may be null.
//  terms_of_service: Option<TermsOfService>,
  /// Information about the authorization code that was sent.
  code_info: Option<AuthenticationCodeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitEncryptionKey {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// True, if the database is currently encrypted.
  is_encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPassword {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Hint for the password; may be empty.
  password_hint: Option<String>,
  /// True if a recovery email address has been set up.
  has_recovery_email_address: Option<bool>,
  /// Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent.
  recovery_email_address_pattern: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPhoneNumber {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationStateWaitTdlibParameters {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCodeInfo {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// A phone number that is being authenticated.
  phone_number: String,
  /// Describes the way the code was sent to the user.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: AuthenticationCodeType,
  /// Describes the way the next code will be sent to the user; may be null.
  next_type: Option<AuthenticationCodeType>,
  /// Timeout before the code should be re-sent, in seconds.
  timeout: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationCodeType {
  TypeCall             (AuthenticationCodeTypeCall            ),
  TypeFlashCall        (AuthenticationCodeTypeFlashCall       ),
  TypeSms              (AuthenticationCodeTypeSms             ),
  TypeTelegramMessage  (AuthenticationCodeTypeTelegramMessage ),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeCall {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Length of the code.
  length: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFlashCall {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Pattern of the phone number from which the call will be made.
  pattern: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeSms {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Length of the code.
  length: i32,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeTelegramMessage {
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Length of the code.
  length: i32,
}


macro_rules! rtd_enum_deserialize {
  ($type_name:ident, $(($td_name:ident, $enum_item:ident));*;) => {
    // example json
    // {"@type":"authorizationStateWaitEncryptionKey","is_encrypted":false}
    |deserializer: D| -> Result<$type_name, D::Error> {
      let rtd_trait_value: serde_json::Value = Deserialize::deserialize(deserializer)?;
      // the `rtd_trait_value` variable type is &serde_json::Value, tdlib trait will return a object, convert this type to object `&Map<String, Value>`
      let rtd_trait_map = match rtd_trait_value.as_object() {
        Some(map) => map,
        None => return Err(D::Error::unknown_field(stringify!($type_name), &[stringify!("{} is not the correct type", $type_name)])) // &format!("{} is not the correct type", stringify!($field))[..]
      };
      // get `@type` value, detect specific types
      let rtd_trait_type = match rtd_trait_map.get("@type") {
        // the `t` variable type is `serde_json::Value`, convert `t` to str
        Some(t) => match t.as_str() {
          Some(s) => s,
          None => return Err(D::Error::unknown_field(stringify!("{} -> @type", $field), &[stringify!("{} -> @type is not the correct type", $type_name)])) // &format!("{} -> @type is not the correct type", stringify!($field))[..]
        },
        None => return Err(D::Error::missing_field(stringify!("{} -> @type", $field)))
      };

      let obj = match rtd_trait_type {
        $(
          stringify!($td_name) => $type_name::$enum_item(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(e) => return Err(D::Error::unknown_field(stringify!("{} can't deserialize to {}::{}", $td_name, $type_name, $enum_item, e), &[stringify!("{:?}", e)]))
          }),
        )*
        _ => return Err(D::Error::missing_field(stringify!($field)))
      };
      Ok(obj)
    }
  }
}

impl<'de> Deserialize<'de> for AuthorizationState {
  fn deserialize<D>(deserializer: D) -> Result<AuthorizationState, D::Error> where D: Deserializer<'de> {

    use serde::de::Error;

    rtd_enum_deserialize!(
      AuthorizationState,
      (authorizationStateWaitTdlibParameters, WaitTdlibParameters);
      (authorizationStateWaitEncryptionKey  , WaitEncryptionKey  );
    )(deserializer)

    // test code

////    Err(D::Error::unknown_field("authorization_state", &["field fail"]))
//
//    let authorization_state_value: serde_json::Value = Deserialize::deserialize(deserializer)?;
//    let authorization_state = match authorization_state_value.as_object() {
//      Some(t) => t,
//      None => return Err(D::Error::unknown_field("authorization_state", &["field fail"]))
//    };
//    let aut_type = match authorization_state.get("@type") {
//      Some(t) => t,
//      None => return Err(D::Error::missing_field("authorization_state -> @type"))
//    };
//    let aut_type = match aut_type.as_str() {
//      Some(t) => t,
//      None => return Err(D::Error::unknown_field("authorization_state", &["field fail"]))
//    };
//    let uas = match aut_type {
//      "authorizationStateWaitTdlibParameters" => {
//        AuthorizationState::WaitTdlibParameters(match serde_json::from_value(authorization_state_value.clone()) {
//          Ok(t) => t,
//          Err(e) => return Err(D::Error::unknown_field("authorization_state", &["field fail"]))
//        })
//      },
//      "authorizationStateWaitEncryptionKey" => {
//        AuthorizationState::WaitEncryptionKey(match serde_json::from_value(authorization_state_value.clone()) {
//          Ok(t) => t,
//          Err(e) => return Err(D::Error::unknown_field("authorization_state", &["field fail"]))
//        })
//      },
//      _ => return Err(D::Error::missing_field("authorization_state"))
//    };
//    Ok(uas)
  }
}



#[test]
fn test_authorization_state() {
  let json = r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitEncryptionKey","is_encrypted":false}}"#;
  let update_authorization_stat: UpdateAuthorizationState = serde_json::from_str(json).unwrap();
  let aut = serde_json::to_string(&update_authorization_stat);
  println!("{:?}", aut);
  assert!(aut.is_ok(), true);
  assert_eq!(aut.unwrap(), json);
}
