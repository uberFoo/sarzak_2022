// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_generic_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::enum_generic::EnumGeneric;
use crate::v2::lu_dog_vec_tracy::types::enumeration::Enumeration;
use crate::v2::lu_dog_vec_tracy::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-documentation"}}}
/// This is the type that an instance of an enum generic has. For example, `foo` in `Some(foo
/// )` is `T` in `Option<T>`. If `foo` is `int`, then this would point to an `int`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumGenericType {
    pub id: usize,
    /// R119: [`EnumGenericType`] 'has a' [`ValueType`]
    pub ty: usize,
    /// R120: [`EnumGeneric`] '🚧 Comments are out of order — see sarzak#14.' [`EnumGeneric`]
    pub generic: usize,
    /// R120: [`Enumeration`] '🚧 Comments are out of order — see sarzak#14.' [`Enumeration`]
    pub enumeration: usize,
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
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<EnumGenericType>> {
        store.inter_enum_generic_type(|id| {
            Rc::new(RefCell::new(EnumGenericType {
                id,
                ty: ty.borrow().id,
                generic: generic.borrow().id,
                enumeration: enumeration.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R119(1-*)
    pub fn r119_value_type<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r119_value_type");
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-impl-nav-forward-assoc-to-generic"}}}
    /// Navigate to [`EnumGeneric`] across R120(1-*)
    pub fn r120_enum_generic<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<EnumGeneric>>> {
        span!("r120_enum_generic");
        vec![store.exhume_enum_generic(&self.generic).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-struct-impl-nav-forward-assoc-to-enumeration"}}}
    /// Navigate to [`Enumeration`] across R120(1-*)
    pub fn r120_enumeration<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Enumeration>>> {
        span!("r120_enumeration");
        vec![store.exhume_enumeration(&self.enumeration).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic_type-implementation"}}}
impl PartialEq for EnumGenericType {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
            && self.generic == other.generic
            && self.enumeration == other.enumeration
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
