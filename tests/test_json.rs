use rtdlib::types::*;

#[test]
fn test_authorization_state() {
  let json = r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#;
  let state: UpdateAuthorizationState = serde_json::from_str(&json[..]).expect("Json fail");
  assert_eq!("updateAuthorizationState", state.td_name());
  let rjson = state.to_json();
  assert!(rjson.is_ok(), true);
  assert_eq!(json, rjson.unwrap());
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

  let update_user: Result<UpdateUser, rtdlib::errors::RTDError> = UpdateUser::from_json(json);
  assert!(update_user.is_ok(), true);
  let update_user = update_user.unwrap();
  let user = update_user.user();
  assert_eq!(user.is_verified(), false);
  assert_eq!(user.is_support(), false);
  assert_eq!(user.type_().is_regular(), true);
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


