// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"print-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-struct-documentation"}}}
/// A Print Expression?
///
/// Shold this be a statement?
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Print {
    pub id: Uuid,
    /// R32: [`Print`] '' [`Expression`]
    pub expression: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-implementation"}}}
impl Print {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-struct-impl-new"}}}
    /// Inter a new 'Print' in the store, and return it's `id`.
    pub fn new(
        expression: &Arc<Mutex<Expression>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Print>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Print {
            id,
            expression: expression.lock().id(),
        }));
        store.inter_print(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R32(1-*)
    pub fn r32_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        span!("r32_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"print-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
