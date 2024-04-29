// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"map_element-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::map_expression::MapExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-documentation"}}}
/// An element in a hash map
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MapElement {
    pub id: Uuid,
    /// R116: [`MapElement`] '' [`Expression`]
    pub key: Uuid,
    /// R117: [`MapElement`] 'is contained in a' [`MapExpression`]
    pub map: Uuid,
    /// R118: [`MapElement`] '' [`Expression`]
    pub x_value: Uuid,
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
        store: &mut LuDogStore,
    ) -> Rc<RefCell<MapElement>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(MapElement {
            id,
            key: key.borrow().id,
            map: map.borrow().id,
            x_value: x_value.borrow().id,
        }));
        store.inter_map_element(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-nav-forward-to-key"}}}
    /// Navigate to [`Expression`] across R116(1-*)
    pub fn r116_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.key).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-nav-forward-to-map"}}}
    /// Navigate to [`MapExpression`] across R117(1-*)
    pub fn r117_map_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<MapExpression>>> {
        vec![store.exhume_map_expression(&self.map).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-nav-forward-to-x_value"}}}
    /// Navigate to [`Expression`] across R118(1-*)
    pub fn r118_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.x_value).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
