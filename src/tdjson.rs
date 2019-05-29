extern crate libc;

use libc::{c_double, c_int, c_long, c_void};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

type TdlibClient = *mut c_void;

#[derive(Debug)]
pub struct Tdlib {
  instance: TdlibClient,
}

unsafe impl Send for Tdlib {}
unsafe impl Sync for Tdlib {}

#[link(name = "tdjson")]
extern "C" {
  fn td_json_client_create() -> TdlibClient;
  fn td_json_client_send(client: TdlibClient, request: *const c_char);
  fn td_json_client_receive(client: TdlibClient, timeout: c_double) -> *mut c_char;
  fn td_json_client_execute(client: TdlibClient, request: *const c_char) -> *mut c_char;
  fn td_json_client_destroy(client: TdlibClient);

  fn td_set_log_verbosity_level(level: c_int);
  fn td_set_log_file_path(path: *const c_char) -> c_int;
  fn td_set_log_max_file_size(size: c_long);
}

impl Tdlib {
  /// Sets the verbosity level of the internal logging of TDLib.
  ///
  /// By default the TDLib uses a log verbosity level of 5.
  ///
  /// # Parameters
  ///
  /// `level` New value of logging verbosity level. Value 0 corresponds to fatal errors,
  /// value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings,
  /// value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds
  /// to verbose debug, value greater than 5 and up to 1024 can be used to enable even more logging.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::tdjson::Tdlib;
  /// Tdlib::set_log_verbosity_level(3);
  /// ```
  pub fn set_log_verbosity_level<'a>(level: i32) -> Result<(), &'a str> {
    if level < 0 {
      Err("log verbosity level should be >= 0")
    } else if level > 1024 {
      Err("log verbosity level should be <= 1024")
    } else {
      unsafe { td_set_log_verbosity_level(level) };
      Ok(())
    }
  }

  /// Sets maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated.
  ///
  /// Unused if log is not written to a file. Defaults to 10 MB.
  ///
  /// # Parameters
  ///
  /// `size` Maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated. Should be positive.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::tdjson::Tdlib;
  /// Tdlib::set_log_max_file_size(1024 * 1024);
  /// ```
  pub fn set_log_max_file_size(size: i64) {
    unsafe { td_set_log_max_file_size(size as c_long) };
  }

  /// Sets the path to the file where the internal TDLib log will be written.
  ///
  /// By default TDLib writes logs to stderr or an OS specific log. Use this method to write the log to a file instead.
  ///
  /// # Parameters
  ///
  /// `path` Maybe path to a file where the internal TDLib log will be written. Use `None` to switch back to the default logging behaviour.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::tdjson::Tdlib;
  /// Tdlib::set_log_file_path(Some("/var/log/tdlib/tdlib.log"));
  /// ```
  pub fn set_log_file_path(path: Option<&str>) -> bool {
    let result = match path {
      None => unsafe { td_set_log_file_path(ptr::null()) },
      Some(path_) => {
        let cpath = CString::new(path_).unwrap();
        unsafe { td_set_log_file_path(cpath.as_ptr()) }
      }
    };
    match result {
      1 => true,
      0 => false,
      _ => panic!("unexpected response from libtdjson: {:?}", result)
    }
  }

  /// Creates a new instance of TDLib.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::tdjson::Tdlib;
  /// let tdlib = Tdlib::new();
  /// ```
  pub fn new() -> Self {
    let client = unsafe { td_json_client_create() };
    Tdlib { instance: client }
  }

  /// Sends request to the TDLib client.
  ///
  /// May be called from any thread.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::tdjson::Tdlib;
  /// let tdlib = Tdlib::new();
  /// let request = r#"{"@type": "getMe"}"#;
  /// tdlib.send(request);
  /// ```
  pub fn send(&self, request: &str) {
    let cstring = CString::new(request).unwrap();
    unsafe { td_json_client_send(self.instance, cstring.as_ptr()) }
  }

  /// Synchronously executes TDLib request.
  ///
  /// May be called from any thread. Only a few requests can be executed synchronously.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::tdjson::Tdlib;
  /// let tdlib = Tdlib::new();
  /// let request = r#"{"@type": "getTextEntities", "text": "@telegram /test_command https://telegram.org telegram.me"}"#;
  /// tdlib.execute(request);
  /// ```
  pub fn execute(&self, request: &str) -> Option<String> {
    let cstring = CString::new(request).unwrap();
    let result = unsafe {
      td_json_client_execute(self.instance, cstring.as_ptr())
        .as_ref()
        .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
    };
    result
  }

  /// Receives incoming updates and request responses from the TDLib client.
  ///
  /// May be called from any thread, but shouldn't be called simultaneously
  /// from two different threads.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::tdjson::Tdlib;
  /// let tdlib = Tdlib::new();
  /// tdlib.receive(5.0);
  /// ```
  pub fn receive(&self, timeout: f64) -> Option<String> {
//    debug!("tdlib receive with timeout {}s", timeout);
    unsafe {
      match td_json_client_receive(self.instance, timeout)
        .as_ref()
        .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned()) {
        None => {
          None
        }
        Some(contents) => {
          Some(contents)
        }
      }
    }
  }
}

impl Drop for Tdlib {
  fn drop(&mut self) {
    unsafe {
      td_json_client_destroy(self.instance);
    }
  }
}
