// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"lambda-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::body::Body;
use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-documentation"}}}
/// Lambda Function
///
/// It’s a function, it has a type, parameters, etc. It does not have a name, which is problematic
///  with Function having one. It’s also an Expression, unlike a Function.
///
/// I should think about creating another function subtype that contains just the name...
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Lambda {
    pub id: Uuid,
    /// R73: [`Lambda`] 'contains a' [`Body`]
    pub body: Option<Uuid>,
    /// R103: [`Lambda`] 'may have a' [`LambdaParameter`]
    pub first_param: Option<Uuid>,
    /// R74: [`Lambda`] 'has a' [`ValueType`]
    pub return_type: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-implementation"}}}
impl Lambda {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-new"}}}
    /// Inter a new 'Lambda' in the store, and return it's `id`.
    pub fn new(
        body: Option<&Rc<RefCell<Body>>>,
        first_param: Option<&Rc<RefCell<LambdaParameter>>>,
        return_type: &Rc<RefCell<ValueType>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Lambda>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Lambda {
            id,
            body: body.map(|body| body.borrow().id),
            first_param: first_param.map(|lambda_parameter| lambda_parameter.borrow().id),
            return_type: return_type.borrow().id(),
        }));
        store.inter_lambda(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-cond-to-block"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-cond-to-body"}}}
    /// Navigate to [`Body`] across R73(1-*c)
    pub fn r73_body<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Body>>> {
        match self.body {
            Some(ref body) => vec![store.exhume_body(&body).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-cond-to-first_param"}}}
    /// Navigate to [`LambdaParameter`] across R103(1-*c)
    pub fn r103_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<LambdaParameter>>> {
        match self.first_param {
            Some(ref first_param) => vec![store.exhume_lambda_parameter(&first_param).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-forward-to-return_type"}}}
    /// Navigate to [`ValueType`] across R74(1-*)
    pub fn r74_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store.exhume_value_type(&self.return_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-struct-impl-nav-backward-1_M-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R76(1-M)
    pub fn r76_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<LambdaParameter>>> {
        store
            .iter_lambda_parameter()
            .filter(|lambda_parameter| lambda_parameter.borrow().lambda == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"lambda-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store.exhume_value_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
