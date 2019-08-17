use std::fs;
use std::path::{Path, PathBuf};

use tera::{Tera, Context};

use crate::apipe::Apipe;
use crate::filter;
use crate::log;
use crate::parser;
use case::CaseExt;
use rstring_builder::StringBuilder;

pub struct Generator<P: AsRef<Path>> {
  base_path: P
}

impl<P: AsRef<Path>> Generator<P> {
  pub fn new(base_path: P) -> Self {
    Self { base_path }
  }

  pub fn generate(&self, classes: Vec<(String, PathBuf)>) {
    self.generate_code(classes.clone());
  }

  fn rtdpath<PH: AsRef<Path>>(&self, path: PH) -> PathBuf {
    let path = path.as_ref();
    let base_path = self.base_path.as_ref();
    let td_path = base_path.join(path);
    if !td_path.exists() {
      fs::create_dir_all(&td_path).expect(&format!("Can not create {:?} path", path));
    }
    td_path.canonicalize().expect(&format!("Can not get {:?} path", path))
  }

  fn rtdfile<PH: AsRef<Path>, S: AsRef<str>>(&self, path: PH, file: S) -> PathBuf {
    let file = self.rtdpath(path).join(file.as_ref());
    if file.exists() {
      fs::remove_file(&file).expect(&format!("Can not remove {:?} file", file)[..]);
    }
    file
  }

  fn generate_code(&self, classes: Vec<(String, PathBuf)>) {
    let tpl_path = "tpl/**/*";
    log::debug(format!("Template path: {}", tpl_path));
    let mut tera = Tera::new(tpl_path).expect("Can not create Tera template engine.");
    tera.register_filter("field_type", filter::filter_field_type);

    let apipe = Apipe::new(classes);

    let base_path = self.base_path.as_ref();
    let file_supplement = self.rtdfile("src", "tdsupplement.rs");
    let file_common = self.rtdfile("src/types","_common.rs");
    let file_schema = self.rtdfile("schema", "schema.toml");
    let file_rtdmod = self.rtdfile("src/types", "mod.rs");

    let mut modbuilder = StringBuilder::new();

    // supplement
    let td_supplement = parser::td_supplement(&apipe);
    let mut context_supplement = Context::new();
    context_supplement.insert("sp", &td_supplement);
    render(&tera, "tdsupplement.tpl.txt", &context_supplement, &file_supplement, "rust");

    // common
    let td_common = parser::td_common(&apipe);
    let mut context_common = Context::new();
    context_common.insert("common", &td_common);
    render(&tera, "rtd_common.tpl.txt", &context_common, &file_common, "rust");

    modbuilder.append("#[macro_use] mod _common;\n")
      .append("pub use self::_common::*;\n\n\n\n");

    // schema
    render(&tera, "schema.common.tpl.toml", &context_common, &file_schema, "toml");


    // rtd types
    let td_all_types = parser::td_all_types(&apipe);

    td_all_types.iter().for_each(|td| {
      let snake_clz_name = td.clz_name.clone().to_snake();
      modbuilder.append("pub use self::").append(snake_clz_name).append("::").append(td.clz_name.clone()).append(";\n");
    });
    modbuilder.append("\n\n\n");
    td_all_types.iter().for_each(|td| {
      let snake_clz_name = td.clz_name.to_snake();
      modbuilder.append("mod ").append(snake_clz_name).append(";\n");
    });
    td_all_types.iter().for_each(|td| {
      let mut context = Context::new();
      context.insert("td", &td);
      let snake_clz_name = td.clz_name.to_snake();
      // rtd type
      let file_clz = self.rtdfile("src/types", format!("{}.rs", snake_clz_name));
      render(&tera, "tdfn.tpl.txt", &context, &file_clz, "rust");

      // schema
      render(&tera, "schema.rtd.tpl.toml", &context, &file_schema, "toml");
    });

    fs::write(file_rtdmod, modbuilder.string()).expect("Can not write mod.rs");

  }

  fn generate_schema(&self, classes: Vec<(String, PathBuf)>) {}
}


fn render<P: AsRef<Path>>(tera: &Tera, tpl: &str, context: &Context, write_to: P, code_type: &str) {
  let body = tera.render(tpl, &context).unwrap();
  log::info(format!("Append code to {} ```{}\\n{}\\n```",
                    write_to.as_ref().to_str().unwrap(),
                    code_type,
                    body.trim().replace("\n", "\\n"))
  );
  toolkit::fs::append(write_to, &body[..]).unwrap();
}


