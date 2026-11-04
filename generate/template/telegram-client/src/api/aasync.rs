use futures::StreamExt;
use rtdlib::errors::{RTDError, RTDResult};
use rtdlib::types::*;

use crate::api::Api;
use crate::observer;

macro_rules! async_caller {
  ($td_type:ident) => {
    async fn async_caller<Fnc: RFunction>(api: &Api, fnc: Fnc) -> RTDResult<$td_type> {
      let extra = fnc.extra()
        .ok_or(RTDError::Custom("invalid tdjson response type, not have `extra` field".to_string()))?;
      let mut rec = observer::subscribe(&extra);
      api.send(&fnc)?;
      let val = rec.next().await;
      observer::unsubscribe(&extra);
      if let Some(TdType::Error(v)) = val {
        return Err(RTDError::custom(format!("[{}] {}", v.code(), v.message())));
      }
      match val {
        Some(TdType::$td_type(v)) => { Ok(v) }
        _ => { Err(RTDError::custom("invalid tdjson response type, unexpected `extra` field".to_string())) }
      }
    }
  }
}

#[derive(Clone)]
pub struct AsyncApi {
  api: Api,
}

impl AsyncApi {
  pub fn new(api: Api) -> Self {
    Self { api}
  }

  #[doc(hidden)]
  pub fn api(&self) -> &Api {
    &self.api
  }

{% for token in tokens %}{% if token.type_ == 'Function' %}
  pub async fn {{token.name | to_snake}}<C: AsRef<{{token.name | to_camel}}>>(&self, {{token.name | to_snake}}: C) -> RTDResult<{{token.blood | to_camel}}> {
    async_caller!({{token.blood | to_camel}});
    async_caller(&self.api, {{token.name | to_snake}}.as_ref()).await
  }
{% endif %}{% endfor %}

}
