// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"named_field_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::field_expression::FieldExpression;
use crate::v2::lu_dog_rwlock::types::field_expression::FieldExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NamedFieldExpression {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-implementation"}}}
impl NamedFieldExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-struct-impl-new"}}}
    /// Inter a new 'Named Field Expression' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut LuDogRwlockStore) -> Arc<RwLock<NamedFieldExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(NamedFieldExpression { id, name }));
        store.inter_named_field_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-impl-nav-subtype-to-supertype-field_expression"}}}
    // Navigate to [`FieldExpression`] across R94(isa)
    pub fn r94_field_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FieldExpression>>> {
        span!("r94_field_expression");
        vec![store
            .iter_field_expression()
            .find(|field_expression| {
                if let FieldExpressionEnum::NamedFieldExpression(id) =
                    field_expression.read().unwrap().subtype
                {
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
