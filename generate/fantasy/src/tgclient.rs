use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use case::CaseExt;
use colored::Colorize;
use tera::Context;

use tl_parser::types::TLTokenGroup;

use crate::Cycle;
use crate::tokenwrap::TokenWrap;

pub struct TGClient<'a> {
  cycle: &'a Cycle,
}


impl<'a> TGClient<'a> {
  pub fn new(cycle: &'a Cycle) -> Self {
    Self { cycle }
  }


  pub fn generate(&self) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let path_template: PathBuf = config.path_template().join("telegram-client");

    if !path_template.is_dir() {
      return bail!("RTD template path is not dir -> {:?}", path_template);
    }

    self.clearance()?;

    // move root path file
    self.copy_file_to(&path_template, config.path_telegram_client())?;

    // generate src file
    self.gensrc(&path_template)?;

    Ok(())
  }

  fn clearance(&self) -> Result<(), failure::Error> {
    let base_dir = self.cycle.config().path_telegram_client();
    for dir in vec!["src", "src/api"] {
      let path_src = base_dir.join(dir);
      if path_src.exists() {
        std::fs::remove_dir_all(&path_src)?;
      }
      std::fs::create_dir_all(&path_src)?;
    }
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
    self.copy_rs(path_template)?;
    self.gen_api()?;
    self.gen_listener()?;
    self.gen_handler()?;
    self.gen_observer()?;

    Ok(())
  }

  fn gen_observer(&self) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let tknwrap = self.cycle.tknwrap();

    let mut context = Context::new();
    let tokens = tknwrap.tokens();
    context.insert("tokens", tokens);

    let listener: HashMap<&String, &String> = tknwrap.tdtypefill().listener()
      .iter()
      .filter(|(key, value)| {
        tknwrap.tokens().iter()
          .filter(|&token| token.blood() == Some("Update".to_string()))
          .find(|&token| token.name().to_lowercase() == value.to_lowercase())
          .is_none()
      })
      .collect();

    context.insert("listener", &listener);

    self.cycle.renderer().render("telegram-client/src/observer.rs",
                                 config.path_telegram_client().join("src/observer.rs"),
                                 &mut context)?;
    Ok(())
  }


  fn gen_listener(&self) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let tknwrap = self.cycle.tknwrap();

    let mut context = Context::new();
    let tokens = tknwrap.tokens();
    context.insert("tokens", tokens);

    let listener: HashMap<&String, &String> = tknwrap.tdtypefill().listener()
      .iter()
      .filter(|(key, value)| {
        tknwrap.tokens().iter()
          .filter(|&token| token.blood() == Some("Update".to_string()))
          .find(|&token| token.name().to_lowercase() == value.to_lowercase())
          .is_none()
      })
      .collect();

    context.insert("listener", &listener);

    self.cycle.renderer().render("telegram-client/src/listener/lasync.rs",
                                 config.path_telegram_client().join("src/listener/lasync.rs"),
                                 &mut context)?;
    self.cycle.renderer().render("telegram-client/src/listener/levent.rs",
                                 config.path_telegram_client().join("src/listener/levent.rs"),
                                 &mut context)?;
    Ok(())
  }


  fn gen_handler(&self) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let tknwrap = self.cycle.tknwrap();

    let mut context = Context::new();
    let tokens = tknwrap.tokens();

    context.insert("tokens", tokens);
    let listener: HashMap<&String, &String> = tknwrap.tdtypefill().listener()
      .iter()
      .filter(|(key, value)| {
        tknwrap.tokens().iter()
          .filter(|&token| token.blood() == Some("Update".to_string()))
          .find(|&token| token.name().to_lowercase() == value.to_lowercase())
          .is_none()
      })
      .collect();

    context.insert("listener", &listener);

    self.cycle.renderer().render("telegram-client/src/handler.rs",
                                 config.path_telegram_client().join("src/handler.rs"),
                                 &mut context)?;
    Ok(())
  }

  fn gen_api(&self) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let tknwrap = self.cycle.tknwrap();

    let mut context = Context::new();
    let tokens = tknwrap.tokens();
    context.insert("tokens", tokens);

    self.cycle.renderer().render("telegram-client/src/api/aasync.rs",
                                 config.path_telegram_client().join("src/api/aasync.rs"),
                                 &mut context)?;
    self.cycle.renderer().render("telegram-client/src/api/aevent.rs",
                                 config.path_telegram_client().join("src/api/aevent.rs"),
                                 &mut context)?;
    Ok(())
  }


  fn copy_rs<P: AsRef<Path>>(&self, path_template: P) -> Result<(), failure::Error> {
    let path_template = path_template.as_ref();
    let base_dir = self.cycle.config().path_telegram_client();
    let wait_copies: Vec<(PathBuf, PathBuf)> = vec![
      (path_template.join("README.md"), base_dir.join("README.md")),
      // (path_template.join("CHANGELOG.md"), base_dir.join("CHANGELOG.md")),
      (path_template.join("src/lib.rs"), base_dir.join("src/lib.rs")),
      (path_template.join("src/client.rs"), base_dir.join("src/client.rs")),
      (path_template.join("src/rtd.rs"), base_dir.join("src/rtd.rs")),
      (path_template.join("src/rtd.rs"), base_dir.join("src/rtd.rs")),
      (path_template.join("src/tip.rs"), base_dir.join("src/tip.rs")),
      (path_template.join("src/errors.rs"), base_dir.join("src/errors.rs")),
      (path_template.join("src/api/mod.rs"), base_dir.join("src/api/mod.rs")),
      (path_template.join("src/api/api.rs"), base_dir.join("src/api/api.rs")),
      (path_template.join("src/listener/mod.rs"), base_dir.join("src/listener/mod.rs")),
      (path_template.join("src/listener/listener.rs"), base_dir.join("src/listener/listener.rs")),
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
