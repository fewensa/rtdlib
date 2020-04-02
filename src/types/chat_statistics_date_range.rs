
use crate::types::*;
use crate::errors::*;




/// chatStatisticsDateRange
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsDateRange {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// min_date
  min_date: i64,
  /// max_date
  max_date: i64,
  
}

impl RObject for ChatStatisticsDateRange {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsDateRange" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatStatisticsDateRange {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsDateRangeBuilder {
    let mut inner = ChatStatisticsDateRange::default();
    inner.td_name = "chatStatisticsDateRange".to_string();
    RTDChatStatisticsDateRangeBuilder { inner }
  }

  pub fn min_date(&self) -> i64 { self.min_date }

  pub fn max_date(&self) -> i64 { self.max_date }

}

#[doc(hidden)]
pub struct RTDChatStatisticsDateRangeBuilder {
  inner: ChatStatisticsDateRange
}

impl RTDChatStatisticsDateRangeBuilder {
  pub fn build(&self) -> ChatStatisticsDateRange { self.inner.clone() }

   
  pub fn min_date(&mut self, min_date: i64) -> &mut Self {
    self.inner.min_date = min_date;
    self
  }

   
  pub fn max_date(&mut self, max_date: i64) -> &mut Self {
    self.inner.max_date = max_date;
    self
  }

}

impl AsRef<ChatStatisticsDateRange> for ChatStatisticsDateRange {
  fn as_ref(&self) -> &ChatStatisticsDateRange { self }
}

impl AsRef<ChatStatisticsDateRange> for RTDChatStatisticsDateRangeBuilder {
  fn as_ref(&self) -> &ChatStatisticsDateRange { &self.inner }
}



