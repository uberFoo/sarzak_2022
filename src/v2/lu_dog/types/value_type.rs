// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::any_list::ANY_LIST;
use crate::v2::lu_dog::types::char::CHAR;
use crate::v2::lu_dog::types::empty::EMPTY;
use crate::v2::lu_dog::types::enum_generic::EnumGeneric;
use crate::v2::lu_dog::types::enum_generic_type::EnumGenericType;
use crate::v2::lu_dog::types::enumeration::Enumeration;
use crate::v2::lu_dog::types::field::Field;
use crate::v2::lu_dog::types::func_generic::FuncGeneric;
use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::import::Import;
use crate::v2::lu_dog::types::lambda::Lambda;
use crate::v2::lu_dog::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog::types::list::List;
use crate::v2::lu_dog::types::list_expression::ListExpression;
use crate::v2::lu_dog::types::map::Map;
use crate::v2::lu_dog::types::parameter::Parameter;
use crate::v2::lu_dog::types::range::RANGE;
use crate::v2::lu_dog::types::span::Span;
use crate::v2::lu_dog::types::struct_generic::StructGeneric;
use crate::v2::lu_dog::types::task::TASK;
use crate::v2::lu_dog::types::tuple_field::TupleField;
use crate::v2::lu_dog::types::type_cast::TypeCast;
use crate::v2::lu_dog::types::unknown::UNKNOWN;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use crate::v2::lu_dog::types::x_future::XFuture;
use crate::v2::lu_dog::types::x_plugin::XPlugin;
use crate::v2::lu_dog::types::x_value::XValue;
use crate::v2::lu_dog::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-enum-documentation"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ValueType {
    pub subtype: ValueTypeEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ValueTypeEnum {
    AnyList(Uuid),
    Char(Uuid),
    Empty(Uuid),
    EnumGeneric(Uuid),
    Enumeration(Uuid),
    FuncGeneric(Uuid),
    Function(Uuid),
    XFuture(Uuid),
    Import(Uuid),
    Lambda(Uuid),
    List(Uuid),
    Map(Uuid),
    ZObjectStore(Uuid),
    XPlugin(Uuid),
    Range(Uuid),
    WoogStruct(Uuid),
    StructGeneric(Uuid),
    Task(Uuid),
    Ty(Uuid),
    Unknown(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl ValueType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_any_list"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_any_list(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::AnyList(ANY_LIST),
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_char"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_char(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Char(CHAR),
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_empty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_empty(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Empty(EMPTY),
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enumeration"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enum_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_enum_generic(
        bogus: bool,
        subtype: &Rc<RefCell<EnumGeneric>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::EnumGeneric(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_function"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enumeration"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_enumeration(
        bogus: bool,
        subtype: &Rc<RefCell<Enumeration>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Enumeration(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_future"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_func_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_func_generic(
        bogus: bool,
        subtype: &Rc<RefCell<FuncGeneric>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::FuncGeneric(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_function"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_function(
        bogus: bool,
        subtype: &Rc<RefCell<Function>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Function(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_generic"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_future"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_x_future(
        bogus: bool,
        subtype: &Rc<RefCell<XFuture>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::XFuture(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_import"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_import(
        bogus: bool,
        subtype: &Rc<RefCell<Import>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Import(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_lambda"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_lambda(
        bogus: bool,
        subtype: &Rc<RefCell<Lambda>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Lambda(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_list"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_list(
        bogus: bool,
        subtype: &Rc<RefCell<List>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::List(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_map"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_map(
        bogus: bool,
        subtype: &Rc<RefCell<Map>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Map(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_z_object_store"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_z_object_store(
        bogus: bool,
        subtype: &Rc<RefCell<ZObjectStore>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::ZObjectStore(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_plugin"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_x_plugin(
        bogus: bool,
        subtype: &Rc<RefCell<XPlugin>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::XPlugin(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_range"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_range(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Range(RANGE),
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_struct"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_woog_struct(
        bogus: bool,
        subtype: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::WoogStruct(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_struct_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_struct_generic(
        bogus: bool,
        subtype: &Rc<RefCell<StructGeneric>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::StructGeneric(subtype.borrow().id), // b
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_task"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_task(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Task(TASK),
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_ty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_ty(
        bogus: bool,
        subtype: &std::sync::Arc<std::sync::RwLock<Ty>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Ty(subtype.read().unwrap().id()),
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_unknown"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_unknown(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<ValueType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Unknown(UNKNOWN),
            id,
        }));
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-enum_generic_type"}}}
    /// Navigate to [`EnumGenericType`] across R119(1-M)
    pub fn r119_enum_generic_type<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<EnumGenericType>>> {
        store
            .iter_enum_generic_type()
            .filter(|enum_generic_type| enum_generic_type.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R5(1-M)
    pub fn r5_field<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Field>>> {
        store
            .iter_field()
            .filter(|field| field.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub fn r10_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Function>>> {
        store
            .iter_function()
            .filter(|function| function.borrow().return_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_future"}}}
    /// Navigate to [`XFuture`] across R2(1-M)
    pub fn r2_x_future<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<XFuture>>> {
        store
            .iter_x_future()
            .filter(|x_future| x_future.borrow().x_value == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-generic"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-lambda"}}}
    /// Navigate to [`Lambda`] across R74(1-M)
    pub fn r74_lambda<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Lambda>>> {
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
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<LambdaParameter>>> {
        store
            .iter_lambda_parameter()
            .filter(|lambda_parameter| lambda_parameter.borrow().ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub fn r36_list<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<List>>> {
        store
            .iter_list()
            .filter(|list| list.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-one-to-list_expression"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list_expression"}}}
    /// Navigate to [`ListExpression`] across R257(1-M)
    pub fn r257_list_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<ListExpression>>> {
        store
            .iter_list_expression()
            .filter(|list_expression| list_expression.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-map"}}}
    /// Navigate to [`Map`] across R115(1-M)
    pub fn r115_map<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Map>>> {
        store
            .iter_map()
            .filter(|map| map.borrow().key_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-map"}}}
    /// Navigate to [`Map`] across R116(1-M)
    pub fn r116_map<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Map>>> {
        store
            .iter_map()
            .filter(|map| map.borrow().value_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R79(1-M)
    pub fn r79_parameter<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Parameter>>> {
        store
            .iter_parameter()
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
            .filter(|parameter| parameter.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub fn r62_span<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Span>>> {
        store
            .iter_span()
            .filter(|span| span.borrow().ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-tuple_field"}}}
    /// Navigate to [`TupleField`] across R86(1-M)
    pub fn r86_tuple_field<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<TupleField>>> {
        store
            .iter_tuple_field()
            .filter(|tuple_field| tuple_field.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub fn r69_type_cast<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<TypeCast>>> {
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub fn r24_x_value<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<XValue>>> {
        store
            .iter_x_value()
            .filter(|x_value| x_value.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
