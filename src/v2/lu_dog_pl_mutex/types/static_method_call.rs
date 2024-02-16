// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"static_method_call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::call::Call;
use crate::v2::lu_dog_pl_mutex::types::call::CallEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-struct-documentation"}}}
/// A Static Method Call
///
/// This is when you call a function on the type (struct) itself. There is no instance involved
///  in this, although it may return an instance.
///
/// The name attribute is the name of the static method.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StaticMethodCall {
    pub func: String,
    pub id: Uuid,
    pub ty: String,
    pub unique: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-implementation"}}}
impl StaticMethodCall {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-struct-impl-new"}}}
    /// Inter a new 'Static Method Call' in the store, and return it's `id`.
    pub fn new(
        func: String,
        ty: String,
        unique: Uuid,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<StaticMethodCall>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(StaticMethodCall {
            func,
            id,
            ty,
            unique,
        }));
        store.inter_static_method_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"static_method_call-impl-nav-subtype-to-supertype-call"}}}
    // Navigate to [`Call`] across R30(isa)
    pub fn r30_call<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Call>>> {
        vec![store
            .iter_call()
            .find(|call| {
                if let CallEnum::StaticMethodCall(id) = call.lock().subtype {
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