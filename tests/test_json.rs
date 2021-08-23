use rtdlib::types::*;

#[test]
fn test_authorization_state() {
  let json = r#"{"@type":"updateAuthorizationState","@extra":null,"authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@extra":null}}"#;
  let state: UpdateAuthorizationState = serde_json::from_str(&json[..]).expect("Json fail");
  assert_eq!("updateAuthorizationState", state.td_name());
  let rjson = state.to_json();
  assert!(rjson.is_ok());
  assert_eq!(json, rjson.unwrap());
}


#[test]
fn test_builder_set_tdlib_parameters() {
  let set_tdlib_paramters = SetTdlibParameters::builder()
    .parameters(
      TdlibParameters::builder()
        .use_test_dc(false)
        .database_directory("/tmp/td")
        .files_directory("/tmp/td")
        .use_file_database(false)
        .api_id(123)
        .api_hash("abc")
        .build()
    ).build();
  println!("{}", set_tdlib_paramters.to_json().unwrap())
}

#[test]
fn test_json_td_type() {

  let json_update_authorization_state = r#"
  {"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}
  "#;

  let (td_type, extra) = crate::detect_td_type_and_extra(json_update_authorization_state);
  assert_eq!(td_type, Some("updateAuthorizationState".to_string()));
  assert_eq!(extra, None);

  let json_update_option = r#"
  {"@type":"updateOption","name":"version","value":{"@type":"optionValueString","value":"1.6.4"}}
  "#;
  let (td_type, extra) = crate::detect_td_type_and_extra(json_update_option);
  assert_eq!(td_type, Some("updateOption".to_string()));
  assert_eq!(extra, None);

  let json_error = r#"
  {"@type":"error","code":401,"message":"Initialization parameters are needed: call setTdlibParameters first","@extra":"b51afb43-ea2a-45be-afa5-3957482206b3"}
  "#;
  let (td_type, extra) = crate::detect_td_type_and_extra(json_error);
  assert_eq!(td_type, Some("error".to_string()));
  assert_eq!(extra, Some("b51afb43-ea2a-45be-afa5-3957482206b3".to_string()));
  let error: Error = serde_json::from_str(&json_error[..]).expect("Json fail");
  let td_type = TdType::Error(error);
  println!("{:?}", td_type);



}

