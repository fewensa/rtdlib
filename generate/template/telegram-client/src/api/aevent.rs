use rtdlib::errors::*;
use rtdlib::types::*;
use crate::api::Api;


#[derive(Debug, Clone)]
pub struct EventApi {
  api: Api,
}

impl EventApi {
  pub fn new(api: Api) -> Self {
    Self { api }
  }

  #[doc(hidden)]
  pub fn api(&self) -> &Api {
    &self.api
  }

{#
  // now don't know which function is synchronously function, so, not use this block.
{% for token in tokens %}{% if token.type_ == 'Function' %}
  pub fn {{token.name | to_snake}}<C: AsRef<{{token.name | to_camel}}>>(&self, {{token.name | to_snake}}: C) -> {% if token.blood and token.blood == 'Ok' %}RTDResult<{{token.blood}}>{% else %}RTDResult<()>{% endif %} {
    {% if token.blood and token.blood == 'Ok' %}
    match self.execute({{token.name | to_snake}}.as_ref())? {
      Some(json) => Ok({{token.blood}}::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
    {% else %}  self.api.send({{token.name | to_snake}}.as_ref()){% endif %}
  }
{% endif %}{% endfor %}
#}

{% for token in tokens %}{% if token.type_ == 'Function' %}
  pub fn {{token.name | to_snake}}<C: AsRef<{{token.name | to_camel}}>>(&self, {{token.name | to_snake}}: C) -> RTDResult<()> {
    self.api.send({{token.name | to_snake}}.as_ref())
  }
{% endif %}{% endfor %}


}
