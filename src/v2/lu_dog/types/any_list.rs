// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"any_list-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::value_type::ValueType;
use crate::v2::lu_dog::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-documentation"}}}
/// A list that does not enforce that it’s elements all share a type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AnyList {
    pub bogus: Uuid,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-implementation"}}}
impl AnyList {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-impl-new"}}}
    /// Inter a new 'Any List' in the store, and return it's `id`.
    pub fn new(bogus: Uuid, store: &mut LuDogStore) -> Rc<RefCell<AnyList>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(AnyList { bogus, id }));
        store.inter_any_list(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::AnyList(id) = value_type.borrow().subtype {
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
