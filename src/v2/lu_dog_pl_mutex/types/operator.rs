// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"operator-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::binary::Binary;
use crate::v2::lu_dog_pl_mutex::types::comparison::Comparison;
use crate::v2::lu_dog_pl_mutex::types::expression::Expression;
use crate::v2::lu_dog_pl_mutex::types::unary::Unary;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-hybrid-documentation"}}}
/// Operator Expressions
///
/// Basically anything you can do with an expression is a subtype of this beasty.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Operator {
    pub subtype: OperatorEnum,
    pub id: Uuid,
    /// R51: [`Operator`] 'right hand side' [`Expression`]
    pub rhs: Option<Uuid>,
    /// R50: [`Operator`] 'left hand side' [`Expression`]
    pub lhs: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum OperatorEnum {
    Binary(Uuid),
    Comparison(Uuid),
    Unary(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-implementation"}}}
impl Operator {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_binary"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub fn new_binary(
        rhs: Option<&Arc<Mutex<Expression>>>,
        lhs: &Arc<Mutex<Expression>>,
        subtype: &Arc<Mutex<Binary>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Operator>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Operator {
            rhs: rhs.map(|expression| expression.lock().id()),
            lhs: lhs.lock().id(),
            subtype: OperatorEnum::Binary(subtype.lock().id()),
            id,
        }));
        store.inter_operator(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_comparison"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub fn new_comparison(
        rhs: Option<&Arc<Mutex<Expression>>>,
        lhs: &Arc<Mutex<Expression>>,
        subtype: &Arc<Mutex<Comparison>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Operator>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Operator {
            rhs: rhs.map(|expression| expression.lock().id()),
            lhs: lhs.lock().id(),
            subtype: OperatorEnum::Comparison(subtype.lock().id()),
            id,
        }));
        store.inter_operator(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-to-lhs"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-new_unary"}}}
    /// Inter a new Operator in the store, and return it's `id`.
    pub fn new_unary(
        rhs: Option<&Arc<Mutex<Expression>>>,
        lhs: &Arc<Mutex<Expression>>,
        subtype: &Arc<Mutex<Unary>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Operator>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Operator {
            rhs: rhs.map(|expression| expression.lock().id()),
            lhs: lhs.lock().id(),
            subtype: OperatorEnum::Unary(subtype.lock().id()),
            id,
        }));
        store.inter_operator(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-cond-to-rhs"}}}
    /// Navigate to [`Expression`] across R51(1-*c)
    pub fn r51_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        span!("r51_expression");
        match self.rhs {
            Some(ref rhs) => vec![store.exhume_expression(rhs).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-struct-impl-nav-forward-to-lhs"}}}
    /// Navigate to [`Expression`] across R50(1-*)
    pub fn r50_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        span!("r50_expression");
        vec![store.exhume_expression(&self.lhs).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"operator-impl-nav-subtype-to-supertype-expression"}}}
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
