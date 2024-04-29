// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"map_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-const-documentation"}}}
/// A hashmap expression
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"map_expression-const-definition"}}}
pub const MAP_EXPRESSION: Uuid = uuid!["230d0ba5-5173-5ef2-bb2c-29bccf79b3a0"];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapExpression;

impl MapExpression {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        MAP_EXPRESSION
    }
}

impl Default for MapExpression {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
