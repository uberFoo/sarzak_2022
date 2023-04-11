// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::implementation::Implementation;
use crate::v2::lu_dog::types::item::Item;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-documentation"}}}
/// A Function
///
/// Inputs, outputs. Stuff happens.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Function {
    pub id: Uuid,
    pub name: String,
    /// R9: [`Function`] 'may be contained in an' [`Implementation`]
    pub impl_block: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-implementation"}}}
impl Function {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new"}}}
    /// Inter a new 'Function' in the store, and return it's `id`.
    pub fn new(
        name: String,
        impl_block: Option<&Implementation>,
        store: &mut LuDogStore,
    ) -> Function {
        let id = Uuid::new_v4();
        let new = Function {
            id: id,
            name: name,
            impl_block: impl_block.map(|implementation| implementation.id),
        };
        store.inter_function(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new_"}}}
    /// Inter a new 'Function' in the store, and return it's `id`.
    pub fn new_(name: String, impl_block: Option<&Implementation>) -> Function {
        let id = Uuid::new_v4();
        let new = Function {
            id: id,
            name: name,
            impl_block: impl_block.map(|implementation| implementation.id),
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-impl_block"}}}
    /// Navigate to [`Implementation`] across R9(1-*c)
    pub fn r9_implementation<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Implementation> {
        match self.impl_block {
            Some(ref impl_block) => vec![store.exhume_implementation(impl_block).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Item> {
        vec![store.exhume_item(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
