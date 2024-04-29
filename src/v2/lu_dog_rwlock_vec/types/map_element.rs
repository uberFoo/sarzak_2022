// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"map_element-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_rwlock_vec::types::map_expression::MapExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
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
        key: &Arc<RwLock<Expression>>,
        map: &Arc<RwLock<MapExpression>>,
        x_value: &Arc<RwLock<Expression>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<MapElement>> {
        store.inter_map_element(|id| {
            Arc::new(RwLock::new(MapElement {
                id,
                key: key.read().unwrap().id,
                map: map.read().unwrap().id,
                x_value: x_value.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-nav-forward-to-key"}}}
    /// Navigate to [`Expression`] across R116(1-*)
    pub fn r116_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.key).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-nav-forward-to-map"}}}
    /// Navigate to [`MapExpression`] across R117(1-*)
    pub fn r117_map_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<MapExpression>>> {
        vec![store.exhume_map_expression(&self.map).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_element-struct-impl-nav-forward-to-x_value"}}}
    /// Navigate to [`Expression`] across R118(1-*)
    pub fn r118_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
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
