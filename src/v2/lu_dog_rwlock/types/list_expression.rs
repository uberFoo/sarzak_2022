// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"list_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::expression::ExpressionEnum;
use crate::v2::lu_dog_rwlock::types::list_element::ListElement;
use crate::v2::lu_dog_rwlock::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-documentation"}}}
/// A list of expressions
///
/// E.g., `let a = [0, 1, 2, 3];`
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ListExpression {
    pub id: Uuid,
    /// R54: [`ListExpression`] 'contains' [`ListElement`]
    pub elements: Option<Uuid>,
    /// R257: [`ListExpression`] '' [`ValueType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-implementation"}}}
impl ListExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-new"}}}
    /// Inter a new 'List Expression' in the store, and return it's `id`.
    pub fn new(
        elements: Option<&Arc<RwLock<ListElement>>>,
        ty: &Arc<RwLock<ValueType>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<ListExpression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(ListExpression {
            id,
            elements: elements.map(|list_element| list_element.read().unwrap().id),
            ty: ty.read().unwrap().id,
        }));
        store.inter_list_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-nav-forward-cond-to-elements"}}}
    /// Navigate to [`ListElement`] across R54(1-*c)
    pub fn r54_list_element<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<ListElement>>> {
        match self.elements {
            Some(ref elements) => vec![store.exhume_list_element(&elements).unwrap()],
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-nav-backward-one-bi-cond-to-format_string"}}}
            None => Vec::new(),
        }
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-nav-forward-to-ty"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R257(1-*)
    pub fn r257_value_type<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::ListExpression(id) = expression.read().unwrap().subtype {
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
