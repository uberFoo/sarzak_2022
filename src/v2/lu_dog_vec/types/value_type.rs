// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::char::CHAR;
use crate::v2::lu_dog_vec::types::empty::EMPTY;
use crate::v2::lu_dog_vec::types::error::Error;
use crate::v2::lu_dog_vec::types::field::Field;
use crate::v2::lu_dog_vec::types::function::Function;
use crate::v2::lu_dog_vec::types::import::Import;
use crate::v2::lu_dog_vec::types::lambda::Lambda;
use crate::v2::lu_dog_vec::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_vec::types::list::List;
use crate::v2::lu_dog_vec::types::range::RANGE;
use crate::v2::lu_dog_vec::types::reference::Reference;
use crate::v2::lu_dog_vec::types::span::Span;
use crate::v2::lu_dog_vec::types::type_cast::TypeCast;
use crate::v2::lu_dog_vec::types::unknown::UNKNOWN;
use crate::v2::lu_dog_vec::types::woog_option::WoogOption;
use crate::v2::lu_dog_vec::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_vec::types::x_value::XValue;
use crate::v2::lu_dog_vec::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-documentation"}}}
/// Value Type
///
/// This is the main type abstraction used in Lu Dog. We mostly rely on what is available in
///  Sarzak, with two additions: ...
///
/// Two? I know that I need an Option<>. I'm not so sure about a & though. Everything from the
///  store is going to be by UUID, so all of my references are really "pointers" underneath.
///  I want them to be typed in the code though.
///
/// So how will the code work? We could store the type next to the pointer: (type, uuid). Huh
/// . This is the eventual output domain. How does that affect my thinking?
///
/// This should end up looking like woog, but simpler. Woog was for generating rust. I want
///  to generate dwarf. Dwarf needs to be typed? If so, when are they resolved to uuid's eventually
/// ?
///
/// Option for now. We'll see later...
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ValueType {
    pub subtype: ValueTypeEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ValueTypeEnum {
    Char(Uuid),
    Empty(Uuid),
    Error(usize),
    Function(usize),
    Import(usize),
    Lambda(usize),
    List(usize),
    ZObjectStore(usize),
    WoogOption(usize),
    Range(Uuid),
    Reference(usize),
    WoogStruct(usize),
    Ty(Uuid),
    Unknown(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl ValueType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_char"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_char(store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Char(CHAR),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_empty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_empty(store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Empty(EMPTY),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_error"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_error(
        subtype: &Rc<RefCell<Error>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Error(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_function"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_function(
        subtype: &Rc<RefCell<Function>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Function(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_import"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_import(
        subtype: &Rc<RefCell<Import>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Import(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_lambda"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_lambda(
        subtype: &Rc<RefCell<Lambda>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Lambda(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_list"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_list(
        subtype: &Rc<RefCell<List>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::List(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_z_object_store"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_z_object_store(
        subtype: &Rc<RefCell<ZObjectStore>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::ZObjectStore(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_option"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_woog_option(
        subtype: &Rc<RefCell<WoogOption>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::WoogOption(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_range"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_range(store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Range(RANGE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_reference"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_reference(
        subtype: &Rc<RefCell<Reference>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Reference(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_struct"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_woog_struct(
        subtype: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::WoogStruct(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_ty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_ty(subtype: &Ty, store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Ty(subtype.id()),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_unknown"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_unknown(store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Unknown(UNKNOWN),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R5(1-M)
    pub fn r5_field<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Field>>> {
        span!("r5_field");
        store
            .iter_field()
            .filter(|field| field.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub fn r10_function<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Function>>> {
        span!("r10_function");
        store
            .iter_function()
            .filter(|function| function.borrow().return_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-lambda"}}}
    /// Navigate to [`Lambda`] across R74(1-M)
    pub fn r74_lambda<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Lambda>>> {
        span!("r74_lambda");
        store
            .iter_lambda()
            .filter(|lambda| lambda.borrow().return_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R77(1-Mc)
    pub fn r77_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<LambdaParameter>>> {
        span!("r77_lambda_parameter");
        store
            .iter_lambda_parameter()
            .filter(|lambda_parameter| lambda_parameter.borrow().ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub fn r36_list<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<List>>> {
        span!("r36_list");
        store
            .iter_list()
            .filter(|list| list.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    /// Navigate to [`WoogOption`] across R2(1-M)
    pub fn r2_woog_option<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<WoogOption>>> {
        span!("r2_woog_option");
        store
            .iter_woog_option()
            .filter(|woog_option| woog_option.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
    /// Navigate to [`Reference`] across R35(1-M)
    pub fn r35_reference<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Reference>>> {
        span!("r35_reference");
        store
            .iter_reference()
            .filter(|reference| reference.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub fn r62_span<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Span>>> {
        span!("r62_span");
        store
            .iter_span()
            .filter(|span| span.borrow().ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub fn r69_type_cast<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<TypeCast>>> {
        span!("r69_type_cast");
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub fn r24_x_value<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r24_x_value");
        store
            .iter_x_value()
            .filter(|x_value| x_value.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}