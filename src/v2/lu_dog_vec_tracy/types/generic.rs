// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::value_type::ValueType;
use crate::v2::lu_dog_vec_tracy::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-struct-documentation"}}}
/// This is a generic “type”.
///
/// It’s really a placeholder in the extruder/compiler. We’ll use it as a type declaration
/// , and then define a new type for each use.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Generic {
    pub id: usize,
    pub name: String,
    /// R3: [`Generic`] 'next' [`Generic`]
    pub next: Option<usize>,
    /// R99: [`Generic`] 'has an inner' [`ValueType`]
    pub ty: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-implementation"}}}
impl Generic {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-struct-impl-new"}}}
    /// Inter a new 'Generic' in the store, and return it's `id`.
    pub fn new(
        name: String,
        next: Option<&Rc<RefCell<Generic>>>,
        ty: Option<&Rc<RefCell<ValueType>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Generic>> {
        store.inter_generic(|id| {
            Rc::new(RefCell::new(Generic {
                id,
                name: name.to_owned(),
                next: next.map(|generic| generic.borrow().id),
                ty: ty.map(|value_type| value_type.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Generic`] across R3(1-*c)
    pub fn r3_generic<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Generic>>> {
        span!("r3_generic");
        match self.next {
            Some(ref next) => vec![store.exhume_generic(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`ValueType`] across R99(1-*c)
    pub fn r99_value_type<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r99_value_type");
        match self.ty {
            Some(ref ty) => vec![store.exhume_value_type(&ty).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-struct-impl-nav-backward-one-bi-cond-to-generic"}}}
    /// Navigate to [`Generic`] across R3(1c-1c)
    pub fn r3c_generic<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<Generic>>> {
        span!("r3_generic");
        let generic = store
            .iter_generic()
            .find(|generic| generic.borrow().next == Some(self.id));
        match generic {
            Some(ref generic) => vec![generic.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r1_value_type");
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Generic(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"generic-implementation"}}}
impl PartialEq for Generic {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.next == other.next && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}