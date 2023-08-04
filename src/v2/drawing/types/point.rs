// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"point-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::drawing::types::anchor::Anchor;
use crate::v2::drawing::types::associative_ui::AssociativeUi;
use crate::v2::drawing::types::object_ui::ObjectUi;
use serde::{Deserialize, Serialize};

use crate::v2::drawing::store::ObjectStore as DrawingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-documentation"}}}
/// A point is a two-tuple that represents a location on the drawing canvas.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Point {
    pub id: Uuid,
    pub x: i64,
    pub y: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-implementation"}}}
impl Point {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new"}}}
    /// Inter a new 'Point' in the store, and return it's `id`.
    pub fn new(x: i64, y: i64, store: &mut DrawingStore) -> Rc<RefCell<Point>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Point { id, x, y }));
        store.inter_point(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-new_"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-backward-cond-to-anchor"}}}
    /// Navigate to [`Anchor`] across R4(1-1c)
    pub fn r4c_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<Rc<RefCell<Anchor>>> {
        span!("r4_anchor");
        let anchor = store
            .iter_anchor()
            .find(|anchor| anchor.borrow().location == self.id);
        match anchor {
            Some(ref anchor) => vec![anchor.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-backward-cond-to-anchor"}}}
    /// Navigate to [`Anchor`] across R5(1-1c)
    pub fn r5c_anchor<'a>(&'a self, store: &'a DrawingStore) -> Vec<Rc<RefCell<Anchor>>> {
        span!("r5_anchor");
        let anchor = store
            .iter_anchor()
            .find(|anchor| anchor.borrow().offset == self.id);
        match anchor {
            Some(ref anchor) => vec![anchor.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-backward-cond-to-associative_ui"}}}
    /// Navigate to [`AssociativeUi`] across R17(1-1c)
    pub fn r17c_associative_ui<'a>(
        &'a self,
        store: &'a DrawingStore,
    ) -> Vec<Rc<RefCell<AssociativeUi>>> {
        span!("r17_associative_ui");
        let associative_ui = store
            .iter_associative_ui()
            .find(|associative_ui| associative_ui.borrow().from == self.id);
        match associative_ui {
            Some(ref associative_ui) => vec![associative_ui.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"point-struct-impl-nav-backward-cond-to-object_ui"}}}
    /// Navigate to [`ObjectUi`] across R13(1-1c)
    pub fn r13c_object_ui<'a>(&'a self, store: &'a DrawingStore) -> Vec<Rc<RefCell<ObjectUi>>> {
        span!("r13_object_ui");
        let object_ui = store
            .iter_object_ui()
            .find(|object_ui| object_ui.borrow().origin == self.id);
        match object_ui {
            Some(ref object_ui) => vec![object_ui.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
