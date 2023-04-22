// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"reference-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-documentation"}}}
/// A Reference to a Type
///
/// This type depends on instance pointers being unique.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Reference {
    pub address: Uuid,
    pub id: Uuid,
    pub is_valid: bool,
    /// R35: [`Reference`] '' [`ValueType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-implementation"}}}
impl Reference {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-new"}}}
    /// Inter a new 'Reference' in the store, and return it's `id`.
    pub fn new(address: Uuid, is_valid: bool, ty: &ValueType, store: &mut LuDogStore) -> Reference {
        let id = Uuid::new_v4();
        let new = Reference {
            address: address,
            id: id,
            is_valid: is_valid,
            ty: ty.id(),
        };
        store.inter_reference(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-new_"}}}
    /// Inter a new 'Reference' in the store, and return it's `id`.
    pub fn new_(address: Uuid, is_valid: bool, ty: &ValueType) -> Reference {
        let id = Uuid::new_v4();
        let new = Reference {
            address: address,
            id: id,
            is_valid: is_valid,
            ty: ty.id(),
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-nav-forward-to-object"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R35(1-*)
    pub fn r35_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}