//! v2::lu_dog_rwlock Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Argument`]
//! * [`AWait`]
//! * [`Binary`]
//! * [`Block`]
//! * [`Body`]
//! * [`BooleanLiteral`]
//! * [`BooleanOperator`]
//! * [`Call`]
//! * [`CharLiteral`]
//! * [`Comparison`]
//! * [`DataStructure`]
//! * [`DwarfSourceFile`]
//! * [`EnumField`]
//! * [`EnumGeneric`]
//! * [`EnumGenericType`]
//! * [`Enumeration`]
//! * [`Expression`]
//! * [`ExpressionBit`]
//! * [`ExpressionStatement`]
//! * [`ExternalImplementation`]
//! * [`Field`]
//! * [`FieldAccess`]
//! * [`FieldAccessTarget`]
//! * [`FieldExpression`]
//! * [`FloatLiteral`]
//! * [`ForLoop`]
//! * [`FormatBit`]
//! * [`FormatString`]
//! * [`FuncGeneric`]
//! * [`Function`]
//! * [`FunctionCall`]
//! * [`XFuture`]
//! * [`Grouped`]
//! * [`HaltAndCatchFire`]
//! * [`XIf`]
//! * [`ImplementationBlock`]
//! * [`Import`]
//! * [`Index`]
//! * [`IntegerLiteral`]
//! * [`Item`]
//! * [`Lambda`]
//! * [`LambdaParameter`]
//! * [`LetStatement`]
//! * [`List`]
//! * [`ListElement`]
//! * [`ListExpression`]
//! * [`Literal`]
//! * [`LocalVariable`]
//! * [`XMacro`]
//! * [`Map`]
//! * [`MapElement`]
//! * [`MapExpression`]
//! * [`XMatch`]
//! * [`MethodCall`]
//! * [`NamedFieldExpression`]
//! * [`ZObjectStore`]
//! * [`ObjectWrapper`]
//! * [`Operator`]
//! * [`Parameter`]
//! * [`XPath`]
//! * [`PathElement`]
//! * [`Pattern`]
//! * [`XPlugin`]
//! * [`XPrint`]
//! * [`RangeExpression`]
//! * [`ResultStatement`]
//! * [`XReturn`]
//! * [`Span`]
//! * [`Statement`]
//! * [`StaticMethodCall`]
//! * [`StringBit`]
//! * [`StringLiteral`]
//! * [`WoogStruct`]
//! * [`StructExpression`]
//! * [`StructField`]
//! * [`StructGeneric`]
//! * [`TupleField`]
//! * [`TypeCast`]
//! * [`Unary`]
//! * [`Unit`]
//! * [`UnnamedFieldExpression`]
//! * [`XValue`]
//! * [`ValueType`]
//! * [`Variable`]
//! * [`VariableExpression`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock-object-store-definition"}}}
use std::sync::Arc;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use ordered_hash_map::OrderedHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::{
    AWait, Argument, Binary, Block, Body, BooleanLiteral, BooleanOperator, Call, CharLiteral,
    Comparison, DataStructure, DwarfSourceFile, EnumField, EnumGeneric, EnumGenericType,
    Enumeration, Expression, ExpressionBit, ExpressionStatement, ExternalImplementation, Field,
    FieldAccess, FieldAccessTarget, FieldExpression, FloatLiteral, ForLoop, FormatBit,
    FormatString, FuncGeneric, Function, FunctionCall, Grouped, HaltAndCatchFire,
    ImplementationBlock, Import, Index, IntegerLiteral, Item, Lambda, LambdaParameter,
    LetStatement, List, ListElement, ListExpression, Literal, LocalVariable, Map, MapElement,
    MapExpression, MethodCall, NamedFieldExpression, ObjectWrapper, Operator, Parameter,
    PathElement, Pattern, RangeExpression, ResultStatement, Span, Statement, StaticMethodCall,
    StringBit, StringLiteral, StructExpression, StructField, StructGeneric, TupleField, TypeCast,
    Unary, Unit, UnnamedFieldExpression, ValueType, Variable, VariableExpression, WoogStruct,
    XFuture, XIf, XMacro, XMatch, XPath, XPlugin, XPrint, XReturn, XValue, ZObjectStore,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Argument>>>>>,
    a_wait: Arc<RwLock<HashMap<Uuid, Arc<RwLock<AWait>>>>>,
    binary: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Binary>>>>>,
    block: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Block>>>>>,
    body: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Body>>>>>,
    boolean_literal: Arc<RwLock<HashMap<Uuid, Arc<RwLock<BooleanLiteral>>>>>,
    boolean_operator: Arc<RwLock<HashMap<Uuid, Arc<RwLock<BooleanOperator>>>>>,
    call: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Call>>>>>,
    char_literal: Arc<RwLock<HashMap<Uuid, Arc<RwLock<CharLiteral>>>>>,
    comparison: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Comparison>>>>>,
    data_structure: Arc<RwLock<HashMap<Uuid, Arc<RwLock<DataStructure>>>>>,
    dwarf_source_file: Arc<RwLock<HashMap<Uuid, Arc<RwLock<DwarfSourceFile>>>>>,
    enum_field: Arc<RwLock<HashMap<Uuid, Arc<RwLock<EnumField>>>>>,
    enum_generic: Arc<RwLock<HashMap<Uuid, Arc<RwLock<EnumGeneric>>>>>,
    enum_generic_type: Arc<RwLock<HashMap<Uuid, Arc<RwLock<EnumGenericType>>>>>,
    enumeration: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Enumeration>>>>>,
    enumeration_id_by_name: Arc<RwLock<HashMap<String, Uuid>>>,
    expression: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Expression>>>>>,
    expression_bit: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ExpressionBit>>>>>,
    expression_statement: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ExpressionStatement>>>>>,
    external_implementation: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ExternalImplementation>>>>>,
    field: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Field>>>>>,
    field_id_by_name: Arc<RwLock<HashMap<String, Uuid>>>,
    field_access: Arc<RwLock<HashMap<Uuid, Arc<RwLock<FieldAccess>>>>>,
    field_access_target: Arc<RwLock<HashMap<Uuid, Arc<RwLock<FieldAccessTarget>>>>>,
    field_expression: Arc<RwLock<HashMap<Uuid, Arc<RwLock<FieldExpression>>>>>,
    float_literal: Arc<RwLock<HashMap<Uuid, Arc<RwLock<FloatLiteral>>>>>,
    for_loop: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ForLoop>>>>>,
    format_bit: Arc<RwLock<HashMap<Uuid, Arc<RwLock<FormatBit>>>>>,
    format_string: Arc<RwLock<HashMap<Uuid, Arc<RwLock<FormatString>>>>>,
    func_generic: Arc<RwLock<HashMap<Uuid, Arc<RwLock<FuncGeneric>>>>>,
    function: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Function>>>>>,
    function_id_by_name: Arc<RwLock<HashMap<String, Uuid>>>,
    function_call: Arc<RwLock<HashMap<Uuid, Arc<RwLock<FunctionCall>>>>>,
    x_future: Arc<RwLock<HashMap<Uuid, Arc<RwLock<XFuture>>>>>,
    grouped: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Grouped>>>>>,
    halt_and_catch_fire: Arc<RwLock<HashMap<Uuid, Arc<RwLock<HaltAndCatchFire>>>>>,
    x_if: Arc<RwLock<HashMap<Uuid, Arc<RwLock<XIf>>>>>,
    implementation_block: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ImplementationBlock>>>>>,
    import: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Import>>>>>,
    index: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Index>>>>>,
    integer_literal: Arc<RwLock<HashMap<Uuid, Arc<RwLock<IntegerLiteral>>>>>,
    item: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Item>>>>>,
    lambda: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Lambda>>>>>,
    lambda_parameter: Arc<RwLock<HashMap<Uuid, Arc<RwLock<LambdaParameter>>>>>,
    let_statement: Arc<RwLock<HashMap<Uuid, Arc<RwLock<LetStatement>>>>>,
    list: Arc<RwLock<HashMap<Uuid, Arc<RwLock<List>>>>>,
    list_element: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ListElement>>>>>,
    list_expression: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ListExpression>>>>>,
    literal: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Literal>>>>>,
    local_variable: Arc<RwLock<HashMap<Uuid, Arc<RwLock<LocalVariable>>>>>,
    x_macro: Arc<RwLock<HashMap<Uuid, Arc<RwLock<XMacro>>>>>,
    map: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Map>>>>>,
    map_element: Arc<RwLock<HashMap<Uuid, Arc<RwLock<MapElement>>>>>,
    map_expression: Arc<RwLock<HashMap<Uuid, Arc<RwLock<MapExpression>>>>>,
    x_match: Arc<RwLock<HashMap<Uuid, Arc<RwLock<XMatch>>>>>,
    method_call: Arc<RwLock<HashMap<Uuid, Arc<RwLock<MethodCall>>>>>,
    named_field_expression: Arc<RwLock<HashMap<Uuid, Arc<RwLock<NamedFieldExpression>>>>>,
    z_object_store: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ZObjectStore>>>>>,
    z_object_store_id_by_name: Arc<RwLock<HashMap<String, Uuid>>>,
    object_wrapper: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ObjectWrapper>>>>>,
    operator: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Operator>>>>>,
    parameter: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Parameter>>>>>,
    x_path: Arc<RwLock<HashMap<Uuid, Arc<RwLock<XPath>>>>>,
    path_element: Arc<RwLock<HashMap<Uuid, Arc<RwLock<PathElement>>>>>,
    pattern: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Pattern>>>>>,
    x_plugin: Arc<RwLock<HashMap<Uuid, Arc<RwLock<XPlugin>>>>>,
    x_plugin_id_by_name: Arc<RwLock<HashMap<String, Uuid>>>,
    x_print: Arc<RwLock<HashMap<Uuid, Arc<RwLock<XPrint>>>>>,
    range_expression: Arc<RwLock<HashMap<Uuid, Arc<RwLock<RangeExpression>>>>>,
    result_statement: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ResultStatement>>>>>,
    x_return: Arc<RwLock<HashMap<Uuid, Arc<RwLock<XReturn>>>>>,
    span: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Span>>>>>,
    statement: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Statement>>>>>,
    static_method_call: Arc<RwLock<HashMap<Uuid, Arc<RwLock<StaticMethodCall>>>>>,
    string_bit: Arc<RwLock<HashMap<Uuid, Arc<RwLock<StringBit>>>>>,
    string_literal: Arc<RwLock<HashMap<Uuid, Arc<RwLock<StringLiteral>>>>>,
    woog_struct: Arc<RwLock<HashMap<Uuid, Arc<RwLock<WoogStruct>>>>>,
    woog_struct_id_by_name: Arc<RwLock<HashMap<String, Uuid>>>,
    struct_expression: Arc<RwLock<HashMap<Uuid, Arc<RwLock<StructExpression>>>>>,
    struct_field: Arc<RwLock<HashMap<Uuid, Arc<RwLock<StructField>>>>>,
    struct_generic: Arc<RwLock<HashMap<Uuid, Arc<RwLock<StructGeneric>>>>>,
    tuple_field: Arc<RwLock<HashMap<Uuid, Arc<RwLock<TupleField>>>>>,
    type_cast: Arc<RwLock<HashMap<Uuid, Arc<RwLock<TypeCast>>>>>,
    unary: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Unary>>>>>,
    unit: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Unit>>>>>,
    unnamed_field_expression: Arc<RwLock<HashMap<Uuid, Arc<RwLock<UnnamedFieldExpression>>>>>,
    x_value: Arc<RwLock<HashMap<Uuid, Arc<RwLock<XValue>>>>>,
    value_type: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ValueType>>>>>,
    variable: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Variable>>>>>,
    variable_expression: Arc<RwLock<HashMap<Uuid, Arc<RwLock<VariableExpression>>>>>,
}

impl Clone for ObjectStore {
    fn clone(&self) -> Self {
        ObjectStore {
            argument: Arc::new(RwLock::new(self.argument.read().unwrap().clone())),
            a_wait: Arc::new(RwLock::new(self.a_wait.read().unwrap().clone())),
            binary: Arc::new(RwLock::new(self.binary.read().unwrap().clone())),
            block: Arc::new(RwLock::new(self.block.read().unwrap().clone())),
            body: Arc::new(RwLock::new(self.body.read().unwrap().clone())),
            boolean_literal: Arc::new(RwLock::new(self.boolean_literal.read().unwrap().clone())),
            boolean_operator: Arc::new(RwLock::new(self.boolean_operator.read().unwrap().clone())),
            call: Arc::new(RwLock::new(self.call.read().unwrap().clone())),
            char_literal: Arc::new(RwLock::new(self.char_literal.read().unwrap().clone())),
            comparison: Arc::new(RwLock::new(self.comparison.read().unwrap().clone())),
            data_structure: Arc::new(RwLock::new(self.data_structure.read().unwrap().clone())),
            dwarf_source_file: Arc::new(RwLock::new(
                self.dwarf_source_file.read().unwrap().clone(),
            )),
            enum_field: Arc::new(RwLock::new(self.enum_field.read().unwrap().clone())),
            enum_generic: Arc::new(RwLock::new(self.enum_generic.read().unwrap().clone())),
            enum_generic_type: Arc::new(RwLock::new(
                self.enum_generic_type.read().unwrap().clone(),
            )),
            enumeration: Arc::new(RwLock::new(self.enumeration.read().unwrap().clone())),
            enumeration_id_by_name: self.enumeration_id_by_name.clone(),
            expression: Arc::new(RwLock::new(self.expression.read().unwrap().clone())),
            expression_bit: Arc::new(RwLock::new(self.expression_bit.read().unwrap().clone())),
            expression_statement: Arc::new(RwLock::new(
                self.expression_statement.read().unwrap().clone(),
            )),
            external_implementation: Arc::new(RwLock::new(
                self.external_implementation.read().unwrap().clone(),
            )),
            field: Arc::new(RwLock::new(self.field.read().unwrap().clone())),
            field_id_by_name: self.field_id_by_name.clone(),
            field_access: Arc::new(RwLock::new(self.field_access.read().unwrap().clone())),
            field_access_target: Arc::new(RwLock::new(
                self.field_access_target.read().unwrap().clone(),
            )),
            field_expression: Arc::new(RwLock::new(self.field_expression.read().unwrap().clone())),
            float_literal: Arc::new(RwLock::new(self.float_literal.read().unwrap().clone())),
            for_loop: Arc::new(RwLock::new(self.for_loop.read().unwrap().clone())),
            format_bit: Arc::new(RwLock::new(self.format_bit.read().unwrap().clone())),
            format_string: Arc::new(RwLock::new(self.format_string.read().unwrap().clone())),
            func_generic: Arc::new(RwLock::new(self.func_generic.read().unwrap().clone())),
            function: Arc::new(RwLock::new(self.function.read().unwrap().clone())),
            function_id_by_name: self.function_id_by_name.clone(),
            function_call: Arc::new(RwLock::new(self.function_call.read().unwrap().clone())),
            x_future: Arc::new(RwLock::new(self.x_future.read().unwrap().clone())),
            grouped: Arc::new(RwLock::new(self.grouped.read().unwrap().clone())),
            halt_and_catch_fire: Arc::new(RwLock::new(
                self.halt_and_catch_fire.read().unwrap().clone(),
            )),
            x_if: Arc::new(RwLock::new(self.x_if.read().unwrap().clone())),
            implementation_block: Arc::new(RwLock::new(
                self.implementation_block.read().unwrap().clone(),
            )),
            import: Arc::new(RwLock::new(self.import.read().unwrap().clone())),
            index: Arc::new(RwLock::new(self.index.read().unwrap().clone())),
            integer_literal: Arc::new(RwLock::new(self.integer_literal.read().unwrap().clone())),
            item: Arc::new(RwLock::new(self.item.read().unwrap().clone())),
            lambda: Arc::new(RwLock::new(self.lambda.read().unwrap().clone())),
            lambda_parameter: Arc::new(RwLock::new(self.lambda_parameter.read().unwrap().clone())),
            let_statement: Arc::new(RwLock::new(self.let_statement.read().unwrap().clone())),
            list: Arc::new(RwLock::new(self.list.read().unwrap().clone())),
            list_element: Arc::new(RwLock::new(self.list_element.read().unwrap().clone())),
            list_expression: Arc::new(RwLock::new(self.list_expression.read().unwrap().clone())),
            literal: Arc::new(RwLock::new(self.literal.read().unwrap().clone())),
            local_variable: Arc::new(RwLock::new(self.local_variable.read().unwrap().clone())),
            x_macro: Arc::new(RwLock::new(self.x_macro.read().unwrap().clone())),
            map: Arc::new(RwLock::new(self.map.read().unwrap().clone())),
            map_element: Arc::new(RwLock::new(self.map_element.read().unwrap().clone())),
            map_expression: Arc::new(RwLock::new(self.map_expression.read().unwrap().clone())),
            x_match: Arc::new(RwLock::new(self.x_match.read().unwrap().clone())),
            method_call: Arc::new(RwLock::new(self.method_call.read().unwrap().clone())),
            named_field_expression: Arc::new(RwLock::new(
                self.named_field_expression.read().unwrap().clone(),
            )),
            z_object_store: Arc::new(RwLock::new(self.z_object_store.read().unwrap().clone())),
            z_object_store_id_by_name: self.z_object_store_id_by_name.clone(),
            object_wrapper: Arc::new(RwLock::new(self.object_wrapper.read().unwrap().clone())),
            operator: Arc::new(RwLock::new(self.operator.read().unwrap().clone())),
            parameter: Arc::new(RwLock::new(self.parameter.read().unwrap().clone())),
            x_path: Arc::new(RwLock::new(self.x_path.read().unwrap().clone())),
            path_element: Arc::new(RwLock::new(self.path_element.read().unwrap().clone())),
            pattern: Arc::new(RwLock::new(self.pattern.read().unwrap().clone())),
            x_plugin: Arc::new(RwLock::new(self.x_plugin.read().unwrap().clone())),
            x_plugin_id_by_name: self.x_plugin_id_by_name.clone(),
            x_print: Arc::new(RwLock::new(self.x_print.read().unwrap().clone())),
            range_expression: Arc::new(RwLock::new(self.range_expression.read().unwrap().clone())),
            result_statement: Arc::new(RwLock::new(self.result_statement.read().unwrap().clone())),
            x_return: Arc::new(RwLock::new(self.x_return.read().unwrap().clone())),
            span: Arc::new(RwLock::new(self.span.read().unwrap().clone())),
            statement: Arc::new(RwLock::new(self.statement.read().unwrap().clone())),
            static_method_call: Arc::new(RwLock::new(
                self.static_method_call.read().unwrap().clone(),
            )),
            string_bit: Arc::new(RwLock::new(self.string_bit.read().unwrap().clone())),
            string_literal: Arc::new(RwLock::new(self.string_literal.read().unwrap().clone())),
            woog_struct: Arc::new(RwLock::new(self.woog_struct.read().unwrap().clone())),
            woog_struct_id_by_name: self.woog_struct_id_by_name.clone(),
            struct_expression: Arc::new(RwLock::new(
                self.struct_expression.read().unwrap().clone(),
            )),
            struct_field: Arc::new(RwLock::new(self.struct_field.read().unwrap().clone())),
            struct_generic: Arc::new(RwLock::new(self.struct_generic.read().unwrap().clone())),
            tuple_field: Arc::new(RwLock::new(self.tuple_field.read().unwrap().clone())),
            type_cast: Arc::new(RwLock::new(self.type_cast.read().unwrap().clone())),
            unary: Arc::new(RwLock::new(self.unary.read().unwrap().clone())),
            unit: Arc::new(RwLock::new(self.unit.read().unwrap().clone())),
            unnamed_field_expression: Arc::new(RwLock::new(
                self.unnamed_field_expression.read().unwrap().clone(),
            )),
            x_value: Arc::new(RwLock::new(self.x_value.read().unwrap().clone())),
            value_type: Arc::new(RwLock::new(self.value_type.read().unwrap().clone())),
            variable: Arc::new(RwLock::new(self.variable.read().unwrap().clone())),
            variable_expression: Arc::new(RwLock::new(
                self.variable_expression.read().unwrap().clone(),
            )),
        }
    }
}
impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            argument: Arc::new(RwLock::new(HashMap::default())),
            a_wait: Arc::new(RwLock::new(HashMap::default())),
            binary: Arc::new(RwLock::new(HashMap::default())),
            block: Arc::new(RwLock::new(HashMap::default())),
            body: Arc::new(RwLock::new(HashMap::default())),
            boolean_literal: Arc::new(RwLock::new(HashMap::default())),
            boolean_operator: Arc::new(RwLock::new(HashMap::default())),
            call: Arc::new(RwLock::new(HashMap::default())),
            char_literal: Arc::new(RwLock::new(HashMap::default())),
            comparison: Arc::new(RwLock::new(HashMap::default())),
            data_structure: Arc::new(RwLock::new(HashMap::default())),
            dwarf_source_file: Arc::new(RwLock::new(HashMap::default())),
            enum_field: Arc::new(RwLock::new(HashMap::default())),
            enum_generic: Arc::new(RwLock::new(HashMap::default())),
            enum_generic_type: Arc::new(RwLock::new(HashMap::default())),
            enumeration: Arc::new(RwLock::new(HashMap::default())),
            enumeration_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            expression: Arc::new(RwLock::new(HashMap::default())),
            expression_bit: Arc::new(RwLock::new(HashMap::default())),
            expression_statement: Arc::new(RwLock::new(HashMap::default())),
            external_implementation: Arc::new(RwLock::new(HashMap::default())),
            field: Arc::new(RwLock::new(HashMap::default())),
            field_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            field_access: Arc::new(RwLock::new(HashMap::default())),
            field_access_target: Arc::new(RwLock::new(HashMap::default())),
            field_expression: Arc::new(RwLock::new(HashMap::default())),
            float_literal: Arc::new(RwLock::new(HashMap::default())),
            for_loop: Arc::new(RwLock::new(HashMap::default())),
            format_bit: Arc::new(RwLock::new(HashMap::default())),
            format_string: Arc::new(RwLock::new(HashMap::default())),
            func_generic: Arc::new(RwLock::new(HashMap::default())),
            function: Arc::new(RwLock::new(HashMap::default())),
            function_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            function_call: Arc::new(RwLock::new(HashMap::default())),
            x_future: Arc::new(RwLock::new(HashMap::default())),
            grouped: Arc::new(RwLock::new(HashMap::default())),
            halt_and_catch_fire: Arc::new(RwLock::new(HashMap::default())),
            x_if: Arc::new(RwLock::new(HashMap::default())),
            implementation_block: Arc::new(RwLock::new(HashMap::default())),
            import: Arc::new(RwLock::new(HashMap::default())),
            index: Arc::new(RwLock::new(HashMap::default())),
            integer_literal: Arc::new(RwLock::new(HashMap::default())),
            item: Arc::new(RwLock::new(HashMap::default())),
            lambda: Arc::new(RwLock::new(HashMap::default())),
            lambda_parameter: Arc::new(RwLock::new(HashMap::default())),
            let_statement: Arc::new(RwLock::new(HashMap::default())),
            list: Arc::new(RwLock::new(HashMap::default())),
            list_element: Arc::new(RwLock::new(HashMap::default())),
            list_expression: Arc::new(RwLock::new(HashMap::default())),
            literal: Arc::new(RwLock::new(HashMap::default())),
            local_variable: Arc::new(RwLock::new(HashMap::default())),
            x_macro: Arc::new(RwLock::new(HashMap::default())),
            map: Arc::new(RwLock::new(HashMap::default())),
            map_element: Arc::new(RwLock::new(HashMap::default())),
            map_expression: Arc::new(RwLock::new(HashMap::default())),
            x_match: Arc::new(RwLock::new(HashMap::default())),
            method_call: Arc::new(RwLock::new(HashMap::default())),
            named_field_expression: Arc::new(RwLock::new(HashMap::default())),
            z_object_store: Arc::new(RwLock::new(HashMap::default())),
            z_object_store_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            object_wrapper: Arc::new(RwLock::new(HashMap::default())),
            operator: Arc::new(RwLock::new(HashMap::default())),
            parameter: Arc::new(RwLock::new(HashMap::default())),
            x_path: Arc::new(RwLock::new(HashMap::default())),
            path_element: Arc::new(RwLock::new(HashMap::default())),
            pattern: Arc::new(RwLock::new(HashMap::default())),
            x_plugin: Arc::new(RwLock::new(HashMap::default())),
            x_plugin_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            x_print: Arc::new(RwLock::new(HashMap::default())),
            range_expression: Arc::new(RwLock::new(HashMap::default())),
            result_statement: Arc::new(RwLock::new(HashMap::default())),
            x_return: Arc::new(RwLock::new(HashMap::default())),
            span: Arc::new(RwLock::new(HashMap::default())),
            statement: Arc::new(RwLock::new(HashMap::default())),
            static_method_call: Arc::new(RwLock::new(HashMap::default())),
            string_bit: Arc::new(RwLock::new(HashMap::default())),
            string_literal: Arc::new(RwLock::new(HashMap::default())),
            woog_struct: Arc::new(RwLock::new(HashMap::default())),
            woog_struct_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            struct_expression: Arc::new(RwLock::new(HashMap::default())),
            struct_field: Arc::new(RwLock::new(HashMap::default())),
            struct_generic: Arc::new(RwLock::new(HashMap::default())),
            tuple_field: Arc::new(RwLock::new(HashMap::default())),
            type_cast: Arc::new(RwLock::new(HashMap::default())),
            unary: Arc::new(RwLock::new(HashMap::default())),
            unit: Arc::new(RwLock::new(HashMap::default())),
            unnamed_field_expression: Arc::new(RwLock::new(HashMap::default())),
            x_value: Arc::new(RwLock::new(HashMap::default())),
            value_type: Arc::new(RwLock::new(HashMap::default())),
            variable: Arc::new(RwLock::new(HashMap::default())),
            variable_expression: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    pub fn merge(&mut self, other: &ObjectStore) {
        self.argument.write().unwrap().extend(
            other
                .argument
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.a_wait.write().unwrap().extend(
            other
                .a_wait
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.binary.write().unwrap().extend(
            other
                .binary
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.block.write().unwrap().extend(
            other
                .block
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.body.write().unwrap().extend(
            other
                .body
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.boolean_literal.write().unwrap().extend(
            other
                .boolean_literal
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.boolean_operator.write().unwrap().extend(
            other
                .boolean_operator
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.call.write().unwrap().extend(
            other
                .call
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.char_literal.write().unwrap().extend(
            other
                .char_literal
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.comparison.write().unwrap().extend(
            other
                .comparison
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.data_structure.write().unwrap().extend(
            other
                .data_structure
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.dwarf_source_file.write().unwrap().extend(
            other
                .dwarf_source_file
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enum_field.write().unwrap().extend(
            other
                .enum_field
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enum_generic.write().unwrap().extend(
            other
                .enum_generic
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enum_generic_type.write().unwrap().extend(
            other
                .enum_generic_type
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enumeration.write().unwrap().extend(
            other
                .enumeration
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enumeration_id_by_name.write().unwrap().extend(
            other
                .enumeration_id_by_name
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.expression.write().unwrap().extend(
            other
                .expression
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.expression_bit.write().unwrap().extend(
            other
                .expression_bit
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.expression_statement.write().unwrap().extend(
            other
                .expression_statement
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.external_implementation.write().unwrap().extend(
            other
                .external_implementation
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.field.write().unwrap().extend(
            other
                .field
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.field_id_by_name.write().unwrap().extend(
            other
                .field_id_by_name
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.field_access.write().unwrap().extend(
            other
                .field_access
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.field_access_target.write().unwrap().extend(
            other
                .field_access_target
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.field_expression.write().unwrap().extend(
            other
                .field_expression
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.float_literal.write().unwrap().extend(
            other
                .float_literal
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.for_loop.write().unwrap().extend(
            other
                .for_loop
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.format_bit.write().unwrap().extend(
            other
                .format_bit
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.format_string.write().unwrap().extend(
            other
                .format_string
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.func_generic.write().unwrap().extend(
            other
                .func_generic
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.function.write().unwrap().extend(
            other
                .function
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.function_id_by_name.write().unwrap().extend(
            other
                .function_id_by_name
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.function_call.write().unwrap().extend(
            other
                .function_call
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_future.write().unwrap().extend(
            other
                .x_future
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.grouped.write().unwrap().extend(
            other
                .grouped
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.halt_and_catch_fire.write().unwrap().extend(
            other
                .halt_and_catch_fire
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_if.write().unwrap().extend(
            other
                .x_if
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.implementation_block.write().unwrap().extend(
            other
                .implementation_block
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.import.write().unwrap().extend(
            other
                .import
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.index.write().unwrap().extend(
            other
                .index
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.integer_literal.write().unwrap().extend(
            other
                .integer_literal
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.item.write().unwrap().extend(
            other
                .item
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.lambda.write().unwrap().extend(
            other
                .lambda
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.lambda_parameter.write().unwrap().extend(
            other
                .lambda_parameter
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.let_statement.write().unwrap().extend(
            other
                .let_statement
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.list.write().unwrap().extend(
            other
                .list
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.list_element.write().unwrap().extend(
            other
                .list_element
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.list_expression.write().unwrap().extend(
            other
                .list_expression
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.literal.write().unwrap().extend(
            other
                .literal
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.local_variable.write().unwrap().extend(
            other
                .local_variable
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_macro.write().unwrap().extend(
            other
                .x_macro
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.map.write().unwrap().extend(
            other
                .map
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.map_element.write().unwrap().extend(
            other
                .map_element
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.map_expression.write().unwrap().extend(
            other
                .map_expression
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_match.write().unwrap().extend(
            other
                .x_match
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.method_call.write().unwrap().extend(
            other
                .method_call
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.named_field_expression.write().unwrap().extend(
            other
                .named_field_expression
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.z_object_store.write().unwrap().extend(
            other
                .z_object_store
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.z_object_store_id_by_name.write().unwrap().extend(
            other
                .z_object_store_id_by_name
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.object_wrapper.write().unwrap().extend(
            other
                .object_wrapper
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.operator.write().unwrap().extend(
            other
                .operator
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.parameter.write().unwrap().extend(
            other
                .parameter
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_path.write().unwrap().extend(
            other
                .x_path
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.path_element.write().unwrap().extend(
            other
                .path_element
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.pattern.write().unwrap().extend(
            other
                .pattern
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_plugin.write().unwrap().extend(
            other
                .x_plugin
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_plugin_id_by_name.write().unwrap().extend(
            other
                .x_plugin_id_by_name
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.x_print.write().unwrap().extend(
            other
                .x_print
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.range_expression.write().unwrap().extend(
            other
                .range_expression
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.result_statement.write().unwrap().extend(
            other
                .result_statement
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_return.write().unwrap().extend(
            other
                .x_return
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.span.write().unwrap().extend(
            other
                .span
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.statement.write().unwrap().extend(
            other
                .statement
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.static_method_call.write().unwrap().extend(
            other
                .static_method_call
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.string_bit.write().unwrap().extend(
            other
                .string_bit
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.string_literal.write().unwrap().extend(
            other
                .string_literal
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.woog_struct.write().unwrap().extend(
            other
                .woog_struct
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.woog_struct_id_by_name.write().unwrap().extend(
            other
                .woog_struct_id_by_name
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.struct_expression.write().unwrap().extend(
            other
                .struct_expression
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.struct_field.write().unwrap().extend(
            other
                .struct_field
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.struct_generic.write().unwrap().extend(
            other
                .struct_generic
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.tuple_field.write().unwrap().extend(
            other
                .tuple_field
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.type_cast.write().unwrap().extend(
            other
                .type_cast
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.unary.write().unwrap().extend(
            other
                .unary
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.unit.write().unwrap().extend(
            other
                .unit
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.unnamed_field_expression.write().unwrap().extend(
            other
                .unnamed_field_expression
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_value.write().unwrap().extend(
            other
                .x_value
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.value_type.write().unwrap().extend(
            other
                .value_type
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.variable.write().unwrap().extend(
            other
                .variable
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.variable_expression.write().unwrap().extend(
            other
                .variable_expression
                .read()
                .unwrap()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
    }
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument(&mut self, argument: Arc<RwLock<Argument>>) {
        let read = argument.read().unwrap();
        self.argument
            .write()
            .unwrap()
            .insert(read.id, argument.clone());
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &Uuid) -> Option<Arc<RwLock<Argument>>> {
        self.argument
            .read()
            .unwrap()
            .get(id)
            .map(|argument| argument.clone())
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub fn exorcise_argument(&mut self, id: &Uuid) -> Option<Arc<RwLock<Argument>>> {
        self.argument
            .write()
            .unwrap()
            .remove(id)
            .map(|argument| argument.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = Arc<RwLock<Argument>>> + '_ {
        let values: Vec<Arc<RwLock<Argument>>> = self
            .argument
            .read()
            .unwrap()
            .values()
            .map(|argument| argument.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`AWait`] into the store.
    ///
    pub fn inter_a_wait(&mut self, a_wait: Arc<RwLock<AWait>>) {
        let read = a_wait.read().unwrap();
        self.a_wait.write().unwrap().insert(read.id, a_wait.clone());
    }

    /// Exhume (get) [`AWait`] from the store.
    ///
    pub fn exhume_a_wait(&self, id: &Uuid) -> Option<Arc<RwLock<AWait>>> {
        self.a_wait
            .read()
            .unwrap()
            .get(id)
            .map(|a_wait| a_wait.clone())
    }

    /// Exorcise (remove) [`AWait`] from the store.
    ///
    pub fn exorcise_a_wait(&mut self, id: &Uuid) -> Option<Arc<RwLock<AWait>>> {
        self.a_wait
            .write()
            .unwrap()
            .remove(id)
            .map(|a_wait| a_wait.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AWait>`.
    ///
    pub fn iter_a_wait(&self) -> impl Iterator<Item = Arc<RwLock<AWait>>> + '_ {
        let values: Vec<Arc<RwLock<AWait>>> = self
            .a_wait
            .read()
            .unwrap()
            .values()
            .map(|a_wait| a_wait.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Arc<RwLock<Binary>>) {
        let read = binary.read().unwrap();
        self.binary.write().unwrap().insert(read.id, binary.clone());
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<Arc<RwLock<Binary>>> {
        self.binary
            .read()
            .unwrap()
            .get(id)
            .map(|binary| binary.clone())
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &Uuid) -> Option<Arc<RwLock<Binary>>> {
        self.binary
            .write()
            .unwrap()
            .remove(id)
            .map(|binary| binary.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = Arc<RwLock<Binary>>> + '_ {
        let values: Vec<Arc<RwLock<Binary>>> = self
            .binary
            .read()
            .unwrap()
            .values()
            .map(|binary| binary.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Arc<RwLock<Block>>) {
        let read = block.read().unwrap();
        self.block.write().unwrap().insert(read.id, block.clone());
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<Arc<RwLock<Block>>> {
        self.block
            .read()
            .unwrap()
            .get(id)
            .map(|block| block.clone())
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &Uuid) -> Option<Arc<RwLock<Block>>> {
        self.block
            .write()
            .unwrap()
            .remove(id)
            .map(|block| block.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = Arc<RwLock<Block>>> + '_ {
        let values: Vec<Arc<RwLock<Block>>> = self
            .block
            .read()
            .unwrap()
            .values()
            .map(|block| block.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Body`] into the store.
    ///
    pub fn inter_body(&mut self, body: Arc<RwLock<Body>>) {
        let read = body.read().unwrap();
        self.body.write().unwrap().insert(read.id, body.clone());
    }

    /// Exhume (get) [`Body`] from the store.
    ///
    pub fn exhume_body(&self, id: &Uuid) -> Option<Arc<RwLock<Body>>> {
        self.body.read().unwrap().get(id).map(|body| body.clone())
    }

    /// Exorcise (remove) [`Body`] from the store.
    ///
    pub fn exorcise_body(&mut self, id: &Uuid) -> Option<Arc<RwLock<Body>>> {
        self.body
            .write()
            .unwrap()
            .remove(id)
            .map(|body| body.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Body>`.
    ///
    pub fn iter_body(&self) -> impl Iterator<Item = Arc<RwLock<Body>>> + '_ {
        let values: Vec<Arc<RwLock<Body>>> = self
            .body
            .read()
            .unwrap()
            .values()
            .map(|body| body.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal(&mut self, boolean_literal: Arc<RwLock<BooleanLiteral>>) {
        let read = boolean_literal.read().unwrap();
        self.boolean_literal
            .write()
            .unwrap()
            .insert(read.id, boolean_literal.clone());
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &Uuid) -> Option<Arc<RwLock<BooleanLiteral>>> {
        self.boolean_literal
            .read()
            .unwrap()
            .get(id)
            .map(|boolean_literal| boolean_literal.clone())
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub fn exorcise_boolean_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<BooleanLiteral>>> {
        self.boolean_literal
            .write()
            .unwrap()
            .remove(id)
            .map(|boolean_literal| boolean_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Arc<RwLock<BooleanLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<BooleanLiteral>>> = self
            .boolean_literal
            .read()
            .unwrap()
            .values()
            .map(|boolean_literal| boolean_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub fn inter_boolean_operator(&mut self, boolean_operator: Arc<RwLock<BooleanOperator>>) {
        let read = boolean_operator.read().unwrap();
        self.boolean_operator
            .write()
            .unwrap()
            .insert(read.id, boolean_operator.clone());
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub fn exhume_boolean_operator(&self, id: &Uuid) -> Option<Arc<RwLock<BooleanOperator>>> {
        self.boolean_operator
            .read()
            .unwrap()
            .get(id)
            .map(|boolean_operator| boolean_operator.clone())
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub fn exorcise_boolean_operator(&mut self, id: &Uuid) -> Option<Arc<RwLock<BooleanOperator>>> {
        self.boolean_operator
            .write()
            .unwrap()
            .remove(id)
            .map(|boolean_operator| boolean_operator.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = Arc<RwLock<BooleanOperator>>> + '_ {
        let values: Vec<Arc<RwLock<BooleanOperator>>> = self
            .boolean_operator
            .read()
            .unwrap()
            .values()
            .map(|boolean_operator| boolean_operator.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Arc<RwLock<Call>>) {
        let read = call.read().unwrap();
        self.call.write().unwrap().insert(read.id, call.clone());
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<Arc<RwLock<Call>>> {
        self.call.read().unwrap().get(id).map(|call| call.clone())
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &Uuid) -> Option<Arc<RwLock<Call>>> {
        self.call
            .write()
            .unwrap()
            .remove(id)
            .map(|call| call.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = Arc<RwLock<Call>>> + '_ {
        let values: Vec<Arc<RwLock<Call>>> = self
            .call
            .read()
            .unwrap()
            .values()
            .map(|call| call.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`CharLiteral`] into the store.
    ///
    pub fn inter_char_literal(&mut self, char_literal: Arc<RwLock<CharLiteral>>) {
        let read = char_literal.read().unwrap();
        self.char_literal
            .write()
            .unwrap()
            .insert(read.id, char_literal.clone());
    }

    /// Exhume (get) [`CharLiteral`] from the store.
    ///
    pub fn exhume_char_literal(&self, id: &Uuid) -> Option<Arc<RwLock<CharLiteral>>> {
        self.char_literal
            .read()
            .unwrap()
            .get(id)
            .map(|char_literal| char_literal.clone())
    }

    /// Exorcise (remove) [`CharLiteral`] from the store.
    ///
    pub fn exorcise_char_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<CharLiteral>>> {
        self.char_literal
            .write()
            .unwrap()
            .remove(id)
            .map(|char_literal| char_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, CharLiteral>`.
    ///
    pub fn iter_char_literal(&self) -> impl Iterator<Item = Arc<RwLock<CharLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<CharLiteral>>> = self
            .char_literal
            .read()
            .unwrap()
            .values()
            .map(|char_literal| char_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub fn inter_comparison(&mut self, comparison: Arc<RwLock<Comparison>>) {
        let read = comparison.read().unwrap();
        self.comparison
            .write()
            .unwrap()
            .insert(read.id, comparison.clone());
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub fn exhume_comparison(&self, id: &Uuid) -> Option<Arc<RwLock<Comparison>>> {
        self.comparison
            .read()
            .unwrap()
            .get(id)
            .map(|comparison| comparison.clone())
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub fn exorcise_comparison(&mut self, id: &Uuid) -> Option<Arc<RwLock<Comparison>>> {
        self.comparison
            .write()
            .unwrap()
            .remove(id)
            .map(|comparison| comparison.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub fn iter_comparison(&self) -> impl Iterator<Item = Arc<RwLock<Comparison>>> + '_ {
        let values: Vec<Arc<RwLock<Comparison>>> = self
            .comparison
            .read()
            .unwrap()
            .values()
            .map(|comparison| comparison.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`DataStructure`] into the store.
    ///
    pub fn inter_data_structure(&mut self, data_structure: Arc<RwLock<DataStructure>>) {
        let read = data_structure.read().unwrap();
        self.data_structure
            .write()
            .unwrap()
            .insert(read.id, data_structure.clone());
    }

    /// Exhume (get) [`DataStructure`] from the store.
    ///
    pub fn exhume_data_structure(&self, id: &Uuid) -> Option<Arc<RwLock<DataStructure>>> {
        self.data_structure
            .read()
            .unwrap()
            .get(id)
            .map(|data_structure| data_structure.clone())
    }

    /// Exorcise (remove) [`DataStructure`] from the store.
    ///
    pub fn exorcise_data_structure(&mut self, id: &Uuid) -> Option<Arc<RwLock<DataStructure>>> {
        self.data_structure
            .write()
            .unwrap()
            .remove(id)
            .map(|data_structure| data_structure.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DataStructure>`.
    ///
    pub fn iter_data_structure(&self) -> impl Iterator<Item = Arc<RwLock<DataStructure>>> + '_ {
        let values: Vec<Arc<RwLock<DataStructure>>> = self
            .data_structure
            .read()
            .unwrap()
            .values()
            .map(|data_structure| data_structure.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub fn inter_dwarf_source_file(&mut self, dwarf_source_file: Arc<RwLock<DwarfSourceFile>>) {
        let read = dwarf_source_file.read().unwrap();
        self.dwarf_source_file
            .write()
            .unwrap()
            .insert(read.id, dwarf_source_file.clone());
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &Uuid) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        self.dwarf_source_file
            .read()
            .unwrap()
            .get(id)
            .map(|dwarf_source_file| dwarf_source_file.clone())
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub fn exorcise_dwarf_source_file(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        self.dwarf_source_file
            .write()
            .unwrap()
            .remove(id)
            .map(|dwarf_source_file| dwarf_source_file.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<DwarfSourceFile>>> + '_ {
        let values: Vec<Arc<RwLock<DwarfSourceFile>>> = self
            .dwarf_source_file
            .read()
            .unwrap()
            .values()
            .map(|dwarf_source_file| dwarf_source_file.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`EnumField`] into the store.
    ///
    pub fn inter_enum_field(&mut self, enum_field: Arc<RwLock<EnumField>>) {
        let read = enum_field.read().unwrap();
        self.enum_field
            .write()
            .unwrap()
            .insert(read.id, enum_field.clone());
    }

    /// Exhume (get) [`EnumField`] from the store.
    ///
    pub fn exhume_enum_field(&self, id: &Uuid) -> Option<Arc<RwLock<EnumField>>> {
        self.enum_field
            .read()
            .unwrap()
            .get(id)
            .map(|enum_field| enum_field.clone())
    }

    /// Exorcise (remove) [`EnumField`] from the store.
    ///
    pub fn exorcise_enum_field(&mut self, id: &Uuid) -> Option<Arc<RwLock<EnumField>>> {
        self.enum_field
            .write()
            .unwrap()
            .remove(id)
            .map(|enum_field| enum_field.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumField>`.
    ///
    pub fn iter_enum_field(&self) -> impl Iterator<Item = Arc<RwLock<EnumField>>> + '_ {
        let values: Vec<Arc<RwLock<EnumField>>> = self
            .enum_field
            .read()
            .unwrap()
            .values()
            .map(|enum_field| enum_field.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`EnumGeneric`] into the store.
    ///
    pub fn inter_enum_generic(&mut self, enum_generic: Arc<RwLock<EnumGeneric>>) {
        let read = enum_generic.read().unwrap();
        self.enum_generic
            .write()
            .unwrap()
            .insert(read.id, enum_generic.clone());
    }

    /// Exhume (get) [`EnumGeneric`] from the store.
    ///
    pub fn exhume_enum_generic(&self, id: &Uuid) -> Option<Arc<RwLock<EnumGeneric>>> {
        self.enum_generic
            .read()
            .unwrap()
            .get(id)
            .map(|enum_generic| enum_generic.clone())
    }

    /// Exorcise (remove) [`EnumGeneric`] from the store.
    ///
    pub fn exorcise_enum_generic(&mut self, id: &Uuid) -> Option<Arc<RwLock<EnumGeneric>>> {
        self.enum_generic
            .write()
            .unwrap()
            .remove(id)
            .map(|enum_generic| enum_generic.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGeneric>`.
    ///
    pub fn iter_enum_generic(&self) -> impl Iterator<Item = Arc<RwLock<EnumGeneric>>> + '_ {
        let values: Vec<Arc<RwLock<EnumGeneric>>> = self
            .enum_generic
            .read()
            .unwrap()
            .values()
            .map(|enum_generic| enum_generic.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`EnumGenericType`] into the store.
    ///
    pub fn inter_enum_generic_type(&mut self, enum_generic_type: Arc<RwLock<EnumGenericType>>) {
        let read = enum_generic_type.read().unwrap();
        self.enum_generic_type
            .write()
            .unwrap()
            .insert(read.id, enum_generic_type.clone());
    }

    /// Exhume (get) [`EnumGenericType`] from the store.
    ///
    pub fn exhume_enum_generic_type(&self, id: &Uuid) -> Option<Arc<RwLock<EnumGenericType>>> {
        self.enum_generic_type
            .read()
            .unwrap()
            .get(id)
            .map(|enum_generic_type| enum_generic_type.clone())
    }

    /// Exorcise (remove) [`EnumGenericType`] from the store.
    ///
    pub fn exorcise_enum_generic_type(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<EnumGenericType>>> {
        self.enum_generic_type
            .write()
            .unwrap()
            .remove(id)
            .map(|enum_generic_type| enum_generic_type.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGenericType>`.
    ///
    pub fn iter_enum_generic_type(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<EnumGenericType>>> + '_ {
        let values: Vec<Arc<RwLock<EnumGenericType>>> = self
            .enum_generic_type
            .read()
            .unwrap()
            .values()
            .map(|enum_generic_type| enum_generic_type.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    pub fn inter_enumeration(&mut self, enumeration: Arc<RwLock<Enumeration>>) {
        let read = enumeration.read().unwrap();
        self.enumeration_id_by_name
            .write()
            .unwrap()
            .insert(read.name.clone(), read.id);
        self.enumeration
            .write()
            .unwrap()
            .insert(read.id, enumeration.clone());
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    pub fn exhume_enumeration(&self, id: &Uuid) -> Option<Arc<RwLock<Enumeration>>> {
        self.enumeration
            .read()
            .unwrap()
            .get(id)
            .map(|enumeration| enumeration.clone())
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    pub fn exorcise_enumeration(&mut self, id: &Uuid) -> Option<Arc<RwLock<Enumeration>>> {
        self.enumeration
            .write()
            .unwrap()
            .remove(id)
            .map(|enumeration| enumeration.clone())
    }

    /// Exhume [`Enumeration`] id from the store by name.
    ///
    pub fn exhume_enumeration_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.enumeration_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|enumeration| *enumeration)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    pub fn iter_enumeration(&self) -> impl Iterator<Item = Arc<RwLock<Enumeration>>> + '_ {
        let values: Vec<Arc<RwLock<Enumeration>>> = self
            .enumeration
            .read()
            .unwrap()
            .values()
            .map(|enumeration| enumeration.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Arc<RwLock<Expression>>) {
        let read = expression.read().unwrap();
        self.expression
            .write()
            .unwrap()
            .insert(read.id, expression.clone());
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<Arc<RwLock<Expression>>> {
        self.expression
            .read()
            .unwrap()
            .get(id)
            .map(|expression| expression.clone())
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<Expression>>> {
        self.expression
            .write()
            .unwrap()
            .remove(id)
            .map(|expression| expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = Arc<RwLock<Expression>>> + '_ {
        let values: Vec<Arc<RwLock<Expression>>> = self
            .expression
            .read()
            .unwrap()
            .values()
            .map(|expression| expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ExpressionBit`] into the store.
    ///
    pub fn inter_expression_bit(&mut self, expression_bit: Arc<RwLock<ExpressionBit>>) {
        let read = expression_bit.read().unwrap();
        self.expression_bit
            .write()
            .unwrap()
            .insert(read.id, expression_bit.clone());
    }

    /// Exhume (get) [`ExpressionBit`] from the store.
    ///
    pub fn exhume_expression_bit(&self, id: &Uuid) -> Option<Arc<RwLock<ExpressionBit>>> {
        self.expression_bit
            .read()
            .unwrap()
            .get(id)
            .map(|expression_bit| expression_bit.clone())
    }

    /// Exorcise (remove) [`ExpressionBit`] from the store.
    ///
    pub fn exorcise_expression_bit(&mut self, id: &Uuid) -> Option<Arc<RwLock<ExpressionBit>>> {
        self.expression_bit
            .write()
            .unwrap()
            .remove(id)
            .map(|expression_bit| expression_bit.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionBit>`.
    ///
    pub fn iter_expression_bit(&self) -> impl Iterator<Item = Arc<RwLock<ExpressionBit>>> + '_ {
        let values: Vec<Arc<RwLock<ExpressionBit>>> = self
            .expression_bit
            .read()
            .unwrap()
            .values()
            .map(|expression_bit| expression_bit.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    pub fn inter_expression_statement(
        &mut self,
        expression_statement: Arc<RwLock<ExpressionStatement>>,
    ) {
        let read = expression_statement.read().unwrap();
        self.expression_statement
            .write()
            .unwrap()
            .insert(read.id, expression_statement.clone());
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        self.expression_statement
            .read()
            .unwrap()
            .get(id)
            .map(|expression_statement| expression_statement.clone())
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub fn exorcise_expression_statement(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        self.expression_statement
            .write()
            .unwrap()
            .remove(id)
            .map(|expression_statement| expression_statement.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ExpressionStatement>>> + '_ {
        let values: Vec<Arc<RwLock<ExpressionStatement>>> = self
            .expression_statement
            .read()
            .unwrap()
            .values()
            .map(|expression_statement| expression_statement.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ExternalImplementation`] into the store.
    ///
    pub fn inter_external_implementation(
        &mut self,
        external_implementation: Arc<RwLock<ExternalImplementation>>,
    ) {
        let read = external_implementation.read().unwrap();
        self.external_implementation
            .write()
            .unwrap()
            .insert(read.id, external_implementation.clone());
    }

    /// Exhume (get) [`ExternalImplementation`] from the store.
    ///
    pub fn exhume_external_implementation(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ExternalImplementation>>> {
        self.external_implementation
            .read()
            .unwrap()
            .get(id)
            .map(|external_implementation| external_implementation.clone())
    }

    /// Exorcise (remove) [`ExternalImplementation`] from the store.
    ///
    pub fn exorcise_external_implementation(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ExternalImplementation>>> {
        self.external_implementation
            .write()
            .unwrap()
            .remove(id)
            .map(|external_implementation| external_implementation.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExternalImplementation>`.
    ///
    pub fn iter_external_implementation(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ExternalImplementation>>> + '_ {
        let values: Vec<Arc<RwLock<ExternalImplementation>>> = self
            .external_implementation
            .read()
            .unwrap()
            .values()
            .map(|external_implementation| external_implementation.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Arc<RwLock<Field>>) {
        let read = field.read().unwrap();
        self.field_id_by_name
            .write()
            .unwrap()
            .insert(read.name.clone(), read.id);
        self.field.write().unwrap().insert(read.id, field.clone());
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<Arc<RwLock<Field>>> {
        self.field
            .read()
            .unwrap()
            .get(id)
            .map(|field| field.clone())
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &Uuid) -> Option<Arc<RwLock<Field>>> {
        self.field
            .write()
            .unwrap()
            .remove(id)
            .map(|field| field.clone())
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.field_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|field| *field)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = Arc<RwLock<Field>>> + '_ {
        let values: Vec<Arc<RwLock<Field>>> = self
            .field
            .read()
            .unwrap()
            .values()
            .map(|field| field.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access(&mut self, field_access: Arc<RwLock<FieldAccess>>) {
        let read = field_access.read().unwrap();
        self.field_access
            .write()
            .unwrap()
            .insert(read.id, field_access.clone());
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &Uuid) -> Option<Arc<RwLock<FieldAccess>>> {
        self.field_access
            .read()
            .unwrap()
            .get(id)
            .map(|field_access| field_access.clone())
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub fn exorcise_field_access(&mut self, id: &Uuid) -> Option<Arc<RwLock<FieldAccess>>> {
        self.field_access
            .write()
            .unwrap()
            .remove(id)
            .map(|field_access| field_access.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = Arc<RwLock<FieldAccess>>> + '_ {
        let values: Vec<Arc<RwLock<FieldAccess>>> = self
            .field_access
            .read()
            .unwrap()
            .values()
            .map(|field_access| field_access.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    pub fn inter_field_access_target(
        &mut self,
        field_access_target: Arc<RwLock<FieldAccessTarget>>,
    ) {
        let read = field_access_target.read().unwrap();
        self.field_access_target
            .write()
            .unwrap()
            .insert(read.id, field_access_target.clone());
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub fn exhume_field_access_target(&self, id: &Uuid) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        self.field_access_target
            .read()
            .unwrap()
            .get(id)
            .map(|field_access_target| field_access_target.clone())
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub fn exorcise_field_access_target(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        self.field_access_target
            .write()
            .unwrap()
            .remove(id)
            .map(|field_access_target| field_access_target.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<FieldAccessTarget>>> + '_ {
        let values: Vec<Arc<RwLock<FieldAccessTarget>>> = self
            .field_access_target
            .read()
            .unwrap()
            .values()
            .map(|field_access_target| field_access_target.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression(&mut self, field_expression: Arc<RwLock<FieldExpression>>) {
        let read = field_expression.read().unwrap();
        self.field_expression
            .write()
            .unwrap()
            .insert(read.id, field_expression.clone());
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &Uuid) -> Option<Arc<RwLock<FieldExpression>>> {
        self.field_expression
            .read()
            .unwrap()
            .get(id)
            .map(|field_expression| field_expression.clone())
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub fn exorcise_field_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<FieldExpression>>> {
        self.field_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|field_expression| field_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Arc<RwLock<FieldExpression>>> + '_ {
        let values: Vec<Arc<RwLock<FieldExpression>>> = self
            .field_expression
            .read()
            .unwrap()
            .values()
            .map(|field_expression| field_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal(&mut self, float_literal: Arc<RwLock<FloatLiteral>>) {
        let read = float_literal.read().unwrap();
        self.float_literal
            .write()
            .unwrap()
            .insert(read.id, float_literal.clone());
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &Uuid) -> Option<Arc<RwLock<FloatLiteral>>> {
        self.float_literal
            .read()
            .unwrap()
            .get(id)
            .map(|float_literal| float_literal.clone())
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub fn exorcise_float_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<FloatLiteral>>> {
        self.float_literal
            .write()
            .unwrap()
            .remove(id)
            .map(|float_literal| float_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Arc<RwLock<FloatLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<FloatLiteral>>> = self
            .float_literal
            .read()
            .unwrap()
            .values()
            .map(|float_literal| float_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub fn inter_for_loop(&mut self, for_loop: Arc<RwLock<ForLoop>>) {
        let read = for_loop.read().unwrap();
        self.for_loop
            .write()
            .unwrap()
            .insert(read.id, for_loop.clone());
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub fn exhume_for_loop(&self, id: &Uuid) -> Option<Arc<RwLock<ForLoop>>> {
        self.for_loop
            .read()
            .unwrap()
            .get(id)
            .map(|for_loop| for_loop.clone())
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub fn exorcise_for_loop(&mut self, id: &Uuid) -> Option<Arc<RwLock<ForLoop>>> {
        self.for_loop
            .write()
            .unwrap()
            .remove(id)
            .map(|for_loop| for_loop.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub fn iter_for_loop(&self) -> impl Iterator<Item = Arc<RwLock<ForLoop>>> + '_ {
        let values: Vec<Arc<RwLock<ForLoop>>> = self
            .for_loop
            .read()
            .unwrap()
            .values()
            .map(|for_loop| for_loop.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FormatBit`] into the store.
    ///
    pub fn inter_format_bit(&mut self, format_bit: Arc<RwLock<FormatBit>>) {
        let read = format_bit.read().unwrap();
        self.format_bit
            .write()
            .unwrap()
            .insert(read.id, format_bit.clone());
    }

    /// Exhume (get) [`FormatBit`] from the store.
    ///
    pub fn exhume_format_bit(&self, id: &Uuid) -> Option<Arc<RwLock<FormatBit>>> {
        self.format_bit
            .read()
            .unwrap()
            .get(id)
            .map(|format_bit| format_bit.clone())
    }

    /// Exorcise (remove) [`FormatBit`] from the store.
    ///
    pub fn exorcise_format_bit(&mut self, id: &Uuid) -> Option<Arc<RwLock<FormatBit>>> {
        self.format_bit
            .write()
            .unwrap()
            .remove(id)
            .map(|format_bit| format_bit.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FormatBit>`.
    ///
    pub fn iter_format_bit(&self) -> impl Iterator<Item = Arc<RwLock<FormatBit>>> + '_ {
        let values: Vec<Arc<RwLock<FormatBit>>> = self
            .format_bit
            .read()
            .unwrap()
            .values()
            .map(|format_bit| format_bit.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FormatString`] into the store.
    ///
    pub fn inter_format_string(&mut self, format_string: Arc<RwLock<FormatString>>) {
        let read = format_string.read().unwrap();
        self.format_string
            .write()
            .unwrap()
            .insert(read.id, format_string.clone());
    }

    /// Exhume (get) [`FormatString`] from the store.
    ///
    pub fn exhume_format_string(&self, id: &Uuid) -> Option<Arc<RwLock<FormatString>>> {
        self.format_string
            .read()
            .unwrap()
            .get(id)
            .map(|format_string| format_string.clone())
    }

    /// Exorcise (remove) [`FormatString`] from the store.
    ///
    pub fn exorcise_format_string(&mut self, id: &Uuid) -> Option<Arc<RwLock<FormatString>>> {
        self.format_string
            .write()
            .unwrap()
            .remove(id)
            .map(|format_string| format_string.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FormatString>`.
    ///
    pub fn iter_format_string(&self) -> impl Iterator<Item = Arc<RwLock<FormatString>>> + '_ {
        let values: Vec<Arc<RwLock<FormatString>>> = self
            .format_string
            .read()
            .unwrap()
            .values()
            .map(|format_string| format_string.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FuncGeneric`] into the store.
    ///
    pub fn inter_func_generic(&mut self, func_generic: Arc<RwLock<FuncGeneric>>) {
        let read = func_generic.read().unwrap();
        self.func_generic
            .write()
            .unwrap()
            .insert(read.id, func_generic.clone());
    }

    /// Exhume (get) [`FuncGeneric`] from the store.
    ///
    pub fn exhume_func_generic(&self, id: &Uuid) -> Option<Arc<RwLock<FuncGeneric>>> {
        self.func_generic
            .read()
            .unwrap()
            .get(id)
            .map(|func_generic| func_generic.clone())
    }

    /// Exorcise (remove) [`FuncGeneric`] from the store.
    ///
    pub fn exorcise_func_generic(&mut self, id: &Uuid) -> Option<Arc<RwLock<FuncGeneric>>> {
        self.func_generic
            .write()
            .unwrap()
            .remove(id)
            .map(|func_generic| func_generic.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FuncGeneric>`.
    ///
    pub fn iter_func_generic(&self) -> impl Iterator<Item = Arc<RwLock<FuncGeneric>>> + '_ {
        let values: Vec<Arc<RwLock<FuncGeneric>>> = self
            .func_generic
            .read()
            .unwrap()
            .values()
            .map(|func_generic| func_generic.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Arc<RwLock<Function>>) {
        let read = function.read().unwrap();
        self.function_id_by_name
            .write()
            .unwrap()
            .insert(read.name.clone(), read.id);
        self.function
            .write()
            .unwrap()
            .insert(read.id, function.clone());
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<Arc<RwLock<Function>>> {
        self.function
            .read()
            .unwrap()
            .get(id)
            .map(|function| function.clone())
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &Uuid) -> Option<Arc<RwLock<Function>>> {
        self.function
            .write()
            .unwrap()
            .remove(id)
            .map(|function| function.clone())
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.function_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|function| *function)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = Arc<RwLock<Function>>> + '_ {
        let values: Vec<Arc<RwLock<Function>>> = self
            .function
            .read()
            .unwrap()
            .values()
            .map(|function| function.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FunctionCall`] into the store.
    ///
    pub fn inter_function_call(&mut self, function_call: Arc<RwLock<FunctionCall>>) {
        let read = function_call.read().unwrap();
        self.function_call
            .write()
            .unwrap()
            .insert(read.id, function_call.clone());
    }

    /// Exhume (get) [`FunctionCall`] from the store.
    ///
    pub fn exhume_function_call(&self, id: &Uuid) -> Option<Arc<RwLock<FunctionCall>>> {
        self.function_call
            .read()
            .unwrap()
            .get(id)
            .map(|function_call| function_call.clone())
    }

    /// Exorcise (remove) [`FunctionCall`] from the store.
    ///
    pub fn exorcise_function_call(&mut self, id: &Uuid) -> Option<Arc<RwLock<FunctionCall>>> {
        self.function_call
            .write()
            .unwrap()
            .remove(id)
            .map(|function_call| function_call.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FunctionCall>`.
    ///
    pub fn iter_function_call(&self) -> impl Iterator<Item = Arc<RwLock<FunctionCall>>> + '_ {
        let values: Vec<Arc<RwLock<FunctionCall>>> = self
            .function_call
            .read()
            .unwrap()
            .values()
            .map(|function_call| function_call.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XFuture`] into the store.
    ///
    pub fn inter_x_future(&mut self, x_future: Arc<RwLock<XFuture>>) {
        let read = x_future.read().unwrap();
        self.x_future
            .write()
            .unwrap()
            .insert(read.id, x_future.clone());
    }

    /// Exhume (get) [`XFuture`] from the store.
    ///
    pub fn exhume_x_future(&self, id: &Uuid) -> Option<Arc<RwLock<XFuture>>> {
        self.x_future
            .read()
            .unwrap()
            .get(id)
            .map(|x_future| x_future.clone())
    }

    /// Exorcise (remove) [`XFuture`] from the store.
    ///
    pub fn exorcise_x_future(&mut self, id: &Uuid) -> Option<Arc<RwLock<XFuture>>> {
        self.x_future
            .write()
            .unwrap()
            .remove(id)
            .map(|x_future| x_future.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XFuture>`.
    ///
    pub fn iter_x_future(&self) -> impl Iterator<Item = Arc<RwLock<XFuture>>> + '_ {
        let values: Vec<Arc<RwLock<XFuture>>> = self
            .x_future
            .read()
            .unwrap()
            .values()
            .map(|x_future| x_future.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub fn inter_grouped(&mut self, grouped: Arc<RwLock<Grouped>>) {
        let read = grouped.read().unwrap();
        self.grouped
            .write()
            .unwrap()
            .insert(read.id, grouped.clone());
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub fn exhume_grouped(&self, id: &Uuid) -> Option<Arc<RwLock<Grouped>>> {
        self.grouped
            .read()
            .unwrap()
            .get(id)
            .map(|grouped| grouped.clone())
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub fn exorcise_grouped(&mut self, id: &Uuid) -> Option<Arc<RwLock<Grouped>>> {
        self.grouped
            .write()
            .unwrap()
            .remove(id)
            .map(|grouped| grouped.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub fn iter_grouped(&self) -> impl Iterator<Item = Arc<RwLock<Grouped>>> + '_ {
        let values: Vec<Arc<RwLock<Grouped>>> = self
            .grouped
            .read()
            .unwrap()
            .values()
            .map(|grouped| grouped.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`HaltAndCatchFire`] into the store.
    ///
    pub fn inter_halt_and_catch_fire(
        &mut self,
        halt_and_catch_fire: Arc<RwLock<HaltAndCatchFire>>,
    ) {
        let read = halt_and_catch_fire.read().unwrap();
        self.halt_and_catch_fire
            .write()
            .unwrap()
            .insert(read.id, halt_and_catch_fire.clone());
    }

    /// Exhume (get) [`HaltAndCatchFire`] from the store.
    ///
    pub fn exhume_halt_and_catch_fire(&self, id: &Uuid) -> Option<Arc<RwLock<HaltAndCatchFire>>> {
        self.halt_and_catch_fire
            .read()
            .unwrap()
            .get(id)
            .map(|halt_and_catch_fire| halt_and_catch_fire.clone())
    }

    /// Exorcise (remove) [`HaltAndCatchFire`] from the store.
    ///
    pub fn exorcise_halt_and_catch_fire(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<HaltAndCatchFire>>> {
        self.halt_and_catch_fire
            .write()
            .unwrap()
            .remove(id)
            .map(|halt_and_catch_fire| halt_and_catch_fire.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, HaltAndCatchFire>`.
    ///
    pub fn iter_halt_and_catch_fire(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<HaltAndCatchFire>>> + '_ {
        let values: Vec<Arc<RwLock<HaltAndCatchFire>>> = self
            .halt_and_catch_fire
            .read()
            .unwrap()
            .values()
            .map(|halt_and_catch_fire| halt_and_catch_fire.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub fn inter_x_if(&mut self, x_if: Arc<RwLock<XIf>>) {
        let read = x_if.read().unwrap();
        self.x_if.write().unwrap().insert(read.id, x_if.clone());
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub fn exhume_x_if(&self, id: &Uuid) -> Option<Arc<RwLock<XIf>>> {
        self.x_if.read().unwrap().get(id).map(|x_if| x_if.clone())
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub fn exorcise_x_if(&mut self, id: &Uuid) -> Option<Arc<RwLock<XIf>>> {
        self.x_if
            .write()
            .unwrap()
            .remove(id)
            .map(|x_if| x_if.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub fn iter_x_if(&self) -> impl Iterator<Item = Arc<RwLock<XIf>>> + '_ {
        let values: Vec<Arc<RwLock<XIf>>> = self
            .x_if
            .read()
            .unwrap()
            .values()
            .map(|x_if| x_if.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ImplementationBlock`] into the store.
    ///
    pub fn inter_implementation_block(
        &mut self,
        implementation_block: Arc<RwLock<ImplementationBlock>>,
    ) {
        let read = implementation_block.read().unwrap();
        self.implementation_block
            .write()
            .unwrap()
            .insert(read.id, implementation_block.clone());
    }

    /// Exhume (get) [`ImplementationBlock`] from the store.
    ///
    pub fn exhume_implementation_block(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ImplementationBlock>>> {
        self.implementation_block
            .read()
            .unwrap()
            .get(id)
            .map(|implementation_block| implementation_block.clone())
    }

    /// Exorcise (remove) [`ImplementationBlock`] from the store.
    ///
    pub fn exorcise_implementation_block(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ImplementationBlock>>> {
        self.implementation_block
            .write()
            .unwrap()
            .remove(id)
            .map(|implementation_block| implementation_block.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ImplementationBlock>`.
    ///
    pub fn iter_implementation_block(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ImplementationBlock>>> + '_ {
        let values: Vec<Arc<RwLock<ImplementationBlock>>> = self
            .implementation_block
            .read()
            .unwrap()
            .values()
            .map(|implementation_block| implementation_block.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import(&mut self, import: Arc<RwLock<Import>>) {
        let read = import.read().unwrap();
        self.import.write().unwrap().insert(read.id, import.clone());
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &Uuid) -> Option<Arc<RwLock<Import>>> {
        self.import
            .read()
            .unwrap()
            .get(id)
            .map(|import| import.clone())
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub fn exorcise_import(&mut self, id: &Uuid) -> Option<Arc<RwLock<Import>>> {
        self.import
            .write()
            .unwrap()
            .remove(id)
            .map(|import| import.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = Arc<RwLock<Import>>> + '_ {
        let values: Vec<Arc<RwLock<Import>>> = self
            .import
            .read()
            .unwrap()
            .values()
            .map(|import| import.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub fn inter_index(&mut self, index: Arc<RwLock<Index>>) {
        let read = index.read().unwrap();
        self.index.write().unwrap().insert(read.id, index.clone());
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub fn exhume_index(&self, id: &Uuid) -> Option<Arc<RwLock<Index>>> {
        self.index
            .read()
            .unwrap()
            .get(id)
            .map(|index| index.clone())
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub fn exorcise_index(&mut self, id: &Uuid) -> Option<Arc<RwLock<Index>>> {
        self.index
            .write()
            .unwrap()
            .remove(id)
            .map(|index| index.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub fn iter_index(&self) -> impl Iterator<Item = Arc<RwLock<Index>>> + '_ {
        let values: Vec<Arc<RwLock<Index>>> = self
            .index
            .read()
            .unwrap()
            .values()
            .map(|index| index.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal(&mut self, integer_literal: Arc<RwLock<IntegerLiteral>>) {
        let read = integer_literal.read().unwrap();
        self.integer_literal
            .write()
            .unwrap()
            .insert(read.id, integer_literal.clone());
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &Uuid) -> Option<Arc<RwLock<IntegerLiteral>>> {
        self.integer_literal
            .read()
            .unwrap()
            .get(id)
            .map(|integer_literal| integer_literal.clone())
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub fn exorcise_integer_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<IntegerLiteral>>> {
        self.integer_literal
            .write()
            .unwrap()
            .remove(id)
            .map(|integer_literal| integer_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Arc<RwLock<IntegerLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<IntegerLiteral>>> = self
            .integer_literal
            .read()
            .unwrap()
            .values()
            .map(|integer_literal| integer_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Arc<RwLock<Item>>) {
        let read = item.read().unwrap();
        self.item.write().unwrap().insert(read.id, item.clone());
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<Arc<RwLock<Item>>> {
        self.item.read().unwrap().get(id).map(|item| item.clone())
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &Uuid) -> Option<Arc<RwLock<Item>>> {
        self.item
            .write()
            .unwrap()
            .remove(id)
            .map(|item| item.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = Arc<RwLock<Item>>> + '_ {
        let values: Vec<Arc<RwLock<Item>>> = self
            .item
            .read()
            .unwrap()
            .values()
            .map(|item| item.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Lambda`] into the store.
    ///
    pub fn inter_lambda(&mut self, lambda: Arc<RwLock<Lambda>>) {
        let read = lambda.read().unwrap();
        self.lambda.write().unwrap().insert(read.id, lambda.clone());
    }

    /// Exhume (get) [`Lambda`] from the store.
    ///
    pub fn exhume_lambda(&self, id: &Uuid) -> Option<Arc<RwLock<Lambda>>> {
        self.lambda
            .read()
            .unwrap()
            .get(id)
            .map(|lambda| lambda.clone())
    }

    /// Exorcise (remove) [`Lambda`] from the store.
    ///
    pub fn exorcise_lambda(&mut self, id: &Uuid) -> Option<Arc<RwLock<Lambda>>> {
        self.lambda
            .write()
            .unwrap()
            .remove(id)
            .map(|lambda| lambda.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Lambda>`.
    ///
    pub fn iter_lambda(&self) -> impl Iterator<Item = Arc<RwLock<Lambda>>> + '_ {
        let values: Vec<Arc<RwLock<Lambda>>> = self
            .lambda
            .read()
            .unwrap()
            .values()
            .map(|lambda| lambda.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`LambdaParameter`] into the store.
    ///
    pub fn inter_lambda_parameter(&mut self, lambda_parameter: Arc<RwLock<LambdaParameter>>) {
        let read = lambda_parameter.read().unwrap();
        self.lambda_parameter
            .write()
            .unwrap()
            .insert(read.id, lambda_parameter.clone());
    }

    /// Exhume (get) [`LambdaParameter`] from the store.
    ///
    pub fn exhume_lambda_parameter(&self, id: &Uuid) -> Option<Arc<RwLock<LambdaParameter>>> {
        self.lambda_parameter
            .read()
            .unwrap()
            .get(id)
            .map(|lambda_parameter| lambda_parameter.clone())
    }

    /// Exorcise (remove) [`LambdaParameter`] from the store.
    ///
    pub fn exorcise_lambda_parameter(&mut self, id: &Uuid) -> Option<Arc<RwLock<LambdaParameter>>> {
        self.lambda_parameter
            .write()
            .unwrap()
            .remove(id)
            .map(|lambda_parameter| lambda_parameter.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LambdaParameter>`.
    ///
    pub fn iter_lambda_parameter(&self) -> impl Iterator<Item = Arc<RwLock<LambdaParameter>>> + '_ {
        let values: Vec<Arc<RwLock<LambdaParameter>>> = self
            .lambda_parameter
            .read()
            .unwrap()
            .values()
            .map(|lambda_parameter| lambda_parameter.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement(&mut self, let_statement: Arc<RwLock<LetStatement>>) {
        let read = let_statement.read().unwrap();
        self.let_statement
            .write()
            .unwrap()
            .insert(read.id, let_statement.clone());
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &Uuid) -> Option<Arc<RwLock<LetStatement>>> {
        self.let_statement
            .read()
            .unwrap()
            .get(id)
            .map(|let_statement| let_statement.clone())
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub fn exorcise_let_statement(&mut self, id: &Uuid) -> Option<Arc<RwLock<LetStatement>>> {
        self.let_statement
            .write()
            .unwrap()
            .remove(id)
            .map(|let_statement| let_statement.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Arc<RwLock<LetStatement>>> + '_ {
        let values: Vec<Arc<RwLock<LetStatement>>> = self
            .let_statement
            .read()
            .unwrap()
            .values()
            .map(|let_statement| let_statement.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub fn inter_list(&mut self, list: Arc<RwLock<List>>) {
        let read = list.read().unwrap();
        self.list.write().unwrap().insert(read.id, list.clone());
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &Uuid) -> Option<Arc<RwLock<List>>> {
        self.list.read().unwrap().get(id).map(|list| list.clone())
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub fn exorcise_list(&mut self, id: &Uuid) -> Option<Arc<RwLock<List>>> {
        self.list
            .write()
            .unwrap()
            .remove(id)
            .map(|list| list.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = Arc<RwLock<List>>> + '_ {
        let values: Vec<Arc<RwLock<List>>> = self
            .list
            .read()
            .unwrap()
            .values()
            .map(|list| list.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub fn inter_list_element(&mut self, list_element: Arc<RwLock<ListElement>>) {
        let read = list_element.read().unwrap();
        self.list_element
            .write()
            .unwrap()
            .insert(read.id, list_element.clone());
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub fn exhume_list_element(&self, id: &Uuid) -> Option<Arc<RwLock<ListElement>>> {
        self.list_element
            .read()
            .unwrap()
            .get(id)
            .map(|list_element| list_element.clone())
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub fn exorcise_list_element(&mut self, id: &Uuid) -> Option<Arc<RwLock<ListElement>>> {
        self.list_element
            .write()
            .unwrap()
            .remove(id)
            .map(|list_element| list_element.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub fn iter_list_element(&self) -> impl Iterator<Item = Arc<RwLock<ListElement>>> + '_ {
        let values: Vec<Arc<RwLock<ListElement>>> = self
            .list_element
            .read()
            .unwrap()
            .values()
            .map(|list_element| list_element.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub fn inter_list_expression(&mut self, list_expression: Arc<RwLock<ListExpression>>) {
        let read = list_expression.read().unwrap();
        self.list_expression
            .write()
            .unwrap()
            .insert(read.id, list_expression.clone());
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub fn exhume_list_expression(&self, id: &Uuid) -> Option<Arc<RwLock<ListExpression>>> {
        self.list_expression
            .read()
            .unwrap()
            .get(id)
            .map(|list_expression| list_expression.clone())
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub fn exorcise_list_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<ListExpression>>> {
        self.list_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|list_expression| list_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub fn iter_list_expression(&self) -> impl Iterator<Item = Arc<RwLock<ListExpression>>> + '_ {
        let values: Vec<Arc<RwLock<ListExpression>>> = self
            .list_expression
            .read()
            .unwrap()
            .values()
            .map(|list_expression| list_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub fn inter_literal(&mut self, literal: Arc<RwLock<Literal>>) {
        let read = literal.read().unwrap();
        self.literal
            .write()
            .unwrap()
            .insert(read.id, literal.clone());
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &Uuid) -> Option<Arc<RwLock<Literal>>> {
        self.literal
            .read()
            .unwrap()
            .get(id)
            .map(|literal| literal.clone())
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub fn exorcise_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<Literal>>> {
        self.literal
            .write()
            .unwrap()
            .remove(id)
            .map(|literal| literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = Arc<RwLock<Literal>>> + '_ {
        let values: Vec<Arc<RwLock<Literal>>> = self
            .literal
            .read()
            .unwrap()
            .values()
            .map(|literal| literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable(&mut self, local_variable: Arc<RwLock<LocalVariable>>) {
        let read = local_variable.read().unwrap();
        self.local_variable
            .write()
            .unwrap()
            .insert(read.id, local_variable.clone());
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &Uuid) -> Option<Arc<RwLock<LocalVariable>>> {
        self.local_variable
            .read()
            .unwrap()
            .get(id)
            .map(|local_variable| local_variable.clone())
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub fn exorcise_local_variable(&mut self, id: &Uuid) -> Option<Arc<RwLock<LocalVariable>>> {
        self.local_variable
            .write()
            .unwrap()
            .remove(id)
            .map(|local_variable| local_variable.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Arc<RwLock<LocalVariable>>> + '_ {
        let values: Vec<Arc<RwLock<LocalVariable>>> = self
            .local_variable
            .read()
            .unwrap()
            .values()
            .map(|local_variable| local_variable.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub fn inter_x_macro(&mut self, x_macro: Arc<RwLock<XMacro>>) {
        let read = x_macro.read().unwrap();
        self.x_macro
            .write()
            .unwrap()
            .insert(read.id, x_macro.clone());
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub fn exhume_x_macro(&self, id: &Uuid) -> Option<Arc<RwLock<XMacro>>> {
        self.x_macro
            .read()
            .unwrap()
            .get(id)
            .map(|x_macro| x_macro.clone())
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub fn exorcise_x_macro(&mut self, id: &Uuid) -> Option<Arc<RwLock<XMacro>>> {
        self.x_macro
            .write()
            .unwrap()
            .remove(id)
            .map(|x_macro| x_macro.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub fn iter_x_macro(&self) -> impl Iterator<Item = Arc<RwLock<XMacro>>> + '_ {
        let values: Vec<Arc<RwLock<XMacro>>> = self
            .x_macro
            .read()
            .unwrap()
            .values()
            .map(|x_macro| x_macro.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Map`] into the store.
    ///
    pub fn inter_map(&mut self, map: Arc<RwLock<Map>>) {
        let read = map.read().unwrap();
        self.map.write().unwrap().insert(read.id, map.clone());
    }

    /// Exhume (get) [`Map`] from the store.
    ///
    pub fn exhume_map(&self, id: &Uuid) -> Option<Arc<RwLock<Map>>> {
        self.map.read().unwrap().get(id).map(|map| map.clone())
    }

    /// Exorcise (remove) [`Map`] from the store.
    ///
    pub fn exorcise_map(&mut self, id: &Uuid) -> Option<Arc<RwLock<Map>>> {
        self.map.write().unwrap().remove(id).map(|map| map.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Map>`.
    ///
    pub fn iter_map(&self) -> impl Iterator<Item = Arc<RwLock<Map>>> + '_ {
        let values: Vec<Arc<RwLock<Map>>> = self
            .map
            .read()
            .unwrap()
            .values()
            .map(|map| map.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`MapElement`] into the store.
    ///
    pub fn inter_map_element(&mut self, map_element: Arc<RwLock<MapElement>>) {
        let read = map_element.read().unwrap();
        self.map_element
            .write()
            .unwrap()
            .insert(read.id, map_element.clone());
    }

    /// Exhume (get) [`MapElement`] from the store.
    ///
    pub fn exhume_map_element(&self, id: &Uuid) -> Option<Arc<RwLock<MapElement>>> {
        self.map_element
            .read()
            .unwrap()
            .get(id)
            .map(|map_element| map_element.clone())
    }

    /// Exorcise (remove) [`MapElement`] from the store.
    ///
    pub fn exorcise_map_element(&mut self, id: &Uuid) -> Option<Arc<RwLock<MapElement>>> {
        self.map_element
            .write()
            .unwrap()
            .remove(id)
            .map(|map_element| map_element.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MapElement>`.
    ///
    pub fn iter_map_element(&self) -> impl Iterator<Item = Arc<RwLock<MapElement>>> + '_ {
        let values: Vec<Arc<RwLock<MapElement>>> = self
            .map_element
            .read()
            .unwrap()
            .values()
            .map(|map_element| map_element.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`MapExpression`] into the store.
    ///
    pub fn inter_map_expression(&mut self, map_expression: Arc<RwLock<MapExpression>>) {
        let read = map_expression.read().unwrap();
        self.map_expression
            .write()
            .unwrap()
            .insert(read.id, map_expression.clone());
    }

    /// Exhume (get) [`MapExpression`] from the store.
    ///
    pub fn exhume_map_expression(&self, id: &Uuid) -> Option<Arc<RwLock<MapExpression>>> {
        self.map_expression
            .read()
            .unwrap()
            .get(id)
            .map(|map_expression| map_expression.clone())
    }

    /// Exorcise (remove) [`MapExpression`] from the store.
    ///
    pub fn exorcise_map_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<MapExpression>>> {
        self.map_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|map_expression| map_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MapExpression>`.
    ///
    pub fn iter_map_expression(&self) -> impl Iterator<Item = Arc<RwLock<MapExpression>>> + '_ {
        let values: Vec<Arc<RwLock<MapExpression>>> = self
            .map_expression
            .read()
            .unwrap()
            .values()
            .map(|map_expression| map_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XMatch`] into the store.
    ///
    pub fn inter_x_match(&mut self, x_match: Arc<RwLock<XMatch>>) {
        let read = x_match.read().unwrap();
        self.x_match
            .write()
            .unwrap()
            .insert(read.id, x_match.clone());
    }

    /// Exhume (get) [`XMatch`] from the store.
    ///
    pub fn exhume_x_match(&self, id: &Uuid) -> Option<Arc<RwLock<XMatch>>> {
        self.x_match
            .read()
            .unwrap()
            .get(id)
            .map(|x_match| x_match.clone())
    }

    /// Exorcise (remove) [`XMatch`] from the store.
    ///
    pub fn exorcise_x_match(&mut self, id: &Uuid) -> Option<Arc<RwLock<XMatch>>> {
        self.x_match
            .write()
            .unwrap()
            .remove(id)
            .map(|x_match| x_match.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMatch>`.
    ///
    pub fn iter_x_match(&self) -> impl Iterator<Item = Arc<RwLock<XMatch>>> + '_ {
        let values: Vec<Arc<RwLock<XMatch>>> = self
            .x_match
            .read()
            .unwrap()
            .values()
            .map(|x_match| x_match.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub fn inter_method_call(&mut self, method_call: Arc<RwLock<MethodCall>>) {
        let read = method_call.read().unwrap();
        self.method_call
            .write()
            .unwrap()
            .insert(read.id, method_call.clone());
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &Uuid) -> Option<Arc<RwLock<MethodCall>>> {
        self.method_call
            .read()
            .unwrap()
            .get(id)
            .map(|method_call| method_call.clone())
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub fn exorcise_method_call(&mut self, id: &Uuid) -> Option<Arc<RwLock<MethodCall>>> {
        self.method_call
            .write()
            .unwrap()
            .remove(id)
            .map(|method_call| method_call.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = Arc<RwLock<MethodCall>>> + '_ {
        let values: Vec<Arc<RwLock<MethodCall>>> = self
            .method_call
            .read()
            .unwrap()
            .values()
            .map(|method_call| method_call.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`NamedFieldExpression`] into the store.
    ///
    pub fn inter_named_field_expression(
        &mut self,
        named_field_expression: Arc<RwLock<NamedFieldExpression>>,
    ) {
        let read = named_field_expression.read().unwrap();
        self.named_field_expression
            .write()
            .unwrap()
            .insert(read.id, named_field_expression.clone());
    }

    /// Exhume (get) [`NamedFieldExpression`] from the store.
    ///
    pub fn exhume_named_field_expression(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<NamedFieldExpression>>> {
        self.named_field_expression
            .read()
            .unwrap()
            .get(id)
            .map(|named_field_expression| named_field_expression.clone())
    }

    /// Exorcise (remove) [`NamedFieldExpression`] from the store.
    ///
    pub fn exorcise_named_field_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<NamedFieldExpression>>> {
        self.named_field_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|named_field_expression| named_field_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NamedFieldExpression>`.
    ///
    pub fn iter_named_field_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<NamedFieldExpression>>> + '_ {
        let values: Vec<Arc<RwLock<NamedFieldExpression>>> = self
            .named_field_expression
            .read()
            .unwrap()
            .values()
            .map(|named_field_expression| named_field_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store(&mut self, z_object_store: Arc<RwLock<ZObjectStore>>) {
        let read = z_object_store.read().unwrap();
        self.z_object_store_id_by_name
            .write()
            .unwrap()
            .insert(read.name.clone(), read.id);
        self.z_object_store
            .write()
            .unwrap()
            .insert(read.id, z_object_store.clone());
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &Uuid) -> Option<Arc<RwLock<ZObjectStore>>> {
        self.z_object_store
            .read()
            .unwrap()
            .get(id)
            .map(|z_object_store| z_object_store.clone())
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub fn exorcise_z_object_store(&mut self, id: &Uuid) -> Option<Arc<RwLock<ZObjectStore>>> {
        self.z_object_store
            .write()
            .unwrap()
            .remove(id)
            .map(|z_object_store| z_object_store.clone())
    }

    /// Exhume [`ZObjectStore`] id from the store by name.
    ///
    pub fn exhume_z_object_store_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.z_object_store_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|z_object_store| *z_object_store)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Arc<RwLock<ZObjectStore>>> + '_ {
        let values: Vec<Arc<RwLock<ZObjectStore>>> = self
            .z_object_store
            .read()
            .unwrap()
            .values()
            .map(|z_object_store| z_object_store.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ObjectWrapper`] into the store.
    ///
    pub fn inter_object_wrapper(&mut self, object_wrapper: Arc<RwLock<ObjectWrapper>>) {
        let read = object_wrapper.read().unwrap();
        self.object_wrapper
            .write()
            .unwrap()
            .insert(read.id, object_wrapper.clone());
    }

    /// Exhume (get) [`ObjectWrapper`] from the store.
    ///
    pub fn exhume_object_wrapper(&self, id: &Uuid) -> Option<Arc<RwLock<ObjectWrapper>>> {
        self.object_wrapper
            .read()
            .unwrap()
            .get(id)
            .map(|object_wrapper| object_wrapper.clone())
    }

    /// Exorcise (remove) [`ObjectWrapper`] from the store.
    ///
    pub fn exorcise_object_wrapper(&mut self, id: &Uuid) -> Option<Arc<RwLock<ObjectWrapper>>> {
        self.object_wrapper
            .write()
            .unwrap()
            .remove(id)
            .map(|object_wrapper| object_wrapper.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectWrapper>`.
    ///
    pub fn iter_object_wrapper(&self) -> impl Iterator<Item = Arc<RwLock<ObjectWrapper>>> + '_ {
        let values: Vec<Arc<RwLock<ObjectWrapper>>> = self
            .object_wrapper
            .read()
            .unwrap()
            .values()
            .map(|object_wrapper| object_wrapper.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator(&mut self, operator: Arc<RwLock<Operator>>) {
        let read = operator.read().unwrap();
        self.operator
            .write()
            .unwrap()
            .insert(read.id, operator.clone());
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub fn exhume_operator(&self, id: &Uuid) -> Option<Arc<RwLock<Operator>>> {
        self.operator
            .read()
            .unwrap()
            .get(id)
            .map(|operator| operator.clone())
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub fn exorcise_operator(&mut self, id: &Uuid) -> Option<Arc<RwLock<Operator>>> {
        self.operator
            .write()
            .unwrap()
            .remove(id)
            .map(|operator| operator.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub fn iter_operator(&self) -> impl Iterator<Item = Arc<RwLock<Operator>>> + '_ {
        let values: Vec<Arc<RwLock<Operator>>> = self
            .operator
            .read()
            .unwrap()
            .values()
            .map(|operator| operator.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Arc<RwLock<Parameter>>) {
        let read = parameter.read().unwrap();
        self.parameter
            .write()
            .unwrap()
            .insert(read.id, parameter.clone());
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<Arc<RwLock<Parameter>>> {
        self.parameter
            .read()
            .unwrap()
            .get(id)
            .map(|parameter| parameter.clone())
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Arc<RwLock<Parameter>>> {
        self.parameter
            .write()
            .unwrap()
            .remove(id)
            .map(|parameter| parameter.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Arc<RwLock<Parameter>>> + '_ {
        let values: Vec<Arc<RwLock<Parameter>>> = self
            .parameter
            .read()
            .unwrap()
            .values()
            .map(|parameter| parameter.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XPath`] into the store.
    ///
    pub fn inter_x_path(&mut self, x_path: Arc<RwLock<XPath>>) {
        let read = x_path.read().unwrap();
        self.x_path.write().unwrap().insert(read.id, x_path.clone());
    }

    /// Exhume (get) [`XPath`] from the store.
    ///
    pub fn exhume_x_path(&self, id: &Uuid) -> Option<Arc<RwLock<XPath>>> {
        self.x_path
            .read()
            .unwrap()
            .get(id)
            .map(|x_path| x_path.clone())
    }

    /// Exorcise (remove) [`XPath`] from the store.
    ///
    pub fn exorcise_x_path(&mut self, id: &Uuid) -> Option<Arc<RwLock<XPath>>> {
        self.x_path
            .write()
            .unwrap()
            .remove(id)
            .map(|x_path| x_path.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPath>`.
    ///
    pub fn iter_x_path(&self) -> impl Iterator<Item = Arc<RwLock<XPath>>> + '_ {
        let values: Vec<Arc<RwLock<XPath>>> = self
            .x_path
            .read()
            .unwrap()
            .values()
            .map(|x_path| x_path.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`PathElement`] into the store.
    ///
    pub fn inter_path_element(&mut self, path_element: Arc<RwLock<PathElement>>) {
        let read = path_element.read().unwrap();
        self.path_element
            .write()
            .unwrap()
            .insert(read.id, path_element.clone());
    }

    /// Exhume (get) [`PathElement`] from the store.
    ///
    pub fn exhume_path_element(&self, id: &Uuid) -> Option<Arc<RwLock<PathElement>>> {
        self.path_element
            .read()
            .unwrap()
            .get(id)
            .map(|path_element| path_element.clone())
    }

    /// Exorcise (remove) [`PathElement`] from the store.
    ///
    pub fn exorcise_path_element(&mut self, id: &Uuid) -> Option<Arc<RwLock<PathElement>>> {
        self.path_element
            .write()
            .unwrap()
            .remove(id)
            .map(|path_element| path_element.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, PathElement>`.
    ///
    pub fn iter_path_element(&self) -> impl Iterator<Item = Arc<RwLock<PathElement>>> + '_ {
        let values: Vec<Arc<RwLock<PathElement>>> = self
            .path_element
            .read()
            .unwrap()
            .values()
            .map(|path_element| path_element.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Pattern`] into the store.
    ///
    pub fn inter_pattern(&mut self, pattern: Arc<RwLock<Pattern>>) {
        let read = pattern.read().unwrap();
        self.pattern
            .write()
            .unwrap()
            .insert(read.id, pattern.clone());
    }

    /// Exhume (get) [`Pattern`] from the store.
    ///
    pub fn exhume_pattern(&self, id: &Uuid) -> Option<Arc<RwLock<Pattern>>> {
        self.pattern
            .read()
            .unwrap()
            .get(id)
            .map(|pattern| pattern.clone())
    }

    /// Exorcise (remove) [`Pattern`] from the store.
    ///
    pub fn exorcise_pattern(&mut self, id: &Uuid) -> Option<Arc<RwLock<Pattern>>> {
        self.pattern
            .write()
            .unwrap()
            .remove(id)
            .map(|pattern| pattern.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Pattern>`.
    ///
    pub fn iter_pattern(&self) -> impl Iterator<Item = Arc<RwLock<Pattern>>> + '_ {
        let values: Vec<Arc<RwLock<Pattern>>> = self
            .pattern
            .read()
            .unwrap()
            .values()
            .map(|pattern| pattern.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XPlugin`] into the store.
    ///
    pub fn inter_x_plugin(&mut self, x_plugin: Arc<RwLock<XPlugin>>) {
        let read = x_plugin.read().unwrap();
        self.x_plugin_id_by_name
            .write()
            .unwrap()
            .insert(read.name.clone(), read.id);
        self.x_plugin
            .write()
            .unwrap()
            .insert(read.id, x_plugin.clone());
    }

    /// Exhume (get) [`XPlugin`] from the store.
    ///
    pub fn exhume_x_plugin(&self, id: &Uuid) -> Option<Arc<RwLock<XPlugin>>> {
        self.x_plugin
            .read()
            .unwrap()
            .get(id)
            .map(|x_plugin| x_plugin.clone())
    }

    /// Exorcise (remove) [`XPlugin`] from the store.
    ///
    pub fn exorcise_x_plugin(&mut self, id: &Uuid) -> Option<Arc<RwLock<XPlugin>>> {
        self.x_plugin
            .write()
            .unwrap()
            .remove(id)
            .map(|x_plugin| x_plugin.clone())
    }

    /// Exhume [`XPlugin`] id from the store by name.
    ///
    pub fn exhume_x_plugin_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.x_plugin_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|x_plugin| *x_plugin)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPlugin>`.
    ///
    pub fn iter_x_plugin(&self) -> impl Iterator<Item = Arc<RwLock<XPlugin>>> + '_ {
        let values: Vec<Arc<RwLock<XPlugin>>> = self
            .x_plugin
            .read()
            .unwrap()
            .values()
            .map(|x_plugin| x_plugin.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XPrint`] into the store.
    ///
    pub fn inter_x_print(&mut self, x_print: Arc<RwLock<XPrint>>) {
        let read = x_print.read().unwrap();
        self.x_print
            .write()
            .unwrap()
            .insert(read.id, x_print.clone());
    }

    /// Exhume (get) [`XPrint`] from the store.
    ///
    pub fn exhume_x_print(&self, id: &Uuid) -> Option<Arc<RwLock<XPrint>>> {
        self.x_print
            .read()
            .unwrap()
            .get(id)
            .map(|x_print| x_print.clone())
    }

    /// Exorcise (remove) [`XPrint`] from the store.
    ///
    pub fn exorcise_x_print(&mut self, id: &Uuid) -> Option<Arc<RwLock<XPrint>>> {
        self.x_print
            .write()
            .unwrap()
            .remove(id)
            .map(|x_print| x_print.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPrint>`.
    ///
    pub fn iter_x_print(&self) -> impl Iterator<Item = Arc<RwLock<XPrint>>> + '_ {
        let values: Vec<Arc<RwLock<XPrint>>> = self
            .x_print
            .read()
            .unwrap()
            .values()
            .map(|x_print| x_print.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub fn inter_range_expression(&mut self, range_expression: Arc<RwLock<RangeExpression>>) {
        let read = range_expression.read().unwrap();
        self.range_expression
            .write()
            .unwrap()
            .insert(read.id, range_expression.clone());
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub fn exhume_range_expression(&self, id: &Uuid) -> Option<Arc<RwLock<RangeExpression>>> {
        self.range_expression
            .read()
            .unwrap()
            .get(id)
            .map(|range_expression| range_expression.clone())
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub fn exorcise_range_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<RangeExpression>>> {
        self.range_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|range_expression| range_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub fn iter_range_expression(&self) -> impl Iterator<Item = Arc<RwLock<RangeExpression>>> + '_ {
        let values: Vec<Arc<RwLock<RangeExpression>>> = self
            .range_expression
            .read()
            .unwrap()
            .values()
            .map(|range_expression| range_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement(&mut self, result_statement: Arc<RwLock<ResultStatement>>) {
        let read = result_statement.read().unwrap();
        self.result_statement
            .write()
            .unwrap()
            .insert(read.id, result_statement.clone());
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &Uuid) -> Option<Arc<RwLock<ResultStatement>>> {
        self.result_statement
            .read()
            .unwrap()
            .get(id)
            .map(|result_statement| result_statement.clone())
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub fn exorcise_result_statement(&mut self, id: &Uuid) -> Option<Arc<RwLock<ResultStatement>>> {
        self.result_statement
            .write()
            .unwrap()
            .remove(id)
            .map(|result_statement| result_statement.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Arc<RwLock<ResultStatement>>> + '_ {
        let values: Vec<Arc<RwLock<ResultStatement>>> = self
            .result_statement
            .read()
            .unwrap()
            .values()
            .map(|result_statement| result_statement.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub fn inter_x_return(&mut self, x_return: Arc<RwLock<XReturn>>) {
        let read = x_return.read().unwrap();
        self.x_return
            .write()
            .unwrap()
            .insert(read.id, x_return.clone());
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub fn exhume_x_return(&self, id: &Uuid) -> Option<Arc<RwLock<XReturn>>> {
        self.x_return
            .read()
            .unwrap()
            .get(id)
            .map(|x_return| x_return.clone())
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub fn exorcise_x_return(&mut self, id: &Uuid) -> Option<Arc<RwLock<XReturn>>> {
        self.x_return
            .write()
            .unwrap()
            .remove(id)
            .map(|x_return| x_return.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub fn iter_x_return(&self) -> impl Iterator<Item = Arc<RwLock<XReturn>>> + '_ {
        let values: Vec<Arc<RwLock<XReturn>>> = self
            .x_return
            .read()
            .unwrap()
            .values()
            .map(|x_return| x_return.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub fn inter_span(&mut self, span: Arc<RwLock<Span>>) {
        let read = span.read().unwrap();
        self.span.write().unwrap().insert(read.id, span.clone());
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub fn exhume_span(&self, id: &Uuid) -> Option<Arc<RwLock<Span>>> {
        self.span.read().unwrap().get(id).map(|span| span.clone())
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub fn exorcise_span(&mut self, id: &Uuid) -> Option<Arc<RwLock<Span>>> {
        self.span
            .write()
            .unwrap()
            .remove(id)
            .map(|span| span.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub fn iter_span(&self) -> impl Iterator<Item = Arc<RwLock<Span>>> + '_ {
        let values: Vec<Arc<RwLock<Span>>> = self
            .span
            .read()
            .unwrap()
            .values()
            .map(|span| span.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Arc<RwLock<Statement>>) {
        let read = statement.read().unwrap();
        self.statement
            .write()
            .unwrap()
            .insert(read.id, statement.clone());
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<Arc<RwLock<Statement>>> {
        self.statement
            .read()
            .unwrap()
            .get(id)
            .map(|statement| statement.clone())
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &Uuid) -> Option<Arc<RwLock<Statement>>> {
        self.statement
            .write()
            .unwrap()
            .remove(id)
            .map(|statement| statement.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = Arc<RwLock<Statement>>> + '_ {
        let values: Vec<Arc<RwLock<Statement>>> = self
            .statement
            .read()
            .unwrap()
            .values()
            .map(|statement| statement.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call(&mut self, static_method_call: Arc<RwLock<StaticMethodCall>>) {
        let read = static_method_call.read().unwrap();
        self.static_method_call
            .write()
            .unwrap()
            .insert(read.id, static_method_call.clone());
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &Uuid) -> Option<Arc<RwLock<StaticMethodCall>>> {
        self.static_method_call
            .read()
            .unwrap()
            .get(id)
            .map(|static_method_call| static_method_call.clone())
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub fn exorcise_static_method_call(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<StaticMethodCall>>> {
        self.static_method_call
            .write()
            .unwrap()
            .remove(id)
            .map(|static_method_call| static_method_call.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StaticMethodCall>>> + '_ {
        let values: Vec<Arc<RwLock<StaticMethodCall>>> = self
            .static_method_call
            .read()
            .unwrap()
            .values()
            .map(|static_method_call| static_method_call.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StringBit`] into the store.
    ///
    pub fn inter_string_bit(&mut self, string_bit: Arc<RwLock<StringBit>>) {
        let read = string_bit.read().unwrap();
        self.string_bit
            .write()
            .unwrap()
            .insert(read.id, string_bit.clone());
    }

    /// Exhume (get) [`StringBit`] from the store.
    ///
    pub fn exhume_string_bit(&self, id: &Uuid) -> Option<Arc<RwLock<StringBit>>> {
        self.string_bit
            .read()
            .unwrap()
            .get(id)
            .map(|string_bit| string_bit.clone())
    }

    /// Exorcise (remove) [`StringBit`] from the store.
    ///
    pub fn exorcise_string_bit(&mut self, id: &Uuid) -> Option<Arc<RwLock<StringBit>>> {
        self.string_bit
            .write()
            .unwrap()
            .remove(id)
            .map(|string_bit| string_bit.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringBit>`.
    ///
    pub fn iter_string_bit(&self) -> impl Iterator<Item = Arc<RwLock<StringBit>>> + '_ {
        let values: Vec<Arc<RwLock<StringBit>>> = self
            .string_bit
            .read()
            .unwrap()
            .values()
            .map(|string_bit| string_bit.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal(&mut self, string_literal: Arc<RwLock<StringLiteral>>) {
        let read = string_literal.read().unwrap();
        self.string_literal
            .write()
            .unwrap()
            .insert(read.id, string_literal.clone());
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &Uuid) -> Option<Arc<RwLock<StringLiteral>>> {
        self.string_literal
            .read()
            .unwrap()
            .get(id)
            .map(|string_literal| string_literal.clone())
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub fn exorcise_string_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<StringLiteral>>> {
        self.string_literal
            .write()
            .unwrap()
            .remove(id)
            .map(|string_literal| string_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Arc<RwLock<StringLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<StringLiteral>>> = self
            .string_literal
            .read()
            .unwrap()
            .values()
            .map(|string_literal| string_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct(&mut self, woog_struct: Arc<RwLock<WoogStruct>>) {
        let read = woog_struct.read().unwrap();
        self.woog_struct_id_by_name
            .write()
            .unwrap()
            .insert(read.name.clone(), read.id);
        self.woog_struct
            .write()
            .unwrap()
            .insert(read.id, woog_struct.clone());
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &Uuid) -> Option<Arc<RwLock<WoogStruct>>> {
        self.woog_struct
            .read()
            .unwrap()
            .get(id)
            .map(|woog_struct| woog_struct.clone())
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub fn exorcise_woog_struct(&mut self, id: &Uuid) -> Option<Arc<RwLock<WoogStruct>>> {
        self.woog_struct
            .write()
            .unwrap()
            .remove(id)
            .map(|woog_struct| woog_struct.clone())
    }

    /// Exhume [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.woog_struct_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|woog_struct| *woog_struct)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Arc<RwLock<WoogStruct>>> + '_ {
        let values: Vec<Arc<RwLock<WoogStruct>>> = self
            .woog_struct
            .read()
            .unwrap()
            .values()
            .map(|woog_struct| woog_struct.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub fn inter_struct_expression(&mut self, struct_expression: Arc<RwLock<StructExpression>>) {
        let read = struct_expression.read().unwrap();
        self.struct_expression
            .write()
            .unwrap()
            .insert(read.id, struct_expression.clone());
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &Uuid) -> Option<Arc<RwLock<StructExpression>>> {
        self.struct_expression
            .read()
            .unwrap()
            .get(id)
            .map(|struct_expression| struct_expression.clone())
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub fn exorcise_struct_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<StructExpression>>> {
        self.struct_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|struct_expression| struct_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StructExpression>>> + '_ {
        let values: Vec<Arc<RwLock<StructExpression>>> = self
            .struct_expression
            .read()
            .unwrap()
            .values()
            .map(|struct_expression| struct_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StructField`] into the store.
    ///
    pub fn inter_struct_field(&mut self, struct_field: Arc<RwLock<StructField>>) {
        let read = struct_field.read().unwrap();
        self.struct_field
            .write()
            .unwrap()
            .insert(read.id, struct_field.clone());
    }

    /// Exhume (get) [`StructField`] from the store.
    ///
    pub fn exhume_struct_field(&self, id: &Uuid) -> Option<Arc<RwLock<StructField>>> {
        self.struct_field
            .read()
            .unwrap()
            .get(id)
            .map(|struct_field| struct_field.clone())
    }

    /// Exorcise (remove) [`StructField`] from the store.
    ///
    pub fn exorcise_struct_field(&mut self, id: &Uuid) -> Option<Arc<RwLock<StructField>>> {
        self.struct_field
            .write()
            .unwrap()
            .remove(id)
            .map(|struct_field| struct_field.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructField>`.
    ///
    pub fn iter_struct_field(&self) -> impl Iterator<Item = Arc<RwLock<StructField>>> + '_ {
        let values: Vec<Arc<RwLock<StructField>>> = self
            .struct_field
            .read()
            .unwrap()
            .values()
            .map(|struct_field| struct_field.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StructGeneric`] into the store.
    ///
    pub fn inter_struct_generic(&mut self, struct_generic: Arc<RwLock<StructGeneric>>) {
        let read = struct_generic.read().unwrap();
        self.struct_generic
            .write()
            .unwrap()
            .insert(read.id, struct_generic.clone());
    }

    /// Exhume (get) [`StructGeneric`] from the store.
    ///
    pub fn exhume_struct_generic(&self, id: &Uuid) -> Option<Arc<RwLock<StructGeneric>>> {
        self.struct_generic
            .read()
            .unwrap()
            .get(id)
            .map(|struct_generic| struct_generic.clone())
    }

    /// Exorcise (remove) [`StructGeneric`] from the store.
    ///
    pub fn exorcise_struct_generic(&mut self, id: &Uuid) -> Option<Arc<RwLock<StructGeneric>>> {
        self.struct_generic
            .write()
            .unwrap()
            .remove(id)
            .map(|struct_generic| struct_generic.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructGeneric>`.
    ///
    pub fn iter_struct_generic(&self) -> impl Iterator<Item = Arc<RwLock<StructGeneric>>> + '_ {
        let values: Vec<Arc<RwLock<StructGeneric>>> = self
            .struct_generic
            .read()
            .unwrap()
            .values()
            .map(|struct_generic| struct_generic.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`TupleField`] into the store.
    ///
    pub fn inter_tuple_field(&mut self, tuple_field: Arc<RwLock<TupleField>>) {
        let read = tuple_field.read().unwrap();
        self.tuple_field
            .write()
            .unwrap()
            .insert(read.id, tuple_field.clone());
    }

    /// Exhume (get) [`TupleField`] from the store.
    ///
    pub fn exhume_tuple_field(&self, id: &Uuid) -> Option<Arc<RwLock<TupleField>>> {
        self.tuple_field
            .read()
            .unwrap()
            .get(id)
            .map(|tuple_field| tuple_field.clone())
    }

    /// Exorcise (remove) [`TupleField`] from the store.
    ///
    pub fn exorcise_tuple_field(&mut self, id: &Uuid) -> Option<Arc<RwLock<TupleField>>> {
        self.tuple_field
            .write()
            .unwrap()
            .remove(id)
            .map(|tuple_field| tuple_field.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TupleField>`.
    ///
    pub fn iter_tuple_field(&self) -> impl Iterator<Item = Arc<RwLock<TupleField>>> + '_ {
        let values: Vec<Arc<RwLock<TupleField>>> = self
            .tuple_field
            .read()
            .unwrap()
            .values()
            .map(|tuple_field| tuple_field.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast(&mut self, type_cast: Arc<RwLock<TypeCast>>) {
        let read = type_cast.read().unwrap();
        self.type_cast
            .write()
            .unwrap()
            .insert(read.id, type_cast.clone());
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub fn exhume_type_cast(&self, id: &Uuid) -> Option<Arc<RwLock<TypeCast>>> {
        self.type_cast
            .read()
            .unwrap()
            .get(id)
            .map(|type_cast| type_cast.clone())
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub fn exorcise_type_cast(&mut self, id: &Uuid) -> Option<Arc<RwLock<TypeCast>>> {
        self.type_cast
            .write()
            .unwrap()
            .remove(id)
            .map(|type_cast| type_cast.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub fn iter_type_cast(&self) -> impl Iterator<Item = Arc<RwLock<TypeCast>>> + '_ {
        let values: Vec<Arc<RwLock<TypeCast>>> = self
            .type_cast
            .read()
            .unwrap()
            .values()
            .map(|type_cast| type_cast.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub fn inter_unary(&mut self, unary: Arc<RwLock<Unary>>) {
        let read = unary.read().unwrap();
        self.unary.write().unwrap().insert(read.id, unary.clone());
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub fn exhume_unary(&self, id: &Uuid) -> Option<Arc<RwLock<Unary>>> {
        self.unary
            .read()
            .unwrap()
            .get(id)
            .map(|unary| unary.clone())
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub fn exorcise_unary(&mut self, id: &Uuid) -> Option<Arc<RwLock<Unary>>> {
        self.unary
            .write()
            .unwrap()
            .remove(id)
            .map(|unary| unary.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub fn iter_unary(&self) -> impl Iterator<Item = Arc<RwLock<Unary>>> + '_ {
        let values: Vec<Arc<RwLock<Unary>>> = self
            .unary
            .read()
            .unwrap()
            .values()
            .map(|unary| unary.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Unit`] into the store.
    ///
    pub fn inter_unit(&mut self, unit: Arc<RwLock<Unit>>) {
        let read = unit.read().unwrap();
        self.unit.write().unwrap().insert(read.id, unit.clone());
    }

    /// Exhume (get) [`Unit`] from the store.
    ///
    pub fn exhume_unit(&self, id: &Uuid) -> Option<Arc<RwLock<Unit>>> {
        self.unit.read().unwrap().get(id).map(|unit| unit.clone())
    }

    /// Exorcise (remove) [`Unit`] from the store.
    ///
    pub fn exorcise_unit(&mut self, id: &Uuid) -> Option<Arc<RwLock<Unit>>> {
        self.unit
            .write()
            .unwrap()
            .remove(id)
            .map(|unit| unit.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unit>`.
    ///
    pub fn iter_unit(&self) -> impl Iterator<Item = Arc<RwLock<Unit>>> + '_ {
        let values: Vec<Arc<RwLock<Unit>>> = self
            .unit
            .read()
            .unwrap()
            .values()
            .map(|unit| unit.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`UnnamedFieldExpression`] into the store.
    ///
    pub fn inter_unnamed_field_expression(
        &mut self,
        unnamed_field_expression: Arc<RwLock<UnnamedFieldExpression>>,
    ) {
        let read = unnamed_field_expression.read().unwrap();
        self.unnamed_field_expression
            .write()
            .unwrap()
            .insert(read.id, unnamed_field_expression.clone());
    }

    /// Exhume (get) [`UnnamedFieldExpression`] from the store.
    ///
    pub fn exhume_unnamed_field_expression(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<UnnamedFieldExpression>>> {
        self.unnamed_field_expression
            .read()
            .unwrap()
            .get(id)
            .map(|unnamed_field_expression| unnamed_field_expression.clone())
    }

    /// Exorcise (remove) [`UnnamedFieldExpression`] from the store.
    ///
    pub fn exorcise_unnamed_field_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<UnnamedFieldExpression>>> {
        self.unnamed_field_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|unnamed_field_expression| unnamed_field_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, UnnamedFieldExpression>`.
    ///
    pub fn iter_unnamed_field_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<UnnamedFieldExpression>>> + '_ {
        let values: Vec<Arc<RwLock<UnnamedFieldExpression>>> = self
            .unnamed_field_expression
            .read()
            .unwrap()
            .values()
            .map(|unnamed_field_expression| unnamed_field_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value(&mut self, x_value: Arc<RwLock<XValue>>) {
        let read = x_value.read().unwrap();
        self.x_value
            .write()
            .unwrap()
            .insert(read.id, x_value.clone());
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &Uuid) -> Option<Arc<RwLock<XValue>>> {
        self.x_value
            .read()
            .unwrap()
            .get(id)
            .map(|x_value| x_value.clone())
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &Uuid) -> Option<Arc<RwLock<XValue>>> {
        self.x_value
            .write()
            .unwrap()
            .remove(id)
            .map(|x_value| x_value.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = Arc<RwLock<XValue>>> + '_ {
        let values: Vec<Arc<RwLock<XValue>>> = self
            .x_value
            .read()
            .unwrap()
            .values()
            .map(|x_value| x_value.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub fn inter_value_type(&mut self, value_type: Arc<RwLock<ValueType>>) {
        let read = value_type.read().unwrap();
        self.value_type
            .write()
            .unwrap()
            .insert(read.id, value_type.clone());
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &Uuid) -> Option<Arc<RwLock<ValueType>>> {
        self.value_type
            .read()
            .unwrap()
            .get(id)
            .map(|value_type| value_type.clone())
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub fn exorcise_value_type(&mut self, id: &Uuid) -> Option<Arc<RwLock<ValueType>>> {
        self.value_type
            .write()
            .unwrap()
            .remove(id)
            .map(|value_type| value_type.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = Arc<RwLock<ValueType>>> + '_ {
        let values: Vec<Arc<RwLock<ValueType>>> = self
            .value_type
            .read()
            .unwrap()
            .values()
            .map(|value_type| value_type.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Arc<RwLock<Variable>>) {
        let read = variable.read().unwrap();
        self.variable
            .write()
            .unwrap()
            .insert(read.id, variable.clone());
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<Arc<RwLock<Variable>>> {
        self.variable
            .read()
            .unwrap()
            .get(id)
            .map(|variable| variable.clone())
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &Uuid) -> Option<Arc<RwLock<Variable>>> {
        self.variable
            .write()
            .unwrap()
            .remove(id)
            .map(|variable| variable.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = Arc<RwLock<Variable>>> + '_ {
        let values: Vec<Arc<RwLock<Variable>>> = self
            .variable
            .read()
            .unwrap()
            .values()
            .map(|variable| variable.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    pub fn inter_variable_expression(
        &mut self,
        variable_expression: Arc<RwLock<VariableExpression>>,
    ) {
        let read = variable_expression.read().unwrap();
        self.variable_expression
            .write()
            .unwrap()
            .insert(read.id, variable_expression.clone());
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(&self, id: &Uuid) -> Option<Arc<RwLock<VariableExpression>>> {
        self.variable_expression
            .read()
            .unwrap()
            .get(id)
            .map(|variable_expression| variable_expression.clone())
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub fn exorcise_variable_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        self.variable_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|variable_expression| variable_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<VariableExpression>>> + '_ {
        let values: Vec<Arc<RwLock<VariableExpression>>> = self
            .variable_expression
            .read()
            .unwrap()
            .values()
            .map(|variable_expression| variable_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock-object-store-persistence"}}}
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

        let path = path.join("lu_dog.json");
        fs::create_dir_all(&path)?;

        // Persist Argument.
        {
            let path = path.join("argument");
            fs::create_dir_all(&path)?;
            for argument in self.argument.read().unwrap().values() {
                let path = path.join(format!("{}.json", argument.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &argument)?;
            }
        }

        // Persist Await.
        {
            let path = path.join("a_wait");
            fs::create_dir_all(&path)?;
            for a_wait in self.a_wait.read().unwrap().values() {
                let path = path.join(format!("{}.json", a_wait.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &a_wait)?;
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary in self.binary.read().unwrap().values() {
                let path = path.join(format!("{}.json", binary.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &binary)?;
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block in self.block.read().unwrap().values() {
                let path = path.join(format!("{}.json", block.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &block)?;
            }
        }

        // Persist Body.
        {
            let path = path.join("body");
            fs::create_dir_all(&path)?;
            for body in self.body.read().unwrap().values() {
                let path = path.join(format!("{}.json", body.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &body)?;
            }
        }

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal in self.boolean_literal.read().unwrap().values() {
                let path = path.join(format!("{}.json", boolean_literal.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &boolean_literal)?;
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator in self.boolean_operator.read().unwrap().values() {
                let path = path.join(format!("{}.json", boolean_operator.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &boolean_operator)?;
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call in self.call.read().unwrap().values() {
                let path = path.join(format!("{}.json", call.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &call)?;
            }
        }

        // Persist Char Literal.
        {
            let path = path.join("char_literal");
            fs::create_dir_all(&path)?;
            for char_literal in self.char_literal.read().unwrap().values() {
                let path = path.join(format!("{}.json", char_literal.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &char_literal)?;
            }
        }

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison in self.comparison.read().unwrap().values() {
                let path = path.join(format!("{}.json", comparison.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &comparison)?;
            }
        }

        // Persist Data Structure.
        {
            let path = path.join("data_structure");
            fs::create_dir_all(&path)?;
            for data_structure in self.data_structure.read().unwrap().values() {
                let path = path.join(format!("{}.json", data_structure.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &data_structure)?;
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file in self.dwarf_source_file.read().unwrap().values() {
                let path = path.join(format!("{}.json", dwarf_source_file.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &dwarf_source_file)?;
            }
        }

        // Persist Enum Field.
        {
            let path = path.join("enum_field");
            fs::create_dir_all(&path)?;
            for enum_field in self.enum_field.read().unwrap().values() {
                let path = path.join(format!("{}.json", enum_field.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enum_field)?;
            }
        }

        // Persist Enum Generic.
        {
            let path = path.join("enum_generic");
            fs::create_dir_all(&path)?;
            for enum_generic in self.enum_generic.read().unwrap().values() {
                let path = path.join(format!("{}.json", enum_generic.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enum_generic)?;
            }
        }

        // Persist Enum Generic Type.
        {
            let path = path.join("enum_generic_type");
            fs::create_dir_all(&path)?;
            for enum_generic_type in self.enum_generic_type.read().unwrap().values() {
                let path = path.join(format!("{}.json", enum_generic_type.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enum_generic_type)?;
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration in self.enumeration.read().unwrap().values() {
                let path = path.join(format!("{}.json", enumeration.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enumeration)?;
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression in self.expression.read().unwrap().values() {
                let path = path.join(format!("{}.json", expression.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &expression)?;
            }
        }

        // Persist Expression Bit.
        {
            let path = path.join("expression_bit");
            fs::create_dir_all(&path)?;
            for expression_bit in self.expression_bit.read().unwrap().values() {
                let path = path.join(format!("{}.json", expression_bit.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &expression_bit)?;
            }
        }

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement in self.expression_statement.read().unwrap().values() {
                let path = path.join(format!("{}.json", expression_statement.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &expression_statement)?;
            }
        }

        // Persist External Implementation.
        {
            let path = path.join("external_implementation");
            fs::create_dir_all(&path)?;
            for external_implementation in self.external_implementation.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    external_implementation.read().unwrap().id
                ));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &external_implementation)?;
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field in self.field.read().unwrap().values() {
                let path = path.join(format!("{}.json", field.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field)?;
            }
        }

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access in self.field_access.read().unwrap().values() {
                let path = path.join(format!("{}.json", field_access.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field_access)?;
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target in self.field_access_target.read().unwrap().values() {
                let path = path.join(format!("{}.json", field_access_target.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field_access_target)?;
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression in self.field_expression.read().unwrap().values() {
                let path = path.join(format!("{}.json", field_expression.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field_expression)?;
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal in self.float_literal.read().unwrap().values() {
                let path = path.join(format!("{}.json", float_literal.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &float_literal)?;
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop in self.for_loop.read().unwrap().values() {
                let path = path.join(format!("{}.json", for_loop.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &for_loop)?;
            }
        }

        // Persist Format Bit.
        {
            let path = path.join("format_bit");
            fs::create_dir_all(&path)?;
            for format_bit in self.format_bit.read().unwrap().values() {
                let path = path.join(format!("{}.json", format_bit.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &format_bit)?;
            }
        }

        // Persist Format String.
        {
            let path = path.join("format_string");
            fs::create_dir_all(&path)?;
            for format_string in self.format_string.read().unwrap().values() {
                let path = path.join(format!("{}.json", format_string.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &format_string)?;
            }
        }

        // Persist Func Generic.
        {
            let path = path.join("func_generic");
            fs::create_dir_all(&path)?;
            for func_generic in self.func_generic.read().unwrap().values() {
                let path = path.join(format!("{}.json", func_generic.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &func_generic)?;
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function in self.function.read().unwrap().values() {
                let path = path.join(format!("{}.json", function.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &function)?;
            }
        }

        // Persist Function Call.
        {
            let path = path.join("function_call");
            fs::create_dir_all(&path)?;
            for function_call in self.function_call.read().unwrap().values() {
                let path = path.join(format!("{}.json", function_call.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &function_call)?;
            }
        }

        // Persist Future.
        {
            let path = path.join("x_future");
            fs::create_dir_all(&path)?;
            for x_future in self.x_future.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_future.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_future)?;
            }
        }

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped in self.grouped.read().unwrap().values() {
                let path = path.join(format!("{}.json", grouped.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &grouped)?;
            }
        }

        // Persist Halt and Catch Fire.
        {
            let path = path.join("halt_and_catch_fire");
            fs::create_dir_all(&path)?;
            for halt_and_catch_fire in self.halt_and_catch_fire.read().unwrap().values() {
                let path = path.join(format!("{}.json", halt_and_catch_fire.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &halt_and_catch_fire)?;
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if in self.x_if.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_if.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_if)?;
            }
        }

        // Persist Implementation Block.
        {
            let path = path.join("implementation_block");
            fs::create_dir_all(&path)?;
            for implementation_block in self.implementation_block.read().unwrap().values() {
                let path = path.join(format!("{}.json", implementation_block.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &implementation_block)?;
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import in self.import.read().unwrap().values() {
                let path = path.join(format!("{}.json", import.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &import)?;
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index in self.index.read().unwrap().values() {
                let path = path.join(format!("{}.json", index.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &index)?;
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal in self.integer_literal.read().unwrap().values() {
                let path = path.join(format!("{}.json", integer_literal.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &integer_literal)?;
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item in self.item.read().unwrap().values() {
                let path = path.join(format!("{}.json", item.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &item)?;
            }
        }

        // Persist Lambda.
        {
            let path = path.join("lambda");
            fs::create_dir_all(&path)?;
            for lambda in self.lambda.read().unwrap().values() {
                let path = path.join(format!("{}.json", lambda.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &lambda)?;
            }
        }

        // Persist Lambda Parameter.
        {
            let path = path.join("lambda_parameter");
            fs::create_dir_all(&path)?;
            for lambda_parameter in self.lambda_parameter.read().unwrap().values() {
                let path = path.join(format!("{}.json", lambda_parameter.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &lambda_parameter)?;
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement in self.let_statement.read().unwrap().values() {
                let path = path.join(format!("{}.json", let_statement.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &let_statement)?;
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list in self.list.read().unwrap().values() {
                let path = path.join(format!("{}.json", list.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &list)?;
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element in self.list_element.read().unwrap().values() {
                let path = path.join(format!("{}.json", list_element.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &list_element)?;
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression in self.list_expression.read().unwrap().values() {
                let path = path.join(format!("{}.json", list_expression.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &list_expression)?;
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal in self.literal.read().unwrap().values() {
                let path = path.join(format!("{}.json", literal.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &literal)?;
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable in self.local_variable.read().unwrap().values() {
                let path = path.join(format!("{}.json", local_variable.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &local_variable)?;
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro in self.x_macro.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_macro.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_macro)?;
            }
        }

        // Persist Map.
        {
            let path = path.join("map");
            fs::create_dir_all(&path)?;
            for map in self.map.read().unwrap().values() {
                let path = path.join(format!("{}.json", map.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &map)?;
            }
        }

        // Persist Map Element.
        {
            let path = path.join("map_element");
            fs::create_dir_all(&path)?;
            for map_element in self.map_element.read().unwrap().values() {
                let path = path.join(format!("{}.json", map_element.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &map_element)?;
            }
        }

        // Persist Map Expression.
        {
            let path = path.join("map_expression");
            fs::create_dir_all(&path)?;
            for map_expression in self.map_expression.read().unwrap().values() {
                let path = path.join(format!("{}.json", map_expression.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &map_expression)?;
            }
        }

        // Persist Match.
        {
            let path = path.join("x_match");
            fs::create_dir_all(&path)?;
            for x_match in self.x_match.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_match.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_match)?;
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call in self.method_call.read().unwrap().values() {
                let path = path.join(format!("{}.json", method_call.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &method_call)?;
            }
        }

        // Persist Named Field Expression.
        {
            let path = path.join("named_field_expression");
            fs::create_dir_all(&path)?;
            for named_field_expression in self.named_field_expression.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    named_field_expression.read().unwrap().id
                ));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &named_field_expression)?;
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store in self.z_object_store.read().unwrap().values() {
                let path = path.join(format!("{}.json", z_object_store.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &z_object_store)?;
            }
        }

        // Persist Object Wrapper.
        {
            let path = path.join("object_wrapper");
            fs::create_dir_all(&path)?;
            for object_wrapper in self.object_wrapper.read().unwrap().values() {
                let path = path.join(format!("{}.json", object_wrapper.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &object_wrapper)?;
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator in self.operator.read().unwrap().values() {
                let path = path.join(format!("{}.json", operator.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &operator)?;
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter in self.parameter.read().unwrap().values() {
                let path = path.join(format!("{}.json", parameter.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &parameter)?;
            }
        }

        // Persist Path.
        {
            let path = path.join("x_path");
            fs::create_dir_all(&path)?;
            for x_path in self.x_path.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_path.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_path)?;
            }
        }

        // Persist Path Element.
        {
            let path = path.join("path_element");
            fs::create_dir_all(&path)?;
            for path_element in self.path_element.read().unwrap().values() {
                let path = path.join(format!("{}.json", path_element.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &path_element)?;
            }
        }

        // Persist Pattern.
        {
            let path = path.join("pattern");
            fs::create_dir_all(&path)?;
            for pattern in self.pattern.read().unwrap().values() {
                let path = path.join(format!("{}.json", pattern.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &pattern)?;
            }
        }

        // Persist Plugin.
        {
            let path = path.join("x_plugin");
            fs::create_dir_all(&path)?;
            for x_plugin in self.x_plugin.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_plugin.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_plugin)?;
            }
        }

        // Persist Print.
        {
            let path = path.join("x_print");
            fs::create_dir_all(&path)?;
            for x_print in self.x_print.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_print.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_print)?;
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression in self.range_expression.read().unwrap().values() {
                let path = path.join(format!("{}.json", range_expression.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &range_expression)?;
            }
        }

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement in self.result_statement.read().unwrap().values() {
                let path = path.join(format!("{}.json", result_statement.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &result_statement)?;
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return in self.x_return.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_return.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_return)?;
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span in self.span.read().unwrap().values() {
                let path = path.join(format!("{}.json", span.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &span)?;
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement in self.statement.read().unwrap().values() {
                let path = path.join(format!("{}.json", statement.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &statement)?;
            }
        }

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call in self.static_method_call.read().unwrap().values() {
                let path = path.join(format!("{}.json", static_method_call.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &static_method_call)?;
            }
        }

        // Persist String Bit.
        {
            let path = path.join("string_bit");
            fs::create_dir_all(&path)?;
            for string_bit in self.string_bit.read().unwrap().values() {
                let path = path.join(format!("{}.json", string_bit.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &string_bit)?;
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal in self.string_literal.read().unwrap().values() {
                let path = path.join(format!("{}.json", string_literal.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &string_literal)?;
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct in self.woog_struct.read().unwrap().values() {
                let path = path.join(format!("{}.json", woog_struct.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &woog_struct)?;
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression in self.struct_expression.read().unwrap().values() {
                let path = path.join(format!("{}.json", struct_expression.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &struct_expression)?;
            }
        }

        // Persist Struct Field.
        {
            let path = path.join("struct_field");
            fs::create_dir_all(&path)?;
            for struct_field in self.struct_field.read().unwrap().values() {
                let path = path.join(format!("{}.json", struct_field.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &struct_field)?;
            }
        }

        // Persist Struct Generic.
        {
            let path = path.join("struct_generic");
            fs::create_dir_all(&path)?;
            for struct_generic in self.struct_generic.read().unwrap().values() {
                let path = path.join(format!("{}.json", struct_generic.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &struct_generic)?;
            }
        }

        // Persist Tuple Field.
        {
            let path = path.join("tuple_field");
            fs::create_dir_all(&path)?;
            for tuple_field in self.tuple_field.read().unwrap().values() {
                let path = path.join(format!("{}.json", tuple_field.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &tuple_field)?;
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast in self.type_cast.read().unwrap().values() {
                let path = path.join(format!("{}.json", type_cast.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &type_cast)?;
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary in self.unary.read().unwrap().values() {
                let path = path.join(format!("{}.json", unary.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &unary)?;
            }
        }

        // Persist Unit.
        {
            let path = path.join("unit");
            fs::create_dir_all(&path)?;
            for unit in self.unit.read().unwrap().values() {
                let path = path.join(format!("{}.json", unit.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &unit)?;
            }
        }

        // Persist Unnamed Field Expression.
        {
            let path = path.join("unnamed_field_expression");
            fs::create_dir_all(&path)?;
            for unnamed_field_expression in self.unnamed_field_expression.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    unnamed_field_expression.read().unwrap().id
                ));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &unnamed_field_expression)?;
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value in self.x_value.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_value.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_value)?;
            }
        }

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type in self.value_type.read().unwrap().values() {
                let path = path.join(format!("{}.json", value_type.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &value_type)?;
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable in self.variable.read().unwrap().values() {
                let path = path.join(format!("{}.json", variable.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &variable)?;
            }
        }

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression in self.variable_expression.read().unwrap().values() {
                let path = path.join(format!("{}.json", variable_expression.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &variable_expression)?;
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
        let path = path.join("lu_dog.json");

        let store = Self::new();

        // Load Argument.
        {
            let path = path.join("argument");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let argument: Arc<RwLock<Argument>> = serde_json::from_reader(reader)?;
                store
                    .argument
                    .write()
                    .unwrap()
                    .insert(argument.read().unwrap().id, argument.clone());
            }
        }

        // Load Await.
        {
            let path = path.join("a_wait");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let a_wait: Arc<RwLock<AWait>> = serde_json::from_reader(reader)?;
                store
                    .a_wait
                    .write()
                    .unwrap()
                    .insert(a_wait.read().unwrap().id, a_wait.clone());
            }
        }

        // Load Binary.
        {
            let path = path.join("binary");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let binary: Arc<RwLock<Binary>> = serde_json::from_reader(reader)?;
                store
                    .binary
                    .write()
                    .unwrap()
                    .insert(binary.read().unwrap().id, binary.clone());
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
                let block: Arc<RwLock<Block>> = serde_json::from_reader(reader)?;
                store
                    .block
                    .write()
                    .unwrap()
                    .insert(block.read().unwrap().id, block.clone());
            }
        }

        // Load Body.
        {
            let path = path.join("body");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let body: Arc<RwLock<Body>> = serde_json::from_reader(reader)?;
                store
                    .body
                    .write()
                    .unwrap()
                    .insert(body.read().unwrap().id, body.clone());
            }
        }

        // Load Boolean Literal.
        {
            let path = path.join("boolean_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let boolean_literal: Arc<RwLock<BooleanLiteral>> = serde_json::from_reader(reader)?;
                store
                    .boolean_literal
                    .write()
                    .unwrap()
                    .insert(boolean_literal.read().unwrap().id, boolean_literal.clone());
            }
        }

        // Load Boolean Operator.
        {
            let path = path.join("boolean_operator");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let boolean_operator: Arc<RwLock<BooleanOperator>> =
                    serde_json::from_reader(reader)?;
                store.boolean_operator.write().unwrap().insert(
                    boolean_operator.read().unwrap().id,
                    boolean_operator.clone(),
                );
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
                let call: Arc<RwLock<Call>> = serde_json::from_reader(reader)?;
                store
                    .call
                    .write()
                    .unwrap()
                    .insert(call.read().unwrap().id, call.clone());
            }
        }

        // Load Char Literal.
        {
            let path = path.join("char_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let char_literal: Arc<RwLock<CharLiteral>> = serde_json::from_reader(reader)?;
                store
                    .char_literal
                    .write()
                    .unwrap()
                    .insert(char_literal.read().unwrap().id, char_literal.clone());
            }
        }

        // Load Comparison.
        {
            let path = path.join("comparison");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let comparison: Arc<RwLock<Comparison>> = serde_json::from_reader(reader)?;
                store
                    .comparison
                    .write()
                    .unwrap()
                    .insert(comparison.read().unwrap().id, comparison.clone());
            }
        }

        // Load Data Structure.
        {
            let path = path.join("data_structure");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let data_structure: Arc<RwLock<DataStructure>> = serde_json::from_reader(reader)?;
                store
                    .data_structure
                    .write()
                    .unwrap()
                    .insert(data_structure.read().unwrap().id, data_structure.clone());
            }
        }

        // Load Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let dwarf_source_file: Arc<RwLock<DwarfSourceFile>> =
                    serde_json::from_reader(reader)?;
                store.dwarf_source_file.write().unwrap().insert(
                    dwarf_source_file.read().unwrap().id,
                    dwarf_source_file.clone(),
                );
            }
        }

        // Load Enum Field.
        {
            let path = path.join("enum_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enum_field: Arc<RwLock<EnumField>> = serde_json::from_reader(reader)?;
                store
                    .enum_field
                    .write()
                    .unwrap()
                    .insert(enum_field.read().unwrap().id, enum_field.clone());
            }
        }

        // Load Enum Generic.
        {
            let path = path.join("enum_generic");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enum_generic: Arc<RwLock<EnumGeneric>> = serde_json::from_reader(reader)?;
                store
                    .enum_generic
                    .write()
                    .unwrap()
                    .insert(enum_generic.read().unwrap().id, enum_generic.clone());
            }
        }

        // Load Enum Generic Type.
        {
            let path = path.join("enum_generic_type");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enum_generic_type: Arc<RwLock<EnumGenericType>> =
                    serde_json::from_reader(reader)?;
                store.enum_generic_type.write().unwrap().insert(
                    enum_generic_type.read().unwrap().id,
                    enum_generic_type.clone(),
                );
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
                let enumeration: Arc<RwLock<Enumeration>> = serde_json::from_reader(reader)?;
                store.enumeration_id_by_name.write().unwrap().insert(
                    enumeration.read().unwrap().name.clone(),
                    enumeration.read().unwrap().id,
                );
                store
                    .enumeration
                    .write()
                    .unwrap()
                    .insert(enumeration.read().unwrap().id, enumeration.clone());
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
                let expression: Arc<RwLock<Expression>> = serde_json::from_reader(reader)?;
                store
                    .expression
                    .write()
                    .unwrap()
                    .insert(expression.read().unwrap().id, expression.clone());
            }
        }

        // Load Expression Bit.
        {
            let path = path.join("expression_bit");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let expression_bit: Arc<RwLock<ExpressionBit>> = serde_json::from_reader(reader)?;
                store
                    .expression_bit
                    .write()
                    .unwrap()
                    .insert(expression_bit.read().unwrap().id, expression_bit.clone());
            }
        }

        // Load Expression Statement.
        {
            let path = path.join("expression_statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let expression_statement: Arc<RwLock<ExpressionStatement>> =
                    serde_json::from_reader(reader)?;
                store.expression_statement.write().unwrap().insert(
                    expression_statement.read().unwrap().id,
                    expression_statement.clone(),
                );
            }
        }

        // Load External Implementation.
        {
            let path = path.join("external_implementation");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let external_implementation: Arc<RwLock<ExternalImplementation>> =
                    serde_json::from_reader(reader)?;
                store.external_implementation.write().unwrap().insert(
                    external_implementation.read().unwrap().id,
                    external_implementation.clone(),
                );
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
                let field: Arc<RwLock<Field>> = serde_json::from_reader(reader)?;
                store
                    .field_id_by_name
                    .write()
                    .unwrap()
                    .insert(field.read().unwrap().name.clone(), field.read().unwrap().id);
                store
                    .field
                    .write()
                    .unwrap()
                    .insert(field.read().unwrap().id, field.clone());
            }
        }

        // Load Field Access.
        {
            let path = path.join("field_access");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field_access: Arc<RwLock<FieldAccess>> = serde_json::from_reader(reader)?;
                store
                    .field_access
                    .write()
                    .unwrap()
                    .insert(field_access.read().unwrap().id, field_access.clone());
            }
        }

        // Load Field Access Target.
        {
            let path = path.join("field_access_target");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field_access_target: Arc<RwLock<FieldAccessTarget>> =
                    serde_json::from_reader(reader)?;
                store.field_access_target.write().unwrap().insert(
                    field_access_target.read().unwrap().id,
                    field_access_target.clone(),
                );
            }
        }

        // Load Field Expression.
        {
            let path = path.join("field_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field_expression: Arc<RwLock<FieldExpression>> =
                    serde_json::from_reader(reader)?;
                store.field_expression.write().unwrap().insert(
                    field_expression.read().unwrap().id,
                    field_expression.clone(),
                );
            }
        }

        // Load Float Literal.
        {
            let path = path.join("float_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let float_literal: Arc<RwLock<FloatLiteral>> = serde_json::from_reader(reader)?;
                store
                    .float_literal
                    .write()
                    .unwrap()
                    .insert(float_literal.read().unwrap().id, float_literal.clone());
            }
        }

        // Load For Loop.
        {
            let path = path.join("for_loop");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let for_loop: Arc<RwLock<ForLoop>> = serde_json::from_reader(reader)?;
                store
                    .for_loop
                    .write()
                    .unwrap()
                    .insert(for_loop.read().unwrap().id, for_loop.clone());
            }
        }

        // Load Format Bit.
        {
            let path = path.join("format_bit");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let format_bit: Arc<RwLock<FormatBit>> = serde_json::from_reader(reader)?;
                store
                    .format_bit
                    .write()
                    .unwrap()
                    .insert(format_bit.read().unwrap().id, format_bit.clone());
            }
        }

        // Load Format String.
        {
            let path = path.join("format_string");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let format_string: Arc<RwLock<FormatString>> = serde_json::from_reader(reader)?;
                store
                    .format_string
                    .write()
                    .unwrap()
                    .insert(format_string.read().unwrap().id, format_string.clone());
            }
        }

        // Load Func Generic.
        {
            let path = path.join("func_generic");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let func_generic: Arc<RwLock<FuncGeneric>> = serde_json::from_reader(reader)?;
                store
                    .func_generic
                    .write()
                    .unwrap()
                    .insert(func_generic.read().unwrap().id, func_generic.clone());
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
                let function: Arc<RwLock<Function>> = serde_json::from_reader(reader)?;
                store.function_id_by_name.write().unwrap().insert(
                    function.read().unwrap().name.clone(),
                    function.read().unwrap().id,
                );
                store
                    .function
                    .write()
                    .unwrap()
                    .insert(function.read().unwrap().id, function.clone());
            }
        }

        // Load Function Call.
        {
            let path = path.join("function_call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let function_call: Arc<RwLock<FunctionCall>> = serde_json::from_reader(reader)?;
                store
                    .function_call
                    .write()
                    .unwrap()
                    .insert(function_call.read().unwrap().id, function_call.clone());
            }
        }

        // Load Future.
        {
            let path = path.join("x_future");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_future: Arc<RwLock<XFuture>> = serde_json::from_reader(reader)?;
                store
                    .x_future
                    .write()
                    .unwrap()
                    .insert(x_future.read().unwrap().id, x_future.clone());
            }
        }

        // Load Grouped.
        {
            let path = path.join("grouped");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let grouped: Arc<RwLock<Grouped>> = serde_json::from_reader(reader)?;
                store
                    .grouped
                    .write()
                    .unwrap()
                    .insert(grouped.read().unwrap().id, grouped.clone());
            }
        }

        // Load Halt and Catch Fire.
        {
            let path = path.join("halt_and_catch_fire");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let halt_and_catch_fire: Arc<RwLock<HaltAndCatchFire>> =
                    serde_json::from_reader(reader)?;
                store.halt_and_catch_fire.write().unwrap().insert(
                    halt_and_catch_fire.read().unwrap().id,
                    halt_and_catch_fire.clone(),
                );
            }
        }

        // Load If.
        {
            let path = path.join("x_if");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_if: Arc<RwLock<XIf>> = serde_json::from_reader(reader)?;
                store
                    .x_if
                    .write()
                    .unwrap()
                    .insert(x_if.read().unwrap().id, x_if.clone());
            }
        }

        // Load Implementation Block.
        {
            let path = path.join("implementation_block");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let implementation_block: Arc<RwLock<ImplementationBlock>> =
                    serde_json::from_reader(reader)?;
                store.implementation_block.write().unwrap().insert(
                    implementation_block.read().unwrap().id,
                    implementation_block.clone(),
                );
            }
        }

        // Load Import.
        {
            let path = path.join("import");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let import: Arc<RwLock<Import>> = serde_json::from_reader(reader)?;
                store
                    .import
                    .write()
                    .unwrap()
                    .insert(import.read().unwrap().id, import.clone());
            }
        }

        // Load Index.
        {
            let path = path.join("index");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let index: Arc<RwLock<Index>> = serde_json::from_reader(reader)?;
                store
                    .index
                    .write()
                    .unwrap()
                    .insert(index.read().unwrap().id, index.clone());
            }
        }

        // Load Integer Literal.
        {
            let path = path.join("integer_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let integer_literal: Arc<RwLock<IntegerLiteral>> = serde_json::from_reader(reader)?;
                store
                    .integer_literal
                    .write()
                    .unwrap()
                    .insert(integer_literal.read().unwrap().id, integer_literal.clone());
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
                let item: Arc<RwLock<Item>> = serde_json::from_reader(reader)?;
                store
                    .item
                    .write()
                    .unwrap()
                    .insert(item.read().unwrap().id, item.clone());
            }
        }

        // Load Lambda.
        {
            let path = path.join("lambda");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let lambda: Arc<RwLock<Lambda>> = serde_json::from_reader(reader)?;
                store
                    .lambda
                    .write()
                    .unwrap()
                    .insert(lambda.read().unwrap().id, lambda.clone());
            }
        }

        // Load Lambda Parameter.
        {
            let path = path.join("lambda_parameter");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let lambda_parameter: Arc<RwLock<LambdaParameter>> =
                    serde_json::from_reader(reader)?;
                store.lambda_parameter.write().unwrap().insert(
                    lambda_parameter.read().unwrap().id,
                    lambda_parameter.clone(),
                );
            }
        }

        // Load Let Statement.
        {
            let path = path.join("let_statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let let_statement: Arc<RwLock<LetStatement>> = serde_json::from_reader(reader)?;
                store
                    .let_statement
                    .write()
                    .unwrap()
                    .insert(let_statement.read().unwrap().id, let_statement.clone());
            }
        }

        // Load List.
        {
            let path = path.join("list");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let list: Arc<RwLock<List>> = serde_json::from_reader(reader)?;
                store
                    .list
                    .write()
                    .unwrap()
                    .insert(list.read().unwrap().id, list.clone());
            }
        }

        // Load List Element.
        {
            let path = path.join("list_element");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let list_element: Arc<RwLock<ListElement>> = serde_json::from_reader(reader)?;
                store
                    .list_element
                    .write()
                    .unwrap()
                    .insert(list_element.read().unwrap().id, list_element.clone());
            }
        }

        // Load List Expression.
        {
            let path = path.join("list_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let list_expression: Arc<RwLock<ListExpression>> = serde_json::from_reader(reader)?;
                store
                    .list_expression
                    .write()
                    .unwrap()
                    .insert(list_expression.read().unwrap().id, list_expression.clone());
            }
        }

        // Load Literal.
        {
            let path = path.join("literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let literal: Arc<RwLock<Literal>> = serde_json::from_reader(reader)?;
                store
                    .literal
                    .write()
                    .unwrap()
                    .insert(literal.read().unwrap().id, literal.clone());
            }
        }

        // Load Local Variable.
        {
            let path = path.join("local_variable");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let local_variable: Arc<RwLock<LocalVariable>> = serde_json::from_reader(reader)?;
                store
                    .local_variable
                    .write()
                    .unwrap()
                    .insert(local_variable.read().unwrap().id, local_variable.clone());
            }
        }

        // Load Macro.
        {
            let path = path.join("x_macro");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_macro: Arc<RwLock<XMacro>> = serde_json::from_reader(reader)?;
                store
                    .x_macro
                    .write()
                    .unwrap()
                    .insert(x_macro.read().unwrap().id, x_macro.clone());
            }
        }

        // Load Map.
        {
            let path = path.join("map");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let map: Arc<RwLock<Map>> = serde_json::from_reader(reader)?;
                store
                    .map
                    .write()
                    .unwrap()
                    .insert(map.read().unwrap().id, map.clone());
            }
        }

        // Load Map Element.
        {
            let path = path.join("map_element");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let map_element: Arc<RwLock<MapElement>> = serde_json::from_reader(reader)?;
                store
                    .map_element
                    .write()
                    .unwrap()
                    .insert(map_element.read().unwrap().id, map_element.clone());
            }
        }

        // Load Map Expression.
        {
            let path = path.join("map_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let map_expression: Arc<RwLock<MapExpression>> = serde_json::from_reader(reader)?;
                store
                    .map_expression
                    .write()
                    .unwrap()
                    .insert(map_expression.read().unwrap().id, map_expression.clone());
            }
        }

        // Load Match.
        {
            let path = path.join("x_match");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_match: Arc<RwLock<XMatch>> = serde_json::from_reader(reader)?;
                store
                    .x_match
                    .write()
                    .unwrap()
                    .insert(x_match.read().unwrap().id, x_match.clone());
            }
        }

        // Load Method Call.
        {
            let path = path.join("method_call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let method_call: Arc<RwLock<MethodCall>> = serde_json::from_reader(reader)?;
                store
                    .method_call
                    .write()
                    .unwrap()
                    .insert(method_call.read().unwrap().id, method_call.clone());
            }
        }

        // Load Named Field Expression.
        {
            let path = path.join("named_field_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let named_field_expression: Arc<RwLock<NamedFieldExpression>> =
                    serde_json::from_reader(reader)?;
                store.named_field_expression.write().unwrap().insert(
                    named_field_expression.read().unwrap().id,
                    named_field_expression.clone(),
                );
            }
        }

        // Load Object Store.
        {
            let path = path.join("z_object_store");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let z_object_store: Arc<RwLock<ZObjectStore>> = serde_json::from_reader(reader)?;
                store.z_object_store_id_by_name.write().unwrap().insert(
                    z_object_store.read().unwrap().name.clone(),
                    z_object_store.read().unwrap().id,
                );
                store
                    .z_object_store
                    .write()
                    .unwrap()
                    .insert(z_object_store.read().unwrap().id, z_object_store.clone());
            }
        }

        // Load Object Wrapper.
        {
            let path = path.join("object_wrapper");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object_wrapper: Arc<RwLock<ObjectWrapper>> = serde_json::from_reader(reader)?;
                store
                    .object_wrapper
                    .write()
                    .unwrap()
                    .insert(object_wrapper.read().unwrap().id, object_wrapper.clone());
            }
        }

        // Load Operator.
        {
            let path = path.join("operator");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let operator: Arc<RwLock<Operator>> = serde_json::from_reader(reader)?;
                store
                    .operator
                    .write()
                    .unwrap()
                    .insert(operator.read().unwrap().id, operator.clone());
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
                let parameter: Arc<RwLock<Parameter>> = serde_json::from_reader(reader)?;
                store
                    .parameter
                    .write()
                    .unwrap()
                    .insert(parameter.read().unwrap().id, parameter.clone());
            }
        }

        // Load Path.
        {
            let path = path.join("x_path");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_path: Arc<RwLock<XPath>> = serde_json::from_reader(reader)?;
                store
                    .x_path
                    .write()
                    .unwrap()
                    .insert(x_path.read().unwrap().id, x_path.clone());
            }
        }

        // Load Path Element.
        {
            let path = path.join("path_element");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let path_element: Arc<RwLock<PathElement>> = serde_json::from_reader(reader)?;
                store
                    .path_element
                    .write()
                    .unwrap()
                    .insert(path_element.read().unwrap().id, path_element.clone());
            }
        }

        // Load Pattern.
        {
            let path = path.join("pattern");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let pattern: Arc<RwLock<Pattern>> = serde_json::from_reader(reader)?;
                store
                    .pattern
                    .write()
                    .unwrap()
                    .insert(pattern.read().unwrap().id, pattern.clone());
            }
        }

        // Load Plugin.
        {
            let path = path.join("x_plugin");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_plugin: Arc<RwLock<XPlugin>> = serde_json::from_reader(reader)?;
                store.x_plugin_id_by_name.write().unwrap().insert(
                    x_plugin.read().unwrap().name.clone(),
                    x_plugin.read().unwrap().id,
                );
                store
                    .x_plugin
                    .write()
                    .unwrap()
                    .insert(x_plugin.read().unwrap().id, x_plugin.clone());
            }
        }

        // Load Print.
        {
            let path = path.join("x_print");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_print: Arc<RwLock<XPrint>> = serde_json::from_reader(reader)?;
                store
                    .x_print
                    .write()
                    .unwrap()
                    .insert(x_print.read().unwrap().id, x_print.clone());
            }
        }

        // Load Range Expression.
        {
            let path = path.join("range_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let range_expression: Arc<RwLock<RangeExpression>> =
                    serde_json::from_reader(reader)?;
                store.range_expression.write().unwrap().insert(
                    range_expression.read().unwrap().id,
                    range_expression.clone(),
                );
            }
        }

        // Load Result Statement.
        {
            let path = path.join("result_statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let result_statement: Arc<RwLock<ResultStatement>> =
                    serde_json::from_reader(reader)?;
                store.result_statement.write().unwrap().insert(
                    result_statement.read().unwrap().id,
                    result_statement.clone(),
                );
            }
        }

        // Load Return.
        {
            let path = path.join("x_return");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_return: Arc<RwLock<XReturn>> = serde_json::from_reader(reader)?;
                store
                    .x_return
                    .write()
                    .unwrap()
                    .insert(x_return.read().unwrap().id, x_return.clone());
            }
        }

        // Load Span.
        {
            let path = path.join("span");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let span: Arc<RwLock<Span>> = serde_json::from_reader(reader)?;
                store
                    .span
                    .write()
                    .unwrap()
                    .insert(span.read().unwrap().id, span.clone());
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
                let statement: Arc<RwLock<Statement>> = serde_json::from_reader(reader)?;
                store
                    .statement
                    .write()
                    .unwrap()
                    .insert(statement.read().unwrap().id, statement.clone());
            }
        }

        // Load Static Method Call.
        {
            let path = path.join("static_method_call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let static_method_call: Arc<RwLock<StaticMethodCall>> =
                    serde_json::from_reader(reader)?;
                store.static_method_call.write().unwrap().insert(
                    static_method_call.read().unwrap().id,
                    static_method_call.clone(),
                );
            }
        }

        // Load String Bit.
        {
            let path = path.join("string_bit");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let string_bit: Arc<RwLock<StringBit>> = serde_json::from_reader(reader)?;
                store
                    .string_bit
                    .write()
                    .unwrap()
                    .insert(string_bit.read().unwrap().id, string_bit.clone());
            }
        }

        // Load String Literal.
        {
            let path = path.join("string_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let string_literal: Arc<RwLock<StringLiteral>> = serde_json::from_reader(reader)?;
                store
                    .string_literal
                    .write()
                    .unwrap()
                    .insert(string_literal.read().unwrap().id, string_literal.clone());
            }
        }

        // Load Struct.
        {
            let path = path.join("woog_struct");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let woog_struct: Arc<RwLock<WoogStruct>> = serde_json::from_reader(reader)?;
                store.woog_struct_id_by_name.write().unwrap().insert(
                    woog_struct.read().unwrap().name.clone(),
                    woog_struct.read().unwrap().id,
                );
                store
                    .woog_struct
                    .write()
                    .unwrap()
                    .insert(woog_struct.read().unwrap().id, woog_struct.clone());
            }
        }

        // Load Struct Expression.
        {
            let path = path.join("struct_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let struct_expression: Arc<RwLock<StructExpression>> =
                    serde_json::from_reader(reader)?;
                store.struct_expression.write().unwrap().insert(
                    struct_expression.read().unwrap().id,
                    struct_expression.clone(),
                );
            }
        }

        // Load Struct Field.
        {
            let path = path.join("struct_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let struct_field: Arc<RwLock<StructField>> = serde_json::from_reader(reader)?;
                store
                    .struct_field
                    .write()
                    .unwrap()
                    .insert(struct_field.read().unwrap().id, struct_field.clone());
            }
        }

        // Load Struct Generic.
        {
            let path = path.join("struct_generic");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let struct_generic: Arc<RwLock<StructGeneric>> = serde_json::from_reader(reader)?;
                store
                    .struct_generic
                    .write()
                    .unwrap()
                    .insert(struct_generic.read().unwrap().id, struct_generic.clone());
            }
        }

        // Load Tuple Field.
        {
            let path = path.join("tuple_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let tuple_field: Arc<RwLock<TupleField>> = serde_json::from_reader(reader)?;
                store
                    .tuple_field
                    .write()
                    .unwrap()
                    .insert(tuple_field.read().unwrap().id, tuple_field.clone());
            }
        }

        // Load Type Cast.
        {
            let path = path.join("type_cast");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let type_cast: Arc<RwLock<TypeCast>> = serde_json::from_reader(reader)?;
                store
                    .type_cast
                    .write()
                    .unwrap()
                    .insert(type_cast.read().unwrap().id, type_cast.clone());
            }
        }

        // Load Unary.
        {
            let path = path.join("unary");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let unary: Arc<RwLock<Unary>> = serde_json::from_reader(reader)?;
                store
                    .unary
                    .write()
                    .unwrap()
                    .insert(unary.read().unwrap().id, unary.clone());
            }
        }

        // Load Unit.
        {
            let path = path.join("unit");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let unit: Arc<RwLock<Unit>> = serde_json::from_reader(reader)?;
                store
                    .unit
                    .write()
                    .unwrap()
                    .insert(unit.read().unwrap().id, unit.clone());
            }
        }

        // Load Unnamed Field Expression.
        {
            let path = path.join("unnamed_field_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let unnamed_field_expression: Arc<RwLock<UnnamedFieldExpression>> =
                    serde_json::from_reader(reader)?;
                store.unnamed_field_expression.write().unwrap().insert(
                    unnamed_field_expression.read().unwrap().id,
                    unnamed_field_expression.clone(),
                );
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
                let x_value: Arc<RwLock<XValue>> = serde_json::from_reader(reader)?;
                store
                    .x_value
                    .write()
                    .unwrap()
                    .insert(x_value.read().unwrap().id, x_value.clone());
            }
        }

        // Load Value Type.
        {
            let path = path.join("value_type");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let value_type: Arc<RwLock<ValueType>> = serde_json::from_reader(reader)?;
                store
                    .value_type
                    .write()
                    .unwrap()
                    .insert(value_type.read().unwrap().id, value_type.clone());
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
                let variable: Arc<RwLock<Variable>> = serde_json::from_reader(reader)?;
                store
                    .variable
                    .write()
                    .unwrap()
                    .insert(variable.read().unwrap().id, variable.clone());
            }
        }

        // Load Variable Expression.
        {
            let path = path.join("variable_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let variable_expression: Arc<RwLock<VariableExpression>> =
                    serde_json::from_reader(reader)?;
                store.variable_expression.write().unwrap().insert(
                    variable_expression.read().unwrap().id,
                    variable_expression.clone(),
                );
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
