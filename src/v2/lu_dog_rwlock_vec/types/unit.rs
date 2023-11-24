// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unit-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::enum_field::EnumField;
use crate::v2::lu_dog_rwlock_vec::types::enum_field::EnumFieldEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-struct-documentation"}}}
/// Just a marker, no other value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unit {
    pub id: usize,
    pub x_value: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-implementation"}}}
impl Unit {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-struct-impl-new"}}}
    /// Inter a new 'Unit' in the store, and return it's `id`.
    pub fn new(x_value: i64, store: &mut LuDogRwlockVecStore) -> Arc<RwLock<Unit>> {
        store.inter_unit(|id| Arc::new(RwLock::new(Unit { id, x_value })))
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-impl-nav-subtype-to-supertype-enum_field"}}}
    // Navigate to [`EnumField`] across R85(isa)
    pub fn r85_enum_field<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<EnumField>>> {
        vec![store
            .iter_enum_field()
            .find(|enum_field| {
                if let EnumFieldEnum::Unit(id) = enum_field.read().unwrap().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unit-implementation"}}}
impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.x_value == other.x_value
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
