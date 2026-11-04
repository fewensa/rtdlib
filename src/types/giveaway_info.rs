use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about a giveaway
pub trait TDGiveawayInfo: Debug + RObject {}

/// Contains information about a giveaway
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GiveawayInfo {
  #[doc(hidden)]
  _Default(()),
  /// Returns information about a giveaway
  GetGiveawayInfo(GetGiveawayInfo),
  /// Describes a completed giveaway
  Completed(GiveawayInfoCompleted),
  /// Describes an ongoing giveaway
  Ongoing(GiveawayInfoOngoing),
}

impl Default for GiveawayInfo {
  fn default() -> Self {
    GiveawayInfo::_Default(())
  }
}

impl<'de> Deserialize<'de> for GiveawayInfo {
  fn deserialize<D>(deserializer: D) -> Result<GiveawayInfo, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      GiveawayInfo,
      (getGiveawayInfo, GetGiveawayInfo);
      (giveawayInfoCompleted, Completed);
      (giveawayInfoOngoing, Ongoing);

    )(deserializer)
  }
}

impl RObject for GiveawayInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      GiveawayInfo::GetGiveawayInfo(t) => t.td_name(),
      GiveawayInfo::Completed(t) => t.td_name(),
      GiveawayInfo::Ongoing(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      GiveawayInfo::GetGiveawayInfo(t) => t.extra(),
      GiveawayInfo::Completed(t) => t.extra(),
      GiveawayInfo::Ongoing(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiveawayInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, GiveawayInfo::_Default(_))
  }

  pub fn is_get_giveaway_info(&self) -> bool {
    matches!(self, GiveawayInfo::GetGiveawayInfo(_))
  }
  pub fn is_completed(&self) -> bool {
    matches!(self, GiveawayInfo::Completed(_))
  }
  pub fn is_ongoing(&self) -> bool {
    matches!(self, GiveawayInfo::Ongoing(_))
  }

  pub fn on_get_giveaway_info<F: FnOnce(&GetGiveawayInfo)>(&self, fnc: F) -> &Self {
    if let GiveawayInfo::GetGiveawayInfo(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_completed<F: FnOnce(&GiveawayInfoCompleted)>(&self, fnc: F) -> &Self {
    if let GiveawayInfo::Completed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ongoing<F: FnOnce(&GiveawayInfoOngoing)>(&self, fnc: F) -> &Self {
    if let GiveawayInfo::Ongoing(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_get_giveaway_info(&self) -> Option<&GetGiveawayInfo> {
    if let GiveawayInfo::GetGiveawayInfo(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_completed(&self) -> Option<&GiveawayInfoCompleted> {
    if let GiveawayInfo::Completed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ongoing(&self) -> Option<&GiveawayInfoOngoing> {
    if let GiveawayInfo::Ongoing(t) = self {
      return Some(t);
    }
    None
  }

  pub fn get_giveaway_info<T: AsRef<GetGiveawayInfo>>(t: T) -> Self {
    GiveawayInfo::GetGiveawayInfo(t.as_ref().clone())
  }

  pub fn completed<T: AsRef<GiveawayInfoCompleted>>(t: T) -> Self {
    GiveawayInfo::Completed(t.as_ref().clone())
  }

  pub fn ongoing<T: AsRef<GiveawayInfoOngoing>>(t: T) -> Self {
    GiveawayInfo::Ongoing(t.as_ref().clone())
  }
}

impl AsRef<GiveawayInfo> for GiveawayInfo {
  fn as_ref(&self) -> &GiveawayInfo {
    self
  }
}

/// Describes a completed giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayInfoCompleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the giveaway was created
  creation_date: i64,
  /// Point in time (Unix timestamp) when the winners were selected. May be bigger than winners selection date specified in parameters of the giveaway
  actual_winners_selection_date: i64,
  /// True, if the giveaway was canceled and was fully refunded
  was_refunded: bool,
  /// True, if the current user is a winner of the giveaway
  is_winner: bool,
  /// Number of winners in the giveaway
  winner_count: i64,
  /// Number of winners, which activated their gift codes; for Telegram Premium giveaways only
  activation_count: i64,
  /// Telegram Premium gift code that was received by the current user; empty if the user isn't a winner in the giveaway or the giveaway isn't a Telegram Premium giveaway
  gift_code: String,
  /// The Telegram Star amount won by the current user; 0 if the user isn't a winner in the giveaway or the giveaway isn't a Telegram Star giveaway
  won_star_count: i64,
}

impl RObject for GiveawayInfoCompleted {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayInfoCompleted"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiveawayInfo for GiveawayInfoCompleted {}

impl GiveawayInfoCompleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayInfoCompletedBuilder {
    let inner = GiveawayInfoCompleted {
      td_name: "giveawayInfoCompleted".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiveawayInfoCompletedBuilder { inner }
  }

  pub fn creation_date(&self) -> i64 {
    self.creation_date
  }

  pub fn actual_winners_selection_date(&self) -> i64 {
    self.actual_winners_selection_date
  }

  pub fn was_refunded(&self) -> bool {
    self.was_refunded
  }

  pub fn is_winner(&self) -> bool {
    self.is_winner
  }

  pub fn winner_count(&self) -> i64 {
    self.winner_count
  }

  pub fn activation_count(&self) -> i64 {
    self.activation_count
  }

  pub fn gift_code(&self) -> &String {
    &self.gift_code
  }

  pub fn won_star_count(&self) -> i64 {
    self.won_star_count
  }
}

#[doc(hidden)]
pub struct RTDGiveawayInfoCompletedBuilder {
  inner: GiveawayInfoCompleted,
}

impl RTDGiveawayInfoCompletedBuilder {
  pub fn build(&self) -> GiveawayInfoCompleted {
    self.inner.clone()
  }

  pub fn creation_date(&mut self, creation_date: i64) -> &mut Self {
    self.inner.creation_date = creation_date;
    self
  }

  pub fn actual_winners_selection_date(&mut self, actual_winners_selection_date: i64) -> &mut Self {
    self.inner.actual_winners_selection_date = actual_winners_selection_date;
    self
  }

  pub fn was_refunded(&mut self, was_refunded: bool) -> &mut Self {
    self.inner.was_refunded = was_refunded;
    self
  }

  pub fn is_winner(&mut self, is_winner: bool) -> &mut Self {
    self.inner.is_winner = is_winner;
    self
  }

  pub fn winner_count(&mut self, winner_count: i64) -> &mut Self {
    self.inner.winner_count = winner_count;
    self
  }

  pub fn activation_count(&mut self, activation_count: i64) -> &mut Self {
    self.inner.activation_count = activation_count;
    self
  }

  pub fn gift_code<T: AsRef<str>>(&mut self, gift_code: T) -> &mut Self {
    self.inner.gift_code = gift_code.as_ref().to_string();
    self
  }

  pub fn won_star_count(&mut self, won_star_count: i64) -> &mut Self {
    self.inner.won_star_count = won_star_count;
    self
  }
}

impl AsRef<GiveawayInfoCompleted> for GiveawayInfoCompleted {
  fn as_ref(&self) -> &GiveawayInfoCompleted {
    self
  }
}

impl AsRef<GiveawayInfoCompleted> for RTDGiveawayInfoCompletedBuilder {
  fn as_ref(&self) -> &GiveawayInfoCompleted {
    &self.inner
  }
}

/// Describes an ongoing giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiveawayInfoOngoing {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Point in time (Unix timestamp) when the giveaway was created
  creation_date: i64,
  /// Status of the current user in the giveaway
  status: GiveawayParticipantStatus,
  /// True, if the giveaway has ended and results are being prepared
  is_ended: bool,
}

impl RObject for GiveawayInfoOngoing {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giveawayInfoOngoing"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiveawayInfo for GiveawayInfoOngoing {}

impl GiveawayInfoOngoing {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiveawayInfoOngoingBuilder {
    let inner = GiveawayInfoOngoing {
      td_name: "giveawayInfoOngoing".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiveawayInfoOngoingBuilder { inner }
  }

  pub fn creation_date(&self) -> i64 {
    self.creation_date
  }

  pub fn status(&self) -> &GiveawayParticipantStatus {
    &self.status
  }

  pub fn is_ended(&self) -> bool {
    self.is_ended
  }
}

#[doc(hidden)]
pub struct RTDGiveawayInfoOngoingBuilder {
  inner: GiveawayInfoOngoing,
}

impl RTDGiveawayInfoOngoingBuilder {
  pub fn build(&self) -> GiveawayInfoOngoing {
    self.inner.clone()
  }

  pub fn creation_date(&mut self, creation_date: i64) -> &mut Self {
    self.inner.creation_date = creation_date;
    self
  }

  pub fn status<T: AsRef<GiveawayParticipantStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

  pub fn is_ended(&mut self, is_ended: bool) -> &mut Self {
    self.inner.is_ended = is_ended;
    self
  }
}

impl AsRef<GiveawayInfoOngoing> for GiveawayInfoOngoing {
  fn as_ref(&self) -> &GiveawayInfoOngoing {
    self
  }
}

impl AsRef<GiveawayInfoOngoing> for RTDGiveawayInfoOngoingBuilder {
  fn as_ref(&self) -> &GiveawayInfoOngoing {
    &self.inner
  }
}
