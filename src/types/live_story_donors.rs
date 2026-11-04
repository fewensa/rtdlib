use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of users and chats that spend most money on paid messages and reactions in a live story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LiveStoryDonors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total amount of spent Telegram Stars
  total_star_count: i64,
  /// List of top donors in the live story
  top_donors: Vec<PaidReactor>,
}

impl RObject for LiveStoryDonors {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "liveStoryDonors"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl LiveStoryDonors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDLiveStoryDonorsBuilder {
    let inner = LiveStoryDonors {
      td_name: "liveStoryDonors".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDLiveStoryDonorsBuilder { inner }
  }

  pub fn total_star_count(&self) -> i64 {
    self.total_star_count
  }

  pub fn top_donors(&self) -> &Vec<PaidReactor> {
    &self.top_donors
  }
}

#[doc(hidden)]
pub struct RTDLiveStoryDonorsBuilder {
  inner: LiveStoryDonors,
}

impl RTDLiveStoryDonorsBuilder {
  pub fn build(&self) -> LiveStoryDonors {
    self.inner.clone()
  }

  pub fn total_star_count(&mut self, total_star_count: i64) -> &mut Self {
    self.inner.total_star_count = total_star_count;
    self
  }

  pub fn top_donors(&mut self, top_donors: Vec<PaidReactor>) -> &mut Self {
    self.inner.top_donors = top_donors;
    self
  }
}

impl AsRef<LiveStoryDonors> for LiveStoryDonors {
  fn as_ref(&self) -> &LiveStoryDonors {
    self
  }
}

impl AsRef<LiveStoryDonors> for RTDLiveStoryDonorsBuilder {
  fn as_ref(&self) -> &LiveStoryDonors {
    &self.inner
  }
}
