// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_match-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_rwlock_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_rwlock_vec::types::pattern::Pattern;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-documentation"}}}
/// Match a pattern to a scrutinee and evaluate a branch based on the results.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XMatch {
    pub id: usize,
    pub uniqueness_generator: Uuid,
    /// R91: [`XMatch`] 'deconstructs ' [`Expression`]
    pub scrutinee: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-implementation"}}}
impl XMatch {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-new"}}}
    /// Inter a new 'Match' in the store, and return it's `id`.
    pub fn new(
        uniqueness_generator: Uuid,
        scrutinee: &Arc<RwLock<Expression>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<XMatch>> {
        store.inter_x_match(|id| {
            Arc::new(RwLock::new(XMatch {
                id,
                uniqueness_generator,
                scrutinee: scrutinee.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-forward-to-pattern"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-forward-to-scrutinee"}}}
    /// Navigate to [`Expression`] across R91(1-*)
    pub fn r91_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.scrutinee).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-backward-assoc-many-to-pattern"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-struct-impl-nav-backward-assoc-one-to-pattern"}}}
    /// Navigate to [`Pattern`] across R87(1-1)
    pub fn r87_pattern<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Pattern>>> {
        vec![store
            .iter_pattern()
            .find(|pattern| pattern.read().unwrap().x_match == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::XMatch(id) = expression.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_match-implementation"}}}
impl PartialEq for XMatch {
    fn eq(&self, other: &Self) -> bool {
        self.uniqueness_generator == other.uniqueness_generator && self.scrutinee == other.scrutinee
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
