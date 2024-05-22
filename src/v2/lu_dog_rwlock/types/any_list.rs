// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"any_list-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::value_type::ValueType;
use crate::v2::lu_dog_rwlock::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-documentation"}}}
/// A list that does not enforce that it’s elements all share a type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnyList {
    pub bogus: Uuid,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-implementation"}}}
impl AnyList {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-impl-new"}}}
    /// Inter a new 'Any List' in the store, and return it's `id`.
    pub fn new(bogus: Uuid, store: &mut LuDogRwlockStore) -> Arc<RwLock<AnyList>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(AnyList { bogus, id }));
        store.inter_any_list(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::AnyList(id) = value_type.read().unwrap().subtype {
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
