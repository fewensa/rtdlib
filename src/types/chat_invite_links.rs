
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of chat invite links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinks {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total count of chat invite links found
  total_count: i64,
  /// List of invite links
  invite_links: Vec<ChatInviteLink>,
  
}

impl RObject for ChatInviteLinks {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatInviteLinks" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatInviteLinks {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatInviteLinksBuilder {
    let mut inner = ChatInviteLinks::default();
    inner.td_name = "chatInviteLinks".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatInviteLinksBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn invite_links(&self) -> &Vec<ChatInviteLink> { &self.invite_links }

}

#[doc(hidden)]
pub struct RTDChatInviteLinksBuilder {
  inner: ChatInviteLinks
}

impl RTDChatInviteLinksBuilder {
  pub fn build(&self) -> ChatInviteLinks { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn invite_links(&mut self, invite_links: Vec<ChatInviteLink>) -> &mut Self {
    self.inner.invite_links = invite_links;
    self
  }

}

impl AsRef<ChatInviteLinks> for ChatInviteLinks {
  fn as_ref(&self) -> &ChatInviteLinks { self }
}

impl AsRef<ChatInviteLinks> for RTDChatInviteLinksBuilder {
  fn as_ref(&self) -> &ChatInviteLinks { &self.inner }
}



