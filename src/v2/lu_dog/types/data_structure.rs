// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"data_structure-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::enumeration::Enumeration;
use crate::v2::lu_dog::types::struct_expression::StructExpression;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DataStructure {
    Enumeration(Uuid),
    WoogStruct(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-implementation"}}}
impl DataStructure {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-new-impl"}}}
    /// Create a new instance of DataStructure::Enumeration
    pub fn new_enumeration(
        enumeration: &Rc<RefCell<Enumeration>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = enumeration.borrow().id;
        if let Some(enumeration) = store.exhume_data_structure(&id) {
            enumeration
        } else {
            let new = Rc::new(RefCell::new(Self::Enumeration(id)));
            store.inter_data_structure(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of DataStructure::WoogStruct
    pub fn new_woog_struct(
        woog_struct: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = woog_struct.borrow().id;
        if let Some(woog_struct) = store.exhume_data_structure(&id) {
            woog_struct
        } else {
            let new = Rc::new(RefCell::new(Self::WoogStruct(id)));
            store.inter_data_structure(new.clone());
            new
        }
    } // wtf?

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Enumeration(id) => *id,
            Self::WoogStruct(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"data_structure-struct-impl-nav-backward-1_M-to-struct_expression"}}}
    /// Navigate to [`StructExpression`] across R39(1-M)
    pub fn r39_struct_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<StructExpression>>> {
        store
            .iter_struct_expression()
            .filter(|struct_expression| struct_expression.borrow().data == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
