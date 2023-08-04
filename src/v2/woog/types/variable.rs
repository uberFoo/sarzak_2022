// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"variable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::woog::types::local::Local;
use crate::v2::woog::types::parameter::Parameter;
use crate::v2::woog::types::symbol_table::SymbolTable;
use crate::v2::woog::types::x_let::XLet;
use crate::v2::woog::types::x_value::XValue;
use crate::v2::woog::types::x_value::XValueEnum;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-documentation"}}}
/// A Variable
///
/// Basically a name given to some memory.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Variable {
    pub subtype: VariableEnum,
    pub id: Uuid,
    pub name: String,
    /// R20: [`Variable`] 'exists in a' [`SymbolTable`]
    pub symbol_table: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VariableEnum {
    Local(Uuid),
    Parameter(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-implementation"}}}
impl Variable {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_local"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_local(
        name: String,
        symbol_table: &Rc<RefCell<SymbolTable>>,
        subtype: &Rc<RefCell<Local>>,
        store: &mut WoogStore,
    ) -> Rc<RefCell<Variable>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Variable {
            name: name,
            symbol_table: symbol_table.borrow().id,
            subtype: VariableEnum::Local(subtype.borrow().id),
            id,
        }));
        store.inter_variable(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-new_parameter"}}}
    /// Inter a new Variable in the store, and return it's `id`.
    pub fn new_parameter(
        name: String,
        symbol_table: &Rc<RefCell<SymbolTable>>,
        subtype: &Rc<RefCell<Parameter>>,
        store: &mut WoogStore,
    ) -> Rc<RefCell<Variable>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Variable {
            name: name,
            symbol_table: symbol_table.borrow().id,
            subtype: VariableEnum::Parameter(subtype.borrow().id),
            id,
        }));
        store.inter_variable(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-nav-forward-to-symbol_table"}}}
    /// Navigate to [`SymbolTable`] across R20(1-*)
    pub fn r20_symbol_table<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<SymbolTable>>> {
        span!("r20_symbol_table");
        vec![store.exhume_symbol_table(&self.symbol_table).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-struct-impl-nav-backward-1_M-to-x_let"}}}
    /// Navigate to [`XLet`] across R17(1-M)
    pub fn r17_x_let<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<XLet>>> {
        span!("r17_x_let");
        store
            .iter_x_let()
            .filter(|x_let| x_let.borrow().variable == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-impl-nav-subtype-to-supertype-value"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"variable-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R7(isa)
    pub fn r7_x_value<'a>(&'a self, store: &'a WoogStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r7_x_value");
        vec![store
            .iter_x_value()
            .find(|x_value| {
                if let XValueEnum::Variable(id) = x_value.borrow().subtype {
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
