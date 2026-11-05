use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a button inside a rich message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineButton {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the button; only richTexts, richTextPlain, and richTextCustomEmoji are allowed
  text: RichText,
  /// Style of the button
  style: ButtonStyle,
  /// Type of the button; must be one of inlineKeyboardButtonTypeUrl, inlineKeyboardButtonTypeLoginUrl, inlineKeyboardButtonTypeWebApp, inlineKeyboardButtonTypeCallback, inlineKeyboardButtonTypeSwitchInline, inlineKeyboardButtonTypeUser, inlineKeyboardButtonTypeCopyText. Additionally, inlineKeyboardButtonTypeCallbackWithPassword and inlineKeyboardButtonTypeDisabled may be received in incoming messages. Regular users may use only inlineKeyboardButtonTypeUrl, inlineKeyboardButtonTypeUser and inlineKeyboardButtonTypeCopyText
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: InlineKeyboardButtonType,
}

impl RObject for InlineButton {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineButton"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InlineButton {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineButtonBuilder {
    let inner = InlineButton {
      td_name: "inlineButton".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineButtonBuilder { inner }
  }

  pub fn text(&self) -> &RichText {
    &self.text
  }

  pub fn style(&self) -> &ButtonStyle {
    &self.style
  }

  pub fn type_(&self) -> &InlineKeyboardButtonType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDInlineButtonBuilder {
  inner: InlineButton,
}

impl RTDInlineButtonBuilder {
  pub fn build(&self) -> InlineButton {
    self.inner.clone()
  }

  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn style<T: AsRef<ButtonStyle>>(&mut self, style: T) -> &mut Self {
    self.inner.style = style.as_ref().clone();
    self
  }

  pub fn type_<T: AsRef<InlineKeyboardButtonType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<InlineButton> for InlineButton {
  fn as_ref(&self) -> &InlineButton {
    self
  }
}

impl AsRef<InlineButton> for RTDInlineButtonBuilder {
  fn as_ref(&self) -> &InlineButton {
    &self.inner
  }
}
