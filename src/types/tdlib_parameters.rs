
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains parameters for TDLib initialization. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdlibParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // tdlibParameters
  /// If set to true, the Telegram test environment will be used instead of the production environment.
  use_test_dc: Option<bool>,
  /// The path to the directory for the persistent database; if empty, the current working directory will be used.
  database_directory: Option<String>,
  /// The path to the directory for storing files; if empty, database_directory will be used.
  files_directory: Option<String>,
  /// If set to true, information about downloaded and uploaded files will be saved between application restarts.
  use_file_database: Option<bool>,
  /// If set to true, the library will maintain a cache of users, basic groups, supergroups, channels and secret chats. Implies use_file_database.
  use_chat_info_database: Option<bool>,
  /// If set to true, the library will maintain a cache of chats and messages. Implies use_chat_info_database.
  use_message_database: Option<bool>,
  /// If set to true, support for secret chats will be enabled.
  use_secret_chats: Option<bool>,
  /// Application identifier for Telegram API access, which can be obtained at https://my.telegram.org.
  api_id: Option<i32>,
  /// Application identifier hash for Telegram API access, which can be obtained at https://my.telegram.org.
  api_hash: Option<String>,
  /// IETF language tag of the user's operating system language; must be non-empty.
  system_language_code: Option<String>,
  /// Model of the device the application is being run on; must be non-empty.
  device_model: Option<String>,
  /// Version of the operating system the application is being run on; must be non-empty.
  system_version: Option<String>,
  /// Application version; must be non-empty.
  application_version: Option<String>,
  /// If set to true, old files will automatically be deleted.
  enable_storage_optimizer: Option<bool>,
  /// If set to true, original file names will be ignored. Otherwise, downloaded files will be saved under names as close as possible to the original name.
  ignore_file_names: Option<bool>,
  
}



impl Object for TdlibParameters {}
impl RObject for TdlibParameters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tdlibParameters" }
  fn td_type(&self) -> RTDType { RTDType::TdlibParameters }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TdlibParameters {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "tdlibParameters".to_string(),
      use_test_dc: None,
      database_directory: None,
      files_directory: None,
      use_file_database: None,
      use_chat_info_database: None,
      use_message_database: None,
      use_secret_chats: None,
      api_id: None,
      api_hash: None,
      system_language_code: None,
      device_model: None,
      system_version: None,
      application_version: None,
      enable_storage_optimizer: None,
      ignore_file_names: None,
      
    }
  }
  
  pub fn use_test_dc(&self) -> Option<bool> { self.use_test_dc.clone() }
  #[doc(hidden)] pub fn _set_use_test_dc(&mut self, use_test_dc: bool) -> &mut Self { self.use_test_dc = Some(use_test_dc); self }
  
  pub fn database_directory(&self) -> Option<String> { self.database_directory.clone() }
  #[doc(hidden)] pub fn _set_database_directory(&mut self, database_directory: String) -> &mut Self { self.database_directory = Some(database_directory); self }
  
  pub fn files_directory(&self) -> Option<String> { self.files_directory.clone() }
  #[doc(hidden)] pub fn _set_files_directory(&mut self, files_directory: String) -> &mut Self { self.files_directory = Some(files_directory); self }
  
  pub fn use_file_database(&self) -> Option<bool> { self.use_file_database.clone() }
  #[doc(hidden)] pub fn _set_use_file_database(&mut self, use_file_database: bool) -> &mut Self { self.use_file_database = Some(use_file_database); self }
  
  pub fn use_chat_info_database(&self) -> Option<bool> { self.use_chat_info_database.clone() }
  #[doc(hidden)] pub fn _set_use_chat_info_database(&mut self, use_chat_info_database: bool) -> &mut Self { self.use_chat_info_database = Some(use_chat_info_database); self }
  
  pub fn use_message_database(&self) -> Option<bool> { self.use_message_database.clone() }
  #[doc(hidden)] pub fn _set_use_message_database(&mut self, use_message_database: bool) -> &mut Self { self.use_message_database = Some(use_message_database); self }
  
  pub fn use_secret_chats(&self) -> Option<bool> { self.use_secret_chats.clone() }
  #[doc(hidden)] pub fn _set_use_secret_chats(&mut self, use_secret_chats: bool) -> &mut Self { self.use_secret_chats = Some(use_secret_chats); self }
  
  pub fn api_id(&self) -> Option<i32> { self.api_id.clone() }
  #[doc(hidden)] pub fn _set_api_id(&mut self, api_id: i32) -> &mut Self { self.api_id = Some(api_id); self }
  
  pub fn api_hash(&self) -> Option<String> { self.api_hash.clone() }
  #[doc(hidden)] pub fn _set_api_hash(&mut self, api_hash: String) -> &mut Self { self.api_hash = Some(api_hash); self }
  
  pub fn system_language_code(&self) -> Option<String> { self.system_language_code.clone() }
  #[doc(hidden)] pub fn _set_system_language_code(&mut self, system_language_code: String) -> &mut Self { self.system_language_code = Some(system_language_code); self }
  
  pub fn device_model(&self) -> Option<String> { self.device_model.clone() }
  #[doc(hidden)] pub fn _set_device_model(&mut self, device_model: String) -> &mut Self { self.device_model = Some(device_model); self }
  
  pub fn system_version(&self) -> Option<String> { self.system_version.clone() }
  #[doc(hidden)] pub fn _set_system_version(&mut self, system_version: String) -> &mut Self { self.system_version = Some(system_version); self }
  
  pub fn application_version(&self) -> Option<String> { self.application_version.clone() }
  #[doc(hidden)] pub fn _set_application_version(&mut self, application_version: String) -> &mut Self { self.application_version = Some(application_version); self }
  
  pub fn enable_storage_optimizer(&self) -> Option<bool> { self.enable_storage_optimizer.clone() }
  #[doc(hidden)] pub fn _set_enable_storage_optimizer(&mut self, enable_storage_optimizer: bool) -> &mut Self { self.enable_storage_optimizer = Some(enable_storage_optimizer); self }
  
  pub fn ignore_file_names(&self) -> Option<bool> { self.ignore_file_names.clone() }
  #[doc(hidden)] pub fn _set_ignore_file_names(&mut self, ignore_file_names: bool) -> &mut Self { self.ignore_file_names = Some(ignore_file_names); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



