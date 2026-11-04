use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Information about the OAuth authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OauthLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user for which the link was generated; may be 0 if unknown. The corresponding user may be unknown. If the user is logged in the application, then they must be chosen for authorization by default
  user_id: i64,
  /// An HTTP URL where the user authorizes
  url: String,
  /// A domain of the URL
  domain: String,
  /// True, if the authorization originates from an application
  from_app: bool,
  /// Verified name of the application; if empty, then "Unverified App" must be shown instead
  verified_app_name: String,
  /// User identifier of a bot linked with the website
  bot_user_id: i64,
  /// True, if the user must be asked for the permission to the bot to send them messages
  request_write_access: bool,
  /// True, if the user must be asked for the permission to share their phone number
  request_phone_number_access: bool,
  /// True, if access to the user's TON wallet is requested by the app. If true and the user agrees to give access, then use createTonConnectSession with dapp_client_id and ton_connect_manifest_url to create the TON Connect session, then setTonConnectSessionWalletClientId, then create a tonWalletOwnershipProof for the given domain with the payload ton_proof_payload, and then pass all data to acceptOauthRequest in the parameter ton_connect_connect_data
  request_ton_wallet_access: bool,
  /// Hex-encoded public X25519 key of the dApp; may be empty if TON wallet access wasn't requested
  dapp_client_id: String,
  /// URL of the TON Connect manifest of the dApp; may be empty if TON wallet access wasn't requested
  ton_connect_manifest_url: String,
  /// Payload for TON wallet ownership proof; may be empty if TON wallet access wasn't requested
  ton_proof_payload: String,
  /// The version of a browser used for the authorization
  browser: String,
  /// Operating system the browser is running on
  platform: String,
  /// IP address from which the authorization is performed, in human-readable format
  ip_address: String,
  /// Human-readable description of a country and a region from which the authorization is performed, based on the IP address
  location: String,
  /// True, if code matching dialog must be shown first and checkOauthRequestMatchCode must be called before acceptOauthRequest. Otherwise, checkOauthRequestMatchCode must not be called
  match_code_first: bool,
  /// The list of codes to match; may be empty if irrelevant
  match_codes: Vec<String>,
}

impl RObject for OauthLinkInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "oauthLinkInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl OauthLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDOauthLinkInfoBuilder {
    let inner = OauthLinkInfo {
      td_name: "oauthLinkInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDOauthLinkInfoBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn domain(&self) -> &String {
    &self.domain
  }

  pub fn from_app(&self) -> bool {
    self.from_app
  }

  pub fn verified_app_name(&self) -> &String {
    &self.verified_app_name
  }

  pub fn bot_user_id(&self) -> i64 {
    self.bot_user_id
  }

  pub fn request_write_access(&self) -> bool {
    self.request_write_access
  }

  pub fn request_phone_number_access(&self) -> bool {
    self.request_phone_number_access
  }

  pub fn request_ton_wallet_access(&self) -> bool {
    self.request_ton_wallet_access
  }

  pub fn dapp_client_id(&self) -> &String {
    &self.dapp_client_id
  }

  pub fn ton_connect_manifest_url(&self) -> &String {
    &self.ton_connect_manifest_url
  }

  pub fn ton_proof_payload(&self) -> &String {
    &self.ton_proof_payload
  }

  pub fn browser(&self) -> &String {
    &self.browser
  }

  pub fn platform(&self) -> &String {
    &self.platform
  }

  pub fn ip_address(&self) -> &String {
    &self.ip_address
  }

  pub fn location(&self) -> &String {
    &self.location
  }

  pub fn match_code_first(&self) -> bool {
    self.match_code_first
  }

  pub fn match_codes(&self) -> &Vec<String> {
    &self.match_codes
  }
}

#[doc(hidden)]
pub struct RTDOauthLinkInfoBuilder {
  inner: OauthLinkInfo,
}

impl RTDOauthLinkInfoBuilder {
  pub fn build(&self) -> OauthLinkInfo {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn domain<T: AsRef<str>>(&mut self, domain: T) -> &mut Self {
    self.inner.domain = domain.as_ref().to_string();
    self
  }

  pub fn from_app(&mut self, from_app: bool) -> &mut Self {
    self.inner.from_app = from_app;
    self
  }

  pub fn verified_app_name<T: AsRef<str>>(&mut self, verified_app_name: T) -> &mut Self {
    self.inner.verified_app_name = verified_app_name.as_ref().to_string();
    self
  }

  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

  pub fn request_write_access(&mut self, request_write_access: bool) -> &mut Self {
    self.inner.request_write_access = request_write_access;
    self
  }

  pub fn request_phone_number_access(&mut self, request_phone_number_access: bool) -> &mut Self {
    self.inner.request_phone_number_access = request_phone_number_access;
    self
  }

  pub fn request_ton_wallet_access(&mut self, request_ton_wallet_access: bool) -> &mut Self {
    self.inner.request_ton_wallet_access = request_ton_wallet_access;
    self
  }

  pub fn dapp_client_id<T: AsRef<str>>(&mut self, dapp_client_id: T) -> &mut Self {
    self.inner.dapp_client_id = dapp_client_id.as_ref().to_string();
    self
  }

  pub fn ton_connect_manifest_url<T: AsRef<str>>(
    &mut self,
    ton_connect_manifest_url: T,
  ) -> &mut Self {
    self.inner.ton_connect_manifest_url = ton_connect_manifest_url.as_ref().to_string();
    self
  }

  pub fn ton_proof_payload<T: AsRef<str>>(&mut self, ton_proof_payload: T) -> &mut Self {
    self.inner.ton_proof_payload = ton_proof_payload.as_ref().to_string();
    self
  }

  pub fn browser<T: AsRef<str>>(&mut self, browser: T) -> &mut Self {
    self.inner.browser = browser.as_ref().to_string();
    self
  }

  pub fn platform<T: AsRef<str>>(&mut self, platform: T) -> &mut Self {
    self.inner.platform = platform.as_ref().to_string();
    self
  }

  pub fn ip_address<T: AsRef<str>>(&mut self, ip_address: T) -> &mut Self {
    self.inner.ip_address = ip_address.as_ref().to_string();
    self
  }

  pub fn location<T: AsRef<str>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().to_string();
    self
  }

  pub fn match_code_first(&mut self, match_code_first: bool) -> &mut Self {
    self.inner.match_code_first = match_code_first;
    self
  }

  pub fn match_codes(&mut self, match_codes: Vec<String>) -> &mut Self {
    self.inner.match_codes = match_codes;
    self
  }
}

impl AsRef<OauthLinkInfo> for OauthLinkInfo {
  fn as_ref(&self) -> &OauthLinkInfo {
    self
  }
}

impl AsRef<OauthLinkInfo> for RTDOauthLinkInfoBuilder {
  fn as_ref(&self) -> &OauthLinkInfo {
    &self.inner
  }
}
