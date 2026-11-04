
pub use self::_common::{
  RObject,
  RFunction,
  detect_td_type,
  detect_td_type_and_extra,
  from_json,
  TdType,
};

#[macro_use] mod _common;

{% for key, value in file_obj_map %}pub use self::{{key}}::*;
{% endfor %}

{#
//{% for key, value in file_obj_map %}pub use self::{{key}}::{
//{% for token in value %}  {% if token.type_ == "Trait" %}TD{{token.name | to_camel}},
//  {{token.name | to_camel}},{% else %}{{token.name | to_camel}},{% endif %}
//{% endfor %}
//};
//{% endfor %}
#}

{% for key, value in file_obj_map %}mod {{key}};
{% endfor %}
