// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"integer_literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::literal::Literal;
use crate::v2::lu_dog_ndrwlock_vec::types::literal::LiteralEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-struct-documentation"}}}
/// An Integer
///
/// I'm not sure what to do about width. I think I coded it as an i64 in the parser.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IntegerLiteral {
    pub id: usize,
    pub x_value: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-implementation"}}}
impl IntegerLiteral {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-struct-impl-new"}}}
    /// Inter a new 'Integer Literal' in the store, and return it's `id`.
    pub fn new(x_value: i64, store: &mut LuDogNdrwlockVecStore) -> Arc<RwLock<IntegerLiteral>> {
        store.inter_integer_literal(|id| Arc::new(RwLock::new(IntegerLiteral { id, x_value })))
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"integer_literal-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Literal>>> {
        span!("r22_literal");
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::IntegerLiteral(id) = literal.read().unwrap().subtype {
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