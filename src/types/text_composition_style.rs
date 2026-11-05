use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a style that can be used to compose a text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextCompositionStyle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the style
  name: String,
  /// Identifier of the custom emoji corresponding to the style; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  custom_emoji_id: isize,
  /// Title of the style in the user application's language
  title: String,
  /// True, if the style is created by a user
  is_custom: bool,
  /// True, if the user is creator of the style
  is_creator: bool,
  /// Number of users that installed the style; for created custom styles only; 0 if unknown
  install_count: i64,
  /// Prompt of the style; for created custom styles only
  prompt: String,
  /// User identifier of the creator of the style; 0 if none or unknown
  creator_user_id: i64,
  /// Example of the style usage in English; may be null if unknown
  english_example: Option<TextCompositionStyleExample>,
}

impl RObject for TextCompositionStyle {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "textCompositionStyle"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TextCompositionStyle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTextCompositionStyleBuilder {
    let inner = TextCompositionStyle {
      td_name: "textCompositionStyle".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTextCompositionStyleBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn custom_emoji_id(&self) -> isize {
    self.custom_emoji_id
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn is_custom(&self) -> bool {
    self.is_custom
  }

  pub fn is_creator(&self) -> bool {
    self.is_creator
  }

  pub fn install_count(&self) -> i64 {
    self.install_count
  }

  pub fn prompt(&self) -> &String {
    &self.prompt
  }

  pub fn creator_user_id(&self) -> i64 {
    self.creator_user_id
  }

  pub fn english_example(&self) -> &Option<TextCompositionStyleExample> {
    &self.english_example
  }
}

#[doc(hidden)]
pub struct RTDTextCompositionStyleBuilder {
  inner: TextCompositionStyle,
}

impl RTDTextCompositionStyleBuilder {
  pub fn build(&self) -> TextCompositionStyle {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn custom_emoji_id(&mut self, custom_emoji_id: isize) -> &mut Self {
    self.inner.custom_emoji_id = custom_emoji_id;
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn is_custom(&mut self, is_custom: bool) -> &mut Self {
    self.inner.is_custom = is_custom;
    self
  }

  pub fn is_creator(&mut self, is_creator: bool) -> &mut Self {
    self.inner.is_creator = is_creator;
    self
  }

  pub fn install_count(&mut self, install_count: i64) -> &mut Self {
    self.inner.install_count = install_count;
    self
  }

  pub fn prompt<T: AsRef<str>>(&mut self, prompt: T) -> &mut Self {
    self.inner.prompt = prompt.as_ref().to_string();
    self
  }

  pub fn creator_user_id(&mut self, creator_user_id: i64) -> &mut Self {
    self.inner.creator_user_id = creator_user_id;
    self
  }

  pub fn english_example<T: AsRef<TextCompositionStyleExample>>(
    &mut self,
    english_example: T,
  ) -> &mut Self {
    self.inner.english_example = Some(english_example.as_ref().clone());
    self
  }
}

impl AsRef<TextCompositionStyle> for TextCompositionStyle {
  fn as_ref(&self) -> &TextCompositionStyle {
    self
  }
}

impl AsRef<TextCompositionStyle> for RTDTextCompositionStyleBuilder {
  fn as_ref(&self) -> &TextCompositionStyle {
    &self.inner
  }
}
