// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::sarzak_single::types::binary::Binary;
use crate::v2::sarzak_single::types::cardinality::Cardinality;
use crate::v2::sarzak_single::types::conditionality::Conditionality;
use crate::v2::sarzak_single::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::sarzak_single::store::ObjectStore as SarzakSingleStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-documentation"}}}
/// This is the side being referred to in a binary relationship. It is the “to” side.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Referent {
    pub description: String,
    pub id: Uuid,
    /// R8: [`Referent`] 'has' [`Cardinality`]
    pub cardinality: Uuid,
    /// R12: [`Referent`] 'has' [`Conditionality`]
    pub conditionality: Uuid,
    /// R16: [`Referent`] 'is an instance of an' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-implementation"}}}
impl Referent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-new"}}}
    /// Inter a new 'Referent' in the store, and return it's `id`.
    pub fn new(
        description: String,
        cardinality: &Rc<RefCell<Cardinality>>,
        conditionality: &Rc<RefCell<Conditionality>>,
        obj_id: &Rc<RefCell<Object>>,
        store: &mut SarzakSingleStore,
    ) -> Rc<RefCell<Referent>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Referent {
            description,
            id,
            cardinality: cardinality.borrow().id(),
            conditionality: conditionality.borrow().id(),
            obj_id: obj_id.borrow().id,
        }));
        store.inter_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-forward-to-cardinality"}}}
    /// Navigate to [`Cardinality`] across R8(1-*)
    pub fn r8_cardinality<'a>(
        &'a self,
        store: &'a SarzakSingleStore,
    ) -> Vec<Rc<RefCell<Cardinality>>> {
        vec![store.exhume_cardinality(&self.cardinality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-forward-to-conditionality"}}}
    /// Navigate to [`Conditionality`] across R12(1-*)
    pub fn r12_conditionality<'a>(
        &'a self,
        store: &'a SarzakSingleStore,
    ) -> Vec<Rc<RefCell<Conditionality>>> {
        vec![store.exhume_conditionality(&self.conditionality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R16(1-*)
    pub fn r16_object<'a>(&'a self, store: &'a SarzakSingleStore) -> Vec<Rc<RefCell<Object>>> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-one-to-binary"}}}
    /// Navigate to [`Binary`] across R5(1-1)
    pub fn r5_binary<'a>(&'a self, store: &'a SarzakSingleStore) -> Vec<Rc<RefCell<Binary>>> {
        vec![store
            .iter_binary()
            .find(|binary| binary.borrow().to == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
