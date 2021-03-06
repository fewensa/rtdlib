
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of messages found by a search
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total count of messages found; 1 if unknown
  total_count: i64,
  /// List of messages
  messages: Vec<Message>,
  /// The offset for the next request. If empty, there are no more results
  next_offset: String,
  
}

impl RObject for FoundMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "foundMessages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl FoundMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFoundMessagesBuilder {
    let mut inner = FoundMessages::default();
    inner.td_name = "foundMessages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDFoundMessagesBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn messages(&self) -> &Vec<Message> { &self.messages }

  pub fn next_offset(&self) -> &String { &self.next_offset }

}

#[doc(hidden)]
pub struct RTDFoundMessagesBuilder {
  inner: FoundMessages
}

impl RTDFoundMessagesBuilder {
  pub fn build(&self) -> FoundMessages { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn messages(&mut self, messages: Vec<Message>) -> &mut Self {
    self.inner.messages = messages;
    self
  }

   
  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }

}

impl AsRef<FoundMessages> for FoundMessages {
  fn as_ref(&self) -> &FoundMessages { self }
}

impl AsRef<FoundMessages> for RTDFoundMessagesBuilder {
  fn as_ref(&self) -> &FoundMessages { &self.inner }
}



