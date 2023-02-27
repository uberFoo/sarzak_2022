// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"grace_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

// Subtype imports
use crate::v2::sarzak::types::s_type::SType;
use crate::v2::woog::types::reference::Reference;
use crate::v2::woog::types::woog_option::WoogOption;

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-enum-documentation"}}}
/// Grace Model Compiler Type
///
/// The model compiler domain contains at least one type that doesn't make sense within the
/// modeling domain. That type is an object reference. References, in my mind, have no place
/// in a modeling domain.
///
/// So that's what this is about.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum GraceType {
    WoogOption(Uuid),
    Reference(Uuid),
    SType(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-implementation"}}}
impl GraceType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-new-impl"}}}
    /// Create a new instance of GraceType::WoogOption
    pub fn new_woog_option(woog_option: &WoogOption, store: &mut WoogStore) -> Self {
        let new = Self::WoogOption(woog_option.id);
        store.inter_grace_type(new.clone());
        new
    }

    /// Create a new instance of GraceType::Reference
    pub fn new_reference(reference: &Reference, store: &mut WoogStore) -> Self {
        let new = Self::Reference(reference.id);
        store.inter_grace_type(new.clone());
        new
    }

    /// Create a new instance of GraceType::SType
    pub fn new_s_type(s_type: &SType, store: &mut WoogStore) -> Self {
        let new = Self::SType(s_type.id());
        store.inter_grace_type(new.clone());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            GraceType::WoogOption(id) => *id,
            GraceType::Reference(id) => *id,
            GraceType::SType(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

impl From<SType> for GraceType {
    fn from(ty: SType) -> Self {
        GraceType::SType(ty.id())
    }
}

impl From<&SType> for GraceType {
    fn from(ty: &SType) -> Self {
        GraceType::SType(ty.id())
    }
}

// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
