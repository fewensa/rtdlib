
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a statistics graph
pub trait TDStatisticsGraph: Debug + RObject {}

/// Describes a statistics graph
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StatisticsGraph {
  #[doc(hidden)] _Default(()),
  /// Loads asynchronous or zoomed in chat statistics graph
  GetChatStatisticsGraph(GetChatStatisticsGraph),
  /// The graph data to be asynchronously loaded through getChatStatisticsGraph
  Async(StatisticsGraphAsync),
  /// A graph data
  Data(StatisticsGraphData),
  /// An error message to be shown to the user instead of the graph
  Error(StatisticsGraphError),

}

impl Default for StatisticsGraph {
  fn default() -> Self { StatisticsGraph::_Default(()) }
}

impl<'de> Deserialize<'de> for StatisticsGraph {
  fn deserialize<D>(deserializer: D) -> Result<StatisticsGraph, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StatisticsGraph,
      (getChatStatisticsGraph, GetChatStatisticsGraph);
      (statisticsGraphAsync, Async);
      (statisticsGraphData, Data);
      (statisticsGraphError, Error);

    )(deserializer)
  }
}

impl RObject for StatisticsGraph {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      StatisticsGraph::GetChatStatisticsGraph(t) => t.td_name(),
      StatisticsGraph::Async(t) => t.td_name(),
      StatisticsGraph::Data(t) => t.td_name(),
      StatisticsGraph::Error(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      StatisticsGraph::GetChatStatisticsGraph(t) => t.extra(),
      StatisticsGraph::Async(t) => t.extra(),
      StatisticsGraph::Data(t) => t.extra(),
      StatisticsGraph::Error(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl StatisticsGraph {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let StatisticsGraph::_Default(_) = self { true } else { false } }

  pub fn is_get_chat_statistics_graph(&self) -> bool { if let StatisticsGraph::GetChatStatisticsGraph(_) = self { true } else { false } }
  pub fn is_async(&self) -> bool { if let StatisticsGraph::Async(_) = self { true } else { false } }
  pub fn is_data(&self) -> bool { if let StatisticsGraph::Data(_) = self { true } else { false } }
  pub fn is_error(&self) -> bool { if let StatisticsGraph::Error(_) = self { true } else { false } }

  pub fn on_get_chat_statistics_graph<F: FnOnce(&GetChatStatisticsGraph)>(&self, fnc: F) -> &Self { if let StatisticsGraph::GetChatStatisticsGraph(t) = self { fnc(t) }; self }
  pub fn on_async<F: FnOnce(&StatisticsGraphAsync)>(&self, fnc: F) -> &Self { if let StatisticsGraph::Async(t) = self { fnc(t) }; self }
  pub fn on_data<F: FnOnce(&StatisticsGraphData)>(&self, fnc: F) -> &Self { if let StatisticsGraph::Data(t) = self { fnc(t) }; self }
  pub fn on_error<F: FnOnce(&StatisticsGraphError)>(&self, fnc: F) -> &Self { if let StatisticsGraph::Error(t) = self { fnc(t) }; self }

  pub fn as_get_chat_statistics_graph(&self) -> Option<&GetChatStatisticsGraph> { if let StatisticsGraph::GetChatStatisticsGraph(t) = self { return Some(t) } None }
  pub fn as_async(&self) -> Option<&StatisticsGraphAsync> { if let StatisticsGraph::Async(t) = self { return Some(t) } None }
  pub fn as_data(&self) -> Option<&StatisticsGraphData> { if let StatisticsGraph::Data(t) = self { return Some(t) } None }
  pub fn as_error(&self) -> Option<&StatisticsGraphError> { if let StatisticsGraph::Error(t) = self { return Some(t) } None }



  pub fn get_chat_statistics_graph<T: AsRef<GetChatStatisticsGraph>>(t: T) -> Self { StatisticsGraph::GetChatStatisticsGraph(t.as_ref().clone()) }

  pub fn async_<T: AsRef<StatisticsGraphAsync>>(t: T) -> Self { StatisticsGraph::Async(t.as_ref().clone()) }

  pub fn data<T: AsRef<StatisticsGraphData>>(t: T) -> Self { StatisticsGraph::Data(t.as_ref().clone()) }

  pub fn error<T: AsRef<StatisticsGraphError>>(t: T) -> Self { StatisticsGraph::Error(t.as_ref().clone()) }

}

impl AsRef<StatisticsGraph> for StatisticsGraph {
  fn as_ref(&self) -> &StatisticsGraph { self }
}







/// The graph data to be asynchronously loaded through getChatStatisticsGraph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticsGraphAsync {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The token to use for data loading
  token: String,
  
}

impl RObject for StatisticsGraphAsync {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "statisticsGraphAsync" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDStatisticsGraph for StatisticsGraphAsync {}



impl StatisticsGraphAsync {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStatisticsGraphAsyncBuilder {
    let mut inner = StatisticsGraphAsync::default();
    inner.td_name = "statisticsGraphAsync".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDStatisticsGraphAsyncBuilder { inner }
  }

  pub fn token(&self) -> &String { &self.token }

}

#[doc(hidden)]
pub struct RTDStatisticsGraphAsyncBuilder {
  inner: StatisticsGraphAsync
}

impl RTDStatisticsGraphAsyncBuilder {
  pub fn build(&self) -> StatisticsGraphAsync { self.inner.clone() }

   
  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }

}

impl AsRef<StatisticsGraphAsync> for StatisticsGraphAsync {
  fn as_ref(&self) -> &StatisticsGraphAsync { self }
}

impl AsRef<StatisticsGraphAsync> for RTDStatisticsGraphAsyncBuilder {
  fn as_ref(&self) -> &StatisticsGraphAsync { &self.inner }
}







/// A graph data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticsGraphData {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Graph data in JSON format
  json_data: String,
  /// If non-empty, a token which can be used to receive a zoomed in graph
  zoom_token: String,
  
}

impl RObject for StatisticsGraphData {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "statisticsGraphData" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDStatisticsGraph for StatisticsGraphData {}



impl StatisticsGraphData {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStatisticsGraphDataBuilder {
    let mut inner = StatisticsGraphData::default();
    inner.td_name = "statisticsGraphData".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDStatisticsGraphDataBuilder { inner }
  }

  pub fn json_data(&self) -> &String { &self.json_data }

  pub fn zoom_token(&self) -> &String { &self.zoom_token }

}

#[doc(hidden)]
pub struct RTDStatisticsGraphDataBuilder {
  inner: StatisticsGraphData
}

impl RTDStatisticsGraphDataBuilder {
  pub fn build(&self) -> StatisticsGraphData { self.inner.clone() }

   
  pub fn json_data<T: AsRef<str>>(&mut self, json_data: T) -> &mut Self {
    self.inner.json_data = json_data.as_ref().to_string();
    self
  }

   
  pub fn zoom_token<T: AsRef<str>>(&mut self, zoom_token: T) -> &mut Self {
    self.inner.zoom_token = zoom_token.as_ref().to_string();
    self
  }

}

impl AsRef<StatisticsGraphData> for StatisticsGraphData {
  fn as_ref(&self) -> &StatisticsGraphData { self }
}

impl AsRef<StatisticsGraphData> for RTDStatisticsGraphDataBuilder {
  fn as_ref(&self) -> &StatisticsGraphData { &self.inner }
}







/// An error message to be shown to the user instead of the graph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticsGraphError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The error message
  error_message: String,
  
}

impl RObject for StatisticsGraphError {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "statisticsGraphError" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDStatisticsGraph for StatisticsGraphError {}



impl StatisticsGraphError {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStatisticsGraphErrorBuilder {
    let mut inner = StatisticsGraphError::default();
    inner.td_name = "statisticsGraphError".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDStatisticsGraphErrorBuilder { inner }
  }

  pub fn error_message(&self) -> &String { &self.error_message }

}

#[doc(hidden)]
pub struct RTDStatisticsGraphErrorBuilder {
  inner: StatisticsGraphError
}

impl RTDStatisticsGraphErrorBuilder {
  pub fn build(&self) -> StatisticsGraphError { self.inner.clone() }

   
  pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
    self.inner.error_message = error_message.as_ref().to_string();
    self
  }

}

impl AsRef<StatisticsGraphError> for StatisticsGraphError {
  fn as_ref(&self) -> &StatisticsGraphError { self }
}

impl AsRef<StatisticsGraphError> for RTDStatisticsGraphErrorBuilder {
  fn as_ref(&self) -> &StatisticsGraphError { &self.inner }
}



