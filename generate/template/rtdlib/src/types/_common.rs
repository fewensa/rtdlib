use std::fmt::Debug;

use serde::de::{Deserialize, Deserializer};

use crate::errors::*;
use crate::types::*;

macro_rules! rtd_enum_deserialize {
  ($type_name:ident, $(($td_name:ident, $enum_item:ident));*;) => {
    // example json
    // {"@type":"authorizationStateWaitEncryptionKey","is_encrypted":false}
    |deserializer: D| -> Result<$type_name, D::Error> {
      let rtd_trait_value: serde_json::Value = Deserialize::deserialize(deserializer)?;
      // the `rtd_trait_value` variable type is &serde_json::Value, tdlib trait will return a object, convert this type to object `&Map<String, Value>`
      let rtd_trait_map = match rtd_trait_value.as_object() {
        Some(map) => map,
        None => return Err(
          D::Error::custom(format!(
            "{} is not the correct type", stringify!($type_name)
          ))
        ) // &format!("{} is not the correct type", stringify!($field))[..]
      };
      // get `@type` value, detect specific types
      let rtd_trait_type = match rtd_trait_map.get("@type") {
        // the `t` variable type is `serde_json::Value`, convert `t` to str
        Some(t) => match t.as_str() {
          Some(s) => s,
          None => return Err(
            D::Error::custom(format!(
              "{} -> @type is not the correct type", stringify!($type_name)
            ))
          ) // &format!("{} -> @type is not the correct type", stringify!($field))[..]
        },
        None => return Err(D::Error::custom(format!("unknown field {} -> @type", stringify!($type_name))))
      };

      let obj = match rtd_trait_type {
        $(
          stringify!($td_name) => $type_name::$enum_item(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(
              D::Error::custom(format!(
                "{} can't deserialize to {}::{}; {:?}",
                stringify!($td_name),
                stringify!($type_name),
                stringify!($enum_item),
                _e,
              ))
            )
          }),
        )*
        _ => return Err(D::Error::custom(format!("missing field {}", rtd_trait_type)))
      };
      Ok(obj)
    }
  }
}


pub fn detect_td_type<S: AsRef<str>>(json: S) -> Option<String> {
  let value: serde_json::Value = serde_json::from_str(json.as_ref()).ok()?;
  let obj = value.as_object()?;
  obj.get("@type")?.as_str().map(str::to_string)
}

pub fn detect_td_type_and_extra<S: AsRef<str>>(json: S) -> (Option<String>, Option<String>) {
  let Ok(value) = serde_json::from_str::<serde_json::Value>(json.as_ref()) else {
    return (None, None);
  };
  let Some(map) = value.as_object() else {
    return (None, None);
  };
  let type_ = map.get("@type").and_then(serde_json::Value::as_str).map(str::to_string);
  let extra = map.get("@extra").and_then(serde_json::Value::as_str).map(str::to_string);
  (type_, extra)
}

pub fn from_json<'a, T>(json: &'a str) -> RTDResult<T> where T: serde::de::Deserialize<'a>, {
  Ok(serde_json::from_str(json)?)
}

/// All tdlib type abstract class defined the same behavior
pub trait RObject: Debug {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str;
  #[doc(hidden)]
  fn extra(&self) -> Option<String>;
  /// Return td type to json string
  fn to_json(&self) -> RTDResult<String>;
}

pub trait RFunction: Debug + RObject {}


impl<RObj: RObject> RObject for &RObj {
  fn td_name(&self) -> &'static str { (*self).td_name() }
  fn to_json(&self) -> RTDResult<String> { (*self).to_json() }
  fn extra(&self) -> Option<String> { (*self).extra() }
}

impl<RObj: RObject> RObject for &mut RObj {
  fn td_name(&self) -> &'static str { (**self).td_name() }
  fn to_json(&self) -> RTDResult<String> { (**self).to_json() }
  fn extra(&self) -> Option<String> { (**self).extra() }
}


impl<Fnc: RFunction> RFunction for &Fnc {}
impl<Fnc: RFunction> RFunction for &mut Fnc {}

{% for token in tokens %}{% if token.type_ == 'Trait' %}
impl<{{token.name | upper}}: TD{{token.name | to_camel}}> TD{{token.name | to_camel}} for &{{token.name | upper}} {}
impl<{{token.name | upper}}: TD{{token.name | to_camel}}> TD{{token.name | to_camel}} for &mut {{token.name | upper}} {}
{% endif %}{% endfor %}

#[derive(Debug, Clone)]
pub enum TdType {
{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}  {{token.name | to_camel }}({{token.name | to_camel}}),
{% endif %}{% endfor %}
{% for token in tokens %}{% if token.is_return_type %}  {{token.name | to_camel }}({{token.name | to_camel}}),
{% endif %}{% endfor %}
}
impl<'de> Deserialize<'de> for TdType {
fn deserialize<D>(deserializer: D) -> Result<TdType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TdType,
{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}  ({{token.name }}, {{token.name | to_camel}});
{% endif %}{% endfor %}
{% for token in tokens %}{% if token.is_return_type %}  ({{token.name }}, {{token.name | to_camel}});
{% endif %}{% endfor %}
 )(deserializer)

 }
}

impl RObject for TdType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
    {% for token in tokens %}{% if token.blood and token.blood == 'Update' %}  Self::{{token.name | to_camel }}(value) => value.td_name(),
    {% endif %}{% endfor %}
    {% for token in tokens %}{% if token.is_return_type %}  Self::{{token.name | to_camel }}(value) => value.td_name(),
    {% endif %}{% endfor %}
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      {% for token in tokens %}{% if token.blood and token.blood == 'Update' %}  Self::{{token.name | to_camel }}(value) => value.extra(),
      {% endif %}{% endfor %}
      {% for token in tokens %}{% if token.is_return_type %}  Self::{{token.name | to_camel }}(value) => value.extra(),
      {% endif %}{% endfor %}
    }
  }
  /// Return td type to json string
  fn to_json(&self) -> RTDResult<String> {
    match self {
      {% for token in tokens %}{% if token.blood and token.blood == 'Update' %}  Self::{{token.name | to_camel }}(value) => value.to_json(),
      {% endif %}{% endfor %}
      {% for token in tokens %}{% if token.is_return_type %}  Self::{{token.name | to_camel }}(value) => value.to_json(),
      {% endif %}{% endfor %}
    }
  }
}



#[cfg(test)]
mod tests {
  use crate::types::{TdType, from_json, UpdateAuthorizationState};

  #[test]
  fn test_deserialize_enum() {
    match from_json::<UpdateAuthorizationState>(r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#) {
      Ok(t) => {},
      Err(e) => {panic!("{}", e)}
    };

    match from_json::<TdType>(r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#) {
      Ok(t) => {
        match t {
          TdType::UpdateAuthorizationState(v) => {},
          _ => panic!("from_json failed: {:?}", t)
        }
      },
      Err(e) => {panic!("{}", e)}
    };
  }
}

