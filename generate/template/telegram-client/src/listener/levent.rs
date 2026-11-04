use rtdlib::types::*;
use std::sync::Arc;

use crate::api::Api;
use crate::errors::*;

/// Telegram client event listener
#[derive(Clone, Default)]
pub struct EventListener {
  exception: Option<Arc<dyn Fn((&Api, &TGError)) + Send + Sync + 'static>>,
  receive: Option<Arc<dyn Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static>>,

{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}  {{name | to_snake}}: Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>>,
{% endfor %}

{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}  {{token.name  | to_snake}}: Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>>,
{% endif %}{% endfor %}

{% for token in tokens %}{% if token.is_return_type %}  {{token.name | to_snake}}: Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>>,
{% endif %}{% endfor %}
}


impl EventListener {
  pub fn new() -> Self { EventListener::default() }

  #[allow(dead_code)]
  pub(crate) fn has_receive_listen(&self) -> bool { self.receive.is_some() }

  pub(crate) fn lout(&self) -> EventLout { EventLout::new(self.clone()) }


  /// when receive data from tdlib
  pub fn on_receive<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static {
    self.receive = Some(Arc::new(fnc));
    self
  }

  /// when telegram client throw exception
  pub fn on_exception<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &TGError)) + Send + Sync + 'static {
    self.exception = Some(Arc::new(fnc));
    self
  }

{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
  /// {{token.description}}
  pub fn on_{{name | to_snake}}<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static {
    self.{{name}} = Some(Arc::new(fnc));
    self
  }
{% endfor %}



{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
  /// {{token.description}}
  pub fn on_{{token.name  | to_snake}}<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static {
    self.{{token.name  | to_snake}} = Some(Arc::new(fnc));
    self
  }
{% endif %}{% endfor %}

{% for token in tokens %}{% if token.is_return_type %}
  /// {{token.description}}
  pub fn on_{{token.name | to_snake}}<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static {
    self.{{token.name | to_snake}} = Some(Arc::new(fnc));
    self
  }
{% endif %}{% endfor %}
}


/// Get listener
#[derive(Clone)]
pub struct EventLout {
  listener: EventListener,
  supports: Vec<&'static str>
}

impl EventLout {
  fn new(listener: EventListener) -> Self {
    let supports = vec![
{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}      "{{token.name | to_snake | to_camel_lowercase}}",
{% endfor %}

{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}      "{{token.name}}",
{% endif %}{% endfor %}

{% for token in tokens %}{% if token.is_return_type %}      "{{token.name}}",
{% endif %}{% endfor %}

    ];
    Self { listener, supports }
  }

  pub fn is_support<S: AsRef<str>>(&self, name: S) -> bool {
    self.supports.iter()
      .find(|&&item| item == name.as_ref())
      .is_some()
  }

  pub fn handle_type(&self, api: &Api, td_type: &TdType) -> TGResult<bool>  {
    match td_type {
{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
    TdType::{{token.name | to_camel}}(value) => match &self.listener.{{name | to_snake}} {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },
{% endfor %}
{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
    TdType::{{token.name | to_camel}}(value) => match &self.listener.{{token.name | to_snake}} {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },
{% endif %}{% endfor %}
{% for token in tokens %}{% if token.is_return_type %}
    TdType::{{token.name | to_camel}}(value) => match &self.listener.{{token.name | to_snake}} {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },
{% endif %}{% endfor %}

  }
  }

  /// when telegram client throw exception
  pub fn exception(&self) -> &Option<Arc<dyn Fn((&Api, &TGError)) + Send + Sync + 'static>> {
    &self.listener.exception
  }

  /// when receive data from tdlib
  pub fn receive(&self) -> &Option<Arc<dyn Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.receive
  }

{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
  /// {{token.description}}
  pub fn {{name | to_snake}}(&self) -> &Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.{{name | to_snake}}
  }
{% endfor %}


{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
  /// {{token.description}}
  pub fn {{token.name  | to_snake}}(&self) -> &Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.{{token.name  | to_snake}}
  }
{% endif %}{% endfor %}

{% for token in tokens %}{% if token.is_return_type %}
  /// {{token.description}}
  pub fn {{token.name | to_snake}}(&self) -> &Option<Arc<dyn Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.{{token.name | to_snake}}
  }
{% endif %}{% endfor %}

}


