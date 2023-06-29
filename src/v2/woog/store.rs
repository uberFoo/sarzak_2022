//! v2::woog Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Access`]
//! * [`Block`]
//! * [`Call`]
//! * [`Constant`]
//! * [`Enumeration`]
//! * [`EnumerationField`]
//! * [`Expression`]
//! * [`Field`]
//! * [`Function`]
//! * [`GenerationUnit`]
//! * [`GraceType`]
//! * [`Item`]
//! * [`XLet`]
//! * [`Local`]
//! * [`ObjectMethod`]
//! * [`WoogOption`]
//! * [`Ownership`]
//! * [`Parameter`]
//! * [`Reference`]
//! * [`Statement`]
//! * [`Structure`]
//! * [`StructureField`]
//! * [`SymbolTable`]
//! * [`TimeStamp`]
//! * [`XValue`]
//! * [`Variable`]
//! * [`Visibility`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog::types::{
    Access, Block, Call, Constant, Enumeration, EnumerationField, Expression, Field, Function,
    GenerationUnit, GraceType, Item, Local, ObjectMethod, Ownership, Parameter, Reference,
    Statement, Structure, StructureField, SymbolTable, TimeStamp, Variable, Visibility, WoogOption,
    XLet, XValue, BORROWED, IMPLEMENTATION, KRATE, LITERAL, MUTABLE, OWNED, PRIVATE, PUBLIC, USIZE,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    access: HashMap<Uuid, (Access, SystemTime)>,
    block: HashMap<Uuid, (Block, SystemTime)>,
    call: HashMap<Uuid, (Call, SystemTime)>,
    constant: HashMap<Uuid, (Constant, SystemTime)>,
    enumeration: HashMap<Uuid, (Enumeration, SystemTime)>,
    enumeration_field: HashMap<Uuid, (EnumerationField, SystemTime)>,
    expression: HashMap<Uuid, (Expression, SystemTime)>,
    field: HashMap<Uuid, (Field, SystemTime)>,
    field_id_by_name: HashMap<String, (Uuid, SystemTime)>,
    function: HashMap<Uuid, (Function, SystemTime)>,
    function_id_by_name: HashMap<String, (Uuid, SystemTime)>,
    generation_unit: HashMap<Uuid, (GenerationUnit, SystemTime)>,
    grace_type: HashMap<Uuid, (GraceType, SystemTime)>,
    item: HashMap<Uuid, (Item, SystemTime)>,
    x_let: HashMap<Uuid, (XLet, SystemTime)>,
    local: HashMap<Uuid, (Local, SystemTime)>,
    object_method: HashMap<Uuid, (ObjectMethod, SystemTime)>,
    woog_option: HashMap<Uuid, (WoogOption, SystemTime)>,
    ownership: HashMap<Uuid, (Ownership, SystemTime)>,
    parameter: HashMap<Uuid, (Parameter, SystemTime)>,
    reference: HashMap<Uuid, (Reference, SystemTime)>,
    statement: HashMap<Uuid, (Statement, SystemTime)>,
    structure: HashMap<Uuid, (Structure, SystemTime)>,
    structure_field: HashMap<Uuid, (StructureField, SystemTime)>,
    symbol_table: HashMap<Uuid, (SymbolTable, SystemTime)>,
    time_stamp: HashMap<Uuid, (TimeStamp, SystemTime)>,
    x_value: HashMap<Uuid, (XValue, SystemTime)>,
    variable: HashMap<Uuid, (Variable, SystemTime)>,
    visibility: HashMap<Uuid, (Visibility, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            access: HashMap::default(),
            block: HashMap::default(),
            call: HashMap::default(),
            constant: HashMap::default(),
            enumeration: HashMap::default(),
            enumeration_field: HashMap::default(),
            expression: HashMap::default(),
            field: HashMap::default(),
            field_id_by_name: HashMap::default(),
            function: HashMap::default(),
            function_id_by_name: HashMap::default(),
            generation_unit: HashMap::default(),
            grace_type: HashMap::default(),
            item: HashMap::default(),
            x_let: HashMap::default(),
            local: HashMap::default(),
            object_method: HashMap::default(),
            woog_option: HashMap::default(),
            ownership: HashMap::default(),
            parameter: HashMap::default(),
            reference: HashMap::default(),
            statement: HashMap::default(),
            structure: HashMap::default(),
            structure_field: HashMap::default(),
            symbol_table: HashMap::default(),
            time_stamp: HashMap::default(),
            x_value: HashMap::default(),
            variable: HashMap::default(),
            visibility: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_expression(Expression::Literal(LITERAL));
        store.inter_grace_type(GraceType::Usize(USIZE));
        store.inter_item(Item::Implementation(IMPLEMENTATION));
        store.inter_ownership(Ownership::Borrowed(BORROWED));
        store.inter_ownership(Ownership::Mutable(MUTABLE));
        store.inter_ownership(Ownership::Owned(OWNED));
        store.inter_visibility(Visibility::Krate(KRATE));
        store.inter_visibility(Visibility::Private(PRIVATE));
        store.inter_visibility(Visibility::Public(PUBLIC));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-methods"}}}
    /// Inter (insert) [`Access`] into the store.
    ///
    pub fn inter_access(&mut self, access: Access) {
        self.access.insert(access.id, (access, SystemTime::now()));
    }

    /// Exhume (get) [`Access`] from the store.
    ///
    pub fn exhume_access(&self, id: &Uuid) -> Option<&Access> {
        self.access.get(id).map(|access| &access.0)
    }

    /// Exorcise (remove) [`Access`] from the store.
    ///
    pub fn exorcise_access(&mut self, id: &Uuid) -> Option<Access> {
        self.access.remove(id).map(|access| access.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Access>`.
    ///
    pub fn iter_access(&self) -> impl Iterator<Item = &Access> {
        self.access.values().map(|access| &access.0)
    }

    /// Get the timestamp for Access.
    ///
    pub fn access_timestamp(&self, access: &Access) -> SystemTime {
        self.access
            .get(&access.id)
            .map(|access| access.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Block) {
        self.block.insert(block.id, (block, SystemTime::now()));
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<&Block> {
        self.block.get(id).map(|block| &block.0)
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &Uuid) -> Option<Block> {
        self.block.remove(id).map(|block| block.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = &Block> {
        self.block.values().map(|block| &block.0)
    }

    /// Get the timestamp for Block.
    ///
    pub fn block_timestamp(&self, block: &Block) -> SystemTime {
        self.block
            .get(&block.id)
            .map(|block| block.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Call) {
        self.call.insert(call.id, (call, SystemTime::now()));
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<&Call> {
        self.call.get(id).map(|call| &call.0)
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &Uuid) -> Option<Call> {
        self.call.remove(id).map(|call| call.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = &Call> {
        self.call.values().map(|call| &call.0)
    }

    /// Get the timestamp for Call.
    ///
    pub fn call_timestamp(&self, call: &Call) -> SystemTime {
        self.call
            .get(&call.id)
            .map(|call| call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Constant`] into the store.
    ///
    pub fn inter_constant(&mut self, constant: Constant) {
        self.constant
            .insert(constant.id, (constant, SystemTime::now()));
    }

    /// Exhume (get) [`Constant`] from the store.
    ///
    pub fn exhume_constant(&self, id: &Uuid) -> Option<&Constant> {
        self.constant.get(id).map(|constant| &constant.0)
    }

    /// Exorcise (remove) [`Constant`] from the store.
    ///
    pub fn exorcise_constant(&mut self, id: &Uuid) -> Option<Constant> {
        self.constant.remove(id).map(|constant| constant.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Constant>`.
    ///
    pub fn iter_constant(&self) -> impl Iterator<Item = &Constant> {
        self.constant.values().map(|constant| &constant.0)
    }

    /// Get the timestamp for Constant.
    ///
    pub fn constant_timestamp(&self, constant: &Constant) -> SystemTime {
        self.constant
            .get(&constant.id)
            .map(|constant| constant.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    pub fn inter_enumeration(&mut self, enumeration: Enumeration) {
        self.enumeration
            .insert(enumeration.id, (enumeration, SystemTime::now()));
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    pub fn exhume_enumeration(&self, id: &Uuid) -> Option<&Enumeration> {
        self.enumeration.get(id).map(|enumeration| &enumeration.0)
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    pub fn exorcise_enumeration(&mut self, id: &Uuid) -> Option<Enumeration> {
        self.enumeration.remove(id).map(|enumeration| enumeration.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    pub fn iter_enumeration(&self) -> impl Iterator<Item = &Enumeration> {
        self.enumeration.values().map(|enumeration| &enumeration.0)
    }

    /// Get the timestamp for Enumeration.
    ///
    pub fn enumeration_timestamp(&self, enumeration: &Enumeration) -> SystemTime {
        self.enumeration
            .get(&enumeration.id)
            .map(|enumeration| enumeration.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`EnumerationField`] into the store.
    ///
    pub fn inter_enumeration_field(&mut self, enumeration_field: EnumerationField) {
        self.enumeration_field
            .insert(enumeration_field.id, (enumeration_field, SystemTime::now()));
    }

    /// Exhume (get) [`EnumerationField`] from the store.
    ///
    pub fn exhume_enumeration_field(&self, id: &Uuid) -> Option<&EnumerationField> {
        self.enumeration_field
            .get(id)
            .map(|enumeration_field| &enumeration_field.0)
    }

    /// Exorcise (remove) [`EnumerationField`] from the store.
    ///
    pub fn exorcise_enumeration_field(&mut self, id: &Uuid) -> Option<EnumerationField> {
        self.enumeration_field
            .remove(id)
            .map(|enumeration_field| enumeration_field.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumerationField>`.
    ///
    pub fn iter_enumeration_field(&self) -> impl Iterator<Item = &EnumerationField> {
        self.enumeration_field
            .values()
            .map(|enumeration_field| &enumeration_field.0)
    }

    /// Get the timestamp for EnumerationField.
    ///
    pub fn enumeration_field_timestamp(&self, enumeration_field: &EnumerationField) -> SystemTime {
        self.enumeration_field
            .get(&enumeration_field.id)
            .map(|enumeration_field| enumeration_field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Expression) {
        self.expression
            .insert(expression.id(), (expression, SystemTime::now()));
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<&Expression> {
        self.expression.get(id).map(|expression| &expression.0)
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &Uuid) -> Option<Expression> {
        self.expression.remove(id).map(|expression| expression.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = &Expression> {
        self.expression.values().map(|expression| &expression.0)
    }

    /// Get the timestamp for Expression.
    ///
    pub fn expression_timestamp(&self, expression: &Expression) -> SystemTime {
        self.expression
            .get(&expression.id())
            .map(|expression| expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Field) {
        let value = (field, SystemTime::now());
        self.field_id_by_name
            .insert(value.0.name.to_upper_camel_case(), (value.0.id, value.1));
        self.field.insert(value.0.id, value);
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<&Field> {
        self.field.get(id).map(|field| &field.0)
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &Uuid) -> Option<Field> {
        self.field.remove(id).map(|field| field.0)
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.field_id_by_name.get(name).map(|field| field.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = &Field> {
        self.field.values().map(|field| &field.0)
    }

    /// Get the timestamp for Field.
    ///
    pub fn field_timestamp(&self, field: &Field) -> SystemTime {
        self.field
            .get(&field.id)
            .map(|field| field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Function) {
        let value = (function, SystemTime::now());
        self.function_id_by_name
            .insert(value.0.name.to_upper_camel_case(), (value.0.id, value.1));
        self.function.insert(value.0.id, value);
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<&Function> {
        self.function.get(id).map(|function| &function.0)
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &Uuid) -> Option<Function> {
        self.function.remove(id).map(|function| function.0)
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.function_id_by_name
            .get(name)
            .map(|function| function.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = &Function> {
        self.function.values().map(|function| &function.0)
    }

    /// Get the timestamp for Function.
    ///
    pub fn function_timestamp(&self, function: &Function) -> SystemTime {
        self.function
            .get(&function.id)
            .map(|function| function.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`GenerationUnit`] into the store.
    ///
    pub fn inter_generation_unit(&mut self, generation_unit: GenerationUnit) {
        self.generation_unit
            .insert(generation_unit.id, (generation_unit, SystemTime::now()));
    }

    /// Exhume (get) [`GenerationUnit`] from the store.
    ///
    pub fn exhume_generation_unit(&self, id: &Uuid) -> Option<&GenerationUnit> {
        self.generation_unit
            .get(id)
            .map(|generation_unit| &generation_unit.0)
    }

    /// Exorcise (remove) [`GenerationUnit`] from the store.
    ///
    pub fn exorcise_generation_unit(&mut self, id: &Uuid) -> Option<GenerationUnit> {
        self.generation_unit
            .remove(id)
            .map(|generation_unit| generation_unit.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GenerationUnit>`.
    ///
    pub fn iter_generation_unit(&self) -> impl Iterator<Item = &GenerationUnit> {
        self.generation_unit
            .values()
            .map(|generation_unit| &generation_unit.0)
    }

    /// Get the timestamp for GenerationUnit.
    ///
    pub fn generation_unit_timestamp(&self, generation_unit: &GenerationUnit) -> SystemTime {
        self.generation_unit
            .get(&generation_unit.id)
            .map(|generation_unit| generation_unit.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`GraceType`] into the store.
    ///
    pub fn inter_grace_type(&mut self, grace_type: GraceType) {
        self.grace_type
            .insert(grace_type.id(), (grace_type, SystemTime::now()));
    }

    /// Exhume (get) [`GraceType`] from the store.
    ///
    pub fn exhume_grace_type(&self, id: &Uuid) -> Option<&GraceType> {
        self.grace_type.get(id).map(|grace_type| &grace_type.0)
    }

    /// Exorcise (remove) [`GraceType`] from the store.
    ///
    pub fn exorcise_grace_type(&mut self, id: &Uuid) -> Option<GraceType> {
        self.grace_type.remove(id).map(|grace_type| grace_type.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GraceType>`.
    ///
    pub fn iter_grace_type(&self) -> impl Iterator<Item = &GraceType> {
        self.grace_type.values().map(|grace_type| &grace_type.0)
    }

    /// Get the timestamp for GraceType.
    ///
    pub fn grace_type_timestamp(&self, grace_type: &GraceType) -> SystemTime {
        self.grace_type
            .get(&grace_type.id())
            .map(|grace_type| grace_type.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Item) {
        self.item.insert(item.id(), (item, SystemTime::now()));
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<&Item> {
        self.item.get(id).map(|item| &item.0)
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &Uuid) -> Option<Item> {
        self.item.remove(id).map(|item| item.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = &Item> {
        self.item.values().map(|item| &item.0)
    }

    /// Get the timestamp for Item.
    ///
    pub fn item_timestamp(&self, item: &Item) -> SystemTime {
        self.item
            .get(&item.id())
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XLet`] into the store.
    ///
    pub fn inter_x_let(&mut self, x_let: XLet) {
        self.x_let.insert(x_let.id, (x_let, SystemTime::now()));
    }

    /// Exhume (get) [`XLet`] from the store.
    ///
    pub fn exhume_x_let(&self, id: &Uuid) -> Option<&XLet> {
        self.x_let.get(id).map(|x_let| &x_let.0)
    }

    /// Exorcise (remove) [`XLet`] from the store.
    ///
    pub fn exorcise_x_let(&mut self, id: &Uuid) -> Option<XLet> {
        self.x_let.remove(id).map(|x_let| x_let.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XLet>`.
    ///
    pub fn iter_x_let(&self) -> impl Iterator<Item = &XLet> {
        self.x_let.values().map(|x_let| &x_let.0)
    }

    /// Get the timestamp for XLet.
    ///
    pub fn x_let_timestamp(&self, x_let: &XLet) -> SystemTime {
        self.x_let
            .get(&x_let.id)
            .map(|x_let| x_let.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Local`] into the store.
    ///
    pub fn inter_local(&mut self, local: Local) {
        self.local.insert(local.id, (local, SystemTime::now()));
    }

    /// Exhume (get) [`Local`] from the store.
    ///
    pub fn exhume_local(&self, id: &Uuid) -> Option<&Local> {
        self.local.get(id).map(|local| &local.0)
    }

    /// Exorcise (remove) [`Local`] from the store.
    ///
    pub fn exorcise_local(&mut self, id: &Uuid) -> Option<Local> {
        self.local.remove(id).map(|local| local.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Local>`.
    ///
    pub fn iter_local(&self) -> impl Iterator<Item = &Local> {
        self.local.values().map(|local| &local.0)
    }

    /// Get the timestamp for Local.
    ///
    pub fn local_timestamp(&self, local: &Local) -> SystemTime {
        self.local
            .get(&local.id)
            .map(|local| local.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ObjectMethod`] into the store.
    ///
    pub fn inter_object_method(&mut self, object_method: ObjectMethod) {
        self.object_method
            .insert(object_method.id, (object_method, SystemTime::now()));
    }

    /// Exhume (get) [`ObjectMethod`] from the store.
    ///
    pub fn exhume_object_method(&self, id: &Uuid) -> Option<&ObjectMethod> {
        self.object_method
            .get(id)
            .map(|object_method| &object_method.0)
    }

    /// Exorcise (remove) [`ObjectMethod`] from the store.
    ///
    pub fn exorcise_object_method(&mut self, id: &Uuid) -> Option<ObjectMethod> {
        self.object_method
            .remove(id)
            .map(|object_method| object_method.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectMethod>`.
    ///
    pub fn iter_object_method(&self) -> impl Iterator<Item = &ObjectMethod> {
        self.object_method
            .values()
            .map(|object_method| &object_method.0)
    }

    /// Get the timestamp for ObjectMethod.
    ///
    pub fn object_method_timestamp(&self, object_method: &ObjectMethod) -> SystemTime {
        self.object_method
            .get(&object_method.id)
            .map(|object_method| object_method.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: WoogOption) {
        self.woog_option
            .insert(woog_option.id, (woog_option, SystemTime::now()));
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &Uuid) -> Option<&WoogOption> {
        self.woog_option.get(id).map(|woog_option| &woog_option.0)
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub fn exorcise_woog_option(&mut self, id: &Uuid) -> Option<WoogOption> {
        self.woog_option.remove(id).map(|woog_option| woog_option.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = &WoogOption> {
        self.woog_option.values().map(|woog_option| &woog_option.0)
    }

    /// Get the timestamp for WoogOption.
    ///
    pub fn woog_option_timestamp(&self, woog_option: &WoogOption) -> SystemTime {
        self.woog_option
            .get(&woog_option.id)
            .map(|woog_option| woog_option.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Ownership`] into the store.
    ///
    pub fn inter_ownership(&mut self, ownership: Ownership) {
        self.ownership
            .insert(ownership.id(), (ownership, SystemTime::now()));
    }

    /// Exhume (get) [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &Uuid) -> Option<&Ownership> {
        self.ownership.get(id).map(|ownership| &ownership.0)
    }

    /// Exorcise (remove) [`Ownership`] from the store.
    ///
    pub fn exorcise_ownership(&mut self, id: &Uuid) -> Option<Ownership> {
        self.ownership.remove(id).map(|ownership| ownership.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = &Ownership> {
        self.ownership.values().map(|ownership| &ownership.0)
    }

    /// Get the timestamp for Ownership.
    ///
    pub fn ownership_timestamp(&self, ownership: &Ownership) -> SystemTime {
        self.ownership
            .get(&ownership.id())
            .map(|ownership| ownership.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Parameter) {
        self.parameter
            .insert(parameter.id, (parameter, SystemTime::now()));
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<&Parameter> {
        self.parameter.get(id).map(|parameter| &parameter.0)
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Parameter> {
        self.parameter.remove(id).map(|parameter| parameter.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = &Parameter> {
        self.parameter.values().map(|parameter| &parameter.0)
    }

    /// Get the timestamp for Parameter.
    ///
    pub fn parameter_timestamp(&self, parameter: &Parameter) -> SystemTime {
        self.parameter
            .get(&parameter.id)
            .map(|parameter| parameter.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Reference) {
        self.reference
            .insert(reference.id, (reference, SystemTime::now()));
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<&Reference> {
        self.reference.get(id).map(|reference| &reference.0)
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &Uuid) -> Option<Reference> {
        self.reference.remove(id).map(|reference| reference.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = &Reference> {
        self.reference.values().map(|reference| &reference.0)
    }

    /// Get the timestamp for Reference.
    ///
    pub fn reference_timestamp(&self, reference: &Reference) -> SystemTime {
        self.reference
            .get(&reference.id)
            .map(|reference| reference.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Statement) {
        self.statement
            .insert(statement.id, (statement, SystemTime::now()));
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<&Statement> {
        self.statement.get(id).map(|statement| &statement.0)
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &Uuid) -> Option<Statement> {
        self.statement.remove(id).map(|statement| statement.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = &Statement> {
        self.statement.values().map(|statement| &statement.0)
    }

    /// Get the timestamp for Statement.
    ///
    pub fn statement_timestamp(&self, statement: &Statement) -> SystemTime {
        self.statement
            .get(&statement.id)
            .map(|statement| statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Structure`] into the store.
    ///
    pub fn inter_structure(&mut self, structure: Structure) {
        self.structure
            .insert(structure.id, (structure, SystemTime::now()));
    }

    /// Exhume (get) [`Structure`] from the store.
    ///
    pub fn exhume_structure(&self, id: &Uuid) -> Option<&Structure> {
        self.structure.get(id).map(|structure| &structure.0)
    }

    /// Exorcise (remove) [`Structure`] from the store.
    ///
    pub fn exorcise_structure(&mut self, id: &Uuid) -> Option<Structure> {
        self.structure.remove(id).map(|structure| structure.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Structure>`.
    ///
    pub fn iter_structure(&self) -> impl Iterator<Item = &Structure> {
        self.structure.values().map(|structure| &structure.0)
    }

    /// Get the timestamp for Structure.
    ///
    pub fn structure_timestamp(&self, structure: &Structure) -> SystemTime {
        self.structure
            .get(&structure.id)
            .map(|structure| structure.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StructureField`] into the store.
    ///
    pub fn inter_structure_field(&mut self, structure_field: StructureField) {
        self.structure_field
            .insert(structure_field.id, (structure_field, SystemTime::now()));
    }

    /// Exhume (get) [`StructureField`] from the store.
    ///
    pub fn exhume_structure_field(&self, id: &Uuid) -> Option<&StructureField> {
        self.structure_field
            .get(id)
            .map(|structure_field| &structure_field.0)
    }

    /// Exorcise (remove) [`StructureField`] from the store.
    ///
    pub fn exorcise_structure_field(&mut self, id: &Uuid) -> Option<StructureField> {
        self.structure_field
            .remove(id)
            .map(|structure_field| structure_field.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructureField>`.
    ///
    pub fn iter_structure_field(&self) -> impl Iterator<Item = &StructureField> {
        self.structure_field
            .values()
            .map(|structure_field| &structure_field.0)
    }

    /// Get the timestamp for StructureField.
    ///
    pub fn structure_field_timestamp(&self, structure_field: &StructureField) -> SystemTime {
        self.structure_field
            .get(&structure_field.id)
            .map(|structure_field| structure_field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`SymbolTable`] into the store.
    ///
    pub fn inter_symbol_table(&mut self, symbol_table: SymbolTable) {
        self.symbol_table
            .insert(symbol_table.id, (symbol_table, SystemTime::now()));
    }

    /// Exhume (get) [`SymbolTable`] from the store.
    ///
    pub fn exhume_symbol_table(&self, id: &Uuid) -> Option<&SymbolTable> {
        self.symbol_table
            .get(id)
            .map(|symbol_table| &symbol_table.0)
    }

    /// Exorcise (remove) [`SymbolTable`] from the store.
    ///
    pub fn exorcise_symbol_table(&mut self, id: &Uuid) -> Option<SymbolTable> {
        self.symbol_table
            .remove(id)
            .map(|symbol_table| symbol_table.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SymbolTable>`.
    ///
    pub fn iter_symbol_table(&self) -> impl Iterator<Item = &SymbolTable> {
        self.symbol_table
            .values()
            .map(|symbol_table| &symbol_table.0)
    }

    /// Get the timestamp for SymbolTable.
    ///
    pub fn symbol_table_timestamp(&self, symbol_table: &SymbolTable) -> SystemTime {
        self.symbol_table
            .get(&symbol_table.id)
            .map(|symbol_table| symbol_table.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`TimeStamp`] into the store.
    ///
    pub fn inter_time_stamp(&mut self, time_stamp: TimeStamp) {
        self.time_stamp
            .insert(time_stamp.id, (time_stamp, SystemTime::now()));
    }

    /// Exhume (get) [`TimeStamp`] from the store.
    ///
    pub fn exhume_time_stamp(&self, id: &Uuid) -> Option<&TimeStamp> {
        self.time_stamp.get(id).map(|time_stamp| &time_stamp.0)
    }

    /// Exorcise (remove) [`TimeStamp`] from the store.
    ///
    pub fn exorcise_time_stamp(&mut self, id: &Uuid) -> Option<TimeStamp> {
        self.time_stamp.remove(id).map(|time_stamp| time_stamp.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TimeStamp>`.
    ///
    pub fn iter_time_stamp(&self) -> impl Iterator<Item = &TimeStamp> {
        self.time_stamp.values().map(|time_stamp| &time_stamp.0)
    }

    /// Get the timestamp for TimeStamp.
    ///
    pub fn time_stamp_timestamp(&self, time_stamp: &TimeStamp) -> SystemTime {
        self.time_stamp
            .get(&time_stamp.id)
            .map(|time_stamp| time_stamp.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value(&mut self, x_value: XValue) {
        self.x_value
            .insert(x_value.id, (x_value, SystemTime::now()));
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &Uuid) -> Option<&XValue> {
        self.x_value.get(id).map(|x_value| &x_value.0)
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &Uuid) -> Option<XValue> {
        self.x_value.remove(id).map(|x_value| x_value.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = &XValue> {
        self.x_value.values().map(|x_value| &x_value.0)
    }

    /// Get the timestamp for XValue.
    ///
    pub fn x_value_timestamp(&self, x_value: &XValue) -> SystemTime {
        self.x_value
            .get(&x_value.id)
            .map(|x_value| x_value.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Variable) {
        self.variable
            .insert(variable.id, (variable, SystemTime::now()));
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<&Variable> {
        self.variable.get(id).map(|variable| &variable.0)
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &Uuid) -> Option<Variable> {
        self.variable.remove(id).map(|variable| variable.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = &Variable> {
        self.variable.values().map(|variable| &variable.0)
    }

    /// Get the timestamp for Variable.
    ///
    pub fn variable_timestamp(&self, variable: &Variable) -> SystemTime {
        self.variable
            .get(&variable.id)
            .map(|variable| variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Visibility`] into the store.
    ///
    pub fn inter_visibility(&mut self, visibility: Visibility) {
        self.visibility
            .insert(visibility.id(), (visibility, SystemTime::now()));
    }

    /// Exhume (get) [`Visibility`] from the store.
    ///
    pub fn exhume_visibility(&self, id: &Uuid) -> Option<&Visibility> {
        self.visibility.get(id).map(|visibility| &visibility.0)
    }

    /// Exorcise (remove) [`Visibility`] from the store.
    ///
    pub fn exorcise_visibility(&mut self, id: &Uuid) -> Option<Visibility> {
        self.visibility.remove(id).map(|visibility| visibility.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Visibility>`.
    ///
    pub fn iter_visibility(&self) -> impl Iterator<Item = &Visibility> {
        self.visibility.values().map(|visibility| &visibility.0)
    }

    /// Get the timestamp for Visibility.
    ///
    pub fn visibility_timestamp(&self, visibility: &Visibility) -> SystemTime {
        self.visibility
            .get(&visibility.id())
            .map(|visibility| visibility.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a a bincode file.
    pub fn persist_bincode<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let mut bin_file = fs::File::create(path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;
        Ok(())
    }

    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        let path = path.join("woog.json");
        fs::create_dir_all(&path)?;

        // Persist Access.
        {
            let path = path.join("access");
            fs::create_dir_all(&path)?;
            for access_tuple in self.access.values() {
                let path = path.join(format!("{}.json", access_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Access, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != access_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &access_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &access_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.access.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block_tuple in self.block.values() {
                let path = path.join(format!("{}.json", block_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Block, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != block_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &block_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &block_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.block.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call_tuple in self.call.values() {
                let path = path.join(format!("{}.json", call_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Call, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != call_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &call_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &call_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.call.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Constant.
        {
            let path = path.join("constant");
            fs::create_dir_all(&path)?;
            for constant_tuple in self.constant.values() {
                let path = path.join(format!("{}.json", constant_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Constant, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != constant_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &constant_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &constant_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.constant.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration_tuple in self.enumeration.values() {
                let path = path.join(format!("{}.json", enumeration_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Enumeration, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != enumeration_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &enumeration_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enumeration_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.enumeration.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Enumeration Field.
        {
            let path = path.join("enumeration_field");
            fs::create_dir_all(&path)?;
            for enumeration_field_tuple in self.enumeration_field.values() {
                let path = path.join(format!("{}.json", enumeration_field_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (EnumerationField, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != enumeration_field_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &enumeration_field_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enumeration_field_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.enumeration_field.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression_tuple in self.expression.values() {
                let path = path.join(format!("{}.json", expression_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Expression, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != expression_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &expression_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &expression_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.expression.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field_tuple in self.field.values() {
                let path = path.join(format!("{}.json", field_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Field, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != field_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &field_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function_tuple in self.function.values() {
                let path = path.join(format!("{}.json", function_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Function, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != function_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &function_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &function_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.function.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Generation Unit.
        {
            let path = path.join("generation_unit");
            fs::create_dir_all(&path)?;
            for generation_unit_tuple in self.generation_unit.values() {
                let path = path.join(format!("{}.json", generation_unit_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (GenerationUnit, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != generation_unit_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &generation_unit_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &generation_unit_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.generation_unit.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Grace Type.
        {
            let path = path.join("grace_type");
            fs::create_dir_all(&path)?;
            for grace_type_tuple in self.grace_type.values() {
                let path = path.join(format!("{}.json", grace_type_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (GraceType, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != grace_type_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &grace_type_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &grace_type_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.grace_type.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item_tuple in self.item.values() {
                let path = path.join(format!("{}.json", item_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Item, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != item_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &item_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &item_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.item.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Let.
        {
            let path = path.join("x_let");
            fs::create_dir_all(&path)?;
            for x_let_tuple in self.x_let.values() {
                let path = path.join(format!("{}.json", x_let_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (XLet, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != x_let_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &x_let_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_let_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_let.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Local.
        {
            let path = path.join("local");
            fs::create_dir_all(&path)?;
            for local_tuple in self.local.values() {
                let path = path.join(format!("{}.json", local_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Local, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != local_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &local_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &local_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.local.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Method.
        {
            let path = path.join("object_method");
            fs::create_dir_all(&path)?;
            for object_method_tuple in self.object_method.values() {
                let path = path.join(format!("{}.json", object_method_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ObjectMethod, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != object_method_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &object_method_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object_method_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.object_method.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option_tuple in self.woog_option.values() {
                let path = path.join(format!("{}.json", woog_option_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (WoogOption, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != woog_option_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &woog_option_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &woog_option_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.woog_option.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Ownership.
        {
            let path = path.join("ownership");
            fs::create_dir_all(&path)?;
            for ownership_tuple in self.ownership.values() {
                let path = path.join(format!("{}.json", ownership_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Ownership, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != ownership_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &ownership_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &ownership_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.ownership.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter_tuple in self.parameter.values() {
                let path = path.join(format!("{}.json", parameter_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Parameter, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != parameter_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &parameter_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &parameter_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.parameter.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference_tuple in self.reference.values() {
                let path = path.join(format!("{}.json", reference_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Reference, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != reference_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &reference_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &reference_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.reference.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement_tuple in self.statement.values() {
                let path = path.join(format!("{}.json", statement_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Statement, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != statement_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &statement_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &statement_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.statement.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Structure.
        {
            let path = path.join("structure");
            fs::create_dir_all(&path)?;
            for structure_tuple in self.structure.values() {
                let path = path.join(format!("{}.json", structure_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Structure, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != structure_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &structure_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &structure_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.structure.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Structure Field.
        {
            let path = path.join("structure_field");
            fs::create_dir_all(&path)?;
            for structure_field_tuple in self.structure_field.values() {
                let path = path.join(format!("{}.json", structure_field_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (StructureField, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != structure_field_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &structure_field_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &structure_field_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.structure_field.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Symbol Table.
        {
            let path = path.join("symbol_table");
            fs::create_dir_all(&path)?;
            for symbol_table_tuple in self.symbol_table.values() {
                let path = path.join(format!("{}.json", symbol_table_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SymbolTable, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != symbol_table_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &symbol_table_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &symbol_table_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.symbol_table.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Time Stamp.
        {
            let path = path.join("time_stamp");
            fs::create_dir_all(&path)?;
            for time_stamp_tuple in self.time_stamp.values() {
                let path = path.join(format!("{}.json", time_stamp_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (TimeStamp, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != time_stamp_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &time_stamp_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &time_stamp_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.time_stamp.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value_tuple in self.x_value.values() {
                let path = path.join(format!("{}.json", x_value_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (XValue, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != x_value_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &x_value_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_value_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_value.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable_tuple in self.variable.values() {
                let path = path.join(format!("{}.json", variable_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Variable, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != variable_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &variable_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &variable_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.variable.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Visibility.
        {
            let path = path.join("visibility");
            fs::create_dir_all(&path)?;
            for visibility_tuple in self.visibility.values() {
                let path = path.join(format!("{}.json", visibility_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Visibility, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != visibility_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &visibility_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &visibility_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.visibility.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
    pub fn from_bincode(code: &[u8]) -> io::Result<Self> {
        Ok(bincode::deserialize(code).unwrap())
    }

    /// The store is as a bincode file.
    pub fn load_bincode<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let bin_file = fs::File::open(path)?;
        Ok(bincode::deserialize_from(bin_file).unwrap())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("woog.json");

        let mut store = Self::new();

        // Load Access.
        {
            let path = path.join("access");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let access: (Access, SystemTime) = serde_json::from_reader(reader)?;
                store.access.insert(access.0.id, access);
            }
        }

        // Load Block.
        {
            let path = path.join("block");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let block: (Block, SystemTime) = serde_json::from_reader(reader)?;
                store.block.insert(block.0.id, block);
            }
        }

        // Load Call.
        {
            let path = path.join("call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let call: (Call, SystemTime) = serde_json::from_reader(reader)?;
                store.call.insert(call.0.id, call);
            }
        }

        // Load Constant.
        {
            let path = path.join("constant");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let constant: (Constant, SystemTime) = serde_json::from_reader(reader)?;
                store.constant.insert(constant.0.id, constant);
            }
        }

        // Load Enumeration.
        {
            let path = path.join("enumeration");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enumeration: (Enumeration, SystemTime) = serde_json::from_reader(reader)?;
                store.enumeration.insert(enumeration.0.id, enumeration);
            }
        }

        // Load Enumeration Field.
        {
            let path = path.join("enumeration_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enumeration_field: (EnumerationField, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .enumeration_field
                    .insert(enumeration_field.0.id, enumeration_field);
            }
        }

        // Load Expression.
        {
            let path = path.join("expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let expression: (Expression, SystemTime) = serde_json::from_reader(reader)?;
                store.expression.insert(expression.0.id(), expression);
            }
        }

        // Load Field.
        {
            let path = path.join("field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field: (Field, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .field_id_by_name
                    .insert(field.0.name.to_upper_camel_case(), (field.0.id, field.1));
                store.field.insert(field.0.id, field);
            }
        }

        // Load Function.
        {
            let path = path.join("function");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let function: (Function, SystemTime) = serde_json::from_reader(reader)?;
                store.function_id_by_name.insert(
                    function.0.name.to_upper_camel_case(),
                    (function.0.id, function.1),
                );
                store.function.insert(function.0.id, function);
            }
        }

        // Load Generation Unit.
        {
            let path = path.join("generation_unit");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let generation_unit: (GenerationUnit, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .generation_unit
                    .insert(generation_unit.0.id, generation_unit);
            }
        }

        // Load Grace Type.
        {
            let path = path.join("grace_type");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let grace_type: (GraceType, SystemTime) = serde_json::from_reader(reader)?;
                store.grace_type.insert(grace_type.0.id(), grace_type);
            }
        }

        // Load Item.
        {
            let path = path.join("item");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let item: (Item, SystemTime) = serde_json::from_reader(reader)?;
                store.item.insert(item.0.id(), item);
            }
        }

        // Load Let.
        {
            let path = path.join("x_let");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_let: (XLet, SystemTime) = serde_json::from_reader(reader)?;
                store.x_let.insert(x_let.0.id, x_let);
            }
        }

        // Load Local.
        {
            let path = path.join("local");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let local: (Local, SystemTime) = serde_json::from_reader(reader)?;
                store.local.insert(local.0.id, local);
            }
        }

        // Load Object Method.
        {
            let path = path.join("object_method");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object_method: (ObjectMethod, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .object_method
                    .insert(object_method.0.id, object_method);
            }
        }

        // Load Option.
        {
            let path = path.join("woog_option");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let woog_option: (WoogOption, SystemTime) = serde_json::from_reader(reader)?;
                store.woog_option.insert(woog_option.0.id, woog_option);
            }
        }

        // Load Ownership.
        {
            let path = path.join("ownership");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let ownership: (Ownership, SystemTime) = serde_json::from_reader(reader)?;
                store.ownership.insert(ownership.0.id(), ownership);
            }
        }

        // Load Parameter.
        {
            let path = path.join("parameter");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let parameter: (Parameter, SystemTime) = serde_json::from_reader(reader)?;
                store.parameter.insert(parameter.0.id, parameter);
            }
        }

        // Load Reference.
        {
            let path = path.join("reference");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let reference: (Reference, SystemTime) = serde_json::from_reader(reader)?;
                store.reference.insert(reference.0.id, reference);
            }
        }

        // Load Statement.
        {
            let path = path.join("statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let statement: (Statement, SystemTime) = serde_json::from_reader(reader)?;
                store.statement.insert(statement.0.id, statement);
            }
        }

        // Load Structure.
        {
            let path = path.join("structure");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let structure: (Structure, SystemTime) = serde_json::from_reader(reader)?;
                store.structure.insert(structure.0.id, structure);
            }
        }

        // Load Structure Field.
        {
            let path = path.join("structure_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let structure_field: (StructureField, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .structure_field
                    .insert(structure_field.0.id, structure_field);
            }
        }

        // Load Symbol Table.
        {
            let path = path.join("symbol_table");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let symbol_table: (SymbolTable, SystemTime) = serde_json::from_reader(reader)?;
                store.symbol_table.insert(symbol_table.0.id, symbol_table);
            }
        }

        // Load Time Stamp.
        {
            let path = path.join("time_stamp");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let time_stamp: (TimeStamp, SystemTime) = serde_json::from_reader(reader)?;
                store.time_stamp.insert(time_stamp.0.id, time_stamp);
            }
        }

        // Load Value.
        {
            let path = path.join("x_value");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_value: (XValue, SystemTime) = serde_json::from_reader(reader)?;
                store.x_value.insert(x_value.0.id, x_value);
            }
        }

        // Load Variable.
        {
            let path = path.join("variable");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let variable: (Variable, SystemTime) = serde_json::from_reader(reader)?;
                store.variable.insert(variable.0.id, variable);
            }
        }

        // Load Visibility.
        {
            let path = path.join("visibility");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let visibility: (Visibility, SystemTime) = serde_json::from_reader(reader)?;
                store.visibility.insert(visibility.0.id(), visibility);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
