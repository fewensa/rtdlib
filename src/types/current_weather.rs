use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes the current weather
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CurrentWeather {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Temperature, in degree Celsius
  temperature: f32,
  /// Emoji representing the weather
  emoji: String,
}

impl RObject for CurrentWeather {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "currentWeather"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CurrentWeather {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCurrentWeatherBuilder {
    let inner = CurrentWeather {
      td_name: "currentWeather".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCurrentWeatherBuilder { inner }
  }

  pub fn temperature(&self) -> f32 {
    self.temperature
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }
}

#[doc(hidden)]
pub struct RTDCurrentWeatherBuilder {
  inner: CurrentWeather,
}

impl RTDCurrentWeatherBuilder {
  pub fn build(&self) -> CurrentWeather {
    self.inner.clone()
  }

  pub fn temperature(&mut self, temperature: f32) -> &mut Self {
    self.inner.temperature = temperature;
    self
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }
}

impl AsRef<CurrentWeather> for CurrentWeather {
  fn as_ref(&self) -> &CurrentWeather {
    self
  }
}

impl AsRef<CurrentWeather> for RTDCurrentWeatherBuilder {
  fn as_ref(&self) -> &CurrentWeather {
    &self.inner
  }
}
