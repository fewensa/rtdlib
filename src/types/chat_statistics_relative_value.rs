
use crate::types::*;
use crate::errors::*;




/// chatStatisticsRelativeValue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsRelativeValue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// part
  part: f32,
  /// total
  total: f32,
  
}

impl RObject for ChatStatisticsRelativeValue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsRelativeValue" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatStatisticsRelativeValue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsRelativeValueBuilder {
    let mut inner = ChatStatisticsRelativeValue::default();
    inner.td_name = "chatStatisticsRelativeValue".to_string();
    RTDChatStatisticsRelativeValueBuilder { inner }
  }

  pub fn part(&self) -> f32 { self.part }

  pub fn total(&self) -> f32 { self.total }

}

#[doc(hidden)]
pub struct RTDChatStatisticsRelativeValueBuilder {
  inner: ChatStatisticsRelativeValue
}

impl RTDChatStatisticsRelativeValueBuilder {
  pub fn build(&self) -> ChatStatisticsRelativeValue { self.inner.clone() }

   
  pub fn part(&mut self, part: f32) -> &mut Self {
    self.inner.part = part;
    self
  }

   
  pub fn total(&mut self, total: f32) -> &mut Self {
    self.inner.total = total;
    self
  }

}

impl AsRef<ChatStatisticsRelativeValue> for ChatStatisticsRelativeValue {
  fn as_ref(&self) -> &ChatStatisticsRelativeValue { self }
}

impl AsRef<ChatStatisticsRelativeValue> for RTDChatStatisticsRelativeValueBuilder {
  fn as_ref(&self) -> &ChatStatisticsRelativeValue { &self.inner }
}



