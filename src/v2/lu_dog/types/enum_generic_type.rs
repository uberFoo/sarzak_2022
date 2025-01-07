// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_generic_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::enum_generic::EnumGeneric;
use crate::v2::lu_dog::types::enumeration::Enumeration;
use crate::v2::lu_dog::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-documentation"}}}
/// This is the type that an instance of an enum generic has. For example, `foo` in `Some(foo
/// )` is `T` in `Option<T>`. If `foo` is `int`, then this would point to an `int`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnumGenericType {
    pub id: Uuid,
    /// R119: [`EnumGenericType`] 'has a' [`ValueType`]
    pub ty: Uuid,
    /// R120: [`EnumGeneric`] '🚧 Comments are out of order — see sarzak#14.' [`EnumGeneric`]
    pub generic: Uuid,
    /// R120: [`Enumeration`] '🚧 Comments are out of order — see sarzak#14.' [`Enumeration`]
    pub enumeration: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-implementation"}}}
impl EnumGenericType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-impl-new"}}}
    /// Inter a new 'Enum Generic Type' in the store, and return it's `id`.
    pub fn new(
        ty: &Rc<RefCell<ValueType>>,
        generic: &Rc<RefCell<EnumGeneric>>,
        enumeration: &Rc<RefCell<Enumeration>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<EnumGenericType>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(EnumGenericType {
            id,
            ty: ty.borrow().id,
            generic: generic.borrow().id,
            enumeration: enumeration.borrow().id,
        }));
        store.inter_enum_generic_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R119(1-*)
    pub fn r119_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-impl-nav-forward-assoc-to-generic"}}}
    /// Navigate to [`EnumGeneric`] across R120(1-*)
    pub fn r120_enum_generic<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<EnumGeneric>>> {
        vec![store.exhume_enum_generic(&self.generic).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-impl-nav-forward-assoc-to-enumeration"}}}
    /// Navigate to [`Enumeration`] across R120(1-*)
    pub fn r120_enumeration<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Enumeration>>> {
        vec![store.exhume_enumeration(&self.enumeration).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}