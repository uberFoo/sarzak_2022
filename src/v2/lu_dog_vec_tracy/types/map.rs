// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"map-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::value_type::ValueType;
use crate::v2::lu_dog_vec_tracy::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-documentation"}}}
/// This is a hashmap.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Map {
    pub id: usize,
    /// R115: [`Map`] 'has a key' [`ValueType`]
    pub key_type: usize,
    /// R116: [`Map`] 'values have' [`ValueType`]
    pub value_type: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-implementation"}}}
impl Map {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-impl-new"}}}
    /// Inter a new 'Map' in the store, and return it's `id`.
    pub fn new(
        key_type: &Rc<RefCell<ValueType>>,
        value_type: &Rc<RefCell<ValueType>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Map>> {
        store.inter_map(|id| {
            Rc::new(RefCell::new(Map {
                id,
                key_type: key_type.borrow().id,
                value_type: value_type.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-impl-nav-forward-to-key_type"}}}
    /// Navigate to [`ValueType`] across R115(1-*)
    pub fn r115_value_type<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r115_value_type");
        vec![store.exhume_value_type(&self.key_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-impl-nav-forward-to-value_type"}}}
    /// Navigate to [`ValueType`] across R116(1-*)
    pub fn r116_value_type<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r116_value_type");
        vec![store.exhume_value_type(&self.value_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Map(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-implementation"}}}
impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.key_type == other.key_type && self.value_type == other.value_type
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
