// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"line-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-use-statements"}}}
use uuid::Uuid;

use crate::v2::merlin::types::anchor::Anchor;
use crate::v2::merlin::types::glyph::Glyph;
use crate::v2::merlin::types::line_segment::LineSegment;
use crate::v2::merlin::types::relationship_name::RelationshipName;
use crate::v2::merlin::types::relationship_phrase::RelationshipPhrase;
use crate::v2::sarzak::types::relationship::Relationship;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-documentation"}}}
/// Relationship Line
///
/// A line is how we represent a relationship. A line is composed of many [`Line Segment`]-
/// s.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Line {
    pub id: Uuid,
    /// R2: [`Line`] 'represents a' [`Relationship`]
    pub relationship: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-implementation"}}}
impl Line {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-impl-new"}}}
    /// Inter a new 'Line' in the store, and return it's `id`.
    pub fn new(relationship: &Relationship, store: &mut MerlinStore) -> Line {
        let id = Uuid::new_v4();
        let new = Line {
            id: id,
            relationship: relationship.id(),
        };
        store.inter_line(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-impl-new_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-impl-nav-forward-to-relationship"}}}
    /// Navigate to [`Relationship`] across R2(1-*)
    pub fn r2_relationship<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Relationship> {
        vec![store.exhume_relationship(&self.relationship).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-impl-nav-backward-cond-to-glyph"}}}
    /// Navigate to [`Glyph`] across R16(1-1c)
    pub fn r16c_glyph<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Glyph> {
        let glyph = store.iter_glyph().find(|glyph| glyph.line == self.id);
        match glyph {
            Some(ref glyph) => vec![glyph],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-impl-nav-backward-one-to-line_segment"}}}
    /// Navigate to [`LineSegment`] across R4(1-1)
    pub fn r4_line_segment<'a>(&'a self, store: &'a MerlinStore) -> Vec<&LineSegment> {
        vec![store
            .iter_line_segment()
            .find(|line_segment| line_segment.line == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-impl-nav-backward-one-to-relationship_name"}}}
    /// Navigate to [`RelationshipName`] across R11(1-1)
    pub fn r11_relationship_name<'a>(&'a self, store: &'a MerlinStore) -> Vec<&RelationshipName> {
        vec![store
            .iter_relationship_name()
            .find(|relationship_name| relationship_name.line == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-impl-nav-backward-1_M-to-relationship_phrase"}}}
    /// Navigate to [`RelationshipPhrase`] across R12(1-M)
    pub fn r12_relationship_phrase<'a>(
        &'a self,
        store: &'a MerlinStore,
    ) -> Vec<&RelationshipPhrase> {
        store
            .iter_relationship_phrase()
            .filter_map(|relationship_phrase| {
                if relationship_phrase.line == self.id {
                    Some(relationship_phrase)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line-struct-impl-nav-backward-assoc_many-to-anchor"}}}
    /// Navigate to [`Anchor`] across R3(1-M)
    pub fn r3_anchor<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Anchor> {
        store
            .iter_anchor()
            .filter_map(|anchor| {
                if anchor.line == self.id {
                    Some(anchor)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
