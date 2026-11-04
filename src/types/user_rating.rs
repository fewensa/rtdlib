use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains description of user rating
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserRating {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The level of the user; may be negative
  level: i64,
  /// True, if the maximum level is reached
  is_maximum_level_reached: bool,
  /// Numerical value of the rating
  rating: i64,
  /// The rating required for the current level
  current_level_rating: i64,
  /// The rating required for the next level; 0 if the maximum level is reached
  next_level_rating: i64,
}

impl RObject for UserRating {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userRating"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UserRating {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserRatingBuilder {
    let inner = UserRating {
      td_name: "userRating".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserRatingBuilder { inner }
  }

  pub fn level(&self) -> i64 {
    self.level
  }

  pub fn is_maximum_level_reached(&self) -> bool {
    self.is_maximum_level_reached
  }

  pub fn rating(&self) -> i64 {
    self.rating
  }

  pub fn current_level_rating(&self) -> i64 {
    self.current_level_rating
  }

  pub fn next_level_rating(&self) -> i64 {
    self.next_level_rating
  }
}

#[doc(hidden)]
pub struct RTDUserRatingBuilder {
  inner: UserRating,
}

impl RTDUserRatingBuilder {
  pub fn build(&self) -> UserRating {
    self.inner.clone()
  }

  pub fn level(&mut self, level: i64) -> &mut Self {
    self.inner.level = level;
    self
  }

  pub fn is_maximum_level_reached(&mut self, is_maximum_level_reached: bool) -> &mut Self {
    self.inner.is_maximum_level_reached = is_maximum_level_reached;
    self
  }

  pub fn rating(&mut self, rating: i64) -> &mut Self {
    self.inner.rating = rating;
    self
  }

  pub fn current_level_rating(&mut self, current_level_rating: i64) -> &mut Self {
    self.inner.current_level_rating = current_level_rating;
    self
  }

  pub fn next_level_rating(&mut self, next_level_rating: i64) -> &mut Self {
    self.inner.next_level_rating = next_level_rating;
    self
  }
}

impl AsRef<UserRating> for UserRating {
  fn as_ref(&self) -> &UserRating {
    self
  }
}

impl AsRef<UserRating> for RTDUserRatingBuilder {
  fn as_ref(&self) -> &UserRating {
    &self.inner
  }
}
