use crate::ctgo::apipe::Apipe;
use crate::ctgo::types::{TdType, TdTypeClassMapper, TdTypeCommon, TdTypeSupplement};

pub fn td_common(apipe: &Apipe) -> TdTypeCommon {
  let names = apipe.names();
  let clznames = names.clone().iter()
    .map(|item| toolkit::text::uppercase_first_char(item))
    .collect::<Vec<String>>();
  let len = names.len();
  TdTypeCommon {
    origin_names: names,
    clz_names: clznames,
    len,
  }
}

pub fn td_all_types(apipe: &Apipe) -> Vec<TdType> {
  apipe.names().iter().map(|name| {
    let mut tdtype = TdType {
      origin_name: name.clone(),
      clz_name: toolkit::text::uppercase_first_char(name),
      is_private: false,
      clz_is_trait: false,
      clz_description: None,
      has_subclasses: false,
      sub_classes: None,
      clz_super: None,
      fields: vec![],
      fields_size: 0,
      has_trait_field: false,
      set_trait_typetag: false,
      impl_trait_typetag: false,
      has_builder_struct: false,
    };
    self::td_type_trait(apipe, &mut tdtype, name);
    self::td_type_father(apipe, &mut tdtype, name);
    self::td_type_fields(apipe, &mut tdtype, name);
    self::td_type_fill(apipe, &mut tdtype);
    tdtype
  })
    .collect::<Vec<TdType>>()
}


pub fn td_supplement(apipe: &Apipe) -> TdTypeSupplement {
  let object_classes = apipe.names().iter().map(|name| {
    let clz_name = toolkit::text::uppercase_first_char(name);
    let is_trait = apipe.is_trait(name);
    TdTypeClassMapper {
      origin_name: name.clone(),
      clz_name,
      is_trait,
    }
  })
    .collect::<Vec<TdTypeClassMapper>>();

  let function_classes = apipe.names().iter()
    .filter(|&name| {
      let fa = apipe.father_class(name);
      fa.is_some() && fa.unwrap() == "Function"
    })
    .map(|name| {
      let clz_name = toolkit::text::uppercase_first_char(name);
      let is_trait = apipe.is_trait(name);
      TdTypeClassMapper {
        origin_name: name.clone(),
        clz_name,
        is_trait,
      }
    })
    .collect::<Vec<TdTypeClassMapper>>();
  TdTypeSupplement { objects: object_classes, functions: function_classes }
}


fn td_type_trait(apipe: &Apipe, tdtype: &mut TdType, name: &String) {
  let is_trait = apipe.is_trait(name);
  let description = apipe.description(name);
  tdtype.clz_is_trait = is_trait;
  tdtype.clz_description = description.map(|v| v.clone());
  if is_trait {
    let mut has_subclasses = false;
    if let Some(sub_classes) = apipe.sub_classes(name) {
      tdtype.sub_classes = Some(sub_classes.clone());
      has_subclasses = sub_classes.len() > 0;
    }
    tdtype.has_subclasses = has_subclasses;
  }
}

fn td_type_father(apipe: &Apipe, tdtype: &mut TdType, name: &String) {
  apipe.father_class(name).map(|father| {
    let text = toolkit::text::uppercase_first_char(father);
    tdtype.clz_super = Some(text);
  });
}


fn td_type_fields(apipe: &Apipe, tdtype: &mut TdType, name: &String) {
  let mut has_trait_field = false;
  let afie = apipe.fields(name);
  if afie.is_none() {
    return;
  }
  let afie = afie.map(|v| v.clone()).unwrap();
  afie.iter().for_each(|field| {
    if !has_trait_field && field.is_trait {
      has_trait_field = true;
    }
  });

  tdtype.fields_size = afie.len();
  tdtype.fields = afie;
  tdtype.has_trait_field = has_trait_field;
}


fn td_type_fill(apipe: &Apipe, tdtype: &mut TdType) {

  // set trait typetag
  let mut set_trait_typetag = true;
  if tdtype.clz_is_trait.clone() {
    let clz_name = tdtype.clz_name.clone().to_lowercase();
    if &clz_name[..] == "object" || &clz_name[..] == "function" {
      set_trait_typetag = false;
    }
  }
  tdtype.set_trait_typetag = set_trait_typetag;

  // impl trait typetag
  let mut impl_trait_typetag = true;
  if let Some(clz_super) = tdtype.clz_super.clone() {
    let clz_super = &clz_super.to_lowercase();
    if &clz_super[..] == "object" ||
      &clz_super[..] == "function" ||
      &clz_super[..] == "tlobject" {
      impl_trait_typetag = false;
    }
  }
  tdtype.impl_trait_typetag = impl_trait_typetag;

  // has builder struct
  let mut has_builder_struct = true;
  if tdtype.is_private.clone() {
    has_builder_struct = false;
  }
  if tdtype.clz_is_trait.clone() {
    has_builder_struct = false;
  }
  if let Some(clz_super) = tdtype.clz_super.clone() {
    let clz_super = clz_super.to_lowercase();
    if &clz_super[..] == "update" ||
      &clz_super[..] == "authorizationstate" {
      has_builder_struct = false;
    }
  }

//  let mut has_builder_struct = false;
//  if let Some(clz_super) = tdtype.clz_super.clone() {
//    let clz_super = clz_super.to_lowercase();
//    if &clz_super[..] == "function" {
//      has_builder_struct = true;
//    }
//  }

  tdtype.has_builder_struct = has_builder_struct;
}




