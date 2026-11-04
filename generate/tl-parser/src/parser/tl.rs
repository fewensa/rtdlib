use std::collections::{HashMap, HashSet};

use failure::Error;
use rstring_builder::StringBuilder;
use text_reader::TextReader;

use crate::types::*;

pub fn token_group(grammars: &Vec<Box<TLGrammar>>) -> Result<Vec<TLTokenGroup>, Error> {
  let mut tokens : HashMap<String, TLTokenGroup> = HashMap::new();
  let mut return_types: HashSet<String> = HashSet::new();
  let mut super_types_to_types: HashMap<String, Vec<String>> = HashMap::new();

  let mut token_group_type = TLTokenGroupType::Struct;
  for grammar in grammars {
    if grammar.is_group() {
      let group: TLGroup = grammar.to_group().expect("Impossible error");
      let token_group = parse_token_group(&group, token_group_type.clone())?;

      if let (Some(return_type), TLTokenGroupType::Function) = (&token_group.blood, &token_group.type_) {
        if let Some(token) = tokens.get_mut(return_type) {
          (*token).is_return_type = true;
        }
        else {
          return_types.insert(return_type.clone());
        }
      }

      if let (Some(super_type), TLTokenGroupType::Struct) = (&token_group.blood, &token_group.type_) {
        if let Some(child_types) = super_types_to_types.get_mut(super_type) {
          child_types.push(token_group.name.clone());
        }
        else {
          super_types_to_types.insert(super_type.clone(), vec![token_group.name.clone()]);
        }
      }

      tokens.insert(token_group.name.clone(), token_group);
    }
    if grammar.is_paragraph() {
      let paragraph: TLParagraph = grammar.to_paragraph().expect("Impossible error");
      match paragraph {
        TLParagraph::Functions { start, end } => {
          token_group_type = TLTokenGroupType::Function
        }
      }
      debug!("PARAGRAPH: {:?}", paragraph);
    }
  }

  for return_type in return_types.iter() {
    if let Some(token) = tokens.get_mut(return_type) {
      (*token).is_return_type = true;
    }
    else if let Some(child_tokens) = super_types_to_types.get(return_type) {
      match child_tokens.len() {
        1 => {
          let token_name = child_tokens.first().unwrap();
          if let Some(token_group) = tokens.get_mut(token_name) {
            (*token_group).is_return_type = true;
          }
        },
        _ => debug!("Skipping candidate return type {} because it is a super type with multiple children.", return_type)
      }
    }
  }

  let mut arrs: Vec<(&String, &TLTokenGroup)> = tokens.iter().collect();
  arrs.sort_by(|a, b| a.0.cmp(b.0));
  let rets: Vec<TLTokenGroup> = arrs.into_iter()
    .map(|(_, token_group)| token_group.clone())
    .collect();
  // for x in &rets {
  //   println!("{}", x.name);
  // }
  Ok(rets)
}


/// TLTokenGroup
fn parse_token_group(group: &TLGroup, token_group_type: TLTokenGroupType) -> Result<TLTokenGroup, Error> {
  let lines = &group.lines;

  let mut token_group = TLTokenGroup {
    description_all: None,
    description: None,
    name: "".to_string(),
    arguments: Default::default(),
    type_: token_group_type,
    blood: None,
    is_return_type: false
  };

  // description builder
  let mut dbuilder = StringBuilder::new();
  for (ix, gl) in lines.iter().enumerate() {
    let text = gl.text.clone();
    match gl.token {
      TLGroupLineToken::Trait => {
        return group_trait(gl);
      }
      TLGroupLineToken::Description => {
        dbuilder.append(text).append(' ');
      }
      TLGroupLineToken::Struct => {
        let name = group_name(&text)?;
        let blood = group_blood(&text)?;
        let args = group_args(gl.line, &text)?;

        token_group.name = name;
        token_group.blood = Some(blood);
        token_group.arguments = args;
      }
    }
  };

  let description_all = group_description(dbuilder.string());
  if let Some(dall) = &description_all {
    token_group.description = dall.get("description").map(|v| v.clone());
    let args = &mut token_group.arguments;
    for tat in args {
      tat.description = dall.get(&tat.sign_name[..]).map(|v| v.clone());
    }
  }
  token_group.description_all = description_all;
  Ok(token_group)
}

/// parse group trait
fn group_trait(gl: &TLGroupLine) -> Result<TLTokenGroup, Error> {
  let mut token_group = TLTokenGroup {
    description_all: None,
    description: None,
    name: "------".to_string(),
    arguments: Default::default(),
    type_: TLTokenGroupType::Trait,
    blood: None,
    is_return_type: false
  };

  let description_map = tl_description_map(gl.text.clone());
  let name = match description_map.get("class") {
    Some(class) => class.clone(),
    None => return bail!("Syntax error line -> {} -> {}", gl.line, gl.text)
  };
  let description = match description_map.get("description") {
    Some(class) => class.clone(),
    None => return bail!("Syntax error line -> {} -> {}", gl.line, gl.text)
  };

  token_group.description_all = Some(description_map);
  token_group.description = Some(description);
  token_group.name = name;
  Ok(token_group)
}

/// parse group arguments
fn group_args(line: i32, code: &String) -> Result<Vec<TLTokenArgType>, Error> {
  let words: Vec<&str> = code.split(" ").collect();
  let mut args = vec![];

  for (ix, &word) in words.iter().enumerate() {
    if ix == 0 { continue; }
    if word == "=" || word == "?" { break; }
    // component type defined
    if ix == 1 && word.starts_with("{") && word.ends_with("}") {
      return arg_type_define_with_component(line, code);
    }

    // struct sign
    if !word.contains(":") { return bail!("Syntax fail. line -> {} -> {}", line, code); }

    let arg_type = arg_type(line, code, word)?;
    args.push(arg_type);
  }

  Ok(args)
}

fn arg_type_define_with_component(line: i32, code: &String) -> Result<Vec<TLTokenArgType>, Error> {
  let mut reader = TextReader::new(code);
  let mut args = vec![];
  while reader.has_next() {
    match reader.next() {
      Some(' ') => continue,
      Some('{') => {
        let mut end = false;
        let mut builder = StringBuilder::new();
        while reader.has_next() {
          match reader.next() {
            Some(' ') => continue,
            Some('}') => {
              end = true;
              break;
            }
            Some(ch) => {
              builder.append(ch);
            }
            None => {}
          };
        };
        if !end { return bail!("Syntax fail. line -> {} -> {}", line, code); }
        let component_type_text = builder.string(); // like `t:Type`
        if component_type_text.is_empty() { return bail!("Syntax fail. line -> {} -> {}", line, code); }
        let arg_type = arg_type(line, code, &component_type_text)?;
        args.push(arg_type);
      }
      Some(ch) => {}
      None => {}
    };
  }
  Ok(args)
}

// TLTokenArgType
fn arg_type<S: AsRef<str>>(line: i32, code: &String, arg_text: S) -> Result<TLTokenArgType, Error> {
  let word = arg_text.as_ref();
  let signs: Vec<&str> = word.split(":").collect();
  if signs.len() != 2 { return bail!("Syntax fail. line -> {} -> {}", line, code); }
  let sign_name = match signs.get(0) {
    Some(&t) => t,
    None => return bail!("Syntax fail. line -> {} -> {}", line, code)
  };
  let sign_type = match signs.get(1) {
    Some(&t) => t,
    None => return bail!("Syntax fail. line -> {} -> {}", line, code)
  };

  // not have component type
  if !sign_type.contains("<") {
    let tat = TLTokenArgType::builder()
      .sign_name(sign_name)
      .sign_type(sign_type)
      .components(vec![])
      .build();
//    debug!("{:?}", tat);
    return Ok(tat);
  }

  let component_sign_type = component_sign_type(sign_type);
  let component_sign_components = component_sign_components(sign_type);
  let tat = TLTokenArgType::builder()
    .sign_name(sign_name)
    .sign_type(component_sign_type)
    .components(arg_component_types(component_sign_components))
    .build();
//  debug!("{}:{}  ----> {:#?}", sign_name, sign_type, tat);

  Ok(tat)
}


fn arg_component_types<S: AsRef<str>>(sign_type: S) -> Vec<TLTokenComponentType> {
  let mut rets = vec![];
  let sign_type = sign_type.as_ref();

  // not have sub components
  if !sign_type.contains("<") {
    let component_sign_type = component_sign_type(sign_type);
    let tct = TLTokenComponentType::builder()
      .sign_type(component_sign_type)
      .components(vec![])
      .build();
    rets.push(tct);
    return rets;
  }

  // have sub components
  let mut reader = TextReader::new(sign_type);
  let mut builder = StringBuilder::new();
  while reader.has_next() {
    match reader.next() {
      Some('<') => {
        let component_sign_type = component_sign_type(sign_type);
        let component_sign_components = component_sign_components(sign_type);
        let tct = TLTokenComponentType::builder()
          .sign_type(component_sign_type)
          .components(arg_component_types(component_sign_components))
          .build();
        rets.push(tct);
      }
      Some(',') => {
        // tl schema not support `,` , so don't have multi component type (like map<string, string>). nothing to do.
      }
      Some(ch) => {
        builder.append(ch);
      }
      None => {}
    };
  };

  rets
}

/// parse component sign type     vec<vec<string>>    -> vec
fn component_sign_type<S: AsRef<str>>(sign_type: S) -> String {
  let sign_type = sign_type.as_ref();
  let chs = sign_type.chars().collect::<Vec<char>>();
  let ix = chs.iter().enumerate()
    .find(|(_, &ch)| ch == '<')
    .map(|(ix, _)| ix)
    .map_or(chs.len(), |v| v);
  sign_type.chars().take(ix).collect()
}

/// parse component sign type components   vec<vec<string>> -> vec<string>
fn component_sign_components<S: AsRef<str>>(sign_type: S) -> String {
  let sign_type = sign_type.as_ref();
  let chs = sign_type.chars().collect::<Vec<char>>();
  let ix = chs.iter().enumerate()
    .find(|(_, &ch)| ch == '<')
    .map(|(ix, _)| ix)
    .map_or(chs.len(), |v| v);
  sign_type.chars().skip(ix + 1).take(chs.len() - (ix + 2)).collect()
}


/// parse group name
fn group_name(code: &String) -> Result<String, Error> {
  let words: Vec<&str> = code.split(" ").collect();
  match words.get(0) {
    Some(t) => Ok(t.to_string()),
    None => bail!("Not found group name"),
  }
}

/// parse group blood
fn group_blood(code: &String) -> Result<String, Error> {
  let words: Vec<&str> = code.split(" ").collect();
  let mut entry = false;
  let mut bloods = vec![];
  for word in words {
    if !entry && word == "=" {
      entry = true;
      continue;
    }
    if entry {
      bloods.push(word);
    }
  }
  Ok(bloods.join(" "))
}

/// parse group description
fn group_description(description_text: String) -> Option<HashMap<String, String>> {
  if description_text.is_empty() {
    return None;
  }
  Some(tl_description_map(description_text))
}


fn tl_description_map<S: AsRef<str>>(text: S) -> HashMap<String, String> {
  let mut description_map = HashMap::new();
  let dwords: Vec<&str> = text.as_ref().split(" ").collect();

  let mut name = None;
  let mut vvec = Vec::with_capacity(dwords.len());
  for word in dwords {
    if word.is_empty() { continue; }
    if !word.starts_with("@") {
      vvec.push(if word.starts_with("-") { word[1..].to_string() } else { word.to_string() });
      continue;
    }
    if let Some(n) = name {
      description_map.insert(n, vvec.join(" "));
      vvec.clear();
    }
    name = Some(word[1..].to_string());
  }
  description_map.insert(name.expect("Impossible error"), vvec.join(" "));
  vvec.clear();
  description_map
}



