// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_option-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::value_type::ValueType;
use crate::v2::lu_dog::types::z_none::Z_NONE;
use crate::v2::lu_dog::types::z_some::ZSome;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-hybrid-documentation"}}}
/// An Optional Type
///
/// This type is either `None` or `Some(`[Type]`)`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-enum-definition"}}}
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_none"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_z_none"}}}
    /// Inter a new WoogOption in the store, and return it's `id`.
    pub fn new_z_none(ty: &ValueType, store: &mut LuDogStore) -> WoogOption {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = Z_NONE;
        let new = WoogOption {
            ty: ty.id(),
            subtype: WoogOptionEnum::ZNone(Z_NONE),
            id,
        };
        store.inter_woog_option(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_none_"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_z_none_"}}}
    /// Inter a new WoogOption in the store, and return it's `id`.
    pub fn new_z_none_(ty: &ValueType) -> WoogOption {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = Z_NONE;
        let new = WoogOption {
            ty: ty.id(),
            subtype: WoogOptionEnum::ZNone(Z_NONE),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_some"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_z_some"}}}
    /// Inter a new WoogOption in the store, and return it's `id`.
    pub fn new_z_some(ty: &ValueType, subtype: &ZSome, store: &mut LuDogStore) -> WoogOption {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = WoogOption {
            ty: ty.id(),
            subtype: WoogOptionEnum::ZSome(subtype.id),
            id,
        };
        store.inter_woog_option(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_some_"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-new_z_some_"}}}
    /// Inter a new WoogOption in the store, and return it's `id`.
    pub fn new_z_some_(ty: &ValueType, subtype: &ZSome) -> WoogOption {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = WoogOption {
            ty: ty.id(),
            subtype: WoogOptionEnum::ZSome(subtype.id),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R2(1-*)
    pub fn r2_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
