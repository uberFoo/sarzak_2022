// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_box-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_box-use-statements"}}}
use uuid::Uuid;

use crate::v2::merlin::types::anchor::Anchor;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_box-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XBox {
    pub height: i64,
    pub id: Uuid,
    pub width: i64,
    pub x: i64,
    pub y: i64,
    /// R1: [`XBox`] 'renders an' [`Object`]
    pub object: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_box-implementation"}}}
impl XBox {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_box-struct-impl-new"}}}
    /// Inter a new 'Box' in the store, and return it's `id`.
    pub fn new(
        height: i64,
        width: i64,
        x: i64,
        y: i64,
        object: &Object,
        store: &mut MerlinStore,
    ) -> XBox {
        let id = Uuid::new_v4();
        let new = XBox {
            height: height,
            id: id,
            width: width,
            x: x,
            y: y,
            object: object.id,
        };
        store.inter_x_box(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_box-struct-impl-new_"}}}
    /// Inter a new 'Box' in the store, and return it's `id`.
    pub fn new_(height: i64, width: i64, x: i64, y: i64, object: &Object) -> XBox {
        let id = Uuid::new_v4();
        let new = XBox {
            height: height,
            id: id,
            width: width,
            x: x,
            y: y,
            object: object.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_box-struct-impl-nav-forward-to-object"}}}
    /// Navigate to [`Object`] across R1(1-*)
    pub fn r1_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.object).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_box-struct-impl-nav-backward-assoc_many-to-anchor"}}}
    /// Navigate to [`Anchor`] across R3(1-M)
    pub fn r3_anchor<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Anchor> {
        store
            .iter_anchor()
            .filter_map(|anchor| {
                if anchor.x_box == self.id {
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
