use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an item of a list page block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockListItem {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Item label
  label: String,
  /// Item blocks
  blocks: Vec<PageBlock>,
  /// True, if the item has a checkbox
  has_checkbox: bool,
  /// True, if the item is checked
  is_checked: bool,
  /// Value of the item; 0 for unordered lists
  value: i64,
  /// Type of item numbering; must be one of "a" for lowercase letters, "A" for uppercase letters, "i" for lowercase Roman numerals, "I" for uppercase Roman numerals, "1" for decimal numbers, or an empty string for unordered lists
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: String,
}

impl RObject for PageBlockListItem {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "pageBlockListItem"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PageBlockListItem {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPageBlockListItemBuilder {
    let inner = PageBlockListItem {
      td_name: "pageBlockListItem".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPageBlockListItemBuilder { inner }
  }

  pub fn label(&self) -> &String {
    &self.label
  }

  pub fn blocks(&self) -> &Vec<PageBlock> {
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
pub struct RTDPageBlockListItemBuilder {
  inner: PageBlockListItem,
}

impl RTDPageBlockListItemBuilder {
  pub fn build(&self) -> PageBlockListItem {
    self.inner.clone()
  }

  pub fn label<T: AsRef<str>>(&mut self, label: T) -> &mut Self {
    self.inner.label = label.as_ref().to_string();
    self
  }

  pub fn blocks(&mut self, blocks: Vec<PageBlock>) -> &mut Self {
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

impl AsRef<PageBlockListItem> for PageBlockListItem {
  fn as_ref(&self) -> &PageBlockListItem {
    self
  }
}

impl AsRef<PageBlockListItem> for RTDPageBlockListItemBuilder {
  fn as_ref(&self) -> &PageBlockListItem {
    &self.inner
  }
}
