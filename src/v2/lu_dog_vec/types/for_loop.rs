// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"for_loop-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::block::Block;
use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-documentation"}}}
/// A For Loop Expression
///
/// An expression that matches for IDENT in EXPRESSION BLOCK.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ForLoop {
    pub id: usize,
    pub ident: String,
    /// R43: [`ForLoop`] 'executes a' [`Block`]
    pub block: usize,
    /// R42: [`ForLoop`] 'iterates over an' [`Expression`]
    pub expression: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-implementation"}}}
impl ForLoop {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-new"}}}
    /// Inter a new 'For Loop' in the store, and return it's `id`.
    pub fn new(
        ident: String,
        block: &Rc<RefCell<Block>>,
        expression: &Rc<RefCell<Expression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ForLoop>> {
        store.inter_for_loop(|id| {
            Rc::new(RefCell::new(ForLoop {
                id,
                ident: ident.to_owned(),
                block: block.borrow().id,
                expression: expression.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R43(1-*)
    pub fn r43_block<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Block>>> {
        span!("r43_block");
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R42(1-*)
    pub fn r42_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r42_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::ForLoop(id) = expression.borrow().subtype {
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