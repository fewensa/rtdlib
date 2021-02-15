
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a JSON value
pub trait TDJsonValue: Debug + RObject {}

/// Represents a JSON value
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum JsonValue {
  #[doc(hidden)] _Default(()),
  /// Returns application config, provided by the server. Can be called before authorization
  GetApplicationConfig(GetApplicationConfig),
  /// Converts a JSON-serialized string to corresponding JsonValue object. This is an offline method. Can be called before authorization. Can be called synchronously
  GetJsonValue(GetJsonValue),
  /// Represents a JSON array
  Array(JsonValueArray),
  /// Represents a boolean JSON value
  Boolean(JsonValueBoolean),
  /// Represents a null JSON value
  Null(JsonValueNull),
  /// Represents a numeric JSON value
  Number(JsonValueNumber),
  /// Represents a JSON object
  Object(JsonValueObject),
  /// Represents a string JSON value
  String(JsonValueString),

}

impl Default for JsonValue {
  fn default() -> Self { JsonValue::_Default(()) }
}

impl<'de> Deserialize<'de> for JsonValue {
  fn deserialize<D>(deserializer: D) -> Result<JsonValue, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      JsonValue,
      (getApplicationConfig, GetApplicationConfig);
      (getJsonValue, GetJsonValue);
      (jsonValueArray, Array);
      (jsonValueBoolean, Boolean);
      (jsonValueNull, Null);
      (jsonValueNumber, Number);
      (jsonValueObject, Object);
      (jsonValueString, String);

    )(deserializer)
  }
}

impl RObject for JsonValue {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      JsonValue::GetApplicationConfig(t) => t.td_name(),
      JsonValue::GetJsonValue(t) => t.td_name(),
      JsonValue::Array(t) => t.td_name(),
      JsonValue::Boolean(t) => t.td_name(),
      JsonValue::Null(t) => t.td_name(),
      JsonValue::Number(t) => t.td_name(),
      JsonValue::Object(t) => t.td_name(),
      JsonValue::String(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      JsonValue::GetApplicationConfig(t) => t.extra(),
      JsonValue::GetJsonValue(t) => t.extra(),
      JsonValue::Array(t) => t.extra(),
      JsonValue::Boolean(t) => t.extra(),
      JsonValue::Null(t) => t.extra(),
      JsonValue::Number(t) => t.extra(),
      JsonValue::Object(t) => t.extra(),
      JsonValue::String(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl JsonValue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let JsonValue::_Default(_) = self { true } else { false } }

  pub fn is_get_application_config(&self) -> bool { if let JsonValue::GetApplicationConfig(_) = self { true } else { false } }
  pub fn is_get_json_value(&self) -> bool { if let JsonValue::GetJsonValue(_) = self { true } else { false } }
  pub fn is_array(&self) -> bool { if let JsonValue::Array(_) = self { true } else { false } }
  pub fn is_boolean(&self) -> bool { if let JsonValue::Boolean(_) = self { true } else { false } }
  pub fn is_null(&self) -> bool { if let JsonValue::Null(_) = self { true } else { false } }
  pub fn is_number(&self) -> bool { if let JsonValue::Number(_) = self { true } else { false } }
  pub fn is_object(&self) -> bool { if let JsonValue::Object(_) = self { true } else { false } }
  pub fn is_string(&self) -> bool { if let JsonValue::String(_) = self { true } else { false } }

  pub fn on_get_application_config<F: FnOnce(&GetApplicationConfig)>(&self, fnc: F) -> &Self { if let JsonValue::GetApplicationConfig(t) = self { fnc(t) }; self }
  pub fn on_get_json_value<F: FnOnce(&GetJsonValue)>(&self, fnc: F) -> &Self { if let JsonValue::GetJsonValue(t) = self { fnc(t) }; self }
  pub fn on_array<F: FnOnce(&JsonValueArray)>(&self, fnc: F) -> &Self { if let JsonValue::Array(t) = self { fnc(t) }; self }
  pub fn on_boolean<F: FnOnce(&JsonValueBoolean)>(&self, fnc: F) -> &Self { if let JsonValue::Boolean(t) = self { fnc(t) }; self }
  pub fn on_null<F: FnOnce(&JsonValueNull)>(&self, fnc: F) -> &Self { if let JsonValue::Null(t) = self { fnc(t) }; self }
  pub fn on_number<F: FnOnce(&JsonValueNumber)>(&self, fnc: F) -> &Self { if let JsonValue::Number(t) = self { fnc(t) }; self }
  pub fn on_object<F: FnOnce(&JsonValueObject)>(&self, fnc: F) -> &Self { if let JsonValue::Object(t) = self { fnc(t) }; self }
  pub fn on_string<F: FnOnce(&JsonValueString)>(&self, fnc: F) -> &Self { if let JsonValue::String(t) = self { fnc(t) }; self }

  pub fn as_get_application_config(&self) -> Option<&GetApplicationConfig> { if let JsonValue::GetApplicationConfig(t) = self { return Some(t) } None }
  pub fn as_get_json_value(&self) -> Option<&GetJsonValue> { if let JsonValue::GetJsonValue(t) = self { return Some(t) } None }
  pub fn as_array(&self) -> Option<&JsonValueArray> { if let JsonValue::Array(t) = self { return Some(t) } None }
  pub fn as_boolean(&self) -> Option<&JsonValueBoolean> { if let JsonValue::Boolean(t) = self { return Some(t) } None }
  pub fn as_null(&self) -> Option<&JsonValueNull> { if let JsonValue::Null(t) = self { return Some(t) } None }
  pub fn as_number(&self) -> Option<&JsonValueNumber> { if let JsonValue::Number(t) = self { return Some(t) } None }
  pub fn as_object(&self) -> Option<&JsonValueObject> { if let JsonValue::Object(t) = self { return Some(t) } None }
  pub fn as_string(&self) -> Option<&JsonValueString> { if let JsonValue::String(t) = self { return Some(t) } None }



  pub fn get_application_config<T: AsRef<GetApplicationConfig>>(t: T) -> Self { JsonValue::GetApplicationConfig(t.as_ref().clone()) }

  pub fn get_json_value<T: AsRef<GetJsonValue>>(t: T) -> Self { JsonValue::GetJsonValue(t.as_ref().clone()) }

  pub fn array<T: AsRef<JsonValueArray>>(t: T) -> Self { JsonValue::Array(t.as_ref().clone()) }

  pub fn boolean<T: AsRef<JsonValueBoolean>>(t: T) -> Self { JsonValue::Boolean(t.as_ref().clone()) }

  pub fn null<T: AsRef<JsonValueNull>>(t: T) -> Self { JsonValue::Null(t.as_ref().clone()) }

  pub fn number<T: AsRef<JsonValueNumber>>(t: T) -> Self { JsonValue::Number(t.as_ref().clone()) }

  pub fn object<T: AsRef<JsonValueObject>>(t: T) -> Self { JsonValue::Object(t.as_ref().clone()) }

  pub fn string<T: AsRef<JsonValueString>>(t: T) -> Self { JsonValue::String(t.as_ref().clone()) }

}

impl AsRef<JsonValue> for JsonValue {
  fn as_ref(&self) -> &JsonValue { self }
}







/// Represents a JSON array
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueArray {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of array elements
  values: Vec<JsonValue>,
  
}

impl RObject for JsonValueArray {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "jsonValueArray" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDJsonValue for JsonValueArray {}



impl JsonValueArray {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJsonValueArrayBuilder {
    let mut inner = JsonValueArray::default();
    inner.td_name = "jsonValueArray".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDJsonValueArrayBuilder { inner }
  }

  pub fn values(&self) -> &Vec<JsonValue> { &self.values }

}

#[doc(hidden)]
pub struct RTDJsonValueArrayBuilder {
  inner: JsonValueArray
}

impl RTDJsonValueArrayBuilder {
  pub fn build(&self) -> JsonValueArray { self.inner.clone() }

   
  pub fn values(&mut self, values: Vec<JsonValue>) -> &mut Self {
    self.inner.values = values;
    self
  }

}

impl AsRef<JsonValueArray> for JsonValueArray {
  fn as_ref(&self) -> &JsonValueArray { self }
}

impl AsRef<JsonValueArray> for RTDJsonValueArrayBuilder {
  fn as_ref(&self) -> &JsonValueArray { &self.inner }
}







/// Represents a boolean JSON value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueBoolean {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The value
  value: bool,
  
}

impl RObject for JsonValueBoolean {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "jsonValueBoolean" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDJsonValue for JsonValueBoolean {}



impl JsonValueBoolean {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJsonValueBooleanBuilder {
    let mut inner = JsonValueBoolean::default();
    inner.td_name = "jsonValueBoolean".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDJsonValueBooleanBuilder { inner }
  }

  pub fn value(&self) -> bool { self.value }

}

#[doc(hidden)]
pub struct RTDJsonValueBooleanBuilder {
  inner: JsonValueBoolean
}

impl RTDJsonValueBooleanBuilder {
  pub fn build(&self) -> JsonValueBoolean { self.inner.clone() }

   
  pub fn value(&mut self, value: bool) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<JsonValueBoolean> for JsonValueBoolean {
  fn as_ref(&self) -> &JsonValueBoolean { self }
}

impl AsRef<JsonValueBoolean> for RTDJsonValueBooleanBuilder {
  fn as_ref(&self) -> &JsonValueBoolean { &self.inner }
}







/// Represents a null JSON value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueNull {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for JsonValueNull {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "jsonValueNull" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDJsonValue for JsonValueNull {}



impl JsonValueNull {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJsonValueNullBuilder {
    let mut inner = JsonValueNull::default();
    inner.td_name = "jsonValueNull".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDJsonValueNullBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDJsonValueNullBuilder {
  inner: JsonValueNull
}

impl RTDJsonValueNullBuilder {
  pub fn build(&self) -> JsonValueNull { self.inner.clone() }

}

impl AsRef<JsonValueNull> for JsonValueNull {
  fn as_ref(&self) -> &JsonValueNull { self }
}

impl AsRef<JsonValueNull> for RTDJsonValueNullBuilder {
  fn as_ref(&self) -> &JsonValueNull { &self.inner }
}







/// Represents a numeric JSON value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The value
  value: f32,
  
}

impl RObject for JsonValueNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "jsonValueNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDJsonValue for JsonValueNumber {}



impl JsonValueNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJsonValueNumberBuilder {
    let mut inner = JsonValueNumber::default();
    inner.td_name = "jsonValueNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDJsonValueNumberBuilder { inner }
  }

  pub fn value(&self) -> f32 { self.value }

}

#[doc(hidden)]
pub struct RTDJsonValueNumberBuilder {
  inner: JsonValueNumber
}

impl RTDJsonValueNumberBuilder {
  pub fn build(&self) -> JsonValueNumber { self.inner.clone() }

   
  pub fn value(&mut self, value: f32) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<JsonValueNumber> for JsonValueNumber {
  fn as_ref(&self) -> &JsonValueNumber { self }
}

impl AsRef<JsonValueNumber> for RTDJsonValueNumberBuilder {
  fn as_ref(&self) -> &JsonValueNumber { &self.inner }
}







/// Represents a JSON object
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueObject {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of object members
  members: Vec<JsonObjectMember>,
  
}

impl RObject for JsonValueObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "jsonValueObject" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDJsonValue for JsonValueObject {}



impl JsonValueObject {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJsonValueObjectBuilder {
    let mut inner = JsonValueObject::default();
    inner.td_name = "jsonValueObject".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDJsonValueObjectBuilder { inner }
  }

  pub fn members(&self) -> &Vec<JsonObjectMember> { &self.members }

}

#[doc(hidden)]
pub struct RTDJsonValueObjectBuilder {
  inner: JsonValueObject
}

impl RTDJsonValueObjectBuilder {
  pub fn build(&self) -> JsonValueObject { self.inner.clone() }

   
  pub fn members(&mut self, members: Vec<JsonObjectMember>) -> &mut Self {
    self.inner.members = members;
    self
  }

}

impl AsRef<JsonValueObject> for JsonValueObject {
  fn as_ref(&self) -> &JsonValueObject { self }
}

impl AsRef<JsonValueObject> for RTDJsonValueObjectBuilder {
  fn as_ref(&self) -> &JsonValueObject { &self.inner }
}







/// Represents a string JSON value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The value
  value: String,
  
}

impl RObject for JsonValueString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "jsonValueString" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDJsonValue for JsonValueString {}



impl JsonValueString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJsonValueStringBuilder {
    let mut inner = JsonValueString::default();
    inner.td_name = "jsonValueString".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDJsonValueStringBuilder { inner }
  }

  pub fn value(&self) -> &String { &self.value }

}

#[doc(hidden)]
pub struct RTDJsonValueStringBuilder {
  inner: JsonValueString
}

impl RTDJsonValueStringBuilder {
  pub fn build(&self) -> JsonValueString { self.inner.clone() }

   
  pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().to_string();
    self
  }

}

impl AsRef<JsonValueString> for JsonValueString {
  fn as_ref(&self) -> &JsonValueString { self }
}

impl AsRef<JsonValueString> for RTDJsonValueStringBuilder {
  fn as_ref(&self) -> &JsonValueString { &self.inner }
}



