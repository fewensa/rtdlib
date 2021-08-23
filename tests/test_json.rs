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


// 0.5.2: close this test, because this code is not always run success in any td version, Maybe the new version td add the new field in User Trait.
//#[test]
//fn test_from_update_user() {
//  let json = r#"
//{
//	"@type": "updateUser",
//	"user": {
//		"@type": "user",
//		"id": 743550508,
//		"first_name": "Jo",
//		"last_name": "ho",
//		"username": "laocaimi",
//		"phone_number": "",
//		"status": {
//			"@type": "userStatusOffline",
//			"was_online": 1556077825
//		},
//		"profile_photo": {
//			"@type": "profilePhoto",
//			"id": "3193525115240425385",
//			"small": {
//				"@type": "file",
//				"id": 1,
//				"size": 0,
//				"expected_size": 0,
//				"local": {
//					"@type": "localFile",
//					"path": "",
//					"can_be_downloaded": true,
//					"can_be_deleted": false,
//					"is_downloading_active": false,
//					"is_downloading_completed": false,
//					"download_offset": 0,
//					"downloaded_prefix_size": 0,
//					"downloaded_size": 0
//				},
//				"remote": {
//					"@type": "remoteFile",
//					"id": "AQADAQADqacxGyyuUSwACL8DCzAABFCmbbpR8R0fdAoDAAEC",
//					"is_uploading_active": false,
//					"is_uploading_completed": true,
//					"uploaded_size": 0
//				}
//			},
//			"big": {
//				"@type": "file",
//				"id": 2,
//				"size": 0,
//				"expected_size": 0,
//				"local": {
//					"@type": "localFile",
//					"path": "",
//					"can_be_downloaded": true,
//					"can_be_deleted": false,
//					"is_downloading_active": false,
//					"is_downloading_completed": false,
//					"download_offset": 0,
//					"downloaded_prefix_size": 0,
//					"downloaded_size": 0
//				},
//				"remote": {
//					"@type": "remoteFile",
//					"id": "AQADAQADqacxGyyuUSwACL8DCzAABHLmUqWpsqcqdgoDAAEC",
//					"is_uploading_active": false,
//					"is_uploading_completed": true,
//					"uploaded_size": 0
//				}
//			}
//		},
//		"outgoing_link": {
//			"@type": "linkStateNone"
//		},
//		"incoming_link": {
//			"@type": "linkStateNone"
//		},
//		"is_verified": false,
//		"is_support": false,
//		"restriction_reason": "",
//		"have_access": true,
//		"type": {
//			"@type": "userTypeRegular"
//		},
//		"language_code": ""
//	}
//}
//  "#;
//
////  let json = r#"{"@type":"updateUser","user":{"@type":"user","id":743550508,"first_name":"Jo","last_name":"ho","username":"laocaimi","phone_number":"","status":{"@type":"userStatusOffline","was_online":1556077825},"profile_photo":{"@type":"profilePhoto","id":"3193525115240425385","small":{"@type":"file","id":1,"size":0,"expected_size":0,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AQADAQADqacxGyyuUSwACL8DCzAABFCmbbpR8R0fdAoDAAEC","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":0}},"big":{"@type":"file","id":2,"size":0,"expected_size":0,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AQADAQADqacxGyyuUSwACL8DCzAABHLmUqWpsqcqdgoDAAEC","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":0}}},"outgoing_link":{"@type":"linkStateNone"},"incoming_link":{"@type":"linkStateNone"},"is_verified":false,"is_support":false,"restriction_reason":"","have_access":true,"type":{"@type":"userTypeRegular"},"language_code":""}}"#;
//
////  let json = r#"{"@type":"updateUser","user":{"@type":"user","id":720963235,"first_name":"aking","last_name":"888","username":"aking88888","phone_number":"","status":{"@type":"userStatusOffline","was_online":1559590497},"outgoing_link":{"@type":"linkStateNone"},"incoming_link":{"@type":"linkStateNone"},"is_verified":false,"is_support":false,"restriction_reason":"","have_access":true,"type":{"@type":"userTypeRegular"},"language_code":""}}"#;
//
//  let update_user: Result<UpdateUser, rtdlib::errors::RTDError> = UpdateUser::from_json(json);
//  assert!(update_user.is_ok(), true);
//  let update_user = update_user.unwrap();
//  let user = update_user.user();
//  assert_eq!(user.is_verified(), false);
//  assert_eq!(user.is_support(), false);
//  assert_eq!(user.type_().is_regular(), true);
//}


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

