use std::sync::{RwLock};
use std::collections::HashMap;
use futures::channel::mpsc;
use rtdlib::types::{RObject, TdType};

lazy_static! {
  static ref OBSERVER: Observer = {
    Observer::new()
  };
}

struct Observer {
  channels: RwLock<HashMap<String, mpsc::Sender<TdType>>>,
}

impl Observer {
  fn new() -> Self {
    Self {
      channels: RwLock::new(HashMap::new())
    }
  }

  fn notify(&self, payload: TdType) {
    let extra = match &payload {
{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
      TdType::{{token.name | to_camel}}(value) => value.extra(),
{% endfor %}
{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
      TdType::{{token.name | to_camel}}(value) => value.extra(),
{% endif %}{% endfor %}
{% for token in tokens %}{% if token.is_return_type %}
      TdType::{{token.name | to_camel}}(value) => value.extra(),
{% endif %}{% endfor %}

    };
    match extra {
      Some(extra) => {
        let mut map = self.channels.write().unwrap();
        if let Some(sender) = map.get_mut(&extra) {
          sender.try_send(payload).unwrap();
        }
      },
      None => {}
    }
  }

  fn subscribe(&self, extra: String) -> mpsc::Receiver<TdType> {
    let (sender, receiver) = mpsc::channel::<TdType>(1);
    match self.channels.write() {
      Ok(mut map) => {
        map.insert(extra, sender);
      }
      _ => {}
    };
    receiver
  }

  fn unsubscribe(&self, extra: &str) {
    match self.channels.write() {
      Ok(mut map) => {
        map.remove(extra);
      }
      _ => {}
    };
  }
}


pub fn notify(payload: TdType) {
  OBSERVER.notify(payload)
}

pub fn subscribe<T: AsRef<str>>(extra: T) -> mpsc::Receiver<TdType>{
  OBSERVER.subscribe(extra.as_ref().to_string())
}

pub fn unsubscribe<T: AsRef<str>>(extra: T) {
  OBSERVER.unsubscribe(extra.as_ref())
}
