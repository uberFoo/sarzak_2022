// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"z_some-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::expression::Expression;
use crate::v2::lu_dog_vanilla::types::woog_option::WoogOption;
use crate::v2::lu_dog_vanilla::types::woog_option::WoogOptionEnum;
use crate::v2::lu_dog_vanilla::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
    pub fn new(inner: &XValue, store: &mut LuDogVanillaStore) -> ZSome {
        let id = Uuid::new_v4();
        let new = ZSome {
            id,
            inner: inner.id,
        };
        store.inter_z_some(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-struct-impl-nav-forward-to-inner"}}}
    /// Navigate to [`XValue`] across R23(1-*)
    pub fn r23_x_value<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&XValue> {
        vec![store.exhume_x_value(&self.inner).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"z_some-impl-nav-subtype-to-supertype-woog_option"}}}
    // Navigate to [`WoogOption`] across R3(isa)
    pub fn r3_woog_option<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&WoogOption> {
        vec![store
            .iter_woog_option()
            .find(|woog_option| {
                if let WoogOptionEnum::ZSome(id) = woog_option.subtype {
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
