
use crate::types::*;
use crate::errors::*;




/// chatStatistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// period
  period: ChatStatisticsDateRange,
  /// followers
  followers: ChatStatisticsAbsoluteValue,
  /// views_per_post
  views_per_post: ChatStatisticsAbsoluteValue,
  /// shares_per_post
  shares_per_post: ChatStatisticsAbsoluteValue,
  /// enabled_notifications
  enabled_notifications: ChatStatisticsRelativeValue,
  /// growth_graph
  growth_graph: ChatStatisticsGraph,
  /// followers_graph
  followers_graph: ChatStatisticsGraph,
  /// mute_graph
  mute_graph: ChatStatisticsGraph,
  /// top_hours_graph
  top_hours_graph: ChatStatisticsGraph,
  /// interactions_graph
  interactions_graph: ChatStatisticsGraph,
  /// iv_interactions_graph
  iv_interactions_graph: ChatStatisticsGraph,
  /// views_by_source_graph
  views_by_source_graph: ChatStatisticsGraph,
  /// new_followers_by_source_graph
  new_followers_by_source_graph: ChatStatisticsGraph,
  /// languages_graph
  languages_graph: ChatStatisticsGraph,
  /// recent_message_interactions
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

  pub fn period(&self) -> &ChatStatisticsDateRange { &self.period }

  pub fn followers(&self) -> &ChatStatisticsAbsoluteValue { &self.followers }

  pub fn views_per_post(&self) -> &ChatStatisticsAbsoluteValue { &self.views_per_post }

  pub fn shares_per_post(&self) -> &ChatStatisticsAbsoluteValue { &self.shares_per_post }

  pub fn enabled_notifications(&self) -> &ChatStatisticsRelativeValue { &self.enabled_notifications }

  pub fn growth_graph(&self) -> &ChatStatisticsGraph { &self.growth_graph }

  pub fn followers_graph(&self) -> &ChatStatisticsGraph { &self.followers_graph }

  pub fn mute_graph(&self) -> &ChatStatisticsGraph { &self.mute_graph }

  pub fn top_hours_graph(&self) -> &ChatStatisticsGraph { &self.top_hours_graph }

  pub fn interactions_graph(&self) -> &ChatStatisticsGraph { &self.interactions_graph }

  pub fn iv_interactions_graph(&self) -> &ChatStatisticsGraph { &self.iv_interactions_graph }

  pub fn views_by_source_graph(&self) -> &ChatStatisticsGraph { &self.views_by_source_graph }

  pub fn new_followers_by_source_graph(&self) -> &ChatStatisticsGraph { &self.new_followers_by_source_graph }

  pub fn languages_graph(&self) -> &ChatStatisticsGraph { &self.languages_graph }

  pub fn recent_message_interactions(&self) -> &Vec<ChatStatisticsMessageInteractionCounters> { &self.recent_message_interactions }

}

#[doc(hidden)]
pub struct RTDChatStatisticsBuilder {
  inner: ChatStatistics
}

impl RTDChatStatisticsBuilder {
  pub fn build(&self) -> ChatStatistics { self.inner.clone() }

   
  pub fn period<T: AsRef<ChatStatisticsDateRange>>(&mut self, period: T) -> &mut Self {
    self.inner.period = period.as_ref().clone();
    self
  }

   
  pub fn followers<T: AsRef<ChatStatisticsAbsoluteValue>>(&mut self, followers: T) -> &mut Self {
    self.inner.followers = followers.as_ref().clone();
    self
  }

   
  pub fn views_per_post<T: AsRef<ChatStatisticsAbsoluteValue>>(&mut self, views_per_post: T) -> &mut Self {
    self.inner.views_per_post = views_per_post.as_ref().clone();
    self
  }

   
  pub fn shares_per_post<T: AsRef<ChatStatisticsAbsoluteValue>>(&mut self, shares_per_post: T) -> &mut Self {
    self.inner.shares_per_post = shares_per_post.as_ref().clone();
    self
  }

   
  pub fn enabled_notifications<T: AsRef<ChatStatisticsRelativeValue>>(&mut self, enabled_notifications: T) -> &mut Self {
    self.inner.enabled_notifications = enabled_notifications.as_ref().clone();
    self
  }

   
  pub fn growth_graph<T: AsRef<ChatStatisticsGraph>>(&mut self, growth_graph: T) -> &mut Self {
    self.inner.growth_graph = growth_graph.as_ref().clone();
    self
  }

   
  pub fn followers_graph<T: AsRef<ChatStatisticsGraph>>(&mut self, followers_graph: T) -> &mut Self {
    self.inner.followers_graph = followers_graph.as_ref().clone();
    self
  }

   
  pub fn mute_graph<T: AsRef<ChatStatisticsGraph>>(&mut self, mute_graph: T) -> &mut Self {
    self.inner.mute_graph = mute_graph.as_ref().clone();
    self
  }

   
  pub fn top_hours_graph<T: AsRef<ChatStatisticsGraph>>(&mut self, top_hours_graph: T) -> &mut Self {
    self.inner.top_hours_graph = top_hours_graph.as_ref().clone();
    self
  }

   
  pub fn interactions_graph<T: AsRef<ChatStatisticsGraph>>(&mut self, interactions_graph: T) -> &mut Self {
    self.inner.interactions_graph = interactions_graph.as_ref().clone();
    self
  }

   
  pub fn iv_interactions_graph<T: AsRef<ChatStatisticsGraph>>(&mut self, iv_interactions_graph: T) -> &mut Self {
    self.inner.iv_interactions_graph = iv_interactions_graph.as_ref().clone();
    self
  }

   
  pub fn views_by_source_graph<T: AsRef<ChatStatisticsGraph>>(&mut self, views_by_source_graph: T) -> &mut Self {
    self.inner.views_by_source_graph = views_by_source_graph.as_ref().clone();
    self
  }

   
  pub fn new_followers_by_source_graph<T: AsRef<ChatStatisticsGraph>>(&mut self, new_followers_by_source_graph: T) -> &mut Self {
    self.inner.new_followers_by_source_graph = new_followers_by_source_graph.as_ref().clone();
    self
  }

   
  pub fn languages_graph<T: AsRef<ChatStatisticsGraph>>(&mut self, languages_graph: T) -> &mut Self {
    self.inner.languages_graph = languages_graph.as_ref().clone();
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



