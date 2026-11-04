use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A detailed statistics about Telegram Stars earned by a user or a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarRevenueStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A graph containing amount of revenue in a given day
  revenue_by_day_graph: StatisticalGraph,
  /// Telegram Star revenue status
  status: StarRevenueStatus,
  /// Current exchange rate of a Telegram Star to USD
  usd_rate: f32,
}

impl RObject for StarRevenueStatistics {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starRevenueStatistics"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarRevenueStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarRevenueStatisticsBuilder {
    let inner = StarRevenueStatistics {
      td_name: "starRevenueStatistics".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarRevenueStatisticsBuilder { inner }
  }

  pub fn revenue_by_day_graph(&self) -> &StatisticalGraph {
    &self.revenue_by_day_graph
  }

  pub fn status(&self) -> &StarRevenueStatus {
    &self.status
  }

  pub fn usd_rate(&self) -> f32 {
    self.usd_rate
  }
}

#[doc(hidden)]
pub struct RTDStarRevenueStatisticsBuilder {
  inner: StarRevenueStatistics,
}

impl RTDStarRevenueStatisticsBuilder {
  pub fn build(&self) -> StarRevenueStatistics {
    self.inner.clone()
  }

  pub fn revenue_by_day_graph<T: AsRef<StatisticalGraph>>(
    &mut self,
    revenue_by_day_graph: T,
  ) -> &mut Self {
    self.inner.revenue_by_day_graph = revenue_by_day_graph.as_ref().clone();
    self
  }

  pub fn status<T: AsRef<StarRevenueStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

  pub fn usd_rate(&mut self, usd_rate: f32) -> &mut Self {
    self.inner.usd_rate = usd_rate;
    self
  }
}

impl AsRef<StarRevenueStatistics> for StarRevenueStatistics {
  fn as_ref(&self) -> &StarRevenueStatistics {
    self
  }
}

impl AsRef<StarRevenueStatistics> for RTDStarRevenueStatisticsBuilder {
  fn as_ref(&self) -> &StarRevenueStatistics {
    &self.inner
  }
}
