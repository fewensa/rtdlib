use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a bot, which can be added to attachment or side menu
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentMenuBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the bot
  bot_user_id: i64,
  /// True, if the bot supports opening from attachment menu in the chat with the bot
  supports_self_chat: bool,
  /// True, if the bot supports opening from attachment menu in private chats with ordinary users
  supports_user_chats: bool,
  /// True, if the bot supports opening from attachment menu in private chats with other bots
  supports_bot_chats: bool,
  /// True, if the bot supports opening from attachment menu in basic group and supergroup chats
  supports_group_chats: bool,
  /// True, if the bot supports opening from attachment menu in channel chats
  supports_channel_chats: bool,
  /// True, if the user must be asked for the permission to send messages to the bot
  request_write_access: bool,
  /// True, if the bot was explicitly added by the user. If the bot isn't added, then on the first bot launch toggleBotIsAddedToAttachmentMenu must be called and the bot must be added or removed
  is_added: bool,
  /// True, if the bot must be shown in the attachment menu
  show_in_attachment_menu: bool,
  /// True, if the bot must be shown in the side menu
  show_in_side_menu: bool,
  /// True, if a disclaimer, why the bot is shown in the side menu, is needed
  show_disclaimer_in_side_menu: bool,
  /// Name for the bot in attachment menu
  name: String,
  /// Color to highlight selected name of the bot if appropriate; may be null
  name_color: Option<AttachmentMenuBotColor>,
  /// Default icon for the bot in SVG format; may be null
  default_icon: Option<File>,
  /// Icon for the bot in SVG format for the official iOS app; may be null
  ios_static_icon: Option<File>,
  /// Icon for the bot in TGS format for the official iOS app; may be null
  ios_animated_icon: Option<File>,
  /// Icon for the bot in PNG format for the official iOS app side menu; may be null
  ios_side_menu_icon: Option<File>,
  /// Icon for the bot in TGS format for the official Android app; may be null
  android_icon: Option<File>,
  /// Icon for the bot in SVG format for the official Android app side menu; may be null
  android_side_menu_icon: Option<File>,
  /// Icon for the bot in TGS format for the official native macOS app; may be null
  macos_icon: Option<File>,
  /// Icon for the bot in PNG format for the official macOS app side menu; may be null
  macos_side_menu_icon: Option<File>,
  /// Color to highlight selected icon of the bot if appropriate; may be null
  icon_color: Option<AttachmentMenuBotColor>,
  /// Default placeholder for opened Web Apps in SVG format; may be null
  web_app_placeholder: Option<File>,
}

impl RObject for AttachmentMenuBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "attachmentMenuBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AttachmentMenuBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAttachmentMenuBotBuilder {
    let inner = AttachmentMenuBot {
      td_name: "attachmentMenuBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAttachmentMenuBotBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn supports_self_chat(&self) -> bool {
    self.supports_self_chat
  }

  pub fn supports_user_chats(&self) -> bool {
    self.supports_user_chats
  }

  pub fn supports_bot_chats(&self) -> bool {
    self.supports_bot_chats
  }

  pub fn supports_group_chats(&self) -> bool {
    self.supports_group_chats
  }

  pub fn supports_channel_chats(&self) -> bool {
    self.supports_channel_chats
  }

  pub fn request_write_access(&self) -> bool {
    self.request_write_access
  }

  pub fn is_added(&self) -> bool {
    self.is_added
  }

  pub fn show_in_attachment_menu(&self) -> bool {
    self.show_in_attachment_menu
  }

  pub fn show_in_side_menu(&self) -> bool {
    self.show_in_side_menu
  }

  pub fn show_disclaimer_in_side_menu(&self) -> bool {
    self.show_disclaimer_in_side_menu
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn name_color(&self) -> &Option<AttachmentMenuBotColor> {
    &self.name_color
  }

  pub fn default_icon(&self) -> &Option<File> {
    &self.default_icon
  }

  pub fn ios_static_icon(&self) -> &Option<File> {
    &self.ios_static_icon
  }

  pub fn ios_animated_icon(&self) -> &Option<File> {
    &self.ios_animated_icon
  }

  pub fn ios_side_menu_icon(&self) -> &Option<File> {
    &self.ios_side_menu_icon
  }

  pub fn android_icon(&self) -> &Option<File> {
    &self.android_icon
  }

  pub fn android_side_menu_icon(&self) -> &Option<File> {
    &self.android_side_menu_icon
  }

  pub fn macos_icon(&self) -> &Option<File> {
    &self.macos_icon
  }

  pub fn macos_side_menu_icon(&self) -> &Option<File> {
    &self.macos_side_menu_icon
  }

  pub fn icon_color(&self) -> &Option<AttachmentMenuBotColor> {
    &self.icon_color
  }

  pub fn web_app_placeholder(&self) -> &Option<File> {
    &self.web_app_placeholder
  }
}

#[doc(hidden)]
pub struct RTDAttachmentMenuBotBuilder {
  inner: AttachmentMenuBot,
}

impl RTDAttachmentMenuBotBuilder {
  pub fn build(&self) -> AttachmentMenuBot {
    self.inner.clone()
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn supports_self_chat(&mut self, supports_self_chat: bool) -> &mut Self {
    self.inner.supports_self_chat = supports_self_chat;
    self
  }

  pub fn supports_user_chats(&mut self, supports_user_chats: bool) -> &mut Self {
    self.inner.supports_user_chats = supports_user_chats;
    self
  }

  pub fn supports_bot_chats(&mut self, supports_bot_chats: bool) -> &mut Self {
    self.inner.supports_bot_chats = supports_bot_chats;
    self
  }

  pub fn supports_group_chats(&mut self, supports_group_chats: bool) -> &mut Self {
    self.inner.supports_group_chats = supports_group_chats;
    self
  }

  pub fn supports_channel_chats(&mut self, supports_channel_chats: bool) -> &mut Self {
    self.inner.supports_channel_chats = supports_channel_chats;
    self
  }

  pub fn request_write_access(&mut self, request_write_access: bool) -> &mut Self {
    self.inner.request_write_access = request_write_access;
    self
  }

  pub fn is_added(&mut self, is_added: bool) -> &mut Self {
    self.inner.is_added = is_added;
    self
  }

  pub fn show_in_attachment_menu(&mut self, show_in_attachment_menu: bool) -> &mut Self {
    self.inner.show_in_attachment_menu = show_in_attachment_menu;
    self
  }

  pub fn show_in_side_menu(&mut self, show_in_side_menu: bool) -> &mut Self {
    self.inner.show_in_side_menu = show_in_side_menu;
    self
  }

  pub fn show_disclaimer_in_side_menu(&mut self, show_disclaimer_in_side_menu: bool) -> &mut Self {
    self.inner.show_disclaimer_in_side_menu = show_disclaimer_in_side_menu;
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn name_color<T: AsRef<AttachmentMenuBotColor>>(&mut self, name_color: T) -> &mut Self {
    self.inner.name_color = Some(name_color.as_ref().clone());
    self
  }

  pub fn default_icon<T: AsRef<File>>(&mut self, default_icon: T) -> &mut Self {
    self.inner.default_icon = Some(default_icon.as_ref().clone());
    self
  }

  pub fn ios_static_icon<T: AsRef<File>>(&mut self, ios_static_icon: T) -> &mut Self {
    self.inner.ios_static_icon = Some(ios_static_icon.as_ref().clone());
    self
  }

  pub fn ios_animated_icon<T: AsRef<File>>(&mut self, ios_animated_icon: T) -> &mut Self {
    self.inner.ios_animated_icon = Some(ios_animated_icon.as_ref().clone());
    self
  }

  pub fn ios_side_menu_icon<T: AsRef<File>>(&mut self, ios_side_menu_icon: T) -> &mut Self {
    self.inner.ios_side_menu_icon = Some(ios_side_menu_icon.as_ref().clone());
    self
  }

  pub fn android_icon<T: AsRef<File>>(&mut self, android_icon: T) -> &mut Self {
    self.inner.android_icon = Some(android_icon.as_ref().clone());
    self
  }

  pub fn android_side_menu_icon<T: AsRef<File>>(&mut self, android_side_menu_icon: T) -> &mut Self {
    self.inner.android_side_menu_icon = Some(android_side_menu_icon.as_ref().clone());
    self
  }

  pub fn macos_icon<T: AsRef<File>>(&mut self, macos_icon: T) -> &mut Self {
    self.inner.macos_icon = Some(macos_icon.as_ref().clone());
    self
  }

  pub fn macos_side_menu_icon<T: AsRef<File>>(&mut self, macos_side_menu_icon: T) -> &mut Self {
    self.inner.macos_side_menu_icon = Some(macos_side_menu_icon.as_ref().clone());
    self
  }

  pub fn icon_color<T: AsRef<AttachmentMenuBotColor>>(&mut self, icon_color: T) -> &mut Self {
    self.inner.icon_color = Some(icon_color.as_ref().clone());
    self
  }

  pub fn web_app_placeholder<T: AsRef<File>>(&mut self, web_app_placeholder: T) -> &mut Self {
    self.inner.web_app_placeholder = Some(web_app_placeholder.as_ref().clone());
    self
  }
}

impl AsRef<AttachmentMenuBot> for AttachmentMenuBot {
  fn as_ref(&self) -> &AttachmentMenuBot {
    self
  }
}

impl AsRef<AttachmentMenuBot> for RTDAttachmentMenuBotBuilder {
  fn as_ref(&self) -> &AttachmentMenuBot {
    &self.inner
  }
}
