use rstring_builder::StringBuilder;
use text_reader::TextReader;

fn remove_all_struct<S: AsRef<str>>(json: S) -> String {
  let json = json.as_ref();

  let mut builder = StringBuilder::new();
  let mut reader = TextReader::new(json);
  while reader.has_next() {
    match reader.next() {
      Some('"') => {
        let mut last_coma = false;
        if let Some(',') = reader.back().peek() {
          last_coma = true;
        }
        reader.next();
        let mut detector = reader.detector();
        if detector.next_text("@struct\"").no() {
          builder.append('"');
          continue;
        }

        let mut times = 0;
        while reader.has_next() {
          match reader.next() {
            Some('"') => {
              times += 1;
              if times == 2 {
                break;
              }
            },
            Some(ch) => {},
            None => {}
          }
        }

        let need_rm_last_coma = match reader.next() {
          Some(',') => false,
          Some('}') => {
            reader.back();
            true
          },
          Some(']') => {
            reader.back();
            true
          },
          _ => panic!("Json fail")
        };
        if need_rm_last_coma {
          builder.delete_at(builder.len() - 1);
        }
      }
      Some(ch) => {
        builder.append(ch);
        continue;
      }
      None => {}
    }
  }

  builder.string()
}


/// fill @struct key to json string.
pub fn fill_json_struct<S: AsRef<str>>(json: S) -> String {
  let json = self::remove_all_struct(json.as_ref());

  let mut builder = StringBuilder::new();
  let mut reader = TextReader::new(json);

  while reader.has_next() {
    match reader.next() {
      Some('"') => {
        builder.append('"');
        let mut detector = reader.detector();
        if detector.next_text("@type\"").no() {
          continue;
        }
        builder.append("@type\"");
        let mut fnbuilder = StringBuilder::new();
        let mut times = 0;

        while reader.has_next() {
          match reader.next() {
            Some(':') => {
              builder.append(':');
              while reader.has_next() {
                match reader.next() {
                  Some('"') => {
                    reader.back();
                    break
                  }
                  Some(ch) => {
                    builder.append(ch);
                    continue
                  },
                  None => continue
                }
              }
              continue;
            }
            Some('"') => {
              match times {
                0 => {
                  builder.append("\"");
                  times += 1;
                  continue;
                }
                1 => {
                  builder.append(fnbuilder.string())
                    .append("\",")
                    .append(r#""@struct":""#)
                    .append(uppercase_first_char(fnbuilder.string()))
                    .append('"');
                  break;
                }
                _ => {}
              }
              continue;
            }
            Some(ch) => {
              fnbuilder.append(ch);
              continue;
            }
            None => {}
          }
        }
      }
      Some(ch) => {
        builder.append(ch);
        continue;
      }
      None => {}
    }
  }

  builder.string()
}

pub fn extra_struct<S: AsRef<str>>(json: S) -> Option<String> {
  let json = self::fill_json_struct(json);

  let jve = serde_json::from_str::<serde_json::Value>(&json[..]);
  if let Err(e) = jve {
    eprintln!("{} => {:?}", json, e);
    return None;
  }
  let jve = jve.unwrap();
  match jve.get("@struct") {
    Some(t) => {
      if t.is_string() {
        return t.as_str().map(|v| v.to_string());
      }
      None
    }
    None => None
  }


  // Performance comparison

//  let mut builder = StringBuilder::new();
//  let mut reader = TextReader::new(json);
//  while reader.has_next() {
//    match reader.next() {
//      Some('"') => {
//        let mut detector = reader.detector();
//        if detector.next_text("@struct\"").no() {
//          continue;
//        }
//
//        let mut times = 0;
//        while reader.has_next() {
//          match reader.next() {
//            Some(':') => {}
//            Some('"') => {
//              times += 1;
//              if times == 1 {
//                continue;
//              }
//              if times == 2 {
//                return Some(builder.string())
//              }
//            },
//            Some(ch) => {
//              builder.append(ch);
//              continue;
//            },
//            None => {}
//          }
//        }
//      }
//      Some(_) => continue,
//      None => {}
//    }
//  }
//  None
}


pub(crate) fn uppercase_first_char<S: AsRef<str>>(s: S) -> String {
  let mut c = s.as_ref().chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}


#[cfg(test)]
mod rdtest {
  use crate::types::{RObject, UpdateAuthorizationState};

  #[test]
  fn test_fill_json_struct() {
    let json = r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#;
    let json = super::fill_json_struct(json);
    assert_eq!(json, r#"{"@type":"updateAuthorizationState","@struct":"UpdateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@struct":"AuthorizationStateWaitTdlibParameters"}}"#);
    let state: UpdateAuthorizationState = serde_json::from_str(&json[..]).expect("Json fail");
    assert_eq!("updateAuthorizationState", state.td_name());
  }

  #[test]
  fn test_fill_lose_struct() {
    let json = r#"{"@type":"updateAuthorizationState","authorization_state":{"@struct":"AuthorizationStateWaitTdlibParameters","@type":"authorizationStateWaitTdlibParameters"}}"#;
    let ret = super::fill_json_struct(json);
    assert_eq!(ret, r#"{"@type":"updateAuthorizationState","@struct":"UpdateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@struct":"AuthorizationStateWaitTdlibParameters"}}"#)
  }

  #[test]
  fn test_extra_struct() {
    let json = r#"{"@type":"updateAuthorizationState","@struct":"UpdateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@struct":"AuthorizationStateWaitTdlibParameters"}}"#;
    let stname = super::extra_struct(json);
    assert!(stname.is_some());
    assert_eq!("UpdateAuthorizationState", stname.unwrap());
  }

  #[test]
  fn test_fill2() {
    let json = r#"{"@type":"updateOption","@struct":"UpdateOption","name":"version","value":{"@type":"optionValueString","@struct":"OptionValueString","value":"1.4.0"}}"#;
    //                  {"@type":"updateOption","@struct":"UpdateOption","name":"version","value":{"@type":"optionValueString","@struct":"OptionValueString","value":"1.4.0"}}
    let ret = super::fill_json_struct(json);
    assert_eq!(json, &ret[..])
  }
}


