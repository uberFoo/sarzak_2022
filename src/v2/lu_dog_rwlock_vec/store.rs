//! v2::lu_dog_rwlock_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock_vec-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock_vec-object-store-definition"}}}
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::{
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
    XFuture, XIf, XMacro, XMatch, XPath, XPlugin, XPrint, XReturn, XValue, ZObjectStore, ADDITION,
    AND, ANY_LIST, ASSIGNMENT, CHAR, DIVISION, EMPTY, EMPTY_EXPRESSION, EQUAL, FALSE_LITERAL, FROM,
    FULL, GREATER_THAN, GREATER_THAN_OR_EQUAL, INCLUSIVE, ITEM_STATEMENT, LESS_THAN,
    LESS_THAN_OR_EQUAL, MACRO_CALL, MULTIPLICATION, NEGATION, NOT, NOT_EQUAL, OR, RANGE,
    SUBTRACTION, TASK, TO, TO_INCLUSIVE, TRUE_LITERAL, UNKNOWN, X_DEBUGGER,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument_free_list: std::sync::Mutex<Vec<usize>>,
    argument: Arc<RwLock<Vec<Option<Arc<RwLock<Argument>>>>>>,
    argument_dirty: bool,
    a_wait_free_list: std::sync::Mutex<Vec<usize>>,
    a_wait: Arc<RwLock<Vec<Option<Arc<RwLock<AWait>>>>>>,
    a_wait_dirty: bool,
    binary_free_list: std::sync::Mutex<Vec<usize>>,
    binary: Arc<RwLock<Vec<Option<Arc<RwLock<Binary>>>>>>,
    binary_dirty: bool,
    block_free_list: std::sync::Mutex<Vec<usize>>,
    block: Arc<RwLock<Vec<Option<Arc<RwLock<Block>>>>>>,
    block_dirty: bool,
    body_free_list: std::sync::Mutex<Vec<usize>>,
    body: Arc<RwLock<Vec<Option<Arc<RwLock<Body>>>>>>,
    body_dirty: bool,
    boolean_literal_free_list: std::sync::Mutex<Vec<usize>>,
    boolean_literal: Arc<RwLock<Vec<Option<Arc<RwLock<BooleanLiteral>>>>>>,
    boolean_literal_dirty: bool,
    boolean_operator_free_list: std::sync::Mutex<Vec<usize>>,
    boolean_operator: Arc<RwLock<Vec<Option<Arc<RwLock<BooleanOperator>>>>>>,
    boolean_operator_dirty: bool,
    call_free_list: std::sync::Mutex<Vec<usize>>,
    call: Arc<RwLock<Vec<Option<Arc<RwLock<Call>>>>>>,
    call_dirty: bool,
    char_literal_free_list: std::sync::Mutex<Vec<usize>>,
    char_literal: Arc<RwLock<Vec<Option<Arc<RwLock<CharLiteral>>>>>>,
    char_literal_dirty: bool,
    comparison_free_list: std::sync::Mutex<Vec<usize>>,
    comparison: Arc<RwLock<Vec<Option<Arc<RwLock<Comparison>>>>>>,
    comparison_dirty: bool,
    data_structure_free_list: std::sync::Mutex<Vec<usize>>,
    data_structure: Arc<RwLock<Vec<Option<Arc<RwLock<DataStructure>>>>>>,
    data_structure_dirty: bool,
    dwarf_source_file_free_list: std::sync::Mutex<Vec<usize>>,
    dwarf_source_file: Arc<RwLock<Vec<Option<Arc<RwLock<DwarfSourceFile>>>>>>,
    dwarf_source_file_dirty: bool,
    enum_field_free_list: std::sync::Mutex<Vec<usize>>,
    enum_field: Arc<RwLock<Vec<Option<Arc<RwLock<EnumField>>>>>>,
    enum_field_dirty: bool,
    enum_generic_free_list: std::sync::Mutex<Vec<usize>>,
    enum_generic: Arc<RwLock<Vec<Option<Arc<RwLock<EnumGeneric>>>>>>,
    enum_generic_dirty: bool,
    enum_generic_type_free_list: std::sync::Mutex<Vec<usize>>,
    enum_generic_type: Arc<RwLock<Vec<Option<Arc<RwLock<EnumGenericType>>>>>>,
    enum_generic_type_dirty: bool,
    enumeration_free_list: std::sync::Mutex<Vec<usize>>,
    enumeration: Arc<RwLock<Vec<Option<Arc<RwLock<Enumeration>>>>>>,
    enumeration_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    enumeration_dirty: bool,
    expression_free_list: std::sync::Mutex<Vec<usize>>,
    expression: Arc<RwLock<Vec<Option<Arc<RwLock<Expression>>>>>>,
    expression_dirty: bool,
    expression_bit_free_list: std::sync::Mutex<Vec<usize>>,
    expression_bit: Arc<RwLock<Vec<Option<Arc<RwLock<ExpressionBit>>>>>>,
    expression_bit_dirty: bool,
    expression_statement_free_list: std::sync::Mutex<Vec<usize>>,
    expression_statement: Arc<RwLock<Vec<Option<Arc<RwLock<ExpressionStatement>>>>>>,
    expression_statement_dirty: bool,
    external_implementation_free_list: std::sync::Mutex<Vec<usize>>,
    external_implementation: Arc<RwLock<Vec<Option<Arc<RwLock<ExternalImplementation>>>>>>,
    external_implementation_dirty: bool,
    field_free_list: std::sync::Mutex<Vec<usize>>,
    field: Arc<RwLock<Vec<Option<Arc<RwLock<Field>>>>>>,
    field_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    field_dirty: bool,
    field_access_free_list: std::sync::Mutex<Vec<usize>>,
    field_access: Arc<RwLock<Vec<Option<Arc<RwLock<FieldAccess>>>>>>,
    field_access_dirty: bool,
    field_access_target_free_list: std::sync::Mutex<Vec<usize>>,
    field_access_target: Arc<RwLock<Vec<Option<Arc<RwLock<FieldAccessTarget>>>>>>,
    field_access_target_dirty: bool,
    field_expression_free_list: std::sync::Mutex<Vec<usize>>,
    field_expression: Arc<RwLock<Vec<Option<Arc<RwLock<FieldExpression>>>>>>,
    field_expression_dirty: bool,
    float_literal_free_list: std::sync::Mutex<Vec<usize>>,
    float_literal: Arc<RwLock<Vec<Option<Arc<RwLock<FloatLiteral>>>>>>,
    float_literal_dirty: bool,
    for_loop_free_list: std::sync::Mutex<Vec<usize>>,
    for_loop: Arc<RwLock<Vec<Option<Arc<RwLock<ForLoop>>>>>>,
    for_loop_dirty: bool,
    format_bit_free_list: std::sync::Mutex<Vec<usize>>,
    format_bit: Arc<RwLock<Vec<Option<Arc<RwLock<FormatBit>>>>>>,
    format_bit_dirty: bool,
    format_string_free_list: std::sync::Mutex<Vec<usize>>,
    format_string: Arc<RwLock<Vec<Option<Arc<RwLock<FormatString>>>>>>,
    format_string_dirty: bool,
    func_generic_free_list: std::sync::Mutex<Vec<usize>>,
    func_generic: Arc<RwLock<Vec<Option<Arc<RwLock<FuncGeneric>>>>>>,
    func_generic_dirty: bool,
    function_free_list: std::sync::Mutex<Vec<usize>>,
    function: Arc<RwLock<Vec<Option<Arc<RwLock<Function>>>>>>,
    function_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    function_dirty: bool,
    function_call_free_list: std::sync::Mutex<Vec<usize>>,
    function_call: Arc<RwLock<Vec<Option<Arc<RwLock<FunctionCall>>>>>>,
    function_call_dirty: bool,
    x_future_free_list: std::sync::Mutex<Vec<usize>>,
    x_future: Arc<RwLock<Vec<Option<Arc<RwLock<XFuture>>>>>>,
    x_future_dirty: bool,
    grouped_free_list: std::sync::Mutex<Vec<usize>>,
    grouped: Arc<RwLock<Vec<Option<Arc<RwLock<Grouped>>>>>>,
    grouped_dirty: bool,
    halt_and_catch_fire_free_list: std::sync::Mutex<Vec<usize>>,
    halt_and_catch_fire: Arc<RwLock<Vec<Option<Arc<RwLock<HaltAndCatchFire>>>>>>,
    halt_and_catch_fire_dirty: bool,
    x_if_free_list: std::sync::Mutex<Vec<usize>>,
    x_if: Arc<RwLock<Vec<Option<Arc<RwLock<XIf>>>>>>,
    x_if_dirty: bool,
    implementation_block_free_list: std::sync::Mutex<Vec<usize>>,
    implementation_block: Arc<RwLock<Vec<Option<Arc<RwLock<ImplementationBlock>>>>>>,
    implementation_block_dirty: bool,
    import_free_list: std::sync::Mutex<Vec<usize>>,
    import: Arc<RwLock<Vec<Option<Arc<RwLock<Import>>>>>>,
    import_dirty: bool,
    index_free_list: std::sync::Mutex<Vec<usize>>,
    index: Arc<RwLock<Vec<Option<Arc<RwLock<Index>>>>>>,
    index_dirty: bool,
    integer_literal_free_list: std::sync::Mutex<Vec<usize>>,
    integer_literal: Arc<RwLock<Vec<Option<Arc<RwLock<IntegerLiteral>>>>>>,
    integer_literal_dirty: bool,
    item_free_list: std::sync::Mutex<Vec<usize>>,
    item: Arc<RwLock<Vec<Option<Arc<RwLock<Item>>>>>>,
    item_dirty: bool,
    lambda_free_list: std::sync::Mutex<Vec<usize>>,
    lambda: Arc<RwLock<Vec<Option<Arc<RwLock<Lambda>>>>>>,
    lambda_dirty: bool,
    lambda_parameter_free_list: std::sync::Mutex<Vec<usize>>,
    lambda_parameter: Arc<RwLock<Vec<Option<Arc<RwLock<LambdaParameter>>>>>>,
    lambda_parameter_dirty: bool,
    let_statement_free_list: std::sync::Mutex<Vec<usize>>,
    let_statement: Arc<RwLock<Vec<Option<Arc<RwLock<LetStatement>>>>>>,
    let_statement_dirty: bool,
    list_free_list: std::sync::Mutex<Vec<usize>>,
    list: Arc<RwLock<Vec<Option<Arc<RwLock<List>>>>>>,
    list_dirty: bool,
    list_element_free_list: std::sync::Mutex<Vec<usize>>,
    list_element: Arc<RwLock<Vec<Option<Arc<RwLock<ListElement>>>>>>,
    list_element_dirty: bool,
    list_expression_free_list: std::sync::Mutex<Vec<usize>>,
    list_expression: Arc<RwLock<Vec<Option<Arc<RwLock<ListExpression>>>>>>,
    list_expression_dirty: bool,
    literal_free_list: std::sync::Mutex<Vec<usize>>,
    literal: Arc<RwLock<Vec<Option<Arc<RwLock<Literal>>>>>>,
    literal_dirty: bool,
    local_variable_free_list: std::sync::Mutex<Vec<usize>>,
    local_variable: Arc<RwLock<Vec<Option<Arc<RwLock<LocalVariable>>>>>>,
    local_variable_dirty: bool,
    x_macro_free_list: std::sync::Mutex<Vec<usize>>,
    x_macro: Arc<RwLock<Vec<Option<Arc<RwLock<XMacro>>>>>>,
    x_macro_dirty: bool,
    map_free_list: std::sync::Mutex<Vec<usize>>,
    map: Arc<RwLock<Vec<Option<Arc<RwLock<Map>>>>>>,
    map_dirty: bool,
    map_element_free_list: std::sync::Mutex<Vec<usize>>,
    map_element: Arc<RwLock<Vec<Option<Arc<RwLock<MapElement>>>>>>,
    map_element_dirty: bool,
    map_expression_free_list: std::sync::Mutex<Vec<usize>>,
    map_expression: Arc<RwLock<Vec<Option<Arc<RwLock<MapExpression>>>>>>,
    map_expression_dirty: bool,
    x_match_free_list: std::sync::Mutex<Vec<usize>>,
    x_match: Arc<RwLock<Vec<Option<Arc<RwLock<XMatch>>>>>>,
    x_match_dirty: bool,
    method_call_free_list: std::sync::Mutex<Vec<usize>>,
    method_call: Arc<RwLock<Vec<Option<Arc<RwLock<MethodCall>>>>>>,
    method_call_dirty: bool,
    named_field_expression_free_list: std::sync::Mutex<Vec<usize>>,
    named_field_expression: Arc<RwLock<Vec<Option<Arc<RwLock<NamedFieldExpression>>>>>>,
    named_field_expression_dirty: bool,
    z_object_store_free_list: std::sync::Mutex<Vec<usize>>,
    z_object_store: Arc<RwLock<Vec<Option<Arc<RwLock<ZObjectStore>>>>>>,
    z_object_store_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    z_object_store_dirty: bool,
    object_wrapper_free_list: std::sync::Mutex<Vec<usize>>,
    object_wrapper: Arc<RwLock<Vec<Option<Arc<RwLock<ObjectWrapper>>>>>>,
    object_wrapper_dirty: bool,
    operator_free_list: std::sync::Mutex<Vec<usize>>,
    operator: Arc<RwLock<Vec<Option<Arc<RwLock<Operator>>>>>>,
    operator_dirty: bool,
    parameter_free_list: std::sync::Mutex<Vec<usize>>,
    parameter: Arc<RwLock<Vec<Option<Arc<RwLock<Parameter>>>>>>,
    parameter_dirty: bool,
    x_path_free_list: std::sync::Mutex<Vec<usize>>,
    x_path: Arc<RwLock<Vec<Option<Arc<RwLock<XPath>>>>>>,
    x_path_dirty: bool,
    path_element_free_list: std::sync::Mutex<Vec<usize>>,
    path_element: Arc<RwLock<Vec<Option<Arc<RwLock<PathElement>>>>>>,
    path_element_dirty: bool,
    pattern_free_list: std::sync::Mutex<Vec<usize>>,
    pattern: Arc<RwLock<Vec<Option<Arc<RwLock<Pattern>>>>>>,
    pattern_dirty: bool,
    x_plugin_free_list: std::sync::Mutex<Vec<usize>>,
    x_plugin: Arc<RwLock<Vec<Option<Arc<RwLock<XPlugin>>>>>>,
    x_plugin_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    x_plugin_dirty: bool,
    x_print_free_list: std::sync::Mutex<Vec<usize>>,
    x_print: Arc<RwLock<Vec<Option<Arc<RwLock<XPrint>>>>>>,
    x_print_dirty: bool,
    range_expression_free_list: std::sync::Mutex<Vec<usize>>,
    range_expression: Arc<RwLock<Vec<Option<Arc<RwLock<RangeExpression>>>>>>,
    range_expression_dirty: bool,
    result_statement_free_list: std::sync::Mutex<Vec<usize>>,
    result_statement: Arc<RwLock<Vec<Option<Arc<RwLock<ResultStatement>>>>>>,
    result_statement_dirty: bool,
    x_return_free_list: std::sync::Mutex<Vec<usize>>,
    x_return: Arc<RwLock<Vec<Option<Arc<RwLock<XReturn>>>>>>,
    x_return_dirty: bool,
    span_free_list: std::sync::Mutex<Vec<usize>>,
    span: Arc<RwLock<Vec<Option<Arc<RwLock<Span>>>>>>,
    span_dirty: bool,
    statement_free_list: std::sync::Mutex<Vec<usize>>,
    statement: Arc<RwLock<Vec<Option<Arc<RwLock<Statement>>>>>>,
    statement_dirty: bool,
    static_method_call_free_list: std::sync::Mutex<Vec<usize>>,
    static_method_call: Arc<RwLock<Vec<Option<Arc<RwLock<StaticMethodCall>>>>>>,
    static_method_call_dirty: bool,
    string_bit_free_list: std::sync::Mutex<Vec<usize>>,
    string_bit: Arc<RwLock<Vec<Option<Arc<RwLock<StringBit>>>>>>,
    string_bit_dirty: bool,
    string_literal_free_list: std::sync::Mutex<Vec<usize>>,
    string_literal: Arc<RwLock<Vec<Option<Arc<RwLock<StringLiteral>>>>>>,
    string_literal_dirty: bool,
    woog_struct_free_list: std::sync::Mutex<Vec<usize>>,
    woog_struct: Arc<RwLock<Vec<Option<Arc<RwLock<WoogStruct>>>>>>,
    woog_struct_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    woog_struct_dirty: bool,
    struct_expression_free_list: std::sync::Mutex<Vec<usize>>,
    struct_expression: Arc<RwLock<Vec<Option<Arc<RwLock<StructExpression>>>>>>,
    struct_expression_dirty: bool,
    struct_field_free_list: std::sync::Mutex<Vec<usize>>,
    struct_field: Arc<RwLock<Vec<Option<Arc<RwLock<StructField>>>>>>,
    struct_field_dirty: bool,
    struct_generic_free_list: std::sync::Mutex<Vec<usize>>,
    struct_generic: Arc<RwLock<Vec<Option<Arc<RwLock<StructGeneric>>>>>>,
    struct_generic_dirty: bool,
    tuple_field_free_list: std::sync::Mutex<Vec<usize>>,
    tuple_field: Arc<RwLock<Vec<Option<Arc<RwLock<TupleField>>>>>>,
    tuple_field_dirty: bool,
    type_cast_free_list: std::sync::Mutex<Vec<usize>>,
    type_cast: Arc<RwLock<Vec<Option<Arc<RwLock<TypeCast>>>>>>,
    type_cast_dirty: bool,
    unary_free_list: std::sync::Mutex<Vec<usize>>,
    unary: Arc<RwLock<Vec<Option<Arc<RwLock<Unary>>>>>>,
    unary_dirty: bool,
    unit_free_list: std::sync::Mutex<Vec<usize>>,
    unit: Arc<RwLock<Vec<Option<Arc<RwLock<Unit>>>>>>,
    unit_dirty: bool,
    unnamed_field_expression_free_list: std::sync::Mutex<Vec<usize>>,
    unnamed_field_expression: Arc<RwLock<Vec<Option<Arc<RwLock<UnnamedFieldExpression>>>>>>,
    unnamed_field_expression_dirty: bool,
    x_value_free_list: std::sync::Mutex<Vec<usize>>,
    x_value: Arc<RwLock<Vec<Option<Arc<RwLock<XValue>>>>>>,
    x_value_dirty: bool,
    value_type_free_list: std::sync::Mutex<Vec<usize>>,
    value_type: Arc<RwLock<Vec<Option<Arc<RwLock<ValueType>>>>>>,
    value_type_dirty: bool,
    variable_free_list: std::sync::Mutex<Vec<usize>>,
    variable: Arc<RwLock<Vec<Option<Arc<RwLock<Variable>>>>>>,
    variable_dirty: bool,
    variable_expression_free_list: std::sync::Mutex<Vec<usize>>,
    variable_expression: Arc<RwLock<Vec<Option<Arc<RwLock<VariableExpression>>>>>>,
    variable_expression_dirty: bool,
}

impl Clone for ObjectStore {
    fn clone(&self) -> Self {
        ObjectStore {
            argument_free_list: Mutex::new(self.argument_free_list.lock().unwrap().clone()),
            argument: Arc::new(RwLock::new(self.argument.read().unwrap().clone())),
            argument_dirty: false,
            a_wait_free_list: Mutex::new(self.a_wait_free_list.lock().unwrap().clone()),
            a_wait: Arc::new(RwLock::new(self.a_wait.read().unwrap().clone())),
            a_wait_dirty: false,
            binary_free_list: Mutex::new(self.binary_free_list.lock().unwrap().clone()),
            binary: Arc::new(RwLock::new(self.binary.read().unwrap().clone())),
            binary_dirty: false,
            block_free_list: Mutex::new(self.block_free_list.lock().unwrap().clone()),
            block: Arc::new(RwLock::new(self.block.read().unwrap().clone())),
            block_dirty: false,
            body_free_list: Mutex::new(self.body_free_list.lock().unwrap().clone()),
            body: Arc::new(RwLock::new(self.body.read().unwrap().clone())),
            body_dirty: false,
            boolean_literal_free_list: Mutex::new(
                self.boolean_literal_free_list.lock().unwrap().clone(),
            ),
            boolean_literal: Arc::new(RwLock::new(self.boolean_literal.read().unwrap().clone())),
            boolean_literal_dirty: false,
            boolean_operator_free_list: Mutex::new(
                self.boolean_operator_free_list.lock().unwrap().clone(),
            ),
            boolean_operator: Arc::new(RwLock::new(self.boolean_operator.read().unwrap().clone())),
            boolean_operator_dirty: false,
            call_free_list: Mutex::new(self.call_free_list.lock().unwrap().clone()),
            call: Arc::new(RwLock::new(self.call.read().unwrap().clone())),
            call_dirty: false,
            char_literal_free_list: Mutex::new(self.char_literal_free_list.lock().unwrap().clone()),
            char_literal: Arc::new(RwLock::new(self.char_literal.read().unwrap().clone())),
            char_literal_dirty: false,
            comparison_free_list: Mutex::new(self.comparison_free_list.lock().unwrap().clone()),
            comparison: Arc::new(RwLock::new(self.comparison.read().unwrap().clone())),
            comparison_dirty: false,
            data_structure_free_list: Mutex::new(
                self.data_structure_free_list.lock().unwrap().clone(),
            ),
            data_structure: Arc::new(RwLock::new(self.data_structure.read().unwrap().clone())),
            data_structure_dirty: false,
            dwarf_source_file_free_list: Mutex::new(
                self.dwarf_source_file_free_list.lock().unwrap().clone(),
            ),
            dwarf_source_file: Arc::new(RwLock::new(
                self.dwarf_source_file.read().unwrap().clone(),
            )),
            dwarf_source_file_dirty: false,
            enum_field_free_list: Mutex::new(self.enum_field_free_list.lock().unwrap().clone()),
            enum_field: Arc::new(RwLock::new(self.enum_field.read().unwrap().clone())),
            enum_field_dirty: false,
            enum_generic_free_list: Mutex::new(self.enum_generic_free_list.lock().unwrap().clone()),
            enum_generic: Arc::new(RwLock::new(self.enum_generic.read().unwrap().clone())),
            enum_generic_dirty: false,
            enum_generic_type_free_list: Mutex::new(
                self.enum_generic_type_free_list.lock().unwrap().clone(),
            ),
            enum_generic_type: Arc::new(RwLock::new(
                self.enum_generic_type.read().unwrap().clone(),
            )),
            enum_generic_type_dirty: false,
            enumeration_free_list: Mutex::new(self.enumeration_free_list.lock().unwrap().clone()),
            enumeration: Arc::new(RwLock::new(self.enumeration.read().unwrap().clone())),
            enumeration_id_by_name: self.enumeration_id_by_name.clone(),
            enumeration_dirty: false,
            expression_free_list: Mutex::new(self.expression_free_list.lock().unwrap().clone()),
            expression: Arc::new(RwLock::new(self.expression.read().unwrap().clone())),
            expression_dirty: false,
            expression_bit_free_list: Mutex::new(
                self.expression_bit_free_list.lock().unwrap().clone(),
            ),
            expression_bit: Arc::new(RwLock::new(self.expression_bit.read().unwrap().clone())),
            expression_bit_dirty: false,
            expression_statement_free_list: Mutex::new(
                self.expression_statement_free_list.lock().unwrap().clone(),
            ),
            expression_statement: Arc::new(RwLock::new(
                self.expression_statement.read().unwrap().clone(),
            )),
            expression_statement_dirty: false,
            external_implementation_free_list: Mutex::new(
                self.external_implementation_free_list
                    .lock()
                    .unwrap()
                    .clone(),
            ),
            external_implementation: Arc::new(RwLock::new(
                self.external_implementation.read().unwrap().clone(),
            )),
            external_implementation_dirty: false,
            field_free_list: Mutex::new(self.field_free_list.lock().unwrap().clone()),
            field: Arc::new(RwLock::new(self.field.read().unwrap().clone())),
            field_id_by_name: self.field_id_by_name.clone(),
            field_dirty: false,
            field_access_free_list: Mutex::new(self.field_access_free_list.lock().unwrap().clone()),
            field_access: Arc::new(RwLock::new(self.field_access.read().unwrap().clone())),
            field_access_dirty: false,
            field_access_target_free_list: Mutex::new(
                self.field_access_target_free_list.lock().unwrap().clone(),
            ),
            field_access_target: Arc::new(RwLock::new(
                self.field_access_target.read().unwrap().clone(),
            )),
            field_access_target_dirty: false,
            field_expression_free_list: Mutex::new(
                self.field_expression_free_list.lock().unwrap().clone(),
            ),
            field_expression: Arc::new(RwLock::new(self.field_expression.read().unwrap().clone())),
            field_expression_dirty: false,
            float_literal_free_list: Mutex::new(
                self.float_literal_free_list.lock().unwrap().clone(),
            ),
            float_literal: Arc::new(RwLock::new(self.float_literal.read().unwrap().clone())),
            float_literal_dirty: false,
            for_loop_free_list: Mutex::new(self.for_loop_free_list.lock().unwrap().clone()),
            for_loop: Arc::new(RwLock::new(self.for_loop.read().unwrap().clone())),
            for_loop_dirty: false,
            format_bit_free_list: Mutex::new(self.format_bit_free_list.lock().unwrap().clone()),
            format_bit: Arc::new(RwLock::new(self.format_bit.read().unwrap().clone())),
            format_bit_dirty: false,
            format_string_free_list: Mutex::new(
                self.format_string_free_list.lock().unwrap().clone(),
            ),
            format_string: Arc::new(RwLock::new(self.format_string.read().unwrap().clone())),
            format_string_dirty: false,
            func_generic_free_list: Mutex::new(self.func_generic_free_list.lock().unwrap().clone()),
            func_generic: Arc::new(RwLock::new(self.func_generic.read().unwrap().clone())),
            func_generic_dirty: false,
            function_free_list: Mutex::new(self.function_free_list.lock().unwrap().clone()),
            function: Arc::new(RwLock::new(self.function.read().unwrap().clone())),
            function_id_by_name: self.function_id_by_name.clone(),
            function_dirty: false,
            function_call_free_list: Mutex::new(
                self.function_call_free_list.lock().unwrap().clone(),
            ),
            function_call: Arc::new(RwLock::new(self.function_call.read().unwrap().clone())),
            function_call_dirty: false,
            x_future_free_list: Mutex::new(self.x_future_free_list.lock().unwrap().clone()),
            x_future: Arc::new(RwLock::new(self.x_future.read().unwrap().clone())),
            x_future_dirty: false,
            grouped_free_list: Mutex::new(self.grouped_free_list.lock().unwrap().clone()),
            grouped: Arc::new(RwLock::new(self.grouped.read().unwrap().clone())),
            grouped_dirty: false,
            halt_and_catch_fire_free_list: Mutex::new(
                self.halt_and_catch_fire_free_list.lock().unwrap().clone(),
            ),
            halt_and_catch_fire: Arc::new(RwLock::new(
                self.halt_and_catch_fire.read().unwrap().clone(),
            )),
            halt_and_catch_fire_dirty: false,
            x_if_free_list: Mutex::new(self.x_if_free_list.lock().unwrap().clone()),
            x_if: Arc::new(RwLock::new(self.x_if.read().unwrap().clone())),
            x_if_dirty: false,
            implementation_block_free_list: Mutex::new(
                self.implementation_block_free_list.lock().unwrap().clone(),
            ),
            implementation_block: Arc::new(RwLock::new(
                self.implementation_block.read().unwrap().clone(),
            )),
            implementation_block_dirty: false,
            import_free_list: Mutex::new(self.import_free_list.lock().unwrap().clone()),
            import: Arc::new(RwLock::new(self.import.read().unwrap().clone())),
            import_dirty: false,
            index_free_list: Mutex::new(self.index_free_list.lock().unwrap().clone()),
            index: Arc::new(RwLock::new(self.index.read().unwrap().clone())),
            index_dirty: false,
            integer_literal_free_list: Mutex::new(
                self.integer_literal_free_list.lock().unwrap().clone(),
            ),
            integer_literal: Arc::new(RwLock::new(self.integer_literal.read().unwrap().clone())),
            integer_literal_dirty: false,
            item_free_list: Mutex::new(self.item_free_list.lock().unwrap().clone()),
            item: Arc::new(RwLock::new(self.item.read().unwrap().clone())),
            item_dirty: false,
            lambda_free_list: Mutex::new(self.lambda_free_list.lock().unwrap().clone()),
            lambda: Arc::new(RwLock::new(self.lambda.read().unwrap().clone())),
            lambda_dirty: false,
            lambda_parameter_free_list: Mutex::new(
                self.lambda_parameter_free_list.lock().unwrap().clone(),
            ),
            lambda_parameter: Arc::new(RwLock::new(self.lambda_parameter.read().unwrap().clone())),
            lambda_parameter_dirty: false,
            let_statement_free_list: Mutex::new(
                self.let_statement_free_list.lock().unwrap().clone(),
            ),
            let_statement: Arc::new(RwLock::new(self.let_statement.read().unwrap().clone())),
            let_statement_dirty: false,
            list_free_list: Mutex::new(self.list_free_list.lock().unwrap().clone()),
            list: Arc::new(RwLock::new(self.list.read().unwrap().clone())),
            list_dirty: false,
            list_element_free_list: Mutex::new(self.list_element_free_list.lock().unwrap().clone()),
            list_element: Arc::new(RwLock::new(self.list_element.read().unwrap().clone())),
            list_element_dirty: false,
            list_expression_free_list: Mutex::new(
                self.list_expression_free_list.lock().unwrap().clone(),
            ),
            list_expression: Arc::new(RwLock::new(self.list_expression.read().unwrap().clone())),
            list_expression_dirty: false,
            literal_free_list: Mutex::new(self.literal_free_list.lock().unwrap().clone()),
            literal: Arc::new(RwLock::new(self.literal.read().unwrap().clone())),
            literal_dirty: false,
            local_variable_free_list: Mutex::new(
                self.local_variable_free_list.lock().unwrap().clone(),
            ),
            local_variable: Arc::new(RwLock::new(self.local_variable.read().unwrap().clone())),
            local_variable_dirty: false,
            x_macro_free_list: Mutex::new(self.x_macro_free_list.lock().unwrap().clone()),
            x_macro: Arc::new(RwLock::new(self.x_macro.read().unwrap().clone())),
            x_macro_dirty: false,
            map_free_list: Mutex::new(self.map_free_list.lock().unwrap().clone()),
            map: Arc::new(RwLock::new(self.map.read().unwrap().clone())),
            map_dirty: false,
            map_element_free_list: Mutex::new(self.map_element_free_list.lock().unwrap().clone()),
            map_element: Arc::new(RwLock::new(self.map_element.read().unwrap().clone())),
            map_element_dirty: false,
            map_expression_free_list: Mutex::new(
                self.map_expression_free_list.lock().unwrap().clone(),
            ),
            map_expression: Arc::new(RwLock::new(self.map_expression.read().unwrap().clone())),
            map_expression_dirty: false,
            x_match_free_list: Mutex::new(self.x_match_free_list.lock().unwrap().clone()),
            x_match: Arc::new(RwLock::new(self.x_match.read().unwrap().clone())),
            x_match_dirty: false,
            method_call_free_list: Mutex::new(self.method_call_free_list.lock().unwrap().clone()),
            method_call: Arc::new(RwLock::new(self.method_call.read().unwrap().clone())),
            method_call_dirty: false,
            named_field_expression_free_list: Mutex::new(
                self.named_field_expression_free_list
                    .lock()
                    .unwrap()
                    .clone(),
            ),
            named_field_expression: Arc::new(RwLock::new(
                self.named_field_expression.read().unwrap().clone(),
            )),
            named_field_expression_dirty: false,
            z_object_store_free_list: Mutex::new(
                self.z_object_store_free_list.lock().unwrap().clone(),
            ),
            z_object_store: Arc::new(RwLock::new(self.z_object_store.read().unwrap().clone())),
            z_object_store_id_by_name: self.z_object_store_id_by_name.clone(),
            z_object_store_dirty: false,
            object_wrapper_free_list: Mutex::new(
                self.object_wrapper_free_list.lock().unwrap().clone(),
            ),
            object_wrapper: Arc::new(RwLock::new(self.object_wrapper.read().unwrap().clone())),
            object_wrapper_dirty: false,
            operator_free_list: Mutex::new(self.operator_free_list.lock().unwrap().clone()),
            operator: Arc::new(RwLock::new(self.operator.read().unwrap().clone())),
            operator_dirty: false,
            parameter_free_list: Mutex::new(self.parameter_free_list.lock().unwrap().clone()),
            parameter: Arc::new(RwLock::new(self.parameter.read().unwrap().clone())),
            parameter_dirty: false,
            x_path_free_list: Mutex::new(self.x_path_free_list.lock().unwrap().clone()),
            x_path: Arc::new(RwLock::new(self.x_path.read().unwrap().clone())),
            x_path_dirty: false,
            path_element_free_list: Mutex::new(self.path_element_free_list.lock().unwrap().clone()),
            path_element: Arc::new(RwLock::new(self.path_element.read().unwrap().clone())),
            path_element_dirty: false,
            pattern_free_list: Mutex::new(self.pattern_free_list.lock().unwrap().clone()),
            pattern: Arc::new(RwLock::new(self.pattern.read().unwrap().clone())),
            pattern_dirty: false,
            x_plugin_free_list: Mutex::new(self.x_plugin_free_list.lock().unwrap().clone()),
            x_plugin: Arc::new(RwLock::new(self.x_plugin.read().unwrap().clone())),
            x_plugin_id_by_name: self.x_plugin_id_by_name.clone(),
            x_plugin_dirty: false,
            x_print_free_list: Mutex::new(self.x_print_free_list.lock().unwrap().clone()),
            x_print: Arc::new(RwLock::new(self.x_print.read().unwrap().clone())),
            x_print_dirty: false,
            range_expression_free_list: Mutex::new(
                self.range_expression_free_list.lock().unwrap().clone(),
            ),
            range_expression: Arc::new(RwLock::new(self.range_expression.read().unwrap().clone())),
            range_expression_dirty: false,
            result_statement_free_list: Mutex::new(
                self.result_statement_free_list.lock().unwrap().clone(),
            ),
            result_statement: Arc::new(RwLock::new(self.result_statement.read().unwrap().clone())),
            result_statement_dirty: false,
            x_return_free_list: Mutex::new(self.x_return_free_list.lock().unwrap().clone()),
            x_return: Arc::new(RwLock::new(self.x_return.read().unwrap().clone())),
            x_return_dirty: false,
            span_free_list: Mutex::new(self.span_free_list.lock().unwrap().clone()),
            span: Arc::new(RwLock::new(self.span.read().unwrap().clone())),
            span_dirty: false,
            statement_free_list: Mutex::new(self.statement_free_list.lock().unwrap().clone()),
            statement: Arc::new(RwLock::new(self.statement.read().unwrap().clone())),
            statement_dirty: false,
            static_method_call_free_list: Mutex::new(
                self.static_method_call_free_list.lock().unwrap().clone(),
            ),
            static_method_call: Arc::new(RwLock::new(
                self.static_method_call.read().unwrap().clone(),
            )),
            static_method_call_dirty: false,
            string_bit_free_list: Mutex::new(self.string_bit_free_list.lock().unwrap().clone()),
            string_bit: Arc::new(RwLock::new(self.string_bit.read().unwrap().clone())),
            string_bit_dirty: false,
            string_literal_free_list: Mutex::new(
                self.string_literal_free_list.lock().unwrap().clone(),
            ),
            string_literal: Arc::new(RwLock::new(self.string_literal.read().unwrap().clone())),
            string_literal_dirty: false,
            woog_struct_free_list: Mutex::new(self.woog_struct_free_list.lock().unwrap().clone()),
            woog_struct: Arc::new(RwLock::new(self.woog_struct.read().unwrap().clone())),
            woog_struct_id_by_name: self.woog_struct_id_by_name.clone(),
            woog_struct_dirty: false,
            struct_expression_free_list: Mutex::new(
                self.struct_expression_free_list.lock().unwrap().clone(),
            ),
            struct_expression: Arc::new(RwLock::new(
                self.struct_expression.read().unwrap().clone(),
            )),
            struct_expression_dirty: false,
            struct_field_free_list: Mutex::new(self.struct_field_free_list.lock().unwrap().clone()),
            struct_field: Arc::new(RwLock::new(self.struct_field.read().unwrap().clone())),
            struct_field_dirty: false,
            struct_generic_free_list: Mutex::new(
                self.struct_generic_free_list.lock().unwrap().clone(),
            ),
            struct_generic: Arc::new(RwLock::new(self.struct_generic.read().unwrap().clone())),
            struct_generic_dirty: false,
            tuple_field_free_list: Mutex::new(self.tuple_field_free_list.lock().unwrap().clone()),
            tuple_field: Arc::new(RwLock::new(self.tuple_field.read().unwrap().clone())),
            tuple_field_dirty: false,
            type_cast_free_list: Mutex::new(self.type_cast_free_list.lock().unwrap().clone()),
            type_cast: Arc::new(RwLock::new(self.type_cast.read().unwrap().clone())),
            type_cast_dirty: false,
            unary_free_list: Mutex::new(self.unary_free_list.lock().unwrap().clone()),
            unary: Arc::new(RwLock::new(self.unary.read().unwrap().clone())),
            unary_dirty: false,
            unit_free_list: Mutex::new(self.unit_free_list.lock().unwrap().clone()),
            unit: Arc::new(RwLock::new(self.unit.read().unwrap().clone())),
            unit_dirty: false,
            unnamed_field_expression_free_list: Mutex::new(
                self.unnamed_field_expression_free_list
                    .lock()
                    .unwrap()
                    .clone(),
            ),
            unnamed_field_expression: Arc::new(RwLock::new(
                self.unnamed_field_expression.read().unwrap().clone(),
            )),
            unnamed_field_expression_dirty: false,
            x_value_free_list: Mutex::new(self.x_value_free_list.lock().unwrap().clone()),
            x_value: Arc::new(RwLock::new(self.x_value.read().unwrap().clone())),
            x_value_dirty: false,
            value_type_free_list: Mutex::new(self.value_type_free_list.lock().unwrap().clone()),
            value_type: Arc::new(RwLock::new(self.value_type.read().unwrap().clone())),
            value_type_dirty: false,
            variable_free_list: Mutex::new(self.variable_free_list.lock().unwrap().clone()),
            variable: Arc::new(RwLock::new(self.variable.read().unwrap().clone())),
            variable_dirty: false,
            variable_expression_free_list: Mutex::new(
                self.variable_expression_free_list.lock().unwrap().clone(),
            ),
            variable_expression: Arc::new(RwLock::new(
                self.variable_expression.read().unwrap().clone(),
            )),
            variable_expression_dirty: false,
        }
    }
}
impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument_free_list: std::sync::Mutex::new(Vec::new()),
            argument: Arc::new(RwLock::new(Vec::new())),
            argument_dirty: false,
            a_wait_free_list: std::sync::Mutex::new(Vec::new()),
            a_wait: Arc::new(RwLock::new(Vec::new())),
            a_wait_dirty: false,
            binary_free_list: std::sync::Mutex::new(Vec::new()),
            binary: Arc::new(RwLock::new(Vec::new())),
            binary_dirty: false,
            block_free_list: std::sync::Mutex::new(Vec::new()),
            block: Arc::new(RwLock::new(Vec::new())),
            block_dirty: false,
            body_free_list: std::sync::Mutex::new(Vec::new()),
            body: Arc::new(RwLock::new(Vec::new())),
            body_dirty: false,
            boolean_literal_free_list: std::sync::Mutex::new(Vec::new()),
            boolean_literal: Arc::new(RwLock::new(Vec::new())),
            boolean_literal_dirty: false,
            boolean_operator_free_list: std::sync::Mutex::new(Vec::new()),
            boolean_operator: Arc::new(RwLock::new(Vec::new())),
            boolean_operator_dirty: false,
            call_free_list: std::sync::Mutex::new(Vec::new()),
            call: Arc::new(RwLock::new(Vec::new())),
            call_dirty: false,
            char_literal_free_list: std::sync::Mutex::new(Vec::new()),
            char_literal: Arc::new(RwLock::new(Vec::new())),
            char_literal_dirty: false,
            comparison_free_list: std::sync::Mutex::new(Vec::new()),
            comparison: Arc::new(RwLock::new(Vec::new())),
            comparison_dirty: false,
            data_structure_free_list: std::sync::Mutex::new(Vec::new()),
            data_structure: Arc::new(RwLock::new(Vec::new())),
            data_structure_dirty: false,
            dwarf_source_file_free_list: std::sync::Mutex::new(Vec::new()),
            dwarf_source_file: Arc::new(RwLock::new(Vec::new())),
            dwarf_source_file_dirty: false,
            enum_field_free_list: std::sync::Mutex::new(Vec::new()),
            enum_field: Arc::new(RwLock::new(Vec::new())),
            enum_field_dirty: false,
            enum_generic_free_list: std::sync::Mutex::new(Vec::new()),
            enum_generic: Arc::new(RwLock::new(Vec::new())),
            enum_generic_dirty: false,
            enum_generic_type_free_list: std::sync::Mutex::new(Vec::new()),
            enum_generic_type: Arc::new(RwLock::new(Vec::new())),
            enum_generic_type_dirty: false,
            enumeration_free_list: std::sync::Mutex::new(Vec::new()),
            enumeration: Arc::new(RwLock::new(Vec::new())),
            enumeration_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            enumeration_dirty: false,
            expression_free_list: std::sync::Mutex::new(Vec::new()),
            expression: Arc::new(RwLock::new(Vec::new())),
            expression_dirty: false,
            expression_bit_free_list: std::sync::Mutex::new(Vec::new()),
            expression_bit: Arc::new(RwLock::new(Vec::new())),
            expression_bit_dirty: false,
            expression_statement_free_list: std::sync::Mutex::new(Vec::new()),
            expression_statement: Arc::new(RwLock::new(Vec::new())),
            expression_statement_dirty: false,
            external_implementation_free_list: std::sync::Mutex::new(Vec::new()),
            external_implementation: Arc::new(RwLock::new(Vec::new())),
            external_implementation_dirty: false,
            field_free_list: std::sync::Mutex::new(Vec::new()),
            field: Arc::new(RwLock::new(Vec::new())),
            field_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            field_dirty: false,
            field_access_free_list: std::sync::Mutex::new(Vec::new()),
            field_access: Arc::new(RwLock::new(Vec::new())),
            field_access_dirty: false,
            field_access_target_free_list: std::sync::Mutex::new(Vec::new()),
            field_access_target: Arc::new(RwLock::new(Vec::new())),
            field_access_target_dirty: false,
            field_expression_free_list: std::sync::Mutex::new(Vec::new()),
            field_expression: Arc::new(RwLock::new(Vec::new())),
            field_expression_dirty: false,
            float_literal_free_list: std::sync::Mutex::new(Vec::new()),
            float_literal: Arc::new(RwLock::new(Vec::new())),
            float_literal_dirty: false,
            for_loop_free_list: std::sync::Mutex::new(Vec::new()),
            for_loop: Arc::new(RwLock::new(Vec::new())),
            for_loop_dirty: false,
            format_bit_free_list: std::sync::Mutex::new(Vec::new()),
            format_bit: Arc::new(RwLock::new(Vec::new())),
            format_bit_dirty: false,
            format_string_free_list: std::sync::Mutex::new(Vec::new()),
            format_string: Arc::new(RwLock::new(Vec::new())),
            format_string_dirty: false,
            func_generic_free_list: std::sync::Mutex::new(Vec::new()),
            func_generic: Arc::new(RwLock::new(Vec::new())),
            func_generic_dirty: false,
            function_free_list: std::sync::Mutex::new(Vec::new()),
            function: Arc::new(RwLock::new(Vec::new())),
            function_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            function_dirty: false,
            function_call_free_list: std::sync::Mutex::new(Vec::new()),
            function_call: Arc::new(RwLock::new(Vec::new())),
            function_call_dirty: false,
            x_future_free_list: std::sync::Mutex::new(Vec::new()),
            x_future: Arc::new(RwLock::new(Vec::new())),
            x_future_dirty: false,
            grouped_free_list: std::sync::Mutex::new(Vec::new()),
            grouped: Arc::new(RwLock::new(Vec::new())),
            grouped_dirty: false,
            halt_and_catch_fire_free_list: std::sync::Mutex::new(Vec::new()),
            halt_and_catch_fire: Arc::new(RwLock::new(Vec::new())),
            halt_and_catch_fire_dirty: false,
            x_if_free_list: std::sync::Mutex::new(Vec::new()),
            x_if: Arc::new(RwLock::new(Vec::new())),
            x_if_dirty: false,
            implementation_block_free_list: std::sync::Mutex::new(Vec::new()),
            implementation_block: Arc::new(RwLock::new(Vec::new())),
            implementation_block_dirty: false,
            import_free_list: std::sync::Mutex::new(Vec::new()),
            import: Arc::new(RwLock::new(Vec::new())),
            import_dirty: false,
            index_free_list: std::sync::Mutex::new(Vec::new()),
            index: Arc::new(RwLock::new(Vec::new())),
            index_dirty: false,
            integer_literal_free_list: std::sync::Mutex::new(Vec::new()),
            integer_literal: Arc::new(RwLock::new(Vec::new())),
            integer_literal_dirty: false,
            item_free_list: std::sync::Mutex::new(Vec::new()),
            item: Arc::new(RwLock::new(Vec::new())),
            item_dirty: false,
            lambda_free_list: std::sync::Mutex::new(Vec::new()),
            lambda: Arc::new(RwLock::new(Vec::new())),
            lambda_dirty: false,
            lambda_parameter_free_list: std::sync::Mutex::new(Vec::new()),
            lambda_parameter: Arc::new(RwLock::new(Vec::new())),
            lambda_parameter_dirty: false,
            let_statement_free_list: std::sync::Mutex::new(Vec::new()),
            let_statement: Arc::new(RwLock::new(Vec::new())),
            let_statement_dirty: false,
            list_free_list: std::sync::Mutex::new(Vec::new()),
            list: Arc::new(RwLock::new(Vec::new())),
            list_dirty: false,
            list_element_free_list: std::sync::Mutex::new(Vec::new()),
            list_element: Arc::new(RwLock::new(Vec::new())),
            list_element_dirty: false,
            list_expression_free_list: std::sync::Mutex::new(Vec::new()),
            list_expression: Arc::new(RwLock::new(Vec::new())),
            list_expression_dirty: false,
            literal_free_list: std::sync::Mutex::new(Vec::new()),
            literal: Arc::new(RwLock::new(Vec::new())),
            literal_dirty: false,
            local_variable_free_list: std::sync::Mutex::new(Vec::new()),
            local_variable: Arc::new(RwLock::new(Vec::new())),
            local_variable_dirty: false,
            x_macro_free_list: std::sync::Mutex::new(Vec::new()),
            x_macro: Arc::new(RwLock::new(Vec::new())),
            x_macro_dirty: false,
            map_free_list: std::sync::Mutex::new(Vec::new()),
            map: Arc::new(RwLock::new(Vec::new())),
            map_dirty: false,
            map_element_free_list: std::sync::Mutex::new(Vec::new()),
            map_element: Arc::new(RwLock::new(Vec::new())),
            map_element_dirty: false,
            map_expression_free_list: std::sync::Mutex::new(Vec::new()),
            map_expression: Arc::new(RwLock::new(Vec::new())),
            map_expression_dirty: false,
            x_match_free_list: std::sync::Mutex::new(Vec::new()),
            x_match: Arc::new(RwLock::new(Vec::new())),
            x_match_dirty: false,
            method_call_free_list: std::sync::Mutex::new(Vec::new()),
            method_call: Arc::new(RwLock::new(Vec::new())),
            method_call_dirty: false,
            named_field_expression_free_list: std::sync::Mutex::new(Vec::new()),
            named_field_expression: Arc::new(RwLock::new(Vec::new())),
            named_field_expression_dirty: false,
            z_object_store_free_list: std::sync::Mutex::new(Vec::new()),
            z_object_store: Arc::new(RwLock::new(Vec::new())),
            z_object_store_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            z_object_store_dirty: false,
            object_wrapper_free_list: std::sync::Mutex::new(Vec::new()),
            object_wrapper: Arc::new(RwLock::new(Vec::new())),
            object_wrapper_dirty: false,
            operator_free_list: std::sync::Mutex::new(Vec::new()),
            operator: Arc::new(RwLock::new(Vec::new())),
            operator_dirty: false,
            parameter_free_list: std::sync::Mutex::new(Vec::new()),
            parameter: Arc::new(RwLock::new(Vec::new())),
            parameter_dirty: false,
            x_path_free_list: std::sync::Mutex::new(Vec::new()),
            x_path: Arc::new(RwLock::new(Vec::new())),
            x_path_dirty: false,
            path_element_free_list: std::sync::Mutex::new(Vec::new()),
            path_element: Arc::new(RwLock::new(Vec::new())),
            path_element_dirty: false,
            pattern_free_list: std::sync::Mutex::new(Vec::new()),
            pattern: Arc::new(RwLock::new(Vec::new())),
            pattern_dirty: false,
            x_plugin_free_list: std::sync::Mutex::new(Vec::new()),
            x_plugin: Arc::new(RwLock::new(Vec::new())),
            x_plugin_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            x_plugin_dirty: false,
            x_print_free_list: std::sync::Mutex::new(Vec::new()),
            x_print: Arc::new(RwLock::new(Vec::new())),
            x_print_dirty: false,
            range_expression_free_list: std::sync::Mutex::new(Vec::new()),
            range_expression: Arc::new(RwLock::new(Vec::new())),
            range_expression_dirty: false,
            result_statement_free_list: std::sync::Mutex::new(Vec::new()),
            result_statement: Arc::new(RwLock::new(Vec::new())),
            result_statement_dirty: false,
            x_return_free_list: std::sync::Mutex::new(Vec::new()),
            x_return: Arc::new(RwLock::new(Vec::new())),
            x_return_dirty: false,
            span_free_list: std::sync::Mutex::new(Vec::new()),
            span: Arc::new(RwLock::new(Vec::new())),
            span_dirty: false,
            statement_free_list: std::sync::Mutex::new(Vec::new()),
            statement: Arc::new(RwLock::new(Vec::new())),
            statement_dirty: false,
            static_method_call_free_list: std::sync::Mutex::new(Vec::new()),
            static_method_call: Arc::new(RwLock::new(Vec::new())),
            static_method_call_dirty: false,
            string_bit_free_list: std::sync::Mutex::new(Vec::new()),
            string_bit: Arc::new(RwLock::new(Vec::new())),
            string_bit_dirty: false,
            string_literal_free_list: std::sync::Mutex::new(Vec::new()),
            string_literal: Arc::new(RwLock::new(Vec::new())),
            string_literal_dirty: false,
            woog_struct_free_list: std::sync::Mutex::new(Vec::new()),
            woog_struct: Arc::new(RwLock::new(Vec::new())),
            woog_struct_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            woog_struct_dirty: false,
            struct_expression_free_list: std::sync::Mutex::new(Vec::new()),
            struct_expression: Arc::new(RwLock::new(Vec::new())),
            struct_expression_dirty: false,
            struct_field_free_list: std::sync::Mutex::new(Vec::new()),
            struct_field: Arc::new(RwLock::new(Vec::new())),
            struct_field_dirty: false,
            struct_generic_free_list: std::sync::Mutex::new(Vec::new()),
            struct_generic: Arc::new(RwLock::new(Vec::new())),
            struct_generic_dirty: false,
            tuple_field_free_list: std::sync::Mutex::new(Vec::new()),
            tuple_field: Arc::new(RwLock::new(Vec::new())),
            tuple_field_dirty: false,
            type_cast_free_list: std::sync::Mutex::new(Vec::new()),
            type_cast: Arc::new(RwLock::new(Vec::new())),
            type_cast_dirty: false,
            unary_free_list: std::sync::Mutex::new(Vec::new()),
            unary: Arc::new(RwLock::new(Vec::new())),
            unary_dirty: false,
            unit_free_list: std::sync::Mutex::new(Vec::new()),
            unit: Arc::new(RwLock::new(Vec::new())),
            unit_dirty: false,
            unnamed_field_expression_free_list: std::sync::Mutex::new(Vec::new()),
            unnamed_field_expression: Arc::new(RwLock::new(Vec::new())),
            unnamed_field_expression_dirty: false,
            x_value_free_list: std::sync::Mutex::new(Vec::new()),
            x_value: Arc::new(RwLock::new(Vec::new())),
            x_value_dirty: false,
            value_type_free_list: std::sync::Mutex::new(Vec::new()),
            value_type: Arc::new(RwLock::new(Vec::new())),
            value_type_dirty: false,
            variable_free_list: std::sync::Mutex::new(Vec::new()),
            variable: Arc::new(RwLock::new(Vec::new())),
            variable_dirty: false,
            variable_expression_free_list: std::sync::Mutex::new(Vec::new()),
            variable_expression: Arc::new(RwLock::new(Vec::new())),
            variable_expression_dirty: false,
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    pub fn merge(&mut self, other: &ObjectStore) {
        if other.argument_dirty {
            other.argument.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in argument, if it's not there add it to argument.
                    if self
                        .argument
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.argument.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_argument(|id| -> Arc<RwLock<Argument>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.a_wait_dirty {
            other.a_wait.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in a_wait, if it's not there add it to a_wait.
                    if self
                        .a_wait
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.a_wait.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_a_wait(|id| -> Arc<RwLock<AWait>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.binary_dirty {
            other.binary.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in binary, if it's not there add it to binary.
                    if self
                        .binary
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.binary.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_binary(|id| -> Arc<RwLock<Binary>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.block_dirty {
            other.block.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in block, if it's not there add it to block.
                    if self
                        .block
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.block.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_block(|id| -> Arc<RwLock<Block>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.body_dirty {
            other.body.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in body, if it's not there add it to body.
                    if self
                        .body
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.body.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_body(|id| -> Arc<RwLock<Body>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.boolean_literal_dirty {
            other.boolean_literal.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in boolean_literal, if it's not there add it to boolean_literal.
                    if self
                        .boolean_literal
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.boolean_literal.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_boolean_literal(|id| -> Arc<RwLock<BooleanLiteral>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.boolean_operator_dirty {
            other.boolean_operator.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in boolean_operator, if it's not there add it to boolean_operator.
                    if self
                        .boolean_operator
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.boolean_operator.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_boolean_operator(|id| -> Arc<RwLock<BooleanOperator>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.call_dirty {
            other.call.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in call, if it's not there add it to call.
                    if self
                        .call
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.call.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_call(|id| -> Arc<RwLock<Call>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.char_literal_dirty {
            other.char_literal.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in char_literal, if it's not there add it to char_literal.
                    if self
                        .char_literal
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.char_literal.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_char_literal(|id| -> Arc<RwLock<CharLiteral>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.comparison_dirty {
            other.comparison.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in comparison, if it's not there add it to comparison.
                    if self
                        .comparison
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.comparison.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_comparison(|id| -> Arc<RwLock<Comparison>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.data_structure_dirty {
            other.data_structure.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in data_structure, if it's not there add it to data_structure.
                    if self
                        .data_structure
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.data_structure.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_data_structure(|id| -> Arc<RwLock<DataStructure>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.dwarf_source_file_dirty {
            other
                .dwarf_source_file
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in dwarf_source_file, if it's not there add it to dwarf_source_file.
                        if self
                            .dwarf_source_file
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.dwarf_source_file.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_dwarf_source_file(|id| -> Arc<RwLock<DwarfSourceFile>> {
                                if x.read().unwrap().id != id {
                                    dbg!(x, id);
                                    // panic!("id mismatch");
                                    x.write().unwrap().id = id;
                                }

                                x.clone()
                            });
                        }
                    }
                });
        }

        if other.enum_field_dirty {
            other.enum_field.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in enum_field, if it's not there add it to enum_field.
                    if self
                        .enum_field
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.enum_field.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_enum_field(|id| -> Arc<RwLock<EnumField>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.enum_generic_dirty {
            other.enum_generic.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in enum_generic, if it's not there add it to enum_generic.
                    if self
                        .enum_generic
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.enum_generic.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_enum_generic(|id| -> Arc<RwLock<EnumGeneric>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.enum_generic_type_dirty {
            other
                .enum_generic_type
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in enum_generic_type, if it's not there add it to enum_generic_type.
                        if self
                            .enum_generic_type
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.enum_generic_type.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_enum_generic_type(|id| -> Arc<RwLock<EnumGenericType>> {
                                if x.read().unwrap().id != id {
                                    dbg!(x, id);
                                    // panic!("id mismatch");
                                    x.write().unwrap().id = id;
                                }

                                x.clone()
                            });
                        }
                    }
                });
        }

        if other.enumeration_dirty {
            other.enumeration.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in enumeration, if it's not there add it to enumeration.
                    if self
                        .enumeration
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.enumeration.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_enumeration(|id| -> Arc<RwLock<Enumeration>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.expression_dirty {
            other.expression.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in expression, if it's not there add it to expression.
                    if self
                        .expression
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.expression.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_expression(|id| -> Arc<RwLock<Expression>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.expression_bit_dirty {
            other.expression_bit.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in expression_bit, if it's not there add it to expression_bit.
                    if self
                        .expression_bit
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.expression_bit.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_expression_bit(|id| -> Arc<RwLock<ExpressionBit>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.expression_statement_dirty {
            other
                .expression_statement
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in expression_statement, if it's not there add it to expression_statement.
                        if self
                            .expression_statement
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.expression_statement.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_expression_statement(
                                |id| -> Arc<RwLock<ExpressionStatement>> {
                                    if x.read().unwrap().id != id {
                                        dbg!(x, id);
                                        // panic!("id mismatch");
                                        x.write().unwrap().id = id;
                                    }

                                    x.clone()
                                },
                            );
                        }
                    }
                });
        }

        if other.external_implementation_dirty {
            other
                .external_implementation
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in external_implementation, if it's not there add it to external_implementation.
                        if self
                            .external_implementation
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.external_implementation.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_external_implementation(
                                |id| -> Arc<RwLock<ExternalImplementation>> {
                                    if x.read().unwrap().id != id {
                                        dbg!(x, id);
                                        // panic!("id mismatch");
                                        x.write().unwrap().id = id;
                                    }

                                    x.clone()
                                },
                            );
                        }
                    }
                });
        }

        if other.field_dirty {
            other.field.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in field, if it's not there add it to field.
                    if self
                        .field
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.field.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_field(|id| -> Arc<RwLock<Field>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.field_access_dirty {
            other.field_access.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in field_access, if it's not there add it to field_access.
                    if self
                        .field_access
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.field_access.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_field_access(|id| -> Arc<RwLock<FieldAccess>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.field_access_target_dirty {
            other
                .field_access_target
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in field_access_target, if it's not there add it to field_access_target.
                        if self
                            .field_access_target
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.field_access_target.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_field_access_target(
                                |id| -> Arc<RwLock<FieldAccessTarget>> {
                                    if x.read().unwrap().id != id {
                                        dbg!(x, id);
                                        // panic!("id mismatch");
                                        x.write().unwrap().id = id;
                                    }

                                    x.clone()
                                },
                            );
                        }
                    }
                });
        }

        if other.field_expression_dirty {
            other.field_expression.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in field_expression, if it's not there add it to field_expression.
                    if self
                        .field_expression
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.field_expression.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_field_expression(|id| -> Arc<RwLock<FieldExpression>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.float_literal_dirty {
            other.float_literal.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in float_literal, if it's not there add it to float_literal.
                    if self
                        .float_literal
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.float_literal.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_float_literal(|id| -> Arc<RwLock<FloatLiteral>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.for_loop_dirty {
            other.for_loop.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in for_loop, if it's not there add it to for_loop.
                    if self
                        .for_loop
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.for_loop.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_for_loop(|id| -> Arc<RwLock<ForLoop>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.format_bit_dirty {
            other.format_bit.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in format_bit, if it's not there add it to format_bit.
                    if self
                        .format_bit
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.format_bit.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_format_bit(|id| -> Arc<RwLock<FormatBit>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.format_string_dirty {
            other.format_string.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in format_string, if it's not there add it to format_string.
                    if self
                        .format_string
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.format_string.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_format_string(|id| -> Arc<RwLock<FormatString>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.func_generic_dirty {
            other.func_generic.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in func_generic, if it's not there add it to func_generic.
                    if self
                        .func_generic
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.func_generic.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_func_generic(|id| -> Arc<RwLock<FuncGeneric>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.function_dirty {
            other.function.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in function, if it's not there add it to function.
                    if self
                        .function
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.function.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_function(|id| -> Arc<RwLock<Function>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.function_call_dirty {
            other.function_call.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in function_call, if it's not there add it to function_call.
                    if self
                        .function_call
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.function_call.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_function_call(|id| -> Arc<RwLock<FunctionCall>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_future_dirty {
            other.x_future.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_future, if it's not there add it to x_future.
                    if self
                        .x_future
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.x_future.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_x_future(|id| -> Arc<RwLock<XFuture>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.grouped_dirty {
            other.grouped.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in grouped, if it's not there add it to grouped.
                    if self
                        .grouped
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.grouped.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_grouped(|id| -> Arc<RwLock<Grouped>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.halt_and_catch_fire_dirty {
            other
                .halt_and_catch_fire
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in halt_and_catch_fire, if it's not there add it to halt_and_catch_fire.
                        if self
                            .halt_and_catch_fire
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.halt_and_catch_fire.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_halt_and_catch_fire(|id| -> Arc<RwLock<HaltAndCatchFire>> {
                                if x.read().unwrap().id != id {
                                    dbg!(x, id);
                                    // panic!("id mismatch");
                                    x.write().unwrap().id = id;
                                }

                                x.clone()
                            });
                        }
                    }
                });
        }

        if other.x_if_dirty {
            other.x_if.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_if, if it's not there add it to x_if.
                    if self
                        .x_if
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.x_if.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_x_if(|id| -> Arc<RwLock<XIf>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.implementation_block_dirty {
            other
                .implementation_block
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in implementation_block, if it's not there add it to implementation_block.
                        if self
                            .implementation_block
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.implementation_block.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_implementation_block(
                                |id| -> Arc<RwLock<ImplementationBlock>> {
                                    if x.read().unwrap().id != id {
                                        dbg!(x, id);
                                        // panic!("id mismatch");
                                        x.write().unwrap().id = id;
                                    }

                                    x.clone()
                                },
                            );
                        }
                    }
                });
        }

        if other.import_dirty {
            other.import.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in import, if it's not there add it to import.
                    if self
                        .import
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.import.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_import(|id| -> Arc<RwLock<Import>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.index_dirty {
            other.index.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in index, if it's not there add it to index.
                    if self
                        .index
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.index.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_index(|id| -> Arc<RwLock<Index>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.integer_literal_dirty {
            other.integer_literal.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in integer_literal, if it's not there add it to integer_literal.
                    if self
                        .integer_literal
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.integer_literal.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_integer_literal(|id| -> Arc<RwLock<IntegerLiteral>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.item_dirty {
            other.item.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in item, if it's not there add it to item.
                    if self
                        .item
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.item.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_item(|id| -> Arc<RwLock<Item>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.lambda_dirty {
            other.lambda.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in lambda, if it's not there add it to lambda.
                    if self
                        .lambda
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.lambda.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_lambda(|id| -> Arc<RwLock<Lambda>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.lambda_parameter_dirty {
            other.lambda_parameter.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in lambda_parameter, if it's not there add it to lambda_parameter.
                    if self
                        .lambda_parameter
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.lambda_parameter.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_lambda_parameter(|id| -> Arc<RwLock<LambdaParameter>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.let_statement_dirty {
            other.let_statement.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in let_statement, if it's not there add it to let_statement.
                    if self
                        .let_statement
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.let_statement.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_let_statement(|id| -> Arc<RwLock<LetStatement>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.list_dirty {
            other.list.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in list, if it's not there add it to list.
                    if self
                        .list
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.list.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_list(|id| -> Arc<RwLock<List>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.list_element_dirty {
            other.list_element.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in list_element, if it's not there add it to list_element.
                    if self
                        .list_element
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.list_element.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_list_element(|id| -> Arc<RwLock<ListElement>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.list_expression_dirty {
            other.list_expression.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in list_expression, if it's not there add it to list_expression.
                    if self
                        .list_expression
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.list_expression.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_list_expression(|id| -> Arc<RwLock<ListExpression>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.literal_dirty {
            other.literal.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in literal, if it's not there add it to literal.
                    if self
                        .literal
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.literal.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_literal(|id| -> Arc<RwLock<Literal>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.local_variable_dirty {
            other.local_variable.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in local_variable, if it's not there add it to local_variable.
                    if self
                        .local_variable
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.local_variable.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_local_variable(|id| -> Arc<RwLock<LocalVariable>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_macro_dirty {
            other.x_macro.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_macro, if it's not there add it to x_macro.
                    if self
                        .x_macro
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.x_macro.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_x_macro(|id| -> Arc<RwLock<XMacro>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.map_dirty {
            other.map.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in map, if it's not there add it to map.
                    if self
                        .map
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.map.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_map(|id| -> Arc<RwLock<Map>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.map_element_dirty {
            other.map_element.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in map_element, if it's not there add it to map_element.
                    if self
                        .map_element
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.map_element.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_map_element(|id| -> Arc<RwLock<MapElement>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.map_expression_dirty {
            other.map_expression.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in map_expression, if it's not there add it to map_expression.
                    if self
                        .map_expression
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.map_expression.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_map_expression(|id| -> Arc<RwLock<MapExpression>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_match_dirty {
            other.x_match.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_match, if it's not there add it to x_match.
                    if self
                        .x_match
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.x_match.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_x_match(|id| -> Arc<RwLock<XMatch>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.method_call_dirty {
            other.method_call.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in method_call, if it's not there add it to method_call.
                    if self
                        .method_call
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.method_call.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_method_call(|id| -> Arc<RwLock<MethodCall>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.named_field_expression_dirty {
            other
                .named_field_expression
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in named_field_expression, if it's not there add it to named_field_expression.
                        if self
                            .named_field_expression
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.named_field_expression.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_named_field_expression(
                                |id| -> Arc<RwLock<NamedFieldExpression>> {
                                    if x.read().unwrap().id != id {
                                        dbg!(x, id);
                                        // panic!("id mismatch");
                                        x.write().unwrap().id = id;
                                    }

                                    x.clone()
                                },
                            );
                        }
                    }
                });
        }

        if other.z_object_store_dirty {
            other.z_object_store.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in z_object_store, if it's not there add it to z_object_store.
                    if self
                        .z_object_store
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.z_object_store.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_z_object_store(|id| -> Arc<RwLock<ZObjectStore>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.object_wrapper_dirty {
            other.object_wrapper.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in object_wrapper, if it's not there add it to object_wrapper.
                    if self
                        .object_wrapper
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.object_wrapper.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_object_wrapper(|id| -> Arc<RwLock<ObjectWrapper>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.operator_dirty {
            other.operator.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in operator, if it's not there add it to operator.
                    if self
                        .operator
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.operator.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_operator(|id| -> Arc<RwLock<Operator>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.parameter_dirty {
            other.parameter.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in parameter, if it's not there add it to parameter.
                    if self
                        .parameter
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.parameter.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_parameter(|id| -> Arc<RwLock<Parameter>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_path_dirty {
            other.x_path.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_path, if it's not there add it to x_path.
                    if self
                        .x_path
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.x_path.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_x_path(|id| -> Arc<RwLock<XPath>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.path_element_dirty {
            other.path_element.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in path_element, if it's not there add it to path_element.
                    if self
                        .path_element
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.path_element.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_path_element(|id| -> Arc<RwLock<PathElement>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.pattern_dirty {
            other.pattern.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in pattern, if it's not there add it to pattern.
                    if self
                        .pattern
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.pattern.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_pattern(|id| -> Arc<RwLock<Pattern>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_plugin_dirty {
            other.x_plugin.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_plugin, if it's not there add it to x_plugin.
                    if self
                        .x_plugin
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.x_plugin.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_x_plugin(|id| -> Arc<RwLock<XPlugin>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_print_dirty {
            other.x_print.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_print, if it's not there add it to x_print.
                    if self
                        .x_print
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.x_print.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_x_print(|id| -> Arc<RwLock<XPrint>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.range_expression_dirty {
            other.range_expression.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in range_expression, if it's not there add it to range_expression.
                    if self
                        .range_expression
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.range_expression.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_range_expression(|id| -> Arc<RwLock<RangeExpression>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.result_statement_dirty {
            other.result_statement.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in result_statement, if it's not there add it to result_statement.
                    if self
                        .result_statement
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.result_statement.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_result_statement(|id| -> Arc<RwLock<ResultStatement>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_return_dirty {
            other.x_return.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_return, if it's not there add it to x_return.
                    if self
                        .x_return
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.x_return.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_x_return(|id| -> Arc<RwLock<XReturn>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.span_dirty {
            other.span.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in span, if it's not there add it to span.
                    if self
                        .span
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.span.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_span(|id| -> Arc<RwLock<Span>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.statement_dirty {
            other.statement.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in statement, if it's not there add it to statement.
                    if self
                        .statement
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.statement.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_statement(|id| -> Arc<RwLock<Statement>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.static_method_call_dirty {
            other
                .static_method_call
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in static_method_call, if it's not there add it to static_method_call.
                        if self
                            .static_method_call
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.static_method_call.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_static_method_call(|id| -> Arc<RwLock<StaticMethodCall>> {
                                if x.read().unwrap().id != id {
                                    dbg!(x, id);
                                    // panic!("id mismatch");
                                    x.write().unwrap().id = id;
                                }

                                x.clone()
                            });
                        }
                    }
                });
        }

        if other.string_bit_dirty {
            other.string_bit.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in string_bit, if it's not there add it to string_bit.
                    if self
                        .string_bit
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.string_bit.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_string_bit(|id| -> Arc<RwLock<StringBit>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.string_literal_dirty {
            other.string_literal.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in string_literal, if it's not there add it to string_literal.
                    if self
                        .string_literal
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.string_literal.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_string_literal(|id| -> Arc<RwLock<StringLiteral>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.woog_struct_dirty {
            other.woog_struct.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in woog_struct, if it's not there add it to woog_struct.
                    if self
                        .woog_struct
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.woog_struct.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_woog_struct(|id| -> Arc<RwLock<WoogStruct>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.struct_expression_dirty {
            other
                .struct_expression
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in struct_expression, if it's not there add it to struct_expression.
                        if self
                            .struct_expression
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.struct_expression.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_struct_expression(|id| -> Arc<RwLock<StructExpression>> {
                                if x.read().unwrap().id != id {
                                    dbg!(x, id);
                                    // panic!("id mismatch");
                                    x.write().unwrap().id = id;
                                }

                                x.clone()
                            });
                        }
                    }
                });
        }

        if other.struct_field_dirty {
            other.struct_field.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in struct_field, if it's not there add it to struct_field.
                    if self
                        .struct_field
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.struct_field.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_struct_field(|id| -> Arc<RwLock<StructField>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.struct_generic_dirty {
            other.struct_generic.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in struct_generic, if it's not there add it to struct_generic.
                    if self
                        .struct_generic
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.struct_generic.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_struct_generic(|id| -> Arc<RwLock<StructGeneric>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.tuple_field_dirty {
            other.tuple_field.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in tuple_field, if it's not there add it to tuple_field.
                    if self
                        .tuple_field
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.tuple_field.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_tuple_field(|id| -> Arc<RwLock<TupleField>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.type_cast_dirty {
            other.type_cast.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in type_cast, if it's not there add it to type_cast.
                    if self
                        .type_cast
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.type_cast.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_type_cast(|id| -> Arc<RwLock<TypeCast>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.unary_dirty {
            other.unary.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in unary, if it's not there add it to unary.
                    if self
                        .unary
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.unary.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_unary(|id| -> Arc<RwLock<Unary>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.unit_dirty {
            other.unit.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in unit, if it's not there add it to unit.
                    if self
                        .unit
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.unit.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_unit(|id| -> Arc<RwLock<Unit>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.unnamed_field_expression_dirty {
            other
                .unnamed_field_expression
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in unnamed_field_expression, if it's not there add it to unnamed_field_expression.
                        if self
                            .unnamed_field_expression
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.unnamed_field_expression.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_unnamed_field_expression(
                                |id| -> Arc<RwLock<UnnamedFieldExpression>> {
                                    if x.read().unwrap().id != id {
                                        dbg!(x, id);
                                        // panic!("id mismatch");
                                        x.write().unwrap().id = id;
                                    }

                                    x.clone()
                                },
                            );
                        }
                    }
                });
        }

        if other.x_value_dirty {
            other.x_value.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_value, if it's not there add it to x_value.
                    if self
                        .x_value
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.x_value.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_x_value(|id| -> Arc<RwLock<XValue>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.value_type_dirty {
            other.value_type.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in value_type, if it's not there add it to value_type.
                    if self
                        .value_type
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.value_type.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_value_type(|id| -> Arc<RwLock<ValueType>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.variable_dirty {
            other.variable.read().unwrap().iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in variable, if it's not there add it to variable.
                    if self
                        .variable
                        .read()
                        .unwrap()
                        .iter()
                        .find(|&y| {
                            if let Some(y) = y {
                                *y.read().unwrap() == *x.read().unwrap()
                            } else {
                                false
                            }
                        })
                        .is_none()
                    {
                        // let _index_ = self.variable.read().unwrap().len();
                        // if x.read().unwrap().id != _index_ {
                        //     x.write().unwrap().id = _index_;
                        // }
                        self.inter_variable(|id| -> Arc<RwLock<Variable>> {
                            if x.read().unwrap().id != id {
                                dbg!(x, id);
                                // panic!("id mismatch");
                                x.write().unwrap().id = id;
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.variable_expression_dirty {
            other
                .variable_expression
                .read()
                .unwrap()
                .iter()
                .for_each(|x| {
                    if let Some(x) = x {
                        // Look for other in variable_expression, if it's not there add it to variable_expression.
                        if self
                            .variable_expression
                            .read()
                            .unwrap()
                            .iter()
                            .find(|&y| {
                                if let Some(y) = y {
                                    *y.read().unwrap() == *x.read().unwrap()
                                } else {
                                    false
                                }
                            })
                            .is_none()
                        {
                            // let _index_ = self.variable_expression.read().unwrap().len();
                            // if x.read().unwrap().id != _index_ {
                            //     x.write().unwrap().id = _index_;
                            // }
                            self.inter_variable_expression(
                                |id| -> Arc<RwLock<VariableExpression>> {
                                    if x.read().unwrap().id != id {
                                        dbg!(x, id);
                                        // panic!("id mismatch");
                                        x.write().unwrap().id = id;
                                    }

                                    x.clone()
                                },
                            );
                        }
                    }
                });
        }
    }
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock_vec-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    #[inline]
    pub fn inter_argument<F>(&mut self, argument: F) -> Arc<RwLock<Argument>>
    where
        F: Fn(usize) -> Arc<RwLock<Argument>>,
    {
        let _index = if let Some(_index) = self.argument_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.argument.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.argument.write().unwrap().push(None);
            _index
        };

        let argument = argument(_index);

        let found = if let Some(argument) = self.argument.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *argument.read().unwrap()
            } else {
                false
            }
        }) {
            argument.clone()
        } else {
            None
        };

        if let Some(argument) = found {
            tracing::debug!(target: "store", "found duplicate {argument:?}.");
            self.argument_free_list.lock().unwrap().push(_index);
            argument.clone()
        } else {
            tracing::debug!(target: "store", "interring {argument:?}.");
            self.argument.write().unwrap()[_index] = Some(argument.clone());
            self.argument_dirty = true;
            argument
        }
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    #[inline]
    pub fn exhume_argument(&self, id: &usize) -> Option<Arc<RwLock<Argument>>> {
        match self.argument.read().unwrap().get(*id) {
            Some(argument) => argument.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    #[inline]
    pub fn exorcise_argument(&mut self, id: &usize) -> Option<Arc<RwLock<Argument>>> {
        tracing::debug!(target: "store", "exorcising argument slot: {id}.");
        let result = self.argument.write().unwrap()[*id].take();
        self.argument_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    #[inline]
    pub fn iter_argument(&self) -> impl Iterator<Item = Arc<RwLock<Argument>>> + '_ {
        let len = self.argument.read().unwrap().len();
        (0..len)
            .filter(|i| self.argument.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.argument.read().unwrap()[i]
                    .as_ref()
                    .map(|argument| argument.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`AWait`] into the store.
    ///
    #[inline]
    pub fn inter_a_wait<F>(&mut self, a_wait: F) -> Arc<RwLock<AWait>>
    where
        F: Fn(usize) -> Arc<RwLock<AWait>>,
    {
        let _index = if let Some(_index) = self.a_wait_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.a_wait.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.a_wait.write().unwrap().push(None);
            _index
        };

        let a_wait = a_wait(_index);

        let found = if let Some(a_wait) = self.a_wait.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *a_wait.read().unwrap()
            } else {
                false
            }
        }) {
            a_wait.clone()
        } else {
            None
        };

        if let Some(a_wait) = found {
            tracing::debug!(target: "store", "found duplicate {a_wait:?}.");
            self.a_wait_free_list.lock().unwrap().push(_index);
            a_wait.clone()
        } else {
            tracing::debug!(target: "store", "interring {a_wait:?}.");
            self.a_wait.write().unwrap()[_index] = Some(a_wait.clone());
            self.a_wait_dirty = true;
            a_wait
        }
    }

    /// Exhume (get) [`AWait`] from the store.
    ///
    #[inline]
    pub fn exhume_a_wait(&self, id: &usize) -> Option<Arc<RwLock<AWait>>> {
        match self.a_wait.read().unwrap().get(*id) {
            Some(a_wait) => a_wait.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`AWait`] from the store.
    ///
    #[inline]
    pub fn exorcise_a_wait(&mut self, id: &usize) -> Option<Arc<RwLock<AWait>>> {
        tracing::debug!(target: "store", "exorcising a_wait slot: {id}.");
        let result = self.a_wait.write().unwrap()[*id].take();
        self.a_wait_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AWait>`.
    ///
    #[inline]
    pub fn iter_a_wait(&self) -> impl Iterator<Item = Arc<RwLock<AWait>>> + '_ {
        let len = self.a_wait.read().unwrap().len();
        (0..len)
            .filter(|i| self.a_wait.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.a_wait.read().unwrap()[i]
                    .as_ref()
                    .map(|a_wait| a_wait.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    #[inline]
    pub fn inter_binary<F>(&mut self, binary: F) -> Arc<RwLock<Binary>>
    where
        F: Fn(usize) -> Arc<RwLock<Binary>>,
    {
        let _index = if let Some(_index) = self.binary_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.binary.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.binary.write().unwrap().push(None);
            _index
        };

        let binary = binary(_index);

        let found = if let Some(binary) = self.binary.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *binary.read().unwrap()
            } else {
                false
            }
        }) {
            binary.clone()
        } else {
            None
        };

        if let Some(binary) = found {
            tracing::debug!(target: "store", "found duplicate {binary:?}.");
            self.binary_free_list.lock().unwrap().push(_index);
            binary.clone()
        } else {
            tracing::debug!(target: "store", "interring {binary:?}.");
            self.binary.write().unwrap()[_index] = Some(binary.clone());
            self.binary_dirty = true;
            binary
        }
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    #[inline]
    pub fn exhume_binary(&self, id: &usize) -> Option<Arc<RwLock<Binary>>> {
        match self.binary.read().unwrap().get(*id) {
            Some(binary) => binary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    #[inline]
    pub fn exorcise_binary(&mut self, id: &usize) -> Option<Arc<RwLock<Binary>>> {
        tracing::debug!(target: "store", "exorcising binary slot: {id}.");
        let result = self.binary.write().unwrap()[*id].take();
        self.binary_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    #[inline]
    pub fn iter_binary(&self) -> impl Iterator<Item = Arc<RwLock<Binary>>> + '_ {
        let len = self.binary.read().unwrap().len();
        (0..len)
            .filter(|i| self.binary.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.binary.read().unwrap()[i]
                    .as_ref()
                    .map(|binary| binary.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    #[inline]
    pub fn inter_block<F>(&mut self, block: F) -> Arc<RwLock<Block>>
    where
        F: Fn(usize) -> Arc<RwLock<Block>>,
    {
        let _index = if let Some(_index) = self.block_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.block.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.block.write().unwrap().push(None);
            _index
        };

        let block = block(_index);

        let found = if let Some(block) = self.block.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *block.read().unwrap()
            } else {
                false
            }
        }) {
            block.clone()
        } else {
            None
        };

        if let Some(block) = found {
            tracing::debug!(target: "store", "found duplicate {block:?}.");
            self.block_free_list.lock().unwrap().push(_index);
            block.clone()
        } else {
            tracing::debug!(target: "store", "interring {block:?}.");
            self.block.write().unwrap()[_index] = Some(block.clone());
            self.block_dirty = true;
            block
        }
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    #[inline]
    pub fn exhume_block(&self, id: &usize) -> Option<Arc<RwLock<Block>>> {
        match self.block.read().unwrap().get(*id) {
            Some(block) => block.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    #[inline]
    pub fn exorcise_block(&mut self, id: &usize) -> Option<Arc<RwLock<Block>>> {
        tracing::debug!(target: "store", "exorcising block slot: {id}.");
        let result = self.block.write().unwrap()[*id].take();
        self.block_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    #[inline]
    pub fn iter_block(&self) -> impl Iterator<Item = Arc<RwLock<Block>>> + '_ {
        let len = self.block.read().unwrap().len();
        (0..len)
            .filter(|i| self.block.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.block.read().unwrap()[i]
                    .as_ref()
                    .map(|block| block.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Body`] into the store.
    ///
    #[inline]
    pub fn inter_body<F>(&mut self, body: F) -> Arc<RwLock<Body>>
    where
        F: Fn(usize) -> Arc<RwLock<Body>>,
    {
        let _index = if let Some(_index) = self.body_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.body.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.body.write().unwrap().push(None);
            _index
        };

        let body = body(_index);

        let found = if let Some(body) = self.body.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *body.read().unwrap()
            } else {
                false
            }
        }) {
            body.clone()
        } else {
            None
        };

        if let Some(body) = found {
            tracing::debug!(target: "store", "found duplicate {body:?}.");
            self.body_free_list.lock().unwrap().push(_index);
            body.clone()
        } else {
            tracing::debug!(target: "store", "interring {body:?}.");
            self.body.write().unwrap()[_index] = Some(body.clone());
            self.body_dirty = true;
            body
        }
    }

    /// Exhume (get) [`Body`] from the store.
    ///
    #[inline]
    pub fn exhume_body(&self, id: &usize) -> Option<Arc<RwLock<Body>>> {
        match self.body.read().unwrap().get(*id) {
            Some(body) => body.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Body`] from the store.
    ///
    #[inline]
    pub fn exorcise_body(&mut self, id: &usize) -> Option<Arc<RwLock<Body>>> {
        tracing::debug!(target: "store", "exorcising body slot: {id}.");
        let result = self.body.write().unwrap()[*id].take();
        self.body_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Body>`.
    ///
    #[inline]
    pub fn iter_body(&self) -> impl Iterator<Item = Arc<RwLock<Body>>> + '_ {
        let len = self.body.read().unwrap().len();
        (0..len)
            .filter(|i| self.body.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.body.read().unwrap()[i]
                    .as_ref()
                    .map(|body| body.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_boolean_literal<F>(&mut self, boolean_literal: F) -> Arc<RwLock<BooleanLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<BooleanLiteral>>,
    {
        let _index = if let Some(_index) = self.boolean_literal_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.boolean_literal.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.boolean_literal.write().unwrap().push(None);
            _index
        };

        let boolean_literal = boolean_literal(_index);

        let found = if let Some(boolean_literal) =
            self.boolean_literal.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *boolean_literal.read().unwrap()
                } else {
                    false
                }
            }) {
            boolean_literal.clone()
        } else {
            None
        };

        if let Some(boolean_literal) = found {
            tracing::debug!(target: "store", "found duplicate {boolean_literal:?}.");
            self.boolean_literal_free_list.lock().unwrap().push(_index);
            boolean_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {boolean_literal:?}.");
            self.boolean_literal.write().unwrap()[_index] = Some(boolean_literal.clone());
            self.boolean_literal_dirty = true;
            boolean_literal
        }
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_boolean_literal(&self, id: &usize) -> Option<Arc<RwLock<BooleanLiteral>>> {
        match self.boolean_literal.read().unwrap().get(*id) {
            Some(boolean_literal) => boolean_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_boolean_literal(&mut self, id: &usize) -> Option<Arc<RwLock<BooleanLiteral>>> {
        tracing::debug!(target: "store", "exorcising boolean_literal slot: {id}.");
        let result = self.boolean_literal.write().unwrap()[*id].take();
        self.boolean_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    #[inline]
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Arc<RwLock<BooleanLiteral>>> + '_ {
        let len = self.boolean_literal.read().unwrap().len();
        (0..len)
            .filter(|i| self.boolean_literal.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.boolean_literal.read().unwrap()[i]
                    .as_ref()
                    .map(|boolean_literal| boolean_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    #[inline]
    pub fn inter_boolean_operator<F>(&mut self, boolean_operator: F) -> Arc<RwLock<BooleanOperator>>
    where
        F: Fn(usize) -> Arc<RwLock<BooleanOperator>>,
    {
        let _index = if let Some(_index) = self.boolean_operator_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.boolean_operator.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.boolean_operator.write().unwrap().push(None);
            _index
        };

        let boolean_operator = boolean_operator(_index);

        let found = if let Some(boolean_operator) =
            self.boolean_operator.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *boolean_operator.read().unwrap()
                } else {
                    false
                }
            }) {
            boolean_operator.clone()
        } else {
            None
        };

        if let Some(boolean_operator) = found {
            tracing::debug!(target: "store", "found duplicate {boolean_operator:?}.");
            self.boolean_operator_free_list.lock().unwrap().push(_index);
            boolean_operator.clone()
        } else {
            tracing::debug!(target: "store", "interring {boolean_operator:?}.");
            self.boolean_operator.write().unwrap()[_index] = Some(boolean_operator.clone());
            self.boolean_operator_dirty = true;
            boolean_operator
        }
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    #[inline]
    pub fn exhume_boolean_operator(&self, id: &usize) -> Option<Arc<RwLock<BooleanOperator>>> {
        match self.boolean_operator.read().unwrap().get(*id) {
            Some(boolean_operator) => boolean_operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    #[inline]
    pub fn exorcise_boolean_operator(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<BooleanOperator>>> {
        tracing::debug!(target: "store", "exorcising boolean_operator slot: {id}.");
        let result = self.boolean_operator.write().unwrap()[*id].take();
        self.boolean_operator_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    #[inline]
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = Arc<RwLock<BooleanOperator>>> + '_ {
        let len = self.boolean_operator.read().unwrap().len();
        (0..len)
            .filter(|i| self.boolean_operator.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.boolean_operator.read().unwrap()[i]
                    .as_ref()
                    .map(|boolean_operator| boolean_operator.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    #[inline]
    pub fn inter_call<F>(&mut self, call: F) -> Arc<RwLock<Call>>
    where
        F: Fn(usize) -> Arc<RwLock<Call>>,
    {
        let _index = if let Some(_index) = self.call_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.call.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.call.write().unwrap().push(None);
            _index
        };

        let call = call(_index);

        let found = if let Some(call) = self.call.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *call.read().unwrap()
            } else {
                false
            }
        }) {
            call.clone()
        } else {
            None
        };

        if let Some(call) = found {
            tracing::debug!(target: "store", "found duplicate {call:?}.");
            self.call_free_list.lock().unwrap().push(_index);
            call.clone()
        } else {
            tracing::debug!(target: "store", "interring {call:?}.");
            self.call.write().unwrap()[_index] = Some(call.clone());
            self.call_dirty = true;
            call
        }
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    #[inline]
    pub fn exhume_call(&self, id: &usize) -> Option<Arc<RwLock<Call>>> {
        match self.call.read().unwrap().get(*id) {
            Some(call) => call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    #[inline]
    pub fn exorcise_call(&mut self, id: &usize) -> Option<Arc<RwLock<Call>>> {
        tracing::debug!(target: "store", "exorcising call slot: {id}.");
        let result = self.call.write().unwrap()[*id].take();
        self.call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    #[inline]
    pub fn iter_call(&self) -> impl Iterator<Item = Arc<RwLock<Call>>> + '_ {
        let len = self.call.read().unwrap().len();
        (0..len)
            .filter(|i| self.call.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.call.read().unwrap()[i]
                    .as_ref()
                    .map(|call| call.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`CharLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_char_literal<F>(&mut self, char_literal: F) -> Arc<RwLock<CharLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<CharLiteral>>,
    {
        let _index = if let Some(_index) = self.char_literal_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.char_literal.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.char_literal.write().unwrap().push(None);
            _index
        };

        let char_literal = char_literal(_index);

        let found = if let Some(char_literal) =
            self.char_literal.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *char_literal.read().unwrap()
                } else {
                    false
                }
            }) {
            char_literal.clone()
        } else {
            None
        };

        if let Some(char_literal) = found {
            tracing::debug!(target: "store", "found duplicate {char_literal:?}.");
            self.char_literal_free_list.lock().unwrap().push(_index);
            char_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {char_literal:?}.");
            self.char_literal.write().unwrap()[_index] = Some(char_literal.clone());
            self.char_literal_dirty = true;
            char_literal
        }
    }

    /// Exhume (get) [`CharLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_char_literal(&self, id: &usize) -> Option<Arc<RwLock<CharLiteral>>> {
        match self.char_literal.read().unwrap().get(*id) {
            Some(char_literal) => char_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`CharLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_char_literal(&mut self, id: &usize) -> Option<Arc<RwLock<CharLiteral>>> {
        tracing::debug!(target: "store", "exorcising char_literal slot: {id}.");
        let result = self.char_literal.write().unwrap()[*id].take();
        self.char_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, CharLiteral>`.
    ///
    #[inline]
    pub fn iter_char_literal(&self) -> impl Iterator<Item = Arc<RwLock<CharLiteral>>> + '_ {
        let len = self.char_literal.read().unwrap().len();
        (0..len)
            .filter(|i| self.char_literal.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.char_literal.read().unwrap()[i]
                    .as_ref()
                    .map(|char_literal| char_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    #[inline]
    pub fn inter_comparison<F>(&mut self, comparison: F) -> Arc<RwLock<Comparison>>
    where
        F: Fn(usize) -> Arc<RwLock<Comparison>>,
    {
        let _index = if let Some(_index) = self.comparison_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.comparison.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.comparison.write().unwrap().push(None);
            _index
        };

        let comparison = comparison(_index);

        let found = if let Some(comparison) =
            self.comparison.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *comparison.read().unwrap()
                } else {
                    false
                }
            }) {
            comparison.clone()
        } else {
            None
        };

        if let Some(comparison) = found {
            tracing::debug!(target: "store", "found duplicate {comparison:?}.");
            self.comparison_free_list.lock().unwrap().push(_index);
            comparison.clone()
        } else {
            tracing::debug!(target: "store", "interring {comparison:?}.");
            self.comparison.write().unwrap()[_index] = Some(comparison.clone());
            self.comparison_dirty = true;
            comparison
        }
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    #[inline]
    pub fn exhume_comparison(&self, id: &usize) -> Option<Arc<RwLock<Comparison>>> {
        match self.comparison.read().unwrap().get(*id) {
            Some(comparison) => comparison.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    #[inline]
    pub fn exorcise_comparison(&mut self, id: &usize) -> Option<Arc<RwLock<Comparison>>> {
        tracing::debug!(target: "store", "exorcising comparison slot: {id}.");
        let result = self.comparison.write().unwrap()[*id].take();
        self.comparison_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    #[inline]
    pub fn iter_comparison(&self) -> impl Iterator<Item = Arc<RwLock<Comparison>>> + '_ {
        let len = self.comparison.read().unwrap().len();
        (0..len)
            .filter(|i| self.comparison.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.comparison.read().unwrap()[i]
                    .as_ref()
                    .map(|comparison| comparison.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`DataStructure`] into the store.
    ///
    #[inline]
    pub fn inter_data_structure<F>(&mut self, data_structure: F) -> Arc<RwLock<DataStructure>>
    where
        F: Fn(usize) -> Arc<RwLock<DataStructure>>,
    {
        let _index = if let Some(_index) = self.data_structure_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.data_structure.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.data_structure.write().unwrap().push(None);
            _index
        };

        let data_structure = data_structure(_index);

        let found = if let Some(data_structure) =
            self.data_structure.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *data_structure.read().unwrap()
                } else {
                    false
                }
            }) {
            data_structure.clone()
        } else {
            None
        };

        if let Some(data_structure) = found {
            tracing::debug!(target: "store", "found duplicate {data_structure:?}.");
            self.data_structure_free_list.lock().unwrap().push(_index);
            data_structure.clone()
        } else {
            tracing::debug!(target: "store", "interring {data_structure:?}.");
            self.data_structure.write().unwrap()[_index] = Some(data_structure.clone());
            self.data_structure_dirty = true;
            data_structure
        }
    }

    /// Exhume (get) [`DataStructure`] from the store.
    ///
    #[inline]
    pub fn exhume_data_structure(&self, id: &usize) -> Option<Arc<RwLock<DataStructure>>> {
        match self.data_structure.read().unwrap().get(*id) {
            Some(data_structure) => data_structure.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`DataStructure`] from the store.
    ///
    #[inline]
    pub fn exorcise_data_structure(&mut self, id: &usize) -> Option<Arc<RwLock<DataStructure>>> {
        tracing::debug!(target: "store", "exorcising data_structure slot: {id}.");
        let result = self.data_structure.write().unwrap()[*id].take();
        self.data_structure_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DataStructure>`.
    ///
    #[inline]
    pub fn iter_data_structure(&self) -> impl Iterator<Item = Arc<RwLock<DataStructure>>> + '_ {
        let len = self.data_structure.read().unwrap().len();
        (0..len)
            .filter(|i| self.data_structure.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.data_structure.read().unwrap()[i]
                    .as_ref()
                    .map(|data_structure| data_structure.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    #[inline]
    pub fn inter_dwarf_source_file<F>(
        &mut self,
        dwarf_source_file: F,
    ) -> Arc<RwLock<DwarfSourceFile>>
    where
        F: Fn(usize) -> Arc<RwLock<DwarfSourceFile>>,
    {
        let _index = if let Some(_index) = self.dwarf_source_file_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.dwarf_source_file.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.dwarf_source_file.write().unwrap().push(None);
            _index
        };

        let dwarf_source_file = dwarf_source_file(_index);

        let found = if let Some(dwarf_source_file) = self
            .dwarf_source_file
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *dwarf_source_file.read().unwrap()
                } else {
                    false
                }
            }) {
            dwarf_source_file.clone()
        } else {
            None
        };

        if let Some(dwarf_source_file) = found {
            tracing::debug!(target: "store", "found duplicate {dwarf_source_file:?}.");
            self.dwarf_source_file_free_list
                .lock()
                .unwrap()
                .push(_index);
            dwarf_source_file.clone()
        } else {
            tracing::debug!(target: "store", "interring {dwarf_source_file:?}.");
            self.dwarf_source_file.write().unwrap()[_index] = Some(dwarf_source_file.clone());
            self.dwarf_source_file_dirty = true;
            dwarf_source_file
        }
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    #[inline]
    pub fn exhume_dwarf_source_file(&self, id: &usize) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        match self.dwarf_source_file.read().unwrap().get(*id) {
            Some(dwarf_source_file) => dwarf_source_file.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    #[inline]
    pub fn exorcise_dwarf_source_file(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        tracing::debug!(target: "store", "exorcising dwarf_source_file slot: {id}.");
        let result = self.dwarf_source_file.write().unwrap()[*id].take();
        self.dwarf_source_file_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    #[inline]
    pub fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<DwarfSourceFile>>> + '_ {
        let len = self.dwarf_source_file.read().unwrap().len();
        (0..len)
            .filter(|i| self.dwarf_source_file.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.dwarf_source_file.read().unwrap()[i]
                    .as_ref()
                    .map(|dwarf_source_file| dwarf_source_file.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`EnumField`] into the store.
    ///
    #[inline]
    pub fn inter_enum_field<F>(&mut self, enum_field: F) -> Arc<RwLock<EnumField>>
    where
        F: Fn(usize) -> Arc<RwLock<EnumField>>,
    {
        let _index = if let Some(_index) = self.enum_field_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enum_field.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.enum_field.write().unwrap().push(None);
            _index
        };

        let enum_field = enum_field(_index);

        let found = if let Some(enum_field) =
            self.enum_field.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *enum_field.read().unwrap()
                } else {
                    false
                }
            }) {
            enum_field.clone()
        } else {
            None
        };

        if let Some(enum_field) = found {
            tracing::debug!(target: "store", "found duplicate {enum_field:?}.");
            self.enum_field_free_list.lock().unwrap().push(_index);
            enum_field.clone()
        } else {
            tracing::debug!(target: "store", "interring {enum_field:?}.");
            self.enum_field.write().unwrap()[_index] = Some(enum_field.clone());
            self.enum_field_dirty = true;
            enum_field
        }
    }

    /// Exhume (get) [`EnumField`] from the store.
    ///
    #[inline]
    pub fn exhume_enum_field(&self, id: &usize) -> Option<Arc<RwLock<EnumField>>> {
        match self.enum_field.read().unwrap().get(*id) {
            Some(enum_field) => enum_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`EnumField`] from the store.
    ///
    #[inline]
    pub fn exorcise_enum_field(&mut self, id: &usize) -> Option<Arc<RwLock<EnumField>>> {
        tracing::debug!(target: "store", "exorcising enum_field slot: {id}.");
        let result = self.enum_field.write().unwrap()[*id].take();
        self.enum_field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumField>`.
    ///
    #[inline]
    pub fn iter_enum_field(&self) -> impl Iterator<Item = Arc<RwLock<EnumField>>> + '_ {
        let len = self.enum_field.read().unwrap().len();
        (0..len)
            .filter(|i| self.enum_field.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.enum_field.read().unwrap()[i]
                    .as_ref()
                    .map(|enum_field| enum_field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`EnumGeneric`] into the store.
    ///
    #[inline]
    pub fn inter_enum_generic<F>(&mut self, enum_generic: F) -> Arc<RwLock<EnumGeneric>>
    where
        F: Fn(usize) -> Arc<RwLock<EnumGeneric>>,
    {
        let _index = if let Some(_index) = self.enum_generic_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enum_generic.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.enum_generic.write().unwrap().push(None);
            _index
        };

        let enum_generic = enum_generic(_index);

        let found = if let Some(enum_generic) =
            self.enum_generic.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *enum_generic.read().unwrap()
                } else {
                    false
                }
            }) {
            enum_generic.clone()
        } else {
            None
        };

        if let Some(enum_generic) = found {
            tracing::debug!(target: "store", "found duplicate {enum_generic:?}.");
            self.enum_generic_free_list.lock().unwrap().push(_index);
            enum_generic.clone()
        } else {
            tracing::debug!(target: "store", "interring {enum_generic:?}.");
            self.enum_generic.write().unwrap()[_index] = Some(enum_generic.clone());
            self.enum_generic_dirty = true;
            enum_generic
        }
    }

    /// Exhume (get) [`EnumGeneric`] from the store.
    ///
    #[inline]
    pub fn exhume_enum_generic(&self, id: &usize) -> Option<Arc<RwLock<EnumGeneric>>> {
        match self.enum_generic.read().unwrap().get(*id) {
            Some(enum_generic) => enum_generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`EnumGeneric`] from the store.
    ///
    #[inline]
    pub fn exorcise_enum_generic(&mut self, id: &usize) -> Option<Arc<RwLock<EnumGeneric>>> {
        tracing::debug!(target: "store", "exorcising enum_generic slot: {id}.");
        let result = self.enum_generic.write().unwrap()[*id].take();
        self.enum_generic_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGeneric>`.
    ///
    #[inline]
    pub fn iter_enum_generic(&self) -> impl Iterator<Item = Arc<RwLock<EnumGeneric>>> + '_ {
        let len = self.enum_generic.read().unwrap().len();
        (0..len)
            .filter(|i| self.enum_generic.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.enum_generic.read().unwrap()[i]
                    .as_ref()
                    .map(|enum_generic| enum_generic.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`EnumGenericType`] into the store.
    ///
    #[inline]
    pub fn inter_enum_generic_type<F>(
        &mut self,
        enum_generic_type: F,
    ) -> Arc<RwLock<EnumGenericType>>
    where
        F: Fn(usize) -> Arc<RwLock<EnumGenericType>>,
    {
        let _index = if let Some(_index) = self.enum_generic_type_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enum_generic_type.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.enum_generic_type.write().unwrap().push(None);
            _index
        };

        let enum_generic_type = enum_generic_type(_index);

        let found = if let Some(enum_generic_type) = self
            .enum_generic_type
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *enum_generic_type.read().unwrap()
                } else {
                    false
                }
            }) {
            enum_generic_type.clone()
        } else {
            None
        };

        if let Some(enum_generic_type) = found {
            tracing::debug!(target: "store", "found duplicate {enum_generic_type:?}.");
            self.enum_generic_type_free_list
                .lock()
                .unwrap()
                .push(_index);
            enum_generic_type.clone()
        } else {
            tracing::debug!(target: "store", "interring {enum_generic_type:?}.");
            self.enum_generic_type.write().unwrap()[_index] = Some(enum_generic_type.clone());
            self.enum_generic_type_dirty = true;
            enum_generic_type
        }
    }

    /// Exhume (get) [`EnumGenericType`] from the store.
    ///
    #[inline]
    pub fn exhume_enum_generic_type(&self, id: &usize) -> Option<Arc<RwLock<EnumGenericType>>> {
        match self.enum_generic_type.read().unwrap().get(*id) {
            Some(enum_generic_type) => enum_generic_type.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`EnumGenericType`] from the store.
    ///
    #[inline]
    pub fn exorcise_enum_generic_type(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<EnumGenericType>>> {
        tracing::debug!(target: "store", "exorcising enum_generic_type slot: {id}.");
        let result = self.enum_generic_type.write().unwrap()[*id].take();
        self.enum_generic_type_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGenericType>`.
    ///
    #[inline]
    pub fn iter_enum_generic_type(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<EnumGenericType>>> + '_ {
        let len = self.enum_generic_type.read().unwrap().len();
        (0..len)
            .filter(|i| self.enum_generic_type.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.enum_generic_type.read().unwrap()[i]
                    .as_ref()
                    .map(|enum_generic_type| enum_generic_type.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    #[inline]
    pub fn inter_enumeration<F>(&mut self, enumeration: F) -> Arc<RwLock<Enumeration>>
    where
        F: Fn(usize) -> Arc<RwLock<Enumeration>>,
    {
        let _index = if let Some(_index) = self.enumeration_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enumeration.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.enumeration.write().unwrap().push(None);
            _index
        };

        let enumeration = enumeration(_index);

        let found = if let Some(enumeration) =
            self.enumeration.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *enumeration.read().unwrap()
                } else {
                    false
                }
            }) {
            enumeration.clone()
        } else {
            None
        };

        let enumeration = if let Some(enumeration) = found {
            tracing::debug!(target: "store", "found duplicate {enumeration:?}.");
            self.enumeration_free_list.lock().unwrap().push(_index);
            enumeration.clone()
        } else {
            tracing::debug!(target: "store", "interring {enumeration:?}.");
            self.enumeration.write().unwrap()[_index] = Some(enumeration.clone());
            self.enumeration_dirty = true;
            enumeration
        };
        self.enumeration_id_by_name.write().unwrap().insert(
            enumeration.read().unwrap().name.to_owned(),
            enumeration.read().unwrap().id,
        );
        enumeration
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    #[inline]
    pub fn exhume_enumeration(&self, id: &usize) -> Option<Arc<RwLock<Enumeration>>> {
        match self.enumeration.read().unwrap().get(*id) {
            Some(enumeration) => enumeration.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    #[inline]
    pub fn exorcise_enumeration(&mut self, id: &usize) -> Option<Arc<RwLock<Enumeration>>> {
        tracing::debug!(target: "store", "exorcising enumeration slot: {id}.");
        let result = self.enumeration.write().unwrap()[*id].take();
        self.enumeration_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Enumeration`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_enumeration_id_by_name(&self, name: &str) -> Option<usize> {
        self.enumeration_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|enumeration| *enumeration)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    #[inline]
    pub fn iter_enumeration(&self) -> impl Iterator<Item = Arc<RwLock<Enumeration>>> + '_ {
        let len = self.enumeration.read().unwrap().len();
        (0..len)
            .filter(|i| self.enumeration.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.enumeration.read().unwrap()[i]
                    .as_ref()
                    .map(|enumeration| enumeration.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    #[inline]
    pub fn inter_expression<F>(&mut self, expression: F) -> Arc<RwLock<Expression>>
    where
        F: Fn(usize) -> Arc<RwLock<Expression>>,
    {
        let _index = if let Some(_index) = self.expression_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.expression.write().unwrap().push(None);
            _index
        };

        let expression = expression(_index);

        let found = if let Some(expression) =
            self.expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *expression.read().unwrap()
                } else {
                    false
                }
            }) {
            expression.clone()
        } else {
            None
        };

        if let Some(expression) = found {
            tracing::debug!(target: "store", "found duplicate {expression:?}.");
            self.expression_free_list.lock().unwrap().push(_index);
            expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {expression:?}.");
            self.expression.write().unwrap()[_index] = Some(expression.clone());
            self.expression_dirty = true;
            expression
        }
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    #[inline]
    pub fn exhume_expression(&self, id: &usize) -> Option<Arc<RwLock<Expression>>> {
        match self.expression.read().unwrap().get(*id) {
            Some(expression) => expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    #[inline]
    pub fn exorcise_expression(&mut self, id: &usize) -> Option<Arc<RwLock<Expression>>> {
        tracing::debug!(target: "store", "exorcising expression slot: {id}.");
        let result = self.expression.write().unwrap()[*id].take();
        self.expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    #[inline]
    pub fn iter_expression(&self) -> impl Iterator<Item = Arc<RwLock<Expression>>> + '_ {
        let len = self.expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.expression.read().unwrap()[i]
                    .as_ref()
                    .map(|expression| expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ExpressionBit`] into the store.
    ///
    #[inline]
    pub fn inter_expression_bit<F>(&mut self, expression_bit: F) -> Arc<RwLock<ExpressionBit>>
    where
        F: Fn(usize) -> Arc<RwLock<ExpressionBit>>,
    {
        let _index = if let Some(_index) = self.expression_bit_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression_bit.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.expression_bit.write().unwrap().push(None);
            _index
        };

        let expression_bit = expression_bit(_index);

        let found = if let Some(expression_bit) =
            self.expression_bit.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *expression_bit.read().unwrap()
                } else {
                    false
                }
            }) {
            expression_bit.clone()
        } else {
            None
        };

        if let Some(expression_bit) = found {
            tracing::debug!(target: "store", "found duplicate {expression_bit:?}.");
            self.expression_bit_free_list.lock().unwrap().push(_index);
            expression_bit.clone()
        } else {
            tracing::debug!(target: "store", "interring {expression_bit:?}.");
            self.expression_bit.write().unwrap()[_index] = Some(expression_bit.clone());
            self.expression_bit_dirty = true;
            expression_bit
        }
    }

    /// Exhume (get) [`ExpressionBit`] from the store.
    ///
    #[inline]
    pub fn exhume_expression_bit(&self, id: &usize) -> Option<Arc<RwLock<ExpressionBit>>> {
        match self.expression_bit.read().unwrap().get(*id) {
            Some(expression_bit) => expression_bit.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ExpressionBit`] from the store.
    ///
    #[inline]
    pub fn exorcise_expression_bit(&mut self, id: &usize) -> Option<Arc<RwLock<ExpressionBit>>> {
        tracing::debug!(target: "store", "exorcising expression_bit slot: {id}.");
        let result = self.expression_bit.write().unwrap()[*id].take();
        self.expression_bit_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionBit>`.
    ///
    #[inline]
    pub fn iter_expression_bit(&self) -> impl Iterator<Item = Arc<RwLock<ExpressionBit>>> + '_ {
        let len = self.expression_bit.read().unwrap().len();
        (0..len)
            .filter(|i| self.expression_bit.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.expression_bit.read().unwrap()[i]
                    .as_ref()
                    .map(|expression_bit| expression_bit.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    #[inline]
    pub fn inter_expression_statement<F>(
        &mut self,
        expression_statement: F,
    ) -> Arc<RwLock<ExpressionStatement>>
    where
        F: Fn(usize) -> Arc<RwLock<ExpressionStatement>>,
    {
        let _index = if let Some(_index) = self.expression_statement_free_list.lock().unwrap().pop()
        {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression_statement.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.expression_statement.write().unwrap().push(None);
            _index
        };

        let expression_statement = expression_statement(_index);

        let found = if let Some(expression_statement) = self
            .expression_statement
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *expression_statement.read().unwrap()
                } else {
                    false
                }
            }) {
            expression_statement.clone()
        } else {
            None
        };

        if let Some(expression_statement) = found {
            tracing::debug!(target: "store", "found duplicate {expression_statement:?}.");
            self.expression_statement_free_list
                .lock()
                .unwrap()
                .push(_index);
            expression_statement.clone()
        } else {
            tracing::debug!(target: "store", "interring {expression_statement:?}.");
            self.expression_statement.write().unwrap()[_index] = Some(expression_statement.clone());
            self.expression_statement_dirty = true;
            expression_statement
        }
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    #[inline]
    pub fn exhume_expression_statement(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        match self.expression_statement.read().unwrap().get(*id) {
            Some(expression_statement) => expression_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    #[inline]
    pub fn exorcise_expression_statement(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        tracing::debug!(target: "store", "exorcising expression_statement slot: {id}.");
        let result = self.expression_statement.write().unwrap()[*id].take();
        self.expression_statement_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    #[inline]
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ExpressionStatement>>> + '_ {
        let len = self.expression_statement.read().unwrap().len();
        (0..len)
            .filter(|i| self.expression_statement.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.expression_statement.read().unwrap()[i]
                    .as_ref()
                    .map(|expression_statement| expression_statement.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ExternalImplementation`] into the store.
    ///
    #[inline]
    pub fn inter_external_implementation<F>(
        &mut self,
        external_implementation: F,
    ) -> Arc<RwLock<ExternalImplementation>>
    where
        F: Fn(usize) -> Arc<RwLock<ExternalImplementation>>,
    {
        let _index =
            if let Some(_index) = self.external_implementation_free_list.lock().unwrap().pop() {
                tracing::trace!(target: "store", "recycling block {_index}.");
                _index
            } else {
                let _index = self.external_implementation.read().unwrap().len();
                tracing::trace!(target: "store", "allocating block {_index}.");
                self.external_implementation.write().unwrap().push(None);
                _index
            };

        let external_implementation = external_implementation(_index);

        let found = if let Some(external_implementation) = self
            .external_implementation
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *external_implementation.read().unwrap()
                } else {
                    false
                }
            }) {
            external_implementation.clone()
        } else {
            None
        };

        if let Some(external_implementation) = found {
            tracing::debug!(target: "store", "found duplicate {external_implementation:?}.");
            self.external_implementation_free_list
                .lock()
                .unwrap()
                .push(_index);
            external_implementation.clone()
        } else {
            tracing::debug!(target: "store", "interring {external_implementation:?}.");
            self.external_implementation.write().unwrap()[_index] =
                Some(external_implementation.clone());
            self.external_implementation_dirty = true;
            external_implementation
        }
    }

    /// Exhume (get) [`ExternalImplementation`] from the store.
    ///
    #[inline]
    pub fn exhume_external_implementation(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExternalImplementation>>> {
        match self.external_implementation.read().unwrap().get(*id) {
            Some(external_implementation) => external_implementation.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ExternalImplementation`] from the store.
    ///
    #[inline]
    pub fn exorcise_external_implementation(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExternalImplementation>>> {
        tracing::debug!(target: "store", "exorcising external_implementation slot: {id}.");
        let result = self.external_implementation.write().unwrap()[*id].take();
        self.external_implementation_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExternalImplementation>`.
    ///
    #[inline]
    pub fn iter_external_implementation(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ExternalImplementation>>> + '_ {
        let len = self.external_implementation.read().unwrap().len();
        (0..len)
            .filter(|i| self.external_implementation.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.external_implementation.read().unwrap()[i]
                    .as_ref()
                    .map(|external_implementation| external_implementation.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    #[inline]
    pub fn inter_field<F>(&mut self, field: F) -> Arc<RwLock<Field>>
    where
        F: Fn(usize) -> Arc<RwLock<Field>>,
    {
        let _index = if let Some(_index) = self.field_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.field.write().unwrap().push(None);
            _index
        };

        let field = field(_index);

        let found = if let Some(field) = self.field.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *field.read().unwrap()
            } else {
                false
            }
        }) {
            field.clone()
        } else {
            None
        };

        let field = if let Some(field) = found {
            tracing::debug!(target: "store", "found duplicate {field:?}.");
            self.field_free_list.lock().unwrap().push(_index);
            field.clone()
        } else {
            tracing::debug!(target: "store", "interring {field:?}.");
            self.field.write().unwrap()[_index] = Some(field.clone());
            self.field_dirty = true;
            field
        };
        self.field_id_by_name.write().unwrap().insert(
            field.read().unwrap().name.to_owned(),
            field.read().unwrap().id,
        );
        field
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    #[inline]
    pub fn exhume_field(&self, id: &usize) -> Option<Arc<RwLock<Field>>> {
        match self.field.read().unwrap().get(*id) {
            Some(field) => field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    #[inline]
    pub fn exorcise_field(&mut self, id: &usize) -> Option<Arc<RwLock<Field>>> {
        tracing::debug!(target: "store", "exorcising field slot: {id}.");
        let result = self.field.write().unwrap()[*id].take();
        self.field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Field`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<usize> {
        self.field_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|field| *field)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    #[inline]
    pub fn iter_field(&self) -> impl Iterator<Item = Arc<RwLock<Field>>> + '_ {
        let len = self.field.read().unwrap().len();
        (0..len)
            .filter(|i| self.field.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.field.read().unwrap()[i]
                    .as_ref()
                    .map(|field| field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    #[inline]
    pub fn inter_field_access<F>(&mut self, field_access: F) -> Arc<RwLock<FieldAccess>>
    where
        F: Fn(usize) -> Arc<RwLock<FieldAccess>>,
    {
        let _index = if let Some(_index) = self.field_access_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_access.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.field_access.write().unwrap().push(None);
            _index
        };

        let field_access = field_access(_index);

        let found = if let Some(field_access) =
            self.field_access.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *field_access.read().unwrap()
                } else {
                    false
                }
            }) {
            field_access.clone()
        } else {
            None
        };

        if let Some(field_access) = found {
            tracing::debug!(target: "store", "found duplicate {field_access:?}.");
            self.field_access_free_list.lock().unwrap().push(_index);
            field_access.clone()
        } else {
            tracing::debug!(target: "store", "interring {field_access:?}.");
            self.field_access.write().unwrap()[_index] = Some(field_access.clone());
            self.field_access_dirty = true;
            field_access
        }
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    #[inline]
    pub fn exhume_field_access(&self, id: &usize) -> Option<Arc<RwLock<FieldAccess>>> {
        match self.field_access.read().unwrap().get(*id) {
            Some(field_access) => field_access.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    #[inline]
    pub fn exorcise_field_access(&mut self, id: &usize) -> Option<Arc<RwLock<FieldAccess>>> {
        tracing::debug!(target: "store", "exorcising field_access slot: {id}.");
        let result = self.field_access.write().unwrap()[*id].take();
        self.field_access_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    #[inline]
    pub fn iter_field_access(&self) -> impl Iterator<Item = Arc<RwLock<FieldAccess>>> + '_ {
        let len = self.field_access.read().unwrap().len();
        (0..len)
            .filter(|i| self.field_access.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.field_access.read().unwrap()[i]
                    .as_ref()
                    .map(|field_access| field_access.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    #[inline]
    pub fn inter_field_access_target<F>(
        &mut self,
        field_access_target: F,
    ) -> Arc<RwLock<FieldAccessTarget>>
    where
        F: Fn(usize) -> Arc<RwLock<FieldAccessTarget>>,
    {
        let _index = if let Some(_index) = self.field_access_target_free_list.lock().unwrap().pop()
        {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_access_target.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.field_access_target.write().unwrap().push(None);
            _index
        };

        let field_access_target = field_access_target(_index);

        let found = if let Some(field_access_target) = self
            .field_access_target
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *field_access_target.read().unwrap()
                } else {
                    false
                }
            }) {
            field_access_target.clone()
        } else {
            None
        };

        if let Some(field_access_target) = found {
            tracing::debug!(target: "store", "found duplicate {field_access_target:?}.");
            self.field_access_target_free_list
                .lock()
                .unwrap()
                .push(_index);
            field_access_target.clone()
        } else {
            tracing::debug!(target: "store", "interring {field_access_target:?}.");
            self.field_access_target.write().unwrap()[_index] = Some(field_access_target.clone());
            self.field_access_target_dirty = true;
            field_access_target
        }
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    #[inline]
    pub fn exhume_field_access_target(&self, id: &usize) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        match self.field_access_target.read().unwrap().get(*id) {
            Some(field_access_target) => field_access_target.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    #[inline]
    pub fn exorcise_field_access_target(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        tracing::debug!(target: "store", "exorcising field_access_target slot: {id}.");
        let result = self.field_access_target.write().unwrap()[*id].take();
        self.field_access_target_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    #[inline]
    pub fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<FieldAccessTarget>>> + '_ {
        let len = self.field_access_target.read().unwrap().len();
        (0..len)
            .filter(|i| self.field_access_target.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.field_access_target.read().unwrap()[i]
                    .as_ref()
                    .map(|field_access_target| field_access_target.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    #[inline]
    pub fn inter_field_expression<F>(&mut self, field_expression: F) -> Arc<RwLock<FieldExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<FieldExpression>>,
    {
        let _index = if let Some(_index) = self.field_expression_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_expression.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.field_expression.write().unwrap().push(None);
            _index
        };

        let field_expression = field_expression(_index);

        let found = if let Some(field_expression) =
            self.field_expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *field_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            field_expression.clone()
        } else {
            None
        };

        if let Some(field_expression) = found {
            tracing::debug!(target: "store", "found duplicate {field_expression:?}.");
            self.field_expression_free_list.lock().unwrap().push(_index);
            field_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {field_expression:?}.");
            self.field_expression.write().unwrap()[_index] = Some(field_expression.clone());
            self.field_expression_dirty = true;
            field_expression
        }
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_field_expression(&self, id: &usize) -> Option<Arc<RwLock<FieldExpression>>> {
        match self.field_expression.read().unwrap().get(*id) {
            Some(field_expression) => field_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_field_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldExpression>>> {
        tracing::debug!(target: "store", "exorcising field_expression slot: {id}.");
        let result = self.field_expression.write().unwrap()[*id].take();
        self.field_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    #[inline]
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Arc<RwLock<FieldExpression>>> + '_ {
        let len = self.field_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.field_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.field_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|field_expression| field_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_float_literal<F>(&mut self, float_literal: F) -> Arc<RwLock<FloatLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<FloatLiteral>>,
    {
        let _index = if let Some(_index) = self.float_literal_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.float_literal.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.float_literal.write().unwrap().push(None);
            _index
        };

        let float_literal = float_literal(_index);

        let found = if let Some(float_literal) =
            self.float_literal.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *float_literal.read().unwrap()
                } else {
                    false
                }
            }) {
            float_literal.clone()
        } else {
            None
        };

        if let Some(float_literal) = found {
            tracing::debug!(target: "store", "found duplicate {float_literal:?}.");
            self.float_literal_free_list.lock().unwrap().push(_index);
            float_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {float_literal:?}.");
            self.float_literal.write().unwrap()[_index] = Some(float_literal.clone());
            self.float_literal_dirty = true;
            float_literal
        }
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_float_literal(&self, id: &usize) -> Option<Arc<RwLock<FloatLiteral>>> {
        match self.float_literal.read().unwrap().get(*id) {
            Some(float_literal) => float_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_float_literal(&mut self, id: &usize) -> Option<Arc<RwLock<FloatLiteral>>> {
        tracing::debug!(target: "store", "exorcising float_literal slot: {id}.");
        let result = self.float_literal.write().unwrap()[*id].take();
        self.float_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    #[inline]
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Arc<RwLock<FloatLiteral>>> + '_ {
        let len = self.float_literal.read().unwrap().len();
        (0..len)
            .filter(|i| self.float_literal.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.float_literal.read().unwrap()[i]
                    .as_ref()
                    .map(|float_literal| float_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    #[inline]
    pub fn inter_for_loop<F>(&mut self, for_loop: F) -> Arc<RwLock<ForLoop>>
    where
        F: Fn(usize) -> Arc<RwLock<ForLoop>>,
    {
        let _index = if let Some(_index) = self.for_loop_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.for_loop.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.for_loop.write().unwrap().push(None);
            _index
        };

        let for_loop = for_loop(_index);

        let found = if let Some(for_loop) = self.for_loop.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *for_loop.read().unwrap()
            } else {
                false
            }
        }) {
            for_loop.clone()
        } else {
            None
        };

        if let Some(for_loop) = found {
            tracing::debug!(target: "store", "found duplicate {for_loop:?}.");
            self.for_loop_free_list.lock().unwrap().push(_index);
            for_loop.clone()
        } else {
            tracing::debug!(target: "store", "interring {for_loop:?}.");
            self.for_loop.write().unwrap()[_index] = Some(for_loop.clone());
            self.for_loop_dirty = true;
            for_loop
        }
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    #[inline]
    pub fn exhume_for_loop(&self, id: &usize) -> Option<Arc<RwLock<ForLoop>>> {
        match self.for_loop.read().unwrap().get(*id) {
            Some(for_loop) => for_loop.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    #[inline]
    pub fn exorcise_for_loop(&mut self, id: &usize) -> Option<Arc<RwLock<ForLoop>>> {
        tracing::debug!(target: "store", "exorcising for_loop slot: {id}.");
        let result = self.for_loop.write().unwrap()[*id].take();
        self.for_loop_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    #[inline]
    pub fn iter_for_loop(&self) -> impl Iterator<Item = Arc<RwLock<ForLoop>>> + '_ {
        let len = self.for_loop.read().unwrap().len();
        (0..len)
            .filter(|i| self.for_loop.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.for_loop.read().unwrap()[i]
                    .as_ref()
                    .map(|for_loop| for_loop.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FormatBit`] into the store.
    ///
    #[inline]
    pub fn inter_format_bit<F>(&mut self, format_bit: F) -> Arc<RwLock<FormatBit>>
    where
        F: Fn(usize) -> Arc<RwLock<FormatBit>>,
    {
        let _index = if let Some(_index) = self.format_bit_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.format_bit.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.format_bit.write().unwrap().push(None);
            _index
        };

        let format_bit = format_bit(_index);

        let found = if let Some(format_bit) =
            self.format_bit.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *format_bit.read().unwrap()
                } else {
                    false
                }
            }) {
            format_bit.clone()
        } else {
            None
        };

        if let Some(format_bit) = found {
            tracing::debug!(target: "store", "found duplicate {format_bit:?}.");
            self.format_bit_free_list.lock().unwrap().push(_index);
            format_bit.clone()
        } else {
            tracing::debug!(target: "store", "interring {format_bit:?}.");
            self.format_bit.write().unwrap()[_index] = Some(format_bit.clone());
            self.format_bit_dirty = true;
            format_bit
        }
    }

    /// Exhume (get) [`FormatBit`] from the store.
    ///
    #[inline]
    pub fn exhume_format_bit(&self, id: &usize) -> Option<Arc<RwLock<FormatBit>>> {
        match self.format_bit.read().unwrap().get(*id) {
            Some(format_bit) => format_bit.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FormatBit`] from the store.
    ///
    #[inline]
    pub fn exorcise_format_bit(&mut self, id: &usize) -> Option<Arc<RwLock<FormatBit>>> {
        tracing::debug!(target: "store", "exorcising format_bit slot: {id}.");
        let result = self.format_bit.write().unwrap()[*id].take();
        self.format_bit_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FormatBit>`.
    ///
    #[inline]
    pub fn iter_format_bit(&self) -> impl Iterator<Item = Arc<RwLock<FormatBit>>> + '_ {
        let len = self.format_bit.read().unwrap().len();
        (0..len)
            .filter(|i| self.format_bit.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.format_bit.read().unwrap()[i]
                    .as_ref()
                    .map(|format_bit| format_bit.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FormatString`] into the store.
    ///
    #[inline]
    pub fn inter_format_string<F>(&mut self, format_string: F) -> Arc<RwLock<FormatString>>
    where
        F: Fn(usize) -> Arc<RwLock<FormatString>>,
    {
        let _index = if let Some(_index) = self.format_string_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.format_string.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.format_string.write().unwrap().push(None);
            _index
        };

        let format_string = format_string(_index);

        let found = if let Some(format_string) =
            self.format_string.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *format_string.read().unwrap()
                } else {
                    false
                }
            }) {
            format_string.clone()
        } else {
            None
        };

        if let Some(format_string) = found {
            tracing::debug!(target: "store", "found duplicate {format_string:?}.");
            self.format_string_free_list.lock().unwrap().push(_index);
            format_string.clone()
        } else {
            tracing::debug!(target: "store", "interring {format_string:?}.");
            self.format_string.write().unwrap()[_index] = Some(format_string.clone());
            self.format_string_dirty = true;
            format_string
        }
    }

    /// Exhume (get) [`FormatString`] from the store.
    ///
    #[inline]
    pub fn exhume_format_string(&self, id: &usize) -> Option<Arc<RwLock<FormatString>>> {
        match self.format_string.read().unwrap().get(*id) {
            Some(format_string) => format_string.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FormatString`] from the store.
    ///
    #[inline]
    pub fn exorcise_format_string(&mut self, id: &usize) -> Option<Arc<RwLock<FormatString>>> {
        tracing::debug!(target: "store", "exorcising format_string slot: {id}.");
        let result = self.format_string.write().unwrap()[*id].take();
        self.format_string_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FormatString>`.
    ///
    #[inline]
    pub fn iter_format_string(&self) -> impl Iterator<Item = Arc<RwLock<FormatString>>> + '_ {
        let len = self.format_string.read().unwrap().len();
        (0..len)
            .filter(|i| self.format_string.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.format_string.read().unwrap()[i]
                    .as_ref()
                    .map(|format_string| format_string.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FuncGeneric`] into the store.
    ///
    #[inline]
    pub fn inter_func_generic<F>(&mut self, func_generic: F) -> Arc<RwLock<FuncGeneric>>
    where
        F: Fn(usize) -> Arc<RwLock<FuncGeneric>>,
    {
        let _index = if let Some(_index) = self.func_generic_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.func_generic.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.func_generic.write().unwrap().push(None);
            _index
        };

        let func_generic = func_generic(_index);

        let found = if let Some(func_generic) =
            self.func_generic.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *func_generic.read().unwrap()
                } else {
                    false
                }
            }) {
            func_generic.clone()
        } else {
            None
        };

        if let Some(func_generic) = found {
            tracing::debug!(target: "store", "found duplicate {func_generic:?}.");
            self.func_generic_free_list.lock().unwrap().push(_index);
            func_generic.clone()
        } else {
            tracing::debug!(target: "store", "interring {func_generic:?}.");
            self.func_generic.write().unwrap()[_index] = Some(func_generic.clone());
            self.func_generic_dirty = true;
            func_generic
        }
    }

    /// Exhume (get) [`FuncGeneric`] from the store.
    ///
    #[inline]
    pub fn exhume_func_generic(&self, id: &usize) -> Option<Arc<RwLock<FuncGeneric>>> {
        match self.func_generic.read().unwrap().get(*id) {
            Some(func_generic) => func_generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FuncGeneric`] from the store.
    ///
    #[inline]
    pub fn exorcise_func_generic(&mut self, id: &usize) -> Option<Arc<RwLock<FuncGeneric>>> {
        tracing::debug!(target: "store", "exorcising func_generic slot: {id}.");
        let result = self.func_generic.write().unwrap()[*id].take();
        self.func_generic_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FuncGeneric>`.
    ///
    #[inline]
    pub fn iter_func_generic(&self) -> impl Iterator<Item = Arc<RwLock<FuncGeneric>>> + '_ {
        let len = self.func_generic.read().unwrap().len();
        (0..len)
            .filter(|i| self.func_generic.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.func_generic.read().unwrap()[i]
                    .as_ref()
                    .map(|func_generic| func_generic.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    #[inline]
    pub fn inter_function<F>(&mut self, function: F) -> Arc<RwLock<Function>>
    where
        F: Fn(usize) -> Arc<RwLock<Function>>,
    {
        let _index = if let Some(_index) = self.function_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.function.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.function.write().unwrap().push(None);
            _index
        };

        let function = function(_index);

        let found = if let Some(function) = self.function.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *function.read().unwrap()
            } else {
                false
            }
        }) {
            function.clone()
        } else {
            None
        };

        let function = if let Some(function) = found {
            tracing::debug!(target: "store", "found duplicate {function:?}.");
            self.function_free_list.lock().unwrap().push(_index);
            function.clone()
        } else {
            tracing::debug!(target: "store", "interring {function:?}.");
            self.function.write().unwrap()[_index] = Some(function.clone());
            self.function_dirty = true;
            function
        };
        self.function_id_by_name.write().unwrap().insert(
            function.read().unwrap().name.to_owned(),
            function.read().unwrap().id,
        );
        function
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    #[inline]
    pub fn exhume_function(&self, id: &usize) -> Option<Arc<RwLock<Function>>> {
        match self.function.read().unwrap().get(*id) {
            Some(function) => function.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    #[inline]
    pub fn exorcise_function(&mut self, id: &usize) -> Option<Arc<RwLock<Function>>> {
        tracing::debug!(target: "store", "exorcising function slot: {id}.");
        let result = self.function.write().unwrap()[*id].take();
        self.function_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Function`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<usize> {
        self.function_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|function| *function)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    #[inline]
    pub fn iter_function(&self) -> impl Iterator<Item = Arc<RwLock<Function>>> + '_ {
        let len = self.function.read().unwrap().len();
        (0..len)
            .filter(|i| self.function.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.function.read().unwrap()[i]
                    .as_ref()
                    .map(|function| function.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FunctionCall`] into the store.
    ///
    #[inline]
    pub fn inter_function_call<F>(&mut self, function_call: F) -> Arc<RwLock<FunctionCall>>
    where
        F: Fn(usize) -> Arc<RwLock<FunctionCall>>,
    {
        let _index = if let Some(_index) = self.function_call_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.function_call.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.function_call.write().unwrap().push(None);
            _index
        };

        let function_call = function_call(_index);

        let found = if let Some(function_call) =
            self.function_call.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *function_call.read().unwrap()
                } else {
                    false
                }
            }) {
            function_call.clone()
        } else {
            None
        };

        if let Some(function_call) = found {
            tracing::debug!(target: "store", "found duplicate {function_call:?}.");
            self.function_call_free_list.lock().unwrap().push(_index);
            function_call.clone()
        } else {
            tracing::debug!(target: "store", "interring {function_call:?}.");
            self.function_call.write().unwrap()[_index] = Some(function_call.clone());
            self.function_call_dirty = true;
            function_call
        }
    }

    /// Exhume (get) [`FunctionCall`] from the store.
    ///
    #[inline]
    pub fn exhume_function_call(&self, id: &usize) -> Option<Arc<RwLock<FunctionCall>>> {
        match self.function_call.read().unwrap().get(*id) {
            Some(function_call) => function_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FunctionCall`] from the store.
    ///
    #[inline]
    pub fn exorcise_function_call(&mut self, id: &usize) -> Option<Arc<RwLock<FunctionCall>>> {
        tracing::debug!(target: "store", "exorcising function_call slot: {id}.");
        let result = self.function_call.write().unwrap()[*id].take();
        self.function_call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FunctionCall>`.
    ///
    #[inline]
    pub fn iter_function_call(&self) -> impl Iterator<Item = Arc<RwLock<FunctionCall>>> + '_ {
        let len = self.function_call.read().unwrap().len();
        (0..len)
            .filter(|i| self.function_call.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.function_call.read().unwrap()[i]
                    .as_ref()
                    .map(|function_call| function_call.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XFuture`] into the store.
    ///
    #[inline]
    pub fn inter_x_future<F>(&mut self, x_future: F) -> Arc<RwLock<XFuture>>
    where
        F: Fn(usize) -> Arc<RwLock<XFuture>>,
    {
        let _index = if let Some(_index) = self.x_future_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_future.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_future.write().unwrap().push(None);
            _index
        };

        let x_future = x_future(_index);

        let found = if let Some(x_future) = self.x_future.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_future.read().unwrap()
            } else {
                false
            }
        }) {
            x_future.clone()
        } else {
            None
        };

        if let Some(x_future) = found {
            tracing::debug!(target: "store", "found duplicate {x_future:?}.");
            self.x_future_free_list.lock().unwrap().push(_index);
            x_future.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_future:?}.");
            self.x_future.write().unwrap()[_index] = Some(x_future.clone());
            self.x_future_dirty = true;
            x_future
        }
    }

    /// Exhume (get) [`XFuture`] from the store.
    ///
    #[inline]
    pub fn exhume_x_future(&self, id: &usize) -> Option<Arc<RwLock<XFuture>>> {
        match self.x_future.read().unwrap().get(*id) {
            Some(x_future) => x_future.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XFuture`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_future(&mut self, id: &usize) -> Option<Arc<RwLock<XFuture>>> {
        tracing::debug!(target: "store", "exorcising x_future slot: {id}.");
        let result = self.x_future.write().unwrap()[*id].take();
        self.x_future_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XFuture>`.
    ///
    #[inline]
    pub fn iter_x_future(&self) -> impl Iterator<Item = Arc<RwLock<XFuture>>> + '_ {
        let len = self.x_future.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_future.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_future.read().unwrap()[i]
                    .as_ref()
                    .map(|x_future| x_future.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    #[inline]
    pub fn inter_grouped<F>(&mut self, grouped: F) -> Arc<RwLock<Grouped>>
    where
        F: Fn(usize) -> Arc<RwLock<Grouped>>,
    {
        let _index = if let Some(_index) = self.grouped_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.grouped.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.grouped.write().unwrap().push(None);
            _index
        };

        let grouped = grouped(_index);

        let found = if let Some(grouped) = self.grouped.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *grouped.read().unwrap()
            } else {
                false
            }
        }) {
            grouped.clone()
        } else {
            None
        };

        if let Some(grouped) = found {
            tracing::debug!(target: "store", "found duplicate {grouped:?}.");
            self.grouped_free_list.lock().unwrap().push(_index);
            grouped.clone()
        } else {
            tracing::debug!(target: "store", "interring {grouped:?}.");
            self.grouped.write().unwrap()[_index] = Some(grouped.clone());
            self.grouped_dirty = true;
            grouped
        }
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    #[inline]
    pub fn exhume_grouped(&self, id: &usize) -> Option<Arc<RwLock<Grouped>>> {
        match self.grouped.read().unwrap().get(*id) {
            Some(grouped) => grouped.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    #[inline]
    pub fn exorcise_grouped(&mut self, id: &usize) -> Option<Arc<RwLock<Grouped>>> {
        tracing::debug!(target: "store", "exorcising grouped slot: {id}.");
        let result = self.grouped.write().unwrap()[*id].take();
        self.grouped_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    #[inline]
    pub fn iter_grouped(&self) -> impl Iterator<Item = Arc<RwLock<Grouped>>> + '_ {
        let len = self.grouped.read().unwrap().len();
        (0..len)
            .filter(|i| self.grouped.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.grouped.read().unwrap()[i]
                    .as_ref()
                    .map(|grouped| grouped.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`HaltAndCatchFire`] into the store.
    ///
    #[inline]
    pub fn inter_halt_and_catch_fire<F>(
        &mut self,
        halt_and_catch_fire: F,
    ) -> Arc<RwLock<HaltAndCatchFire>>
    where
        F: Fn(usize) -> Arc<RwLock<HaltAndCatchFire>>,
    {
        let _index = if let Some(_index) = self.halt_and_catch_fire_free_list.lock().unwrap().pop()
        {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.halt_and_catch_fire.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.halt_and_catch_fire.write().unwrap().push(None);
            _index
        };

        let halt_and_catch_fire = halt_and_catch_fire(_index);

        let found = if let Some(halt_and_catch_fire) = self
            .halt_and_catch_fire
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *halt_and_catch_fire.read().unwrap()
                } else {
                    false
                }
            }) {
            halt_and_catch_fire.clone()
        } else {
            None
        };

        if let Some(halt_and_catch_fire) = found {
            tracing::debug!(target: "store", "found duplicate {halt_and_catch_fire:?}.");
            self.halt_and_catch_fire_free_list
                .lock()
                .unwrap()
                .push(_index);
            halt_and_catch_fire.clone()
        } else {
            tracing::debug!(target: "store", "interring {halt_and_catch_fire:?}.");
            self.halt_and_catch_fire.write().unwrap()[_index] = Some(halt_and_catch_fire.clone());
            self.halt_and_catch_fire_dirty = true;
            halt_and_catch_fire
        }
    }

    /// Exhume (get) [`HaltAndCatchFire`] from the store.
    ///
    #[inline]
    pub fn exhume_halt_and_catch_fire(&self, id: &usize) -> Option<Arc<RwLock<HaltAndCatchFire>>> {
        match self.halt_and_catch_fire.read().unwrap().get(*id) {
            Some(halt_and_catch_fire) => halt_and_catch_fire.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`HaltAndCatchFire`] from the store.
    ///
    #[inline]
    pub fn exorcise_halt_and_catch_fire(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<HaltAndCatchFire>>> {
        tracing::debug!(target: "store", "exorcising halt_and_catch_fire slot: {id}.");
        let result = self.halt_and_catch_fire.write().unwrap()[*id].take();
        self.halt_and_catch_fire_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, HaltAndCatchFire>`.
    ///
    #[inline]
    pub fn iter_halt_and_catch_fire(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<HaltAndCatchFire>>> + '_ {
        let len = self.halt_and_catch_fire.read().unwrap().len();
        (0..len)
            .filter(|i| self.halt_and_catch_fire.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.halt_and_catch_fire.read().unwrap()[i]
                    .as_ref()
                    .map(|halt_and_catch_fire| halt_and_catch_fire.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    #[inline]
    pub fn inter_x_if<F>(&mut self, x_if: F) -> Arc<RwLock<XIf>>
    where
        F: Fn(usize) -> Arc<RwLock<XIf>>,
    {
        let _index = if let Some(_index) = self.x_if_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_if.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_if.write().unwrap().push(None);
            _index
        };

        let x_if = x_if(_index);

        let found = if let Some(x_if) = self.x_if.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_if.read().unwrap()
            } else {
                false
            }
        }) {
            x_if.clone()
        } else {
            None
        };

        if let Some(x_if) = found {
            tracing::debug!(target: "store", "found duplicate {x_if:?}.");
            self.x_if_free_list.lock().unwrap().push(_index);
            x_if.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_if:?}.");
            self.x_if.write().unwrap()[_index] = Some(x_if.clone());
            self.x_if_dirty = true;
            x_if
        }
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    #[inline]
    pub fn exhume_x_if(&self, id: &usize) -> Option<Arc<RwLock<XIf>>> {
        match self.x_if.read().unwrap().get(*id) {
            Some(x_if) => x_if.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_if(&mut self, id: &usize) -> Option<Arc<RwLock<XIf>>> {
        tracing::debug!(target: "store", "exorcising x_if slot: {id}.");
        let result = self.x_if.write().unwrap()[*id].take();
        self.x_if_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    #[inline]
    pub fn iter_x_if(&self) -> impl Iterator<Item = Arc<RwLock<XIf>>> + '_ {
        let len = self.x_if.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_if.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_if.read().unwrap()[i]
                    .as_ref()
                    .map(|x_if| x_if.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ImplementationBlock`] into the store.
    ///
    #[inline]
    pub fn inter_implementation_block<F>(
        &mut self,
        implementation_block: F,
    ) -> Arc<RwLock<ImplementationBlock>>
    where
        F: Fn(usize) -> Arc<RwLock<ImplementationBlock>>,
    {
        let _index = if let Some(_index) = self.implementation_block_free_list.lock().unwrap().pop()
        {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.implementation_block.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.implementation_block.write().unwrap().push(None);
            _index
        };

        let implementation_block = implementation_block(_index);

        let found = if let Some(implementation_block) = self
            .implementation_block
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *implementation_block.read().unwrap()
                } else {
                    false
                }
            }) {
            implementation_block.clone()
        } else {
            None
        };

        if let Some(implementation_block) = found {
            tracing::debug!(target: "store", "found duplicate {implementation_block:?}.");
            self.implementation_block_free_list
                .lock()
                .unwrap()
                .push(_index);
            implementation_block.clone()
        } else {
            tracing::debug!(target: "store", "interring {implementation_block:?}.");
            self.implementation_block.write().unwrap()[_index] = Some(implementation_block.clone());
            self.implementation_block_dirty = true;
            implementation_block
        }
    }

    /// Exhume (get) [`ImplementationBlock`] from the store.
    ///
    #[inline]
    pub fn exhume_implementation_block(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ImplementationBlock>>> {
        match self.implementation_block.read().unwrap().get(*id) {
            Some(implementation_block) => implementation_block.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ImplementationBlock`] from the store.
    ///
    #[inline]
    pub fn exorcise_implementation_block(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ImplementationBlock>>> {
        tracing::debug!(target: "store", "exorcising implementation_block slot: {id}.");
        let result = self.implementation_block.write().unwrap()[*id].take();
        self.implementation_block_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ImplementationBlock>`.
    ///
    #[inline]
    pub fn iter_implementation_block(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ImplementationBlock>>> + '_ {
        let len = self.implementation_block.read().unwrap().len();
        (0..len)
            .filter(|i| self.implementation_block.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.implementation_block.read().unwrap()[i]
                    .as_ref()
                    .map(|implementation_block| implementation_block.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    #[inline]
    pub fn inter_import<F>(&mut self, import: F) -> Arc<RwLock<Import>>
    where
        F: Fn(usize) -> Arc<RwLock<Import>>,
    {
        let _index = if let Some(_index) = self.import_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.import.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.import.write().unwrap().push(None);
            _index
        };

        let import = import(_index);

        let found = if let Some(import) = self.import.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *import.read().unwrap()
            } else {
                false
            }
        }) {
            import.clone()
        } else {
            None
        };

        if let Some(import) = found {
            tracing::debug!(target: "store", "found duplicate {import:?}.");
            self.import_free_list.lock().unwrap().push(_index);
            import.clone()
        } else {
            tracing::debug!(target: "store", "interring {import:?}.");
            self.import.write().unwrap()[_index] = Some(import.clone());
            self.import_dirty = true;
            import
        }
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    #[inline]
    pub fn exhume_import(&self, id: &usize) -> Option<Arc<RwLock<Import>>> {
        match self.import.read().unwrap().get(*id) {
            Some(import) => import.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    #[inline]
    pub fn exorcise_import(&mut self, id: &usize) -> Option<Arc<RwLock<Import>>> {
        tracing::debug!(target: "store", "exorcising import slot: {id}.");
        let result = self.import.write().unwrap()[*id].take();
        self.import_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    #[inline]
    pub fn iter_import(&self) -> impl Iterator<Item = Arc<RwLock<Import>>> + '_ {
        let len = self.import.read().unwrap().len();
        (0..len)
            .filter(|i| self.import.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.import.read().unwrap()[i]
                    .as_ref()
                    .map(|import| import.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    #[inline]
    pub fn inter_index<F>(&mut self, index: F) -> Arc<RwLock<Index>>
    where
        F: Fn(usize) -> Arc<RwLock<Index>>,
    {
        let _index = if let Some(_index) = self.index_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.index.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.index.write().unwrap().push(None);
            _index
        };

        let index = index(_index);

        let found = if let Some(index) = self.index.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *index.read().unwrap()
            } else {
                false
            }
        }) {
            index.clone()
        } else {
            None
        };

        if let Some(index) = found {
            tracing::debug!(target: "store", "found duplicate {index:?}.");
            self.index_free_list.lock().unwrap().push(_index);
            index.clone()
        } else {
            tracing::debug!(target: "store", "interring {index:?}.");
            self.index.write().unwrap()[_index] = Some(index.clone());
            self.index_dirty = true;
            index
        }
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    #[inline]
    pub fn exhume_index(&self, id: &usize) -> Option<Arc<RwLock<Index>>> {
        match self.index.read().unwrap().get(*id) {
            Some(index) => index.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    #[inline]
    pub fn exorcise_index(&mut self, id: &usize) -> Option<Arc<RwLock<Index>>> {
        tracing::debug!(target: "store", "exorcising index slot: {id}.");
        let result = self.index.write().unwrap()[*id].take();
        self.index_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    #[inline]
    pub fn iter_index(&self) -> impl Iterator<Item = Arc<RwLock<Index>>> + '_ {
        let len = self.index.read().unwrap().len();
        (0..len)
            .filter(|i| self.index.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.index.read().unwrap()[i]
                    .as_ref()
                    .map(|index| index.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_integer_literal<F>(&mut self, integer_literal: F) -> Arc<RwLock<IntegerLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<IntegerLiteral>>,
    {
        let _index = if let Some(_index) = self.integer_literal_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.integer_literal.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.integer_literal.write().unwrap().push(None);
            _index
        };

        let integer_literal = integer_literal(_index);

        let found = if let Some(integer_literal) =
            self.integer_literal.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *integer_literal.read().unwrap()
                } else {
                    false
                }
            }) {
            integer_literal.clone()
        } else {
            None
        };

        if let Some(integer_literal) = found {
            tracing::debug!(target: "store", "found duplicate {integer_literal:?}.");
            self.integer_literal_free_list.lock().unwrap().push(_index);
            integer_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {integer_literal:?}.");
            self.integer_literal.write().unwrap()[_index] = Some(integer_literal.clone());
            self.integer_literal_dirty = true;
            integer_literal
        }
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_integer_literal(&self, id: &usize) -> Option<Arc<RwLock<IntegerLiteral>>> {
        match self.integer_literal.read().unwrap().get(*id) {
            Some(integer_literal) => integer_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_integer_literal(&mut self, id: &usize) -> Option<Arc<RwLock<IntegerLiteral>>> {
        tracing::debug!(target: "store", "exorcising integer_literal slot: {id}.");
        let result = self.integer_literal.write().unwrap()[*id].take();
        self.integer_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    #[inline]
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Arc<RwLock<IntegerLiteral>>> + '_ {
        let len = self.integer_literal.read().unwrap().len();
        (0..len)
            .filter(|i| self.integer_literal.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.integer_literal.read().unwrap()[i]
                    .as_ref()
                    .map(|integer_literal| integer_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    #[inline]
    pub fn inter_item<F>(&mut self, item: F) -> Arc<RwLock<Item>>
    where
        F: Fn(usize) -> Arc<RwLock<Item>>,
    {
        let _index = if let Some(_index) = self.item_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.item.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.item.write().unwrap().push(None);
            _index
        };

        let item = item(_index);

        let found = if let Some(item) = self.item.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *item.read().unwrap()
            } else {
                false
            }
        }) {
            item.clone()
        } else {
            None
        };

        if let Some(item) = found {
            tracing::debug!(target: "store", "found duplicate {item:?}.");
            self.item_free_list.lock().unwrap().push(_index);
            item.clone()
        } else {
            tracing::debug!(target: "store", "interring {item:?}.");
            self.item.write().unwrap()[_index] = Some(item.clone());
            self.item_dirty = true;
            item
        }
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    #[inline]
    pub fn exhume_item(&self, id: &usize) -> Option<Arc<RwLock<Item>>> {
        match self.item.read().unwrap().get(*id) {
            Some(item) => item.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    #[inline]
    pub fn exorcise_item(&mut self, id: &usize) -> Option<Arc<RwLock<Item>>> {
        tracing::debug!(target: "store", "exorcising item slot: {id}.");
        let result = self.item.write().unwrap()[*id].take();
        self.item_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    #[inline]
    pub fn iter_item(&self) -> impl Iterator<Item = Arc<RwLock<Item>>> + '_ {
        let len = self.item.read().unwrap().len();
        (0..len)
            .filter(|i| self.item.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.item.read().unwrap()[i]
                    .as_ref()
                    .map(|item| item.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Lambda`] into the store.
    ///
    #[inline]
    pub fn inter_lambda<F>(&mut self, lambda: F) -> Arc<RwLock<Lambda>>
    where
        F: Fn(usize) -> Arc<RwLock<Lambda>>,
    {
        let _index = if let Some(_index) = self.lambda_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.lambda.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.lambda.write().unwrap().push(None);
            _index
        };

        let lambda = lambda(_index);

        let found = if let Some(lambda) = self.lambda.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *lambda.read().unwrap()
            } else {
                false
            }
        }) {
            lambda.clone()
        } else {
            None
        };

        if let Some(lambda) = found {
            tracing::debug!(target: "store", "found duplicate {lambda:?}.");
            self.lambda_free_list.lock().unwrap().push(_index);
            lambda.clone()
        } else {
            tracing::debug!(target: "store", "interring {lambda:?}.");
            self.lambda.write().unwrap()[_index] = Some(lambda.clone());
            self.lambda_dirty = true;
            lambda
        }
    }

    /// Exhume (get) [`Lambda`] from the store.
    ///
    #[inline]
    pub fn exhume_lambda(&self, id: &usize) -> Option<Arc<RwLock<Lambda>>> {
        match self.lambda.read().unwrap().get(*id) {
            Some(lambda) => lambda.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Lambda`] from the store.
    ///
    #[inline]
    pub fn exorcise_lambda(&mut self, id: &usize) -> Option<Arc<RwLock<Lambda>>> {
        tracing::debug!(target: "store", "exorcising lambda slot: {id}.");
        let result = self.lambda.write().unwrap()[*id].take();
        self.lambda_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Lambda>`.
    ///
    #[inline]
    pub fn iter_lambda(&self) -> impl Iterator<Item = Arc<RwLock<Lambda>>> + '_ {
        let len = self.lambda.read().unwrap().len();
        (0..len)
            .filter(|i| self.lambda.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.lambda.read().unwrap()[i]
                    .as_ref()
                    .map(|lambda| lambda.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`LambdaParameter`] into the store.
    ///
    #[inline]
    pub fn inter_lambda_parameter<F>(&mut self, lambda_parameter: F) -> Arc<RwLock<LambdaParameter>>
    where
        F: Fn(usize) -> Arc<RwLock<LambdaParameter>>,
    {
        let _index = if let Some(_index) = self.lambda_parameter_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.lambda_parameter.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.lambda_parameter.write().unwrap().push(None);
            _index
        };

        let lambda_parameter = lambda_parameter(_index);

        let found = if let Some(lambda_parameter) =
            self.lambda_parameter.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *lambda_parameter.read().unwrap()
                } else {
                    false
                }
            }) {
            lambda_parameter.clone()
        } else {
            None
        };

        if let Some(lambda_parameter) = found {
            tracing::debug!(target: "store", "found duplicate {lambda_parameter:?}.");
            self.lambda_parameter_free_list.lock().unwrap().push(_index);
            lambda_parameter.clone()
        } else {
            tracing::debug!(target: "store", "interring {lambda_parameter:?}.");
            self.lambda_parameter.write().unwrap()[_index] = Some(lambda_parameter.clone());
            self.lambda_parameter_dirty = true;
            lambda_parameter
        }
    }

    /// Exhume (get) [`LambdaParameter`] from the store.
    ///
    #[inline]
    pub fn exhume_lambda_parameter(&self, id: &usize) -> Option<Arc<RwLock<LambdaParameter>>> {
        match self.lambda_parameter.read().unwrap().get(*id) {
            Some(lambda_parameter) => lambda_parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LambdaParameter`] from the store.
    ///
    #[inline]
    pub fn exorcise_lambda_parameter(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<LambdaParameter>>> {
        tracing::debug!(target: "store", "exorcising lambda_parameter slot: {id}.");
        let result = self.lambda_parameter.write().unwrap()[*id].take();
        self.lambda_parameter_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LambdaParameter>`.
    ///
    #[inline]
    pub fn iter_lambda_parameter(&self) -> impl Iterator<Item = Arc<RwLock<LambdaParameter>>> + '_ {
        let len = self.lambda_parameter.read().unwrap().len();
        (0..len)
            .filter(|i| self.lambda_parameter.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.lambda_parameter.read().unwrap()[i]
                    .as_ref()
                    .map(|lambda_parameter| lambda_parameter.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    #[inline]
    pub fn inter_let_statement<F>(&mut self, let_statement: F) -> Arc<RwLock<LetStatement>>
    where
        F: Fn(usize) -> Arc<RwLock<LetStatement>>,
    {
        let _index = if let Some(_index) = self.let_statement_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.let_statement.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.let_statement.write().unwrap().push(None);
            _index
        };

        let let_statement = let_statement(_index);

        let found = if let Some(let_statement) =
            self.let_statement.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *let_statement.read().unwrap()
                } else {
                    false
                }
            }) {
            let_statement.clone()
        } else {
            None
        };

        if let Some(let_statement) = found {
            tracing::debug!(target: "store", "found duplicate {let_statement:?}.");
            self.let_statement_free_list.lock().unwrap().push(_index);
            let_statement.clone()
        } else {
            tracing::debug!(target: "store", "interring {let_statement:?}.");
            self.let_statement.write().unwrap()[_index] = Some(let_statement.clone());
            self.let_statement_dirty = true;
            let_statement
        }
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    #[inline]
    pub fn exhume_let_statement(&self, id: &usize) -> Option<Arc<RwLock<LetStatement>>> {
        match self.let_statement.read().unwrap().get(*id) {
            Some(let_statement) => let_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    #[inline]
    pub fn exorcise_let_statement(&mut self, id: &usize) -> Option<Arc<RwLock<LetStatement>>> {
        tracing::debug!(target: "store", "exorcising let_statement slot: {id}.");
        let result = self.let_statement.write().unwrap()[*id].take();
        self.let_statement_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    #[inline]
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Arc<RwLock<LetStatement>>> + '_ {
        let len = self.let_statement.read().unwrap().len();
        (0..len)
            .filter(|i| self.let_statement.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.let_statement.read().unwrap()[i]
                    .as_ref()
                    .map(|let_statement| let_statement.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`List`] into the store.
    ///
    #[inline]
    pub fn inter_list<F>(&mut self, list: F) -> Arc<RwLock<List>>
    where
        F: Fn(usize) -> Arc<RwLock<List>>,
    {
        let _index = if let Some(_index) = self.list_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.list.write().unwrap().push(None);
            _index
        };

        let list = list(_index);

        let found = if let Some(list) = self.list.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *list.read().unwrap()
            } else {
                false
            }
        }) {
            list.clone()
        } else {
            None
        };

        if let Some(list) = found {
            tracing::debug!(target: "store", "found duplicate {list:?}.");
            self.list_free_list.lock().unwrap().push(_index);
            list.clone()
        } else {
            tracing::debug!(target: "store", "interring {list:?}.");
            self.list.write().unwrap()[_index] = Some(list.clone());
            self.list_dirty = true;
            list
        }
    }

    /// Exhume (get) [`List`] from the store.
    ///
    #[inline]
    pub fn exhume_list(&self, id: &usize) -> Option<Arc<RwLock<List>>> {
        match self.list.read().unwrap().get(*id) {
            Some(list) => list.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    #[inline]
    pub fn exorcise_list(&mut self, id: &usize) -> Option<Arc<RwLock<List>>> {
        tracing::debug!(target: "store", "exorcising list slot: {id}.");
        let result = self.list.write().unwrap()[*id].take();
        self.list_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    #[inline]
    pub fn iter_list(&self) -> impl Iterator<Item = Arc<RwLock<List>>> + '_ {
        let len = self.list.read().unwrap().len();
        (0..len)
            .filter(|i| self.list.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.list.read().unwrap()[i]
                    .as_ref()
                    .map(|list| list.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    #[inline]
    pub fn inter_list_element<F>(&mut self, list_element: F) -> Arc<RwLock<ListElement>>
    where
        F: Fn(usize) -> Arc<RwLock<ListElement>>,
    {
        let _index = if let Some(_index) = self.list_element_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list_element.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.list_element.write().unwrap().push(None);
            _index
        };

        let list_element = list_element(_index);

        let found = if let Some(list_element) =
            self.list_element.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *list_element.read().unwrap()
                } else {
                    false
                }
            }) {
            list_element.clone()
        } else {
            None
        };

        if let Some(list_element) = found {
            tracing::debug!(target: "store", "found duplicate {list_element:?}.");
            self.list_element_free_list.lock().unwrap().push(_index);
            list_element.clone()
        } else {
            tracing::debug!(target: "store", "interring {list_element:?}.");
            self.list_element.write().unwrap()[_index] = Some(list_element.clone());
            self.list_element_dirty = true;
            list_element
        }
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    #[inline]
    pub fn exhume_list_element(&self, id: &usize) -> Option<Arc<RwLock<ListElement>>> {
        match self.list_element.read().unwrap().get(*id) {
            Some(list_element) => list_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    #[inline]
    pub fn exorcise_list_element(&mut self, id: &usize) -> Option<Arc<RwLock<ListElement>>> {
        tracing::debug!(target: "store", "exorcising list_element slot: {id}.");
        let result = self.list_element.write().unwrap()[*id].take();
        self.list_element_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    #[inline]
    pub fn iter_list_element(&self) -> impl Iterator<Item = Arc<RwLock<ListElement>>> + '_ {
        let len = self.list_element.read().unwrap().len();
        (0..len)
            .filter(|i| self.list_element.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.list_element.read().unwrap()[i]
                    .as_ref()
                    .map(|list_element| list_element.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    #[inline]
    pub fn inter_list_expression<F>(&mut self, list_expression: F) -> Arc<RwLock<ListExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<ListExpression>>,
    {
        let _index = if let Some(_index) = self.list_expression_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list_expression.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.list_expression.write().unwrap().push(None);
            _index
        };

        let list_expression = list_expression(_index);

        let found = if let Some(list_expression) =
            self.list_expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *list_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            list_expression.clone()
        } else {
            None
        };

        if let Some(list_expression) = found {
            tracing::debug!(target: "store", "found duplicate {list_expression:?}.");
            self.list_expression_free_list.lock().unwrap().push(_index);
            list_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {list_expression:?}.");
            self.list_expression.write().unwrap()[_index] = Some(list_expression.clone());
            self.list_expression_dirty = true;
            list_expression
        }
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_list_expression(&self, id: &usize) -> Option<Arc<RwLock<ListExpression>>> {
        match self.list_expression.read().unwrap().get(*id) {
            Some(list_expression) => list_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_list_expression(&mut self, id: &usize) -> Option<Arc<RwLock<ListExpression>>> {
        tracing::debug!(target: "store", "exorcising list_expression slot: {id}.");
        let result = self.list_expression.write().unwrap()[*id].take();
        self.list_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    #[inline]
    pub fn iter_list_expression(&self) -> impl Iterator<Item = Arc<RwLock<ListExpression>>> + '_ {
        let len = self.list_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.list_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.list_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|list_expression| list_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    #[inline]
    pub fn inter_literal<F>(&mut self, literal: F) -> Arc<RwLock<Literal>>
    where
        F: Fn(usize) -> Arc<RwLock<Literal>>,
    {
        let _index = if let Some(_index) = self.literal_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.literal.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.literal.write().unwrap().push(None);
            _index
        };

        let literal = literal(_index);

        let found = if let Some(literal) = self.literal.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *literal.read().unwrap()
            } else {
                false
            }
        }) {
            literal.clone()
        } else {
            None
        };

        if let Some(literal) = found {
            tracing::debug!(target: "store", "found duplicate {literal:?}.");
            self.literal_free_list.lock().unwrap().push(_index);
            literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {literal:?}.");
            self.literal.write().unwrap()[_index] = Some(literal.clone());
            self.literal_dirty = true;
            literal
        }
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    #[inline]
    pub fn exhume_literal(&self, id: &usize) -> Option<Arc<RwLock<Literal>>> {
        match self.literal.read().unwrap().get(*id) {
            Some(literal) => literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    #[inline]
    pub fn exorcise_literal(&mut self, id: &usize) -> Option<Arc<RwLock<Literal>>> {
        tracing::debug!(target: "store", "exorcising literal slot: {id}.");
        let result = self.literal.write().unwrap()[*id].take();
        self.literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    #[inline]
    pub fn iter_literal(&self) -> impl Iterator<Item = Arc<RwLock<Literal>>> + '_ {
        let len = self.literal.read().unwrap().len();
        (0..len)
            .filter(|i| self.literal.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.literal.read().unwrap()[i]
                    .as_ref()
                    .map(|literal| literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    #[inline]
    pub fn inter_local_variable<F>(&mut self, local_variable: F) -> Arc<RwLock<LocalVariable>>
    where
        F: Fn(usize) -> Arc<RwLock<LocalVariable>>,
    {
        let _index = if let Some(_index) = self.local_variable_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.local_variable.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.local_variable.write().unwrap().push(None);
            _index
        };

        let local_variable = local_variable(_index);

        let found = if let Some(local_variable) =
            self.local_variable.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *local_variable.read().unwrap()
                } else {
                    false
                }
            }) {
            local_variable.clone()
        } else {
            None
        };

        if let Some(local_variable) = found {
            tracing::debug!(target: "store", "found duplicate {local_variable:?}.");
            self.local_variable_free_list.lock().unwrap().push(_index);
            local_variable.clone()
        } else {
            tracing::debug!(target: "store", "interring {local_variable:?}.");
            self.local_variable.write().unwrap()[_index] = Some(local_variable.clone());
            self.local_variable_dirty = true;
            local_variable
        }
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    #[inline]
    pub fn exhume_local_variable(&self, id: &usize) -> Option<Arc<RwLock<LocalVariable>>> {
        match self.local_variable.read().unwrap().get(*id) {
            Some(local_variable) => local_variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    #[inline]
    pub fn exorcise_local_variable(&mut self, id: &usize) -> Option<Arc<RwLock<LocalVariable>>> {
        tracing::debug!(target: "store", "exorcising local_variable slot: {id}.");
        let result = self.local_variable.write().unwrap()[*id].take();
        self.local_variable_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    #[inline]
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Arc<RwLock<LocalVariable>>> + '_ {
        let len = self.local_variable.read().unwrap().len();
        (0..len)
            .filter(|i| self.local_variable.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.local_variable.read().unwrap()[i]
                    .as_ref()
                    .map(|local_variable| local_variable.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    #[inline]
    pub fn inter_x_macro<F>(&mut self, x_macro: F) -> Arc<RwLock<XMacro>>
    where
        F: Fn(usize) -> Arc<RwLock<XMacro>>,
    {
        let _index = if let Some(_index) = self.x_macro_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_macro.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_macro.write().unwrap().push(None);
            _index
        };

        let x_macro = x_macro(_index);

        let found = if let Some(x_macro) = self.x_macro.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_macro.read().unwrap()
            } else {
                false
            }
        }) {
            x_macro.clone()
        } else {
            None
        };

        if let Some(x_macro) = found {
            tracing::debug!(target: "store", "found duplicate {x_macro:?}.");
            self.x_macro_free_list.lock().unwrap().push(_index);
            x_macro.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_macro:?}.");
            self.x_macro.write().unwrap()[_index] = Some(x_macro.clone());
            self.x_macro_dirty = true;
            x_macro
        }
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    #[inline]
    pub fn exhume_x_macro(&self, id: &usize) -> Option<Arc<RwLock<XMacro>>> {
        match self.x_macro.read().unwrap().get(*id) {
            Some(x_macro) => x_macro.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_macro(&mut self, id: &usize) -> Option<Arc<RwLock<XMacro>>> {
        tracing::debug!(target: "store", "exorcising x_macro slot: {id}.");
        let result = self.x_macro.write().unwrap()[*id].take();
        self.x_macro_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    #[inline]
    pub fn iter_x_macro(&self) -> impl Iterator<Item = Arc<RwLock<XMacro>>> + '_ {
        let len = self.x_macro.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_macro.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_macro.read().unwrap()[i]
                    .as_ref()
                    .map(|x_macro| x_macro.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Map`] into the store.
    ///
    #[inline]
    pub fn inter_map<F>(&mut self, map: F) -> Arc<RwLock<Map>>
    where
        F: Fn(usize) -> Arc<RwLock<Map>>,
    {
        let _index = if let Some(_index) = self.map_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.map.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.map.write().unwrap().push(None);
            _index
        };

        let map = map(_index);

        let found = if let Some(map) = self.map.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *map.read().unwrap()
            } else {
                false
            }
        }) {
            map.clone()
        } else {
            None
        };

        if let Some(map) = found {
            tracing::debug!(target: "store", "found duplicate {map:?}.");
            self.map_free_list.lock().unwrap().push(_index);
            map.clone()
        } else {
            tracing::debug!(target: "store", "interring {map:?}.");
            self.map.write().unwrap()[_index] = Some(map.clone());
            self.map_dirty = true;
            map
        }
    }

    /// Exhume (get) [`Map`] from the store.
    ///
    #[inline]
    pub fn exhume_map(&self, id: &usize) -> Option<Arc<RwLock<Map>>> {
        match self.map.read().unwrap().get(*id) {
            Some(map) => map.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Map`] from the store.
    ///
    #[inline]
    pub fn exorcise_map(&mut self, id: &usize) -> Option<Arc<RwLock<Map>>> {
        tracing::debug!(target: "store", "exorcising map slot: {id}.");
        let result = self.map.write().unwrap()[*id].take();
        self.map_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Map>`.
    ///
    #[inline]
    pub fn iter_map(&self) -> impl Iterator<Item = Arc<RwLock<Map>>> + '_ {
        let len = self.map.read().unwrap().len();
        (0..len)
            .filter(|i| self.map.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.map.read().unwrap()[i]
                    .as_ref()
                    .map(|map| map.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`MapElement`] into the store.
    ///
    #[inline]
    pub fn inter_map_element<F>(&mut self, map_element: F) -> Arc<RwLock<MapElement>>
    where
        F: Fn(usize) -> Arc<RwLock<MapElement>>,
    {
        let _index = if let Some(_index) = self.map_element_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.map_element.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.map_element.write().unwrap().push(None);
            _index
        };

        let map_element = map_element(_index);

        let found = if let Some(map_element) =
            self.map_element.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *map_element.read().unwrap()
                } else {
                    false
                }
            }) {
            map_element.clone()
        } else {
            None
        };

        if let Some(map_element) = found {
            tracing::debug!(target: "store", "found duplicate {map_element:?}.");
            self.map_element_free_list.lock().unwrap().push(_index);
            map_element.clone()
        } else {
            tracing::debug!(target: "store", "interring {map_element:?}.");
            self.map_element.write().unwrap()[_index] = Some(map_element.clone());
            self.map_element_dirty = true;
            map_element
        }
    }

    /// Exhume (get) [`MapElement`] from the store.
    ///
    #[inline]
    pub fn exhume_map_element(&self, id: &usize) -> Option<Arc<RwLock<MapElement>>> {
        match self.map_element.read().unwrap().get(*id) {
            Some(map_element) => map_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MapElement`] from the store.
    ///
    #[inline]
    pub fn exorcise_map_element(&mut self, id: &usize) -> Option<Arc<RwLock<MapElement>>> {
        tracing::debug!(target: "store", "exorcising map_element slot: {id}.");
        let result = self.map_element.write().unwrap()[*id].take();
        self.map_element_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MapElement>`.
    ///
    #[inline]
    pub fn iter_map_element(&self) -> impl Iterator<Item = Arc<RwLock<MapElement>>> + '_ {
        let len = self.map_element.read().unwrap().len();
        (0..len)
            .filter(|i| self.map_element.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.map_element.read().unwrap()[i]
                    .as_ref()
                    .map(|map_element| map_element.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`MapExpression`] into the store.
    ///
    #[inline]
    pub fn inter_map_expression<F>(&mut self, map_expression: F) -> Arc<RwLock<MapExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<MapExpression>>,
    {
        let _index = if let Some(_index) = self.map_expression_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.map_expression.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.map_expression.write().unwrap().push(None);
            _index
        };

        let map_expression = map_expression(_index);

        let found = if let Some(map_expression) =
            self.map_expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *map_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            map_expression.clone()
        } else {
            None
        };

        if let Some(map_expression) = found {
            tracing::debug!(target: "store", "found duplicate {map_expression:?}.");
            self.map_expression_free_list.lock().unwrap().push(_index);
            map_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {map_expression:?}.");
            self.map_expression.write().unwrap()[_index] = Some(map_expression.clone());
            self.map_expression_dirty = true;
            map_expression
        }
    }

    /// Exhume (get) [`MapExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_map_expression(&self, id: &usize) -> Option<Arc<RwLock<MapExpression>>> {
        match self.map_expression.read().unwrap().get(*id) {
            Some(map_expression) => map_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MapExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_map_expression(&mut self, id: &usize) -> Option<Arc<RwLock<MapExpression>>> {
        tracing::debug!(target: "store", "exorcising map_expression slot: {id}.");
        let result = self.map_expression.write().unwrap()[*id].take();
        self.map_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MapExpression>`.
    ///
    #[inline]
    pub fn iter_map_expression(&self) -> impl Iterator<Item = Arc<RwLock<MapExpression>>> + '_ {
        let len = self.map_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.map_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.map_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|map_expression| map_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XMatch`] into the store.
    ///
    #[inline]
    pub fn inter_x_match<F>(&mut self, x_match: F) -> Arc<RwLock<XMatch>>
    where
        F: Fn(usize) -> Arc<RwLock<XMatch>>,
    {
        let _index = if let Some(_index) = self.x_match_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_match.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_match.write().unwrap().push(None);
            _index
        };

        let x_match = x_match(_index);

        let found = if let Some(x_match) = self.x_match.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_match.read().unwrap()
            } else {
                false
            }
        }) {
            x_match.clone()
        } else {
            None
        };

        if let Some(x_match) = found {
            tracing::debug!(target: "store", "found duplicate {x_match:?}.");
            self.x_match_free_list.lock().unwrap().push(_index);
            x_match.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_match:?}.");
            self.x_match.write().unwrap()[_index] = Some(x_match.clone());
            self.x_match_dirty = true;
            x_match
        }
    }

    /// Exhume (get) [`XMatch`] from the store.
    ///
    #[inline]
    pub fn exhume_x_match(&self, id: &usize) -> Option<Arc<RwLock<XMatch>>> {
        match self.x_match.read().unwrap().get(*id) {
            Some(x_match) => x_match.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMatch`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_match(&mut self, id: &usize) -> Option<Arc<RwLock<XMatch>>> {
        tracing::debug!(target: "store", "exorcising x_match slot: {id}.");
        let result = self.x_match.write().unwrap()[*id].take();
        self.x_match_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMatch>`.
    ///
    #[inline]
    pub fn iter_x_match(&self) -> impl Iterator<Item = Arc<RwLock<XMatch>>> + '_ {
        let len = self.x_match.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_match.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_match.read().unwrap()[i]
                    .as_ref()
                    .map(|x_match| x_match.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    #[inline]
    pub fn inter_method_call<F>(&mut self, method_call: F) -> Arc<RwLock<MethodCall>>
    where
        F: Fn(usize) -> Arc<RwLock<MethodCall>>,
    {
        let _index = if let Some(_index) = self.method_call_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.method_call.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.method_call.write().unwrap().push(None);
            _index
        };

        let method_call = method_call(_index);

        let found = if let Some(method_call) =
            self.method_call.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *method_call.read().unwrap()
                } else {
                    false
                }
            }) {
            method_call.clone()
        } else {
            None
        };

        if let Some(method_call) = found {
            tracing::debug!(target: "store", "found duplicate {method_call:?}.");
            self.method_call_free_list.lock().unwrap().push(_index);
            method_call.clone()
        } else {
            tracing::debug!(target: "store", "interring {method_call:?}.");
            self.method_call.write().unwrap()[_index] = Some(method_call.clone());
            self.method_call_dirty = true;
            method_call
        }
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    #[inline]
    pub fn exhume_method_call(&self, id: &usize) -> Option<Arc<RwLock<MethodCall>>> {
        match self.method_call.read().unwrap().get(*id) {
            Some(method_call) => method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    #[inline]
    pub fn exorcise_method_call(&mut self, id: &usize) -> Option<Arc<RwLock<MethodCall>>> {
        tracing::debug!(target: "store", "exorcising method_call slot: {id}.");
        let result = self.method_call.write().unwrap()[*id].take();
        self.method_call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    #[inline]
    pub fn iter_method_call(&self) -> impl Iterator<Item = Arc<RwLock<MethodCall>>> + '_ {
        let len = self.method_call.read().unwrap().len();
        (0..len)
            .filter(|i| self.method_call.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.method_call.read().unwrap()[i]
                    .as_ref()
                    .map(|method_call| method_call.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`NamedFieldExpression`] into the store.
    ///
    #[inline]
    pub fn inter_named_field_expression<F>(
        &mut self,
        named_field_expression: F,
    ) -> Arc<RwLock<NamedFieldExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<NamedFieldExpression>>,
    {
        let _index =
            if let Some(_index) = self.named_field_expression_free_list.lock().unwrap().pop() {
                tracing::trace!(target: "store", "recycling block {_index}.");
                _index
            } else {
                let _index = self.named_field_expression.read().unwrap().len();
                tracing::trace!(target: "store", "allocating block {_index}.");
                self.named_field_expression.write().unwrap().push(None);
                _index
            };

        let named_field_expression = named_field_expression(_index);

        let found = if let Some(named_field_expression) = self
            .named_field_expression
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *named_field_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            named_field_expression.clone()
        } else {
            None
        };

        if let Some(named_field_expression) = found {
            tracing::debug!(target: "store", "found duplicate {named_field_expression:?}.");
            self.named_field_expression_free_list
                .lock()
                .unwrap()
                .push(_index);
            named_field_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {named_field_expression:?}.");
            self.named_field_expression.write().unwrap()[_index] =
                Some(named_field_expression.clone());
            self.named_field_expression_dirty = true;
            named_field_expression
        }
    }

    /// Exhume (get) [`NamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_named_field_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<NamedFieldExpression>>> {
        match self.named_field_expression.read().unwrap().get(*id) {
            Some(named_field_expression) => named_field_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`NamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_named_field_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<NamedFieldExpression>>> {
        tracing::debug!(target: "store", "exorcising named_field_expression slot: {id}.");
        let result = self.named_field_expression.write().unwrap()[*id].take();
        self.named_field_expression_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NamedFieldExpression>`.
    ///
    #[inline]
    pub fn iter_named_field_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<NamedFieldExpression>>> + '_ {
        let len = self.named_field_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.named_field_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.named_field_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|named_field_expression| named_field_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    #[inline]
    pub fn inter_z_object_store<F>(&mut self, z_object_store: F) -> Arc<RwLock<ZObjectStore>>
    where
        F: Fn(usize) -> Arc<RwLock<ZObjectStore>>,
    {
        let _index = if let Some(_index) = self.z_object_store_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.z_object_store.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.z_object_store.write().unwrap().push(None);
            _index
        };

        let z_object_store = z_object_store(_index);

        let found = if let Some(z_object_store) =
            self.z_object_store.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *z_object_store.read().unwrap()
                } else {
                    false
                }
            }) {
            z_object_store.clone()
        } else {
            None
        };

        let z_object_store = if let Some(z_object_store) = found {
            tracing::debug!(target: "store", "found duplicate {z_object_store:?}.");
            self.z_object_store_free_list.lock().unwrap().push(_index);
            z_object_store.clone()
        } else {
            tracing::debug!(target: "store", "interring {z_object_store:?}.");
            self.z_object_store.write().unwrap()[_index] = Some(z_object_store.clone());
            self.z_object_store_dirty = true;
            z_object_store
        };
        self.z_object_store_id_by_name.write().unwrap().insert(
            z_object_store.read().unwrap().name.to_owned(),
            z_object_store.read().unwrap().id,
        );
        z_object_store
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    #[inline]
    pub fn exhume_z_object_store(&self, id: &usize) -> Option<Arc<RwLock<ZObjectStore>>> {
        match self.z_object_store.read().unwrap().get(*id) {
            Some(z_object_store) => z_object_store.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    #[inline]
    pub fn exorcise_z_object_store(&mut self, id: &usize) -> Option<Arc<RwLock<ZObjectStore>>> {
        tracing::debug!(target: "store", "exorcising z_object_store slot: {id}.");
        let result = self.z_object_store.write().unwrap()[*id].take();
        self.z_object_store_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`ZObjectStore`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_z_object_store_id_by_name(&self, name: &str) -> Option<usize> {
        self.z_object_store_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|z_object_store| *z_object_store)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    #[inline]
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Arc<RwLock<ZObjectStore>>> + '_ {
        let len = self.z_object_store.read().unwrap().len();
        (0..len)
            .filter(|i| self.z_object_store.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.z_object_store.read().unwrap()[i]
                    .as_ref()
                    .map(|z_object_store| z_object_store.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ObjectWrapper`] into the store.
    ///
    #[inline]
    pub fn inter_object_wrapper<F>(&mut self, object_wrapper: F) -> Arc<RwLock<ObjectWrapper>>
    where
        F: Fn(usize) -> Arc<RwLock<ObjectWrapper>>,
    {
        let _index = if let Some(_index) = self.object_wrapper_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.object_wrapper.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.object_wrapper.write().unwrap().push(None);
            _index
        };

        let object_wrapper = object_wrapper(_index);

        let found = if let Some(object_wrapper) =
            self.object_wrapper.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *object_wrapper.read().unwrap()
                } else {
                    false
                }
            }) {
            object_wrapper.clone()
        } else {
            None
        };

        if let Some(object_wrapper) = found {
            tracing::debug!(target: "store", "found duplicate {object_wrapper:?}.");
            self.object_wrapper_free_list.lock().unwrap().push(_index);
            object_wrapper.clone()
        } else {
            tracing::debug!(target: "store", "interring {object_wrapper:?}.");
            self.object_wrapper.write().unwrap()[_index] = Some(object_wrapper.clone());
            self.object_wrapper_dirty = true;
            object_wrapper
        }
    }

    /// Exhume (get) [`ObjectWrapper`] from the store.
    ///
    #[inline]
    pub fn exhume_object_wrapper(&self, id: &usize) -> Option<Arc<RwLock<ObjectWrapper>>> {
        match self.object_wrapper.read().unwrap().get(*id) {
            Some(object_wrapper) => object_wrapper.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ObjectWrapper`] from the store.
    ///
    #[inline]
    pub fn exorcise_object_wrapper(&mut self, id: &usize) -> Option<Arc<RwLock<ObjectWrapper>>> {
        tracing::debug!(target: "store", "exorcising object_wrapper slot: {id}.");
        let result = self.object_wrapper.write().unwrap()[*id].take();
        self.object_wrapper_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectWrapper>`.
    ///
    #[inline]
    pub fn iter_object_wrapper(&self) -> impl Iterator<Item = Arc<RwLock<ObjectWrapper>>> + '_ {
        let len = self.object_wrapper.read().unwrap().len();
        (0..len)
            .filter(|i| self.object_wrapper.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.object_wrapper.read().unwrap()[i]
                    .as_ref()
                    .map(|object_wrapper| object_wrapper.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    #[inline]
    pub fn inter_operator<F>(&mut self, operator: F) -> Arc<RwLock<Operator>>
    where
        F: Fn(usize) -> Arc<RwLock<Operator>>,
    {
        let _index = if let Some(_index) = self.operator_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.operator.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.operator.write().unwrap().push(None);
            _index
        };

        let operator = operator(_index);

        let found = if let Some(operator) = self.operator.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *operator.read().unwrap()
            } else {
                false
            }
        }) {
            operator.clone()
        } else {
            None
        };

        if let Some(operator) = found {
            tracing::debug!(target: "store", "found duplicate {operator:?}.");
            self.operator_free_list.lock().unwrap().push(_index);
            operator.clone()
        } else {
            tracing::debug!(target: "store", "interring {operator:?}.");
            self.operator.write().unwrap()[_index] = Some(operator.clone());
            self.operator_dirty = true;
            operator
        }
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    #[inline]
    pub fn exhume_operator(&self, id: &usize) -> Option<Arc<RwLock<Operator>>> {
        match self.operator.read().unwrap().get(*id) {
            Some(operator) => operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    #[inline]
    pub fn exorcise_operator(&mut self, id: &usize) -> Option<Arc<RwLock<Operator>>> {
        tracing::debug!(target: "store", "exorcising operator slot: {id}.");
        let result = self.operator.write().unwrap()[*id].take();
        self.operator_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    #[inline]
    pub fn iter_operator(&self) -> impl Iterator<Item = Arc<RwLock<Operator>>> + '_ {
        let len = self.operator.read().unwrap().len();
        (0..len)
            .filter(|i| self.operator.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.operator.read().unwrap()[i]
                    .as_ref()
                    .map(|operator| operator.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    #[inline]
    pub fn inter_parameter<F>(&mut self, parameter: F) -> Arc<RwLock<Parameter>>
    where
        F: Fn(usize) -> Arc<RwLock<Parameter>>,
    {
        let _index = if let Some(_index) = self.parameter_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.parameter.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.parameter.write().unwrap().push(None);
            _index
        };

        let parameter = parameter(_index);

        let found = if let Some(parameter) = self.parameter.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *parameter.read().unwrap()
            } else {
                false
            }
        }) {
            parameter.clone()
        } else {
            None
        };

        if let Some(parameter) = found {
            tracing::debug!(target: "store", "found duplicate {parameter:?}.");
            self.parameter_free_list.lock().unwrap().push(_index);
            parameter.clone()
        } else {
            tracing::debug!(target: "store", "interring {parameter:?}.");
            self.parameter.write().unwrap()[_index] = Some(parameter.clone());
            self.parameter_dirty = true;
            parameter
        }
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    #[inline]
    pub fn exhume_parameter(&self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        match self.parameter.read().unwrap().get(*id) {
            Some(parameter) => parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    #[inline]
    pub fn exorcise_parameter(&mut self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        tracing::debug!(target: "store", "exorcising parameter slot: {id}.");
        let result = self.parameter.write().unwrap()[*id].take();
        self.parameter_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    #[inline]
    pub fn iter_parameter(&self) -> impl Iterator<Item = Arc<RwLock<Parameter>>> + '_ {
        let len = self.parameter.read().unwrap().len();
        (0..len)
            .filter(|i| self.parameter.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.parameter.read().unwrap()[i]
                    .as_ref()
                    .map(|parameter| parameter.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XPath`] into the store.
    ///
    #[inline]
    pub fn inter_x_path<F>(&mut self, x_path: F) -> Arc<RwLock<XPath>>
    where
        F: Fn(usize) -> Arc<RwLock<XPath>>,
    {
        let _index = if let Some(_index) = self.x_path_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_path.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_path.write().unwrap().push(None);
            _index
        };

        let x_path = x_path(_index);

        let found = if let Some(x_path) = self.x_path.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_path.read().unwrap()
            } else {
                false
            }
        }) {
            x_path.clone()
        } else {
            None
        };

        if let Some(x_path) = found {
            tracing::debug!(target: "store", "found duplicate {x_path:?}.");
            self.x_path_free_list.lock().unwrap().push(_index);
            x_path.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_path:?}.");
            self.x_path.write().unwrap()[_index] = Some(x_path.clone());
            self.x_path_dirty = true;
            x_path
        }
    }

    /// Exhume (get) [`XPath`] from the store.
    ///
    #[inline]
    pub fn exhume_x_path(&self, id: &usize) -> Option<Arc<RwLock<XPath>>> {
        match self.x_path.read().unwrap().get(*id) {
            Some(x_path) => x_path.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPath`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_path(&mut self, id: &usize) -> Option<Arc<RwLock<XPath>>> {
        tracing::debug!(target: "store", "exorcising x_path slot: {id}.");
        let result = self.x_path.write().unwrap()[*id].take();
        self.x_path_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPath>`.
    ///
    #[inline]
    pub fn iter_x_path(&self) -> impl Iterator<Item = Arc<RwLock<XPath>>> + '_ {
        let len = self.x_path.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_path.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_path.read().unwrap()[i]
                    .as_ref()
                    .map(|x_path| x_path.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`PathElement`] into the store.
    ///
    #[inline]
    pub fn inter_path_element<F>(&mut self, path_element: F) -> Arc<RwLock<PathElement>>
    where
        F: Fn(usize) -> Arc<RwLock<PathElement>>,
    {
        let _index = if let Some(_index) = self.path_element_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.path_element.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.path_element.write().unwrap().push(None);
            _index
        };

        let path_element = path_element(_index);

        let found = if let Some(path_element) =
            self.path_element.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *path_element.read().unwrap()
                } else {
                    false
                }
            }) {
            path_element.clone()
        } else {
            None
        };

        if let Some(path_element) = found {
            tracing::debug!(target: "store", "found duplicate {path_element:?}.");
            self.path_element_free_list.lock().unwrap().push(_index);
            path_element.clone()
        } else {
            tracing::debug!(target: "store", "interring {path_element:?}.");
            self.path_element.write().unwrap()[_index] = Some(path_element.clone());
            self.path_element_dirty = true;
            path_element
        }
    }

    /// Exhume (get) [`PathElement`] from the store.
    ///
    #[inline]
    pub fn exhume_path_element(&self, id: &usize) -> Option<Arc<RwLock<PathElement>>> {
        match self.path_element.read().unwrap().get(*id) {
            Some(path_element) => path_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`PathElement`] from the store.
    ///
    #[inline]
    pub fn exorcise_path_element(&mut self, id: &usize) -> Option<Arc<RwLock<PathElement>>> {
        tracing::debug!(target: "store", "exorcising path_element slot: {id}.");
        let result = self.path_element.write().unwrap()[*id].take();
        self.path_element_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, PathElement>`.
    ///
    #[inline]
    pub fn iter_path_element(&self) -> impl Iterator<Item = Arc<RwLock<PathElement>>> + '_ {
        let len = self.path_element.read().unwrap().len();
        (0..len)
            .filter(|i| self.path_element.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.path_element.read().unwrap()[i]
                    .as_ref()
                    .map(|path_element| path_element.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Pattern`] into the store.
    ///
    #[inline]
    pub fn inter_pattern<F>(&mut self, pattern: F) -> Arc<RwLock<Pattern>>
    where
        F: Fn(usize) -> Arc<RwLock<Pattern>>,
    {
        let _index = if let Some(_index) = self.pattern_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.pattern.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.pattern.write().unwrap().push(None);
            _index
        };

        let pattern = pattern(_index);

        let found = if let Some(pattern) = self.pattern.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *pattern.read().unwrap()
            } else {
                false
            }
        }) {
            pattern.clone()
        } else {
            None
        };

        if let Some(pattern) = found {
            tracing::debug!(target: "store", "found duplicate {pattern:?}.");
            self.pattern_free_list.lock().unwrap().push(_index);
            pattern.clone()
        } else {
            tracing::debug!(target: "store", "interring {pattern:?}.");
            self.pattern.write().unwrap()[_index] = Some(pattern.clone());
            self.pattern_dirty = true;
            pattern
        }
    }

    /// Exhume (get) [`Pattern`] from the store.
    ///
    #[inline]
    pub fn exhume_pattern(&self, id: &usize) -> Option<Arc<RwLock<Pattern>>> {
        match self.pattern.read().unwrap().get(*id) {
            Some(pattern) => pattern.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Pattern`] from the store.
    ///
    #[inline]
    pub fn exorcise_pattern(&mut self, id: &usize) -> Option<Arc<RwLock<Pattern>>> {
        tracing::debug!(target: "store", "exorcising pattern slot: {id}.");
        let result = self.pattern.write().unwrap()[*id].take();
        self.pattern_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Pattern>`.
    ///
    #[inline]
    pub fn iter_pattern(&self) -> impl Iterator<Item = Arc<RwLock<Pattern>>> + '_ {
        let len = self.pattern.read().unwrap().len();
        (0..len)
            .filter(|i| self.pattern.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.pattern.read().unwrap()[i]
                    .as_ref()
                    .map(|pattern| pattern.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XPlugin`] into the store.
    ///
    #[inline]
    pub fn inter_x_plugin<F>(&mut self, x_plugin: F) -> Arc<RwLock<XPlugin>>
    where
        F: Fn(usize) -> Arc<RwLock<XPlugin>>,
    {
        let _index = if let Some(_index) = self.x_plugin_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_plugin.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_plugin.write().unwrap().push(None);
            _index
        };

        let x_plugin = x_plugin(_index);

        let found = if let Some(x_plugin) = self.x_plugin.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_plugin.read().unwrap()
            } else {
                false
            }
        }) {
            x_plugin.clone()
        } else {
            None
        };

        let x_plugin = if let Some(x_plugin) = found {
            tracing::debug!(target: "store", "found duplicate {x_plugin:?}.");
            self.x_plugin_free_list.lock().unwrap().push(_index);
            x_plugin.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_plugin:?}.");
            self.x_plugin.write().unwrap()[_index] = Some(x_plugin.clone());
            self.x_plugin_dirty = true;
            x_plugin
        };
        self.x_plugin_id_by_name.write().unwrap().insert(
            x_plugin.read().unwrap().name.to_owned(),
            x_plugin.read().unwrap().id,
        );
        x_plugin
    }

    /// Exhume (get) [`XPlugin`] from the store.
    ///
    #[inline]
    pub fn exhume_x_plugin(&self, id: &usize) -> Option<Arc<RwLock<XPlugin>>> {
        match self.x_plugin.read().unwrap().get(*id) {
            Some(x_plugin) => x_plugin.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPlugin`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_plugin(&mut self, id: &usize) -> Option<Arc<RwLock<XPlugin>>> {
        tracing::debug!(target: "store", "exorcising x_plugin slot: {id}.");
        let result = self.x_plugin.write().unwrap()[*id].take();
        self.x_plugin_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`XPlugin`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_x_plugin_id_by_name(&self, name: &str) -> Option<usize> {
        self.x_plugin_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|x_plugin| *x_plugin)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPlugin>`.
    ///
    #[inline]
    pub fn iter_x_plugin(&self) -> impl Iterator<Item = Arc<RwLock<XPlugin>>> + '_ {
        let len = self.x_plugin.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_plugin.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_plugin.read().unwrap()[i]
                    .as_ref()
                    .map(|x_plugin| x_plugin.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XPrint`] into the store.
    ///
    #[inline]
    pub fn inter_x_print<F>(&mut self, x_print: F) -> Arc<RwLock<XPrint>>
    where
        F: Fn(usize) -> Arc<RwLock<XPrint>>,
    {
        let _index = if let Some(_index) = self.x_print_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_print.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_print.write().unwrap().push(None);
            _index
        };

        let x_print = x_print(_index);

        let found = if let Some(x_print) = self.x_print.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_print.read().unwrap()
            } else {
                false
            }
        }) {
            x_print.clone()
        } else {
            None
        };

        if let Some(x_print) = found {
            tracing::debug!(target: "store", "found duplicate {x_print:?}.");
            self.x_print_free_list.lock().unwrap().push(_index);
            x_print.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_print:?}.");
            self.x_print.write().unwrap()[_index] = Some(x_print.clone());
            self.x_print_dirty = true;
            x_print
        }
    }

    /// Exhume (get) [`XPrint`] from the store.
    ///
    #[inline]
    pub fn exhume_x_print(&self, id: &usize) -> Option<Arc<RwLock<XPrint>>> {
        match self.x_print.read().unwrap().get(*id) {
            Some(x_print) => x_print.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPrint`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_print(&mut self, id: &usize) -> Option<Arc<RwLock<XPrint>>> {
        tracing::debug!(target: "store", "exorcising x_print slot: {id}.");
        let result = self.x_print.write().unwrap()[*id].take();
        self.x_print_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPrint>`.
    ///
    #[inline]
    pub fn iter_x_print(&self) -> impl Iterator<Item = Arc<RwLock<XPrint>>> + '_ {
        let len = self.x_print.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_print.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_print.read().unwrap()[i]
                    .as_ref()
                    .map(|x_print| x_print.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    #[inline]
    pub fn inter_range_expression<F>(&mut self, range_expression: F) -> Arc<RwLock<RangeExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<RangeExpression>>,
    {
        let _index = if let Some(_index) = self.range_expression_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.range_expression.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.range_expression.write().unwrap().push(None);
            _index
        };

        let range_expression = range_expression(_index);

        let found = if let Some(range_expression) =
            self.range_expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *range_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            range_expression.clone()
        } else {
            None
        };

        if let Some(range_expression) = found {
            tracing::debug!(target: "store", "found duplicate {range_expression:?}.");
            self.range_expression_free_list.lock().unwrap().push(_index);
            range_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {range_expression:?}.");
            self.range_expression.write().unwrap()[_index] = Some(range_expression.clone());
            self.range_expression_dirty = true;
            range_expression
        }
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_range_expression(&self, id: &usize) -> Option<Arc<RwLock<RangeExpression>>> {
        match self.range_expression.read().unwrap().get(*id) {
            Some(range_expression) => range_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_range_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<RangeExpression>>> {
        tracing::debug!(target: "store", "exorcising range_expression slot: {id}.");
        let result = self.range_expression.write().unwrap()[*id].take();
        self.range_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    #[inline]
    pub fn iter_range_expression(&self) -> impl Iterator<Item = Arc<RwLock<RangeExpression>>> + '_ {
        let len = self.range_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.range_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.range_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|range_expression| range_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    #[inline]
    pub fn inter_result_statement<F>(&mut self, result_statement: F) -> Arc<RwLock<ResultStatement>>
    where
        F: Fn(usize) -> Arc<RwLock<ResultStatement>>,
    {
        let _index = if let Some(_index) = self.result_statement_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.result_statement.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.result_statement.write().unwrap().push(None);
            _index
        };

        let result_statement = result_statement(_index);

        let found = if let Some(result_statement) =
            self.result_statement.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *result_statement.read().unwrap()
                } else {
                    false
                }
            }) {
            result_statement.clone()
        } else {
            None
        };

        if let Some(result_statement) = found {
            tracing::debug!(target: "store", "found duplicate {result_statement:?}.");
            self.result_statement_free_list.lock().unwrap().push(_index);
            result_statement.clone()
        } else {
            tracing::debug!(target: "store", "interring {result_statement:?}.");
            self.result_statement.write().unwrap()[_index] = Some(result_statement.clone());
            self.result_statement_dirty = true;
            result_statement
        }
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    #[inline]
    pub fn exhume_result_statement(&self, id: &usize) -> Option<Arc<RwLock<ResultStatement>>> {
        match self.result_statement.read().unwrap().get(*id) {
            Some(result_statement) => result_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    #[inline]
    pub fn exorcise_result_statement(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ResultStatement>>> {
        tracing::debug!(target: "store", "exorcising result_statement slot: {id}.");
        let result = self.result_statement.write().unwrap()[*id].take();
        self.result_statement_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    #[inline]
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Arc<RwLock<ResultStatement>>> + '_ {
        let len = self.result_statement.read().unwrap().len();
        (0..len)
            .filter(|i| self.result_statement.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.result_statement.read().unwrap()[i]
                    .as_ref()
                    .map(|result_statement| result_statement.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    #[inline]
    pub fn inter_x_return<F>(&mut self, x_return: F) -> Arc<RwLock<XReturn>>
    where
        F: Fn(usize) -> Arc<RwLock<XReturn>>,
    {
        let _index = if let Some(_index) = self.x_return_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_return.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_return.write().unwrap().push(None);
            _index
        };

        let x_return = x_return(_index);

        let found = if let Some(x_return) = self.x_return.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_return.read().unwrap()
            } else {
                false
            }
        }) {
            x_return.clone()
        } else {
            None
        };

        if let Some(x_return) = found {
            tracing::debug!(target: "store", "found duplicate {x_return:?}.");
            self.x_return_free_list.lock().unwrap().push(_index);
            x_return.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_return:?}.");
            self.x_return.write().unwrap()[_index] = Some(x_return.clone());
            self.x_return_dirty = true;
            x_return
        }
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    #[inline]
    pub fn exhume_x_return(&self, id: &usize) -> Option<Arc<RwLock<XReturn>>> {
        match self.x_return.read().unwrap().get(*id) {
            Some(x_return) => x_return.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_return(&mut self, id: &usize) -> Option<Arc<RwLock<XReturn>>> {
        tracing::debug!(target: "store", "exorcising x_return slot: {id}.");
        let result = self.x_return.write().unwrap()[*id].take();
        self.x_return_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    #[inline]
    pub fn iter_x_return(&self) -> impl Iterator<Item = Arc<RwLock<XReturn>>> + '_ {
        let len = self.x_return.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_return.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_return.read().unwrap()[i]
                    .as_ref()
                    .map(|x_return| x_return.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    #[inline]
    pub fn inter_span<F>(&mut self, span: F) -> Arc<RwLock<Span>>
    where
        F: Fn(usize) -> Arc<RwLock<Span>>,
    {
        let _index = if let Some(_index) = self.span_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.span.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.span.write().unwrap().push(None);
            _index
        };

        let span = span(_index);

        let found = if let Some(span) = self.span.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *span.read().unwrap()
            } else {
                false
            }
        }) {
            span.clone()
        } else {
            None
        };

        if let Some(span) = found {
            tracing::debug!(target: "store", "found duplicate {span:?}.");
            self.span_free_list.lock().unwrap().push(_index);
            span.clone()
        } else {
            tracing::debug!(target: "store", "interring {span:?}.");
            self.span.write().unwrap()[_index] = Some(span.clone());
            self.span_dirty = true;
            span
        }
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    #[inline]
    pub fn exhume_span(&self, id: &usize) -> Option<Arc<RwLock<Span>>> {
        match self.span.read().unwrap().get(*id) {
            Some(span) => span.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    #[inline]
    pub fn exorcise_span(&mut self, id: &usize) -> Option<Arc<RwLock<Span>>> {
        tracing::debug!(target: "store", "exorcising span slot: {id}.");
        let result = self.span.write().unwrap()[*id].take();
        self.span_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    #[inline]
    pub fn iter_span(&self) -> impl Iterator<Item = Arc<RwLock<Span>>> + '_ {
        let len = self.span.read().unwrap().len();
        (0..len)
            .filter(|i| self.span.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.span.read().unwrap()[i]
                    .as_ref()
                    .map(|span| span.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    #[inline]
    pub fn inter_statement<F>(&mut self, statement: F) -> Arc<RwLock<Statement>>
    where
        F: Fn(usize) -> Arc<RwLock<Statement>>,
    {
        let _index = if let Some(_index) = self.statement_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.statement.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.statement.write().unwrap().push(None);
            _index
        };

        let statement = statement(_index);

        let found = if let Some(statement) = self.statement.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *statement.read().unwrap()
            } else {
                false
            }
        }) {
            statement.clone()
        } else {
            None
        };

        if let Some(statement) = found {
            tracing::debug!(target: "store", "found duplicate {statement:?}.");
            self.statement_free_list.lock().unwrap().push(_index);
            statement.clone()
        } else {
            tracing::debug!(target: "store", "interring {statement:?}.");
            self.statement.write().unwrap()[_index] = Some(statement.clone());
            self.statement_dirty = true;
            statement
        }
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    #[inline]
    pub fn exhume_statement(&self, id: &usize) -> Option<Arc<RwLock<Statement>>> {
        match self.statement.read().unwrap().get(*id) {
            Some(statement) => statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    #[inline]
    pub fn exorcise_statement(&mut self, id: &usize) -> Option<Arc<RwLock<Statement>>> {
        tracing::debug!(target: "store", "exorcising statement slot: {id}.");
        let result = self.statement.write().unwrap()[*id].take();
        self.statement_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    #[inline]
    pub fn iter_statement(&self) -> impl Iterator<Item = Arc<RwLock<Statement>>> + '_ {
        let len = self.statement.read().unwrap().len();
        (0..len)
            .filter(|i| self.statement.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.statement.read().unwrap()[i]
                    .as_ref()
                    .map(|statement| statement.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    #[inline]
    pub fn inter_static_method_call<F>(
        &mut self,
        static_method_call: F,
    ) -> Arc<RwLock<StaticMethodCall>>
    where
        F: Fn(usize) -> Arc<RwLock<StaticMethodCall>>,
    {
        let _index = if let Some(_index) = self.static_method_call_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.static_method_call.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.static_method_call.write().unwrap().push(None);
            _index
        };

        let static_method_call = static_method_call(_index);

        let found = if let Some(static_method_call) = self
            .static_method_call
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *static_method_call.read().unwrap()
                } else {
                    false
                }
            }) {
            static_method_call.clone()
        } else {
            None
        };

        if let Some(static_method_call) = found {
            tracing::debug!(target: "store", "found duplicate {static_method_call:?}.");
            self.static_method_call_free_list
                .lock()
                .unwrap()
                .push(_index);
            static_method_call.clone()
        } else {
            tracing::debug!(target: "store", "interring {static_method_call:?}.");
            self.static_method_call.write().unwrap()[_index] = Some(static_method_call.clone());
            self.static_method_call_dirty = true;
            static_method_call
        }
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    #[inline]
    pub fn exhume_static_method_call(&self, id: &usize) -> Option<Arc<RwLock<StaticMethodCall>>> {
        match self.static_method_call.read().unwrap().get(*id) {
            Some(static_method_call) => static_method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    #[inline]
    pub fn exorcise_static_method_call(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<StaticMethodCall>>> {
        tracing::debug!(target: "store", "exorcising static_method_call slot: {id}.");
        let result = self.static_method_call.write().unwrap()[*id].take();
        self.static_method_call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    #[inline]
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StaticMethodCall>>> + '_ {
        let len = self.static_method_call.read().unwrap().len();
        (0..len)
            .filter(|i| self.static_method_call.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.static_method_call.read().unwrap()[i]
                    .as_ref()
                    .map(|static_method_call| static_method_call.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StringBit`] into the store.
    ///
    #[inline]
    pub fn inter_string_bit<F>(&mut self, string_bit: F) -> Arc<RwLock<StringBit>>
    where
        F: Fn(usize) -> Arc<RwLock<StringBit>>,
    {
        let _index = if let Some(_index) = self.string_bit_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.string_bit.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.string_bit.write().unwrap().push(None);
            _index
        };

        let string_bit = string_bit(_index);

        let found = if let Some(string_bit) =
            self.string_bit.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *string_bit.read().unwrap()
                } else {
                    false
                }
            }) {
            string_bit.clone()
        } else {
            None
        };

        if let Some(string_bit) = found {
            tracing::debug!(target: "store", "found duplicate {string_bit:?}.");
            self.string_bit_free_list.lock().unwrap().push(_index);
            string_bit.clone()
        } else {
            tracing::debug!(target: "store", "interring {string_bit:?}.");
            self.string_bit.write().unwrap()[_index] = Some(string_bit.clone());
            self.string_bit_dirty = true;
            string_bit
        }
    }

    /// Exhume (get) [`StringBit`] from the store.
    ///
    #[inline]
    pub fn exhume_string_bit(&self, id: &usize) -> Option<Arc<RwLock<StringBit>>> {
        match self.string_bit.read().unwrap().get(*id) {
            Some(string_bit) => string_bit.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StringBit`] from the store.
    ///
    #[inline]
    pub fn exorcise_string_bit(&mut self, id: &usize) -> Option<Arc<RwLock<StringBit>>> {
        tracing::debug!(target: "store", "exorcising string_bit slot: {id}.");
        let result = self.string_bit.write().unwrap()[*id].take();
        self.string_bit_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringBit>`.
    ///
    #[inline]
    pub fn iter_string_bit(&self) -> impl Iterator<Item = Arc<RwLock<StringBit>>> + '_ {
        let len = self.string_bit.read().unwrap().len();
        (0..len)
            .filter(|i| self.string_bit.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.string_bit.read().unwrap()[i]
                    .as_ref()
                    .map(|string_bit| string_bit.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_string_literal<F>(&mut self, string_literal: F) -> Arc<RwLock<StringLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<StringLiteral>>,
    {
        let _index = if let Some(_index) = self.string_literal_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.string_literal.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.string_literal.write().unwrap().push(None);
            _index
        };

        let string_literal = string_literal(_index);

        let found = if let Some(string_literal) =
            self.string_literal.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *string_literal.read().unwrap()
                } else {
                    false
                }
            }) {
            string_literal.clone()
        } else {
            None
        };

        if let Some(string_literal) = found {
            tracing::debug!(target: "store", "found duplicate {string_literal:?}.");
            self.string_literal_free_list.lock().unwrap().push(_index);
            string_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {string_literal:?}.");
            self.string_literal.write().unwrap()[_index] = Some(string_literal.clone());
            self.string_literal_dirty = true;
            string_literal
        }
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_string_literal(&self, id: &usize) -> Option<Arc<RwLock<StringLiteral>>> {
        match self.string_literal.read().unwrap().get(*id) {
            Some(string_literal) => string_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_string_literal(&mut self, id: &usize) -> Option<Arc<RwLock<StringLiteral>>> {
        tracing::debug!(target: "store", "exorcising string_literal slot: {id}.");
        let result = self.string_literal.write().unwrap()[*id].take();
        self.string_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    #[inline]
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Arc<RwLock<StringLiteral>>> + '_ {
        let len = self.string_literal.read().unwrap().len();
        (0..len)
            .filter(|i| self.string_literal.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.string_literal.read().unwrap()[i]
                    .as_ref()
                    .map(|string_literal| string_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    #[inline]
    pub fn inter_woog_struct<F>(&mut self, woog_struct: F) -> Arc<RwLock<WoogStruct>>
    where
        F: Fn(usize) -> Arc<RwLock<WoogStruct>>,
    {
        let _index = if let Some(_index) = self.woog_struct_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.woog_struct.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.woog_struct.write().unwrap().push(None);
            _index
        };

        let woog_struct = woog_struct(_index);

        let found = if let Some(woog_struct) =
            self.woog_struct.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *woog_struct.read().unwrap()
                } else {
                    false
                }
            }) {
            woog_struct.clone()
        } else {
            None
        };

        let woog_struct = if let Some(woog_struct) = found {
            tracing::debug!(target: "store", "found duplicate {woog_struct:?}.");
            self.woog_struct_free_list.lock().unwrap().push(_index);
            woog_struct.clone()
        } else {
            tracing::debug!(target: "store", "interring {woog_struct:?}.");
            self.woog_struct.write().unwrap()[_index] = Some(woog_struct.clone());
            self.woog_struct_dirty = true;
            woog_struct
        };
        self.woog_struct_id_by_name.write().unwrap().insert(
            woog_struct.read().unwrap().name.to_owned(),
            woog_struct.read().unwrap().id,
        );
        woog_struct
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    #[inline]
    pub fn exhume_woog_struct(&self, id: &usize) -> Option<Arc<RwLock<WoogStruct>>> {
        match self.woog_struct.read().unwrap().get(*id) {
            Some(woog_struct) => woog_struct.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    #[inline]
    pub fn exorcise_woog_struct(&mut self, id: &usize) -> Option<Arc<RwLock<WoogStruct>>> {
        tracing::debug!(target: "store", "exorcising woog_struct slot: {id}.");
        let result = self.woog_struct.write().unwrap()[*id].take();
        self.woog_struct_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`WoogStruct`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<usize> {
        self.woog_struct_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|woog_struct| *woog_struct)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    #[inline]
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Arc<RwLock<WoogStruct>>> + '_ {
        let len = self.woog_struct.read().unwrap().len();
        (0..len)
            .filter(|i| self.woog_struct.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.woog_struct.read().unwrap()[i]
                    .as_ref()
                    .map(|woog_struct| woog_struct.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    #[inline]
    pub fn inter_struct_expression<F>(
        &mut self,
        struct_expression: F,
    ) -> Arc<RwLock<StructExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<StructExpression>>,
    {
        let _index = if let Some(_index) = self.struct_expression_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_expression.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.struct_expression.write().unwrap().push(None);
            _index
        };

        let struct_expression = struct_expression(_index);

        let found = if let Some(struct_expression) = self
            .struct_expression
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *struct_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            struct_expression.clone()
        } else {
            None
        };

        if let Some(struct_expression) = found {
            tracing::debug!(target: "store", "found duplicate {struct_expression:?}.");
            self.struct_expression_free_list
                .lock()
                .unwrap()
                .push(_index);
            struct_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {struct_expression:?}.");
            self.struct_expression.write().unwrap()[_index] = Some(struct_expression.clone());
            self.struct_expression_dirty = true;
            struct_expression
        }
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_struct_expression(&self, id: &usize) -> Option<Arc<RwLock<StructExpression>>> {
        match self.struct_expression.read().unwrap().get(*id) {
            Some(struct_expression) => struct_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_struct_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<StructExpression>>> {
        tracing::debug!(target: "store", "exorcising struct_expression slot: {id}.");
        let result = self.struct_expression.write().unwrap()[*id].take();
        self.struct_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    #[inline]
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StructExpression>>> + '_ {
        let len = self.struct_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.struct_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.struct_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|struct_expression| struct_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StructField`] into the store.
    ///
    #[inline]
    pub fn inter_struct_field<F>(&mut self, struct_field: F) -> Arc<RwLock<StructField>>
    where
        F: Fn(usize) -> Arc<RwLock<StructField>>,
    {
        let _index = if let Some(_index) = self.struct_field_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_field.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.struct_field.write().unwrap().push(None);
            _index
        };

        let struct_field = struct_field(_index);

        let found = if let Some(struct_field) =
            self.struct_field.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *struct_field.read().unwrap()
                } else {
                    false
                }
            }) {
            struct_field.clone()
        } else {
            None
        };

        if let Some(struct_field) = found {
            tracing::debug!(target: "store", "found duplicate {struct_field:?}.");
            self.struct_field_free_list.lock().unwrap().push(_index);
            struct_field.clone()
        } else {
            tracing::debug!(target: "store", "interring {struct_field:?}.");
            self.struct_field.write().unwrap()[_index] = Some(struct_field.clone());
            self.struct_field_dirty = true;
            struct_field
        }
    }

    /// Exhume (get) [`StructField`] from the store.
    ///
    #[inline]
    pub fn exhume_struct_field(&self, id: &usize) -> Option<Arc<RwLock<StructField>>> {
        match self.struct_field.read().unwrap().get(*id) {
            Some(struct_field) => struct_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructField`] from the store.
    ///
    #[inline]
    pub fn exorcise_struct_field(&mut self, id: &usize) -> Option<Arc<RwLock<StructField>>> {
        tracing::debug!(target: "store", "exorcising struct_field slot: {id}.");
        let result = self.struct_field.write().unwrap()[*id].take();
        self.struct_field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructField>`.
    ///
    #[inline]
    pub fn iter_struct_field(&self) -> impl Iterator<Item = Arc<RwLock<StructField>>> + '_ {
        let len = self.struct_field.read().unwrap().len();
        (0..len)
            .filter(|i| self.struct_field.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.struct_field.read().unwrap()[i]
                    .as_ref()
                    .map(|struct_field| struct_field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StructGeneric`] into the store.
    ///
    #[inline]
    pub fn inter_struct_generic<F>(&mut self, struct_generic: F) -> Arc<RwLock<StructGeneric>>
    where
        F: Fn(usize) -> Arc<RwLock<StructGeneric>>,
    {
        let _index = if let Some(_index) = self.struct_generic_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_generic.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.struct_generic.write().unwrap().push(None);
            _index
        };

        let struct_generic = struct_generic(_index);

        let found = if let Some(struct_generic) =
            self.struct_generic.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *struct_generic.read().unwrap()
                } else {
                    false
                }
            }) {
            struct_generic.clone()
        } else {
            None
        };

        if let Some(struct_generic) = found {
            tracing::debug!(target: "store", "found duplicate {struct_generic:?}.");
            self.struct_generic_free_list.lock().unwrap().push(_index);
            struct_generic.clone()
        } else {
            tracing::debug!(target: "store", "interring {struct_generic:?}.");
            self.struct_generic.write().unwrap()[_index] = Some(struct_generic.clone());
            self.struct_generic_dirty = true;
            struct_generic
        }
    }

    /// Exhume (get) [`StructGeneric`] from the store.
    ///
    #[inline]
    pub fn exhume_struct_generic(&self, id: &usize) -> Option<Arc<RwLock<StructGeneric>>> {
        match self.struct_generic.read().unwrap().get(*id) {
            Some(struct_generic) => struct_generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructGeneric`] from the store.
    ///
    #[inline]
    pub fn exorcise_struct_generic(&mut self, id: &usize) -> Option<Arc<RwLock<StructGeneric>>> {
        tracing::debug!(target: "store", "exorcising struct_generic slot: {id}.");
        let result = self.struct_generic.write().unwrap()[*id].take();
        self.struct_generic_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructGeneric>`.
    ///
    #[inline]
    pub fn iter_struct_generic(&self) -> impl Iterator<Item = Arc<RwLock<StructGeneric>>> + '_ {
        let len = self.struct_generic.read().unwrap().len();
        (0..len)
            .filter(|i| self.struct_generic.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.struct_generic.read().unwrap()[i]
                    .as_ref()
                    .map(|struct_generic| struct_generic.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`TupleField`] into the store.
    ///
    #[inline]
    pub fn inter_tuple_field<F>(&mut self, tuple_field: F) -> Arc<RwLock<TupleField>>
    where
        F: Fn(usize) -> Arc<RwLock<TupleField>>,
    {
        let _index = if let Some(_index) = self.tuple_field_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.tuple_field.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.tuple_field.write().unwrap().push(None);
            _index
        };

        let tuple_field = tuple_field(_index);

        let found = if let Some(tuple_field) =
            self.tuple_field.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *tuple_field.read().unwrap()
                } else {
                    false
                }
            }) {
            tuple_field.clone()
        } else {
            None
        };

        if let Some(tuple_field) = found {
            tracing::debug!(target: "store", "found duplicate {tuple_field:?}.");
            self.tuple_field_free_list.lock().unwrap().push(_index);
            tuple_field.clone()
        } else {
            tracing::debug!(target: "store", "interring {tuple_field:?}.");
            self.tuple_field.write().unwrap()[_index] = Some(tuple_field.clone());
            self.tuple_field_dirty = true;
            tuple_field
        }
    }

    /// Exhume (get) [`TupleField`] from the store.
    ///
    #[inline]
    pub fn exhume_tuple_field(&self, id: &usize) -> Option<Arc<RwLock<TupleField>>> {
        match self.tuple_field.read().unwrap().get(*id) {
            Some(tuple_field) => tuple_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TupleField`] from the store.
    ///
    #[inline]
    pub fn exorcise_tuple_field(&mut self, id: &usize) -> Option<Arc<RwLock<TupleField>>> {
        tracing::debug!(target: "store", "exorcising tuple_field slot: {id}.");
        let result = self.tuple_field.write().unwrap()[*id].take();
        self.tuple_field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TupleField>`.
    ///
    #[inline]
    pub fn iter_tuple_field(&self) -> impl Iterator<Item = Arc<RwLock<TupleField>>> + '_ {
        let len = self.tuple_field.read().unwrap().len();
        (0..len)
            .filter(|i| self.tuple_field.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.tuple_field.read().unwrap()[i]
                    .as_ref()
                    .map(|tuple_field| tuple_field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    #[inline]
    pub fn inter_type_cast<F>(&mut self, type_cast: F) -> Arc<RwLock<TypeCast>>
    where
        F: Fn(usize) -> Arc<RwLock<TypeCast>>,
    {
        let _index = if let Some(_index) = self.type_cast_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.type_cast.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.type_cast.write().unwrap().push(None);
            _index
        };

        let type_cast = type_cast(_index);

        let found = if let Some(type_cast) = self.type_cast.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *type_cast.read().unwrap()
            } else {
                false
            }
        }) {
            type_cast.clone()
        } else {
            None
        };

        if let Some(type_cast) = found {
            tracing::debug!(target: "store", "found duplicate {type_cast:?}.");
            self.type_cast_free_list.lock().unwrap().push(_index);
            type_cast.clone()
        } else {
            tracing::debug!(target: "store", "interring {type_cast:?}.");
            self.type_cast.write().unwrap()[_index] = Some(type_cast.clone());
            self.type_cast_dirty = true;
            type_cast
        }
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    #[inline]
    pub fn exhume_type_cast(&self, id: &usize) -> Option<Arc<RwLock<TypeCast>>> {
        match self.type_cast.read().unwrap().get(*id) {
            Some(type_cast) => type_cast.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    #[inline]
    pub fn exorcise_type_cast(&mut self, id: &usize) -> Option<Arc<RwLock<TypeCast>>> {
        tracing::debug!(target: "store", "exorcising type_cast slot: {id}.");
        let result = self.type_cast.write().unwrap()[*id].take();
        self.type_cast_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    #[inline]
    pub fn iter_type_cast(&self) -> impl Iterator<Item = Arc<RwLock<TypeCast>>> + '_ {
        let len = self.type_cast.read().unwrap().len();
        (0..len)
            .filter(|i| self.type_cast.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.type_cast.read().unwrap()[i]
                    .as_ref()
                    .map(|type_cast| type_cast.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    #[inline]
    pub fn inter_unary<F>(&mut self, unary: F) -> Arc<RwLock<Unary>>
    where
        F: Fn(usize) -> Arc<RwLock<Unary>>,
    {
        let _index = if let Some(_index) = self.unary_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unary.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.unary.write().unwrap().push(None);
            _index
        };

        let unary = unary(_index);

        let found = if let Some(unary) = self.unary.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *unary.read().unwrap()
            } else {
                false
            }
        }) {
            unary.clone()
        } else {
            None
        };

        if let Some(unary) = found {
            tracing::debug!(target: "store", "found duplicate {unary:?}.");
            self.unary_free_list.lock().unwrap().push(_index);
            unary.clone()
        } else {
            tracing::debug!(target: "store", "interring {unary:?}.");
            self.unary.write().unwrap()[_index] = Some(unary.clone());
            self.unary_dirty = true;
            unary
        }
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    #[inline]
    pub fn exhume_unary(&self, id: &usize) -> Option<Arc<RwLock<Unary>>> {
        match self.unary.read().unwrap().get(*id) {
            Some(unary) => unary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    #[inline]
    pub fn exorcise_unary(&mut self, id: &usize) -> Option<Arc<RwLock<Unary>>> {
        tracing::debug!(target: "store", "exorcising unary slot: {id}.");
        let result = self.unary.write().unwrap()[*id].take();
        self.unary_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    #[inline]
    pub fn iter_unary(&self) -> impl Iterator<Item = Arc<RwLock<Unary>>> + '_ {
        let len = self.unary.read().unwrap().len();
        (0..len)
            .filter(|i| self.unary.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.unary.read().unwrap()[i]
                    .as_ref()
                    .map(|unary| unary.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Unit`] into the store.
    ///
    #[inline]
    pub fn inter_unit<F>(&mut self, unit: F) -> Arc<RwLock<Unit>>
    where
        F: Fn(usize) -> Arc<RwLock<Unit>>,
    {
        let _index = if let Some(_index) = self.unit_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unit.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.unit.write().unwrap().push(None);
            _index
        };

        let unit = unit(_index);

        let found = if let Some(unit) = self.unit.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *unit.read().unwrap()
            } else {
                false
            }
        }) {
            unit.clone()
        } else {
            None
        };

        if let Some(unit) = found {
            tracing::debug!(target: "store", "found duplicate {unit:?}.");
            self.unit_free_list.lock().unwrap().push(_index);
            unit.clone()
        } else {
            tracing::debug!(target: "store", "interring {unit:?}.");
            self.unit.write().unwrap()[_index] = Some(unit.clone());
            self.unit_dirty = true;
            unit
        }
    }

    /// Exhume (get) [`Unit`] from the store.
    ///
    #[inline]
    pub fn exhume_unit(&self, id: &usize) -> Option<Arc<RwLock<Unit>>> {
        match self.unit.read().unwrap().get(*id) {
            Some(unit) => unit.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Unit`] from the store.
    ///
    #[inline]
    pub fn exorcise_unit(&mut self, id: &usize) -> Option<Arc<RwLock<Unit>>> {
        tracing::debug!(target: "store", "exorcising unit slot: {id}.");
        let result = self.unit.write().unwrap()[*id].take();
        self.unit_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unit>`.
    ///
    #[inline]
    pub fn iter_unit(&self) -> impl Iterator<Item = Arc<RwLock<Unit>>> + '_ {
        let len = self.unit.read().unwrap().len();
        (0..len)
            .filter(|i| self.unit.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.unit.read().unwrap()[i]
                    .as_ref()
                    .map(|unit| unit.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`UnnamedFieldExpression`] into the store.
    ///
    #[inline]
    pub fn inter_unnamed_field_expression<F>(
        &mut self,
        unnamed_field_expression: F,
    ) -> Arc<RwLock<UnnamedFieldExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<UnnamedFieldExpression>>,
    {
        let _index = if let Some(_index) = self
            .unnamed_field_expression_free_list
            .lock()
            .unwrap()
            .pop()
        {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unnamed_field_expression.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.unnamed_field_expression.write().unwrap().push(None);
            _index
        };

        let unnamed_field_expression = unnamed_field_expression(_index);

        let found = if let Some(unnamed_field_expression) = self
            .unnamed_field_expression
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *unnamed_field_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            unnamed_field_expression.clone()
        } else {
            None
        };

        if let Some(unnamed_field_expression) = found {
            tracing::debug!(target: "store", "found duplicate {unnamed_field_expression:?}.");
            self.unnamed_field_expression_free_list
                .lock()
                .unwrap()
                .push(_index);
            unnamed_field_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {unnamed_field_expression:?}.");
            self.unnamed_field_expression.write().unwrap()[_index] =
                Some(unnamed_field_expression.clone());
            self.unnamed_field_expression_dirty = true;
            unnamed_field_expression
        }
    }

    /// Exhume (get) [`UnnamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_unnamed_field_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<UnnamedFieldExpression>>> {
        match self.unnamed_field_expression.read().unwrap().get(*id) {
            Some(unnamed_field_expression) => unnamed_field_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`UnnamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_unnamed_field_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<UnnamedFieldExpression>>> {
        tracing::debug!(target: "store", "exorcising unnamed_field_expression slot: {id}.");
        let result = self.unnamed_field_expression.write().unwrap()[*id].take();
        self.unnamed_field_expression_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, UnnamedFieldExpression>`.
    ///
    #[inline]
    pub fn iter_unnamed_field_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<UnnamedFieldExpression>>> + '_ {
        let len = self.unnamed_field_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.unnamed_field_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.unnamed_field_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|unnamed_field_expression| unnamed_field_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    #[inline]
    pub fn inter_x_value<F>(&mut self, x_value: F) -> Arc<RwLock<XValue>>
    where
        F: Fn(usize) -> Arc<RwLock<XValue>>,
    {
        let _index = if let Some(_index) = self.x_value_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_value.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_value.write().unwrap().push(None);
            _index
        };

        let x_value = x_value(_index);

        let found = if let Some(x_value) = self.x_value.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_value.read().unwrap()
            } else {
                false
            }
        }) {
            x_value.clone()
        } else {
            None
        };

        if let Some(x_value) = found {
            tracing::debug!(target: "store", "found duplicate {x_value:?}.");
            self.x_value_free_list.lock().unwrap().push(_index);
            x_value.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_value:?}.");
            self.x_value.write().unwrap()[_index] = Some(x_value.clone());
            self.x_value_dirty = true;
            x_value
        }
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    #[inline]
    pub fn exhume_x_value(&self, id: &usize) -> Option<Arc<RwLock<XValue>>> {
        match self.x_value.read().unwrap().get(*id) {
            Some(x_value) => x_value.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_value(&mut self, id: &usize) -> Option<Arc<RwLock<XValue>>> {
        tracing::debug!(target: "store", "exorcising x_value slot: {id}.");
        let result = self.x_value.write().unwrap()[*id].take();
        self.x_value_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    #[inline]
    pub fn iter_x_value(&self) -> impl Iterator<Item = Arc<RwLock<XValue>>> + '_ {
        let len = self.x_value.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_value.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_value.read().unwrap()[i]
                    .as_ref()
                    .map(|x_value| x_value.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    #[inline]
    pub fn inter_value_type<F>(&mut self, value_type: F) -> Arc<RwLock<ValueType>>
    where
        F: Fn(usize) -> Arc<RwLock<ValueType>>,
    {
        let _index = if let Some(_index) = self.value_type_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.value_type.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.value_type.write().unwrap().push(None);
            _index
        };

        let value_type = value_type(_index);

        let found = if let Some(value_type) =
            self.value_type.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *value_type.read().unwrap()
                } else {
                    false
                }
            }) {
            value_type.clone()
        } else {
            None
        };

        if let Some(value_type) = found {
            tracing::debug!(target: "store", "found duplicate {value_type:?}.");
            self.value_type_free_list.lock().unwrap().push(_index);
            value_type.clone()
        } else {
            tracing::debug!(target: "store", "interring {value_type:?}.");
            self.value_type.write().unwrap()[_index] = Some(value_type.clone());
            self.value_type_dirty = true;
            value_type
        }
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    #[inline]
    pub fn exhume_value_type(&self, id: &usize) -> Option<Arc<RwLock<ValueType>>> {
        match self.value_type.read().unwrap().get(*id) {
            Some(value_type) => value_type.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    #[inline]
    pub fn exorcise_value_type(&mut self, id: &usize) -> Option<Arc<RwLock<ValueType>>> {
        tracing::debug!(target: "store", "exorcising value_type slot: {id}.");
        let result = self.value_type.write().unwrap()[*id].take();
        self.value_type_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    #[inline]
    pub fn iter_value_type(&self) -> impl Iterator<Item = Arc<RwLock<ValueType>>> + '_ {
        let len = self.value_type.read().unwrap().len();
        (0..len)
            .filter(|i| self.value_type.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.value_type.read().unwrap()[i]
                    .as_ref()
                    .map(|value_type| value_type.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    #[inline]
    pub fn inter_variable<F>(&mut self, variable: F) -> Arc<RwLock<Variable>>
    where
        F: Fn(usize) -> Arc<RwLock<Variable>>,
    {
        let _index = if let Some(_index) = self.variable_free_list.lock().unwrap().pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.variable.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.variable.write().unwrap().push(None);
            _index
        };

        let variable = variable(_index);

        let found = if let Some(variable) = self.variable.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *variable.read().unwrap()
            } else {
                false
            }
        }) {
            variable.clone()
        } else {
            None
        };

        if let Some(variable) = found {
            tracing::debug!(target: "store", "found duplicate {variable:?}.");
            self.variable_free_list.lock().unwrap().push(_index);
            variable.clone()
        } else {
            tracing::debug!(target: "store", "interring {variable:?}.");
            self.variable.write().unwrap()[_index] = Some(variable.clone());
            self.variable_dirty = true;
            variable
        }
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    #[inline]
    pub fn exhume_variable(&self, id: &usize) -> Option<Arc<RwLock<Variable>>> {
        match self.variable.read().unwrap().get(*id) {
            Some(variable) => variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    #[inline]
    pub fn exorcise_variable(&mut self, id: &usize) -> Option<Arc<RwLock<Variable>>> {
        tracing::debug!(target: "store", "exorcising variable slot: {id}.");
        let result = self.variable.write().unwrap()[*id].take();
        self.variable_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    #[inline]
    pub fn iter_variable(&self) -> impl Iterator<Item = Arc<RwLock<Variable>>> + '_ {
        let len = self.variable.read().unwrap().len();
        (0..len)
            .filter(|i| self.variable.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.variable.read().unwrap()[i]
                    .as_ref()
                    .map(|variable| variable.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    #[inline]
    pub fn inter_variable_expression<F>(
        &mut self,
        variable_expression: F,
    ) -> Arc<RwLock<VariableExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<VariableExpression>>,
    {
        let _index = if let Some(_index) = self.variable_expression_free_list.lock().unwrap().pop()
        {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.variable_expression.read().unwrap().len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.variable_expression.write().unwrap().push(None);
            _index
        };

        let variable_expression = variable_expression(_index);

        let found = if let Some(variable_expression) = self
            .variable_expression
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *variable_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            variable_expression.clone()
        } else {
            None
        };

        if let Some(variable_expression) = found {
            tracing::debug!(target: "store", "found duplicate {variable_expression:?}.");
            self.variable_expression_free_list
                .lock()
                .unwrap()
                .push(_index);
            variable_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {variable_expression:?}.");
            self.variable_expression.write().unwrap()[_index] = Some(variable_expression.clone());
            self.variable_expression_dirty = true;
            variable_expression
        }
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_variable_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        match self.variable_expression.read().unwrap().get(*id) {
            Some(variable_expression) => variable_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_variable_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        tracing::debug!(target: "store", "exorcising variable_expression slot: {id}.");
        let result = self.variable_expression.write().unwrap()[*id].take();
        self.variable_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    #[inline]
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<VariableExpression>>> + '_ {
        let len = self.variable_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.variable_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.variable_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|variable_expression| variable_expression.clone())
                    .unwrap()
            })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock_vec-object-store-persistence"}}}
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
            for argument in &*self.argument.read().unwrap() {
                if let Some(argument) = argument {
                    let path = path.join(format!("{}.json", argument.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &argument)?;
                }
            }
        }

        // Persist Await.
        {
            let path = path.join("a_wait");
            fs::create_dir_all(&path)?;
            for a_wait in &*self.a_wait.read().unwrap() {
                if let Some(a_wait) = a_wait {
                    let path = path.join(format!("{}.json", a_wait.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &a_wait)?;
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary in &*self.binary.read().unwrap() {
                if let Some(binary) = binary {
                    let path = path.join(format!("{}.json", binary.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &binary)?;
                }
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block in &*self.block.read().unwrap() {
                if let Some(block) = block {
                    let path = path.join(format!("{}.json", block.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &block)?;
                }
            }
        }

        // Persist Body.
        {
            let path = path.join("body");
            fs::create_dir_all(&path)?;
            for body in &*self.body.read().unwrap() {
                if let Some(body) = body {
                    let path = path.join(format!("{}.json", body.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &body)?;
                }
            }
        }

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal in &*self.boolean_literal.read().unwrap() {
                if let Some(boolean_literal) = boolean_literal {
                    let path = path.join(format!("{}.json", boolean_literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &boolean_literal)?;
                }
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator in &*self.boolean_operator.read().unwrap() {
                if let Some(boolean_operator) = boolean_operator {
                    let path = path.join(format!("{}.json", boolean_operator.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &boolean_operator)?;
                }
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call in &*self.call.read().unwrap() {
                if let Some(call) = call {
                    let path = path.join(format!("{}.json", call.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &call)?;
                }
            }
        }

        // Persist Char Literal.
        {
            let path = path.join("char_literal");
            fs::create_dir_all(&path)?;
            for char_literal in &*self.char_literal.read().unwrap() {
                if let Some(char_literal) = char_literal {
                    let path = path.join(format!("{}.json", char_literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &char_literal)?;
                }
            }
        }

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison in &*self.comparison.read().unwrap() {
                if let Some(comparison) = comparison {
                    let path = path.join(format!("{}.json", comparison.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &comparison)?;
                }
            }
        }

        // Persist Data Structure.
        {
            let path = path.join("data_structure");
            fs::create_dir_all(&path)?;
            for data_structure in &*self.data_structure.read().unwrap() {
                if let Some(data_structure) = data_structure {
                    let path = path.join(format!("{}.json", data_structure.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &data_structure)?;
                }
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file in &*self.dwarf_source_file.read().unwrap() {
                if let Some(dwarf_source_file) = dwarf_source_file {
                    let path = path.join(format!("{}.json", dwarf_source_file.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &dwarf_source_file)?;
                }
            }
        }

        // Persist Enum Field.
        {
            let path = path.join("enum_field");
            fs::create_dir_all(&path)?;
            for enum_field in &*self.enum_field.read().unwrap() {
                if let Some(enum_field) = enum_field {
                    let path = path.join(format!("{}.json", enum_field.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enum_field)?;
                }
            }
        }

        // Persist Enum Generic.
        {
            let path = path.join("enum_generic");
            fs::create_dir_all(&path)?;
            for enum_generic in &*self.enum_generic.read().unwrap() {
                if let Some(enum_generic) = enum_generic {
                    let path = path.join(format!("{}.json", enum_generic.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enum_generic)?;
                }
            }
        }

        // Persist Enum Generic Type.
        {
            let path = path.join("enum_generic_type");
            fs::create_dir_all(&path)?;
            for enum_generic_type in &*self.enum_generic_type.read().unwrap() {
                if let Some(enum_generic_type) = enum_generic_type {
                    let path = path.join(format!("{}.json", enum_generic_type.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enum_generic_type)?;
                }
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration in &*self.enumeration.read().unwrap() {
                if let Some(enumeration) = enumeration {
                    let path = path.join(format!("{}.json", enumeration.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enumeration)?;
                }
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression in &*self.expression.read().unwrap() {
                if let Some(expression) = expression {
                    let path = path.join(format!("{}.json", expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &expression)?;
                }
            }
        }

        // Persist Expression Bit.
        {
            let path = path.join("expression_bit");
            fs::create_dir_all(&path)?;
            for expression_bit in &*self.expression_bit.read().unwrap() {
                if let Some(expression_bit) = expression_bit {
                    let path = path.join(format!("{}.json", expression_bit.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &expression_bit)?;
                }
            }
        }

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement in &*self.expression_statement.read().unwrap() {
                if let Some(expression_statement) = expression_statement {
                    let path =
                        path.join(format!("{}.json", expression_statement.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &expression_statement)?;
                }
            }
        }

        // Persist External Implementation.
        {
            let path = path.join("external_implementation");
            fs::create_dir_all(&path)?;
            for external_implementation in &*self.external_implementation.read().unwrap() {
                if let Some(external_implementation) = external_implementation {
                    let path = path.join(format!(
                        "{}.json",
                        external_implementation.read().unwrap().id
                    ));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &external_implementation)?;
                }
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field in &*self.field.read().unwrap() {
                if let Some(field) = field {
                    let path = path.join(format!("{}.json", field.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field)?;
                }
            }
        }

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access in &*self.field_access.read().unwrap() {
                if let Some(field_access) = field_access {
                    let path = path.join(format!("{}.json", field_access.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_access)?;
                }
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target in &*self.field_access_target.read().unwrap() {
                if let Some(field_access_target) = field_access_target {
                    let path =
                        path.join(format!("{}.json", field_access_target.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_access_target)?;
                }
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression in &*self.field_expression.read().unwrap() {
                if let Some(field_expression) = field_expression {
                    let path = path.join(format!("{}.json", field_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_expression)?;
                }
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal in &*self.float_literal.read().unwrap() {
                if let Some(float_literal) = float_literal {
                    let path = path.join(format!("{}.json", float_literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &float_literal)?;
                }
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop in &*self.for_loop.read().unwrap() {
                if let Some(for_loop) = for_loop {
                    let path = path.join(format!("{}.json", for_loop.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &for_loop)?;
                }
            }
        }

        // Persist Format Bit.
        {
            let path = path.join("format_bit");
            fs::create_dir_all(&path)?;
            for format_bit in &*self.format_bit.read().unwrap() {
                if let Some(format_bit) = format_bit {
                    let path = path.join(format!("{}.json", format_bit.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &format_bit)?;
                }
            }
        }

        // Persist Format String.
        {
            let path = path.join("format_string");
            fs::create_dir_all(&path)?;
            for format_string in &*self.format_string.read().unwrap() {
                if let Some(format_string) = format_string {
                    let path = path.join(format!("{}.json", format_string.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &format_string)?;
                }
            }
        }

        // Persist Func Generic.
        {
            let path = path.join("func_generic");
            fs::create_dir_all(&path)?;
            for func_generic in &*self.func_generic.read().unwrap() {
                if let Some(func_generic) = func_generic {
                    let path = path.join(format!("{}.json", func_generic.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &func_generic)?;
                }
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function in &*self.function.read().unwrap() {
                if let Some(function) = function {
                    let path = path.join(format!("{}.json", function.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &function)?;
                }
            }
        }

        // Persist Function Call.
        {
            let path = path.join("function_call");
            fs::create_dir_all(&path)?;
            for function_call in &*self.function_call.read().unwrap() {
                if let Some(function_call) = function_call {
                    let path = path.join(format!("{}.json", function_call.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &function_call)?;
                }
            }
        }

        // Persist Future.
        {
            let path = path.join("x_future");
            fs::create_dir_all(&path)?;
            for x_future in &*self.x_future.read().unwrap() {
                if let Some(x_future) = x_future {
                    let path = path.join(format!("{}.json", x_future.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_future)?;
                }
            }
        }

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped in &*self.grouped.read().unwrap() {
                if let Some(grouped) = grouped {
                    let path = path.join(format!("{}.json", grouped.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &grouped)?;
                }
            }
        }

        // Persist Halt and Catch Fire.
        {
            let path = path.join("halt_and_catch_fire");
            fs::create_dir_all(&path)?;
            for halt_and_catch_fire in &*self.halt_and_catch_fire.read().unwrap() {
                if let Some(halt_and_catch_fire) = halt_and_catch_fire {
                    let path =
                        path.join(format!("{}.json", halt_and_catch_fire.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &halt_and_catch_fire)?;
                }
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if in &*self.x_if.read().unwrap() {
                if let Some(x_if) = x_if {
                    let path = path.join(format!("{}.json", x_if.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_if)?;
                }
            }
        }

        // Persist Implementation Block.
        {
            let path = path.join("implementation_block");
            fs::create_dir_all(&path)?;
            for implementation_block in &*self.implementation_block.read().unwrap() {
                if let Some(implementation_block) = implementation_block {
                    let path =
                        path.join(format!("{}.json", implementation_block.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &implementation_block)?;
                }
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import in &*self.import.read().unwrap() {
                if let Some(import) = import {
                    let path = path.join(format!("{}.json", import.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &import)?;
                }
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index in &*self.index.read().unwrap() {
                if let Some(index) = index {
                    let path = path.join(format!("{}.json", index.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &index)?;
                }
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal in &*self.integer_literal.read().unwrap() {
                if let Some(integer_literal) = integer_literal {
                    let path = path.join(format!("{}.json", integer_literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &integer_literal)?;
                }
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item in &*self.item.read().unwrap() {
                if let Some(item) = item {
                    let path = path.join(format!("{}.json", item.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &item)?;
                }
            }
        }

        // Persist Lambda.
        {
            let path = path.join("lambda");
            fs::create_dir_all(&path)?;
            for lambda in &*self.lambda.read().unwrap() {
                if let Some(lambda) = lambda {
                    let path = path.join(format!("{}.json", lambda.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &lambda)?;
                }
            }
        }

        // Persist Lambda Parameter.
        {
            let path = path.join("lambda_parameter");
            fs::create_dir_all(&path)?;
            for lambda_parameter in &*self.lambda_parameter.read().unwrap() {
                if let Some(lambda_parameter) = lambda_parameter {
                    let path = path.join(format!("{}.json", lambda_parameter.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &lambda_parameter)?;
                }
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement in &*self.let_statement.read().unwrap() {
                if let Some(let_statement) = let_statement {
                    let path = path.join(format!("{}.json", let_statement.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &let_statement)?;
                }
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list in &*self.list.read().unwrap() {
                if let Some(list) = list {
                    let path = path.join(format!("{}.json", list.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &list)?;
                }
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element in &*self.list_element.read().unwrap() {
                if let Some(list_element) = list_element {
                    let path = path.join(format!("{}.json", list_element.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &list_element)?;
                }
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression in &*self.list_expression.read().unwrap() {
                if let Some(list_expression) = list_expression {
                    let path = path.join(format!("{}.json", list_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &list_expression)?;
                }
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal in &*self.literal.read().unwrap() {
                if let Some(literal) = literal {
                    let path = path.join(format!("{}.json", literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &literal)?;
                }
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable in &*self.local_variable.read().unwrap() {
                if let Some(local_variable) = local_variable {
                    let path = path.join(format!("{}.json", local_variable.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &local_variable)?;
                }
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro in &*self.x_macro.read().unwrap() {
                if let Some(x_macro) = x_macro {
                    let path = path.join(format!("{}.json", x_macro.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_macro)?;
                }
            }
        }

        // Persist Map.
        {
            let path = path.join("map");
            fs::create_dir_all(&path)?;
            for map in &*self.map.read().unwrap() {
                if let Some(map) = map {
                    let path = path.join(format!("{}.json", map.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &map)?;
                }
            }
        }

        // Persist Map Element.
        {
            let path = path.join("map_element");
            fs::create_dir_all(&path)?;
            for map_element in &*self.map_element.read().unwrap() {
                if let Some(map_element) = map_element {
                    let path = path.join(format!("{}.json", map_element.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &map_element)?;
                }
            }
        }

        // Persist Map Expression.
        {
            let path = path.join("map_expression");
            fs::create_dir_all(&path)?;
            for map_expression in &*self.map_expression.read().unwrap() {
                if let Some(map_expression) = map_expression {
                    let path = path.join(format!("{}.json", map_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &map_expression)?;
                }
            }
        }

        // Persist Match.
        {
            let path = path.join("x_match");
            fs::create_dir_all(&path)?;
            for x_match in &*self.x_match.read().unwrap() {
                if let Some(x_match) = x_match {
                    let path = path.join(format!("{}.json", x_match.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_match)?;
                }
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call in &*self.method_call.read().unwrap() {
                if let Some(method_call) = method_call {
                    let path = path.join(format!("{}.json", method_call.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &method_call)?;
                }
            }
        }

        // Persist Named Field Expression.
        {
            let path = path.join("named_field_expression");
            fs::create_dir_all(&path)?;
            for named_field_expression in &*self.named_field_expression.read().unwrap() {
                if let Some(named_field_expression) = named_field_expression {
                    let path = path.join(format!(
                        "{}.json",
                        named_field_expression.read().unwrap().id
                    ));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &named_field_expression)?;
                }
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store in &*self.z_object_store.read().unwrap() {
                if let Some(z_object_store) = z_object_store {
                    let path = path.join(format!("{}.json", z_object_store.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &z_object_store)?;
                }
            }
        }

        // Persist Object Wrapper.
        {
            let path = path.join("object_wrapper");
            fs::create_dir_all(&path)?;
            for object_wrapper in &*self.object_wrapper.read().unwrap() {
                if let Some(object_wrapper) = object_wrapper {
                    let path = path.join(format!("{}.json", object_wrapper.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object_wrapper)?;
                }
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator in &*self.operator.read().unwrap() {
                if let Some(operator) = operator {
                    let path = path.join(format!("{}.json", operator.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &operator)?;
                }
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter in &*self.parameter.read().unwrap() {
                if let Some(parameter) = parameter {
                    let path = path.join(format!("{}.json", parameter.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &parameter)?;
                }
            }
        }

        // Persist Path.
        {
            let path = path.join("x_path");
            fs::create_dir_all(&path)?;
            for x_path in &*self.x_path.read().unwrap() {
                if let Some(x_path) = x_path {
                    let path = path.join(format!("{}.json", x_path.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_path)?;
                }
            }
        }

        // Persist Path Element.
        {
            let path = path.join("path_element");
            fs::create_dir_all(&path)?;
            for path_element in &*self.path_element.read().unwrap() {
                if let Some(path_element) = path_element {
                    let path = path.join(format!("{}.json", path_element.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &path_element)?;
                }
            }
        }

        // Persist Pattern.
        {
            let path = path.join("pattern");
            fs::create_dir_all(&path)?;
            for pattern in &*self.pattern.read().unwrap() {
                if let Some(pattern) = pattern {
                    let path = path.join(format!("{}.json", pattern.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &pattern)?;
                }
            }
        }

        // Persist Plugin.
        {
            let path = path.join("x_plugin");
            fs::create_dir_all(&path)?;
            for x_plugin in &*self.x_plugin.read().unwrap() {
                if let Some(x_plugin) = x_plugin {
                    let path = path.join(format!("{}.json", x_plugin.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_plugin)?;
                }
            }
        }

        // Persist Print.
        {
            let path = path.join("x_print");
            fs::create_dir_all(&path)?;
            for x_print in &*self.x_print.read().unwrap() {
                if let Some(x_print) = x_print {
                    let path = path.join(format!("{}.json", x_print.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_print)?;
                }
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression in &*self.range_expression.read().unwrap() {
                if let Some(range_expression) = range_expression {
                    let path = path.join(format!("{}.json", range_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &range_expression)?;
                }
            }
        }

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement in &*self.result_statement.read().unwrap() {
                if let Some(result_statement) = result_statement {
                    let path = path.join(format!("{}.json", result_statement.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &result_statement)?;
                }
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return in &*self.x_return.read().unwrap() {
                if let Some(x_return) = x_return {
                    let path = path.join(format!("{}.json", x_return.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_return)?;
                }
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span in &*self.span.read().unwrap() {
                if let Some(span) = span {
                    let path = path.join(format!("{}.json", span.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &span)?;
                }
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement in &*self.statement.read().unwrap() {
                if let Some(statement) = statement {
                    let path = path.join(format!("{}.json", statement.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &statement)?;
                }
            }
        }

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call in &*self.static_method_call.read().unwrap() {
                if let Some(static_method_call) = static_method_call {
                    let path = path.join(format!("{}.json", static_method_call.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &static_method_call)?;
                }
            }
        }

        // Persist String Bit.
        {
            let path = path.join("string_bit");
            fs::create_dir_all(&path)?;
            for string_bit in &*self.string_bit.read().unwrap() {
                if let Some(string_bit) = string_bit {
                    let path = path.join(format!("{}.json", string_bit.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &string_bit)?;
                }
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal in &*self.string_literal.read().unwrap() {
                if let Some(string_literal) = string_literal {
                    let path = path.join(format!("{}.json", string_literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &string_literal)?;
                }
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct in &*self.woog_struct.read().unwrap() {
                if let Some(woog_struct) = woog_struct {
                    let path = path.join(format!("{}.json", woog_struct.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &woog_struct)?;
                }
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression in &*self.struct_expression.read().unwrap() {
                if let Some(struct_expression) = struct_expression {
                    let path = path.join(format!("{}.json", struct_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &struct_expression)?;
                }
            }
        }

        // Persist Struct Field.
        {
            let path = path.join("struct_field");
            fs::create_dir_all(&path)?;
            for struct_field in &*self.struct_field.read().unwrap() {
                if let Some(struct_field) = struct_field {
                    let path = path.join(format!("{}.json", struct_field.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &struct_field)?;
                }
            }
        }

        // Persist Struct Generic.
        {
            let path = path.join("struct_generic");
            fs::create_dir_all(&path)?;
            for struct_generic in &*self.struct_generic.read().unwrap() {
                if let Some(struct_generic) = struct_generic {
                    let path = path.join(format!("{}.json", struct_generic.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &struct_generic)?;
                }
            }
        }

        // Persist Tuple Field.
        {
            let path = path.join("tuple_field");
            fs::create_dir_all(&path)?;
            for tuple_field in &*self.tuple_field.read().unwrap() {
                if let Some(tuple_field) = tuple_field {
                    let path = path.join(format!("{}.json", tuple_field.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &tuple_field)?;
                }
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast in &*self.type_cast.read().unwrap() {
                if let Some(type_cast) = type_cast {
                    let path = path.join(format!("{}.json", type_cast.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &type_cast)?;
                }
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary in &*self.unary.read().unwrap() {
                if let Some(unary) = unary {
                    let path = path.join(format!("{}.json", unary.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &unary)?;
                }
            }
        }

        // Persist Unit.
        {
            let path = path.join("unit");
            fs::create_dir_all(&path)?;
            for unit in &*self.unit.read().unwrap() {
                if let Some(unit) = unit {
                    let path = path.join(format!("{}.json", unit.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &unit)?;
                }
            }
        }

        // Persist Unnamed Field Expression.
        {
            let path = path.join("unnamed_field_expression");
            fs::create_dir_all(&path)?;
            for unnamed_field_expression in &*self.unnamed_field_expression.read().unwrap() {
                if let Some(unnamed_field_expression) = unnamed_field_expression {
                    let path = path.join(format!(
                        "{}.json",
                        unnamed_field_expression.read().unwrap().id
                    ));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &unnamed_field_expression)?;
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value in &*self.x_value.read().unwrap() {
                if let Some(x_value) = x_value {
                    let path = path.join(format!("{}.json", x_value.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_value)?;
                }
            }
        }

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type in &*self.value_type.read().unwrap() {
                if let Some(value_type) = value_type {
                    let path = path.join(format!("{}.json", value_type.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &value_type)?;
                }
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable in &*self.variable.read().unwrap() {
                if let Some(variable) = variable {
                    let path = path.join(format!("{}.json", variable.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &variable)?;
                }
            }
        }

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression in &*self.variable_expression.read().unwrap() {
                if let Some(variable_expression) = variable_expression {
                    let path =
                        path.join(format!("{}.json", variable_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &variable_expression)?;
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
        let path = path.join("lu_dog.json");

        let mut store = Self::new();

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
                    .insert(argument.read().unwrap().id, Some(argument.clone()));
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
                    .insert(a_wait.read().unwrap().id, Some(a_wait.clone()));
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
                    .insert(binary.read().unwrap().id, Some(binary.clone()));
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
                    .insert(block.read().unwrap().id, Some(block.clone()));
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
                    .insert(body.read().unwrap().id, Some(body.clone()));
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
                store.boolean_literal.write().unwrap().insert(
                    boolean_literal.read().unwrap().id,
                    Some(boolean_literal.clone()),
                );
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
                    Some(boolean_operator.clone()),
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
                    .insert(call.read().unwrap().id, Some(call.clone()));
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
                    .insert(char_literal.read().unwrap().id, Some(char_literal.clone()));
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
                    .insert(comparison.read().unwrap().id, Some(comparison.clone()));
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
                store.data_structure.write().unwrap().insert(
                    data_structure.read().unwrap().id,
                    Some(data_structure.clone()),
                );
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
                    Some(dwarf_source_file.clone()),
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
                    .insert(enum_field.read().unwrap().id, Some(enum_field.clone()));
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
                    .insert(enum_generic.read().unwrap().id, Some(enum_generic.clone()));
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
                    Some(enum_generic_type.clone()),
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
                    enumeration.read().unwrap().name.to_owned(),
                    enumeration.read().unwrap().id,
                );
                store
                    .enumeration
                    .write()
                    .unwrap()
                    .insert(enumeration.read().unwrap().id, Some(enumeration.clone()));
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
                    .insert(expression.read().unwrap().id, Some(expression.clone()));
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
                store.expression_bit.write().unwrap().insert(
                    expression_bit.read().unwrap().id,
                    Some(expression_bit.clone()),
                );
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
                    Some(expression_statement.clone()),
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
                    Some(external_implementation.clone()),
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
                store.field_id_by_name.write().unwrap().insert(
                    field.read().unwrap().name.to_owned(),
                    field.read().unwrap().id,
                );
                store
                    .field
                    .write()
                    .unwrap()
                    .insert(field.read().unwrap().id, Some(field.clone()));
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
                    .insert(field_access.read().unwrap().id, Some(field_access.clone()));
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
                    Some(field_access_target.clone()),
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
                    Some(field_expression.clone()),
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
                store.float_literal.write().unwrap().insert(
                    float_literal.read().unwrap().id,
                    Some(float_literal.clone()),
                );
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
                    .insert(for_loop.read().unwrap().id, Some(for_loop.clone()));
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
                    .insert(format_bit.read().unwrap().id, Some(format_bit.clone()));
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
                store.format_string.write().unwrap().insert(
                    format_string.read().unwrap().id,
                    Some(format_string.clone()),
                );
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
                    .insert(func_generic.read().unwrap().id, Some(func_generic.clone()));
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
                    function.read().unwrap().name.to_owned(),
                    function.read().unwrap().id,
                );
                store
                    .function
                    .write()
                    .unwrap()
                    .insert(function.read().unwrap().id, Some(function.clone()));
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
                store.function_call.write().unwrap().insert(
                    function_call.read().unwrap().id,
                    Some(function_call.clone()),
                );
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
                    .insert(x_future.read().unwrap().id, Some(x_future.clone()));
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
                    .insert(grouped.read().unwrap().id, Some(grouped.clone()));
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
                    Some(halt_and_catch_fire.clone()),
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
                    .insert(x_if.read().unwrap().id, Some(x_if.clone()));
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
                    Some(implementation_block.clone()),
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
                    .insert(import.read().unwrap().id, Some(import.clone()));
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
                    .insert(index.read().unwrap().id, Some(index.clone()));
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
                store.integer_literal.write().unwrap().insert(
                    integer_literal.read().unwrap().id,
                    Some(integer_literal.clone()),
                );
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
                    .insert(item.read().unwrap().id, Some(item.clone()));
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
                    .insert(lambda.read().unwrap().id, Some(lambda.clone()));
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
                    Some(lambda_parameter.clone()),
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
                store.let_statement.write().unwrap().insert(
                    let_statement.read().unwrap().id,
                    Some(let_statement.clone()),
                );
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
                    .insert(list.read().unwrap().id, Some(list.clone()));
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
                    .insert(list_element.read().unwrap().id, Some(list_element.clone()));
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
                store.list_expression.write().unwrap().insert(
                    list_expression.read().unwrap().id,
                    Some(list_expression.clone()),
                );
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
                    .insert(literal.read().unwrap().id, Some(literal.clone()));
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
                store.local_variable.write().unwrap().insert(
                    local_variable.read().unwrap().id,
                    Some(local_variable.clone()),
                );
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
                    .insert(x_macro.read().unwrap().id, Some(x_macro.clone()));
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
                    .insert(map.read().unwrap().id, Some(map.clone()));
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
                    .insert(map_element.read().unwrap().id, Some(map_element.clone()));
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
                store.map_expression.write().unwrap().insert(
                    map_expression.read().unwrap().id,
                    Some(map_expression.clone()),
                );
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
                    .insert(x_match.read().unwrap().id, Some(x_match.clone()));
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
                    .insert(method_call.read().unwrap().id, Some(method_call.clone()));
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
                    Some(named_field_expression.clone()),
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
                    z_object_store.read().unwrap().name.to_owned(),
                    z_object_store.read().unwrap().id,
                );
                store.z_object_store.write().unwrap().insert(
                    z_object_store.read().unwrap().id,
                    Some(z_object_store.clone()),
                );
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
                store.object_wrapper.write().unwrap().insert(
                    object_wrapper.read().unwrap().id,
                    Some(object_wrapper.clone()),
                );
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
                    .insert(operator.read().unwrap().id, Some(operator.clone()));
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
                    .insert(parameter.read().unwrap().id, Some(parameter.clone()));
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
                    .insert(x_path.read().unwrap().id, Some(x_path.clone()));
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
                    .insert(path_element.read().unwrap().id, Some(path_element.clone()));
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
                    .insert(pattern.read().unwrap().id, Some(pattern.clone()));
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
                    x_plugin.read().unwrap().name.to_owned(),
                    x_plugin.read().unwrap().id,
                );
                store
                    .x_plugin
                    .write()
                    .unwrap()
                    .insert(x_plugin.read().unwrap().id, Some(x_plugin.clone()));
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
                    .insert(x_print.read().unwrap().id, Some(x_print.clone()));
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
                    Some(range_expression.clone()),
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
                    Some(result_statement.clone()),
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
                    .insert(x_return.read().unwrap().id, Some(x_return.clone()));
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
                    .insert(span.read().unwrap().id, Some(span.clone()));
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
                    .insert(statement.read().unwrap().id, Some(statement.clone()));
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
                    Some(static_method_call.clone()),
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
                    .insert(string_bit.read().unwrap().id, Some(string_bit.clone()));
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
                store.string_literal.write().unwrap().insert(
                    string_literal.read().unwrap().id,
                    Some(string_literal.clone()),
                );
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
                    woog_struct.read().unwrap().name.to_owned(),
                    woog_struct.read().unwrap().id,
                );
                store
                    .woog_struct
                    .write()
                    .unwrap()
                    .insert(woog_struct.read().unwrap().id, Some(woog_struct.clone()));
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
                    Some(struct_expression.clone()),
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
                    .insert(struct_field.read().unwrap().id, Some(struct_field.clone()));
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
                store.struct_generic.write().unwrap().insert(
                    struct_generic.read().unwrap().id,
                    Some(struct_generic.clone()),
                );
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
                    .insert(tuple_field.read().unwrap().id, Some(tuple_field.clone()));
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
                    .insert(type_cast.read().unwrap().id, Some(type_cast.clone()));
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
                    .insert(unary.read().unwrap().id, Some(unary.clone()));
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
                    .insert(unit.read().unwrap().id, Some(unit.clone()));
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
                    Some(unnamed_field_expression.clone()),
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
                    .insert(x_value.read().unwrap().id, Some(x_value.clone()));
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
                    .insert(value_type.read().unwrap().id, Some(value_type.clone()));
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
                    .insert(variable.read().unwrap().id, Some(variable.clone()));
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
                    Some(variable_expression.clone()),
                );
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
