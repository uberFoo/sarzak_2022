// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::boolean_literal::BooleanLiteral;
use crate::v2::lu_dog_vec_tracy::types::char_literal::CharLiteral;
use crate::v2::lu_dog_vec_tracy::types::expression::Expression;
use crate::v2::lu_dog_vec_tracy::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec_tracy::types::float_literal::FloatLiteral;
use crate::v2::lu_dog_vec_tracy::types::format_string::FormatString;
use crate::v2::lu_dog_vec_tracy::types::integer_literal::IntegerLiteral;
use crate::v2::lu_dog_vec_tracy::types::map_expression::MapExpression;
use crate::v2::lu_dog_vec_tracy::types::string_literal::StringLiteral;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-documentation"}}}
/// A Literal Expression
///
/// This is any literal value in the program.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Literal {
    pub subtype: LiteralEnum,
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum LiteralEnum {
    BooleanLiteral(usize),
    CharLiteral(usize),
    FloatLiteral(usize),
    FormatString(usize),
    IntegerLiteral(usize),
    MapExpression(usize),
    StringLiteral(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-implementation"}}}
impl Literal {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_boolean_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_boolean_literal(
        bogus: bool,
        subtype: &Rc<RefCell<BooleanLiteral>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::BooleanLiteral(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_char_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_char_literal(
        bogus: bool,
        subtype: &Rc<RefCell<CharLiteral>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::CharLiteral(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_float_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_float_literal(
        bogus: bool,
        subtype: &Rc<RefCell<FloatLiteral>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::FloatLiteral(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_format_string"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_format_string(
        bogus: bool,
        subtype: &Rc<RefCell<FormatString>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::FormatString(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_integer_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_integer_literal(
        bogus: bool,
        subtype: &Rc<RefCell<IntegerLiteral>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::IntegerLiteral(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_map_expression"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_map_expression(
        bogus: bool,
        subtype: &Rc<RefCell<MapExpression>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::MapExpression(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_string_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_string_literal(
        bogus: bool,
        subtype: &Rc<RefCell<StringLiteral>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Literal>> {
        store.inter_literal(|id| {
            Rc::new(RefCell::new(Literal {
                bogus: bogus,
                subtype: LiteralEnum::StringLiteral(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Literal(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-implementation"}}}
impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
