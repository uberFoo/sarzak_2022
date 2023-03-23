// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"relationship_name-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_name-use-statements"}}}
use uuid::Uuid;

use crate::v2::merlin::types::bisection::Bisection;
use crate::v2::merlin::types::line::Line;
use crate::v2::merlin::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_name-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RelationshipName {
    pub id: Uuid,
    pub text: String,
    pub x: i64,
    pub y: i64,
    /// R15: [`RelationshipName`] 'is anchored by' [`Bisection`]
    pub origin: Uuid,
    /// R11: [`RelationshipName`] 'is derived from' [`Line`]
    pub line: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_name-implementation"}}}
impl RelationshipName {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_name-struct-impl-new"}}}
    /// Inter a new 'Relationship Name' in the store, and return it's `id`.
    pub fn new(
        text: String,
        x: i64,
        y: i64,
        origin: &Bisection,
        line: &Line,
        store: &mut MerlinStore,
    ) -> RelationshipName {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{}:{}:{:?}:{:?}", text, x, y, origin, line).as_bytes(),
        );
        let new = RelationshipName {
            id: id,
            text: text,
            x: x,
            y: y,
            origin: origin.id,
            line: line.id,
        };
        store.inter_relationship_name(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_name-struct-impl-nav-forward-to-origin"}}}
    /// Navigate to [`Bisection`] across R15(1-*)
    pub fn r15_bisection<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Bisection> {
        vec![store.exhume_bisection(&self.origin).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship_name-struct-impl-nav-forward-to-line"}}}
    /// Navigate to [`Line`] across R11(1-*)
    pub fn r11_line<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Line> {
        vec![store.exhume_line(&self.line).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
