// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::enum_generic_type::EnumGenericType;
use crate::v2::lu_dog_vec::types::enumeration::Enumeration;
use crate::v2::lu_dog_vec::types::value_type::ValueType;
use crate::v2::lu_dog_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumGeneric {
    pub goober: Uuid,
    pub id: usize,
    pub name: String,
    /// R104: [`EnumGeneric`] 'parameterizes' [`Enumeration`]
    pub woog_enum: usize,
    /// R106: [`EnumGeneric`] 'next' [`EnumGeneric`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-implementation"}}}
impl EnumGeneric {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-new"}}}
    /// Inter a new 'Enum Generic' in the store, and return it's `id`.
    pub fn new(
        goober: Uuid,
        name: String,
        woog_enum: &Rc<RefCell<Enumeration>>,
        next: Option<&Rc<RefCell<EnumGeneric>>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<EnumGeneric>> {
        store.inter_enum_generic(|id| {
            Rc::new(RefCell::new(EnumGeneric {
                goober,
                id,
                name: name.to_owned(),
                woog_enum: woog_enum.borrow().id,
                next: next.map(|enum_generic| enum_generic.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-to-is_paramaterized_by"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R104(1-*)
    pub fn r104_enumeration<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<Enumeration>>> {
        vec![store.exhume_enumeration(&self.woog_enum).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`EnumGeneric`] across R106(1-*c)
    pub fn r106_enum_generic<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<EnumGeneric>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_enum_generic(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-bi-cond-to-enum_generic"}}}
    /// Navigate to [`EnumGeneric`] across R106(1c-1c)
    pub fn r106c_enum_generic<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<EnumGeneric>>> {
        let enum_generic = store
            .iter_enum_generic()
            .find(|enum_generic| enum_generic.borrow().next == Some(self.id));
        match enum_generic {
            Some(ref enum_generic) => vec![enum_generic.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-to-enumeration"}}}
    /// Navigate to [`Enumeration`] across R105(1-1)
    pub fn r105_enumeration<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<Enumeration>>> {
        vec![store
            .iter_enumeration()
            .find(|enumeration| enumeration.borrow().first_generic == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-assoc-many-to-enum_generic_type"}}}
    /// Navigate to [`EnumGenericType`] across R120(1-M)
    pub fn r120_enum_generic_type<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<EnumGenericType>>> {
        store
            .iter_enum_generic_type()
            .filter(|enum_generic_type| enum_generic_type.borrow().generic == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::EnumGeneric(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-implementation"}}}
impl PartialEq for EnumGeneric {
    fn eq(&self, other: &Self) -> bool {
        self.goober == other.goober
            && self.name == other.name
            && self.woog_enum == other.woog_enum
            && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
