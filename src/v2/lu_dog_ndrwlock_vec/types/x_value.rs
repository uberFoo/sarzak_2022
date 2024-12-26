// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_value-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::block::Block;
use crate::v2::lu_dog_ndrwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_ndrwlock_vec::types::span::Span;
use crate::v2::lu_dog_ndrwlock_vec::types::value_type::ValueType;
use crate::v2::lu_dog_ndrwlock_vec::types::variable::Variable;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-documentation"}}}
/// A Value
///
/// A value has a Type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XValue {
    pub subtype: XValueEnum,
    pub id: usize,
    /// R33: [`XValue`] '' [`Block`]
    pub block: usize,
    /// R258: [`XValue`] '' [`XValue`]
    pub next: Option<usize>,
    /// R24: [`XValue`] 'is decoded by a' [`ValueType`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum XValueEnum {
    Expression(usize),
    Variable(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-implementation"}}}
impl XValue {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_expression"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub fn new_expression(
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<XValue>>>,
        ty: &Arc<RwLock<ValueType>>,
        subtype: &Arc<RwLock<Expression>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<XValue>> {
        store.inter_x_value(|id| {
            Arc::new(RwLock::new(XValue {
                block: block.read().unwrap().id,
                next: next.map(|x_value| x_value.read().unwrap().id),
                ty: ty.read().unwrap().id,
                subtype: XValueEnum::Expression(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_variable"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub fn new_variable(
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<XValue>>>,
        ty: &Arc<RwLock<ValueType>>,
        subtype: &Arc<RwLock<Variable>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<XValue>> {
        store.inter_x_value(|id| {
            Arc::new(RwLock::new(XValue {
                block: block.read().unwrap().id,
                next: next.map(|x_value| x_value.read().unwrap().id),
                ty: ty.read().unwrap().id,
                subtype: XValueEnum::Variable(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R33(1-*)
    pub fn r33_block<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Block>>> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`XValue`] across R258(1-*c)
    pub fn r258_x_value<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<XValue>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_x_value(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R24(1-*)
    pub fn r24_value_type<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-backward-1_M-to-z_some"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R63(1-Mc)
    pub fn r63_span<'a>(&'a self, store: &'a LuDogNdrwlockVecStore) -> Vec<Arc<RwLock<Span>>> {
        store
            .iter_span()
            .filter(|span| span.read().unwrap().x_value == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-backward-one-bi-cond-to-x_value"}}}
    /// Navigate to [`XValue`] across R258(1c-1c)
    pub fn r258c_x_value<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<XValue>>> {
        let x_value = store
            .iter_x_value()
            .find(|x_value| x_value.read().unwrap().next == Some(self.id));
        match x_value {
            Some(ref x_value) => vec![x_value.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-implementation"}}}
impl PartialEq for XValue {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
            && self.block == other.block
            && self.next == other.next
            && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
