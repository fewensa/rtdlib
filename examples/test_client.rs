use rtdlib::Client;

fn main() {
  Client::set_log_verbosity_level(0).unwrap();
  let client = Client::new();

}

