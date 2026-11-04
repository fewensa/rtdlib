use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A detailed statistics about revenue earned from sponsored messages in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatRevenueStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A graph containing amount of revenue in a given hour
  revenue_by_hour_graph: StatisticalGraph,
  /// A graph containing amount of revenue
  revenue_graph: StatisticalGraph,
  /// Amount of earned revenue
  revenue_amount: ChatRevenueAmount,
  /// Current exchange rate of the cryptocurrency in which revenue is calculated to USD
  usd_rate: f32,
}

impl RObject for ChatRevenueStatistics {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatRevenueStatistics"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatRevenueStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatRevenueStatisticsBuilder {
    let inner = ChatRevenueStatistics {
      td_name: "chatRevenueStatistics".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatRevenueStatisticsBuilder { inner }
  }

  pub fn revenue_by_hour_graph(&self) -> &StatisticalGraph {
    &self.revenue_by_hour_graph
  }

  pub fn revenue_graph(&self) -> &StatisticalGraph {
    &self.revenue_graph
  }

  pub fn revenue_amount(&self) -> &ChatRevenueAmount {
    &self.revenue_amount
  }

  pub fn usd_rate(&self) -> f32 {
    self.usd_rate
  }
}

#[doc(hidden)]
pub struct RTDChatRevenueStatisticsBuilder {
  inner: ChatRevenueStatistics,
}

impl RTDChatRevenueStatisticsBuilder {
  pub fn build(&self) -> ChatRevenueStatistics {
    self.inner.clone()
  }

  pub fn revenue_by_hour_graph<T: AsRef<StatisticalGraph>>(
    &mut self,
    revenue_by_hour_graph: T,
  ) -> &mut Self {
    self.inner.revenue_by_hour_graph = revenue_by_hour_graph.as_ref().clone();
    self
  }

  pub fn revenue_graph<T: AsRef<StatisticalGraph>>(&mut self, revenue_graph: T) -> &mut Self {
    self.inner.revenue_graph = revenue_graph.as_ref().clone();
    self
  }

  pub fn revenue_amount<T: AsRef<ChatRevenueAmount>>(&mut self, revenue_amount: T) -> &mut Self {
    self.inner.revenue_amount = revenue_amount.as_ref().clone();
    self
  }

  pub fn usd_rate(&mut self, usd_rate: f32) -> &mut Self {
    self.inner.usd_rate = usd_rate;
    self
  }
}

impl AsRef<ChatRevenueStatistics> for ChatRevenueStatistics {
  fn as_ref(&self) -> &ChatRevenueStatistics {
    self
  }
}

impl AsRef<ChatRevenueStatistics> for RTDChatRevenueStatisticsBuilder {
  fn as_ref(&self) -> &ChatRevenueStatistics {
    &self.inner
  }
}
