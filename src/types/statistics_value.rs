
use crate::types::*;
use crate::errors::*;




/// A statistics value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticsValue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The value
  value: f32,
  /// The value for the previous day
  previous_value: f32,
  /// The growth rate of the value, as a percentage
  growth_rate_percentage: f32,
  
}

impl RObject for StatisticsValue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "statisticsValue" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl StatisticsValue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStatisticsValueBuilder {
    let mut inner = StatisticsValue::default();
    inner.td_name = "statisticsValue".to_string();
    RTDStatisticsValueBuilder { inner }
  }

  pub fn value(&self) -> f32 { self.value }

  pub fn previous_value(&self) -> f32 { self.previous_value }

  pub fn growth_rate_percentage(&self) -> f32 { self.growth_rate_percentage }

}

#[doc(hidden)]
pub struct RTDStatisticsValueBuilder {
  inner: StatisticsValue
}

impl RTDStatisticsValueBuilder {
  pub fn build(&self) -> StatisticsValue { self.inner.clone() }

   
  pub fn value(&mut self, value: f32) -> &mut Self {
    self.inner.value = value;
    self
  }

   
  pub fn previous_value(&mut self, previous_value: f32) -> &mut Self {
    self.inner.previous_value = previous_value;
    self
  }

   
  pub fn growth_rate_percentage(&mut self, growth_rate_percentage: f32) -> &mut Self {
    self.inner.growth_rate_percentage = growth_rate_percentage;
    self
  }

}

impl AsRef<StatisticsValue> for StatisticsValue {
  fn as_ref(&self) -> &StatisticsValue { self }
}

impl AsRef<StatisticsValue> for RTDStatisticsValueBuilder {
  fn as_ref(&self) -> &StatisticsValue { &self.inner }
}



