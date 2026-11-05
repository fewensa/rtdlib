use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of business chat links created by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessChatLinks {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of links
  links: Vec<BusinessChatLink>,
}

impl RObject for BusinessChatLinks {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "businessChatLinks"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BusinessChatLinks {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBusinessChatLinksBuilder {
    let inner = BusinessChatLinks {
      td_name: "businessChatLinks".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBusinessChatLinksBuilder { inner }
  }

  pub fn links(&self) -> &Vec<BusinessChatLink> {
    &self.links
  }
}

#[doc(hidden)]
pub struct RTDBusinessChatLinksBuilder {
  inner: BusinessChatLinks,
}

impl RTDBusinessChatLinksBuilder {
  pub fn build(&self) -> BusinessChatLinks {
    self.inner.clone()
  }

  pub fn links(&mut self, links: Vec<BusinessChatLink>) -> &mut Self {
    self.inner.links = links;
    self
  }
}

impl AsRef<BusinessChatLinks> for BusinessChatLinks {
  fn as_ref(&self) -> &BusinessChatLinks {
    self
  }
}

impl AsRef<BusinessChatLinks> for RTDBusinessChatLinksBuilder {
  fn as_ref(&self) -> &BusinessChatLinks {
    &self.inner
  }
}
