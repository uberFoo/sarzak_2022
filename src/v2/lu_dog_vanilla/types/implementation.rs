// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"implementation-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::function::Function;
use crate::v2::lu_dog_vanilla::types::item::Item;
use crate::v2::lu_dog_vanilla::types::item::ItemEnum;
use crate::v2::lu_dog_vanilla::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-documentation"}}}
/// An Implementation Block
///
/// Inside this block functions are defined on a [`ModellType`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Implementation {
    pub id: Uuid,
    /// R8: [`Implementation`] 'adds functions to a' [`WoogStruct`]
    pub model_type: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-implementation"}}}
impl Implementation {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-impl-new"}}}
    /// Inter a new 'Implementation' in the store, and return it's `id`.
    pub fn new(model_type: &WoogStruct, store: &mut LuDogVanillaStore) -> Implementation {
        let id = Uuid::new_v4();
        let new = Implementation {
            id,
            model_type: model_type.id,
        };
        store.inter_implementation(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-impl-nav-forward-to-model_type"}}}
    /// Navigate to [`WoogStruct`] across R8(1-*)
    pub fn r8_woog_struct<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&WoogStruct> {
        vec![store.exhume_woog_struct(&self.model_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-struct-impl-nav-backward-1_Mc-to-function"}}}
    /// Navigate to [`Function`] across R9(1-Mc)
    pub fn r9_function<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Function> {
        store
            .iter_function()
            .filter_map(|function| {
                if function.impl_block == Some(self.id) {
                    Some(function)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Item> {
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::Implementation(id) = item.subtype {
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