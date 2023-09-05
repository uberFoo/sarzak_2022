// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"implementation_block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::item::Item;
use crate::v2::lu_dog::types::item::ItemEnum;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-documentation"}}}
/// An Implementation Block
///
/// Inside this block functions are defined on a [`ModellType`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ImplementationBlock {
    pub id: Uuid,
    /// R8: [`ImplementationBlock`] 'adds functions to a' [`WoogStruct`]
    pub model_type: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-implementation"}}}
impl ImplementationBlock {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-new"}}}
    /// Inter a new 'Implementation Block' in the store, and return it's `id`.
    pub fn new(
        model_type: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ImplementationBlock>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ImplementationBlock {
            id,
            model_type: model_type.borrow().id,
        }));
        store.inter_implementation_block(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-forward-to-model_type"}}}
    /// Navigate to [`WoogStruct`] across R8(1-*)
    pub fn r8_woog_struct<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<WoogStruct>>> {
        span!("r8_woog_struct");
        vec![store.exhume_woog_struct(&self.model_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-struct-impl-nav-backward-1_Mc-to-function"}}}
    /// Navigate to [`Function`] across R9(1-Mc)
    pub fn r9_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Function>>> {
        span!("r9_function");
        store
            .iter_function()
            .filter(|function| function.borrow().impl_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"implementation_block-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Item>>> {
        span!("r6_item");
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::ImplementationBlock(id) = item.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}