use std::collections::HashMap;

use serde_json::value::Value;

use crate::ctgo::types::TdType;

pub fn filter_field_type(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {

  let td = arg.get("td").expect("field type filter lost [td] argument");
  let td: TdType = serde_json::from_value(td.clone()).expect("field type filter argument [td] is not TdType type");
  let field_name = arg.get("field_name").expect("field type filter lost [field_name] argument");
  let field_name = field_name.as_str().expect("field type filter argument [field_name] is not String type");
  let wrap_option = arg.get("wrap_option").expect("field type filter lost [wrap_option] argument");
  let wrap_option: bool = wrap_option.as_bool().expect("field type filter argument [wrap_option] is not bool type");



  let origin_field_type = value.as_str().expect("field type filter value is not String type");

  let field_type = match &td.clz_name[..] {
    "ProfilePhoto" => {
      match field_name {
        // On the telegram api document website, this type is std::int64_t,
        // So rtdlib use i64, but libtdjson return json is a string.
        // {"@type":"updateUser","user":{"@type":"user","id":743550508,"first_name":"Jo","last_name":"ho","username":"laocaimi","phone_number":"","status":{"@type":"userStatusOffline","was_online":1556077825},"profile_photo":{"@type":"profilePhoto","id":"3193525115240425385","small":{"@type":"file","id":1,"size":0,"expected_size":0,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AQADAQADqacxGyyuUSwACL8DCzAABFCmbbpR8R0fdAoDAAEC","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":0}},"big":{"@type":"file","id":2,"size":0,"expected_size":0,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AQADAQADqacxGyyuUSwACL8DCzAABHLmUqWpsqcqdgoDAAEC","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":0}}},"outgoing_link":{"@type":"linkStateNone"},"incoming_link":{"@type":"linkStateNone"},"is_verified":false,"is_support":false,"restriction_reason":"","have_access":true,"type":{"@type":"userTypeRegular"},"language_code":""}}
        // {"@type":"profilePhoto","id":"3193525115240425385", ...}
        // is not "id": 3193525115240425385
        // if use Option<i64> serde_json deserialize will throw error.
        "id" => "String",
        _ => origin_field_type
      }
    }
    _ => origin_field_type
  };

  Ok(serde_json::value::to_value(if wrap_option { format!("Option<{}>", field_type) } else { field_type.to_string() }).unwrap())
}
