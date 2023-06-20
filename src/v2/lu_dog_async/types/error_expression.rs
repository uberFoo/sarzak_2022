// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"error_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-struct-documentation"}}}
/// An Error...
///
/// I'm not sure what to do with this.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ErrorExpression {
    pub id: Uuid,
    pub span: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-implementation"}}}
impl ErrorExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-struct-impl-new"}}}
    /// Inter a new 'Error Expression' in the store, and return it's `id`.
    pub async fn new(span: String, store: &mut LuDogAsyncStore) -> Arc<RwLock<ErrorExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(ErrorExpression { id, span }));
        store.inter_error_expression(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
