use std::collections::HashMap;
use std::fs;

use rstring_builder::StringBuilder;
use scraper::{ElementRef, Html, Selector};
use text_reader::TextReader;

use crate::{bakit, bog};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum FieldINF {
  Type,
  Name,
  Description,
  IsTrait,
}

impl FieldINF {
  pub fn string(&self) -> &'static str {
    match self {
      FieldINF::Type => "type",
      FieldINF::Name => "name",
      FieldINF::Description => "description",
      FieldINF::IsTrait => "is_trait",
    }
  }
}

pub struct Apipe {
  czs: Vec<(String, String)>
}

impl Apipe {
  pub fn new(czs: Vec<(String, String)>) -> Self {
    Self { czs }
  }

  fn exists_name<S: AsRef<str>>(&self, name: S) -> bool {
    !self.czs.iter()
      .filter(|(cname, _)| cname.to_lowercase() == name.as_ref().to_lowercase())
      .map(|(_, _)| 0)
      .collect::<Vec<(_)>>()
      .is_empty()
  }

  fn path<S: AsRef<str>>(&self, name: S) -> Option<String> {
    self.czs.iter()
      .filter(|(cname, _)| name.as_ref().to_lowercase() == cname.to_lowercase())
      .map(|(_, cpath)| cpath)
      .collect::<Vec<&String>>()
      .first()
      .map(|&cpath| cpath.clone())
  }

  fn document<S: AsRef<str>>(&self, name: S) -> Option<Html> {
    let path = match self.path(name) {
      Some(path) => path,
      None => return None
    };
    let content = fs::read_to_string(&path[..]).unwrap();
    Some(Html::parse_fragment(&content[..]))
  }


  pub fn names(&self) -> Vec<String> {
    self.czs.iter()
      .map(|(cname, _)| cname.clone())
      .collect::<Vec<String>>()
  }

  pub fn is_trait<S: AsRef<str>>(&self, name: S) -> bool {
    let lowercase_text = match self.description(name) {
      Some(text) => text.to_lowercase(),
      None => return false
    };

    lowercase_text.contains("this class is a base class") ||
      lowercase_text.contains("this class is an abstract base class")
  }

  pub fn description<S: AsRef<str>>(&self, name: S) -> Option<String> {
    if !self.exists_name(&name) {
      return None;
    }

    let selector_description = Selector::parse(".textblock").unwrap();
    let ret = self.document(name).map(|doc| {
      let ele = match doc.select(&selector_description).next() {
        Some(ele) => ele,
        None => return None
      };
      match ele.text().next() {
        Some(text) => Some(text.to_string()),
        None => None
      }
    });
    bakit::flatten_option(ret)
  }

  pub fn father_class<S: AsRef<str>>(&self, name: S) -> Option<String> {
    if !self.exists_name(&name) {
      return None;
    }

    let selector_inherits = Selector::parse(".el").unwrap();
    let ret = self.document(name).map(|doc| {
      let ele = match doc.select(&selector_inherits).nth(1) {
        Some(ele) => ele,
        None => return None
      }; // super class
      match ele.text().next() {
        Some(text) => Some(text.to_string()),
        None => None
      }
    });
    bakit::flatten_option(ret)
  }

  pub fn sub_classes<S: AsRef<str>>(&self, name: S) -> Option<Vec<String>> {
    if !self.exists_name(&name) {
      return None;
    }
    let selector_inherited = Selector::parse(".contents p").unwrap();
    let selector_subel = Selector::parse(".el").unwrap();


    let subclasses = self.document(name)
      .map(|doc| {
        let eles = doc.select(&selector_inherited)
          .filter(|eref| self::ele_text(eref).to_lowercase().starts_with("inherited by"))
          .collect::<Vec<ElementRef>>();
        let ele = eles.first();
        if None == ele {
          return None;
        }
        let ele = ele.unwrap();
        let mut rets = vec![];
        ele.select(&selector_subel).for_each(|subclzele| {
          let text = self::ele_text(&subclzele);
          rets.push(toolkit::text::uppercase_first_char(text));
        });
        Some(rets)
      });
    bakit::flatten_option(subclasses)
  }

  pub fn fields<S: AsRef<str>>(&self, name: S) -> Vec<HashMap<FieldINF, String>> {
    let rets = vec![];
    if !self.exists_name(&name) {
      return rets;
    }
    let selector_mems = Selector::parse(".memberdecls").unwrap();
    let selector_heading = Selector::parse(".heading").unwrap();
    let selector_tr = Selector::parse("tr").unwrap();
    let doc = match self.document(name) {
      Some(doc) => doc,
      None => return rets
    };

    // get all public fields
    let pf = doc.select(&selector_mems).filter(|item| {
      let text = match item.select(&selector_heading).next() {
        Some(ele) => match ele.text().next() {
          Some(text) => text,
          None => return false
        },
        None => return false
      };
      text.to_lowercase().contains("public fields")
    }).collect::<Vec<ElementRef>>();
    let pf = match pf.first() {
      Some(pf) => pf,
      None => return rets
    };


    let mut tdpf = HashMap::new();

    pf.select(&selector_tr).map(|tr| {
      let ele = tr.value();
      let css_class = ele.attr("class");
      if let None = css_class {
        return None;
      }
      let css_class = css_class.unwrap();
      if "heading" == css_class {
        return None;
      }

      if css_class.starts_with("memitem") {
        let field_name = self::rs_field_name(self::ele_text_rule(&tr, ".memItemRight"));
        let field_type = self::rs_type(self::ele_text_rule(&tr, ".memItemLeft"));
        let is_trait = self::field_is_trait(self, &field_type);
        tdpf.insert(FieldINF::Name, field_name);
        tdpf.insert(FieldINF::IsTrait, if is_trait { 1 } else { 0 }.to_string());
        tdpf.insert(FieldINF::Type, self::box_trait_field_type(self, field_type, is_trait));
        return None;
      }
      if css_class.starts_with("memdesc") {
        tdpf.insert(FieldINF::Description, self::ele_text_rule(&tr, ".mdescRight"));
        bog::info(format!("FOUND FIELD => {:?}", tdpf));
        let s = Some(tdpf.clone());
        tdpf.clear();
        return s;
      }
      None
    })
      .filter(|item| item.is_some())
      .map(|item| item.unwrap())
      .collect::<Vec<HashMap<FieldINF, String>>>()
  }
}


// safe trait field type
fn box_trait_field_type<S: AsRef<str>>(apipe: &Apipe, field_type: S, is_trait: bool) -> String {
  let field_type = field_type.as_ref().to_string();
  if !is_trait { return field_type; }

  if !field_type.contains("<") { return format!("Box<{}>", field_type); }

  let chs = self::type_items(field_type);

  chs.iter().map(|item| {
    if apipe.is_trait(item) {
      return format!("Box<{}>", item);
    }
    item.clone()
  })
    .collect::<Vec<String>>()
    .join("")
}

fn field_is_trait<S: AsRef<str>>(apipe: &Apipe, field_type: S) -> bool {
  let field_type = field_type.as_ref();
  if !field_type.contains("<") {
    return apipe.is_trait(field_type);
  }

  let chs = self::type_items(field_type);

  for text in chs {
    if text == "<" || text == ">" || text.to_lowercase() == "vec" { continue; }
    if apipe.is_trait(text) { return true; }
  }
  false
}

fn type_items<S: AsRef<str>>(field_type: S) -> Vec<String> {
  let field_type = field_type.as_ref();
  let mut chs = vec![];
  let mut builder = StringBuilder::new();
  let mut reader = TextReader::new(field_type);

  while reader.has_next() {
    match reader.next() {
      Some('<') => {
        chs.push(builder.string());
        chs.push("<".to_string());
        builder.clear();
        continue;
      }
      Some('>') => {
        chs.push(builder.string());
        chs.push(">".to_string());
        builder.clear();
        continue;
      }
      Some(ch) => {
        builder.append(ch);
        continue;
      }
      None => continue
    }
  }
  chs
}

fn ele_text_rule(ele: &ElementRef, rule: &'static str) -> String {
  let mut chs: Vec<char> = Vec::new();
  ele.select(&Selector::parse(rule).unwrap())
    .for_each(|ele| {
      ele.text().for_each(|t| chs.extend(t.chars().into_iter()))
    });
  let text: String = chs.iter().collect();
  text.trim().to_string().replace("&quot;", "\"")
}

fn ele_text(ele: &ElementRef) -> String {
  let mut chs: Vec<char> = Vec::new();
  ele.text().for_each(|t| chs.extend(t.chars().into_iter()));
  let text: String = chs.iter().collect();
  text.trim().to_string().replace("&quot;", "\"")
}


fn rs_type(type_: String) -> String {
  let mut type_ = type_.replace("std::int32_t", "i32");
  type_ = type_.replace("std::int64_t", "i64");
  type_ = type_.replace("std::string", "String");
  type_ = type_.replace("std::vector", "Vec");
  type_ = type_.replace("double", "f64");
  if type_.contains("object_ptr") {
    let mut builder = StringBuilder::new();
    let mut reader = TextReader::new(type_.clone());
    while reader.has_next() {
      match reader.next() {
        Some(ch) => {
          builder.append(ch);
          if !builder.string().ends_with("object_ptr") {
            continue;
          }
          let styp_ = builder.string().replace("object_ptr", "");
          builder = StringBuilder::new();
          builder.append(styp_);
          let mut first = true;
          while reader.has_next() {
            match reader.next() {
              Some('<') => continue,
              Some(' ') => continue,
              Some('>') => break,
              Some(ch) => {
                if first {
                  builder.append(ch.to_uppercase().to_string());
                  first = false;
                  continue;
                }
                builder.append(ch);
              }
              None => continue,
            };
          }
          while reader.has_next() {
            match reader.next() {
              Some(ch) => builder.append(ch),
              None => continue,
            };
          }
          type_ = builder.string();
        }
        None => continue
      }
    }
  }
  type_ = type_.replace(" ", "");
  return type_;
}

fn rs_field_name(name: String) -> String {
  if "type_" == &name[..] {
    return name;
  }
  if name.ends_with("_") {
    let chars = name.chars().rev();
    let mut end = false;
    let mut retchs = Vec::new();
    for ch in chars {
      if end {
        retchs.push(ch);
        continue;
      }
      if ch == '_' {
        continue;
      }
      end = true;
      retchs.push(ch);
    }
    return retchs.iter().rev().collect();
  }
  name
}



