{% set struct_name = token.name | to_camel %}
/// {{token.description}}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct {{struct_name}} {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  {% for field in token.arguments %}/// {{field.description}}
  {% for macro_ in td_macros(arg=field, token=token) %}{{macro_}} {% endfor %}{% if field.sign_name == 'type' %}#[serde(rename(serialize = "type", deserialize = "type"))] {% endif %}{% if field.sign_name == 'extra' %}#[serde(rename(serialize = "extra", deserialize = "extra"))] {% endif %}{{field.sign_name | td_safe_field}}: {{td_arg(arg=field, token=token)}},
  {% endfor %}
}

impl RObject for {{struct_name}} {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "{{token.name}}" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}
{% if token.blood and token.blood | to_snake != token.name | to_snake %}
{% set blood_token = find_token(token_name=token.blood) %}
{% if blood_token.type_ == 'Trait' %}impl TD{{token.blood | to_camel}} for {{struct_name}} {}{% endif %}
{% endif %}
{% if token.type_ == 'Function' %}impl RFunction for {{struct_name}} {}{% endif %}

impl {{struct_name}} {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTD{{struct_name}}Builder {
    let mut inner = {{struct_name}}::default();
    inner.td_name = "{{token.name}}".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTD{{struct_name}}Builder { inner }
  }
{% for field in token.arguments %}{% set field_type = td_arg(arg=field, token=token) %}{% set is_primitive = is_primitive(type_ = field_type) %}
  pub fn {{field.sign_name | td_safe_field}}(&self) -> {% if not is_primitive %}&{% endif %}{{field_type}} { {% if not is_primitive %}&{% endif %}self.{{field.sign_name | td_safe_field}} }
{% endfor %}
}

#[doc(hidden)]
pub struct RTD{{struct_name}}Builder {
  inner: {{struct_name}}
}

impl RTD{{struct_name}}Builder {
  pub fn build(&self) -> {{struct_name}} { self.inner.clone() }
{% for field in token.arguments %}
{% set builder_field_type=td_arg(arg=field, token=token, builder_arg=true) %} {% set sign_name = field.sign_name | td_safe_field %} {% set is_optional = is_optional(type_=td_arg(arg=field, token=token)) %} {% set is_builder_ref = is_builder_ref(type_ = builder_field_type) %}
  pub fn {{sign_name}}{%if is_builder_ref%}<T: AsRef<{% if builder_field_type == 'String' %}str{% else %}{{builder_field_type}}{% endif %}>>{%endif%}(&mut self, {{sign_name}}: {%if is_builder_ref%}T{%else%}{{builder_field_type}}{%endif%}) -> &mut Self {
    self.inner.{{sign_name}} = {% if is_optional %}Some({% endif %}{{sign_name}}{%if is_builder_ref %}.as_ref(){% if builder_field_type == 'String' %}.to_string(){% else %}.clone(){% endif %}{% endif %}{% if is_optional %}){% endif %};
    self
  }
{% endfor %}
}

impl AsRef<{{struct_name}}> for {{struct_name}} {
  fn as_ref(&self) -> &{{struct_name}} { self }
}

impl AsRef<{{struct_name}}> for RTD{{struct_name}}Builder {
  fn as_ref(&self) -> &{{struct_name}} { &self.inner }
}
