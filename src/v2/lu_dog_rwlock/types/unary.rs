// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-use-statements"}}}
use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
use crate::v2::lu_dog_rwlock::types::negation::NEGATION;
use crate::v2::lu_dog_rwlock::types::not::NOT;
use crate::v2::lu_dog_rwlock::types::operator::Operator;
use crate::v2::lu_dog_rwlock::types::operator::OperatorEnum;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-enum-documentation"}}}
/// Unary Operators
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Unary {
    Negation(Uuid),
    Not(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-implementation"}}}
impl Unary {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-new-impl"}}}
    /// Create a new instance of Unary::Negation
    pub fn new_negation(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_unary(&NEGATION).unwrap()
    }

    /// Create a new instance of Unary::Not
    pub fn new_not(store: &LuDogRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_unary(&NOT).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Unary::Negation(id) => *id,
            Unary::Not(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Operator>>> {
        span!("r47_operator");
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Unary(id) = operator.read().unwrap().subtype {
                    id == self.id()
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