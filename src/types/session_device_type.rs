use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents the type of device from which session was created
pub trait TDSessionDeviceType: Debug + RObject {}

/// Represents the type of device from which session was created
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SessionDeviceType {
  #[doc(hidden)]
  _Default(()),
  /// The session is running on an Android device
  Android(SessionDeviceTypeAndroid),
  /// The session is running on a generic Apple device
  Apple(SessionDeviceTypeApple),
  /// The session is running on the Brave browser
  Brave(SessionDeviceTypeBrave),
  /// The session is running on the Chrome browser
  Chrome(SessionDeviceTypeChrome),
  /// The session is running on the Edge browser
  Edge(SessionDeviceTypeEdge),
  /// The session is running on the Firefox browser
  Firefox(SessionDeviceTypeFirefox),
  /// The session is running on an iPad device
  Ipad(SessionDeviceTypeIpad),
  /// The session is running on an iPhone device
  Iphone(SessionDeviceTypeIphone),
  /// The session is running on a Linux device
  Linux(SessionDeviceTypeLinux),
  /// The session is running on a Mac device
  Mac(SessionDeviceTypeMac),
  /// The session is running on the Opera browser
  Opera(SessionDeviceTypeOpera),
  /// The session is running on the Safari browser
  Safari(SessionDeviceTypeSafari),
  /// The session is running on an Ubuntu device
  Ubuntu(SessionDeviceTypeUbuntu),
  /// The session is running on an unknown type of device
  Unknown(SessionDeviceTypeUnknown),
  /// The session is running on the Vivaldi browser
  Vivaldi(SessionDeviceTypeVivaldi),
  /// The session is running on a Windows device
  Windows(SessionDeviceTypeWindows),
  /// The session is running on an Xbox console
  Xbox(SessionDeviceTypeXbox),
}

impl Default for SessionDeviceType {
  fn default() -> Self {
    SessionDeviceType::_Default(())
  }
}

impl<'de> Deserialize<'de> for SessionDeviceType {
  fn deserialize<D>(deserializer: D) -> Result<SessionDeviceType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SessionDeviceType,
      (sessionDeviceTypeAndroid, Android);
      (sessionDeviceTypeApple, Apple);
      (sessionDeviceTypeBrave, Brave);
      (sessionDeviceTypeChrome, Chrome);
      (sessionDeviceTypeEdge, Edge);
      (sessionDeviceTypeFirefox, Firefox);
      (sessionDeviceTypeIpad, Ipad);
      (sessionDeviceTypeIphone, Iphone);
      (sessionDeviceTypeLinux, Linux);
      (sessionDeviceTypeMac, Mac);
      (sessionDeviceTypeOpera, Opera);
      (sessionDeviceTypeSafari, Safari);
      (sessionDeviceTypeUbuntu, Ubuntu);
      (sessionDeviceTypeUnknown, Unknown);
      (sessionDeviceTypeVivaldi, Vivaldi);
      (sessionDeviceTypeWindows, Windows);
      (sessionDeviceTypeXbox, Xbox);

    )(deserializer)
  }
}

impl RObject for SessionDeviceType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SessionDeviceType::Android(t) => t.td_name(),
      SessionDeviceType::Apple(t) => t.td_name(),
      SessionDeviceType::Brave(t) => t.td_name(),
      SessionDeviceType::Chrome(t) => t.td_name(),
      SessionDeviceType::Edge(t) => t.td_name(),
      SessionDeviceType::Firefox(t) => t.td_name(),
      SessionDeviceType::Ipad(t) => t.td_name(),
      SessionDeviceType::Iphone(t) => t.td_name(),
      SessionDeviceType::Linux(t) => t.td_name(),
      SessionDeviceType::Mac(t) => t.td_name(),
      SessionDeviceType::Opera(t) => t.td_name(),
      SessionDeviceType::Safari(t) => t.td_name(),
      SessionDeviceType::Ubuntu(t) => t.td_name(),
      SessionDeviceType::Unknown(t) => t.td_name(),
      SessionDeviceType::Vivaldi(t) => t.td_name(),
      SessionDeviceType::Windows(t) => t.td_name(),
      SessionDeviceType::Xbox(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SessionDeviceType::Android(t) => t.extra(),
      SessionDeviceType::Apple(t) => t.extra(),
      SessionDeviceType::Brave(t) => t.extra(),
      SessionDeviceType::Chrome(t) => t.extra(),
      SessionDeviceType::Edge(t) => t.extra(),
      SessionDeviceType::Firefox(t) => t.extra(),
      SessionDeviceType::Ipad(t) => t.extra(),
      SessionDeviceType::Iphone(t) => t.extra(),
      SessionDeviceType::Linux(t) => t.extra(),
      SessionDeviceType::Mac(t) => t.extra(),
      SessionDeviceType::Opera(t) => t.extra(),
      SessionDeviceType::Safari(t) => t.extra(),
      SessionDeviceType::Ubuntu(t) => t.extra(),
      SessionDeviceType::Unknown(t) => t.extra(),
      SessionDeviceType::Vivaldi(t) => t.extra(),
      SessionDeviceType::Windows(t) => t.extra(),
      SessionDeviceType::Xbox(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SessionDeviceType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SessionDeviceType::_Default(_))
  }

  pub fn is_android(&self) -> bool {
    matches!(self, SessionDeviceType::Android(_))
  }
  pub fn is_apple(&self) -> bool {
    matches!(self, SessionDeviceType::Apple(_))
  }
  pub fn is_brave(&self) -> bool {
    matches!(self, SessionDeviceType::Brave(_))
  }
  pub fn is_chrome(&self) -> bool {
    matches!(self, SessionDeviceType::Chrome(_))
  }
  pub fn is_edge(&self) -> bool {
    matches!(self, SessionDeviceType::Edge(_))
  }
  pub fn is_firefox(&self) -> bool {
    matches!(self, SessionDeviceType::Firefox(_))
  }
  pub fn is_ipad(&self) -> bool {
    matches!(self, SessionDeviceType::Ipad(_))
  }
  pub fn is_iphone(&self) -> bool {
    matches!(self, SessionDeviceType::Iphone(_))
  }
  pub fn is_linux(&self) -> bool {
    matches!(self, SessionDeviceType::Linux(_))
  }
  pub fn is_mac(&self) -> bool {
    matches!(self, SessionDeviceType::Mac(_))
  }
  pub fn is_opera(&self) -> bool {
    matches!(self, SessionDeviceType::Opera(_))
  }
  pub fn is_safari(&self) -> bool {
    matches!(self, SessionDeviceType::Safari(_))
  }
  pub fn is_ubuntu(&self) -> bool {
    matches!(self, SessionDeviceType::Ubuntu(_))
  }
  pub fn is_unknown(&self) -> bool {
    matches!(self, SessionDeviceType::Unknown(_))
  }
  pub fn is_vivaldi(&self) -> bool {
    matches!(self, SessionDeviceType::Vivaldi(_))
  }
  pub fn is_windows(&self) -> bool {
    matches!(self, SessionDeviceType::Windows(_))
  }
  pub fn is_xbox(&self) -> bool {
    matches!(self, SessionDeviceType::Xbox(_))
  }

  pub fn on_android<F: FnOnce(&SessionDeviceTypeAndroid)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Android(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_apple<F: FnOnce(&SessionDeviceTypeApple)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Apple(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_brave<F: FnOnce(&SessionDeviceTypeBrave)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Brave(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_chrome<F: FnOnce(&SessionDeviceTypeChrome)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Chrome(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_edge<F: FnOnce(&SessionDeviceTypeEdge)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Edge(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_firefox<F: FnOnce(&SessionDeviceTypeFirefox)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Firefox(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ipad<F: FnOnce(&SessionDeviceTypeIpad)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Ipad(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_iphone<F: FnOnce(&SessionDeviceTypeIphone)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Iphone(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_linux<F: FnOnce(&SessionDeviceTypeLinux)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Linux(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_mac<F: FnOnce(&SessionDeviceTypeMac)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Mac(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_opera<F: FnOnce(&SessionDeviceTypeOpera)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Opera(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_safari<F: FnOnce(&SessionDeviceTypeSafari)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Safari(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ubuntu<F: FnOnce(&SessionDeviceTypeUbuntu)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Ubuntu(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unknown<F: FnOnce(&SessionDeviceTypeUnknown)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Unknown(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_vivaldi<F: FnOnce(&SessionDeviceTypeVivaldi)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Vivaldi(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_windows<F: FnOnce(&SessionDeviceTypeWindows)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Windows(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_xbox<F: FnOnce(&SessionDeviceTypeXbox)>(&self, fnc: F) -> &Self {
    if let SessionDeviceType::Xbox(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_android(&self) -> Option<&SessionDeviceTypeAndroid> {
    if let SessionDeviceType::Android(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_apple(&self) -> Option<&SessionDeviceTypeApple> {
    if let SessionDeviceType::Apple(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_brave(&self) -> Option<&SessionDeviceTypeBrave> {
    if let SessionDeviceType::Brave(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_chrome(&self) -> Option<&SessionDeviceTypeChrome> {
    if let SessionDeviceType::Chrome(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_edge(&self) -> Option<&SessionDeviceTypeEdge> {
    if let SessionDeviceType::Edge(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_firefox(&self) -> Option<&SessionDeviceTypeFirefox> {
    if let SessionDeviceType::Firefox(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ipad(&self) -> Option<&SessionDeviceTypeIpad> {
    if let SessionDeviceType::Ipad(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_iphone(&self) -> Option<&SessionDeviceTypeIphone> {
    if let SessionDeviceType::Iphone(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_linux(&self) -> Option<&SessionDeviceTypeLinux> {
    if let SessionDeviceType::Linux(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_mac(&self) -> Option<&SessionDeviceTypeMac> {
    if let SessionDeviceType::Mac(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_opera(&self) -> Option<&SessionDeviceTypeOpera> {
    if let SessionDeviceType::Opera(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_safari(&self) -> Option<&SessionDeviceTypeSafari> {
    if let SessionDeviceType::Safari(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ubuntu(&self) -> Option<&SessionDeviceTypeUbuntu> {
    if let SessionDeviceType::Ubuntu(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unknown(&self) -> Option<&SessionDeviceTypeUnknown> {
    if let SessionDeviceType::Unknown(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_vivaldi(&self) -> Option<&SessionDeviceTypeVivaldi> {
    if let SessionDeviceType::Vivaldi(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_windows(&self) -> Option<&SessionDeviceTypeWindows> {
    if let SessionDeviceType::Windows(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_xbox(&self) -> Option<&SessionDeviceTypeXbox> {
    if let SessionDeviceType::Xbox(t) = self {
      return Some(t);
    }
    None
  }

  pub fn android<T: AsRef<SessionDeviceTypeAndroid>>(t: T) -> Self {
    SessionDeviceType::Android(t.as_ref().clone())
  }

  pub fn apple<T: AsRef<SessionDeviceTypeApple>>(t: T) -> Self {
    SessionDeviceType::Apple(t.as_ref().clone())
  }

  pub fn brave<T: AsRef<SessionDeviceTypeBrave>>(t: T) -> Self {
    SessionDeviceType::Brave(t.as_ref().clone())
  }

  pub fn chrome<T: AsRef<SessionDeviceTypeChrome>>(t: T) -> Self {
    SessionDeviceType::Chrome(t.as_ref().clone())
  }

  pub fn edge<T: AsRef<SessionDeviceTypeEdge>>(t: T) -> Self {
    SessionDeviceType::Edge(t.as_ref().clone())
  }

  pub fn firefox<T: AsRef<SessionDeviceTypeFirefox>>(t: T) -> Self {
    SessionDeviceType::Firefox(t.as_ref().clone())
  }

  pub fn ipad<T: AsRef<SessionDeviceTypeIpad>>(t: T) -> Self {
    SessionDeviceType::Ipad(t.as_ref().clone())
  }

  pub fn iphone<T: AsRef<SessionDeviceTypeIphone>>(t: T) -> Self {
    SessionDeviceType::Iphone(t.as_ref().clone())
  }

  pub fn linux<T: AsRef<SessionDeviceTypeLinux>>(t: T) -> Self {
    SessionDeviceType::Linux(t.as_ref().clone())
  }

  pub fn mac<T: AsRef<SessionDeviceTypeMac>>(t: T) -> Self {
    SessionDeviceType::Mac(t.as_ref().clone())
  }

  pub fn opera<T: AsRef<SessionDeviceTypeOpera>>(t: T) -> Self {
    SessionDeviceType::Opera(t.as_ref().clone())
  }

  pub fn safari<T: AsRef<SessionDeviceTypeSafari>>(t: T) -> Self {
    SessionDeviceType::Safari(t.as_ref().clone())
  }

  pub fn ubuntu<T: AsRef<SessionDeviceTypeUbuntu>>(t: T) -> Self {
    SessionDeviceType::Ubuntu(t.as_ref().clone())
  }

  pub fn unknown<T: AsRef<SessionDeviceTypeUnknown>>(t: T) -> Self {
    SessionDeviceType::Unknown(t.as_ref().clone())
  }

  pub fn vivaldi<T: AsRef<SessionDeviceTypeVivaldi>>(t: T) -> Self {
    SessionDeviceType::Vivaldi(t.as_ref().clone())
  }

  pub fn windows<T: AsRef<SessionDeviceTypeWindows>>(t: T) -> Self {
    SessionDeviceType::Windows(t.as_ref().clone())
  }

  pub fn xbox<T: AsRef<SessionDeviceTypeXbox>>(t: T) -> Self {
    SessionDeviceType::Xbox(t.as_ref().clone())
  }
}

impl AsRef<SessionDeviceType> for SessionDeviceType {
  fn as_ref(&self) -> &SessionDeviceType {
    self
  }
}

/// The session is running on an Android device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeAndroid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeAndroid {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeAndroid"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeAndroid {}

impl SessionDeviceTypeAndroid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeAndroidBuilder {
    let inner = SessionDeviceTypeAndroid {
      td_name: "sessionDeviceTypeAndroid".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeAndroidBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeAndroidBuilder {
  inner: SessionDeviceTypeAndroid,
}

impl RTDSessionDeviceTypeAndroidBuilder {
  pub fn build(&self) -> SessionDeviceTypeAndroid {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeAndroid> for SessionDeviceTypeAndroid {
  fn as_ref(&self) -> &SessionDeviceTypeAndroid {
    self
  }
}

impl AsRef<SessionDeviceTypeAndroid> for RTDSessionDeviceTypeAndroidBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeAndroid {
    &self.inner
  }
}

/// The session is running on a generic Apple device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeApple {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeApple {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeApple"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeApple {}

impl SessionDeviceTypeApple {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeAppleBuilder {
    let inner = SessionDeviceTypeApple {
      td_name: "sessionDeviceTypeApple".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeAppleBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeAppleBuilder {
  inner: SessionDeviceTypeApple,
}

impl RTDSessionDeviceTypeAppleBuilder {
  pub fn build(&self) -> SessionDeviceTypeApple {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeApple> for SessionDeviceTypeApple {
  fn as_ref(&self) -> &SessionDeviceTypeApple {
    self
  }
}

impl AsRef<SessionDeviceTypeApple> for RTDSessionDeviceTypeAppleBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeApple {
    &self.inner
  }
}

/// The session is running on the Brave browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeBrave {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeBrave {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeBrave"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeBrave {}

impl SessionDeviceTypeBrave {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeBraveBuilder {
    let inner = SessionDeviceTypeBrave {
      td_name: "sessionDeviceTypeBrave".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeBraveBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeBraveBuilder {
  inner: SessionDeviceTypeBrave,
}

impl RTDSessionDeviceTypeBraveBuilder {
  pub fn build(&self) -> SessionDeviceTypeBrave {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeBrave> for SessionDeviceTypeBrave {
  fn as_ref(&self) -> &SessionDeviceTypeBrave {
    self
  }
}

impl AsRef<SessionDeviceTypeBrave> for RTDSessionDeviceTypeBraveBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeBrave {
    &self.inner
  }
}

/// The session is running on the Chrome browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeChrome {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeChrome {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeChrome"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeChrome {}

impl SessionDeviceTypeChrome {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeChromeBuilder {
    let inner = SessionDeviceTypeChrome {
      td_name: "sessionDeviceTypeChrome".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeChromeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeChromeBuilder {
  inner: SessionDeviceTypeChrome,
}

impl RTDSessionDeviceTypeChromeBuilder {
  pub fn build(&self) -> SessionDeviceTypeChrome {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeChrome> for SessionDeviceTypeChrome {
  fn as_ref(&self) -> &SessionDeviceTypeChrome {
    self
  }
}

impl AsRef<SessionDeviceTypeChrome> for RTDSessionDeviceTypeChromeBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeChrome {
    &self.inner
  }
}

/// The session is running on the Edge browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeEdge {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeEdge {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeEdge"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeEdge {}

impl SessionDeviceTypeEdge {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeEdgeBuilder {
    let inner = SessionDeviceTypeEdge {
      td_name: "sessionDeviceTypeEdge".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeEdgeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeEdgeBuilder {
  inner: SessionDeviceTypeEdge,
}

impl RTDSessionDeviceTypeEdgeBuilder {
  pub fn build(&self) -> SessionDeviceTypeEdge {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeEdge> for SessionDeviceTypeEdge {
  fn as_ref(&self) -> &SessionDeviceTypeEdge {
    self
  }
}

impl AsRef<SessionDeviceTypeEdge> for RTDSessionDeviceTypeEdgeBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeEdge {
    &self.inner
  }
}

/// The session is running on the Firefox browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeFirefox {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeFirefox {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeFirefox"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeFirefox {}

impl SessionDeviceTypeFirefox {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeFirefoxBuilder {
    let inner = SessionDeviceTypeFirefox {
      td_name: "sessionDeviceTypeFirefox".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeFirefoxBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeFirefoxBuilder {
  inner: SessionDeviceTypeFirefox,
}

impl RTDSessionDeviceTypeFirefoxBuilder {
  pub fn build(&self) -> SessionDeviceTypeFirefox {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeFirefox> for SessionDeviceTypeFirefox {
  fn as_ref(&self) -> &SessionDeviceTypeFirefox {
    self
  }
}

impl AsRef<SessionDeviceTypeFirefox> for RTDSessionDeviceTypeFirefoxBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeFirefox {
    &self.inner
  }
}

/// The session is running on an iPad device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeIpad {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeIpad {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeIpad"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeIpad {}

impl SessionDeviceTypeIpad {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeIpadBuilder {
    let inner = SessionDeviceTypeIpad {
      td_name: "sessionDeviceTypeIpad".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeIpadBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeIpadBuilder {
  inner: SessionDeviceTypeIpad,
}

impl RTDSessionDeviceTypeIpadBuilder {
  pub fn build(&self) -> SessionDeviceTypeIpad {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeIpad> for SessionDeviceTypeIpad {
  fn as_ref(&self) -> &SessionDeviceTypeIpad {
    self
  }
}

impl AsRef<SessionDeviceTypeIpad> for RTDSessionDeviceTypeIpadBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeIpad {
    &self.inner
  }
}

/// The session is running on an iPhone device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeIphone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeIphone {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeIphone"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeIphone {}

impl SessionDeviceTypeIphone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeIphoneBuilder {
    let inner = SessionDeviceTypeIphone {
      td_name: "sessionDeviceTypeIphone".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeIphoneBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeIphoneBuilder {
  inner: SessionDeviceTypeIphone,
}

impl RTDSessionDeviceTypeIphoneBuilder {
  pub fn build(&self) -> SessionDeviceTypeIphone {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeIphone> for SessionDeviceTypeIphone {
  fn as_ref(&self) -> &SessionDeviceTypeIphone {
    self
  }
}

impl AsRef<SessionDeviceTypeIphone> for RTDSessionDeviceTypeIphoneBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeIphone {
    &self.inner
  }
}

/// The session is running on a Linux device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeLinux {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeLinux {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeLinux"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeLinux {}

impl SessionDeviceTypeLinux {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeLinuxBuilder {
    let inner = SessionDeviceTypeLinux {
      td_name: "sessionDeviceTypeLinux".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeLinuxBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeLinuxBuilder {
  inner: SessionDeviceTypeLinux,
}

impl RTDSessionDeviceTypeLinuxBuilder {
  pub fn build(&self) -> SessionDeviceTypeLinux {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeLinux> for SessionDeviceTypeLinux {
  fn as_ref(&self) -> &SessionDeviceTypeLinux {
    self
  }
}

impl AsRef<SessionDeviceTypeLinux> for RTDSessionDeviceTypeLinuxBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeLinux {
    &self.inner
  }
}

/// The session is running on a Mac device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeMac {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeMac {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeMac"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeMac {}

impl SessionDeviceTypeMac {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeMacBuilder {
    let inner = SessionDeviceTypeMac {
      td_name: "sessionDeviceTypeMac".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeMacBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeMacBuilder {
  inner: SessionDeviceTypeMac,
}

impl RTDSessionDeviceTypeMacBuilder {
  pub fn build(&self) -> SessionDeviceTypeMac {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeMac> for SessionDeviceTypeMac {
  fn as_ref(&self) -> &SessionDeviceTypeMac {
    self
  }
}

impl AsRef<SessionDeviceTypeMac> for RTDSessionDeviceTypeMacBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeMac {
    &self.inner
  }
}

/// The session is running on the Opera browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeOpera {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeOpera {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeOpera"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeOpera {}

impl SessionDeviceTypeOpera {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeOperaBuilder {
    let inner = SessionDeviceTypeOpera {
      td_name: "sessionDeviceTypeOpera".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeOperaBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeOperaBuilder {
  inner: SessionDeviceTypeOpera,
}

impl RTDSessionDeviceTypeOperaBuilder {
  pub fn build(&self) -> SessionDeviceTypeOpera {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeOpera> for SessionDeviceTypeOpera {
  fn as_ref(&self) -> &SessionDeviceTypeOpera {
    self
  }
}

impl AsRef<SessionDeviceTypeOpera> for RTDSessionDeviceTypeOperaBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeOpera {
    &self.inner
  }
}

/// The session is running on the Safari browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeSafari {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeSafari {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeSafari"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeSafari {}

impl SessionDeviceTypeSafari {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeSafariBuilder {
    let inner = SessionDeviceTypeSafari {
      td_name: "sessionDeviceTypeSafari".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeSafariBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeSafariBuilder {
  inner: SessionDeviceTypeSafari,
}

impl RTDSessionDeviceTypeSafariBuilder {
  pub fn build(&self) -> SessionDeviceTypeSafari {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeSafari> for SessionDeviceTypeSafari {
  fn as_ref(&self) -> &SessionDeviceTypeSafari {
    self
  }
}

impl AsRef<SessionDeviceTypeSafari> for RTDSessionDeviceTypeSafariBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeSafari {
    &self.inner
  }
}

/// The session is running on an Ubuntu device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeUbuntu {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeUbuntu {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeUbuntu"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeUbuntu {}

impl SessionDeviceTypeUbuntu {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeUbuntuBuilder {
    let inner = SessionDeviceTypeUbuntu {
      td_name: "sessionDeviceTypeUbuntu".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeUbuntuBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeUbuntuBuilder {
  inner: SessionDeviceTypeUbuntu,
}

impl RTDSessionDeviceTypeUbuntuBuilder {
  pub fn build(&self) -> SessionDeviceTypeUbuntu {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeUbuntu> for SessionDeviceTypeUbuntu {
  fn as_ref(&self) -> &SessionDeviceTypeUbuntu {
    self
  }
}

impl AsRef<SessionDeviceTypeUbuntu> for RTDSessionDeviceTypeUbuntuBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeUbuntu {
    &self.inner
  }
}

/// The session is running on an unknown type of device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeUnknown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeUnknown {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeUnknown"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeUnknown {}

impl SessionDeviceTypeUnknown {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeUnknownBuilder {
    let inner = SessionDeviceTypeUnknown {
      td_name: "sessionDeviceTypeUnknown".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeUnknownBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeUnknownBuilder {
  inner: SessionDeviceTypeUnknown,
}

impl RTDSessionDeviceTypeUnknownBuilder {
  pub fn build(&self) -> SessionDeviceTypeUnknown {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeUnknown> for SessionDeviceTypeUnknown {
  fn as_ref(&self) -> &SessionDeviceTypeUnknown {
    self
  }
}

impl AsRef<SessionDeviceTypeUnknown> for RTDSessionDeviceTypeUnknownBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeUnknown {
    &self.inner
  }
}

/// The session is running on the Vivaldi browser
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeVivaldi {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeVivaldi {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeVivaldi"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeVivaldi {}

impl SessionDeviceTypeVivaldi {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeVivaldiBuilder {
    let inner = SessionDeviceTypeVivaldi {
      td_name: "sessionDeviceTypeVivaldi".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeVivaldiBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeVivaldiBuilder {
  inner: SessionDeviceTypeVivaldi,
}

impl RTDSessionDeviceTypeVivaldiBuilder {
  pub fn build(&self) -> SessionDeviceTypeVivaldi {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeVivaldi> for SessionDeviceTypeVivaldi {
  fn as_ref(&self) -> &SessionDeviceTypeVivaldi {
    self
  }
}

impl AsRef<SessionDeviceTypeVivaldi> for RTDSessionDeviceTypeVivaldiBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeVivaldi {
    &self.inner
  }
}

/// The session is running on a Windows device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeWindows {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeWindows {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeWindows"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeWindows {}

impl SessionDeviceTypeWindows {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeWindowsBuilder {
    let inner = SessionDeviceTypeWindows {
      td_name: "sessionDeviceTypeWindows".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeWindowsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeWindowsBuilder {
  inner: SessionDeviceTypeWindows,
}

impl RTDSessionDeviceTypeWindowsBuilder {
  pub fn build(&self) -> SessionDeviceTypeWindows {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeWindows> for SessionDeviceTypeWindows {
  fn as_ref(&self) -> &SessionDeviceTypeWindows {
    self
  }
}

impl AsRef<SessionDeviceTypeWindows> for RTDSessionDeviceTypeWindowsBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeWindows {
    &self.inner
  }
}

/// The session is running on an Xbox console
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionDeviceTypeXbox {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SessionDeviceTypeXbox {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sessionDeviceTypeXbox"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSessionDeviceType for SessionDeviceTypeXbox {}

impl SessionDeviceTypeXbox {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSessionDeviceTypeXboxBuilder {
    let inner = SessionDeviceTypeXbox {
      td_name: "sessionDeviceTypeXbox".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSessionDeviceTypeXboxBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSessionDeviceTypeXboxBuilder {
  inner: SessionDeviceTypeXbox,
}

impl RTDSessionDeviceTypeXboxBuilder {
  pub fn build(&self) -> SessionDeviceTypeXbox {
    self.inner.clone()
  }
}

impl AsRef<SessionDeviceTypeXbox> for SessionDeviceTypeXbox {
  fn as_ref(&self) -> &SessionDeviceTypeXbox {
    self
  }
}

impl AsRef<SessionDeviceTypeXbox> for RTDSessionDeviceTypeXboxBuilder {
  fn as_ref(&self) -> &SessionDeviceTypeXbox {
    &self.inner
  }
}
