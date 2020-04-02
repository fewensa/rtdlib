
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | ChatStatisticsGraph
pub trait TDChatStatisticsGraph: Debug + RObject {}

/// ChatStatisticsGraph
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatStatisticsGraph {
  #[doc(hidden)] _Default(()),
  /// chatStatisticsGraphAsync
  Async(ChatStatisticsGraphAsync),
  /// chatStatisticsGraphError
  Error(ChatStatisticsGraphError),
  /// chatStatisticsGraphData
  Data(ChatStatisticsGraphData),
  /// Load async stats
  GetChatStatisticsGraph(GetChatStatisticsGraph),

}

impl Default for ChatStatisticsGraph {
  fn default() -> Self { ChatStatisticsGraph::_Default(()) }
}

impl<'de> Deserialize<'de> for ChatStatisticsGraph {
  fn deserialize<D>(deserializer: D) -> Result<ChatStatisticsGraph, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatStatisticsGraph,
      (chatStatisticsGraphAsync, Async);
      (chatStatisticsGraphError, Error);
      (chatStatisticsGraphData, Data);
      (getChatStatisticsGraph, GetChatStatisticsGraph);

    )(deserializer)
  }
}

impl RObject for ChatStatisticsGraph {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ChatStatisticsGraph::Async(t) => t.td_name(),
      ChatStatisticsGraph::Error(t) => t.td_name(),
      ChatStatisticsGraph::Data(t) => t.td_name(),
      ChatStatisticsGraph::GetChatStatisticsGraph(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ChatStatisticsGraph {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ChatStatisticsGraph::_Default(_) = self { true } else { false } }

  pub fn is_async(&self) -> bool { if let ChatStatisticsGraph::Async(_) = self { true } else { false } }
  pub fn is_error(&self) -> bool { if let ChatStatisticsGraph::Error(_) = self { true } else { false } }
  pub fn is_data(&self) -> bool { if let ChatStatisticsGraph::Data(_) = self { true } else { false } }
  pub fn is_get_chat_statistics_graph(&self) -> bool { if let ChatStatisticsGraph::GetChatStatisticsGraph(_) = self { true } else { false } }

  pub fn on_async<F: FnOnce(&ChatStatisticsGraphAsync)>(&self, fnc: F) -> &Self { if let ChatStatisticsGraph::Async(t) = self { fnc(t) }; self }
  pub fn on_error<F: FnOnce(&ChatStatisticsGraphError)>(&self, fnc: F) -> &Self { if let ChatStatisticsGraph::Error(t) = self { fnc(t) }; self }
  pub fn on_data<F: FnOnce(&ChatStatisticsGraphData)>(&self, fnc: F) -> &Self { if let ChatStatisticsGraph::Data(t) = self { fnc(t) }; self }
  pub fn on_get_chat_statistics_graph<F: FnOnce(&GetChatStatisticsGraph)>(&self, fnc: F) -> &Self { if let ChatStatisticsGraph::GetChatStatisticsGraph(t) = self { fnc(t) }; self }

  pub fn as_async(&self) -> Option<&ChatStatisticsGraphAsync> { if let ChatStatisticsGraph::Async(t) = self { return Some(t) } None }
  pub fn as_error(&self) -> Option<&ChatStatisticsGraphError> { if let ChatStatisticsGraph::Error(t) = self { return Some(t) } None }
  pub fn as_data(&self) -> Option<&ChatStatisticsGraphData> { if let ChatStatisticsGraph::Data(t) = self { return Some(t) } None }
  pub fn as_get_chat_statistics_graph(&self) -> Option<&GetChatStatisticsGraph> { if let ChatStatisticsGraph::GetChatStatisticsGraph(t) = self { return Some(t) } None }



  pub fn async_<T: AsRef<ChatStatisticsGraphAsync>>(t: T) -> Self { ChatStatisticsGraph::Async(t.as_ref().clone()) }

  pub fn error<T: AsRef<ChatStatisticsGraphError>>(t: T) -> Self { ChatStatisticsGraph::Error(t.as_ref().clone()) }

  pub fn data<T: AsRef<ChatStatisticsGraphData>>(t: T) -> Self { ChatStatisticsGraph::Data(t.as_ref().clone()) }

  pub fn get_chat_statistics_graph<T: AsRef<GetChatStatisticsGraph>>(t: T) -> Self { ChatStatisticsGraph::GetChatStatisticsGraph(t.as_ref().clone()) }

}

impl AsRef<ChatStatisticsGraph> for ChatStatisticsGraph {
  fn as_ref(&self) -> &ChatStatisticsGraph { self }
}







/// chatStatisticsGraphAsync
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsGraphAsync {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// token
  token: String,
  
}

impl RObject for ChatStatisticsGraphAsync {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsGraphAsync" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatStatisticsGraph for ChatStatisticsGraphAsync {}



impl ChatStatisticsGraphAsync {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsGraphAsyncBuilder {
    let mut inner = ChatStatisticsGraphAsync::default();
    inner.td_name = "chatStatisticsGraphAsync".to_string();
    RTDChatStatisticsGraphAsyncBuilder { inner }
  }

  pub fn token(&self) -> &String { &self.token }

}

#[doc(hidden)]
pub struct RTDChatStatisticsGraphAsyncBuilder {
  inner: ChatStatisticsGraphAsync
}

impl RTDChatStatisticsGraphAsyncBuilder {
  pub fn build(&self) -> ChatStatisticsGraphAsync { self.inner.clone() }

   
  pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
    self.inner.token = token.as_ref().to_string();
    self
  }

}

impl AsRef<ChatStatisticsGraphAsync> for ChatStatisticsGraphAsync {
  fn as_ref(&self) -> &ChatStatisticsGraphAsync { self }
}

impl AsRef<ChatStatisticsGraphAsync> for RTDChatStatisticsGraphAsyncBuilder {
  fn as_ref(&self) -> &ChatStatisticsGraphAsync { &self.inner }
}







/// chatStatisticsGraphError
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsGraphError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// error_message
  error_message: String,
  
}

impl RObject for ChatStatisticsGraphError {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsGraphError" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatStatisticsGraph for ChatStatisticsGraphError {}



impl ChatStatisticsGraphError {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsGraphErrorBuilder {
    let mut inner = ChatStatisticsGraphError::default();
    inner.td_name = "chatStatisticsGraphError".to_string();
    RTDChatStatisticsGraphErrorBuilder { inner }
  }

  pub fn error_message(&self) -> &String { &self.error_message }

}

#[doc(hidden)]
pub struct RTDChatStatisticsGraphErrorBuilder {
  inner: ChatStatisticsGraphError
}

impl RTDChatStatisticsGraphErrorBuilder {
  pub fn build(&self) -> ChatStatisticsGraphError { self.inner.clone() }

   
  pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
    self.inner.error_message = error_message.as_ref().to_string();
    self
  }

}

impl AsRef<ChatStatisticsGraphError> for ChatStatisticsGraphError {
  fn as_ref(&self) -> &ChatStatisticsGraphError { self }
}

impl AsRef<ChatStatisticsGraphError> for RTDChatStatisticsGraphErrorBuilder {
  fn as_ref(&self) -> &ChatStatisticsGraphError { &self.inner }
}







/// chatStatisticsGraphData
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsGraphData {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// json
  json: String,
  /// zoom_token
  zoom_token: String,
  
}

impl RObject for ChatStatisticsGraphData {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsGraphData" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatStatisticsGraph for ChatStatisticsGraphData {}



impl ChatStatisticsGraphData {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsGraphDataBuilder {
    let mut inner = ChatStatisticsGraphData::default();
    inner.td_name = "chatStatisticsGraphData".to_string();
    RTDChatStatisticsGraphDataBuilder { inner }
  }

  pub fn json(&self) -> &String { &self.json }

  pub fn zoom_token(&self) -> &String { &self.zoom_token }

}

#[doc(hidden)]
pub struct RTDChatStatisticsGraphDataBuilder {
  inner: ChatStatisticsGraphData
}

impl RTDChatStatisticsGraphDataBuilder {
  pub fn build(&self) -> ChatStatisticsGraphData { self.inner.clone() }

   
  pub fn json<T: AsRef<str>>(&mut self, json: T) -> &mut Self {
    self.inner.json = json.as_ref().to_string();
    self
  }

   
  pub fn zoom_token<T: AsRef<str>>(&mut self, zoom_token: T) -> &mut Self {
    self.inner.zoom_token = zoom_token.as_ref().to_string();
    self
  }

}

impl AsRef<ChatStatisticsGraphData> for ChatStatisticsGraphData {
  fn as_ref(&self) -> &ChatStatisticsGraphData { self }
}

impl AsRef<ChatStatisticsGraphData> for RTDChatStatisticsGraphDataBuilder {
  fn as_ref(&self) -> &ChatStatisticsGraphData { &self.inner }
}



