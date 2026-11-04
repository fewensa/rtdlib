use futures::future::LocalBoxFuture;
use rtdlib::types::*;
use std::sync::Arc;

use crate::api::Api;
use crate::errors::*;

/// Telegram client event listener
#[derive(Clone, Default)]
pub struct RasyncListener {
  exception: Option<Arc<dyn Send + Sync + Fn((Api, TGError)) -> LocalBoxFuture<'static, ()>>>,
  receive: Option<Arc<dyn Send + Sync + Fn((Api, String)) -> LocalBoxFuture<'static, TGResult<()>>>>,

{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}  {{name | to_snake}}: Option<Arc<dyn Send + Sync + Fn((Api, {{token.name | to_camel}})) -> LocalBoxFuture<'static, TGResult<()>>>>,
{% endfor %}

{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}  {{token.name | to_snake}}: Option<Arc<dyn Send + Sync + Fn((Api, {{token.name | to_camel}})) -> LocalBoxFuture<'static, TGResult<()>>>>,
{% endif %}{% endfor %}

{% for token in tokens %}{% if token.is_return_type %}  {{token.name | to_snake}}: Option<Arc<dyn Send + Sync + Fn((Api, {{token.name | to_camel}})) -> LocalBoxFuture<'static, TGResult<()>>>>,
{% endif %}{% endfor %}
}


impl RasyncListener {
  pub fn new() -> Self { RasyncListener::default() }

  #[allow(dead_code)]
  pub(crate) fn has_receive_listen(&self) -> bool { self.receive.is_some() }

  pub(crate) fn lout(&self) -> RasyncLout { RasyncLout::new(self.clone()) }


  /// when receive data from tdlib
  pub fn on_receive<F>(&mut self, fnc: F) -> &mut Self where F: Send + Sync + Fn((Api, String)) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.receive = Some(Arc::new(fnc));
    self
  }

  /// when telegram client throw exception
  pub fn on_exception<F>(&mut self, fnc: F) -> &mut Self where F: Send + Sync + Fn((Api, TGError)) -> LocalBoxFuture<'static, ()> + 'static {
    self.exception = Some(Arc::new(fnc));
    self
  }

{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
  /// {{token.description}}
  pub fn on_{{name | to_snake}}<F>(&mut self, fnc: F) -> &mut Self where F: Send + Sync + Fn((Api, {{token.name | to_camel}})) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.{{name}} = Some(Arc::new(fnc));
    self
  }
{% endfor %}



{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
  /// {{token.description}}
  pub fn on_{{token.name  | to_snake}}<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, {{token.name | to_camel}})) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.{{token.name  | to_snake}} = Some(Arc::new(fnc));
    self
  }
{% endif %}{% endfor %}

{% for token in tokens %}{% if token.is_return_type %}
  /// {{token.description}}
  pub fn on_{{token.name | to_snake}}<F>(&mut self, fnc: F) -> &mut Self
    where F: Send + Sync + Fn((Api, {{token.name | to_camel}})) -> LocalBoxFuture<'static, TGResult<()>> + 'static {
    self.{{token.name | to_snake}} = Some(Arc::new(fnc));
    self
  }
{% endif %}{% endfor %}
}


/// Get listener
#[derive(Clone)]
pub struct RasyncLout {
  listener: RasyncListener,
  supports: Vec<&'static str>
}

impl RasyncLout {
  fn new(listener: RasyncListener) -> Self {
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

  pub async fn handle_type(&self, api: &Api, td_type: &TdType) -> TGResult<bool>  {
    match td_type {
{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
      TdType::{{token.name | to_camel}}(value) => match &self.listener.{{name | to_snake}} {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },
{% endfor %}
{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
      TdType::{{token.name | to_camel}}(value) => match &self.listener.{{token.name | to_snake}} {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },
{% endif %}{% endfor %}
{% for token in tokens %}{% if token.is_return_type %}
      TdType::{{token.name | to_camel}}(value) => match &self.listener.{{token.name | to_snake}} {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },
{% endif %}{% endfor %}

  }
  }

  /// when telegram client throw exception
  pub fn exception(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, TGError)) -> LocalBoxFuture<'static, ()> + 'static>> {
    &self.listener.exception
  }

  /// when receive data from tdlib
  pub fn receive(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, String)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.receive
  }

{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
  /// {{token.description}}
  pub fn {{name | to_snake}}(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, {{token.name | to_camel}})) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.{{name | to_snake}}
  }
{% endfor %}


{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
  /// {{token.description}}
  pub fn {{token.name  | to_snake}}(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, {{token.name | to_camel}})) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.{{token.name  | to_snake}}
  }
{% endif %}{% endfor %}

{% for token in tokens %}{% if token.is_return_type %}
  /// {{token.description}}
  pub fn {{token.name | to_snake}}(&self) -> &Option<Arc<dyn Send + Sync + Fn((Api, {{token.name | to_camel}})) -> LocalBoxFuture<'static, TGResult<()>> + 'static>> {
    &self.listener.{{token.name | to_snake}}
  }
{% endif %}{% endfor %}

}


