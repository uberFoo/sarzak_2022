// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_return-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-struct-documentation"}}}
/// The Return Expression
///
/// It’s an expression, and not a statement. Isn’t that interesting?
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XReturn {
    pub id: Uuid,
    /// R45: [`XReturn`] 'evaluates and returns' [`Expression`]
    pub expression: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-implementation"}}}
impl XReturn {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-struct-impl-new"}}}
    /// Inter a new 'Return' in the store, and return it's `id`.
    pub fn new(
        expression: &Arc<RwLock<Expression>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<XReturn>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(XReturn {
            id,
            expression: expression.read().unwrap().id(),
        }));
        store.inter_x_return(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R45(1-*)
    pub fn r45_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r45_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_return-impl-nav-subtype-to-supertype-expression"}}}
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
