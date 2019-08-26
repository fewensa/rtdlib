
use crate::types::*;
use crate::errors::*;




/// Represents a list of chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// List of chat identifiers
  chat_ids: Vec<i32>,
  
}

impl RObject for Chats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chats" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Chats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatsBuilder {
    let mut inner = Chats::default();
    inner.td_name = "chats".to_string();
    RTDChatsBuilder { inner }
  }

  pub fn chat_ids(&self) -> &Vec<i32> { &self.chat_ids }

}

#[doc(hidden)]
pub struct RTDChatsBuilder {
  inner: Chats
}

impl RTDChatsBuilder {
  pub fn build(&self) -> Chats { self.inner.clone() }

   
  pub fn chat_ids(&mut self, chat_ids: Vec<i32>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }

}

impl AsRef<Chats> for Chats {
  fn as_ref(&self) -> &Chats { self }
}

impl AsRef<Chats> for RTDChatsBuilder {
  fn as_ref(&self) -> &Chats { &self.inner }
}



