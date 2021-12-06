
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about found messages, splitted by days according to the option "utc_time_offset"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCalendar {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total number of found messages
  total_count: i64,
  /// Information about messages sent
  days: Vec<MessageCalendarDay>,
  
}

impl RObject for MessageCalendar {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageCalendar" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl MessageCalendar {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageCalendarBuilder {
    let mut inner = MessageCalendar::default();
    inner.td_name = "messageCalendar".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageCalendarBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn days(&self) -> &Vec<MessageCalendarDay> { &self.days }

}

#[doc(hidden)]
pub struct RTDMessageCalendarBuilder {
  inner: MessageCalendar
}

impl RTDMessageCalendarBuilder {
  pub fn build(&self) -> MessageCalendar { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn days(&mut self, days: Vec<MessageCalendarDay>) -> &mut Self {
    self.inner.days = days;
    self
  }

}

impl AsRef<MessageCalendar> for MessageCalendar {
  fn as_ref(&self) -> &MessageCalendar { self }
}

impl AsRef<MessageCalendar> for RTDMessageCalendarBuilder {
  fn as_ref(&self) -> &MessageCalendar { &self.inner }
}



