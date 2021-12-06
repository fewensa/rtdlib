
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of message positions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePositions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total count of messages found
  total_count: i64,
  /// List of message positions
  positions: Vec<MessagePosition>,
  
}

impl RObject for MessagePositions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePositions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl MessagePositions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagePositionsBuilder {
    let mut inner = MessagePositions::default();
    inner.td_name = "messagePositions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessagePositionsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn positions(&self) -> &Vec<MessagePosition> { &self.positions }

}

#[doc(hidden)]
pub struct RTDMessagePositionsBuilder {
  inner: MessagePositions
}

impl RTDMessagePositionsBuilder {
  pub fn build(&self) -> MessagePositions { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn positions(&mut self, positions: Vec<MessagePosition>) -> &mut Self {
    self.inner.positions = positions;
    self
  }

}

impl AsRef<MessagePositions> for MessagePositions {
  fn as_ref(&self) -> &MessagePositions { self }
}

impl AsRef<MessagePositions> for RTDMessagePositionsBuilder {
  fn as_ref(&self) -> &MessagePositions { &self.inner }
}



