use crate::types::{Function, Object, RObject, TDType};

impl <'a, Fnc: Function> Function for &'a Fnc {}

impl <'a, Fnc: Function> Function for &'a mut Fnc {}

impl <'a, Obj: Object> Object for &'a Obj {}

impl <'a, Obj: Object> Object for &'a mut Obj {}

impl <'a, RObj: RObject> RObject for &'a RObj {
  fn td_name(&self) -> &'static str {
    (*self).td_name()
  }

  fn td_type(&self) -> TDType {
    (*self).td_type()
  }

  fn to_json(&self) -> String {
    (*self).to_json()
  }
}

impl <'a, RObj: RObject> RObject for &'a mut RObj {
  fn td_name(&self) -> &'static str {
    (**self).td_name()
  }

  fn td_type(&self) -> TDType {
    (**self).td_type()
  }

  fn to_json(&self) -> String {
    (**self).to_json()
  }
}



