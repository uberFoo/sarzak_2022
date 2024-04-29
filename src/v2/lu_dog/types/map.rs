// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"map-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::value_type::ValueType;
use crate::v2::lu_dog::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-documentation"}}}
/// This is a hashmap.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Map {
    pub id: Uuid,
    /// R115: [`Map`] 'has a key' [`ValueType`]
    pub key_type: Uuid,
    /// R116: [`Map`] 'values have' [`ValueType`]
    pub value_type: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-implementation"}}}
impl Map {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-impl-new"}}}
    /// Inter a new 'Map' in the store, and return it's `id`.
    pub fn new(
        key_type: &Rc<RefCell<ValueType>>,
        value_type: &Rc<RefCell<ValueType>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Map>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Map {
            id,
            key_type: key_type.borrow().id,
            value_type: value_type.borrow().id,
        }));
        store.inter_map(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-impl-nav-forward-to-key_type"}}}
    /// Navigate to [`ValueType`] across R115(1-*)
    pub fn r115_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store.exhume_value_type(&self.key_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-struct-impl-nav-forward-to-value_type"}}}
    /// Navigate to [`ValueType`] across R116(1-*)
    pub fn r116_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store.exhume_value_type(&self.value_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
