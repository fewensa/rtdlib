
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about found messages sent in a specific day
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCalendarDay {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total number of found messages sent in the day
  total_count: i64,
  /// First message sent in the day
  message: Message,
  
}

impl RObject for MessageCalendarDay {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageCalendarDay" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl MessageCalendarDay {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageCalendarDayBuilder {
    let mut inner = MessageCalendarDay::default();
    inner.td_name = "messageCalendarDay".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageCalendarDayBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn message(&self) -> &Message { &self.message }

}

#[doc(hidden)]
pub struct RTDMessageCalendarDayBuilder {
  inner: MessageCalendarDay
}

impl RTDMessageCalendarDayBuilder {
  pub fn build(&self) -> MessageCalendarDay { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

}

impl AsRef<MessageCalendarDay> for MessageCalendarDay {
  fn as_ref(&self) -> &MessageCalendarDay { self }
}

impl AsRef<MessageCalendarDay> for RTDMessageCalendarDayBuilder {
  fn as_ref(&self) -> &MessageCalendarDay { &self.inner }
}



