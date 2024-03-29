
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains basic information about a chat filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFilterInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique chat filter identifier
  id: i64,
  /// The title of the filter; 1-12 characters without line feeds
  title: String,
  /// The chosen or default icon name for short filter representation. One of "All", "Unread", "Unmuted", "Bots", "Channels", "Groups", "Private", "Custom", "Setup", "Cat", "Crown", "Favorite", "Flower", "Game", "Home", "Love", "Mask", "Party", "Sport", "Study", "Trade", "Travel", "Work"
  icon_name: String,
  
}

impl RObject for ChatFilterInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatFilterInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatFilterInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatFilterInfoBuilder {
    let mut inner = ChatFilterInfo::default();
    inner.td_name = "chatFilterInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatFilterInfoBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn title(&self) -> &String { &self.title }

  pub fn icon_name(&self) -> &String { &self.icon_name }

}

#[doc(hidden)]
pub struct RTDChatFilterInfoBuilder {
  inner: ChatFilterInfo
}

impl RTDChatFilterInfoBuilder {
  pub fn build(&self) -> ChatFilterInfo { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn icon_name<T: AsRef<str>>(&mut self, icon_name: T) -> &mut Self {
    self.inner.icon_name = icon_name.as_ref().to_string();
    self
  }

}

impl AsRef<ChatFilterInfo> for ChatFilterInfo {
  fn as_ref(&self) -> &ChatFilterInfo { self }
}

impl AsRef<ChatFilterInfo> for RTDChatFilterInfoBuilder {
  fn as_ref(&self) -> &ChatFilterInfo { &self.inner }
}



