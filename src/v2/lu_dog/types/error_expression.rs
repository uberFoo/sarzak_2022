// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"error_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
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
    pub fn new(span: String, store: &mut LuDogStore) -> ErrorExpression {
        let id = Uuid::new_v4();
        let new = ErrorExpression { id: id, span: span };
        store.inter_error_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-struct-impl-new_"}}}
    /// Inter a new 'Error Expression' in the store, and return it's `id`.
    pub fn new_(span: String) -> ErrorExpression {
        let id = Uuid::new_v4();
        let new = ErrorExpression { id: id, span: span };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"error_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}