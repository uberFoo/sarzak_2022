// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"z_some-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::woog_option::WoogOption;
use crate::v2::lu_dog_rwlock::types::woog_option::WoogOptionEnum;
use crate::v2::lu_dog_rwlock::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-documentation"}}}
/// Some Type
///
/// This type wraps another. It's used by the supertype, `[Option]`, to represent a type that
///  may or may not exist.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ZSome {
    pub id: Uuid,
    /// R23: [`ZSome`] 'contains' [`XValue`]
    pub inner: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-implementation"}}}
impl ZSome {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-new"}}}
    /// Inter a new 'Some' in the store, and return it's `id`.
    pub fn new(inner: &Arc<RwLock<XValue>>, store: &mut LuDogRwlockStore) -> Arc<RwLock<ZSome>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(ZSome {
            id,
            inner: inner.read().unwrap().id,
        }));
        store.inter_z_some(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-nav-forward-to-inner"}}}
    /// Navigate to [`XValue`] across R23(1-*)
    pub fn r23_x_value<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r23_x_value");
        vec![store.exhume_x_value(&self.inner).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-impl-nav-subtype-to-supertype-woog_option"}}}
    // Navigate to [`WoogOption`] across R3(isa)
    pub fn r3_woog_option<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<WoogOption>>> {
        span!("r3_woog_option");
        vec![store
            .iter_woog_option()
            .find(|woog_option| {
                if let WoogOptionEnum::ZSome(id) = woog_option.read().unwrap().subtype {
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
