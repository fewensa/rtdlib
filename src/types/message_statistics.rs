
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A detailed statistics about a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A graph containing number of message views and shares
  message_interaction_graph: StatisticalGraph,
  
}

impl RObject for MessageStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageStatistics" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl MessageStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageStatisticsBuilder {
    let mut inner = MessageStatistics::default();
    inner.td_name = "messageStatistics".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageStatisticsBuilder { inner }
  }

  pub fn message_interaction_graph(&self) -> &StatisticalGraph { &self.message_interaction_graph }

}

#[doc(hidden)]
pub struct RTDMessageStatisticsBuilder {
  inner: MessageStatistics
}

impl RTDMessageStatisticsBuilder {
  pub fn build(&self) -> MessageStatistics { self.inner.clone() }

   
  pub fn message_interaction_graph<T: AsRef<StatisticalGraph>>(&mut self, message_interaction_graph: T) -> &mut Self {
    self.inner.message_interaction_graph = message_interaction_graph.as_ref().clone();
    self
  }

}

impl AsRef<MessageStatistics> for MessageStatistics {
  fn as_ref(&self) -> &MessageStatistics { self }
}

impl AsRef<MessageStatistics> for RTDMessageStatisticsBuilder {
  fn as_ref(&self) -> &MessageStatistics { &self.inner }
}



