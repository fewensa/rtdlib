use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A detailed statistics about TON Grams earned by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GramRevenueStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A graph containing amount of revenue in a given day
  revenue_by_day_graph: StatisticalGraph,
  /// Amount of earned revenue
  status: GramRevenueStatus,
  /// Current exchange rate of nanogram to USD cents
  usd_rate: f32,
}

impl RObject for GramRevenueStatistics {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "gramRevenueStatistics"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GramRevenueStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGramRevenueStatisticsBuilder {
    let inner = GramRevenueStatistics {
      td_name: "gramRevenueStatistics".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGramRevenueStatisticsBuilder { inner }
  }

  pub fn revenue_by_day_graph(&self) -> &StatisticalGraph {
    &self.revenue_by_day_graph
  }

  pub fn status(&self) -> &GramRevenueStatus {
    &self.status
  }

  pub fn usd_rate(&self) -> f32 {
    self.usd_rate
  }
}

#[doc(hidden)]
pub struct RTDGramRevenueStatisticsBuilder {
  inner: GramRevenueStatistics,
}

impl RTDGramRevenueStatisticsBuilder {
  pub fn build(&self) -> GramRevenueStatistics {
    self.inner.clone()
  }

  pub fn revenue_by_day_graph<T: AsRef<StatisticalGraph>>(
    &mut self,
    revenue_by_day_graph: T,
  ) -> &mut Self {
    self.inner.revenue_by_day_graph = revenue_by_day_graph.as_ref().clone();
    self
  }

  pub fn status<T: AsRef<GramRevenueStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

  pub fn usd_rate(&mut self, usd_rate: f32) -> &mut Self {
    self.inner.usd_rate = usd_rate;
    self
  }
}

impl AsRef<GramRevenueStatistics> for GramRevenueStatistics {
  fn as_ref(&self) -> &GramRevenueStatistics {
    self
  }
}

impl AsRef<GramRevenueStatistics> for RTDGramRevenueStatisticsBuilder {
  fn as_ref(&self) -> &GramRevenueStatistics {
    &self.inner
  }
}
