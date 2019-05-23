use crate::tdjson;

pub struct RTDLib {
  tdlib: tdjson::Tdlib
}

impl RTDLib {
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
  /// use rtdlib::RTDLib;
  /// RTdlib::set_log_verbosity_level(3);
  /// ```
  pub fn set_log_verbosity_level<'a>(level: i32) -> Result<(), &'a str> {
    tdjson::Tdlib::set_log_verbosity_level(level)
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
  /// use rtdlib::RTDLib;
  /// RTDLib::set_log_max_file_size(1024 * 1024);
  /// ```
  pub fn set_log_max_file_size(size: i64) {
    tdjson::Tdlib::set_log_max_file_size(size)
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
  /// use rtdlib::RTDLib;
  /// RTDLib::set_log_file_path(Some("/var/log/tdlib/tdlib.log"));
  /// ```
  pub fn set_log_file_path(path: Option<&str>) -> bool {
    tdjson::Tdlib::set_log_file_path(path)
  }

  /// Creates a new instance of TDLib.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::RTDLib;
  /// let tdlib = RTDLib::new();
  /// ```
  pub fn new() -> Self {
    Self {
      tdlib: tdjson::Tdlib::new()
    }
  }


  /// Sends request to the TDLib client.
  ///
  /// May be called from any thread.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::RTDLib;
  /// let tdlib = RTDLib::new();
  /// let request = r#"{"@type": "getMe"}"#;
  /// tdlib.send(request);
  /// ```
  pub fn send(&self, request: &str) {
    self.tdlib.send(request)
  }

  /// Synchronously executes TDLib request.
  ///
  /// May be called from any thread. Only a few requests can be executed synchronously.
  ///
  /// # Examples
  ///
  /// ```
  /// use rtdlib::RTDLib;
  /// let tdlib = RTDLib::new();
  /// let request = r#"{"@type": "getTextEntities", "text": "@telegram /test_command https://telegram.org telegram.me"}"#;
  /// tdlib.execute(request);
  /// ```
  pub fn execute(&self, request: &str) -> Option<String> {
    self.tdlib.execute(request)
  }
}

impl Drop for RTDLib {
  fn drop(&mut self) {
    self.tdlib.drop()
  }
}


