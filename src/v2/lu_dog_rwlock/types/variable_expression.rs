// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"variable_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-struct-documentation"}}}
/// A Local Variable Expression
///
/// This is what happens when a variable is an r-value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VariableExpression {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-implementation"}}}
impl VariableExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-struct-impl-new"}}}
    /// Inter a new 'Variable Expression' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut LuDogRwlockStore) -> Arc<RwLock<VariableExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(VariableExpression { id, name }));
        store.inter_variable_expression(new.clone());
        new
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-struct-impl-nav-forward-cond-to-next"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-struct-impl-nav-backward-one-bi-cond-to-variable_expression"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::VariableExpression(id) = expression.read().unwrap().subtype {
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
