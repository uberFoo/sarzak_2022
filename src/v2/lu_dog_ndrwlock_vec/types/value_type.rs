// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::any_list::ANY_LIST;
use crate::v2::lu_dog_ndrwlock_vec::types::char::CHAR;
use crate::v2::lu_dog_ndrwlock_vec::types::empty::EMPTY;
use crate::v2::lu_dog_ndrwlock_vec::types::enum_generic::EnumGeneric;
use crate::v2::lu_dog_ndrwlock_vec::types::enum_generic_type::EnumGenericType;
use crate::v2::lu_dog_ndrwlock_vec::types::enumeration::Enumeration;
use crate::v2::lu_dog_ndrwlock_vec::types::field::Field;
use crate::v2::lu_dog_ndrwlock_vec::types::func_generic::FuncGeneric;
use crate::v2::lu_dog_ndrwlock_vec::types::function::Function;
use crate::v2::lu_dog_ndrwlock_vec::types::import::Import;
use crate::v2::lu_dog_ndrwlock_vec::types::lambda::Lambda;
use crate::v2::lu_dog_ndrwlock_vec::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_ndrwlock_vec::types::list::List;
use crate::v2::lu_dog_ndrwlock_vec::types::list_expression::ListExpression;
use crate::v2::lu_dog_ndrwlock_vec::types::map::Map;
use crate::v2::lu_dog_ndrwlock_vec::types::parameter::Parameter;
use crate::v2::lu_dog_ndrwlock_vec::types::range::RANGE;
use crate::v2::lu_dog_ndrwlock_vec::types::span::Span;
use crate::v2::lu_dog_ndrwlock_vec::types::struct_generic::StructGeneric;
use crate::v2::lu_dog_ndrwlock_vec::types::task::TASK;
use crate::v2::lu_dog_ndrwlock_vec::types::tuple_field::TupleField;
use crate::v2::lu_dog_ndrwlock_vec::types::type_cast::TypeCast;
use crate::v2::lu_dog_ndrwlock_vec::types::unknown::UNKNOWN;
use crate::v2::lu_dog_ndrwlock_vec::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_ndrwlock_vec::types::x_future::XFuture;
use crate::v2::lu_dog_ndrwlock_vec::types::x_plugin::XPlugin;
use crate::v2::lu_dog_ndrwlock_vec::types::x_value::XValue;
use crate::v2::lu_dog_ndrwlock_vec::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValueType {
    pub subtype: ValueTypeEnum,
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ValueTypeEnum {
    AnyList(Uuid),
    Char(Uuid),
    Empty(Uuid),
    EnumGeneric(usize),
    Enumeration(usize),
    FuncGeneric(usize),
    Function(usize),
    XFuture(usize),
    Import(usize),
    Lambda(usize),
    List(usize),
    Map(usize),
    ZObjectStore(usize),
    XPlugin(usize),
    Range(Uuid),
    WoogStruct(usize),
    StructGeneric(usize),
    Task(Uuid),
    Ty(Uuid),
    Unknown(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl ValueType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_any_list"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_any_list(bogus: bool, store: &mut LuDogNdrwlockVecStore) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::AnyList(ANY_LIST),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_char"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_char(bogus: bool, store: &mut LuDogNdrwlockVecStore) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Char(CHAR),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_empty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_empty(bogus: bool, store: &mut LuDogNdrwlockVecStore) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Empty(EMPTY),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enum_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_enum_generic(
        bogus: bool,
        subtype: &Arc<RwLock<EnumGeneric>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::EnumGeneric(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enumeration"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_enumeration(
        bogus: bool,
        subtype: &Arc<RwLock<Enumeration>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Enumeration(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_error"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_error"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_func_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_func_generic(
        bogus: bool,
        subtype: &Arc<RwLock<FuncGeneric>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::FuncGeneric(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_function"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_function(
        bogus: bool,
        subtype: &Arc<RwLock<Function>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Function(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_future"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_x_future(
        bogus: bool,
        subtype: &Arc<RwLock<XFuture>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::XFuture(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_generic"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_import"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_import(
        bogus: bool,
        subtype: &Arc<RwLock<Import>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Import(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_lambda"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_lambda(
        bogus: bool,
        subtype: &Arc<RwLock<Lambda>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Lambda(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_list"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_list(
        bogus: bool,
        subtype: &Arc<RwLock<List>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::List(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_map"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_map(
        bogus: bool,
        subtype: &Arc<RwLock<Map>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Map(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_z_object_store"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_z_object_store(
        bogus: bool,
        subtype: &Arc<RwLock<ZObjectStore>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::ZObjectStore(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_option"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_range"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_plugin"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_plugin"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_x_plugin(
        bogus: bool,
        subtype: &Arc<RwLock<XPlugin>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::XPlugin(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_reference"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_range"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_range(bogus: bool, store: &mut LuDogNdrwlockVecStore) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Range(RANGE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_struct"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_woog_struct(
        bogus: bool,
        subtype: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::WoogStruct(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_struct_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_struct_generic(
        bogus: bool,
        subtype: &Arc<RwLock<StructGeneric>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::StructGeneric(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_task"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_task(bogus: bool, store: &mut LuDogNdrwlockVecStore) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Task(TASK),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_ty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_ty(
        bogus: bool,
        subtype: &std::sync::Arc<std::sync::RwLock<Ty>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Ty(subtype.read().unwrap().id()),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_unknown"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_unknown(bogus: bool, store: &mut LuDogNdrwlockVecStore) -> Arc<RwLock<ValueType>> {
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                bogus: bogus,
                subtype: ValueTypeEnum::Unknown(UNKNOWN),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-enum_generic_type"}}}
    /// Navigate to [`EnumGenericType`] across R119(1-M)
    pub fn r119_enum_generic_type<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<EnumGenericType>>> {
        store
            .iter_enum_generic_type()
            .filter(|enum_generic_type| enum_generic_type.read().unwrap().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R5(1-M)
    pub fn r5_field<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Field>>> {
        store
            .iter_field()
            .filter(|field| field.read().unwrap().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub fn r10_function<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Function>>> {
        store
            .iter_function()
            .filter(|function| function.read().unwrap().return_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_future"}}}
    /// Navigate to [`XFuture`] across R2(1-M)
    pub fn r2_x_future<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<XFuture>>> {
        store
            .iter_x_future()
            .filter(|x_future| x_future.read().unwrap().x_value == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-generic"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-lambda"}}}
    /// Navigate to [`Lambda`] across R74(1-M)
    pub fn r74_lambda<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Lambda>>> {
        store
            .iter_lambda()
            .filter(|lambda| lambda.read().unwrap().return_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R77(1-Mc)
    pub fn r77_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<LambdaParameter>>> {
        store
            .iter_lambda_parameter()
            .filter(|lambda_parameter| lambda_parameter.read().unwrap().ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub fn r36_list<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<List>>> {
        store
            .iter_list()
            .filter(|list| list.read().unwrap().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-one-to-list_expression"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list_expression"}}}
    /// Navigate to [`ListExpression`] across R257(1-M)
    pub fn r257_list_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ListExpression>>> {
        store
            .iter_list_expression()
            .filter(|list_expression| list_expression.read().unwrap().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-map"}}}
    /// Navigate to [`Map`] across R115(1-M)
    pub fn r115_map<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Map>>> {
        store
            .iter_map()
            .filter(|map| map.read().unwrap().key_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-map"}}}
    /// Navigate to [`Map`] across R116(1-M)
    pub fn r116_map<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Map>>> {
        store
            .iter_map()
            .filter(|map| map.read().unwrap().value_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R79(1-M)
    pub fn r79_parameter<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Parameter>>> {
        store
            .iter_parameter()
            .filter(|parameter| parameter.read().unwrap().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub fn r62_span<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Span>>> {
        store
            .iter_span()
            .filter(|span| span.read().unwrap().ty == Some(self.id))
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-struct_field"}}}
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-tuple_field"}}}
    /// Navigate to [`TupleField`] across R86(1-M)
    pub fn r86_tuple_field<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<TupleField>>> {
        store
            .iter_tuple_field()
            .filter(|tuple_field| tuple_field.read().unwrap().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub fn r69_type_cast<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<TypeCast>>> {
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.read().unwrap().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub fn r24_x_value<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<XValue>>> {
        store
            .iter_x_value()
            .filter(|x_value| x_value.read().unwrap().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
