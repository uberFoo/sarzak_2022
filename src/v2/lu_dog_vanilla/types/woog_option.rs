// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_option-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::value_type::ValueType;
use crate::v2::lu_dog_vanilla::types::z_none::Z_NONE;
use crate::v2::lu_dog_vanilla::types::z_some::ZSome;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-hybrid-documentation"}}}
/// An Optional Type
///
/// This type is either `None` or `Some(`[Type]`)`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WoogOption {
    pub subtype: WoogOptionEnum,
    pub id: Uuid,
    /// R2: [`WoogOption`] 'has a' [`ValueType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum WoogOptionEnum {
    ZNone(Uuid),
    ZSome(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-implementation"}}}
impl WoogOption {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_z_none"}}}
    /// Inter a new WoogOption in the store, and return it's `id`.
    pub fn new_z_none(ty: &ValueType, store: &mut LuDogVanillaStore) -> WoogOption {
        let id = Uuid::new_v4();
        let new = WoogOption {
            ty: ty.id(),
            subtype: WoogOptionEnum::ZNone(Z_NONE),
            id,
        };
        store.inter_woog_option(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_z_some"}}}
    /// Inter a new WoogOption in the store, and return it's `id`.
    pub fn new_z_some(
        ty: &ValueType,
        subtype: &ZSome,
        store: &mut LuDogVanillaStore,
    ) -> WoogOption {
        let id = Uuid::new_v4();
        let new = WoogOption {
            ty: ty.id(),
            subtype: WoogOptionEnum::ZSome(subtype.id),
            id,
        };
        store.inter_woog_option(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R2(1-*)
    pub fn r2_value_type<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}