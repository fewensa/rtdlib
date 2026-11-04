use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an item of a list page block to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPageBlockListItem {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Item blocks
  blocks: Vec<InputPageBlock>,
  /// True, if the item has a checkbox
  has_checkbox: bool,
  /// True, if the item is checked
  is_checked: bool,
  /// Value of the item; pass 0 for unordered lists
  value: i64,
  /// Type of item numbering; pass "a" for lowercase letters, "A" for uppercase letters, "i" for lowercase Roman numerals, "I" for uppercase Roman numerals, "1" for decimal numbers, or an empty string for unordered lists
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: String,
}

impl RObject for InputPageBlockListItem {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPageBlockListItem"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputPageBlockListItem {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPageBlockListItemBuilder {
    let inner = InputPageBlockListItem {
      td_name: "inputPageBlockListItem".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPageBlockListItemBuilder { inner }
  }

  pub fn blocks(&self) -> &Vec<InputPageBlock> {
    &self.blocks
  }

  pub fn has_checkbox(&self) -> bool {
    self.has_checkbox
  }

  pub fn is_checked(&self) -> bool {
    self.is_checked
  }

  pub fn value(&self) -> i64 {
    self.value
  }

  pub fn type_(&self) -> &String {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDInputPageBlockListItemBuilder {
  inner: InputPageBlockListItem,
}

impl RTDInputPageBlockListItemBuilder {
  pub fn build(&self) -> InputPageBlockListItem {
    self.inner.clone()
  }

  pub fn blocks(&mut self, blocks: Vec<InputPageBlock>) -> &mut Self {
    self.inner.blocks = blocks;
    self
  }

  pub fn has_checkbox(&mut self, has_checkbox: bool) -> &mut Self {
    self.inner.has_checkbox = has_checkbox;
    self
  }

  pub fn is_checked(&mut self, is_checked: bool) -> &mut Self {
    self.inner.is_checked = is_checked;
    self
  }

  pub fn value(&mut self, value: i64) -> &mut Self {
    self.inner.value = value;
    self
  }

  pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().to_string();
    self
  }
}

impl AsRef<InputPageBlockListItem> for InputPageBlockListItem {
  fn as_ref(&self) -> &InputPageBlockListItem {
    self
  }
}

impl AsRef<InputPageBlockListItem> for RTDInputPageBlockListItemBuilder {
  fn as_ref(&self) -> &InputPageBlockListItem {
    &self.inner
  }
}
