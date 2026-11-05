use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a formatted text object
pub trait TDRichText: Debug + RObject {}

/// Describes a formatted text object
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RichText {
  #[doc(hidden)]
  _Default(()),
  /// An anchor
  Anchor(RichTextAnchor),
  /// A link to an anchor on the same page
  AnchorLink(RichTextAnchorLink),
  /// A bank card number
  BankCardNumber(RichTextBankCardNumber),
  /// A bold rich text
  Bold(RichTextBold),
  /// A bot command
  BotCommand(RichTextBotCommand),
  /// A button
  Button(RichTextButton),
  /// A cashtag
  Cashtag(RichTextCashtag),
  /// A custom emoji
  CustomEmoji(RichTextCustomEmoji),
  /// A date and time
  DateTime(RichTextDateTime),
  /// A rich text replacing another rich text; not supported in inputRichMessage
  Diff(RichTextDiff),
  /// A rich text email address
  EmailAddress(RichTextEmailAddress),
  /// A fixed-width rich text
  Fixed(RichTextFixed),
  /// A hashtag
  Hashtag(RichTextHashtag),
  /// A small image inside the text; instant view only
  Icon(RichTextIcon),
  /// An italicized rich text
  Italic(RichTextItalic),
  /// A marked rich text
  Marked(RichTextMarked),
  /// A mathematical expression
  MathematicalExpression(RichTextMathematicalExpression),
  /// A mention of a Telegram user or chat by a username
  Mention(RichTextMention),
  /// A rich text that serves as a mention of a user
  MentionName(RichTextMentionName),
  /// A rich text phone number
  PhoneNumber(RichTextPhoneNumber),
  /// A plain text
  Plain(RichTextPlain),
  /// A reference
  Reference(RichTextReference),
  /// A link to a reference on the same page
  ReferenceLink(RichTextReferenceLink),
  /// A spoilered rich text
  Spoiler(RichTextSpoiler),
  /// A strikethrough rich text
  Strikethrough(RichTextStrikethrough),
  /// A subscript rich text
  Subscript(RichTextSubscript),
  /// A superscript rich text
  Superscript(RichTextSuperscript),
  /// An address in the TON blockchain
  TonAddress(RichTextTonAddress),
  /// An underlined rich text
  Underline(RichTextUnderline),
  /// A rich text URL link
  Url(RichTextUrl),
  /// A concatenation of rich texts
  RichTexts(RichTexts),
}

impl Default for RichText {
  fn default() -> Self {
    RichText::_Default(())
  }
}

impl<'de> Deserialize<'de> for RichText {
  fn deserialize<D>(deserializer: D) -> Result<RichText, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      RichText,
      (richTextAnchor, Anchor);
      (richTextAnchorLink, AnchorLink);
      (richTextBankCardNumber, BankCardNumber);
      (richTextBold, Bold);
      (richTextBotCommand, BotCommand);
      (richTextButton, Button);
      (richTextCashtag, Cashtag);
      (richTextCustomEmoji, CustomEmoji);
      (richTextDateTime, DateTime);
      (richTextDiff, Diff);
      (richTextEmailAddress, EmailAddress);
      (richTextFixed, Fixed);
      (richTextHashtag, Hashtag);
      (richTextIcon, Icon);
      (richTextItalic, Italic);
      (richTextMarked, Marked);
      (richTextMathematicalExpression, MathematicalExpression);
      (richTextMention, Mention);
      (richTextMentionName, MentionName);
      (richTextPhoneNumber, PhoneNumber);
      (richTextPlain, Plain);
      (richTextReference, Reference);
      (richTextReferenceLink, ReferenceLink);
      (richTextSpoiler, Spoiler);
      (richTextStrikethrough, Strikethrough);
      (richTextSubscript, Subscript);
      (richTextSuperscript, Superscript);
      (richTextTonAddress, TonAddress);
      (richTextUnderline, Underline);
      (richTextUrl, Url);
      (richTexts, RichTexts);

    )(deserializer)
  }
}

impl RObject for RichText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      RichText::Anchor(t) => t.td_name(),
      RichText::AnchorLink(t) => t.td_name(),
      RichText::BankCardNumber(t) => t.td_name(),
      RichText::Bold(t) => t.td_name(),
      RichText::BotCommand(t) => t.td_name(),
      RichText::Button(t) => t.td_name(),
      RichText::Cashtag(t) => t.td_name(),
      RichText::CustomEmoji(t) => t.td_name(),
      RichText::DateTime(t) => t.td_name(),
      RichText::Diff(t) => t.td_name(),
      RichText::EmailAddress(t) => t.td_name(),
      RichText::Fixed(t) => t.td_name(),
      RichText::Hashtag(t) => t.td_name(),
      RichText::Icon(t) => t.td_name(),
      RichText::Italic(t) => t.td_name(),
      RichText::Marked(t) => t.td_name(),
      RichText::MathematicalExpression(t) => t.td_name(),
      RichText::Mention(t) => t.td_name(),
      RichText::MentionName(t) => t.td_name(),
      RichText::PhoneNumber(t) => t.td_name(),
      RichText::Plain(t) => t.td_name(),
      RichText::Reference(t) => t.td_name(),
      RichText::ReferenceLink(t) => t.td_name(),
      RichText::Spoiler(t) => t.td_name(),
      RichText::Strikethrough(t) => t.td_name(),
      RichText::Subscript(t) => t.td_name(),
      RichText::Superscript(t) => t.td_name(),
      RichText::TonAddress(t) => t.td_name(),
      RichText::Underline(t) => t.td_name(),
      RichText::Url(t) => t.td_name(),
      RichText::RichTexts(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      RichText::Anchor(t) => t.extra(),
      RichText::AnchorLink(t) => t.extra(),
      RichText::BankCardNumber(t) => t.extra(),
      RichText::Bold(t) => t.extra(),
      RichText::BotCommand(t) => t.extra(),
      RichText::Button(t) => t.extra(),
      RichText::Cashtag(t) => t.extra(),
      RichText::CustomEmoji(t) => t.extra(),
      RichText::DateTime(t) => t.extra(),
      RichText::Diff(t) => t.extra(),
      RichText::EmailAddress(t) => t.extra(),
      RichText::Fixed(t) => t.extra(),
      RichText::Hashtag(t) => t.extra(),
      RichText::Icon(t) => t.extra(),
      RichText::Italic(t) => t.extra(),
      RichText::Marked(t) => t.extra(),
      RichText::MathematicalExpression(t) => t.extra(),
      RichText::Mention(t) => t.extra(),
      RichText::MentionName(t) => t.extra(),
      RichText::PhoneNumber(t) => t.extra(),
      RichText::Plain(t) => t.extra(),
      RichText::Reference(t) => t.extra(),
      RichText::ReferenceLink(t) => t.extra(),
      RichText::Spoiler(t) => t.extra(),
      RichText::Strikethrough(t) => t.extra(),
      RichText::Subscript(t) => t.extra(),
      RichText::Superscript(t) => t.extra(),
      RichText::TonAddress(t) => t.extra(),
      RichText::Underline(t) => t.extra(),
      RichText::Url(t) => t.extra(),
      RichText::RichTexts(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl RichText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, RichText::_Default(_))
  }

  pub fn is_anchor(&self) -> bool {
    matches!(self, RichText::Anchor(_))
  }
  pub fn is_anchor_link(&self) -> bool {
    matches!(self, RichText::AnchorLink(_))
  }
  pub fn is_bank_card_number(&self) -> bool {
    matches!(self, RichText::BankCardNumber(_))
  }
  pub fn is_bold(&self) -> bool {
    matches!(self, RichText::Bold(_))
  }
  pub fn is_bot_command(&self) -> bool {
    matches!(self, RichText::BotCommand(_))
  }
  pub fn is_button(&self) -> bool {
    matches!(self, RichText::Button(_))
  }
  pub fn is_cashtag(&self) -> bool {
    matches!(self, RichText::Cashtag(_))
  }
  pub fn is_custom_emoji(&self) -> bool {
    matches!(self, RichText::CustomEmoji(_))
  }
  pub fn is_date_time(&self) -> bool {
    matches!(self, RichText::DateTime(_))
  }
  pub fn is_diff(&self) -> bool {
    matches!(self, RichText::Diff(_))
  }
  pub fn is_email_address(&self) -> bool {
    matches!(self, RichText::EmailAddress(_))
  }
  pub fn is_fixed(&self) -> bool {
    matches!(self, RichText::Fixed(_))
  }
  pub fn is_hashtag(&self) -> bool {
    matches!(self, RichText::Hashtag(_))
  }
  pub fn is_icon(&self) -> bool {
    matches!(self, RichText::Icon(_))
  }
  pub fn is_italic(&self) -> bool {
    matches!(self, RichText::Italic(_))
  }
  pub fn is_marked(&self) -> bool {
    matches!(self, RichText::Marked(_))
  }
  pub fn is_mathematical_expression(&self) -> bool {
    matches!(self, RichText::MathematicalExpression(_))
  }
  pub fn is_mention(&self) -> bool {
    matches!(self, RichText::Mention(_))
  }
  pub fn is_mention_name(&self) -> bool {
    matches!(self, RichText::MentionName(_))
  }
  pub fn is_phone_number(&self) -> bool {
    matches!(self, RichText::PhoneNumber(_))
  }
  pub fn is_plain(&self) -> bool {
    matches!(self, RichText::Plain(_))
  }
  pub fn is_reference(&self) -> bool {
    matches!(self, RichText::Reference(_))
  }
  pub fn is_reference_link(&self) -> bool {
    matches!(self, RichText::ReferenceLink(_))
  }
  pub fn is_spoiler(&self) -> bool {
    matches!(self, RichText::Spoiler(_))
  }
  pub fn is_strikethrough(&self) -> bool {
    matches!(self, RichText::Strikethrough(_))
  }
  pub fn is_subscript(&self) -> bool {
    matches!(self, RichText::Subscript(_))
  }
  pub fn is_superscript(&self) -> bool {
    matches!(self, RichText::Superscript(_))
  }
  pub fn is_ton_address(&self) -> bool {
    matches!(self, RichText::TonAddress(_))
  }
  pub fn is_underline(&self) -> bool {
    matches!(self, RichText::Underline(_))
  }
  pub fn is_url(&self) -> bool {
    matches!(self, RichText::Url(_))
  }
  pub fn is_rich_texts(&self) -> bool {
    matches!(self, RichText::RichTexts(_))
  }

  pub fn on_anchor<F: FnOnce(&RichTextAnchor)>(&self, fnc: F) -> &Self {
    if let RichText::Anchor(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_anchor_link<F: FnOnce(&RichTextAnchorLink)>(&self, fnc: F) -> &Self {
    if let RichText::AnchorLink(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bank_card_number<F: FnOnce(&RichTextBankCardNumber)>(&self, fnc: F) -> &Self {
    if let RichText::BankCardNumber(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bold<F: FnOnce(&RichTextBold)>(&self, fnc: F) -> &Self {
    if let RichText::Bold(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_bot_command<F: FnOnce(&RichTextBotCommand)>(&self, fnc: F) -> &Self {
    if let RichText::BotCommand(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_button<F: FnOnce(&RichTextButton)>(&self, fnc: F) -> &Self {
    if let RichText::Button(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_cashtag<F: FnOnce(&RichTextCashtag)>(&self, fnc: F) -> &Self {
    if let RichText::Cashtag(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_custom_emoji<F: FnOnce(&RichTextCustomEmoji)>(&self, fnc: F) -> &Self {
    if let RichText::CustomEmoji(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_date_time<F: FnOnce(&RichTextDateTime)>(&self, fnc: F) -> &Self {
    if let RichText::DateTime(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_diff<F: FnOnce(&RichTextDiff)>(&self, fnc: F) -> &Self {
    if let RichText::Diff(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_email_address<F: FnOnce(&RichTextEmailAddress)>(&self, fnc: F) -> &Self {
    if let RichText::EmailAddress(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fixed<F: FnOnce(&RichTextFixed)>(&self, fnc: F) -> &Self {
    if let RichText::Fixed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_hashtag<F: FnOnce(&RichTextHashtag)>(&self, fnc: F) -> &Self {
    if let RichText::Hashtag(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_icon<F: FnOnce(&RichTextIcon)>(&self, fnc: F) -> &Self {
    if let RichText::Icon(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_italic<F: FnOnce(&RichTextItalic)>(&self, fnc: F) -> &Self {
    if let RichText::Italic(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_marked<F: FnOnce(&RichTextMarked)>(&self, fnc: F) -> &Self {
    if let RichText::Marked(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_mathematical_expression<F: FnOnce(&RichTextMathematicalExpression)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let RichText::MathematicalExpression(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_mention<F: FnOnce(&RichTextMention)>(&self, fnc: F) -> &Self {
    if let RichText::Mention(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_mention_name<F: FnOnce(&RichTextMentionName)>(&self, fnc: F) -> &Self {
    if let RichText::MentionName(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_phone_number<F: FnOnce(&RichTextPhoneNumber)>(&self, fnc: F) -> &Self {
    if let RichText::PhoneNumber(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_plain<F: FnOnce(&RichTextPlain)>(&self, fnc: F) -> &Self {
    if let RichText::Plain(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_reference<F: FnOnce(&RichTextReference)>(&self, fnc: F) -> &Self {
    if let RichText::Reference(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_reference_link<F: FnOnce(&RichTextReferenceLink)>(&self, fnc: F) -> &Self {
    if let RichText::ReferenceLink(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_spoiler<F: FnOnce(&RichTextSpoiler)>(&self, fnc: F) -> &Self {
    if let RichText::Spoiler(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_strikethrough<F: FnOnce(&RichTextStrikethrough)>(&self, fnc: F) -> &Self {
    if let RichText::Strikethrough(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_subscript<F: FnOnce(&RichTextSubscript)>(&self, fnc: F) -> &Self {
    if let RichText::Subscript(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_superscript<F: FnOnce(&RichTextSuperscript)>(&self, fnc: F) -> &Self {
    if let RichText::Superscript(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ton_address<F: FnOnce(&RichTextTonAddress)>(&self, fnc: F) -> &Self {
    if let RichText::TonAddress(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_underline<F: FnOnce(&RichTextUnderline)>(&self, fnc: F) -> &Self {
    if let RichText::Underline(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_url<F: FnOnce(&RichTextUrl)>(&self, fnc: F) -> &Self {
    if let RichText::Url(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_rich_texts<F: FnOnce(&RichTexts)>(&self, fnc: F) -> &Self {
    if let RichText::RichTexts(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_anchor(&self) -> Option<&RichTextAnchor> {
    if let RichText::Anchor(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_anchor_link(&self) -> Option<&RichTextAnchorLink> {
    if let RichText::AnchorLink(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bank_card_number(&self) -> Option<&RichTextBankCardNumber> {
    if let RichText::BankCardNumber(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bold(&self) -> Option<&RichTextBold> {
    if let RichText::Bold(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_bot_command(&self) -> Option<&RichTextBotCommand> {
    if let RichText::BotCommand(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_button(&self) -> Option<&RichTextButton> {
    if let RichText::Button(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_cashtag(&self) -> Option<&RichTextCashtag> {
    if let RichText::Cashtag(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_custom_emoji(&self) -> Option<&RichTextCustomEmoji> {
    if let RichText::CustomEmoji(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_date_time(&self) -> Option<&RichTextDateTime> {
    if let RichText::DateTime(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_diff(&self) -> Option<&RichTextDiff> {
    if let RichText::Diff(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_email_address(&self) -> Option<&RichTextEmailAddress> {
    if let RichText::EmailAddress(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fixed(&self) -> Option<&RichTextFixed> {
    if let RichText::Fixed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_hashtag(&self) -> Option<&RichTextHashtag> {
    if let RichText::Hashtag(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_icon(&self) -> Option<&RichTextIcon> {
    if let RichText::Icon(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_italic(&self) -> Option<&RichTextItalic> {
    if let RichText::Italic(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_marked(&self) -> Option<&RichTextMarked> {
    if let RichText::Marked(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_mathematical_expression(&self) -> Option<&RichTextMathematicalExpression> {
    if let RichText::MathematicalExpression(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_mention(&self) -> Option<&RichTextMention> {
    if let RichText::Mention(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_mention_name(&self) -> Option<&RichTextMentionName> {
    if let RichText::MentionName(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_phone_number(&self) -> Option<&RichTextPhoneNumber> {
    if let RichText::PhoneNumber(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_plain(&self) -> Option<&RichTextPlain> {
    if let RichText::Plain(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_reference(&self) -> Option<&RichTextReference> {
    if let RichText::Reference(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_reference_link(&self) -> Option<&RichTextReferenceLink> {
    if let RichText::ReferenceLink(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_spoiler(&self) -> Option<&RichTextSpoiler> {
    if let RichText::Spoiler(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_strikethrough(&self) -> Option<&RichTextStrikethrough> {
    if let RichText::Strikethrough(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_subscript(&self) -> Option<&RichTextSubscript> {
    if let RichText::Subscript(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_superscript(&self) -> Option<&RichTextSuperscript> {
    if let RichText::Superscript(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ton_address(&self) -> Option<&RichTextTonAddress> {
    if let RichText::TonAddress(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_underline(&self) -> Option<&RichTextUnderline> {
    if let RichText::Underline(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_url(&self) -> Option<&RichTextUrl> {
    if let RichText::Url(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_rich_texts(&self) -> Option<&RichTexts> {
    if let RichText::RichTexts(t) = self {
      return Some(t);
    }
    None
  }

  pub fn anchor<T: AsRef<RichTextAnchor>>(t: T) -> Self {
    RichText::Anchor(t.as_ref().clone())
  }

  pub fn anchor_link<T: AsRef<RichTextAnchorLink>>(t: T) -> Self {
    RichText::AnchorLink(t.as_ref().clone())
  }

  pub fn bank_card_number<T: AsRef<RichTextBankCardNumber>>(t: T) -> Self {
    RichText::BankCardNumber(t.as_ref().clone())
  }

  pub fn bold<T: AsRef<RichTextBold>>(t: T) -> Self {
    RichText::Bold(t.as_ref().clone())
  }

  pub fn bot_command<T: AsRef<RichTextBotCommand>>(t: T) -> Self {
    RichText::BotCommand(t.as_ref().clone())
  }

  pub fn button<T: AsRef<RichTextButton>>(t: T) -> Self {
    RichText::Button(t.as_ref().clone())
  }

  pub fn cashtag<T: AsRef<RichTextCashtag>>(t: T) -> Self {
    RichText::Cashtag(t.as_ref().clone())
  }

  pub fn custom_emoji<T: AsRef<RichTextCustomEmoji>>(t: T) -> Self {
    RichText::CustomEmoji(t.as_ref().clone())
  }

  pub fn date_time<T: AsRef<RichTextDateTime>>(t: T) -> Self {
    RichText::DateTime(t.as_ref().clone())
  }

  pub fn diff<T: AsRef<RichTextDiff>>(t: T) -> Self {
    RichText::Diff(t.as_ref().clone())
  }

  pub fn email_address<T: AsRef<RichTextEmailAddress>>(t: T) -> Self {
    RichText::EmailAddress(t.as_ref().clone())
  }

  pub fn fixed<T: AsRef<RichTextFixed>>(t: T) -> Self {
    RichText::Fixed(t.as_ref().clone())
  }

  pub fn hashtag<T: AsRef<RichTextHashtag>>(t: T) -> Self {
    RichText::Hashtag(t.as_ref().clone())
  }

  pub fn icon<T: AsRef<RichTextIcon>>(t: T) -> Self {
    RichText::Icon(t.as_ref().clone())
  }

  pub fn italic<T: AsRef<RichTextItalic>>(t: T) -> Self {
    RichText::Italic(t.as_ref().clone())
  }

  pub fn marked<T: AsRef<RichTextMarked>>(t: T) -> Self {
    RichText::Marked(t.as_ref().clone())
  }

  pub fn mathematical_expression<T: AsRef<RichTextMathematicalExpression>>(t: T) -> Self {
    RichText::MathematicalExpression(t.as_ref().clone())
  }

  pub fn mention<T: AsRef<RichTextMention>>(t: T) -> Self {
    RichText::Mention(t.as_ref().clone())
  }

  pub fn mention_name<T: AsRef<RichTextMentionName>>(t: T) -> Self {
    RichText::MentionName(t.as_ref().clone())
  }

  pub fn phone_number<T: AsRef<RichTextPhoneNumber>>(t: T) -> Self {
    RichText::PhoneNumber(t.as_ref().clone())
  }

  pub fn plain<T: AsRef<RichTextPlain>>(t: T) -> Self {
    RichText::Plain(t.as_ref().clone())
  }

  pub fn reference<T: AsRef<RichTextReference>>(t: T) -> Self {
    RichText::Reference(t.as_ref().clone())
  }

  pub fn reference_link<T: AsRef<RichTextReferenceLink>>(t: T) -> Self {
    RichText::ReferenceLink(t.as_ref().clone())
  }

  pub fn spoiler<T: AsRef<RichTextSpoiler>>(t: T) -> Self {
    RichText::Spoiler(t.as_ref().clone())
  }

  pub fn strikethrough<T: AsRef<RichTextStrikethrough>>(t: T) -> Self {
    RichText::Strikethrough(t.as_ref().clone())
  }

  pub fn subscript<T: AsRef<RichTextSubscript>>(t: T) -> Self {
    RichText::Subscript(t.as_ref().clone())
  }

  pub fn superscript<T: AsRef<RichTextSuperscript>>(t: T) -> Self {
    RichText::Superscript(t.as_ref().clone())
  }

  pub fn ton_address<T: AsRef<RichTextTonAddress>>(t: T) -> Self {
    RichText::TonAddress(t.as_ref().clone())
  }

  pub fn underline<T: AsRef<RichTextUnderline>>(t: T) -> Self {
    RichText::Underline(t.as_ref().clone())
  }

  pub fn url<T: AsRef<RichTextUrl>>(t: T) -> Self {
    RichText::Url(t.as_ref().clone())
  }

  pub fn rich_texts<T: AsRef<RichTexts>>(t: T) -> Self {
    RichText::RichTexts(t.as_ref().clone())
  }
}

impl AsRef<RichText> for RichText {
  fn as_ref(&self) -> &RichText {
    self
  }
}

/// An anchor
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextAnchor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Anchor name
  name: String,
}

impl RObject for RichTextAnchor {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextAnchor"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextAnchor {}

impl RichTextAnchor {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextAnchorBuilder {
    let inner = RichTextAnchor {
      td_name: "richTextAnchor".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextAnchorBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[doc(hidden)]
pub struct RTDRichTextAnchorBuilder {
  inner: RichTextAnchor,
}

impl RTDRichTextAnchorBuilder {
  pub fn build(&self) -> RichTextAnchor {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextAnchor> for RichTextAnchor {
  fn as_ref(&self) -> &RichTextAnchor {
    self
  }
}

impl AsRef<RichTextAnchor> for RTDRichTextAnchorBuilder {
  fn as_ref(&self) -> &RichTextAnchor {
    &self.inner
  }
}

/// A link to an anchor on the same page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextAnchorLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The link text
  text: Box<RichText>,
  /// The anchor name. If the name is empty, the link must bring back to top
  anchor_name: String,
  /// An HTTP URL that opens the anchor
  url: String,
}

impl RObject for RichTextAnchorLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextAnchorLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextAnchorLink {}

impl RichTextAnchorLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextAnchorLinkBuilder {
    let inner = RichTextAnchorLink {
      td_name: "richTextAnchorLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextAnchorLinkBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn anchor_name(&self) -> &String {
    &self.anchor_name
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDRichTextAnchorLinkBuilder {
  inner: RichTextAnchorLink,
}

impl RTDRichTextAnchorLinkBuilder {
  pub fn build(&self) -> RichTextAnchorLink {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn anchor_name<T: AsRef<str>>(&mut self, anchor_name: T) -> &mut Self {
    self.inner.anchor_name = anchor_name.as_ref().to_string();
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextAnchorLink> for RichTextAnchorLink {
  fn as_ref(&self) -> &RichTextAnchorLink {
    self
  }
}

impl AsRef<RichTextAnchorLink> for RTDRichTextAnchorLinkBuilder {
  fn as_ref(&self) -> &RichTextAnchorLink {
    &self.inner
  }
}

/// A bank card number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextBankCardNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// The number of the bank card
  bank_card_number: String,
}

impl RObject for RichTextBankCardNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextBankCardNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextBankCardNumber {}

impl RichTextBankCardNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextBankCardNumberBuilder {
    let inner = RichTextBankCardNumber {
      td_name: "richTextBankCardNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextBankCardNumberBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn bank_card_number(&self) -> &String {
    &self.bank_card_number
  }
}

#[doc(hidden)]
pub struct RTDRichTextBankCardNumberBuilder {
  inner: RichTextBankCardNumber,
}

impl RTDRichTextBankCardNumberBuilder {
  pub fn build(&self) -> RichTextBankCardNumber {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn bank_card_number<T: AsRef<str>>(&mut self, bank_card_number: T) -> &mut Self {
    self.inner.bank_card_number = bank_card_number.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextBankCardNumber> for RichTextBankCardNumber {
  fn as_ref(&self) -> &RichTextBankCardNumber {
    self
  }
}

impl AsRef<RichTextBankCardNumber> for RTDRichTextBankCardNumberBuilder {
  fn as_ref(&self) -> &RichTextBankCardNumber {
    &self.inner
  }
}

/// A bold rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextBold {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextBold {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextBold"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextBold {}

impl RichTextBold {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextBoldBuilder {
    let inner = RichTextBold {
      td_name: "richTextBold".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextBoldBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextBoldBuilder {
  inner: RichTextBold,
}

impl RTDRichTextBoldBuilder {
  pub fn build(&self) -> RichTextBold {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextBold> for RichTextBold {
  fn as_ref(&self) -> &RichTextBold {
    self
  }
}

impl AsRef<RichTextBold> for RTDRichTextBoldBuilder {
  fn as_ref(&self) -> &RichTextBold {
    &self.inner
  }
}

/// A bot command
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextBotCommand {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// The bot command
  bot_command: String,
}

impl RObject for RichTextBotCommand {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextBotCommand"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextBotCommand {}

impl RichTextBotCommand {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextBotCommandBuilder {
    let inner = RichTextBotCommand {
      td_name: "richTextBotCommand".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextBotCommandBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn bot_command(&self) -> &String {
    &self.bot_command
  }
}

#[doc(hidden)]
pub struct RTDRichTextBotCommandBuilder {
  inner: RichTextBotCommand,
}

impl RTDRichTextBotCommandBuilder {
  pub fn build(&self) -> RichTextBotCommand {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn bot_command<T: AsRef<str>>(&mut self, bot_command: T) -> &mut Self {
    self.inner.bot_command = bot_command.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextBotCommand> for RichTextBotCommand {
  fn as_ref(&self) -> &RichTextBotCommand {
    self
  }
}

impl AsRef<RichTextBotCommand> for RTDRichTextBotCommandBuilder {
  fn as_ref(&self) -> &RichTextBotCommand {
    &self.inner
  }
}

/// A button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextButton {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The button
  button: Box<InlineButton>,
}

impl RObject for RichTextButton {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextButton"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextButton {}

impl RichTextButton {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextButtonBuilder {
    let inner = RichTextButton {
      td_name: "richTextButton".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextButtonBuilder { inner }
  }

  pub fn button(&self) -> &Box<InlineButton> {
    &self.button
  }
}

#[doc(hidden)]
pub struct RTDRichTextButtonBuilder {
  inner: RichTextButton,
}

impl RTDRichTextButtonBuilder {
  pub fn build(&self) -> RichTextButton {
    self.inner.clone()
  }

  pub fn button<T: AsRef<Box<InlineButton>>>(&mut self, button: T) -> &mut Self {
    self.inner.button = button.as_ref().clone();
    self
  }
}

impl AsRef<RichTextButton> for RichTextButton {
  fn as_ref(&self) -> &RichTextButton {
    self
  }
}

impl AsRef<RichTextButton> for RTDRichTextButtonBuilder {
  fn as_ref(&self) -> &RichTextButton {
    &self.inner
  }
}

/// A cashtag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextCashtag {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// The cashtag
  cashtag: String,
}

impl RObject for RichTextCashtag {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextCashtag"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextCashtag {}

impl RichTextCashtag {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextCashtagBuilder {
    let inner = RichTextCashtag {
      td_name: "richTextCashtag".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextCashtagBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn cashtag(&self) -> &String {
    &self.cashtag
  }
}

#[doc(hidden)]
pub struct RTDRichTextCashtagBuilder {
  inner: RichTextCashtag,
}

impl RTDRichTextCashtagBuilder {
  pub fn build(&self) -> RichTextCashtag {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn cashtag<T: AsRef<str>>(&mut self, cashtag: T) -> &mut Self {
    self.inner.cashtag = cashtag.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextCashtag> for RichTextCashtag {
  fn as_ref(&self) -> &RichTextCashtag {
    self
  }
}

impl AsRef<RichTextCashtag> for RTDRichTextCashtagBuilder {
  fn as_ref(&self) -> &RichTextCashtag {
    &self.inner
  }
}

/// A custom emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextCustomEmoji {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the custom emoji
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  custom_emoji_id: isize,
  /// Alternative text for the custom emoji
  alternative_text: String,
}

impl RObject for RichTextCustomEmoji {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextCustomEmoji"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextCustomEmoji {}

impl RichTextCustomEmoji {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextCustomEmojiBuilder {
    let inner = RichTextCustomEmoji {
      td_name: "richTextCustomEmoji".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextCustomEmojiBuilder { inner }
  }

  pub fn custom_emoji_id(&self) -> isize {
    self.custom_emoji_id
  }

  pub fn alternative_text(&self) -> &String {
    &self.alternative_text
  }
}

#[doc(hidden)]
pub struct RTDRichTextCustomEmojiBuilder {
  inner: RichTextCustomEmoji,
}

impl RTDRichTextCustomEmojiBuilder {
  pub fn build(&self) -> RichTextCustomEmoji {
    self.inner.clone()
  }

  pub fn custom_emoji_id(&mut self, custom_emoji_id: isize) -> &mut Self {
    self.inner.custom_emoji_id = custom_emoji_id;
    self
  }

  pub fn alternative_text<T: AsRef<str>>(&mut self, alternative_text: T) -> &mut Self {
    self.inner.alternative_text = alternative_text.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextCustomEmoji> for RichTextCustomEmoji {
  fn as_ref(&self) -> &RichTextCustomEmoji {
    self
  }
}

impl AsRef<RichTextCustomEmoji> for RTDRichTextCustomEmojiBuilder {
  fn as_ref(&self) -> &RichTextCustomEmoji {
    &self.inner
  }
}

/// A date and time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextDateTime {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Original text
  text: Box<RichText>,
  /// Point in time (Unix timestamp) representing the date and time
  unix_time: i64,
  /// Date and time formatting type; may be null if none and the original text must not be changed
  formatting_type: Option<DateTimeFormattingType>,
}

impl RObject for RichTextDateTime {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextDateTime"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextDateTime {}

impl RichTextDateTime {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextDateTimeBuilder {
    let inner = RichTextDateTime {
      td_name: "richTextDateTime".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextDateTimeBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn unix_time(&self) -> i64 {
    self.unix_time
  }

  pub fn formatting_type(&self) -> &Option<DateTimeFormattingType> {
    &self.formatting_type
  }
}

#[doc(hidden)]
pub struct RTDRichTextDateTimeBuilder {
  inner: RichTextDateTime,
}

impl RTDRichTextDateTimeBuilder {
  pub fn build(&self) -> RichTextDateTime {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn unix_time(&mut self, unix_time: i64) -> &mut Self {
    self.inner.unix_time = unix_time;
    self
  }

  pub fn formatting_type<T: AsRef<DateTimeFormattingType>>(
    &mut self,
    formatting_type: T,
  ) -> &mut Self {
    self.inner.formatting_type = Some(formatting_type.as_ref().clone());
    self
  }
}

impl AsRef<RichTextDateTime> for RichTextDateTime {
  fn as_ref(&self) -> &RichTextDateTime {
    self
  }
}

impl AsRef<RichTextDateTime> for RTDRichTextDateTimeBuilder {
  fn as_ref(&self) -> &RichTextDateTime {
    &self.inner
  }
}

/// A rich text replacing another rich text; not supported in inputRichMessage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextDiff {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// The old text
  old_text: Box<RichText>,
}

impl RObject for RichTextDiff {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextDiff"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextDiff {}

impl RichTextDiff {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextDiffBuilder {
    let inner = RichTextDiff {
      td_name: "richTextDiff".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextDiffBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn old_text(&self) -> &Box<RichText> {
    &self.old_text
  }
}

#[doc(hidden)]
pub struct RTDRichTextDiffBuilder {
  inner: RichTextDiff,
}

impl RTDRichTextDiffBuilder {
  pub fn build(&self) -> RichTextDiff {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn old_text<T: AsRef<Box<RichText>>>(&mut self, old_text: T) -> &mut Self {
    self.inner.old_text = old_text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextDiff> for RichTextDiff {
  fn as_ref(&self) -> &RichTextDiff {
    self
  }
}

impl AsRef<RichTextDiff> for RTDRichTextDiffBuilder {
  fn as_ref(&self) -> &RichTextDiff {
    &self.inner
  }
}

/// A rich text email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// Email address
  email_address: String,
}

impl RObject for RichTextEmailAddress {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextEmailAddress"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextEmailAddress {}

impl RichTextEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextEmailAddressBuilder {
    let inner = RichTextEmailAddress {
      td_name: "richTextEmailAddress".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextEmailAddressBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn email_address(&self) -> &String {
    &self.email_address
  }
}

#[doc(hidden)]
pub struct RTDRichTextEmailAddressBuilder {
  inner: RichTextEmailAddress,
}

impl RTDRichTextEmailAddressBuilder {
  pub fn build(&self) -> RichTextEmailAddress {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
    self.inner.email_address = email_address.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextEmailAddress> for RichTextEmailAddress {
  fn as_ref(&self) -> &RichTextEmailAddress {
    self
  }
}

impl AsRef<RichTextEmailAddress> for RTDRichTextEmailAddressBuilder {
  fn as_ref(&self) -> &RichTextEmailAddress {
    &self.inner
  }
}

/// A fixed-width rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextFixed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextFixed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextFixed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextFixed {}

impl RichTextFixed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextFixedBuilder {
    let inner = RichTextFixed {
      td_name: "richTextFixed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextFixedBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextFixedBuilder {
  inner: RichTextFixed,
}

impl RTDRichTextFixedBuilder {
  pub fn build(&self) -> RichTextFixed {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextFixed> for RichTextFixed {
  fn as_ref(&self) -> &RichTextFixed {
    self
  }
}

impl AsRef<RichTextFixed> for RTDRichTextFixedBuilder {
  fn as_ref(&self) -> &RichTextFixed {
    &self.inner
  }
}

/// A hashtag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextHashtag {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// The hashtag
  hashtag: String,
}

impl RObject for RichTextHashtag {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextHashtag"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextHashtag {}

impl RichTextHashtag {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextHashtagBuilder {
    let inner = RichTextHashtag {
      td_name: "richTextHashtag".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextHashtagBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn hashtag(&self) -> &String {
    &self.hashtag
  }
}

#[doc(hidden)]
pub struct RTDRichTextHashtagBuilder {
  inner: RichTextHashtag,
}

impl RTDRichTextHashtagBuilder {
  pub fn build(&self) -> RichTextHashtag {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn hashtag<T: AsRef<str>>(&mut self, hashtag: T) -> &mut Self {
    self.inner.hashtag = hashtag.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextHashtag> for RichTextHashtag {
  fn as_ref(&self) -> &RichTextHashtag {
    self
  }
}

impl AsRef<RichTextHashtag> for RTDRichTextHashtagBuilder {
  fn as_ref(&self) -> &RichTextHashtag {
    &self.inner
  }
}

/// A small image inside the text; instant view only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextIcon {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The image represented as a document. The image can be in GIF, JPEG, or PNG format
  document: Document,
  /// Width of a bounding box in which the image must be shown; 0 if unknown
  width: i64,
  /// Height of a bounding box in which the image must be shown; 0 if unknown
  height: i64,
}

impl RObject for RichTextIcon {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextIcon"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextIcon {}

impl RichTextIcon {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextIconBuilder {
    let inner = RichTextIcon {
      td_name: "richTextIcon".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextIconBuilder { inner }
  }

  pub fn document(&self) -> &Document {
    &self.document
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }
}

#[doc(hidden)]
pub struct RTDRichTextIconBuilder {
  inner: RichTextIcon,
}

impl RTDRichTextIconBuilder {
  pub fn build(&self) -> RichTextIcon {
    self.inner.clone()
  }

  pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
    self
  }

  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }
}

impl AsRef<RichTextIcon> for RichTextIcon {
  fn as_ref(&self) -> &RichTextIcon {
    self
  }
}

impl AsRef<RichTextIcon> for RTDRichTextIconBuilder {
  fn as_ref(&self) -> &RichTextIcon {
    &self.inner
  }
}

/// An italicized rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextItalic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextItalic {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextItalic"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextItalic {}

impl RichTextItalic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextItalicBuilder {
    let inner = RichTextItalic {
      td_name: "richTextItalic".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextItalicBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextItalicBuilder {
  inner: RichTextItalic,
}

impl RTDRichTextItalicBuilder {
  pub fn build(&self) -> RichTextItalic {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextItalic> for RichTextItalic {
  fn as_ref(&self) -> &RichTextItalic {
    self
  }
}

impl AsRef<RichTextItalic> for RTDRichTextItalicBuilder {
  fn as_ref(&self) -> &RichTextItalic {
    &self.inner
  }
}

/// A marked rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextMarked {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextMarked {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextMarked"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextMarked {}

impl RichTextMarked {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextMarkedBuilder {
    let inner = RichTextMarked {
      td_name: "richTextMarked".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextMarkedBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextMarkedBuilder {
  inner: RichTextMarked,
}

impl RTDRichTextMarkedBuilder {
  pub fn build(&self) -> RichTextMarked {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextMarked> for RichTextMarked {
  fn as_ref(&self) -> &RichTextMarked {
    self
  }
}

impl AsRef<RichTextMarked> for RTDRichTextMarkedBuilder {
  fn as_ref(&self) -> &RichTextMarked {
    &self.inner
  }
}

/// A mathematical expression
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextMathematicalExpression {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The expression in LaTeX format
  expression: String,
}

impl RObject for RichTextMathematicalExpression {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextMathematicalExpression"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextMathematicalExpression {}

impl RichTextMathematicalExpression {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextMathematicalExpressionBuilder {
    let inner = RichTextMathematicalExpression {
      td_name: "richTextMathematicalExpression".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextMathematicalExpressionBuilder { inner }
  }

  pub fn expression(&self) -> &String {
    &self.expression
  }
}

#[doc(hidden)]
pub struct RTDRichTextMathematicalExpressionBuilder {
  inner: RichTextMathematicalExpression,
}

impl RTDRichTextMathematicalExpressionBuilder {
  pub fn build(&self) -> RichTextMathematicalExpression {
    self.inner.clone()
  }

  pub fn expression<T: AsRef<str>>(&mut self, expression: T) -> &mut Self {
    self.inner.expression = expression.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextMathematicalExpression> for RichTextMathematicalExpression {
  fn as_ref(&self) -> &RichTextMathematicalExpression {
    self
  }
}

impl AsRef<RichTextMathematicalExpression> for RTDRichTextMathematicalExpressionBuilder {
  fn as_ref(&self) -> &RichTextMathematicalExpression {
    &self.inner
  }
}

/// A mention of a Telegram user or chat by a username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextMention {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// The username
  username: String,
}

impl RObject for RichTextMention {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextMention"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextMention {}

impl RichTextMention {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextMentionBuilder {
    let inner = RichTextMention {
      td_name: "richTextMention".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextMentionBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn username(&self) -> &String {
    &self.username
  }
}

#[doc(hidden)]
pub struct RTDRichTextMentionBuilder {
  inner: RichTextMention,
}

impl RTDRichTextMentionBuilder {
  pub fn build(&self) -> RichTextMention {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextMention> for RichTextMention {
  fn as_ref(&self) -> &RichTextMention {
    self
  }
}

impl AsRef<RichTextMention> for RTDRichTextMentionBuilder {
  fn as_ref(&self) -> &RichTextMention {
    &self.inner
  }
}

/// A rich text that serves as a mention of a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextMentionName {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// Identifier of the mentioned user
  user_id: i64,
}

impl RObject for RichTextMentionName {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextMentionName"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextMentionName {}

impl RichTextMentionName {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextMentionNameBuilder {
    let inner = RichTextMentionName {
      td_name: "richTextMentionName".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextMentionNameBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }
}

#[doc(hidden)]
pub struct RTDRichTextMentionNameBuilder {
  inner: RichTextMentionName,
}

impl RTDRichTextMentionNameBuilder {
  pub fn build(&self) -> RichTextMentionName {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }
}

impl AsRef<RichTextMentionName> for RichTextMentionName {
  fn as_ref(&self) -> &RichTextMentionName {
    self
  }
}

impl AsRef<RichTextMentionName> for RTDRichTextMentionNameBuilder {
  fn as_ref(&self) -> &RichTextMentionName {
    &self.inner
  }
}

/// A rich text phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// Phone number
  phone_number: String,
}

impl RObject for RichTextPhoneNumber {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextPhoneNumber"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextPhoneNumber {}

impl RichTextPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextPhoneNumberBuilder {
    let inner = RichTextPhoneNumber {
      td_name: "richTextPhoneNumber".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextPhoneNumberBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn phone_number(&self) -> &String {
    &self.phone_number
  }
}

#[doc(hidden)]
pub struct RTDRichTextPhoneNumberBuilder {
  inner: RichTextPhoneNumber,
}

impl RTDRichTextPhoneNumberBuilder {
  pub fn build(&self) -> RichTextPhoneNumber {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextPhoneNumber> for RichTextPhoneNumber {
  fn as_ref(&self) -> &RichTextPhoneNumber {
    self
  }
}

impl AsRef<RichTextPhoneNumber> for RTDRichTextPhoneNumberBuilder {
  fn as_ref(&self) -> &RichTextPhoneNumber {
    &self.inner
  }
}

/// A plain text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextPlain {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextPlain {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextPlain"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextPlain {}

impl RichTextPlain {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextPlainBuilder {
    let inner = RichTextPlain {
      td_name: "richTextPlain".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextPlainBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextPlainBuilder {
  inner: RichTextPlain,
}

impl RTDRichTextPlainBuilder {
  pub fn build(&self) -> RichTextPlain {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextPlain> for RichTextPlain {
  fn as_ref(&self) -> &RichTextPlain {
    self
  }
}

impl AsRef<RichTextPlain> for RTDRichTextPlainBuilder {
  fn as_ref(&self) -> &RichTextPlain {
    &self.inner
  }
}

/// A reference
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextReference {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Reference name
  name: String,
  /// Text of the reference
  text: Box<RichText>,
}

impl RObject for RichTextReference {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextReference"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextReference {}

impl RichTextReference {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextReferenceBuilder {
    let inner = RichTextReference {
      td_name: "richTextReference".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextReferenceBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextReferenceBuilder {
  inner: RichTextReference,
}

impl RTDRichTextReferenceBuilder {
  pub fn build(&self) -> RichTextReference {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextReference> for RichTextReference {
  fn as_ref(&self) -> &RichTextReference {
    self
  }
}

impl AsRef<RichTextReference> for RTDRichTextReferenceBuilder {
  fn as_ref(&self) -> &RichTextReference {
    &self.inner
  }
}

/// A link to a reference on the same page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextReferenceLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The link text
  text: Box<RichText>,
  /// The reference name
  reference_name: String,
  /// An HTTP URL that opens the reference
  url: String,
}

impl RObject for RichTextReferenceLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextReferenceLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextReferenceLink {}

impl RichTextReferenceLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextReferenceLinkBuilder {
    let inner = RichTextReferenceLink {
      td_name: "richTextReferenceLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextReferenceLinkBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn reference_name(&self) -> &String {
    &self.reference_name
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDRichTextReferenceLinkBuilder {
  inner: RichTextReferenceLink,
}

impl RTDRichTextReferenceLinkBuilder {
  pub fn build(&self) -> RichTextReferenceLink {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn reference_name<T: AsRef<str>>(&mut self, reference_name: T) -> &mut Self {
    self.inner.reference_name = reference_name.as_ref().to_string();
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextReferenceLink> for RichTextReferenceLink {
  fn as_ref(&self) -> &RichTextReferenceLink {
    self
  }
}

impl AsRef<RichTextReferenceLink> for RTDRichTextReferenceLinkBuilder {
  fn as_ref(&self) -> &RichTextReferenceLink {
    &self.inner
  }
}

/// A spoilered rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextSpoiler {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextSpoiler {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextSpoiler"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextSpoiler {}

impl RichTextSpoiler {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextSpoilerBuilder {
    let inner = RichTextSpoiler {
      td_name: "richTextSpoiler".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextSpoilerBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextSpoilerBuilder {
  inner: RichTextSpoiler,
}

impl RTDRichTextSpoilerBuilder {
  pub fn build(&self) -> RichTextSpoiler {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextSpoiler> for RichTextSpoiler {
  fn as_ref(&self) -> &RichTextSpoiler {
    self
  }
}

impl AsRef<RichTextSpoiler> for RTDRichTextSpoilerBuilder {
  fn as_ref(&self) -> &RichTextSpoiler {
    &self.inner
  }
}

/// A strikethrough rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextStrikethrough {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextStrikethrough {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextStrikethrough"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextStrikethrough {}

impl RichTextStrikethrough {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextStrikethroughBuilder {
    let inner = RichTextStrikethrough {
      td_name: "richTextStrikethrough".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextStrikethroughBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextStrikethroughBuilder {
  inner: RichTextStrikethrough,
}

impl RTDRichTextStrikethroughBuilder {
  pub fn build(&self) -> RichTextStrikethrough {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextStrikethrough> for RichTextStrikethrough {
  fn as_ref(&self) -> &RichTextStrikethrough {
    self
  }
}

impl AsRef<RichTextStrikethrough> for RTDRichTextStrikethroughBuilder {
  fn as_ref(&self) -> &RichTextStrikethrough {
    &self.inner
  }
}

/// A subscript rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextSubscript {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextSubscript {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextSubscript"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextSubscript {}

impl RichTextSubscript {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextSubscriptBuilder {
    let inner = RichTextSubscript {
      td_name: "richTextSubscript".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextSubscriptBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextSubscriptBuilder {
  inner: RichTextSubscript,
}

impl RTDRichTextSubscriptBuilder {
  pub fn build(&self) -> RichTextSubscript {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextSubscript> for RichTextSubscript {
  fn as_ref(&self) -> &RichTextSubscript {
    self
  }
}

impl AsRef<RichTextSubscript> for RTDRichTextSubscriptBuilder {
  fn as_ref(&self) -> &RichTextSubscript {
    &self.inner
  }
}

/// A superscript rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextSuperscript {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextSuperscript {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextSuperscript"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextSuperscript {}

impl RichTextSuperscript {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextSuperscriptBuilder {
    let inner = RichTextSuperscript {
      td_name: "richTextSuperscript".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextSuperscriptBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextSuperscriptBuilder {
  inner: RichTextSuperscript,
}

impl RTDRichTextSuperscriptBuilder {
  pub fn build(&self) -> RichTextSuperscript {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextSuperscript> for RichTextSuperscript {
  fn as_ref(&self) -> &RichTextSuperscript {
    self
  }
}

impl AsRef<RichTextSuperscript> for RTDRichTextSuperscriptBuilder {
  fn as_ref(&self) -> &RichTextSuperscript {
    &self.inner
  }
}

/// An address in the TON blockchain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextTonAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// The address
  address: String,
}

impl RObject for RichTextTonAddress {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextTonAddress"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextTonAddress {}

impl RichTextTonAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextTonAddressBuilder {
    let inner = RichTextTonAddress {
      td_name: "richTextTonAddress".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextTonAddressBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn address(&self) -> &String {
    &self.address
  }
}

#[doc(hidden)]
pub struct RTDRichTextTonAddressBuilder {
  inner: RichTextTonAddress,
}

impl RTDRichTextTonAddressBuilder {
  pub fn build(&self) -> RichTextTonAddress {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn address<T: AsRef<str>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().to_string();
    self
  }
}

impl AsRef<RichTextTonAddress> for RichTextTonAddress {
  fn as_ref(&self) -> &RichTextTonAddress {
    self
  }
}

impl AsRef<RichTextTonAddress> for RTDRichTextTonAddressBuilder {
  fn as_ref(&self) -> &RichTextTonAddress {
    &self.inner
  }
}

/// An underlined rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextUnderline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
}

impl RObject for RichTextUnderline {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextUnderline"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextUnderline {}

impl RichTextUnderline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextUnderlineBuilder {
    let inner = RichTextUnderline {
      td_name: "richTextUnderline".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextUnderlineBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDRichTextUnderlineBuilder {
  inner: RichTextUnderline,
}

impl RTDRichTextUnderlineBuilder {
  pub fn build(&self) -> RichTextUnderline {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }
}

impl AsRef<RichTextUnderline> for RichTextUnderline {
  fn as_ref(&self) -> &RichTextUnderline {
    self
  }
}

impl AsRef<RichTextUnderline> for RTDRichTextUnderlineBuilder {
  fn as_ref(&self) -> &RichTextUnderline {
    &self.inner
  }
}

/// A rich text URL link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: Box<RichText>,
  /// URL
  url: String,
  /// True, if the URL has cached instant view server-side; instant view only
  is_cached: bool,
}

impl RObject for RichTextUrl {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTextUrl"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTextUrl {}

impl RichTextUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextUrlBuilder {
    let inner = RichTextUrl {
      td_name: "richTextUrl".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextUrlBuilder { inner }
  }

  pub fn text(&self) -> &Box<RichText> {
    &self.text
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn is_cached(&self) -> bool {
    self.is_cached
  }
}

#[doc(hidden)]
pub struct RTDRichTextUrlBuilder {
  inner: RichTextUrl,
}

impl RTDRichTextUrlBuilder {
  pub fn build(&self) -> RichTextUrl {
    self.inner.clone()
  }

  pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn is_cached(&mut self, is_cached: bool) -> &mut Self {
    self.inner.is_cached = is_cached;
    self
  }
}

impl AsRef<RichTextUrl> for RichTextUrl {
  fn as_ref(&self) -> &RichTextUrl {
    self
  }
}

impl AsRef<RichTextUrl> for RTDRichTextUrlBuilder {
  fn as_ref(&self) -> &RichTextUrl {
    &self.inner
  }
}

/// A concatenation of rich texts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTexts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Texts
  texts: Vec<RichText>,
}

impl RObject for RichTexts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "richTexts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDRichText for RichTexts {}

impl RichTexts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDRichTextsBuilder {
    let inner = RichTexts {
      td_name: "richTexts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDRichTextsBuilder { inner }
  }

  pub fn texts(&self) -> &Vec<RichText> {
    &self.texts
  }
}

#[doc(hidden)]
pub struct RTDRichTextsBuilder {
  inner: RichTexts,
}

impl RTDRichTextsBuilder {
  pub fn build(&self) -> RichTexts {
    self.inner.clone()
  }

  pub fn texts(&mut self, texts: Vec<RichText>) -> &mut Self {
    self.inner.texts = texts;
    self
  }
}

impl AsRef<RichTexts> for RichTexts {
  fn as_ref(&self) -> &RichTexts {
    self
  }
}

impl AsRef<RichTexts> for RTDRichTextsBuilder {
  fn as_ref(&self) -> &RichTexts {
    &self.inner
  }
}
