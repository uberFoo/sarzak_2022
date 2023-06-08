// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"list_element-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::expression::Expression;
use crate::v2::lu_dog_vanilla::types::list_expression::ListExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ListElement {
    pub id: Uuid,
    /// R55: [`ListElement`] 'points at an' [`Expression`]
    pub expression: Uuid,
    /// R53: [`ListElement`] 'follows' [`ListElement`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-implementation"}}}
impl ListElement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-new"}}}
    /// Inter a new 'List Element' in the store, and return it's `id`.
    pub fn new(
        expression: &Expression,
        next: Option<&ListElement>,
        store: &mut LuDogVanillaStore,
    ) -> ListElement {
        let id = Uuid::new_v4();
        let new = ListElement {
            id,
            expression: expression.id(),
            next: next.map(|list_element| list_element.id),
        };
        store.inter_list_element(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R55(1-*)
    pub fn r55_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`ListElement`] across R53(1-*c)
    pub fn r53_list_element<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ListElement> {
        match self.next {
            Some(ref next) => vec![store.exhume_list_element(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-backward-one-bi-cond-to-list_element"}}}
    /// Navigate to [`ListElement`] across R53(1c-1c)
    pub fn r53c_list_element<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ListElement> {
        let list_element = store
            .iter_list_element()
            .find(|list_element| list_element.next == Some(self.id));
        match list_element {
            Some(ref list_element) => vec![list_element],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-backward-one-to-list_expression"}}}
    /// Navigate to [`ListExpression`] across R54(1-1)
    pub fn r54_list_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ListExpression> {
        vec![store
            .iter_list_expression()
            .find(|list_expression| list_expression.elements == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}