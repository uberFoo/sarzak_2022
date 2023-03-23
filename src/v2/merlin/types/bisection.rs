// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"bisection-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bisection-use-statements"}}}
use uuid::Uuid;

use crate::v2::merlin::types::line_segment::LineSegment;
use crate::v2::merlin::types::point::Point;
use crate::v2::merlin::types::relationship_name::RelationshipName;
use crate::v2::merlin::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bisection-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Bisection {
    pub id: Uuid,
    pub offset: f64,
    /// R14: [`Bisection`] 'exists on a' [`LineSegment`]
    pub segment: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bisection-implementation"}}}
impl Bisection {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bisection-struct-impl-new"}}}
    /// Inter a new 'Bisection' in the store, and return it's `id`.
    pub fn new(offset: f64, segment: &LineSegment, store: &mut MerlinStore) -> Bisection {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", offset, segment).as_bytes());
        let new = Bisection {
            id: id,
            offset: offset,
            segment: segment.id,
        };
        store.inter_bisection(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bisection-struct-impl-nav-forward-to-segment"}}}
    /// Navigate to [`LineSegment`] across R14(1-*)
    pub fn r14_line_segment<'a>(&'a self, store: &'a MerlinStore) -> Vec<&LineSegment> {
        vec![store.exhume_line_segment(&self.segment).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bisection-struct-impl-nav-backward-cond-to-relationship_name"}}}
    /// Navigate to [`RelationshipName`] across R15(1-1c)
    pub fn r15c_relationship_name<'a>(&'a self, store: &'a MerlinStore) -> Vec<&RelationshipName> {
        let relationship_name = store
            .iter_relationship_name()
            .find(|relationship_name| relationship_name.origin == self.id);
        match relationship_name {
            Some(ref relationship_name) => vec![relationship_name],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"bisection-impl-nav-subtype-to-supertype-point"}}}
    // Navigate to [`Point`] across R6(isa)
    pub fn r6_point<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Point> {
        vec![store.exhume_point(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}