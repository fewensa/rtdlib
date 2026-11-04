use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use case::CaseExt;
use colored::Colorize;
use tera::Context;

use tl_parser::types::TLTokenGroup;

use crate::Cycle;
use crate::tokenwrap::TokenWrap;
use std::collections::HashMap;

pub struct RTD<'a> {
  cycle: &'a Cycle,
}

impl<'a> RTD<'a> {
  pub fn new(cycle: &'a Cycle) -> Self {
    Self { cycle }
  }


  pub fn generate(&self) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let path_template: PathBuf = config.path_template().join("rtdlib");

    if !path_template.is_dir() {
      return bail!("RTD template path is not dir -> {:?}", path_template);
    }

    self.clearance()?;

    // move root path file
    self.copy_file_to(&path_template, config.path_rtd())?;
    // generate src file
    self.gensrc(&path_template)?;
    Ok(())
  }

  fn clearance(&self) -> Result<(), failure::Error> {
    let base_dir = self.cycle.config().path_rtd();
    let src = base_dir.join("src");
    if src.exists() {
      std::fs::remove_dir_all(&src)?;
    }
    std::fs::create_dir_all(src)?;
    Ok(())
  }

  /// copy template file to target path
  fn copy_file_to<P: AsRef<Path>>(&self, from_dir: P, to_dir: P) -> Result<(), failure::Error> {
    let from_dir = from_dir.as_ref();
    let to_dir = to_dir.as_ref();
    if !to_dir.exists() {
      std::fs::create_dir_all(to_dir)?;
    }
    let rtd_read_dir: Vec<Result<DirEntry, std::io::Error>> = from_dir.read_dir()?.collect();
    for entry in rtd_read_dir {
      let entry_path = entry?.path();
      if !entry_path.is_file() { continue; }
      let file_name = match entry_path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => return bail!("Can not read file name")
      };
      let copy_to = to_dir.join(file_name);
      debug!("COPY {} -> {}", entry_path.to_str().map_or("", |v| v).blue(), copy_to.to_str().map_or("", |v| v).blue());
      std::fs::copy(entry_path, copy_to)?;
    }
    Ok(())
  }

  /// generate src rs file.
  fn gensrc<P: AsRef<Path>>(&self, path_template: P) -> Result<(), failure::Error> {
    // let config = self.cycle.config();
    // let path_template = path_template.as_ref();
    //
    // let templatesrc: PathBuf = path_template.join("src");
    // let rtdsrc: PathBuf = config.path_rtd().join("src");
    // // copy src path rs file to target dir
    // self.copy_file_to(&templatesrc, &rtdsrc)?;
    self.copy_rs(path_template)?;

    // generate common rs
    self.gen_common()?;

    self.gen_types()?;

    Ok(())
  }

  /// generate common rs file
  fn gen_common(&self) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let tknwrap = self.cycle.tknwrap();

    let mut context = Context::new();
    let tokens = tknwrap.tokens();

    let mut file_obj_map = HashMap::new();
    for token in tokens {
      if tknwrap.is_skip_type(token.name()) { continue }
      let file_name = tknwrap.which_file(token.name());
      let mut vec_of_file_obj = file_obj_map.get(&file_name).map_or(vec![], |v: &Vec<TLTokenGroup>| v.clone());
      vec_of_file_obj.push(token.clone());
      file_obj_map.insert(file_name, vec_of_file_obj);
    }
    context.insert("file_obj_map", &file_obj_map);
    context.insert("tokens", tokens);



    self.cycle.renderer().render("rtdlib/src/types/_common.rs",
                                 config.path_rtd().join("src/types/_common.rs"),
                                 &mut context)?;
    self.cycle.renderer().render("rtdlib/src/types/mod.rs",
                                 config.path_rtd().join("src/types/mod.rs"),
                                 &mut context)?;
    Ok(())
  }

  /// generate types
  fn gen_types(&self) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let tknwrap = self.cycle.tknwrap();

    let mut context = Context::new();
    let tokens = tknwrap.tokens();
    for token in tokens {
      if tknwrap.is_skip_type(token.name()) { continue }
      let file_name = tknwrap.which_file(token.name());
      context.insert("token", token);
      self.cycle.renderer().render("rtdlib/src/types/td_type.rs",
                                   config.path_rtd().join(&format!("src/types/{}.rs", file_name)[..]),
                                   &mut context)?;
    }
    Ok(())
  }

  fn copy_rs<P: AsRef<Path>>(&self, path_template: P) -> Result<(), failure::Error> {
    let path_template = path_template.as_ref();
    let base_dir = self.cycle.config().path_rtd();
    let wait_copies: Vec<(PathBuf, PathBuf)> = vec![
      (path_template.join("README.md"), base_dir.join("README.md")),
      (path_template.join("src/lib.rs"), base_dir.join("src/lib.rs")),
      (path_template.join("src/errors.rs"), base_dir.join("src/errors.rs")),
    ];

    for (from, to) in wait_copies {
      debug!("COPY {} -> {}", from.to_str().map_or("", |v| v).blue(), to.to_str().map_or("", |v| v).blue());
      if let Some(parent) = to.parent() {
        if !parent.exists() {
          std::fs::create_dir_all(parent)?;
        }
      }
      std::fs::copy(from, to)?;
    }

    Ok(())
  }
}



