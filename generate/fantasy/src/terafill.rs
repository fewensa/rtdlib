use std::collections::HashMap;

use case::CaseExt;
use rstring_builder::StringBuilder;
use serde::ser::Error;
use serde_json::Value;
use tera::Tera;

use tl_parser::types::{TLTokenArgType, TLTokenComponentType, TLTokenGroup};

use crate::tdfill::TDTypeFill;
use crate::tokenwrap::TokenWrap;

pub fn fill(tera: &mut Tera, tknwrap: TokenWrap) -> Result<(), failure::Error> {
  self::add_filter_case(tera)?;
  self::add_filter_td(tera)?;
  self::add_td_fnc(tera, tknwrap)?;
  Ok(())
}

fn add_filter_case(tera: &mut Tera) -> Result<(), failure::Error> {
  // snake
  fn case_to_snake_filter(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_snake()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  fn case_to_camel(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_camel()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  fn case_to_camel_lowercase(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_camel_lowercase()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  fn case_to_dashed(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_dashed()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  fn case_to_capitalized(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_capitalized()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
//  fn case_to
  tera.register_filter("to_snake", case_to_snake_filter);
  tera.register_filter("to_camel", case_to_camel);
  tera.register_filter("to_camel_lowercase", case_to_camel_lowercase);
  tera.register_filter("to_dashed", case_to_dashed);
  tera.register_filter("to_capitalized", case_to_capitalized);
  Ok(())
}

fn add_filter_td(tera: &mut Tera) -> Result<(), failure::Error> {
  fn td_safe_field(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => match text {
        "type" => Ok(serde_json::value::to_value("type_".to_string()).unwrap()),
        "extra" => Ok(serde_json::value::to_value("extra_".to_string()).unwrap()),
        "async" => Ok(serde_json::value::to_value("async_".to_string()).unwrap()),
        "static" => Ok(serde_json::value::to_value("static_".to_string()).unwrap()),
        _ => Ok(serde_json::value::to_value(text.to_snake()).unwrap())
      },
      None => Err(format!("Error value {:?}", value).into())
    }
  }

  fn td_remove_prefix(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    let prefix = match arg.get("prefix") {
      Some(t) => t.as_str(),
      None => None
    };
    match value.as_str() {
      Some(text) => {
        if prefix.is_none() { return Ok(serde_json::value::to_value(text.to_string()).unwrap()) }
        let prefix = prefix.unwrap();
        let plura_prefix = format!("{}s", prefix);
        if plura_prefix.to_lowercase() == text.to_lowercase() {
          return Ok(serde_json::value::to_value(text).unwrap());
        }
        if !text.to_lowercase().starts_with(&prefix.to_lowercase()[..]) {
          return Ok(serde_json::value::to_value(text).unwrap());
        }

        let prefix_chars = prefix.chars().collect::<Vec<char>>();
        let text_chars = text.chars().collect::<Vec<char>>();
        let ret = text.chars().skip(prefix_chars.len()).take(text_chars.len()).collect::<String>();
        Ok(serde_json::value::to_value(ret).unwrap())
      }
      None => Err(format!("Error value {:?}", value).into())
    }
  }

  tera.register_filter("td_safe_field", td_safe_field);
  tera.register_filter("td_remove_prefix", td_remove_prefix);
  Ok(())
}

fn add_td_fnc(tera: &mut Tera, tknwrap: TokenWrap) -> Result<(), failure::Error> {
  let tknwrap0 = tknwrap.clone();
  let tknwrap1 = tknwrap.clone();
  let tknwrap2 = tknwrap.clone();
  let tknwrap3 = tknwrap.clone();

  // argument serde_aux field_attributes
  let td_macros = Box::new(move |argument: HashMap<String, Value>| -> tera::Result<Value> {
    let tdtypefill = tknwrap3.tdtypefill();

    let token: TLTokenGroup = match argument.get("token") {
      Some(t) => match serde_json::from_value(t.clone()) {
        Ok(a) => a,
        Err(e) => return Err("Can't covert token to TLTokenGroup".into())
      },
      None => return Err("Can't found token".into())
    };
    let arg: TLTokenArgType = match argument.get("arg") {
      Some(t) => match serde_json::from_value(t.clone()) {
        Ok(a) => a,
        Err(e) => return Err("Can't covert arg to TLTokenArgType".into())
      },
      None => return Err("Can't found arg".into())
    };

    let mut aux = tdtypefill.td_filter_macros(token.name(), arg.sign_name());
    // TDLib JSON encodes every int64 as a string. serde-aux accepts a string or a number.
    if arg.sign_type() == "int64"
      && !aux.iter().any(|item| item.contains("deserialize_") && item.contains("number_from_string"))
    {
      let optional = tknwrap3.is_optional_arg(&token, &arg);
      let path = if optional {
        "deserialize_option_number_from_string"
      } else {
        "deserialize_number_from_string"
      };
      aux.push(format!(
        "#[serde(deserialize_with = \"serde_aux::field_attributes::{path}\")]"
      ));
    }
    Ok(serde_json::value::to_value(aux).unwrap())
  });

  // argument type
  let td_arg = Box::new(move |argument: HashMap<String, Value>| -> tera::Result<Value> {
    let tdtypefill = tknwrap0.tdtypefill();

    let token: TLTokenGroup = match argument.get("token") {
      Some(t) => match serde_json::from_value(t.clone()) {
        Ok(a) => a,
        Err(e) => return Err("Can't covert token to TLTokenGroup".into())
      },
      None => return Err("Can't found token".into())
    };
    let arg: TLTokenArgType = match argument.get("arg") {
      Some(t) => match serde_json::from_value(t.clone()) {
        Ok(a) => a,
        Err(e) => return Err("Can't covert arg to TLTokenArgType".into())
      },
      None => return Err("Can't found arg".into())
    };
    let builder_arg = match argument.get("builder_arg") {
      Some(t) => match t.as_bool() {
        Some(t) => t,
        None => false,
      },
      None => false
    };
    let mut arg_type = tdtypefill.mapper(arg.sign_type()).map_or(arg.sign_type().to_camel(), |v| v);
    let components = arg.components();
    if !components.is_empty() {
      let component_type = self::fill_type_components(components, &tdtypefill);
      arg_type = format!("{}{}", arg_type, component_type);
    }

    arg_type = tdtypefill.td_filter_type(token.name(), arg.sign_name(), arg_type);
    if !builder_arg && tknwrap0.is_optional_arg(&token, &arg) {
      arg_type = format!("Option<{}>", arg_type);
    }

    Ok(serde_json::value::to_value(arg_type).unwrap())
  });

  // all sub tokens
  let sub_tokens = Box::new(move |argument: HashMap<String, Value>| -> tera::Result<Value> {
    let token: TLTokenGroup = match argument.get("token") {
      Some(t) => match serde_json::from_value(t.clone()) {
        Ok(a) => a,
        Err(e) => return Err("Can't covert token to TLTokenGroup".into())
      },
      None => return Err("Can't found token".into())
    };

    let sub_tokens: Vec<&TLTokenGroup> = tknwrap1.tokens()
      .iter()
      .filter(|&v| v.blood().is_some())
      .filter(|v| v.blood().unwrap().to_lowercase() == token.name().to_lowercase() && v.name().to_lowercase() != token.name().to_lowercase())
      .collect();

    Ok(serde_json::value::to_value(sub_tokens).unwrap())
  });

  let find_token = Box::new(move |argument: HashMap<String, Value>| -> tera::Result<Value> {
    let token_name = match argument.get("token_name") {
      Some(t) => match t.as_str() {
        Some(n) => n,
        None => return Err("Can't get token name".into())
      },
      None => return Err("Lose token name".into())
    };
    let token = match tknwrap2.tokens()
      .iter()
      .find(|&v| v.name().to_lowercase() == token_name.to_lowercase()) {
      Some(t) => t,
      None => return Err(format!("Can't find token by this token_name => {}", token_name).into())
    };
    Ok(serde_json::value::to_value(token).unwrap())
  });

  // type is primitive
  let is_primitive = Box::new(|argument: HashMap<String, Value>| -> tera::Result<Value> {
    let type_ = match argument.get("type_") {
      Some(t) => match t.as_str() {
        Some(n) => n,
        None => return Err("Can't get target".into())
      },
      None => return Err("Lose target".into())
    };
    let is = match type_ {
      "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
      "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
      "f32" | "f64" | "str" |
      "bool" => true,
      _ => false
    };
    Ok(serde_json::value::to_value(is).unwrap())
  });

  // is optional
  let is_optional = Box::new(|argument: HashMap<String, Value>| -> tera::Result<Value> {
    let type_ = match argument.get("type_") {
      Some(t) => match t.as_str() {
        Some(n) => n,
        None => return Err("Can't get target".into())
      },
      None => return Err("Lose target".into())
    };
    let is = type_.starts_with("Option<");
    Ok(serde_json::value::to_value(is).unwrap())
  });

  let is_builder_ref = Box::new(|argument: HashMap<String, Value>| -> tera::Result<Value> {
    let type_ = match argument.get("type_") {
      Some(t) => match t.as_str() {
        Some(n) => n,
        None => return Err("Can't get target".into())
      },
      None => return Err("Lose target".into())
    };
    if type_.to_lowercase().starts_with("vec") { return Ok(serde_json::value::to_value(false).unwrap()) }
    let is = match type_ {
      "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
      "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
      "f32" | "f64" | "str" |
      "bool" => false,
      _ => true
    };
    Ok(serde_json::value::to_value(is).unwrap())
  });

  tera.register_function("td_arg", td_arg);
  tera.register_function("td_macros", td_macros);
  tera.register_function("sub_tokens", sub_tokens);
  tera.register_function("find_token", find_token);
  tera.register_function("is_primitive", is_primitive);
  tera.register_function("is_optional", is_optional);
  tera.register_function("is_builder_ref", is_builder_ref);
  Ok(())
}


fn fill_type_components(components: Vec<TLTokenComponentType>, tdtypefill: &TDTypeFill) -> String {
  let mut rets = vec![];
  for component in components {
    let mut sign_type = component.sign_type();
    sign_type = tdtypefill.mapper(&sign_type).map_or(sign_type.to_camel(), |v| v);

    let sub_components = component.components();
    if !sub_components.is_empty() {
      let component_type = fill_type_components(sub_components, tdtypefill);
      sign_type = format!("{}{}", sign_type, component_type);
    }
    rets.push(format!("<{}>", sign_type));
  }
  rets.join(", ")
}

