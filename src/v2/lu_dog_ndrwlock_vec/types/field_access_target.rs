// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access_target-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::enum_field::EnumField;
use crate::v2::lu_dog_ndrwlock_vec::types::field::Field;
use crate::v2::lu_dog_ndrwlock_vec::types::field_access::FieldAccess;
use crate::v2::lu_dog_ndrwlock_vec::types::function::Function;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-documentation"}}}
/// The target of a field access.
///
/// It may be either a [`Field`] or a [`Function`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldAccessTarget {
    pub subtype: FieldAccessTargetEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FieldAccessTargetEnum {
    EnumField(usize),
    Field(usize),
    Function(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl FieldAccessTarget {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_enum_field"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_enum_field(
        subtype: &Arc<RwLock<EnumField>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<FieldAccessTarget>> {
        store.inter_field_access_target(|id| {
            Arc::new(RwLock::new(FieldAccessTarget {
                subtype: FieldAccessTargetEnum::EnumField(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_field"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_field(
        subtype: &Arc<RwLock<Field>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<FieldAccessTarget>> {
        store.inter_field_access_target(|id| {
            Arc::new(RwLock::new(FieldAccessTarget {
                subtype: FieldAccessTargetEnum::Field(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_function"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_function(
        subtype: &Arc<RwLock<Function>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<FieldAccessTarget>> {
        store.inter_field_access_target(|id| {
            Arc::new(RwLock::new(FieldAccessTarget {
                subtype: FieldAccessTargetEnum::Function(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R65(1-M)
    pub fn r65_field_access<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<FieldAccess>>> {
        span!("r65_field_access");
        store
            .iter_field_access()
            .filter(|field_access| field_access.read().unwrap().field == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl PartialEq for FieldAccessTarget {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
