use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an affiliate program that was connected to an affiliate
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectedAffiliateProgram {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The link that can be used to refer users if the program is still active
  url: String,
  /// User identifier of the bot that created the program
  bot_user_id: i64,
  /// The parameters of the affiliate program
  parameters: AffiliateProgramParameters,
  /// Point in time (Unix timestamp) when the affiliate program was connected
  connection_date: i64,
  /// True, if the program was canceled by the bot, or disconnected by the chat owner and isn't available anymore
  is_disconnected: bool,
  /// The number of users that used the affiliate program
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  user_count: isize,
  /// The number of Telegram Stars that were earned by the affiliate program
  revenue_star_count: i64,
}

impl RObject for ConnectedAffiliateProgram {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "connectedAffiliateProgram"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ConnectedAffiliateProgram {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDConnectedAffiliateProgramBuilder {
    let inner = ConnectedAffiliateProgram {
      td_name: "connectedAffiliateProgram".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDConnectedAffiliateProgramBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn parameters(&self) -> &AffiliateProgramParameters {
    &self.parameters
  }

  pub fn connection_date(&self) -> i64 {
    self.connection_date
  }

  pub fn is_disconnected(&self) -> bool {
    self.is_disconnected
  }

  pub fn user_count(&self) -> isize {
    self.user_count
  }

  pub fn revenue_star_count(&self) -> i64 {
    self.revenue_star_count
  }
}

#[doc(hidden)]
pub struct RTDConnectedAffiliateProgramBuilder {
  inner: ConnectedAffiliateProgram,
}

impl RTDConnectedAffiliateProgramBuilder {
  pub fn build(&self) -> ConnectedAffiliateProgram {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn parameters<T: AsRef<AffiliateProgramParameters>>(&mut self, parameters: T) -> &mut Self {
    self.inner.parameters = parameters.as_ref().clone();
    self
  }

  pub fn connection_date(&mut self, connection_date: i64) -> &mut Self {
    self.inner.connection_date = connection_date;
    self
  }

  pub fn is_disconnected(&mut self, is_disconnected: bool) -> &mut Self {
    self.inner.is_disconnected = is_disconnected;
    self
  }

  pub fn user_count(&mut self, user_count: isize) -> &mut Self {
    self.inner.user_count = user_count;
    self
  }

  pub fn revenue_star_count(&mut self, revenue_star_count: i64) -> &mut Self {
    self.inner.revenue_star_count = revenue_star_count;
    self
  }
}

impl AsRef<ConnectedAffiliateProgram> for ConnectedAffiliateProgram {
  fn as_ref(&self) -> &ConnectedAffiliateProgram {
    self
  }
}

impl AsRef<ConnectedAffiliateProgram> for RTDConnectedAffiliateProgramBuilder {
  fn as_ref(&self) -> &ConnectedAffiliateProgram {
    &self.inner
  }
}
