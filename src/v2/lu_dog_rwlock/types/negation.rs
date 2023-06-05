// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"negation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-documentation"}}}
/// The unary minus
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Negation {
    pub id: Uuid,
    /// R70: [`Negation`] '' [`Expression`]
    pub expr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-implementation"}}}
impl Negation {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-impl-new"}}}
    /// Inter a new 'Negation' in the store, and return it's `id`.
    pub fn new(
        expr: &Arc<RwLock<Expression>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Negation>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Negation {
            id,
            expr: expr.read().unwrap().id(),
        }));
        store.inter_negation(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-struct-impl-nav-forward-to-expr"}}}
    /// Navigate to [`Expression`] across R70(1-*)
    pub fn r70_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r70_expression");
        vec![store.exhume_expression(&self.expr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"negation-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
