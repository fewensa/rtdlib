
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a group call participant's video channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallParticipantVideoInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of synchronization source groups of the video
  source_groups: Vec<GroupCallVideoSourceGroup>,
  /// Video channel endpoint identifier
  endpoint_id: String,
  /// True if the video is paused. This flag needs to be ignored, if new video frames are received
  is_paused: bool,
  
}

impl RObject for GroupCallParticipantVideoInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "groupCallParticipantVideoInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GroupCallParticipantVideoInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGroupCallParticipantVideoInfoBuilder {
    let mut inner = GroupCallParticipantVideoInfo::default();
    inner.td_name = "groupCallParticipantVideoInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGroupCallParticipantVideoInfoBuilder { inner }
  }

  pub fn source_groups(&self) -> &Vec<GroupCallVideoSourceGroup> { &self.source_groups }

  pub fn endpoint_id(&self) -> &String { &self.endpoint_id }

  pub fn is_paused(&self) -> bool { self.is_paused }

}

#[doc(hidden)]
pub struct RTDGroupCallParticipantVideoInfoBuilder {
  inner: GroupCallParticipantVideoInfo
}

impl RTDGroupCallParticipantVideoInfoBuilder {
  pub fn build(&self) -> GroupCallParticipantVideoInfo { self.inner.clone() }

   
  pub fn source_groups(&mut self, source_groups: Vec<GroupCallVideoSourceGroup>) -> &mut Self {
    self.inner.source_groups = source_groups;
    self
  }

   
  pub fn endpoint_id<T: AsRef<str>>(&mut self, endpoint_id: T) -> &mut Self {
    self.inner.endpoint_id = endpoint_id.as_ref().to_string();
    self
  }

   
  pub fn is_paused(&mut self, is_paused: bool) -> &mut Self {
    self.inner.is_paused = is_paused;
    self
  }

}

impl AsRef<GroupCallParticipantVideoInfo> for GroupCallParticipantVideoInfo {
  fn as_ref(&self) -> &GroupCallParticipantVideoInfo { self }
}

impl AsRef<GroupCallParticipantVideoInfo> for RTDGroupCallParticipantVideoInfoBuilder {
  fn as_ref(&self) -> &GroupCallParticipantVideoInfo { &self.inner }
}



