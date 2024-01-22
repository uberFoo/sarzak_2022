// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::value_type::ValueType;
use crate::v2::lu_dog_vec_tracy::types::value_type::ValueTypeEnum;
use crate::v2::lu_dog_vec_tracy::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-struct-documentation"}}}
/// A generic type applied to the struct.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructGeneric {
    pub id: usize,
    pub name: String,
    /// R101: [`StructGeneric`] '' [`StructGeneric`]
    pub next: Option<usize>,
    /// R100: [`StructGeneric`] 'applies to a' [`WoogStruct`]
    pub woog_struct: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-implementation"}}}
impl StructGeneric {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-struct-impl-new"}}}
    /// Inter a new 'Struct Generic' in the store, and return it's `id`.
    pub fn new(
        name: String,
        next: Option<&Rc<RefCell<StructGeneric>>>,
        woog_struct: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<StructGeneric>> {
        store.inter_struct_generic(|id| {
            Rc::new(RefCell::new(StructGeneric {
                id,
                name: name.to_owned(),
                next: next.map(|struct_generic| struct_generic.borrow().id),
                woog_struct: woog_struct.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`StructGeneric`] across R101(1-*c)
    pub fn r101_struct_generic<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<StructGeneric>>> {
        span!("r101_struct_generic");
        match self.next {
            Some(ref next) => vec![store.exhume_struct_generic(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`WoogStruct`] across R100(1-*)
    pub fn r100_woog_struct<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<WoogStruct>>> {
        span!("r100_woog_struct");
        vec![store.exhume_woog_struct(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-struct-impl-nav-backward-one-to-woog_struct"}}}
    /// Navigate to [`WoogStruct`] across R102(1-1)
    pub fn r102_woog_struct<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<WoogStruct>>> {
        span!("r102_woog_struct");
        vec![store
            .iter_woog_struct()
            .find(|woog_struct| woog_struct.borrow().first_generic == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-struct-impl-nav-backward-one-bi-cond-to-struct_generic"}}}
    /// Navigate to [`StructGeneric`] across R101(1c-1c)
    pub fn r101c_struct_generic<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<StructGeneric>>> {
        span!("r101_struct_generic");
        let struct_generic = store
            .iter_struct_generic()
            .find(|struct_generic| struct_generic.borrow().next == Some(self.id));
        match struct_generic {
            Some(ref struct_generic) => vec![struct_generic.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::StructGeneric(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_generic-implementation"}}}
impl PartialEq for StructGeneric {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.next == other.next && self.woog_struct == other.woog_struct
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
