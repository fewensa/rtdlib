use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes parameters for age verification of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgeVerificationParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The minimum age required to view restricted content
  min_age: i64,
  /// Username of the bot which main Web App may be used to verify age of the user
  verification_bot_username: String,
  /// Unique name for the country or region, which legislation required age verification. May be used to get the corresponding localization key
  country: String,
}

impl RObject for AgeVerificationParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "ageVerificationParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AgeVerificationParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAgeVerificationParametersBuilder {
    let inner = AgeVerificationParameters {
      td_name: "ageVerificationParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAgeVerificationParametersBuilder { inner }
  }

  pub fn min_age(&self) -> i64 {
    self.min_age
  }

  pub fn verification_bot_username(&self) -> &String {
    &self.verification_bot_username
  }

  pub fn country(&self) -> &String {
    &self.country
  }
}

#[doc(hidden)]
pub struct RTDAgeVerificationParametersBuilder {
  inner: AgeVerificationParameters,
}

impl RTDAgeVerificationParametersBuilder {
  pub fn build(&self) -> AgeVerificationParameters {
    self.inner.clone()
  }

  pub fn min_age(&mut self, min_age: i64) -> &mut Self {
    self.inner.min_age = min_age;
    self
  }

  pub fn verification_bot_username<T: AsRef<str>>(
    &mut self,
    verification_bot_username: T,
  ) -> &mut Self {
    self.inner.verification_bot_username = verification_bot_username.as_ref().to_string();
    self
  }

  pub fn country<T: AsRef<str>>(&mut self, country: T) -> &mut Self {
    self.inner.country = country.as_ref().to_string();
    self
  }
}

impl AsRef<AgeVerificationParameters> for AgeVerificationParameters {
  fn as_ref(&self) -> &AgeVerificationParameters {
    self
  }
}

impl AsRef<AgeVerificationParameters> for RTDAgeVerificationParametersBuilder {
  fn as_ref(&self) -> &AgeVerificationParameters {
    &self.inner
  }
}
