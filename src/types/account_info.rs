use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains basic information about another user who started a chat with the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccountInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Month when the user was registered in Telegram; 0-12; may be 0 if unknown
  registration_month: i64,
  /// Year when the user was registered in Telegram; 0-9999; may be 0 if unknown
  registration_year: i64,
  /// A two-letter ISO 3166-1 alpha-2 country code based on the phone number of the user; may be empty if unknown
  phone_number_country_code: String,
  /// Point in time (Unix timestamp) when the user changed name last time; 0 if unknown
  last_name_change_date: i64,
  /// Point in time (Unix timestamp) when the user changed photo last time; 0 if unknown
  last_photo_change_date: i64,
}

impl RObject for AccountInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "accountInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AccountInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAccountInfoBuilder {
    let inner = AccountInfo {
      td_name: "accountInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAccountInfoBuilder { inner }
  }

  pub fn registration_month(&self) -> i64 {
    self.registration_month
  }

  pub fn registration_year(&self) -> i64 {
    self.registration_year
  }

  pub fn phone_number_country_code(&self) -> &String {
    &self.phone_number_country_code
  }

  pub fn last_name_change_date(&self) -> i64 {
    self.last_name_change_date
  }

  pub fn last_photo_change_date(&self) -> i64 {
    self.last_photo_change_date
  }
}

#[doc(hidden)]
pub struct RTDAccountInfoBuilder {
  inner: AccountInfo,
}

impl RTDAccountInfoBuilder {
  pub fn build(&self) -> AccountInfo {
    self.inner.clone()
  }

  pub fn registration_month(&mut self, registration_month: i64) -> &mut Self {
    self.inner.registration_month = registration_month;
    self
  }

  pub fn registration_year(&mut self, registration_year: i64) -> &mut Self {
    self.inner.registration_year = registration_year;
    self
  }

  pub fn phone_number_country_code<T: AsRef<str>>(
    &mut self,
    phone_number_country_code: T,
  ) -> &mut Self {
    self.inner.phone_number_country_code = phone_number_country_code.as_ref().to_string();
    self
  }

  pub fn last_name_change_date(&mut self, last_name_change_date: i64) -> &mut Self {
    self.inner.last_name_change_date = last_name_change_date;
    self
  }

  pub fn last_photo_change_date(&mut self, last_photo_change_date: i64) -> &mut Self {
    self.inner.last_photo_change_date = last_photo_change_date;
    self
  }
}

impl AsRef<AccountInfo> for AccountInfo {
  fn as_ref(&self) -> &AccountInfo {
    self
  }
}

impl AsRef<AccountInfo> for RTDAccountInfoBuilder {
  fn as_ref(&self) -> &AccountInfo {
    &self.inner
  }
}
