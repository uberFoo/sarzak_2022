// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::enum_field::EnumField;
use crate::v2::lu_dog_vec::types::enum_field::EnumFieldEnum;
use crate::v2::lu_dog_vec::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-documentation"}}}
/// A field that is a structure.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructField {
    pub id: usize,
    pub name: String,
    /// R89: [`StructField`] 'is composed with a' [`Expression`]
    pub expression: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-implementation"}}}
impl StructField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-impl-new"}}}
    /// Inter a new 'Struct Field' in the store, and return it's `id`.
    pub fn new(
        name: String,
        expression: Option<&Rc<RefCell<Expression>>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<StructField>> {
        store.inter_struct_field(|id| {
            Rc::new(RefCell::new(StructField {
                id,
                name: name.to_owned(),
                expression: expression.map(|expression| expression.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R89(1-*c)
    pub fn r89_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r89_expression");
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(&expression).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-impl-nav-forward-to-ty"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-impl-nav-subtype-to-supertype-enum_field"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-impl-nav-subtype-to-supertype-enum_field"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-impl-nav-subtype-to-supertype-enum_field"}}}
    // Navigate to [`EnumField`] across R85(isa)
    pub fn r85_enum_field<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<EnumField>>> {
        span!("r85_enum_field");
        vec![store
            .iter_enum_field()
            .find(|enum_field| {
                if let EnumFieldEnum::StructField(id) = enum_field.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-implementation"}}}
impl PartialEq for StructField {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.expression == other.expression
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
