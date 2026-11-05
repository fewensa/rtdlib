use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes background of a gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Center color in RGB format
  center_color: i64,
  /// Edge color in RGB format
  edge_color: i64,
  /// Text color in RGB format
  text_color: i64,
}

impl RObject for GiftBackground {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftBackground"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftBackgroundBuilder {
    let inner = GiftBackground {
      td_name: "giftBackground".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftBackgroundBuilder { inner }
  }

  pub fn center_color(&self) -> i64 {
    self.center_color
  }

  pub fn edge_color(&self) -> i64 {
    self.edge_color
  }

  pub fn text_color(&self) -> i64 {
    self.text_color
  }
}

#[doc(hidden)]
pub struct RTDGiftBackgroundBuilder {
  inner: GiftBackground,
}

impl RTDGiftBackgroundBuilder {
  pub fn build(&self) -> GiftBackground {
    self.inner.clone()
  }

  pub fn center_color(&mut self, center_color: i64) -> &mut Self {
    self.inner.center_color = center_color;
    self
  }

  pub fn edge_color(&mut self, edge_color: i64) -> &mut Self {
    self.inner.edge_color = edge_color;
    self
  }

  pub fn text_color(&mut self, text_color: i64) -> &mut Self {
    self.inner.text_color = text_color;
    self
  }
}

impl AsRef<GiftBackground> for GiftBackground {
  fn as_ref(&self) -> &GiftBackground {
    self
  }
}

impl AsRef<GiftBackground> for RTDGiftBackgroundBuilder {
  fn as_ref(&self) -> &GiftBackground {
    &self.inner
  }
}
