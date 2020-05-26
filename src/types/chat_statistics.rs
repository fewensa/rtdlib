
use crate::types::*;
use crate::errors::*;




/// A detailed statistics about a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// A period to which the statistics applies
  period: DateRange,
  /// Number of members in the chat
  member_count: StatisticsValue,
  /// Mean number of times the recently sent messages was viewed
  mean_view_count: StatisticsValue,
  /// Mean number of times the recently sent messages was shared
  mean_share_count: StatisticsValue,
  /// A percentage of users with enabled notifications for the chat
  enabled_notifications_percentage: f32,
  /// A graph containing number of members in the chat
  member_count_graph: StatisticsGraph,
  /// A graph containing number of members joined and left the chat
  join_graph: StatisticsGraph,
  /// A graph containing number of members muted and unmuted the chat
  mute_graph: StatisticsGraph,
  /// A graph containing number of message views in a given hour in the last two weeks
  view_count_by_hour_graph: StatisticsGraph,
  /// A graph containing number of message views per source
  view_count_by_source_graph: StatisticsGraph,
  /// A graph containing number of new member joins per source
  join_by_source_graph: StatisticsGraph,
  /// A graph containing number of users viewed chat messages per language
  language_graph: StatisticsGraph,
  /// A graph containing number of chat message views and shares
  message_interaction_graph: StatisticsGraph,
  /// A graph containing number of views of associated with the chat instant views
  instant_view_interaction_graph: StatisticsGraph,
  /// Detailed statistics about number of views and shares of recently sent messages
  recent_message_interactions: Vec<ChatStatisticsMessageInteractionCounters>,
  
}

impl RObject for ChatStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatistics" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsBuilder {
    let mut inner = ChatStatistics::default();
    inner.td_name = "chatStatistics".to_string();
    RTDChatStatisticsBuilder { inner }
  }

  pub fn period(&self) -> &DateRange { &self.period }

  pub fn member_count(&self) -> &StatisticsValue { &self.member_count }

  pub fn mean_view_count(&self) -> &StatisticsValue { &self.mean_view_count }

  pub fn mean_share_count(&self) -> &StatisticsValue { &self.mean_share_count }

  pub fn enabled_notifications_percentage(&self) -> f32 { self.enabled_notifications_percentage }

  pub fn member_count_graph(&self) -> &StatisticsGraph { &self.member_count_graph }

  pub fn join_graph(&self) -> &StatisticsGraph { &self.join_graph }

  pub fn mute_graph(&self) -> &StatisticsGraph { &self.mute_graph }

  pub fn view_count_by_hour_graph(&self) -> &StatisticsGraph { &self.view_count_by_hour_graph }

  pub fn view_count_by_source_graph(&self) -> &StatisticsGraph { &self.view_count_by_source_graph }

  pub fn join_by_source_graph(&self) -> &StatisticsGraph { &self.join_by_source_graph }

  pub fn language_graph(&self) -> &StatisticsGraph { &self.language_graph }

  pub fn message_interaction_graph(&self) -> &StatisticsGraph { &self.message_interaction_graph }

  pub fn instant_view_interaction_graph(&self) -> &StatisticsGraph { &self.instant_view_interaction_graph }

  pub fn recent_message_interactions(&self) -> &Vec<ChatStatisticsMessageInteractionCounters> { &self.recent_message_interactions }

}

#[doc(hidden)]
pub struct RTDChatStatisticsBuilder {
  inner: ChatStatistics
}

impl RTDChatStatisticsBuilder {
  pub fn build(&self) -> ChatStatistics { self.inner.clone() }

   
  pub fn period<T: AsRef<DateRange>>(&mut self, period: T) -> &mut Self {
    self.inner.period = period.as_ref().clone();
    self
  }

   
  pub fn member_count<T: AsRef<StatisticsValue>>(&mut self, member_count: T) -> &mut Self {
    self.inner.member_count = member_count.as_ref().clone();
    self
  }

   
  pub fn mean_view_count<T: AsRef<StatisticsValue>>(&mut self, mean_view_count: T) -> &mut Self {
    self.inner.mean_view_count = mean_view_count.as_ref().clone();
    self
  }

   
  pub fn mean_share_count<T: AsRef<StatisticsValue>>(&mut self, mean_share_count: T) -> &mut Self {
    self.inner.mean_share_count = mean_share_count.as_ref().clone();
    self
  }

   
  pub fn enabled_notifications_percentage(&mut self, enabled_notifications_percentage: f32) -> &mut Self {
    self.inner.enabled_notifications_percentage = enabled_notifications_percentage;
    self
  }

   
  pub fn member_count_graph<T: AsRef<StatisticsGraph>>(&mut self, member_count_graph: T) -> &mut Self {
    self.inner.member_count_graph = member_count_graph.as_ref().clone();
    self
  }

   
  pub fn join_graph<T: AsRef<StatisticsGraph>>(&mut self, join_graph: T) -> &mut Self {
    self.inner.join_graph = join_graph.as_ref().clone();
    self
  }

   
  pub fn mute_graph<T: AsRef<StatisticsGraph>>(&mut self, mute_graph: T) -> &mut Self {
    self.inner.mute_graph = mute_graph.as_ref().clone();
    self
  }

   
  pub fn view_count_by_hour_graph<T: AsRef<StatisticsGraph>>(&mut self, view_count_by_hour_graph: T) -> &mut Self {
    self.inner.view_count_by_hour_graph = view_count_by_hour_graph.as_ref().clone();
    self
  }

   
  pub fn view_count_by_source_graph<T: AsRef<StatisticsGraph>>(&mut self, view_count_by_source_graph: T) -> &mut Self {
    self.inner.view_count_by_source_graph = view_count_by_source_graph.as_ref().clone();
    self
  }

   
  pub fn join_by_source_graph<T: AsRef<StatisticsGraph>>(&mut self, join_by_source_graph: T) -> &mut Self {
    self.inner.join_by_source_graph = join_by_source_graph.as_ref().clone();
    self
  }

   
  pub fn language_graph<T: AsRef<StatisticsGraph>>(&mut self, language_graph: T) -> &mut Self {
    self.inner.language_graph = language_graph.as_ref().clone();
    self
  }

   
  pub fn message_interaction_graph<T: AsRef<StatisticsGraph>>(&mut self, message_interaction_graph: T) -> &mut Self {
    self.inner.message_interaction_graph = message_interaction_graph.as_ref().clone();
    self
  }

   
  pub fn instant_view_interaction_graph<T: AsRef<StatisticsGraph>>(&mut self, instant_view_interaction_graph: T) -> &mut Self {
    self.inner.instant_view_interaction_graph = instant_view_interaction_graph.as_ref().clone();
    self
  }

   
  pub fn recent_message_interactions(&mut self, recent_message_interactions: Vec<ChatStatisticsMessageInteractionCounters>) -> &mut Self {
    self.inner.recent_message_interactions = recent_message_interactions;
    self
  }

}

impl AsRef<ChatStatistics> for ChatStatistics {
  fn as_ref(&self) -> &ChatStatistics { self }
}

impl AsRef<ChatStatistics> for RTDChatStatisticsBuilder {
  fn as_ref(&self) -> &ChatStatistics { &self.inner }
}



