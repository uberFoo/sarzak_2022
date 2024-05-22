// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"any_list-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-documentation"}}}
/// A list that does not enforce that it’s elements all share a type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-definition"}}}
pub const ANY_LIST: Uuid = uuid!["356fe736-c2f0-5d4e-baa6-8725754ed1d3"];

#[derive(Clone, Debug, Deserialize, Serialize)]
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-implementation"}}}
pub struct AnyList;

impl AnyList {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-struct-impl-new"}}}
    pub fn new() -> Self {
        Self {}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-impl-nav-subtype-to-supertype-value_type"}}}

    pub fn id(&self) -> Uuid {
        ANY_LIST
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"any_list-implementation"}}}

impl Default for AnyList {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
