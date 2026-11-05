use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an active affiliate program
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffiliateProgramInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Parameters of the affiliate program
  parameters: AffiliateProgramParameters,
  /// Point in time (Unix timestamp) when the affiliate program will be closed; 0 if the affiliate program isn't scheduled to be closed. If positive, then the program can't be connected using connectAffiliateProgram, but active connections will work until the date
  end_date: i64,
  /// The amount of daily revenue per user in Telegram Stars of the bot that created the affiliate program
  daily_revenue_per_user_amount: StarAmount,
}

impl RObject for AffiliateProgramInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "affiliateProgramInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AffiliateProgramInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAffiliateProgramInfoBuilder {
    let inner = AffiliateProgramInfo {
      td_name: "affiliateProgramInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAffiliateProgramInfoBuilder { inner }
  }

  pub fn parameters(&self) -> &AffiliateProgramParameters {
    &self.parameters
  }

  pub fn end_date(&self) -> i64 {
    self.end_date
  }

  pub fn daily_revenue_per_user_amount(&self) -> &StarAmount {
    &self.daily_revenue_per_user_amount
  }
}

#[doc(hidden)]
pub struct RTDAffiliateProgramInfoBuilder {
  inner: AffiliateProgramInfo,
}

impl RTDAffiliateProgramInfoBuilder {
  pub fn build(&self) -> AffiliateProgramInfo {
    self.inner.clone()
  }

  pub fn parameters<T: AsRef<AffiliateProgramParameters>>(&mut self, parameters: T) -> &mut Self {
    self.inner.parameters = parameters.as_ref().clone();
    self
  }

  pub fn end_date(&mut self, end_date: i64) -> &mut Self {
    self.inner.end_date = end_date;
    self
  }

  pub fn daily_revenue_per_user_amount<T: AsRef<StarAmount>>(
    &mut self,
    daily_revenue_per_user_amount: T,
  ) -> &mut Self {
    self.inner.daily_revenue_per_user_amount = daily_revenue_per_user_amount.as_ref().clone();
    self
  }
}

impl AsRef<AffiliateProgramInfo> for AffiliateProgramInfo {
  fn as_ref(&self) -> &AffiliateProgramInfo {
    self
  }
}

impl AsRef<AffiliateProgramInfo> for RTDAffiliateProgramInfoBuilder {
  fn as_ref(&self) -> &AffiliateProgramInfo {
    &self.inner
  }
}
