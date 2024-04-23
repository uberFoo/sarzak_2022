// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"halt_and_catch_fire-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-const-documentation"}}}
/// Goes boom!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"halt_and_catch_fire-const-definition"}}}
pub const HALT_AND_CATCH_FIRE: Uuid = uuid!["2da94414-ad10-500c-8960-98cc9bf0e599"];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HaltAndCatchFire;

impl HaltAndCatchFire {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        HALT_AND_CATCH_FIRE
    }
}

impl Default for HaltAndCatchFire {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
