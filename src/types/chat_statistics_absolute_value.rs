
use crate::types::*;
use crate::errors::*;




/// chatStatisticsAbsoluteValue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsAbsoluteValue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// current
  current: f32,
  /// previous
  previous: f32,
  
}

impl RObject for ChatStatisticsAbsoluteValue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsAbsoluteValue" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatStatisticsAbsoluteValue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsAbsoluteValueBuilder {
    let mut inner = ChatStatisticsAbsoluteValue::default();
    inner.td_name = "chatStatisticsAbsoluteValue".to_string();
    RTDChatStatisticsAbsoluteValueBuilder { inner }
  }

  pub fn current(&self) -> f32 { self.current }

  pub fn previous(&self) -> f32 { self.previous }

}

#[doc(hidden)]
pub struct RTDChatStatisticsAbsoluteValueBuilder {
  inner: ChatStatisticsAbsoluteValue
}

impl RTDChatStatisticsAbsoluteValueBuilder {
  pub fn build(&self) -> ChatStatisticsAbsoluteValue { self.inner.clone() }

   
  pub fn current(&mut self, current: f32) -> &mut Self {
    self.inner.current = current;
    self
  }

   
  pub fn previous(&mut self, previous: f32) -> &mut Self {
    self.inner.previous = previous;
    self
  }

}

impl AsRef<ChatStatisticsAbsoluteValue> for ChatStatisticsAbsoluteValue {
  fn as_ref(&self) -> &ChatStatisticsAbsoluteValue { self }
}

impl AsRef<ChatStatisticsAbsoluteValue> for RTDChatStatisticsAbsoluteValueBuilder {
  fn as_ref(&self) -> &ChatStatisticsAbsoluteValue { &self.inner }
}



