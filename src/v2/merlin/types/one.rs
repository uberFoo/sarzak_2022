// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"one-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-const-definition"}}}
pub const ONE: Uuid = uuid!["dae5aac2-064e-5166-96dd-ead71f2c1b4f"];

pub struct One;

impl One {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        ONE
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
