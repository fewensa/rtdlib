use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Telegram Business account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Location of the business; may be null if none
  location: Option<BusinessLocation>,
  /// Opening hours of the business; may be null if none. The hours are guaranteed to be valid and have already been split by week days
  opening_hours: Option<BusinessOpeningHours>,
  /// Opening hours of the business in the local time; may be null if none. The hours are guaranteed to be valid and have already been split by week days. Local time zone identifier will be empty. An updateUserFullInfo update is not triggered when value of this field changes
  local_opening_hours: Option<BusinessOpeningHours>,
  /// Time left before the business will open the next time, in seconds; 0 if unknown. An updateUserFullInfo update is not triggered when value of this field changes
  next_open_in: i64,
  /// Time left before the business will close the next time, in seconds; 0 if unknown. An updateUserFullInfo update is not triggered when value of this field changes
  next_close_in: i64,
  /// The greeting message; may be null if none or the Business account is not of the current user
  greeting_message_settings: Option<BusinessGreetingMessageSettings>,
  /// The away message; may be null if none or the Business account is not of the current user
  away_message_settings: Option<BusinessAwayMessageSettings>,
  /// Information about start page of the account; may be null if none
  start_page: Option<BusinessStartPage>,
}

impl RObject for BusinessInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessInfoBuilder {
    let inner = BusinessInfo {
      td_name: "businessInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessInfoBuilder { inner }
  }

  pub fn location(&self) -> &Option<BusinessLocation> {
    &self.location
  }

  pub fn opening_hours(&self) -> &Option<BusinessOpeningHours> {
    &self.opening_hours
  }

  pub fn local_opening_hours(&self) -> &Option<BusinessOpeningHours> {
    &self.local_opening_hours
  }

  pub fn next_open_in(&self) -> i64 {
    self.next_open_in
  }

  pub fn next_close_in(&self) -> i64 {
    self.next_close_in
  }

  pub fn greeting_message_settings(&self) -> &Option<BusinessGreetingMessageSettings> {
    &self.greeting_message_settings
  }

  pub fn away_message_settings(&self) -> &Option<BusinessAwayMessageSettings> {
    &self.away_message_settings
  }

  pub fn start_page(&self) -> &Option<BusinessStartPage> {
    &self.start_page
  }
}

#[doc(hidden)]
pub struct RTDBusinessInfoBuilder {
  inner: BusinessInfo,
}

impl RTDBusinessInfoBuilder {
  pub fn build(&self) -> BusinessInfo {
    self.inner.clone()
  }

  pub fn location<T: AsRef<BusinessLocation>>(&mut self, location: T) -> &mut Self {
    self.inner.location = Some(location.as_ref().clone());
    self
  }

  pub fn opening_hours<T: AsRef<BusinessOpeningHours>>(&mut self, opening_hours: T) -> &mut Self {
    self.inner.opening_hours = Some(opening_hours.as_ref().clone());
    self
  }

  pub fn local_opening_hours<T: AsRef<BusinessOpeningHours>>(
    &mut self,
    local_opening_hours: T,
  ) -> &mut Self {
    self.inner.local_opening_hours = Some(local_opening_hours.as_ref().clone());
    self
  }

  pub fn next_open_in(&mut self, next_open_in: i64) -> &mut Self {
    self.inner.next_open_in = next_open_in;
    self
  }

  pub fn next_close_in(&mut self, next_close_in: i64) -> &mut Self {
    self.inner.next_close_in = next_close_in;
    self
  }

  pub fn greeting_message_settings<T: AsRef<BusinessGreetingMessageSettings>>(
    &mut self,
    greeting_message_settings: T,
  ) -> &mut Self {
    self.inner.greeting_message_settings = Some(greeting_message_settings.as_ref().clone());
    self
  }

  pub fn away_message_settings<T: AsRef<BusinessAwayMessageSettings>>(
    &mut self,
    away_message_settings: T,
  ) -> &mut Self {
    self.inner.away_message_settings = Some(away_message_settings.as_ref().clone());
    self
  }

  pub fn start_page<T: AsRef<BusinessStartPage>>(&mut self, start_page: T) -> &mut Self {
    self.inner.start_page = Some(start_page.as_ref().clone());
    self
  }
}

impl AsRef<BusinessInfo> for BusinessInfo {
  fn as_ref(&self) -> &BusinessInfo {
    self
  }
}

impl AsRef<BusinessInfo> for RTDBusinessInfoBuilder {
  fn as_ref(&self) -> &BusinessInfo {
    &self.inner
  }
}
