use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of story areas to be added
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreas {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of input story areas. Currently, a story can have up to 10 inputStoryAreaTypeLocation, inputStoryAreaTypeFoundVenue, and inputStoryAreaTypePreviousVenue areas, up to getOption("story_suggested_reaction_area_count_max") inputStoryAreaTypeSuggestedReaction areas, up to 1 inputStoryAreaTypeMessage area, up to getOption("story_link_area_count_max") inputStoryAreaTypeLink areas if the current user is a Telegram Premium user, up to 3 inputStoryAreaTypeWeather areas, and up to 1 inputStoryAreaTypeUpgradedGift area
  areas: Vec<InputStoryArea>,
}

impl RObject for InputStoryAreas {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputStoryAreas"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputStoryAreas {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStoryAreasBuilder {
    let inner = InputStoryAreas {
      td_name: "inputStoryAreas".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStoryAreasBuilder { inner }
  }

  pub fn areas(&self) -> &Vec<InputStoryArea> {
    &self.areas
  }
}

#[doc(hidden)]
pub struct RTDInputStoryAreasBuilder {
  inner: InputStoryAreas,
}

impl RTDInputStoryAreasBuilder {
  pub fn build(&self) -> InputStoryAreas {
    self.inner.clone()
  }

  pub fn areas(&mut self, areas: Vec<InputStoryArea>) -> &mut Self {
    self.inner.areas = areas;
    self
  }
}

impl AsRef<InputStoryAreas> for InputStoryAreas {
  fn as_ref(&self) -> &InputStoryAreas {
    self
  }
}

impl AsRef<InputStoryAreas> for RTDInputStoryAreasBuilder {
  fn as_ref(&self) -> &InputStoryAreas {
    &self.inner
  }
}
