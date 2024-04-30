// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"map_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::literal::Literal;
use crate::v2::lu_dog_vec_tracy::types::literal::LiteralEnum;
use crate::v2::lu_dog_vec_tracy::types::map_element::MapElement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-struct-documentation"}}}
/// A hashmap expression
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapExpression {
    pub bogus: Uuid,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-implementation"}}}
impl MapExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-struct-impl-new"}}}
    /// Inter a new 'Map Expression' in the store, and return it's `id`.
    pub fn new(bogus: Uuid, store: &mut LuDogVecTracyStore) -> Rc<RefCell<MapExpression>> {
        store.inter_map_expression(|id| Rc::new(RefCell::new(MapExpression { bogus, id })))
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-struct-impl-nav-backward-1_M-to-map_element"}}}
    /// Navigate to [`MapElement`] across R117(1-M)
    pub fn r117_map_element<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<MapElement>>> {
        span!("r117_map_element");
        store
            .iter_map_element()
            .filter(|map_element| map_element.borrow().map == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R22(isa)
    pub fn r22_literal<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Literal>>> {
        span!("r22_literal");
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-implementation"}}}
impl PartialEq for MapExpression {
    fn eq(&self, other: &Self) -> bool {
        self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
