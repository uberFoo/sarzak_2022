// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"halt_and_catch_fire-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::expression::ExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-struct-documentation"}}}
/// Goes boom!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HaltAndCatchFire {
    pub id: Uuid,
    /// R114: [`HaltAndCatchFire`] '' [`Expression`]
    pub expression: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-implementation"}}}
impl HaltAndCatchFire {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-struct-impl-new"}}}
    /// Inter a new 'Halt and Catch Fire' in the store, and return it's `id`.
    pub fn new(
        expression: &Rc<RefCell<Expression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<HaltAndCatchFire>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(HaltAndCatchFire {
            id,
            expression: expression.borrow().id,
        }));
        store.inter_halt_and_catch_fire(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R114(1-*)
    pub fn r114_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::HaltAndCatchFire(id) = expression.borrow().subtype {
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