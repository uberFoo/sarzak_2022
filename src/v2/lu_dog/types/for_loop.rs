// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"for_loop-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
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
    pub id: Uuid,
    pub ident: String,
    /// R43: [`ForLoop`] 'executes a' [`Expression`]
    pub block: Uuid,
    /// R42: [`ForLoop`] 'iterates over an' [`Expression`]
    pub expression: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-implementation"}}}
impl ForLoop {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-new"}}}
    /// Inter a new 'For Loop' in the store, and return it's `id`.
    pub fn new(
        ident: String,
        block: &Rc<RefCell<Expression>>,
        expression: &Rc<RefCell<Expression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ForLoop>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ForLoop {
            id,
            ident,
            block: block.borrow().id(),
            expression: expression.borrow().id(),
        }));
        store.inter_for_loop(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Expression`] across R43(1-*)
    pub fn r43_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R42(1-*)
    pub fn r42_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"for_loop-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
