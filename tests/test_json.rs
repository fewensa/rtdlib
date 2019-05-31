use rtdlib::types::*;
use rtdlib::{tdkit, types};

#[test]
fn test_authorization_state() {
  let json = r#"{"@type":"updateAuthorizationState","@struct":"UpdateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@struct":"AuthorizationStateWaitTdlibParameters"}}"#;
  let state: UpdateAuthorizationState = serde_json::from_str(&json[..]).expect("Json fail");
  assert_eq!("updateAuthorizationState", state.td_name());
}


#[test]
fn test_box_object() {
  let json = r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@struct":"AuthorizationStateWaitTdlibParameters"}}"#;
  let bobj: Option<Box<Object>> = Object::from_json(json);
  assert!(bobj.is_some());
  let obj = bobj.unwrap();
  assert_eq!(obj.td_type(), RTDType::UpdateAuthorizationState);
  println!("{}", obj.to_json());
  assert_eq!(obj.to_json(), r#"{"@type":"updateAuthorizationState","@struct":"UpdateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@struct":"AuthorizationStateWaitTdlibParameters"}}"#)
}


#[test]
fn test_boj() {
  let json =r#"{"@type":"updateOption","@struct":"UpdateOption","name":"version","value":{"@type":"optionValueString","@struct":"OptionValueString","value":"1.4.0"}}"#;
  let fjs = tdkit::fill_json_struct(json);
  println!("JSON: {}", json);
  println!("FJS:  {}", fjs);
  let bobj: Option<Box<Object>> = Object::from_json(json);
  assert!(bobj.is_some());
  let obj = bobj.unwrap();
  assert_eq!(obj.td_type(), RTDType::UpdateOption);
  println!("{}", obj.to_json());
}

