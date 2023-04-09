// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_anchors-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchors-use-statements"}}}
use uuid::Uuid;

use crate::v2::drawing::types::anchor::Anchor;
use crate::v2::drawing::types::isa_ui::IsaUi;
use serde::{Deserialize, Serialize};

use crate::v2::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchors-struct-documentation"}}}
/// Subtype Anchors
///
/// Just as it sounds, these are [`Anchor`]s used by [`Subtype`]s in an [`Isa`] relationship
///.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchors-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SubtypeAnchors {
    pub id: Uuid,
    /// R10: [`Anchor`] '🚧 Out of order — see sarzak#14.' [`Anchor`]
    pub anchor_id: Uuid,
    /// R10: [`IsaUi`] '🚧 Out of order — see sarzak#14.' [`IsaUi`]
    pub isaui_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchors-implementation"}}}
impl SubtypeAnchors {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchors-struct-impl-new"}}}
    /// Inter a new 'Subtype Anchors' in the store, and return it's `id`.
    pub fn new(anchor_id: &Anchor, isaui_id: &IsaUi, store: &mut DrawingStore) -> SubtypeAnchors {
        let id = Uuid::new_v4();
        let new = SubtypeAnchors {
            id: id,
            anchor_id: anchor_id.id,
            isaui_id: isaui_id.id,
        };
        store.inter_subtype_anchors(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchors-struct-impl-nav-forward-assoc-to-isaui_id"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchors-struct-impl-new_"}}}
    /// Inter a new 'Subtype Anchors' in the store, and return it's `id`.
    pub fn new_(anchor_id: &Anchor, isaui_id: &IsaUi) -> SubtypeAnchors {
        let id = Uuid::new_v4();
        let new = SubtypeAnchors {
            id: id,
            anchor_id: anchor_id.id,
            isaui_id: isaui_id.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchors-struct-impl-nav-forward-assoc-to-anchor_id"}}}
    /// Navigate to [`Anchor`] across R10(1-*)
    pub fn r10_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<&Anchor> {
        vec![store.exhume_anchor(&self.anchor_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchors-struct-impl-nav-forward-assoc-to-isaui_id"}}}
    /// Navigate to [`IsaUi`] across R10(1-*)
    pub fn r10_isa_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<&IsaUi> {
        vec![store.exhume_isa_ui(&self.isaui_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
