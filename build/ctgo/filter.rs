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
        // On the telegram api document website, this type is std::int64_t   https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1profile_photo.html,
        // So rtdlib use i64, but libtdjson return json is a string.
        // {"@type":"updateUser","user":{"@type":"user","profile_photo":{"@type":"profilePhoto","id":"3193525115240425385",..}}
        // {"@type":"profilePhoto","id":"3193525115240425385", ...}
        // is not "id": 3193525115240425385
        // if use Option<i64> serde_json deserialize will throw error.
        "id" => "String",
        _ => origin_field_type
      }
    }
    "Chat" => {
      match field_name {
        // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1chat.html
        // order is std::int64_t
        // but libtdjson return data is string
        // sample
        // {"@type":"updateNewChat","chat":{"@type":"chat","id":690763082,"type":{"@type":"chatTypePrivate","user_id":190363082},"title":"Fnunkuy","order":"0","is_pinned":false, ...}
        "order" => "String",
        _ => origin_field_type
      }
    }
    "Message" => {
      match field_name {
        // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1message.html
        // media_album_id is std::int64_t
        // sample
        // {"@type":"updateNewMessage","message":{"@type":"message","id":139460608,"media_album_id":"0", ...}}
        "media_album_id" => "String",
        _ => origin_field_type
      }
    }
    "UpdateChatLastMessage" => {
      match field_name {
        // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1update_chat_last_message.html
        // order is std::int64_t
        // sample
        // {"@type":"updateChatLastMessage","chat_id":690763082,"last_message":{"@type":"message",...},"order":"0"}
        "order" => "String",
        _ => origin_field_type
      }
    }
    "Poll" => {
      match field_name {
        // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1poll.html
        // sample
        // "poll":{"@type":"poll","@struct":"Poll","id":"6233357861422891010", ... }
        "id" => "String",
        _ => origin_field_type,
      }
    }
    "Sticker" => {
      match field_name {
        // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1sticker.html
        // sample
        // "sticker":{"@type":"sticker","@struct":"Sticker","set_id":"2463557141785477121", ... }
        "set_id" => "String",
        _ => origin_field_type,
      }
    },
    "SupergroupFullInfo" => {
      match field_name {
        // https://core.telegram.org/tdlib/docs/classtd_1_1td__api_1_1supergroup_full_info.html
        // sample
        // "supergroup_full_info":{"sticker_set_id": "0", ... }
        "sticker_set_id" => "String",
        _ => origin_field_type
      }
    },
    _ => origin_field_type
  };

  Ok(serde_json::value::to_value(if wrap_option { format!("Option<{}>", field_type) } else { field_type.to_string() }).unwrap())
}
