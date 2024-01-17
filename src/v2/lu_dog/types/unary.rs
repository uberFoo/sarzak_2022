// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::negation::NEGATION;
use crate::v2::lu_dog::types::not::NOT;
use crate::v2::lu_dog::types::operator::Operator;
use crate::v2::lu_dog::types::operator::OperatorEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-hybrid-documentation"}}}
/// Unary Operators
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Unary {
    pub subtype: UnaryEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum UnaryEnum {
    Negation(Uuid),
    Not(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-implementation"}}}
impl Unary {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-struct-impl-new_negation"}}}
    /// Inter a new Unary in the store, and return it's `id`.
    pub fn new_negation(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<Unary>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Unary {
            bogus: bogus,
            subtype: UnaryEnum::Negation(NEGATION),
            id,
        }));
        store.inter_unary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-struct-impl-new_not"}}}
    /// Inter a new Unary in the store, and return it's `id`.
    pub fn new_not(bogus: bool, store: &mut LuDogStore) -> Rc<RefCell<Unary>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Unary {
            bogus: bogus,
            subtype: UnaryEnum::Not(NOT),
            id,
        }));
        store.inter_unary(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-impl-nav-subtype-to-supertype-operator"}}}
    // Navigate to [`Operator`] across R47(isa)
    pub fn r47_operator<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Operator>>> {
        vec![store
            .iter_operator()
            .find(|operator| {
                if let OperatorEnum::Unary(id) = operator.borrow().subtype {
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
