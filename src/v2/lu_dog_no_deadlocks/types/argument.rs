// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"argument-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::call::Call;
use crate::v2::lu_dog_rwlock::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-documentation"}}}
/// An Argument to a Function Call
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Argument {
    pub id: Uuid,
    /// R27: [`Argument`] 'follows' [`Argument`]
    pub next: Option<Uuid>,
    /// R28: [`Argument`] 'is part of a' [`Call`]
    pub function: Uuid,
    /// R37: [`Argument`] '' [`Expression`]
    pub expression: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-implementation"}}}
impl Argument {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-new"}}}
    /// Inter a new 'Argument' in the store, and return it's `id`.
    pub fn new(
        next: Option<&Arc<RwLock<Argument>>>,
        function: &Arc<RwLock<Call>>,
        expression: &Arc<RwLock<Expression>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Argument>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Argument {
            id,
            next: next.map(|argument| argument.read().unwrap().id),
            function: function.read().unwrap().id,
            expression: expression.read().unwrap().id(),
        }));
        store.inter_argument(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Argument`] across R27(1-*c)
    pub fn r27_argument<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Argument>>> {
        span!("r27_argument");
        match self.next {
            Some(ref next) => vec![store.exhume_argument(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-to-function"}}}
    /// Navigate to [`Call`] across R28(1-*)
    pub fn r28_call<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Call>>> {
        span!("r28_call");
        vec![store.exhume_call(&self.function).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R37(1-*)
    pub fn r37_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r37_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"argument-struct-impl-nav-backward-one-bi-cond-to-argument"}}}
    /// Navigate to [`Argument`] across R27(1c-1c)
    pub fn r27c_argument<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Argument>>> {
        span!("r27_argument");
        let argument = store
            .iter_argument()
            .find(|argument| argument.read().unwrap().next == Some(self.id));
        match argument {
            Some(ref argument) => vec![argument.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
