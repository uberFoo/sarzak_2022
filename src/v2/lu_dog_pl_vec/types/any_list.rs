// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"any_list-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::value_type::ValueType;
use crate::v2::lu_dog_pl_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-documentation"}}}
/// A list that does not enforce that it’s elements all share a type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnyList {
    pub bogus: Uuid,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-implementation"}}}
impl AnyList {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-impl-new"}}}
    /// Inter a new 'Any List' in the store, and return it's `id`.
    pub fn new(bogus: Uuid, store: &mut LuDogPlVecStore) -> Arc<RwLock<AnyList>> {
        store.inter_any_list(|id| Arc::new(RwLock::new(AnyList { bogus, id })))
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::AnyList(id) = value_type.read().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-implementation"}}}
impl PartialEq for AnyList {
    fn eq(&self, other: &Self) -> bool {
        self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
