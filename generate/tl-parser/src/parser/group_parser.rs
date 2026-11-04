use failure::Error;
use text_reader::TextReader;

use crate::types::*;

/// parse tl schema text to tl grammar group
pub fn parse<S: AsRef<str>>(schema: S) -> Result<Vec<Box<TLGrammar>>, Error> {
  parse_group_by_line(schema)
}


/// parse group by line of schema text
fn parse_group_by_line<S: AsRef<str>>(schema: S) -> Result<Vec<Box<TLGrammar>>, Error> {
  let schema = schema.as_ref();
  if schema.is_empty() { return bail!("tl schema is empty"); }
  let schlines: Vec<&str> = schema.split("\n").collect();
  let mut grammars: Vec<Box<TLGrammar>> = vec![];
  let mut group_lines: Vec<TLGroupLine> = vec![];

  let mut ix = 0;
  for schline in schlines {
    ix += 1;
    if schline.is_empty() {
      continue;
    }
    // paragraph
    if schline.starts_with("---") {
      let linet = schline.replace("-", "");
      match &linet.to_lowercase()[..] {
        "functions" => grammars.push(Box::new(TLParagraph::Functions { start: ix, end: ix })),
        _ => return bail!("Unsupport paragraph token -> {} line -> [{}] line_text -> {}", linet, ix, schline)
      }
      continue;
    }
    let chs = schline.chars().collect::<Vec<char>>();
    let linet: String = schline.chars().skip(2).take(chs.len()).collect();
    // trait
    if schline.starts_with("//@class ") {
      grammars.push(Box::new(TLGroup::builder()
        .start(ix)
        .end(ix)
        .lines(vec![TLGroupLine::builder().line(ix).token(TLGroupLineToken::Trait).text(linet).build()])
        .build()));
      continue;
    }

    // description
    if schline.starts_with("//") {
      group_lines.push(TLGroupLine::builder().line(ix).token(TLGroupLineToken::Description).text(linet).build());
      continue;
    }

    // group end
    let linet: String = if schline.ends_with(";") {
      schline.chars().take(chs.len() - 1).collect()
    } else {
      schline.to_string()
    };
    group_lines.push(TLGroupLine::builder().line(ix).token(TLGroupLineToken::Struct).text(linet).build());

    // group line end
    grammars.push(Box::new(TLGroup::builder()
      .start(group_lines.get(0).map_or(0, |v| v.line))
      .end(ix)
      .lines(group_lines.clone())
      .build()));
    group_lines.clear();
  }

  Ok(grammars)
}


///// parse tl group use text reader
///// text reader parse with all character, poor performance
//#[doc(hidden)]
//fn parse_group_use_text_reader<S: AsRef<str>>(schema: S) -> Result<Vec<Box<TLGrammar>>, Error> {
//  let mut reader = TextReader::new(schema.as_ref());
//
//  let mut grammars: Vec<Box<TLGrammar>> = vec![];
//  let mut group_lines: Vec<TLGroupLine> = vec![];
//
//  while reader.has_next() {
//    match reader.next() {
//      Some('/') => {
//        if reader.cursor() != 1 {
//          continue;
//        }
//        let mut detector = reader.detector();
//        if detector.next_text("/@class ").no() {
//          detector.rollback();
//          continue;
//        }
//        // is class(trait) line, add group
//        let group = TLGroup::builder()
//          .start(reader.line() as i32)
//          .end(reader.line() as i32)
//          .lines(vec![
//            TLGroupLine::builder()
//              .line(reader.line() as i32)
//              .token(TLGroupLineToken::Trait)
//              .text(reader.this_line().expect("Impossible error"))
//              .build()
//          ])
//          .build();
//        grammars.push(Box::new(group));
//
//
//        // skip to next line
//        while reader.has_next() {
//          // skip this line
//          if reader.next() != Some('\n') {
//            continue;
//          }
//          // the next not first char is '\n' line
//          while reader.has_next() {
//            if reader.next() != Some('\n') {
//              reader.back();
//              break;
//            }
//          }
//          break;
//        }
//      }
//      Some(';') => {
//        let line = reader.this_line();
//        if line.is_none() { return bail!("tl schema fail line -> [{}] , line_text -> {:?}", reader.line(), reader.this_line()); }
//        let line_text = line.unwrap();
//        let mut line = line_text.chars();
//        // if current char is ; and this line first char is not / , group end
//        if line.next() != Some('/') {
//          group_lines.push(TLGroupLine::builder()
//            .line(reader.line() as i32)
//            .token(TLGroupLineToken::Description)
//            .text(reader.this_line().expect("Impossible error"))
//            .build());
//          let gpsfirst = group_lines.get(0);
//          if gpsfirst.is_none() {
//            return bail!("tl schema fail line -> [{}] line_text -> {}", reader.line(), line_text);
//          }
//          let gpsfirst = gpsfirst.unwrap();
//
//          let group = TLGroup::builder()
//            .start(gpsfirst.line)
//            .end(reader.line() as i32)
//            .lines(group_lines.clone())
//            .build();
//          grammars.push(Box::new(group));
//          group_lines.clear();
//
//          // skip multi \n flag
//          while reader.has_next() {
//            if reader.next() != Some('\n') {
//              reader.back();
//              break;
//            }
//          }
//          continue;
//        }
//      }
//      Some('-') => {
//        if reader.cursor() != 1 {
//          continue;
//        }
//        let line = reader.this_line().expect("Impossible error");
//        if line.starts_with("---") && line.ends_with("---") {
//          let line = line.replace("---", "");
//          let line_number = reader.line() as i32;
//          match &line.to_lowercase()[..] {
//            "functions" => grammars.push(Box::new(TLParagraph::Functions { start: line_number, end: line_number })),
//            _ => return bail!("Unsupport paragraph token -> {} line -> [{}] line_text -> {}", line, reader.line(), line)
//          }
//
//          // skip to next line
//          while reader.has_next() {
//            // skip this line
//            if reader.next() != Some('\n') {
//              continue;
//            }
//            // the next not first char is '\n' line
//            while reader.has_next() {
//              if reader.next() != Some('\n') {
//                reader.back();
//                break;
//              }
//            }
//            break;
//          }
//          continue;
//        }
//        // faild token
//        return bail!("tl schema fail line -> [{}] line_text -> {}", reader.line(), line);
//      }
//      Some('\n') => {
//        // back a char
//        reader.back();
//        let line = reader.line();
//        let line_text = reader.this_line();
//
//        // skip multi \n flag
//        while reader.has_next() {
//          if reader.next() != Some('\n') {
//            reader.back();
//            break;
//          }
//        }
//        group_lines.push(TLGroupLine::builder()
//          .line(line as i32)
//          .token(TLGroupLineToken::Struct)
//          .text(line_text.expect("Impossible error"))
//          .build());
//      }
//      Some(ch) => {}
//      None => {}
//    }
//  }
//
//  Ok(grammars)
//}


