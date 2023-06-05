// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
use crate::v2::lu_dog_rwlock::types::boolean_literal::BooleanLiteral;
use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::float_literal::FloatLiteral;
use crate::v2::lu_dog_rwlock::types::integer_literal::IntegerLiteral;
use crate::v2::lu_dog_rwlock::types::string_literal::StringLiteral;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-enum-documentation"}}}
/// A Literal Expression
///
/// This is any literal value in the program.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Literal {
    BooleanLiteral(Uuid),
    FloatLiteral(Uuid),
    IntegerLiteral(Uuid),
    StringLiteral(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-implementation"}}}
impl Literal {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-new-impl"}}}
    /// Create a new instance of Literal::BooleanLiteral
    pub fn new_boolean_literal(
        boolean_literal: &Arc<RwLock<BooleanLiteral>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = boolean_literal.read().unwrap().id();
        if let Some(boolean_literal) = store.exhume_literal(&id) {
            boolean_literal
        } else {
            let new = Arc::new(RwLock::new(Self::BooleanLiteral(id)));
            store.inter_literal(new.clone());
            new
        }
    }

    /// Create a new instance of Literal::FloatLiteral
    pub fn new_float_literal(
        float_literal: &Arc<RwLock<FloatLiteral>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = float_literal.read().unwrap().id;
        if let Some(float_literal) = store.exhume_literal(&id) {
            float_literal
        } else {
            let new = Arc::new(RwLock::new(Self::FloatLiteral(id)));
            store.inter_literal(new.clone());
            new
        }
    }

    /// Create a new instance of Literal::IntegerLiteral
    pub fn new_integer_literal(
        integer_literal: &Arc<RwLock<IntegerLiteral>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = integer_literal.read().unwrap().id;
        if let Some(integer_literal) = store.exhume_literal(&id) {
            integer_literal
        } else {
            let new = Arc::new(RwLock::new(Self::IntegerLiteral(id)));
            store.inter_literal(new.clone());
            new
        }
    }

    /// Create a new instance of Literal::StringLiteral
    pub fn new_string_literal(
        string_literal: &Arc<RwLock<StringLiteral>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = string_literal.read().unwrap().id;
        if let Some(string_literal) = store.exhume_literal(&id) {
            string_literal
        } else {
            let new = Arc::new(RwLock::new(Self::StringLiteral(id)));
            store.inter_literal(new.clone());
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Literal::BooleanLiteral(id) => *id,
            Literal::FloatLiteral(id) => *id,
            Literal::IntegerLiteral(id) => *id,
            Literal::StringLiteral(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
