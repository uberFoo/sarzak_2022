// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"map_element-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::map_expression::MapExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-documentation"}}}
/// An element in a hash map
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapElement {
    pub id: usize,
    /// R116: [`MapElement`] '' [`Expression`]
    pub key: usize,
    /// R117: [`MapElement`] 'is contained in a' [`MapExpression`]
    pub map: usize,
    /// R118: [`MapElement`] '' [`Expression`]
    pub x_value: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-implementation"}}}
impl MapElement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-new"}}}
    /// Inter a new 'Map Element' in the store, and return it's `id`.
    pub fn new(
        key: &Rc<RefCell<Expression>>,
        map: &Rc<RefCell<MapExpression>>,
        x_value: &Rc<RefCell<Expression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<MapElement>> {
        store.inter_map_element(|id| {
            Rc::new(RefCell::new(MapElement {
                id,
                key: key.borrow().id,
                map: map.borrow().id,
                x_value: x_value.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-nav-forward-to-key"}}}
    /// Navigate to [`Expression`] across R116(1-*)
    pub fn r116_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.key).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-nav-forward-to-map"}}}
    /// Navigate to [`MapExpression`] across R117(1-*)
    pub fn r117_map_expression<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<MapExpression>>> {
        vec![store.exhume_map_expression(&self.map).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-nav-forward-to-x_value"}}}
    /// Navigate to [`Expression`] across R118(1-*)
    pub fn r118_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.x_value).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-implementation"}}}
impl PartialEq for MapElement {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.map == other.map && self.x_value == other.x_value
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
