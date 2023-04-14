// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"inflection-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"inflection-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"inflection-const-definition"}}}
pub const INFLECTION: Uuid = uuid!["15e9aa5c-b0ae-5ca9-8402-d2197984ef98"];

pub struct Inflection;

impl Inflection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        INFLECTION
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
