// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"map_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::literal::Literal;
use crate::v2::lu_dog::types::literal::LiteralEnum;
use crate::v2::lu_dog::types::map_element::MapElement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-struct-documentation"}}}
/// A hashmap expression
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MapExpression {
    pub bogus: Uuid,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-implementation"}}}
impl MapExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-struct-impl-new"}}}
    /// Inter a new 'Map Expression' in the store, and return it's `id`.
    pub fn new(bogus: Uuid, store: &mut LuDogStore) -> Rc<RefCell<MapExpression>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(MapExpression { bogus, id }));
        store.inter_map_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-struct-impl-nav-backward-1_M-to-map_element"}}}
    /// Navigate to [`MapElement`] across R117(1-M)
    pub fn r117_map_element<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<MapElement>>> {
        store
            .iter_map_element()
            .filter(|map_element| map_element.borrow().map == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Literal>>> {
        vec![store
            .iter_literal()
            .find(|literal| {
                if let LiteralEnum::MapExpression(id) = literal.borrow().subtype {
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
