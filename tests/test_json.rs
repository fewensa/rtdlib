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



#[test]
fn test_from_update_user() {
  let json = r#"
{
	"@type": "updateUser",
	"user": {
		"@type": "user",
		"id": 743550508,
		"first_name": "Jo",
		"last_name": "ho",
		"username": "laocaimi",
		"phone_number": "",
		"status": {
			"@type": "userStatusOffline",
			"was_online": 1556077825
		},
		"profile_photo": {
			"@type": "profilePhoto",
			"id": "3193525115240425385",
			"small": {
				"@type": "file",
				"id": 1,
				"size": 0,
				"expected_size": 0,
				"local": {
					"@type": "localFile",
					"path": "",
					"can_be_downloaded": true,
					"can_be_deleted": false,
					"is_downloading_active": false,
					"is_downloading_completed": false,
					"download_offset": 0,
					"downloaded_prefix_size": 0,
					"downloaded_size": 0
				},
				"remote": {
					"@type": "remoteFile",
					"id": "AQADAQADqacxGyyuUSwACL8DCzAABFCmbbpR8R0fdAoDAAEC",
					"is_uploading_active": false,
					"is_uploading_completed": true,
					"uploaded_size": 0
				}
			},
			"big": {
				"@type": "file",
				"id": 2,
				"size": 0,
				"expected_size": 0,
				"local": {
					"@type": "localFile",
					"path": "",
					"can_be_downloaded": true,
					"can_be_deleted": false,
					"is_downloading_active": false,
					"is_downloading_completed": false,
					"download_offset": 0,
					"downloaded_prefix_size": 0,
					"downloaded_size": 0
				},
				"remote": {
					"@type": "remoteFile",
					"id": "AQADAQADqacxGyyuUSwACL8DCzAABHLmUqWpsqcqdgoDAAEC",
					"is_uploading_active": false,
					"is_uploading_completed": true,
					"uploaded_size": 0
				}
			}
		},
		"outgoing_link": {
			"@type": "linkStateNone"
		},
		"incoming_link": {
			"@type": "linkStateNone"
		},
		"is_verified": false,
		"is_support": false,
		"restriction_reason": "",
		"have_access": true,
		"type": {
			"@type": "userTypeRegular"
		},
		"language_code": ""
	}
}
  "#;

//  let json = r#"{"@type":"updateUser","user":{"@type":"user","id":743550508,"first_name":"Jo","last_name":"ho","username":"laocaimi","phone_number":"","status":{"@type":"userStatusOffline","was_online":1556077825},"profile_photo":{"@type":"profilePhoto","id":"3193525115240425385","small":{"@type":"file","id":1,"size":0,"expected_size":0,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AQADAQADqacxGyyuUSwACL8DCzAABFCmbbpR8R0fdAoDAAEC","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":0}},"big":{"@type":"file","id":2,"size":0,"expected_size":0,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AQADAQADqacxGyyuUSwACL8DCzAABHLmUqWpsqcqdgoDAAEC","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":0}}},"outgoing_link":{"@type":"linkStateNone"},"incoming_link":{"@type":"linkStateNone"},"is_verified":false,"is_support":false,"restriction_reason":"","have_access":true,"type":{"@type":"userTypeRegular"},"language_code":""}}"#;

//  let json = r#"{"@type":"updateUser","user":{"@type":"user","id":720963235,"first_name":"aking","last_name":"888","username":"aking88888","phone_number":"","status":{"@type":"userStatusOffline","was_online":1559590497},"outgoing_link":{"@type":"linkStateNone"},"incoming_link":{"@type":"linkStateNone"},"is_verified":false,"is_support":false,"restriction_reason":"","have_access":true,"type":{"@type":"userTypeRegular"},"language_code":""}}"#;

  let object: Option<Box<rtdlib::types::Object>> = rtdlib::types::Object::from_json(json);
  println!("{:?}", object);
}


