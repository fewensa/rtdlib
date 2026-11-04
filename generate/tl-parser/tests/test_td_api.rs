use std::path::Path;

use failure::Error;

use tl_parser::parser::parser::TLParser;
use tl_parser::types::TLTokenGroup;

fn tokens() -> Result<Vec<TLTokenGroup>, Error> {
  let path = Path::new("../").join("schema/master/td_api.tl");
  TLParser::new(path).parse()
}


#[test]
fn test_td() {
  let tl = self::tokens();
  assert!(tl.is_ok(), true);
  let tokens = tl.unwrap();
  let len = tokens.len();
  assert!(len > 0, true);
  tokens.iter().enumerate().for_each(|(ix, token)| {
    match ix {
      0 => {
        assert_eq!(token.name(), "double".to_string());
        assert_eq!(token.blood(), Some("Double".to_string()));
      }
      1 => {
        assert_eq!(token.name(), "string".to_string());
        assert_eq!(token.blood(), Some("String".to_string()));
      }
      1096 => {
        assert_eq!(token.name(), "testUseError".to_string());
        assert_eq!(token.blood(), Some("Error".to_string()));
      }
      _ => {}
    }
  });
}


