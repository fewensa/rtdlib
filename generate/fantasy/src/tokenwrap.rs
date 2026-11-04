use case::CaseExt;

use tl_parser::types::*;

use crate::tdfill::TDTypeFill;

lazy_static! {
  static ref SKIP_TYPES: Vec<&'static str> = {
    vec![
      "double",
      "string",
      "int32",
      "int53",
      "int64",
      "bytes",
      "boolFalse",
      "boolTrue",
      "vector",
    ]
  };
}

#[derive(Debug, Clone)]
pub struct TokenWrap {
  tokens: Vec<TLTokenGroup>,
  tdtypefill: TDTypeFill,
}

impl TokenWrap {
  pub fn new(tokens: Vec<TLTokenGroup>, tdtypefill: TDTypeFill) -> Self {
    Self { tokens, tdtypefill }
  }

  pub fn tokens(&self) -> &Vec<TLTokenGroup> {
    &self.tokens
  }

  pub fn tdtypefill(&self) -> &TDTypeFill {
    &self.tdtypefill
  }

  /// is skip type
  pub fn is_skip_type(&self, type_name: String) -> bool {
    SKIP_TYPES.contains(&&type_name[..])
  }

  /// The file where td type is located
  pub fn which_file(&self, type_name: String) -> String {
    let token = self.tokens.iter()
      .find(|&item| item.name() == type_name)
      .expect(&format!("Can't found this type -> {}", type_name));
    if token.type_() == TLTokenGroupType::Trait {
      return token.name().to_snake();
    }
    if token.type_() == TLTokenGroupType::Function {
      return "functions".to_string();
    }
    match token.blood() {
      Some(blood) => {
        if blood == type_name {
          type_name.to_snake()
        } else {
          blood.to_snake()
        }
      }
      None => token.name().to_snake()
    }
  }

  pub fn is_optional_arg(&self, token: &TLTokenGroup, arg: &TLTokenArgType) -> bool {
    self.tdtypefill.td_filter(token.name(), arg.sign_name())
      .map_or_else(|| arg.description().map_or(false, |v| {
        let nospace = v.replace(" ", "");
        if nospace.contains("maybenull") { return true }
        false
      }), |v| v.optional())
  }
}
