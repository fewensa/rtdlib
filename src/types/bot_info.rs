use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The text that is shown on the bot's profile page and is sent together with the link when users share the bot
  short_description: String,
  /// Contains information about a bot
  description: String,
  /// Photo shown in the chat with the bot if the chat is empty; may be null
  photo: Option<Photo>,
  /// Animation shown in the chat with the bot if the chat is empty; may be null
  animation: Option<Animation>,
  /// Identifier of the bot, which manages the bot; 0 if none or unknown; for owner of the bot only
  manager_bot_user_id: i64,
  /// Information about a button to show instead of the bot commands menu button; may be null if ordinary bot commands menu must be shown
  menu_button: Option<BotMenuButton>,
  /// List of the bot commands
  commands: Vec<BotCommand>,
  /// The HTTP link to the privacy policy of the bot. If empty, then /privacy command must be used if supported by the bot. If the command isn't supported, then https://telegram.org/privacy-tpa must be opened
  privacy_policy_url: String,
  /// Default administrator rights for adding the bot to basic group and supergroup chats; may be null
  default_group_administrator_rights: Option<ChatAdministratorRights>,
  /// Default administrator rights for adding the bot to channels; may be null
  default_channel_administrator_rights: Option<ChatAdministratorRights>,
  /// Information about the affiliate program of the bot; may be null if none
  affiliate_program: Option<AffiliateProgramInfo>,
  /// Default light background color for bot Web Apps; 1 if not specified
  web_app_background_light_color: i64,
  /// Default dark background color for bot Web Apps; 1 if not specified
  web_app_background_dark_color: i64,
  /// Default light header color for bot Web Apps; 1 if not specified
  web_app_header_light_color: i64,
  /// Default dark header color for bot Web Apps; 1 if not specified
  web_app_header_dark_color: i64,
  /// Parameters of the verification that can be provided by the bot; may be null if none or the current user isn't the owner of the bot
  verification_parameters: Option<BotVerificationParameters>,
  /// True, if the bot's revenue statistics are available to the current user
  can_get_revenue_statistics: bool,
  /// True, if the bot can manage emoji status of the current user
  can_manage_emoji_status: bool,
  /// True, if the bot has media previews
  has_media_previews: bool,
  /// The internal link, which can be used to edit bot commands; may be null
  edit_commands_link: Option<InternalLinkType>,
  /// The internal link, which can be used to edit bot description; may be null
  edit_description_link: Option<InternalLinkType>,
  /// The internal link, which can be used to edit the photo or animation shown in the chat with the bot if the chat is empty; may be null
  edit_description_media_link: Option<InternalLinkType>,
  /// The internal link, which can be used to edit bot settings; may be null
  edit_settings_link: Option<InternalLinkType>,
}

impl RObject for BotInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BotInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotInfoBuilder {
    let inner = BotInfo {
      td_name: "botInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotInfoBuilder { inner }
  }

  pub fn short_description(&self) -> &String {
    &self.short_description
  }

  pub fn description(&self) -> &String {
    &self.description
  }

  pub fn photo(&self) -> &Option<Photo> {
    &self.photo
  }

  pub fn animation(&self) -> &Option<Animation> {
    &self.animation
  }

  pub fn manager_bot_user_id(&self) -> i64 {
    self.manager_bot_user_id
  }

  pub fn menu_button(&self) -> &Option<BotMenuButton> {
    &self.menu_button
  }

  pub fn commands(&self) -> &Vec<BotCommand> {
    &self.commands
  }

  pub fn privacy_policy_url(&self) -> &String {
    &self.privacy_policy_url
  }

  pub fn default_group_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
    &self.default_group_administrator_rights
  }

  pub fn default_channel_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
    &self.default_channel_administrator_rights
  }

  pub fn affiliate_program(&self) -> &Option<AffiliateProgramInfo> {
    &self.affiliate_program
  }

  pub fn web_app_background_light_color(&self) -> i64 {
    self.web_app_background_light_color
  }

  pub fn web_app_background_dark_color(&self) -> i64 {
    self.web_app_background_dark_color
  }

  pub fn web_app_header_light_color(&self) -> i64 {
    self.web_app_header_light_color
  }

  pub fn web_app_header_dark_color(&self) -> i64 {
    self.web_app_header_dark_color
  }

  pub fn verification_parameters(&self) -> &Option<BotVerificationParameters> {
    &self.verification_parameters
  }

  pub fn can_get_revenue_statistics(&self) -> bool {
    self.can_get_revenue_statistics
  }

  pub fn can_manage_emoji_status(&self) -> bool {
    self.can_manage_emoji_status
  }

  pub fn has_media_previews(&self) -> bool {
    self.has_media_previews
  }

  pub fn edit_commands_link(&self) -> &Option<InternalLinkType> {
    &self.edit_commands_link
  }

  pub fn edit_description_link(&self) -> &Option<InternalLinkType> {
    &self.edit_description_link
  }

  pub fn edit_description_media_link(&self) -> &Option<InternalLinkType> {
    &self.edit_description_media_link
  }

  pub fn edit_settings_link(&self) -> &Option<InternalLinkType> {
    &self.edit_settings_link
  }
}

#[doc(hidden)]
pub struct RTDBotInfoBuilder {
  inner: BotInfo,
}

impl RTDBotInfoBuilder {
  pub fn build(&self) -> BotInfo {
    self.inner.clone()
  }

  pub fn short_description<T: AsRef<str>>(&mut self, short_description: T) -> &mut Self {
    self.inner.short_description = short_description.as_ref().to_string();
    self
  }

  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = Some(animation.as_ref().clone());
    self
  }

  pub fn manager_bot_user_id(&mut self, manager_bot_user_id: i64) -> &mut Self {
    self.inner.manager_bot_user_id = manager_bot_user_id;
    self
  }

  pub fn menu_button<T: AsRef<BotMenuButton>>(&mut self, menu_button: T) -> &mut Self {
    self.inner.menu_button = Some(menu_button.as_ref().clone());
    self
  }

  pub fn commands(&mut self, commands: Vec<BotCommand>) -> &mut Self {
    self.inner.commands = commands;
    self
  }

  pub fn privacy_policy_url<T: AsRef<str>>(&mut self, privacy_policy_url: T) -> &mut Self {
    self.inner.privacy_policy_url = privacy_policy_url.as_ref().to_string();
    self
  }

  pub fn default_group_administrator_rights<T: AsRef<ChatAdministratorRights>>(
    &mut self,
    default_group_administrator_rights: T,
  ) -> &mut Self {
    self.inner.default_group_administrator_rights =
      Some(default_group_administrator_rights.as_ref().clone());
    self
  }

  pub fn default_channel_administrator_rights<T: AsRef<ChatAdministratorRights>>(
    &mut self,
    default_channel_administrator_rights: T,
  ) -> &mut Self {
    self.inner.default_channel_administrator_rights =
      Some(default_channel_administrator_rights.as_ref().clone());
    self
  }

  pub fn affiliate_program<T: AsRef<AffiliateProgramInfo>>(
    &mut self,
    affiliate_program: T,
  ) -> &mut Self {
    self.inner.affiliate_program = Some(affiliate_program.as_ref().clone());
    self
  }

  pub fn web_app_background_light_color(
    &mut self,
    web_app_background_light_color: i64,
  ) -> &mut Self {
    self.inner.web_app_background_light_color = web_app_background_light_color;
    self
  }

  pub fn web_app_background_dark_color(&mut self, web_app_background_dark_color: i64) -> &mut Self {
    self.inner.web_app_background_dark_color = web_app_background_dark_color;
    self
  }

  pub fn web_app_header_light_color(&mut self, web_app_header_light_color: i64) -> &mut Self {
    self.inner.web_app_header_light_color = web_app_header_light_color;
    self
  }

  pub fn web_app_header_dark_color(&mut self, web_app_header_dark_color: i64) -> &mut Self {
    self.inner.web_app_header_dark_color = web_app_header_dark_color;
    self
  }

  pub fn verification_parameters<T: AsRef<BotVerificationParameters>>(
    &mut self,
    verification_parameters: T,
  ) -> &mut Self {
    self.inner.verification_parameters = Some(verification_parameters.as_ref().clone());
    self
  }

  pub fn can_get_revenue_statistics(&mut self, can_get_revenue_statistics: bool) -> &mut Self {
    self.inner.can_get_revenue_statistics = can_get_revenue_statistics;
    self
  }

  pub fn can_manage_emoji_status(&mut self, can_manage_emoji_status: bool) -> &mut Self {
    self.inner.can_manage_emoji_status = can_manage_emoji_status;
    self
  }

  pub fn has_media_previews(&mut self, has_media_previews: bool) -> &mut Self {
    self.inner.has_media_previews = has_media_previews;
    self
  }

  pub fn edit_commands_link<T: AsRef<InternalLinkType>>(
    &mut self,
    edit_commands_link: T,
  ) -> &mut Self {
    self.inner.edit_commands_link = Some(edit_commands_link.as_ref().clone());
    self
  }

  pub fn edit_description_link<T: AsRef<InternalLinkType>>(
    &mut self,
    edit_description_link: T,
  ) -> &mut Self {
    self.inner.edit_description_link = Some(edit_description_link.as_ref().clone());
    self
  }

  pub fn edit_description_media_link<T: AsRef<InternalLinkType>>(
    &mut self,
    edit_description_media_link: T,
  ) -> &mut Self {
    self.inner.edit_description_media_link = Some(edit_description_media_link.as_ref().clone());
    self
  }

  pub fn edit_settings_link<T: AsRef<InternalLinkType>>(
    &mut self,
    edit_settings_link: T,
  ) -> &mut Self {
    self.inner.edit_settings_link = Some(edit_settings_link.as_ref().clone());
    self
  }
}

impl AsRef<BotInfo> for BotInfo {
  fn as_ref(&self) -> &BotInfo {
    self
  }
}

impl AsRef<BotInfo> for RTDBotInfoBuilder {
  fn as_ref(&self) -> &BotInfo {
    &self.inner
  }
}
