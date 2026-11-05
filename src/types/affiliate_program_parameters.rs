use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes parameters of an affiliate program
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffiliateProgramParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the program owner; getOption("affiliate_program_commission_per_mille_min")-getOption("affiliate_program_commission_per_mille_max")
  commission_per_mille: i64,
  /// Number of months the program will be active; 0-36. If 0, then the program is eternal
  month_count: i64,
}

impl RObject for AffiliateProgramParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "affiliateProgramParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AffiliateProgramParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAffiliateProgramParametersBuilder {
    let inner = AffiliateProgramParameters {
      td_name: "affiliateProgramParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAffiliateProgramParametersBuilder { inner }
  }

  pub fn commission_per_mille(&self) -> i64 {
    self.commission_per_mille
  }

  pub fn month_count(&self) -> i64 {
    self.month_count
  }
}

#[doc(hidden)]
pub struct RTDAffiliateProgramParametersBuilder {
  inner: AffiliateProgramParameters,
}

impl RTDAffiliateProgramParametersBuilder {
  pub fn build(&self) -> AffiliateProgramParameters {
    self.inner.clone()
  }

  pub fn commission_per_mille(&mut self, commission_per_mille: i64) -> &mut Self {
    self.inner.commission_per_mille = commission_per_mille;
    self
  }

  pub fn month_count(&mut self, month_count: i64) -> &mut Self {
    self.inner.month_count = month_count;
    self
  }
}

impl AsRef<AffiliateProgramParameters> for AffiliateProgramParameters {
  fn as_ref(&self) -> &AffiliateProgramParameters {
    self
  }
}

impl AsRef<AffiliateProgramParameters> for RTDAffiliateProgramParametersBuilder {
  fn as_ref(&self) -> &AffiliateProgramParameters {
    &self.inner
  }
}
