{% set trait_name = token.name | to_camel %}
/// TRAIT | {{token.description}}
pub trait TD{{trait_name}}: Debug + RObject {}

/// {{token.description}}
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum {{trait_name}} {
  #[doc(hidden)] _Default(()),
{% for subt in sub_tokens(token=token) %}  /// {{subt.description}}
  {{subt.name | td_remove_prefix(prefix=trait_name) | to_camel}}({{subt.name | to_camel}}),
{% endfor %}
}

impl Default for {{trait_name}} {
  fn default() -> Self { {{trait_name}}::_Default(()) }
}

impl<'de> Deserialize<'de> for {{trait_name}} {
  fn deserialize<D>(deserializer: D) -> Result<{{trait_name}}, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      {{trait_name}},
{% for subt in sub_tokens(token=token) %}      ({{subt.name}}, {{subt.name | td_remove_prefix(prefix=trait_name) | to_camel}});
{% endfor %}
    )(deserializer)
  }
}

impl RObject for {{trait_name}} {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
{% for subt in sub_tokens(token=token) %}      {{trait_name}}::{{subt.name | td_remove_prefix(prefix=trait_name) | to_camel}}(t) => t.td_name(),
{% endfor %}
      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
{% for subt in sub_tokens(token=token) %}      {{trait_name}}::{{subt.name | td_remove_prefix(prefix=trait_name) | to_camel}}(t) => t.extra(),
{% endfor %}
      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl {{trait_name}} {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let {{trait_name}}::_Default(_) = self { true } else { false } }

{% for subt in sub_tokens(token=token) %}  pub fn is_{{subt.name | td_remove_prefix(prefix=trait_name) | to_snake}}(&self) -> bool { if let {{trait_name}}::{{subt.name | td_remove_prefix(prefix=trait_name) | to_camel}}(_) = self { true } else { false } }
{% endfor %}
{% for subt in sub_tokens(token=token) %}  pub fn on_{{subt.name | td_remove_prefix(prefix=trait_name) | to_snake}}<F: FnOnce(&{{subt.name | to_camel}})>(&self, fnc: F) -> &Self { if let {{trait_name}}::{{subt.name | td_remove_prefix(prefix=trait_name) | to_camel}}(t) = self { fnc(t) }; self }
{% endfor %}
{% for subt in sub_tokens(token=token) %}  pub fn as_{{subt.name | td_remove_prefix(prefix=trait_name) | to_snake}}(&self) -> Option<&{{subt.name | to_camel}}> { if let {{trait_name}}::{{subt.name | td_remove_prefix(prefix=trait_name) | to_camel}}(t) = self { return Some(t) } None }
{% endfor %}

{% for subt in sub_tokens(token=token) %}{% set item_name = subt.name | td_remove_prefix(prefix=trait_name) | to_camel %}
  pub fn {{item_name | to_snake | td_safe_field}}<T: AsRef<{{subt.name | to_camel}}>>(t: T) -> Self { {{trait_name}}::{{item_name}}(t.as_ref().clone()) }
{% endfor %}
}

impl AsRef<{{trait_name}}> for {{trait_name}} {
  fn as_ref(&self) -> &{{trait_name}} { self }
}
