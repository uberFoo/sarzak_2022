//! v2::lu_dog_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vec-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vec-object-store-definition"}}}
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::{
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
    argument_free_list: Vec<usize>,
    argument: Vec<Option<Rc<RefCell<Argument>>>>,
    argument_dirty: bool,
    a_wait_free_list: Vec<usize>,
    a_wait: Vec<Option<Rc<RefCell<AWait>>>>,
    a_wait_dirty: bool,
    binary_free_list: Vec<usize>,
    binary: Vec<Option<Rc<RefCell<Binary>>>>,
    binary_dirty: bool,
    block_free_list: Vec<usize>,
    block: Vec<Option<Rc<RefCell<Block>>>>,
    block_dirty: bool,
    body_free_list: Vec<usize>,
    body: Vec<Option<Rc<RefCell<Body>>>>,
    body_dirty: bool,
    boolean_literal_free_list: Vec<usize>,
    boolean_literal: Vec<Option<Rc<RefCell<BooleanLiteral>>>>,
    boolean_literal_dirty: bool,
    boolean_operator_free_list: Vec<usize>,
    boolean_operator: Vec<Option<Rc<RefCell<BooleanOperator>>>>,
    boolean_operator_dirty: bool,
    call_free_list: Vec<usize>,
    call: Vec<Option<Rc<RefCell<Call>>>>,
    call_dirty: bool,
    char_literal_free_list: Vec<usize>,
    char_literal: Vec<Option<Rc<RefCell<CharLiteral>>>>,
    char_literal_dirty: bool,
    comparison_free_list: Vec<usize>,
    comparison: Vec<Option<Rc<RefCell<Comparison>>>>,
    comparison_dirty: bool,
    data_structure_free_list: Vec<usize>,
    data_structure: Vec<Option<Rc<RefCell<DataStructure>>>>,
    data_structure_dirty: bool,
    dwarf_source_file_free_list: Vec<usize>,
    dwarf_source_file: Vec<Option<Rc<RefCell<DwarfSourceFile>>>>,
    dwarf_source_file_dirty: bool,
    enum_field_free_list: Vec<usize>,
    enum_field: Vec<Option<Rc<RefCell<EnumField>>>>,
    enum_field_dirty: bool,
    enum_generic_free_list: Vec<usize>,
    enum_generic: Vec<Option<Rc<RefCell<EnumGeneric>>>>,
    enum_generic_dirty: bool,
    enum_generic_type_free_list: Vec<usize>,
    enum_generic_type: Vec<Option<Rc<RefCell<EnumGenericType>>>>,
    enum_generic_type_dirty: bool,
    enumeration_free_list: Vec<usize>,
    enumeration: Vec<Option<Rc<RefCell<Enumeration>>>>,
    enumeration_id_by_name: HashMap<String, usize>,
    enumeration_dirty: bool,
    expression_free_list: Vec<usize>,
    expression: Vec<Option<Rc<RefCell<Expression>>>>,
    expression_dirty: bool,
    expression_bit_free_list: Vec<usize>,
    expression_bit: Vec<Option<Rc<RefCell<ExpressionBit>>>>,
    expression_bit_dirty: bool,
    expression_statement_free_list: Vec<usize>,
    expression_statement: Vec<Option<Rc<RefCell<ExpressionStatement>>>>,
    expression_statement_dirty: bool,
    external_implementation_free_list: Vec<usize>,
    external_implementation: Vec<Option<Rc<RefCell<ExternalImplementation>>>>,
    external_implementation_dirty: bool,
    field_free_list: Vec<usize>,
    field: Vec<Option<Rc<RefCell<Field>>>>,
    field_id_by_name: HashMap<String, usize>,
    field_dirty: bool,
    field_access_free_list: Vec<usize>,
    field_access: Vec<Option<Rc<RefCell<FieldAccess>>>>,
    field_access_dirty: bool,
    field_access_target_free_list: Vec<usize>,
    field_access_target: Vec<Option<Rc<RefCell<FieldAccessTarget>>>>,
    field_access_target_dirty: bool,
    field_expression_free_list: Vec<usize>,
    field_expression: Vec<Option<Rc<RefCell<FieldExpression>>>>,
    field_expression_dirty: bool,
    float_literal_free_list: Vec<usize>,
    float_literal: Vec<Option<Rc<RefCell<FloatLiteral>>>>,
    float_literal_dirty: bool,
    for_loop_free_list: Vec<usize>,
    for_loop: Vec<Option<Rc<RefCell<ForLoop>>>>,
    for_loop_dirty: bool,
    format_bit_free_list: Vec<usize>,
    format_bit: Vec<Option<Rc<RefCell<FormatBit>>>>,
    format_bit_dirty: bool,
    format_string_free_list: Vec<usize>,
    format_string: Vec<Option<Rc<RefCell<FormatString>>>>,
    format_string_dirty: bool,
    func_generic_free_list: Vec<usize>,
    func_generic: Vec<Option<Rc<RefCell<FuncGeneric>>>>,
    func_generic_dirty: bool,
    function_free_list: Vec<usize>,
    function: Vec<Option<Rc<RefCell<Function>>>>,
    function_id_by_name: HashMap<String, usize>,
    function_dirty: bool,
    function_call_free_list: Vec<usize>,
    function_call: Vec<Option<Rc<RefCell<FunctionCall>>>>,
    function_call_dirty: bool,
    x_future_free_list: Vec<usize>,
    x_future: Vec<Option<Rc<RefCell<XFuture>>>>,
    x_future_dirty: bool,
    grouped_free_list: Vec<usize>,
    grouped: Vec<Option<Rc<RefCell<Grouped>>>>,
    grouped_dirty: bool,
    halt_and_catch_fire_free_list: Vec<usize>,
    halt_and_catch_fire: Vec<Option<Rc<RefCell<HaltAndCatchFire>>>>,
    halt_and_catch_fire_dirty: bool,
    x_if_free_list: Vec<usize>,
    x_if: Vec<Option<Rc<RefCell<XIf>>>>,
    x_if_dirty: bool,
    implementation_block_free_list: Vec<usize>,
    implementation_block: Vec<Option<Rc<RefCell<ImplementationBlock>>>>,
    implementation_block_dirty: bool,
    import_free_list: Vec<usize>,
    import: Vec<Option<Rc<RefCell<Import>>>>,
    import_dirty: bool,
    index_free_list: Vec<usize>,
    index: Vec<Option<Rc<RefCell<Index>>>>,
    index_dirty: bool,
    integer_literal_free_list: Vec<usize>,
    integer_literal: Vec<Option<Rc<RefCell<IntegerLiteral>>>>,
    integer_literal_dirty: bool,
    item_free_list: Vec<usize>,
    item: Vec<Option<Rc<RefCell<Item>>>>,
    item_dirty: bool,
    lambda_free_list: Vec<usize>,
    lambda: Vec<Option<Rc<RefCell<Lambda>>>>,
    lambda_dirty: bool,
    lambda_parameter_free_list: Vec<usize>,
    lambda_parameter: Vec<Option<Rc<RefCell<LambdaParameter>>>>,
    lambda_parameter_dirty: bool,
    let_statement_free_list: Vec<usize>,
    let_statement: Vec<Option<Rc<RefCell<LetStatement>>>>,
    let_statement_dirty: bool,
    list_free_list: Vec<usize>,
    list: Vec<Option<Rc<RefCell<List>>>>,
    list_dirty: bool,
    list_element_free_list: Vec<usize>,
    list_element: Vec<Option<Rc<RefCell<ListElement>>>>,
    list_element_dirty: bool,
    list_expression_free_list: Vec<usize>,
    list_expression: Vec<Option<Rc<RefCell<ListExpression>>>>,
    list_expression_dirty: bool,
    literal_free_list: Vec<usize>,
    literal: Vec<Option<Rc<RefCell<Literal>>>>,
    literal_dirty: bool,
    local_variable_free_list: Vec<usize>,
    local_variable: Vec<Option<Rc<RefCell<LocalVariable>>>>,
    local_variable_dirty: bool,
    x_macro_free_list: Vec<usize>,
    x_macro: Vec<Option<Rc<RefCell<XMacro>>>>,
    x_macro_dirty: bool,
    map_free_list: Vec<usize>,
    map: Vec<Option<Rc<RefCell<Map>>>>,
    map_dirty: bool,
    map_element_free_list: Vec<usize>,
    map_element: Vec<Option<Rc<RefCell<MapElement>>>>,
    map_element_dirty: bool,
    map_expression_free_list: Vec<usize>,
    map_expression: Vec<Option<Rc<RefCell<MapExpression>>>>,
    map_expression_dirty: bool,
    x_match_free_list: Vec<usize>,
    x_match: Vec<Option<Rc<RefCell<XMatch>>>>,
    x_match_dirty: bool,
    method_call_free_list: Vec<usize>,
    method_call: Vec<Option<Rc<RefCell<MethodCall>>>>,
    method_call_dirty: bool,
    named_field_expression_free_list: Vec<usize>,
    named_field_expression: Vec<Option<Rc<RefCell<NamedFieldExpression>>>>,
    named_field_expression_dirty: bool,
    z_object_store_free_list: Vec<usize>,
    z_object_store: Vec<Option<Rc<RefCell<ZObjectStore>>>>,
    z_object_store_id_by_name: HashMap<String, usize>,
    z_object_store_dirty: bool,
    object_wrapper_free_list: Vec<usize>,
    object_wrapper: Vec<Option<Rc<RefCell<ObjectWrapper>>>>,
    object_wrapper_dirty: bool,
    operator_free_list: Vec<usize>,
    operator: Vec<Option<Rc<RefCell<Operator>>>>,
    operator_dirty: bool,
    parameter_free_list: Vec<usize>,
    parameter: Vec<Option<Rc<RefCell<Parameter>>>>,
    parameter_dirty: bool,
    x_path_free_list: Vec<usize>,
    x_path: Vec<Option<Rc<RefCell<XPath>>>>,
    x_path_dirty: bool,
    path_element_free_list: Vec<usize>,
    path_element: Vec<Option<Rc<RefCell<PathElement>>>>,
    path_element_dirty: bool,
    pattern_free_list: Vec<usize>,
    pattern: Vec<Option<Rc<RefCell<Pattern>>>>,
    pattern_dirty: bool,
    x_plugin_free_list: Vec<usize>,
    x_plugin: Vec<Option<Rc<RefCell<XPlugin>>>>,
    x_plugin_id_by_name: HashMap<String, usize>,
    x_plugin_dirty: bool,
    x_print_free_list: Vec<usize>,
    x_print: Vec<Option<Rc<RefCell<XPrint>>>>,
    x_print_dirty: bool,
    range_expression_free_list: Vec<usize>,
    range_expression: Vec<Option<Rc<RefCell<RangeExpression>>>>,
    range_expression_dirty: bool,
    result_statement_free_list: Vec<usize>,
    result_statement: Vec<Option<Rc<RefCell<ResultStatement>>>>,
    result_statement_dirty: bool,
    x_return_free_list: Vec<usize>,
    x_return: Vec<Option<Rc<RefCell<XReturn>>>>,
    x_return_dirty: bool,
    span_free_list: Vec<usize>,
    span: Vec<Option<Rc<RefCell<Span>>>>,
    span_dirty: bool,
    statement_free_list: Vec<usize>,
    statement: Vec<Option<Rc<RefCell<Statement>>>>,
    statement_dirty: bool,
    static_method_call_free_list: Vec<usize>,
    static_method_call: Vec<Option<Rc<RefCell<StaticMethodCall>>>>,
    static_method_call_dirty: bool,
    string_bit_free_list: Vec<usize>,
    string_bit: Vec<Option<Rc<RefCell<StringBit>>>>,
    string_bit_dirty: bool,
    string_literal_free_list: Vec<usize>,
    string_literal: Vec<Option<Rc<RefCell<StringLiteral>>>>,
    string_literal_dirty: bool,
    woog_struct_free_list: Vec<usize>,
    woog_struct: Vec<Option<Rc<RefCell<WoogStruct>>>>,
    woog_struct_id_by_name: HashMap<String, usize>,
    woog_struct_dirty: bool,
    struct_expression_free_list: Vec<usize>,
    struct_expression: Vec<Option<Rc<RefCell<StructExpression>>>>,
    struct_expression_dirty: bool,
    struct_field_free_list: Vec<usize>,
    struct_field: Vec<Option<Rc<RefCell<StructField>>>>,
    struct_field_dirty: bool,
    struct_generic_free_list: Vec<usize>,
    struct_generic: Vec<Option<Rc<RefCell<StructGeneric>>>>,
    struct_generic_dirty: bool,
    tuple_field_free_list: Vec<usize>,
    tuple_field: Vec<Option<Rc<RefCell<TupleField>>>>,
    tuple_field_dirty: bool,
    type_cast_free_list: Vec<usize>,
    type_cast: Vec<Option<Rc<RefCell<TypeCast>>>>,
    type_cast_dirty: bool,
    unary_free_list: Vec<usize>,
    unary: Vec<Option<Rc<RefCell<Unary>>>>,
    unary_dirty: bool,
    unit_free_list: Vec<usize>,
    unit: Vec<Option<Rc<RefCell<Unit>>>>,
    unit_dirty: bool,
    unnamed_field_expression_free_list: Vec<usize>,
    unnamed_field_expression: Vec<Option<Rc<RefCell<UnnamedFieldExpression>>>>,
    unnamed_field_expression_dirty: bool,
    x_value_free_list: Vec<usize>,
    x_value: Vec<Option<Rc<RefCell<XValue>>>>,
    x_value_dirty: bool,
    value_type_free_list: Vec<usize>,
    value_type: Vec<Option<Rc<RefCell<ValueType>>>>,
    value_type_dirty: bool,
    variable_free_list: Vec<usize>,
    variable: Vec<Option<Rc<RefCell<Variable>>>>,
    variable_dirty: bool,
    variable_expression_free_list: Vec<usize>,
    variable_expression: Vec<Option<Rc<RefCell<VariableExpression>>>>,
    variable_expression_dirty: bool,
}

impl Clone for ObjectStore {
    fn clone(&self) -> Self {
        ObjectStore {
            argument_free_list: self.argument_free_list.clone(),
            argument: self.argument.clone(),
            argument_dirty: false,
            a_wait_free_list: self.a_wait_free_list.clone(),
            a_wait: self.a_wait.clone(),
            a_wait_dirty: false,
            binary_free_list: self.binary_free_list.clone(),
            binary: self.binary.clone(),
            binary_dirty: false,
            block_free_list: self.block_free_list.clone(),
            block: self.block.clone(),
            block_dirty: false,
            body_free_list: self.body_free_list.clone(),
            body: self.body.clone(),
            body_dirty: false,
            boolean_literal_free_list: self.boolean_literal_free_list.clone(),
            boolean_literal: self.boolean_literal.clone(),
            boolean_literal_dirty: false,
            boolean_operator_free_list: self.boolean_operator_free_list.clone(),
            boolean_operator: self.boolean_operator.clone(),
            boolean_operator_dirty: false,
            call_free_list: self.call_free_list.clone(),
            call: self.call.clone(),
            call_dirty: false,
            char_literal_free_list: self.char_literal_free_list.clone(),
            char_literal: self.char_literal.clone(),
            char_literal_dirty: false,
            comparison_free_list: self.comparison_free_list.clone(),
            comparison: self.comparison.clone(),
            comparison_dirty: false,
            data_structure_free_list: self.data_structure_free_list.clone(),
            data_structure: self.data_structure.clone(),
            data_structure_dirty: false,
            dwarf_source_file_free_list: self.dwarf_source_file_free_list.clone(),
            dwarf_source_file: self.dwarf_source_file.clone(),
            dwarf_source_file_dirty: false,
            enum_field_free_list: self.enum_field_free_list.clone(),
            enum_field: self.enum_field.clone(),
            enum_field_dirty: false,
            enum_generic_free_list: self.enum_generic_free_list.clone(),
            enum_generic: self.enum_generic.clone(),
            enum_generic_dirty: false,
            enum_generic_type_free_list: self.enum_generic_type_free_list.clone(),
            enum_generic_type: self.enum_generic_type.clone(),
            enum_generic_type_dirty: false,
            enumeration_free_list: self.enumeration_free_list.clone(),
            enumeration: self.enumeration.clone(),
            enumeration_id_by_name: self.enumeration_id_by_name.clone(),
            enumeration_dirty: false,
            expression_free_list: self.expression_free_list.clone(),
            expression: self.expression.clone(),
            expression_dirty: false,
            expression_bit_free_list: self.expression_bit_free_list.clone(),
            expression_bit: self.expression_bit.clone(),
            expression_bit_dirty: false,
            expression_statement_free_list: self.expression_statement_free_list.clone(),
            expression_statement: self.expression_statement.clone(),
            expression_statement_dirty: false,
            external_implementation_free_list: self.external_implementation_free_list.clone(),
            external_implementation: self.external_implementation.clone(),
            external_implementation_dirty: false,
            field_free_list: self.field_free_list.clone(),
            field: self.field.clone(),
            field_id_by_name: self.field_id_by_name.clone(),
            field_dirty: false,
            field_access_free_list: self.field_access_free_list.clone(),
            field_access: self.field_access.clone(),
            field_access_dirty: false,
            field_access_target_free_list: self.field_access_target_free_list.clone(),
            field_access_target: self.field_access_target.clone(),
            field_access_target_dirty: false,
            field_expression_free_list: self.field_expression_free_list.clone(),
            field_expression: self.field_expression.clone(),
            field_expression_dirty: false,
            float_literal_free_list: self.float_literal_free_list.clone(),
            float_literal: self.float_literal.clone(),
            float_literal_dirty: false,
            for_loop_free_list: self.for_loop_free_list.clone(),
            for_loop: self.for_loop.clone(),
            for_loop_dirty: false,
            format_bit_free_list: self.format_bit_free_list.clone(),
            format_bit: self.format_bit.clone(),
            format_bit_dirty: false,
            format_string_free_list: self.format_string_free_list.clone(),
            format_string: self.format_string.clone(),
            format_string_dirty: false,
            func_generic_free_list: self.func_generic_free_list.clone(),
            func_generic: self.func_generic.clone(),
            func_generic_dirty: false,
            function_free_list: self.function_free_list.clone(),
            function: self.function.clone(),
            function_id_by_name: self.function_id_by_name.clone(),
            function_dirty: false,
            function_call_free_list: self.function_call_free_list.clone(),
            function_call: self.function_call.clone(),
            function_call_dirty: false,
            x_future_free_list: self.x_future_free_list.clone(),
            x_future: self.x_future.clone(),
            x_future_dirty: false,
            grouped_free_list: self.grouped_free_list.clone(),
            grouped: self.grouped.clone(),
            grouped_dirty: false,
            halt_and_catch_fire_free_list: self.halt_and_catch_fire_free_list.clone(),
            halt_and_catch_fire: self.halt_and_catch_fire.clone(),
            halt_and_catch_fire_dirty: false,
            x_if_free_list: self.x_if_free_list.clone(),
            x_if: self.x_if.clone(),
            x_if_dirty: false,
            implementation_block_free_list: self.implementation_block_free_list.clone(),
            implementation_block: self.implementation_block.clone(),
            implementation_block_dirty: false,
            import_free_list: self.import_free_list.clone(),
            import: self.import.clone(),
            import_dirty: false,
            index_free_list: self.index_free_list.clone(),
            index: self.index.clone(),
            index_dirty: false,
            integer_literal_free_list: self.integer_literal_free_list.clone(),
            integer_literal: self.integer_literal.clone(),
            integer_literal_dirty: false,
            item_free_list: self.item_free_list.clone(),
            item: self.item.clone(),
            item_dirty: false,
            lambda_free_list: self.lambda_free_list.clone(),
            lambda: self.lambda.clone(),
            lambda_dirty: false,
            lambda_parameter_free_list: self.lambda_parameter_free_list.clone(),
            lambda_parameter: self.lambda_parameter.clone(),
            lambda_parameter_dirty: false,
            let_statement_free_list: self.let_statement_free_list.clone(),
            let_statement: self.let_statement.clone(),
            let_statement_dirty: false,
            list_free_list: self.list_free_list.clone(),
            list: self.list.clone(),
            list_dirty: false,
            list_element_free_list: self.list_element_free_list.clone(),
            list_element: self.list_element.clone(),
            list_element_dirty: false,
            list_expression_free_list: self.list_expression_free_list.clone(),
            list_expression: self.list_expression.clone(),
            list_expression_dirty: false,
            literal_free_list: self.literal_free_list.clone(),
            literal: self.literal.clone(),
            literal_dirty: false,
            local_variable_free_list: self.local_variable_free_list.clone(),
            local_variable: self.local_variable.clone(),
            local_variable_dirty: false,
            x_macro_free_list: self.x_macro_free_list.clone(),
            x_macro: self.x_macro.clone(),
            x_macro_dirty: false,
            map_free_list: self.map_free_list.clone(),
            map: self.map.clone(),
            map_dirty: false,
            map_element_free_list: self.map_element_free_list.clone(),
            map_element: self.map_element.clone(),
            map_element_dirty: false,
            map_expression_free_list: self.map_expression_free_list.clone(),
            map_expression: self.map_expression.clone(),
            map_expression_dirty: false,
            x_match_free_list: self.x_match_free_list.clone(),
            x_match: self.x_match.clone(),
            x_match_dirty: false,
            method_call_free_list: self.method_call_free_list.clone(),
            method_call: self.method_call.clone(),
            method_call_dirty: false,
            named_field_expression_free_list: self.named_field_expression_free_list.clone(),
            named_field_expression: self.named_field_expression.clone(),
            named_field_expression_dirty: false,
            z_object_store_free_list: self.z_object_store_free_list.clone(),
            z_object_store: self.z_object_store.clone(),
            z_object_store_id_by_name: self.z_object_store_id_by_name.clone(),
            z_object_store_dirty: false,
            object_wrapper_free_list: self.object_wrapper_free_list.clone(),
            object_wrapper: self.object_wrapper.clone(),
            object_wrapper_dirty: false,
            operator_free_list: self.operator_free_list.clone(),
            operator: self.operator.clone(),
            operator_dirty: false,
            parameter_free_list: self.parameter_free_list.clone(),
            parameter: self.parameter.clone(),
            parameter_dirty: false,
            x_path_free_list: self.x_path_free_list.clone(),
            x_path: self.x_path.clone(),
            x_path_dirty: false,
            path_element_free_list: self.path_element_free_list.clone(),
            path_element: self.path_element.clone(),
            path_element_dirty: false,
            pattern_free_list: self.pattern_free_list.clone(),
            pattern: self.pattern.clone(),
            pattern_dirty: false,
            x_plugin_free_list: self.x_plugin_free_list.clone(),
            x_plugin: self.x_plugin.clone(),
            x_plugin_id_by_name: self.x_plugin_id_by_name.clone(),
            x_plugin_dirty: false,
            x_print_free_list: self.x_print_free_list.clone(),
            x_print: self.x_print.clone(),
            x_print_dirty: false,
            range_expression_free_list: self.range_expression_free_list.clone(),
            range_expression: self.range_expression.clone(),
            range_expression_dirty: false,
            result_statement_free_list: self.result_statement_free_list.clone(),
            result_statement: self.result_statement.clone(),
            result_statement_dirty: false,
            x_return_free_list: self.x_return_free_list.clone(),
            x_return: self.x_return.clone(),
            x_return_dirty: false,
            span_free_list: self.span_free_list.clone(),
            span: self.span.clone(),
            span_dirty: false,
            statement_free_list: self.statement_free_list.clone(),
            statement: self.statement.clone(),
            statement_dirty: false,
            static_method_call_free_list: self.static_method_call_free_list.clone(),
            static_method_call: self.static_method_call.clone(),
            static_method_call_dirty: false,
            string_bit_free_list: self.string_bit_free_list.clone(),
            string_bit: self.string_bit.clone(),
            string_bit_dirty: false,
            string_literal_free_list: self.string_literal_free_list.clone(),
            string_literal: self.string_literal.clone(),
            string_literal_dirty: false,
            woog_struct_free_list: self.woog_struct_free_list.clone(),
            woog_struct: self.woog_struct.clone(),
            woog_struct_id_by_name: self.woog_struct_id_by_name.clone(),
            woog_struct_dirty: false,
            struct_expression_free_list: self.struct_expression_free_list.clone(),
            struct_expression: self.struct_expression.clone(),
            struct_expression_dirty: false,
            struct_field_free_list: self.struct_field_free_list.clone(),
            struct_field: self.struct_field.clone(),
            struct_field_dirty: false,
            struct_generic_free_list: self.struct_generic_free_list.clone(),
            struct_generic: self.struct_generic.clone(),
            struct_generic_dirty: false,
            tuple_field_free_list: self.tuple_field_free_list.clone(),
            tuple_field: self.tuple_field.clone(),
            tuple_field_dirty: false,
            type_cast_free_list: self.type_cast_free_list.clone(),
            type_cast: self.type_cast.clone(),
            type_cast_dirty: false,
            unary_free_list: self.unary_free_list.clone(),
            unary: self.unary.clone(),
            unary_dirty: false,
            unit_free_list: self.unit_free_list.clone(),
            unit: self.unit.clone(),
            unit_dirty: false,
            unnamed_field_expression_free_list: self.unnamed_field_expression_free_list.clone(),
            unnamed_field_expression: self.unnamed_field_expression.clone(),
            unnamed_field_expression_dirty: false,
            x_value_free_list: self.x_value_free_list.clone(),
            x_value: self.x_value.clone(),
            x_value_dirty: false,
            value_type_free_list: self.value_type_free_list.clone(),
            value_type: self.value_type.clone(),
            value_type_dirty: false,
            variable_free_list: self.variable_free_list.clone(),
            variable: self.variable.clone(),
            variable_dirty: false,
            variable_expression_free_list: self.variable_expression_free_list.clone(),
            variable_expression: self.variable_expression.clone(),
            variable_expression_dirty: false,
        }
    }
}
impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument_free_list: Vec::new(),
            argument: Vec::new(),
            argument_dirty: false,
            a_wait_free_list: Vec::new(),
            a_wait: Vec::new(),
            a_wait_dirty: false,
            binary_free_list: Vec::new(),
            binary: Vec::new(),
            binary_dirty: false,
            block_free_list: Vec::new(),
            block: Vec::new(),
            block_dirty: false,
            body_free_list: Vec::new(),
            body: Vec::new(),
            body_dirty: false,
            boolean_literal_free_list: Vec::new(),
            boolean_literal: Vec::new(),
            boolean_literal_dirty: false,
            boolean_operator_free_list: Vec::new(),
            boolean_operator: Vec::new(),
            boolean_operator_dirty: false,
            call_free_list: Vec::new(),
            call: Vec::new(),
            call_dirty: false,
            char_literal_free_list: Vec::new(),
            char_literal: Vec::new(),
            char_literal_dirty: false,
            comparison_free_list: Vec::new(),
            comparison: Vec::new(),
            comparison_dirty: false,
            data_structure_free_list: Vec::new(),
            data_structure: Vec::new(),
            data_structure_dirty: false,
            dwarf_source_file_free_list: Vec::new(),
            dwarf_source_file: Vec::new(),
            dwarf_source_file_dirty: false,
            enum_field_free_list: Vec::new(),
            enum_field: Vec::new(),
            enum_field_dirty: false,
            enum_generic_free_list: Vec::new(),
            enum_generic: Vec::new(),
            enum_generic_dirty: false,
            enum_generic_type_free_list: Vec::new(),
            enum_generic_type: Vec::new(),
            enum_generic_type_dirty: false,
            enumeration_free_list: Vec::new(),
            enumeration: Vec::new(),
            enumeration_id_by_name: HashMap::default(),
            enumeration_dirty: false,
            expression_free_list: Vec::new(),
            expression: Vec::new(),
            expression_dirty: false,
            expression_bit_free_list: Vec::new(),
            expression_bit: Vec::new(),
            expression_bit_dirty: false,
            expression_statement_free_list: Vec::new(),
            expression_statement: Vec::new(),
            expression_statement_dirty: false,
            external_implementation_free_list: Vec::new(),
            external_implementation: Vec::new(),
            external_implementation_dirty: false,
            field_free_list: Vec::new(),
            field: Vec::new(),
            field_id_by_name: HashMap::default(),
            field_dirty: false,
            field_access_free_list: Vec::new(),
            field_access: Vec::new(),
            field_access_dirty: false,
            field_access_target_free_list: Vec::new(),
            field_access_target: Vec::new(),
            field_access_target_dirty: false,
            field_expression_free_list: Vec::new(),
            field_expression: Vec::new(),
            field_expression_dirty: false,
            float_literal_free_list: Vec::new(),
            float_literal: Vec::new(),
            float_literal_dirty: false,
            for_loop_free_list: Vec::new(),
            for_loop: Vec::new(),
            for_loop_dirty: false,
            format_bit_free_list: Vec::new(),
            format_bit: Vec::new(),
            format_bit_dirty: false,
            format_string_free_list: Vec::new(),
            format_string: Vec::new(),
            format_string_dirty: false,
            func_generic_free_list: Vec::new(),
            func_generic: Vec::new(),
            func_generic_dirty: false,
            function_free_list: Vec::new(),
            function: Vec::new(),
            function_id_by_name: HashMap::default(),
            function_dirty: false,
            function_call_free_list: Vec::new(),
            function_call: Vec::new(),
            function_call_dirty: false,
            x_future_free_list: Vec::new(),
            x_future: Vec::new(),
            x_future_dirty: false,
            grouped_free_list: Vec::new(),
            grouped: Vec::new(),
            grouped_dirty: false,
            halt_and_catch_fire_free_list: Vec::new(),
            halt_and_catch_fire: Vec::new(),
            halt_and_catch_fire_dirty: false,
            x_if_free_list: Vec::new(),
            x_if: Vec::new(),
            x_if_dirty: false,
            implementation_block_free_list: Vec::new(),
            implementation_block: Vec::new(),
            implementation_block_dirty: false,
            import_free_list: Vec::new(),
            import: Vec::new(),
            import_dirty: false,
            index_free_list: Vec::new(),
            index: Vec::new(),
            index_dirty: false,
            integer_literal_free_list: Vec::new(),
            integer_literal: Vec::new(),
            integer_literal_dirty: false,
            item_free_list: Vec::new(),
            item: Vec::new(),
            item_dirty: false,
            lambda_free_list: Vec::new(),
            lambda: Vec::new(),
            lambda_dirty: false,
            lambda_parameter_free_list: Vec::new(),
            lambda_parameter: Vec::new(),
            lambda_parameter_dirty: false,
            let_statement_free_list: Vec::new(),
            let_statement: Vec::new(),
            let_statement_dirty: false,
            list_free_list: Vec::new(),
            list: Vec::new(),
            list_dirty: false,
            list_element_free_list: Vec::new(),
            list_element: Vec::new(),
            list_element_dirty: false,
            list_expression_free_list: Vec::new(),
            list_expression: Vec::new(),
            list_expression_dirty: false,
            literal_free_list: Vec::new(),
            literal: Vec::new(),
            literal_dirty: false,
            local_variable_free_list: Vec::new(),
            local_variable: Vec::new(),
            local_variable_dirty: false,
            x_macro_free_list: Vec::new(),
            x_macro: Vec::new(),
            x_macro_dirty: false,
            map_free_list: Vec::new(),
            map: Vec::new(),
            map_dirty: false,
            map_element_free_list: Vec::new(),
            map_element: Vec::new(),
            map_element_dirty: false,
            map_expression_free_list: Vec::new(),
            map_expression: Vec::new(),
            map_expression_dirty: false,
            x_match_free_list: Vec::new(),
            x_match: Vec::new(),
            x_match_dirty: false,
            method_call_free_list: Vec::new(),
            method_call: Vec::new(),
            method_call_dirty: false,
            named_field_expression_free_list: Vec::new(),
            named_field_expression: Vec::new(),
            named_field_expression_dirty: false,
            z_object_store_free_list: Vec::new(),
            z_object_store: Vec::new(),
            z_object_store_id_by_name: HashMap::default(),
            z_object_store_dirty: false,
            object_wrapper_free_list: Vec::new(),
            object_wrapper: Vec::new(),
            object_wrapper_dirty: false,
            operator_free_list: Vec::new(),
            operator: Vec::new(),
            operator_dirty: false,
            parameter_free_list: Vec::new(),
            parameter: Vec::new(),
            parameter_dirty: false,
            x_path_free_list: Vec::new(),
            x_path: Vec::new(),
            x_path_dirty: false,
            path_element_free_list: Vec::new(),
            path_element: Vec::new(),
            path_element_dirty: false,
            pattern_free_list: Vec::new(),
            pattern: Vec::new(),
            pattern_dirty: false,
            x_plugin_free_list: Vec::new(),
            x_plugin: Vec::new(),
            x_plugin_id_by_name: HashMap::default(),
            x_plugin_dirty: false,
            x_print_free_list: Vec::new(),
            x_print: Vec::new(),
            x_print_dirty: false,
            range_expression_free_list: Vec::new(),
            range_expression: Vec::new(),
            range_expression_dirty: false,
            result_statement_free_list: Vec::new(),
            result_statement: Vec::new(),
            result_statement_dirty: false,
            x_return_free_list: Vec::new(),
            x_return: Vec::new(),
            x_return_dirty: false,
            span_free_list: Vec::new(),
            span: Vec::new(),
            span_dirty: false,
            statement_free_list: Vec::new(),
            statement: Vec::new(),
            statement_dirty: false,
            static_method_call_free_list: Vec::new(),
            static_method_call: Vec::new(),
            static_method_call_dirty: false,
            string_bit_free_list: Vec::new(),
            string_bit: Vec::new(),
            string_bit_dirty: false,
            string_literal_free_list: Vec::new(),
            string_literal: Vec::new(),
            string_literal_dirty: false,
            woog_struct_free_list: Vec::new(),
            woog_struct: Vec::new(),
            woog_struct_id_by_name: HashMap::default(),
            woog_struct_dirty: false,
            struct_expression_free_list: Vec::new(),
            struct_expression: Vec::new(),
            struct_expression_dirty: false,
            struct_field_free_list: Vec::new(),
            struct_field: Vec::new(),
            struct_field_dirty: false,
            struct_generic_free_list: Vec::new(),
            struct_generic: Vec::new(),
            struct_generic_dirty: false,
            tuple_field_free_list: Vec::new(),
            tuple_field: Vec::new(),
            tuple_field_dirty: false,
            type_cast_free_list: Vec::new(),
            type_cast: Vec::new(),
            type_cast_dirty: false,
            unary_free_list: Vec::new(),
            unary: Vec::new(),
            unary_dirty: false,
            unit_free_list: Vec::new(),
            unit: Vec::new(),
            unit_dirty: false,
            unnamed_field_expression_free_list: Vec::new(),
            unnamed_field_expression: Vec::new(),
            unnamed_field_expression_dirty: false,
            x_value_free_list: Vec::new(),
            x_value: Vec::new(),
            x_value_dirty: false,
            value_type_free_list: Vec::new(),
            value_type: Vec::new(),
            value_type_dirty: false,
            variable_free_list: Vec::new(),
            variable: Vec::new(),
            variable_dirty: false,
            variable_expression_free_list: Vec::new(),
            variable_expression: Vec::new(),
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
            other.argument.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in argument, if it's not there add it to argument.
                    if self
                        .argument
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.argument.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_argument(|id| -> Rc<RefCell<Argument>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.a_wait_dirty {
            other.a_wait.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in a_wait, if it's not there add it to a_wait.
                    if self
                        .a_wait
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.a_wait.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_a_wait(|id| -> Rc<RefCell<AWait>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.binary_dirty {
            other.binary.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in binary, if it's not there add it to binary.
                    if self
                        .binary
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.binary.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_binary(|id| -> Rc<RefCell<Binary>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.block_dirty {
            other.block.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in block, if it's not there add it to block.
                    if self
                        .block
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.block.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_block(|id| -> Rc<RefCell<Block>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.body_dirty {
            other.body.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in body, if it's not there add it to body.
                    if self
                        .body
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.body.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_body(|id| -> Rc<RefCell<Body>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.boolean_literal_dirty {
            other.boolean_literal.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in boolean_literal, if it's not there add it to boolean_literal.
                    if self
                        .boolean_literal
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.boolean_literal.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_boolean_literal(|id| -> Rc<RefCell<BooleanLiteral>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.boolean_operator_dirty {
            other.boolean_operator.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in boolean_operator, if it's not there add it to boolean_operator.
                    if self
                        .boolean_operator
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.boolean_operator.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_boolean_operator(|id| -> Rc<RefCell<BooleanOperator>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.call_dirty {
            other.call.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in call, if it's not there add it to call.
                    if self
                        .call
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.call.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_call(|id| -> Rc<RefCell<Call>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.char_literal_dirty {
            other.char_literal.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in char_literal, if it's not there add it to char_literal.
                    if self
                        .char_literal
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.char_literal.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_char_literal(|id| -> Rc<RefCell<CharLiteral>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.comparison_dirty {
            other.comparison.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in comparison, if it's not there add it to comparison.
                    if self
                        .comparison
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.comparison.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_comparison(|id| -> Rc<RefCell<Comparison>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.data_structure_dirty {
            other.data_structure.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in data_structure, if it's not there add it to data_structure.
                    if self
                        .data_structure
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.data_structure.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_data_structure(|id| -> Rc<RefCell<DataStructure>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.dwarf_source_file_dirty {
            other.dwarf_source_file.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in dwarf_source_file, if it's not there add it to dwarf_source_file.
                    if self
                        .dwarf_source_file
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.dwarf_source_file.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_dwarf_source_file(|id| -> Rc<RefCell<DwarfSourceFile>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.enum_field_dirty {
            other.enum_field.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in enum_field, if it's not there add it to enum_field.
                    if self
                        .enum_field
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.enum_field.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_enum_field(|id| -> Rc<RefCell<EnumField>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.enum_generic_dirty {
            other.enum_generic.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in enum_generic, if it's not there add it to enum_generic.
                    if self
                        .enum_generic
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.enum_generic.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_enum_generic(|id| -> Rc<RefCell<EnumGeneric>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.enum_generic_type_dirty {
            other.enum_generic_type.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in enum_generic_type, if it's not there add it to enum_generic_type.
                    if self
                        .enum_generic_type
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.enum_generic_type.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_enum_generic_type(|id| -> Rc<RefCell<EnumGenericType>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.enumeration_dirty {
            other.enumeration.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in enumeration, if it's not there add it to enumeration.
                    if self
                        .enumeration
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.enumeration.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_enumeration(|id| -> Rc<RefCell<Enumeration>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.expression_dirty {
            other.expression.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in expression, if it's not there add it to expression.
                    if self
                        .expression
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.expression.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_expression(|id| -> Rc<RefCell<Expression>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.expression_bit_dirty {
            other.expression_bit.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in expression_bit, if it's not there add it to expression_bit.
                    if self
                        .expression_bit
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.expression_bit.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_expression_bit(|id| -> Rc<RefCell<ExpressionBit>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.expression_statement_dirty {
            other.expression_statement.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in expression_statement, if it's not there add it to expression_statement.
                    if self
                        .expression_statement
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.expression_statement.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_expression_statement(|id| -> Rc<RefCell<ExpressionStatement>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.external_implementation_dirty {
            other.external_implementation.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in external_implementation, if it's not there add it to external_implementation.
                    if self
                        .external_implementation
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.external_implementation.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_external_implementation(
                            |id| -> Rc<RefCell<ExternalImplementation>> {
                                if x.borrow().id != id {
                                    panic!("id mismatch");
                                }

                                x.clone()
                            },
                        );
                    }
                }
            });
        }

        if other.field_dirty {
            other.field.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in field, if it's not there add it to field.
                    if self
                        .field
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.field.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_field(|id| -> Rc<RefCell<Field>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.field_access_dirty {
            other.field_access.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in field_access, if it's not there add it to field_access.
                    if self
                        .field_access
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.field_access.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_field_access(|id| -> Rc<RefCell<FieldAccess>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.field_access_target_dirty {
            other.field_access_target.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in field_access_target, if it's not there add it to field_access_target.
                    if self
                        .field_access_target
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.field_access_target.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_field_access_target(|id| -> Rc<RefCell<FieldAccessTarget>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.field_expression_dirty {
            other.field_expression.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in field_expression, if it's not there add it to field_expression.
                    if self
                        .field_expression
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.field_expression.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_field_expression(|id| -> Rc<RefCell<FieldExpression>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.float_literal_dirty {
            other.float_literal.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in float_literal, if it's not there add it to float_literal.
                    if self
                        .float_literal
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.float_literal.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_float_literal(|id| -> Rc<RefCell<FloatLiteral>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.for_loop_dirty {
            other.for_loop.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in for_loop, if it's not there add it to for_loop.
                    if self
                        .for_loop
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.for_loop.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_for_loop(|id| -> Rc<RefCell<ForLoop>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.format_bit_dirty {
            other.format_bit.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in format_bit, if it's not there add it to format_bit.
                    if self
                        .format_bit
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.format_bit.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_format_bit(|id| -> Rc<RefCell<FormatBit>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.format_string_dirty {
            other.format_string.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in format_string, if it's not there add it to format_string.
                    if self
                        .format_string
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.format_string.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_format_string(|id| -> Rc<RefCell<FormatString>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.func_generic_dirty {
            other.func_generic.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in func_generic, if it's not there add it to func_generic.
                    if self
                        .func_generic
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.func_generic.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_func_generic(|id| -> Rc<RefCell<FuncGeneric>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.function_dirty {
            other.function.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in function, if it's not there add it to function.
                    if self
                        .function
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.function.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_function(|id| -> Rc<RefCell<Function>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.function_call_dirty {
            other.function_call.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in function_call, if it's not there add it to function_call.
                    if self
                        .function_call
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.function_call.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_function_call(|id| -> Rc<RefCell<FunctionCall>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_future_dirty {
            other.x_future.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_future, if it's not there add it to x_future.
                    if self
                        .x_future
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.x_future.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_x_future(|id| -> Rc<RefCell<XFuture>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.grouped_dirty {
            other.grouped.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in grouped, if it's not there add it to grouped.
                    if self
                        .grouped
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.grouped.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_grouped(|id| -> Rc<RefCell<Grouped>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.halt_and_catch_fire_dirty {
            other.halt_and_catch_fire.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in halt_and_catch_fire, if it's not there add it to halt_and_catch_fire.
                    if self
                        .halt_and_catch_fire
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.halt_and_catch_fire.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_halt_and_catch_fire(|id| -> Rc<RefCell<HaltAndCatchFire>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_if_dirty {
            other.x_if.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_if, if it's not there add it to x_if.
                    if self
                        .x_if
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.x_if.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_x_if(|id| -> Rc<RefCell<XIf>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.implementation_block_dirty {
            other.implementation_block.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in implementation_block, if it's not there add it to implementation_block.
                    if self
                        .implementation_block
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.implementation_block.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_implementation_block(|id| -> Rc<RefCell<ImplementationBlock>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.import_dirty {
            other.import.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in import, if it's not there add it to import.
                    if self
                        .import
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.import.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_import(|id| -> Rc<RefCell<Import>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.index_dirty {
            other.index.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in index, if it's not there add it to index.
                    if self
                        .index
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.index.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_index(|id| -> Rc<RefCell<Index>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.integer_literal_dirty {
            other.integer_literal.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in integer_literal, if it's not there add it to integer_literal.
                    if self
                        .integer_literal
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.integer_literal.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_integer_literal(|id| -> Rc<RefCell<IntegerLiteral>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.item_dirty {
            other.item.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in item, if it's not there add it to item.
                    if self
                        .item
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.item.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_item(|id| -> Rc<RefCell<Item>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.lambda_dirty {
            other.lambda.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in lambda, if it's not there add it to lambda.
                    if self
                        .lambda
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.lambda.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_lambda(|id| -> Rc<RefCell<Lambda>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.lambda_parameter_dirty {
            other.lambda_parameter.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in lambda_parameter, if it's not there add it to lambda_parameter.
                    if self
                        .lambda_parameter
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.lambda_parameter.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_lambda_parameter(|id| -> Rc<RefCell<LambdaParameter>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.let_statement_dirty {
            other.let_statement.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in let_statement, if it's not there add it to let_statement.
                    if self
                        .let_statement
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.let_statement.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_let_statement(|id| -> Rc<RefCell<LetStatement>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.list_dirty {
            other.list.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in list, if it's not there add it to list.
                    if self
                        .list
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.list.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_list(|id| -> Rc<RefCell<List>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.list_element_dirty {
            other.list_element.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in list_element, if it's not there add it to list_element.
                    if self
                        .list_element
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.list_element.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_list_element(|id| -> Rc<RefCell<ListElement>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.list_expression_dirty {
            other.list_expression.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in list_expression, if it's not there add it to list_expression.
                    if self
                        .list_expression
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.list_expression.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_list_expression(|id| -> Rc<RefCell<ListExpression>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.literal_dirty {
            other.literal.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in literal, if it's not there add it to literal.
                    if self
                        .literal
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.literal.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_literal(|id| -> Rc<RefCell<Literal>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.local_variable_dirty {
            other.local_variable.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in local_variable, if it's not there add it to local_variable.
                    if self
                        .local_variable
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.local_variable.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_local_variable(|id| -> Rc<RefCell<LocalVariable>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_macro_dirty {
            other.x_macro.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_macro, if it's not there add it to x_macro.
                    if self
                        .x_macro
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.x_macro.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_x_macro(|id| -> Rc<RefCell<XMacro>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.map_dirty {
            other.map.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in map, if it's not there add it to map.
                    if self
                        .map
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.map.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_map(|id| -> Rc<RefCell<Map>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.map_element_dirty {
            other.map_element.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in map_element, if it's not there add it to map_element.
                    if self
                        .map_element
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.map_element.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_map_element(|id| -> Rc<RefCell<MapElement>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.map_expression_dirty {
            other.map_expression.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in map_expression, if it's not there add it to map_expression.
                    if self
                        .map_expression
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.map_expression.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_map_expression(|id| -> Rc<RefCell<MapExpression>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_match_dirty {
            other.x_match.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_match, if it's not there add it to x_match.
                    if self
                        .x_match
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.x_match.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_x_match(|id| -> Rc<RefCell<XMatch>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.method_call_dirty {
            other.method_call.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in method_call, if it's not there add it to method_call.
                    if self
                        .method_call
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.method_call.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_method_call(|id| -> Rc<RefCell<MethodCall>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.named_field_expression_dirty {
            other.named_field_expression.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in named_field_expression, if it's not there add it to named_field_expression.
                    if self
                        .named_field_expression
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.named_field_expression.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_named_field_expression(
                            |id| -> Rc<RefCell<NamedFieldExpression>> {
                                if x.borrow().id != id {
                                    panic!("id mismatch");
                                }

                                x.clone()
                            },
                        );
                    }
                }
            });
        }

        if other.z_object_store_dirty {
            other.z_object_store.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in z_object_store, if it's not there add it to z_object_store.
                    if self
                        .z_object_store
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.z_object_store.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_z_object_store(|id| -> Rc<RefCell<ZObjectStore>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.object_wrapper_dirty {
            other.object_wrapper.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in object_wrapper, if it's not there add it to object_wrapper.
                    if self
                        .object_wrapper
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.object_wrapper.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_object_wrapper(|id| -> Rc<RefCell<ObjectWrapper>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.operator_dirty {
            other.operator.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in operator, if it's not there add it to operator.
                    if self
                        .operator
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.operator.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_operator(|id| -> Rc<RefCell<Operator>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.parameter_dirty {
            other.parameter.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in parameter, if it's not there add it to parameter.
                    if self
                        .parameter
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.parameter.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_parameter(|id| -> Rc<RefCell<Parameter>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_path_dirty {
            other.x_path.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_path, if it's not there add it to x_path.
                    if self
                        .x_path
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.x_path.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_x_path(|id| -> Rc<RefCell<XPath>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.path_element_dirty {
            other.path_element.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in path_element, if it's not there add it to path_element.
                    if self
                        .path_element
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.path_element.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_path_element(|id| -> Rc<RefCell<PathElement>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.pattern_dirty {
            other.pattern.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in pattern, if it's not there add it to pattern.
                    if self
                        .pattern
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.pattern.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_pattern(|id| -> Rc<RefCell<Pattern>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_plugin_dirty {
            other.x_plugin.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_plugin, if it's not there add it to x_plugin.
                    if self
                        .x_plugin
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.x_plugin.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_x_plugin(|id| -> Rc<RefCell<XPlugin>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_print_dirty {
            other.x_print.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_print, if it's not there add it to x_print.
                    if self
                        .x_print
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.x_print.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_x_print(|id| -> Rc<RefCell<XPrint>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.range_expression_dirty {
            other.range_expression.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in range_expression, if it's not there add it to range_expression.
                    if self
                        .range_expression
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.range_expression.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_range_expression(|id| -> Rc<RefCell<RangeExpression>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.result_statement_dirty {
            other.result_statement.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in result_statement, if it's not there add it to result_statement.
                    if self
                        .result_statement
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.result_statement.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_result_statement(|id| -> Rc<RefCell<ResultStatement>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.x_return_dirty {
            other.x_return.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_return, if it's not there add it to x_return.
                    if self
                        .x_return
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.x_return.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_x_return(|id| -> Rc<RefCell<XReturn>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.span_dirty {
            other.span.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in span, if it's not there add it to span.
                    if self
                        .span
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.span.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_span(|id| -> Rc<RefCell<Span>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.statement_dirty {
            other.statement.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in statement, if it's not there add it to statement.
                    if self
                        .statement
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.statement.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_statement(|id| -> Rc<RefCell<Statement>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.static_method_call_dirty {
            other.static_method_call.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in static_method_call, if it's not there add it to static_method_call.
                    if self
                        .static_method_call
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.static_method_call.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_static_method_call(|id| -> Rc<RefCell<StaticMethodCall>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.string_bit_dirty {
            other.string_bit.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in string_bit, if it's not there add it to string_bit.
                    if self
                        .string_bit
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.string_bit.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_string_bit(|id| -> Rc<RefCell<StringBit>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.string_literal_dirty {
            other.string_literal.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in string_literal, if it's not there add it to string_literal.
                    if self
                        .string_literal
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.string_literal.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_string_literal(|id| -> Rc<RefCell<StringLiteral>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.woog_struct_dirty {
            other.woog_struct.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in woog_struct, if it's not there add it to woog_struct.
                    if self
                        .woog_struct
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.woog_struct.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_woog_struct(|id| -> Rc<RefCell<WoogStruct>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.struct_expression_dirty {
            other.struct_expression.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in struct_expression, if it's not there add it to struct_expression.
                    if self
                        .struct_expression
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.struct_expression.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_struct_expression(|id| -> Rc<RefCell<StructExpression>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.struct_field_dirty {
            other.struct_field.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in struct_field, if it's not there add it to struct_field.
                    if self
                        .struct_field
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.struct_field.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_struct_field(|id| -> Rc<RefCell<StructField>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.struct_generic_dirty {
            other.struct_generic.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in struct_generic, if it's not there add it to struct_generic.
                    if self
                        .struct_generic
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.struct_generic.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_struct_generic(|id| -> Rc<RefCell<StructGeneric>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.tuple_field_dirty {
            other.tuple_field.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in tuple_field, if it's not there add it to tuple_field.
                    if self
                        .tuple_field
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.tuple_field.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_tuple_field(|id| -> Rc<RefCell<TupleField>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.type_cast_dirty {
            other.type_cast.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in type_cast, if it's not there add it to type_cast.
                    if self
                        .type_cast
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.type_cast.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_type_cast(|id| -> Rc<RefCell<TypeCast>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.unary_dirty {
            other.unary.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in unary, if it's not there add it to unary.
                    if self
                        .unary
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.unary.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_unary(|id| -> Rc<RefCell<Unary>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.unit_dirty {
            other.unit.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in unit, if it's not there add it to unit.
                    if self
                        .unit
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.unit.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_unit(|id| -> Rc<RefCell<Unit>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.unnamed_field_expression_dirty {
            other.unnamed_field_expression.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in unnamed_field_expression, if it's not there add it to unnamed_field_expression.
                    if self
                        .unnamed_field_expression
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.unnamed_field_expression.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_unnamed_field_expression(
                            |id| -> Rc<RefCell<UnnamedFieldExpression>> {
                                if x.borrow().id != id {
                                    panic!("id mismatch");
                                }

                                x.clone()
                            },
                        );
                    }
                }
            });
        }

        if other.x_value_dirty {
            other.x_value.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in x_value, if it's not there add it to x_value.
                    if self
                        .x_value
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.x_value.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_x_value(|id| -> Rc<RefCell<XValue>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.value_type_dirty {
            other.value_type.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in value_type, if it's not there add it to value_type.
                    if self
                        .value_type
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.value_type.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_value_type(|id| -> Rc<RefCell<ValueType>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.variable_dirty {
            other.variable.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in variable, if it's not there add it to variable.
                    if self
                        .variable
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.variable.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_variable(|id| -> Rc<RefCell<Variable>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }

        if other.variable_expression_dirty {
            other.variable_expression.iter().for_each(|x| {
                if let Some(x) = x {
                    // Look for other in variable_expression, if it's not there add it to variable_expression.
                    if self
                        .variable_expression
                        .borrow()
                        .iter()
                        .find(|&y| if let Some(y) = y { *y == *x } else { false })
                        .is_none()
                    {
                        // let _index_ = self.variable_expression.borrow().len();
                        // if x.borrow().id != _index_ {
                        //     x.borrow_mut().id = _index_;
                        // }
                        self.inter_variable_expression(|id| -> Rc<RefCell<VariableExpression>> {
                            if x.borrow().id != id {
                                panic!("id mismatch");
                            }

                            x.clone()
                        });
                    }
                }
            });
        }
    }
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vec-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    #[inline]
    pub fn inter_argument<F>(&mut self, argument: F) -> Rc<RefCell<Argument>>
    where
        F: Fn(usize) -> Rc<RefCell<Argument>>,
    {
        let _index = if let Some(_index) = self.argument_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.argument.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.argument.push(None);
            _index
        };

        let argument = argument(_index);

        if let Some(Some(argument)) = self.argument.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *argument.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {argument:?}.");
            self.argument_free_list.push(_index);
            argument.clone()
        } else {
            tracing::debug!(target: "store", "interring {argument:?}.");
            self.argument[_index] = Some(argument.clone());
            argument
        }
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    #[inline]
    pub fn exhume_argument(&self, id: &usize) -> Option<Rc<RefCell<Argument>>> {
        match self.argument.get(*id) {
            Some(argument) => argument.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    #[inline]
    pub fn exorcise_argument(&mut self, id: &usize) -> Option<Rc<RefCell<Argument>>> {
        tracing::debug!(target: "store", "exorcising argument slot: {id}.");
        let result = self.argument[*id].take();
        self.argument_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    #[inline]
    pub fn iter_argument(&self) -> impl Iterator<Item = Rc<RefCell<Argument>>> + '_ {
        let len = self.argument.len();
        (0..len)
            .filter(|i| self.argument[*i].is_some())
            .map(move |i| {
                self.argument[i]
                    .as_ref()
                    .map(|argument| argument.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`AWait`] into the store.
    ///
    #[inline]
    pub fn inter_a_wait<F>(&mut self, a_wait: F) -> Rc<RefCell<AWait>>
    where
        F: Fn(usize) -> Rc<RefCell<AWait>>,
    {
        let _index = if let Some(_index) = self.a_wait_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.a_wait.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.a_wait.push(None);
            _index
        };

        let a_wait = a_wait(_index);

        if let Some(Some(a_wait)) = self.a_wait.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *a_wait.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {a_wait:?}.");
            self.a_wait_free_list.push(_index);
            a_wait.clone()
        } else {
            tracing::debug!(target: "store", "interring {a_wait:?}.");
            self.a_wait[_index] = Some(a_wait.clone());
            a_wait
        }
    }

    /// Exhume (get) [`AWait`] from the store.
    ///
    #[inline]
    pub fn exhume_a_wait(&self, id: &usize) -> Option<Rc<RefCell<AWait>>> {
        match self.a_wait.get(*id) {
            Some(a_wait) => a_wait.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`AWait`] from the store.
    ///
    #[inline]
    pub fn exorcise_a_wait(&mut self, id: &usize) -> Option<Rc<RefCell<AWait>>> {
        tracing::debug!(target: "store", "exorcising a_wait slot: {id}.");
        let result = self.a_wait[*id].take();
        self.a_wait_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AWait>`.
    ///
    #[inline]
    pub fn iter_a_wait(&self) -> impl Iterator<Item = Rc<RefCell<AWait>>> + '_ {
        let len = self.a_wait.len();
        (0..len)
            .filter(|i| self.a_wait[*i].is_some())
            .map(move |i| {
                self.a_wait[i]
                    .as_ref()
                    .map(|a_wait| a_wait.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    #[inline]
    pub fn inter_binary<F>(&mut self, binary: F) -> Rc<RefCell<Binary>>
    where
        F: Fn(usize) -> Rc<RefCell<Binary>>,
    {
        let _index = if let Some(_index) = self.binary_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.binary.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.binary.push(None);
            _index
        };

        let binary = binary(_index);

        if let Some(Some(binary)) = self.binary.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *binary.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {binary:?}.");
            self.binary_free_list.push(_index);
            binary.clone()
        } else {
            tracing::debug!(target: "store", "interring {binary:?}.");
            self.binary[_index] = Some(binary.clone());
            binary
        }
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    #[inline]
    pub fn exhume_binary(&self, id: &usize) -> Option<Rc<RefCell<Binary>>> {
        match self.binary.get(*id) {
            Some(binary) => binary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    #[inline]
    pub fn exorcise_binary(&mut self, id: &usize) -> Option<Rc<RefCell<Binary>>> {
        tracing::debug!(target: "store", "exorcising binary slot: {id}.");
        let result = self.binary[*id].take();
        self.binary_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    #[inline]
    pub fn iter_binary(&self) -> impl Iterator<Item = Rc<RefCell<Binary>>> + '_ {
        let len = self.binary.len();
        (0..len)
            .filter(|i| self.binary[*i].is_some())
            .map(move |i| {
                self.binary[i]
                    .as_ref()
                    .map(|binary| binary.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    #[inline]
    pub fn inter_block<F>(&mut self, block: F) -> Rc<RefCell<Block>>
    where
        F: Fn(usize) -> Rc<RefCell<Block>>,
    {
        let _index = if let Some(_index) = self.block_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.block.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.block.push(None);
            _index
        };

        let block = block(_index);

        if let Some(Some(block)) = self.block.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *block.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {block:?}.");
            self.block_free_list.push(_index);
            block.clone()
        } else {
            tracing::debug!(target: "store", "interring {block:?}.");
            self.block[_index] = Some(block.clone());
            block
        }
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    #[inline]
    pub fn exhume_block(&self, id: &usize) -> Option<Rc<RefCell<Block>>> {
        match self.block.get(*id) {
            Some(block) => block.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    #[inline]
    pub fn exorcise_block(&mut self, id: &usize) -> Option<Rc<RefCell<Block>>> {
        tracing::debug!(target: "store", "exorcising block slot: {id}.");
        let result = self.block[*id].take();
        self.block_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    #[inline]
    pub fn iter_block(&self) -> impl Iterator<Item = Rc<RefCell<Block>>> + '_ {
        let len = self.block.len();
        (0..len)
            .filter(|i| self.block[*i].is_some())
            .map(move |i| self.block[i].as_ref().map(|block| block.clone()).unwrap())
    }

    /// Inter (insert) [`Body`] into the store.
    ///
    #[inline]
    pub fn inter_body<F>(&mut self, body: F) -> Rc<RefCell<Body>>
    where
        F: Fn(usize) -> Rc<RefCell<Body>>,
    {
        let _index = if let Some(_index) = self.body_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.body.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.body.push(None);
            _index
        };

        let body = body(_index);

        if let Some(Some(body)) = self.body.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *body.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {body:?}.");
            self.body_free_list.push(_index);
            body.clone()
        } else {
            tracing::debug!(target: "store", "interring {body:?}.");
            self.body[_index] = Some(body.clone());
            body
        }
    }

    /// Exhume (get) [`Body`] from the store.
    ///
    #[inline]
    pub fn exhume_body(&self, id: &usize) -> Option<Rc<RefCell<Body>>> {
        match self.body.get(*id) {
            Some(body) => body.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Body`] from the store.
    ///
    #[inline]
    pub fn exorcise_body(&mut self, id: &usize) -> Option<Rc<RefCell<Body>>> {
        tracing::debug!(target: "store", "exorcising body slot: {id}.");
        let result = self.body[*id].take();
        self.body_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Body>`.
    ///
    #[inline]
    pub fn iter_body(&self) -> impl Iterator<Item = Rc<RefCell<Body>>> + '_ {
        let len = self.body.len();
        (0..len)
            .filter(|i| self.body[*i].is_some())
            .map(move |i| self.body[i].as_ref().map(|body| body.clone()).unwrap())
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_boolean_literal<F>(&mut self, boolean_literal: F) -> Rc<RefCell<BooleanLiteral>>
    where
        F: Fn(usize) -> Rc<RefCell<BooleanLiteral>>,
    {
        let _index = if let Some(_index) = self.boolean_literal_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.boolean_literal.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.boolean_literal.push(None);
            _index
        };

        let boolean_literal = boolean_literal(_index);

        if let Some(Some(boolean_literal)) = self.boolean_literal.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *boolean_literal.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {boolean_literal:?}.");
            self.boolean_literal_free_list.push(_index);
            boolean_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {boolean_literal:?}.");
            self.boolean_literal[_index] = Some(boolean_literal.clone());
            boolean_literal
        }
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_boolean_literal(&self, id: &usize) -> Option<Rc<RefCell<BooleanLiteral>>> {
        match self.boolean_literal.get(*id) {
            Some(boolean_literal) => boolean_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_boolean_literal(&mut self, id: &usize) -> Option<Rc<RefCell<BooleanLiteral>>> {
        tracing::debug!(target: "store", "exorcising boolean_literal slot: {id}.");
        let result = self.boolean_literal[*id].take();
        self.boolean_literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    #[inline]
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Rc<RefCell<BooleanLiteral>>> + '_ {
        let len = self.boolean_literal.len();
        (0..len)
            .filter(|i| self.boolean_literal[*i].is_some())
            .map(move |i| {
                self.boolean_literal[i]
                    .as_ref()
                    .map(|boolean_literal| boolean_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    #[inline]
    pub fn inter_boolean_operator<F>(&mut self, boolean_operator: F) -> Rc<RefCell<BooleanOperator>>
    where
        F: Fn(usize) -> Rc<RefCell<BooleanOperator>>,
    {
        let _index = if let Some(_index) = self.boolean_operator_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.boolean_operator.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.boolean_operator.push(None);
            _index
        };

        let boolean_operator = boolean_operator(_index);

        if let Some(Some(boolean_operator)) = self.boolean_operator.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *boolean_operator.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {boolean_operator:?}.");
            self.boolean_operator_free_list.push(_index);
            boolean_operator.clone()
        } else {
            tracing::debug!(target: "store", "interring {boolean_operator:?}.");
            self.boolean_operator[_index] = Some(boolean_operator.clone());
            boolean_operator
        }
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    #[inline]
    pub fn exhume_boolean_operator(&self, id: &usize) -> Option<Rc<RefCell<BooleanOperator>>> {
        match self.boolean_operator.get(*id) {
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
    ) -> Option<Rc<RefCell<BooleanOperator>>> {
        tracing::debug!(target: "store", "exorcising boolean_operator slot: {id}.");
        let result = self.boolean_operator[*id].take();
        self.boolean_operator_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    #[inline]
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = Rc<RefCell<BooleanOperator>>> + '_ {
        let len = self.boolean_operator.len();
        (0..len)
            .filter(|i| self.boolean_operator[*i].is_some())
            .map(move |i| {
                self.boolean_operator[i]
                    .as_ref()
                    .map(|boolean_operator| boolean_operator.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    #[inline]
    pub fn inter_call<F>(&mut self, call: F) -> Rc<RefCell<Call>>
    where
        F: Fn(usize) -> Rc<RefCell<Call>>,
    {
        let _index = if let Some(_index) = self.call_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.call.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.call.push(None);
            _index
        };

        let call = call(_index);

        if let Some(Some(call)) = self.call.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *call.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {call:?}.");
            self.call_free_list.push(_index);
            call.clone()
        } else {
            tracing::debug!(target: "store", "interring {call:?}.");
            self.call[_index] = Some(call.clone());
            call
        }
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    #[inline]
    pub fn exhume_call(&self, id: &usize) -> Option<Rc<RefCell<Call>>> {
        match self.call.get(*id) {
            Some(call) => call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    #[inline]
    pub fn exorcise_call(&mut self, id: &usize) -> Option<Rc<RefCell<Call>>> {
        tracing::debug!(target: "store", "exorcising call slot: {id}.");
        let result = self.call[*id].take();
        self.call_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    #[inline]
    pub fn iter_call(&self) -> impl Iterator<Item = Rc<RefCell<Call>>> + '_ {
        let len = self.call.len();
        (0..len)
            .filter(|i| self.call[*i].is_some())
            .map(move |i| self.call[i].as_ref().map(|call| call.clone()).unwrap())
    }

    /// Inter (insert) [`CharLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_char_literal<F>(&mut self, char_literal: F) -> Rc<RefCell<CharLiteral>>
    where
        F: Fn(usize) -> Rc<RefCell<CharLiteral>>,
    {
        let _index = if let Some(_index) = self.char_literal_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.char_literal.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.char_literal.push(None);
            _index
        };

        let char_literal = char_literal(_index);

        if let Some(Some(char_literal)) = self.char_literal.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *char_literal.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {char_literal:?}.");
            self.char_literal_free_list.push(_index);
            char_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {char_literal:?}.");
            self.char_literal[_index] = Some(char_literal.clone());
            char_literal
        }
    }

    /// Exhume (get) [`CharLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_char_literal(&self, id: &usize) -> Option<Rc<RefCell<CharLiteral>>> {
        match self.char_literal.get(*id) {
            Some(char_literal) => char_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`CharLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_char_literal(&mut self, id: &usize) -> Option<Rc<RefCell<CharLiteral>>> {
        tracing::debug!(target: "store", "exorcising char_literal slot: {id}.");
        let result = self.char_literal[*id].take();
        self.char_literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, CharLiteral>`.
    ///
    #[inline]
    pub fn iter_char_literal(&self) -> impl Iterator<Item = Rc<RefCell<CharLiteral>>> + '_ {
        let len = self.char_literal.len();
        (0..len)
            .filter(|i| self.char_literal[*i].is_some())
            .map(move |i| {
                self.char_literal[i]
                    .as_ref()
                    .map(|char_literal| char_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    #[inline]
    pub fn inter_comparison<F>(&mut self, comparison: F) -> Rc<RefCell<Comparison>>
    where
        F: Fn(usize) -> Rc<RefCell<Comparison>>,
    {
        let _index = if let Some(_index) = self.comparison_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.comparison.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.comparison.push(None);
            _index
        };

        let comparison = comparison(_index);

        if let Some(Some(comparison)) = self.comparison.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *comparison.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {comparison:?}.");
            self.comparison_free_list.push(_index);
            comparison.clone()
        } else {
            tracing::debug!(target: "store", "interring {comparison:?}.");
            self.comparison[_index] = Some(comparison.clone());
            comparison
        }
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    #[inline]
    pub fn exhume_comparison(&self, id: &usize) -> Option<Rc<RefCell<Comparison>>> {
        match self.comparison.get(*id) {
            Some(comparison) => comparison.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    #[inline]
    pub fn exorcise_comparison(&mut self, id: &usize) -> Option<Rc<RefCell<Comparison>>> {
        tracing::debug!(target: "store", "exorcising comparison slot: {id}.");
        let result = self.comparison[*id].take();
        self.comparison_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    #[inline]
    pub fn iter_comparison(&self) -> impl Iterator<Item = Rc<RefCell<Comparison>>> + '_ {
        let len = self.comparison.len();
        (0..len)
            .filter(|i| self.comparison[*i].is_some())
            .map(move |i| {
                self.comparison[i]
                    .as_ref()
                    .map(|comparison| comparison.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`DataStructure`] into the store.
    ///
    #[inline]
    pub fn inter_data_structure<F>(&mut self, data_structure: F) -> Rc<RefCell<DataStructure>>
    where
        F: Fn(usize) -> Rc<RefCell<DataStructure>>,
    {
        let _index = if let Some(_index) = self.data_structure_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.data_structure.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.data_structure.push(None);
            _index
        };

        let data_structure = data_structure(_index);

        if let Some(Some(data_structure)) = self.data_structure.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *data_structure.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {data_structure:?}.");
            self.data_structure_free_list.push(_index);
            data_structure.clone()
        } else {
            tracing::debug!(target: "store", "interring {data_structure:?}.");
            self.data_structure[_index] = Some(data_structure.clone());
            data_structure
        }
    }

    /// Exhume (get) [`DataStructure`] from the store.
    ///
    #[inline]
    pub fn exhume_data_structure(&self, id: &usize) -> Option<Rc<RefCell<DataStructure>>> {
        match self.data_structure.get(*id) {
            Some(data_structure) => data_structure.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`DataStructure`] from the store.
    ///
    #[inline]
    pub fn exorcise_data_structure(&mut self, id: &usize) -> Option<Rc<RefCell<DataStructure>>> {
        tracing::debug!(target: "store", "exorcising data_structure slot: {id}.");
        let result = self.data_structure[*id].take();
        self.data_structure_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DataStructure>`.
    ///
    #[inline]
    pub fn iter_data_structure(&self) -> impl Iterator<Item = Rc<RefCell<DataStructure>>> + '_ {
        let len = self.data_structure.len();
        (0..len)
            .filter(|i| self.data_structure[*i].is_some())
            .map(move |i| {
                self.data_structure[i]
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
    ) -> Rc<RefCell<DwarfSourceFile>>
    where
        F: Fn(usize) -> Rc<RefCell<DwarfSourceFile>>,
    {
        let _index = if let Some(_index) = self.dwarf_source_file_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.dwarf_source_file.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.dwarf_source_file.push(None);
            _index
        };

        let dwarf_source_file = dwarf_source_file(_index);

        if let Some(Some(dwarf_source_file)) = self.dwarf_source_file.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *dwarf_source_file.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {dwarf_source_file:?}.");
            self.dwarf_source_file_free_list.push(_index);
            dwarf_source_file.clone()
        } else {
            tracing::debug!(target: "store", "interring {dwarf_source_file:?}.");
            self.dwarf_source_file[_index] = Some(dwarf_source_file.clone());
            dwarf_source_file
        }
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    #[inline]
    pub fn exhume_dwarf_source_file(&self, id: &usize) -> Option<Rc<RefCell<DwarfSourceFile>>> {
        match self.dwarf_source_file.get(*id) {
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
    ) -> Option<Rc<RefCell<DwarfSourceFile>>> {
        tracing::debug!(target: "store", "exorcising dwarf_source_file slot: {id}.");
        let result = self.dwarf_source_file[*id].take();
        self.dwarf_source_file_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    #[inline]
    pub fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<DwarfSourceFile>>> + '_ {
        let len = self.dwarf_source_file.len();
        (0..len)
            .filter(|i| self.dwarf_source_file[*i].is_some())
            .map(move |i| {
                self.dwarf_source_file[i]
                    .as_ref()
                    .map(|dwarf_source_file| dwarf_source_file.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`EnumField`] into the store.
    ///
    #[inline]
    pub fn inter_enum_field<F>(&mut self, enum_field: F) -> Rc<RefCell<EnumField>>
    where
        F: Fn(usize) -> Rc<RefCell<EnumField>>,
    {
        let _index = if let Some(_index) = self.enum_field_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enum_field.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.enum_field.push(None);
            _index
        };

        let enum_field = enum_field(_index);

        if let Some(Some(enum_field)) = self.enum_field.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *enum_field.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {enum_field:?}.");
            self.enum_field_free_list.push(_index);
            enum_field.clone()
        } else {
            tracing::debug!(target: "store", "interring {enum_field:?}.");
            self.enum_field[_index] = Some(enum_field.clone());
            enum_field
        }
    }

    /// Exhume (get) [`EnumField`] from the store.
    ///
    #[inline]
    pub fn exhume_enum_field(&self, id: &usize) -> Option<Rc<RefCell<EnumField>>> {
        match self.enum_field.get(*id) {
            Some(enum_field) => enum_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`EnumField`] from the store.
    ///
    #[inline]
    pub fn exorcise_enum_field(&mut self, id: &usize) -> Option<Rc<RefCell<EnumField>>> {
        tracing::debug!(target: "store", "exorcising enum_field slot: {id}.");
        let result = self.enum_field[*id].take();
        self.enum_field_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumField>`.
    ///
    #[inline]
    pub fn iter_enum_field(&self) -> impl Iterator<Item = Rc<RefCell<EnumField>>> + '_ {
        let len = self.enum_field.len();
        (0..len)
            .filter(|i| self.enum_field[*i].is_some())
            .map(move |i| {
                self.enum_field[i]
                    .as_ref()
                    .map(|enum_field| enum_field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`EnumGeneric`] into the store.
    ///
    #[inline]
    pub fn inter_enum_generic<F>(&mut self, enum_generic: F) -> Rc<RefCell<EnumGeneric>>
    where
        F: Fn(usize) -> Rc<RefCell<EnumGeneric>>,
    {
        let _index = if let Some(_index) = self.enum_generic_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enum_generic.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.enum_generic.push(None);
            _index
        };

        let enum_generic = enum_generic(_index);

        if let Some(Some(enum_generic)) = self.enum_generic.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *enum_generic.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {enum_generic:?}.");
            self.enum_generic_free_list.push(_index);
            enum_generic.clone()
        } else {
            tracing::debug!(target: "store", "interring {enum_generic:?}.");
            self.enum_generic[_index] = Some(enum_generic.clone());
            enum_generic
        }
    }

    /// Exhume (get) [`EnumGeneric`] from the store.
    ///
    #[inline]
    pub fn exhume_enum_generic(&self, id: &usize) -> Option<Rc<RefCell<EnumGeneric>>> {
        match self.enum_generic.get(*id) {
            Some(enum_generic) => enum_generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`EnumGeneric`] from the store.
    ///
    #[inline]
    pub fn exorcise_enum_generic(&mut self, id: &usize) -> Option<Rc<RefCell<EnumGeneric>>> {
        tracing::debug!(target: "store", "exorcising enum_generic slot: {id}.");
        let result = self.enum_generic[*id].take();
        self.enum_generic_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGeneric>`.
    ///
    #[inline]
    pub fn iter_enum_generic(&self) -> impl Iterator<Item = Rc<RefCell<EnumGeneric>>> + '_ {
        let len = self.enum_generic.len();
        (0..len)
            .filter(|i| self.enum_generic[*i].is_some())
            .map(move |i| {
                self.enum_generic[i]
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
    ) -> Rc<RefCell<EnumGenericType>>
    where
        F: Fn(usize) -> Rc<RefCell<EnumGenericType>>,
    {
        let _index = if let Some(_index) = self.enum_generic_type_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enum_generic_type.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.enum_generic_type.push(None);
            _index
        };

        let enum_generic_type = enum_generic_type(_index);

        if let Some(Some(enum_generic_type)) = self.enum_generic_type.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *enum_generic_type.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {enum_generic_type:?}.");
            self.enum_generic_type_free_list.push(_index);
            enum_generic_type.clone()
        } else {
            tracing::debug!(target: "store", "interring {enum_generic_type:?}.");
            self.enum_generic_type[_index] = Some(enum_generic_type.clone());
            enum_generic_type
        }
    }

    /// Exhume (get) [`EnumGenericType`] from the store.
    ///
    #[inline]
    pub fn exhume_enum_generic_type(&self, id: &usize) -> Option<Rc<RefCell<EnumGenericType>>> {
        match self.enum_generic_type.get(*id) {
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
    ) -> Option<Rc<RefCell<EnumGenericType>>> {
        tracing::debug!(target: "store", "exorcising enum_generic_type slot: {id}.");
        let result = self.enum_generic_type[*id].take();
        self.enum_generic_type_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGenericType>`.
    ///
    #[inline]
    pub fn iter_enum_generic_type(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<EnumGenericType>>> + '_ {
        let len = self.enum_generic_type.len();
        (0..len)
            .filter(|i| self.enum_generic_type[*i].is_some())
            .map(move |i| {
                self.enum_generic_type[i]
                    .as_ref()
                    .map(|enum_generic_type| enum_generic_type.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    #[inline]
    pub fn inter_enumeration<F>(&mut self, enumeration: F) -> Rc<RefCell<Enumeration>>
    where
        F: Fn(usize) -> Rc<RefCell<Enumeration>>,
    {
        let _index = if let Some(_index) = self.enumeration_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enumeration.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.enumeration.push(None);
            _index
        };

        let enumeration = enumeration(_index);

        let enumeration = if let Some(Some(enumeration)) = self.enumeration.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *enumeration.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {enumeration:?}.");
            self.enumeration_free_list.push(_index);
            enumeration.clone()
        } else {
            tracing::debug!(target: "store", "interring {enumeration:?}.");
            self.enumeration[_index] = Some(enumeration.clone());
            enumeration
        };
        self.enumeration_id_by_name.insert(
            enumeration.borrow().name.to_owned(),
            enumeration.borrow().id,
        );
        enumeration
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    #[inline]
    pub fn exhume_enumeration(&self, id: &usize) -> Option<Rc<RefCell<Enumeration>>> {
        match self.enumeration.get(*id) {
            Some(enumeration) => enumeration.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    #[inline]
    pub fn exorcise_enumeration(&mut self, id: &usize) -> Option<Rc<RefCell<Enumeration>>> {
        tracing::debug!(target: "store", "exorcising enumeration slot: {id}.");
        let result = self.enumeration[*id].take();
        self.enumeration_free_list.push(*id);
        result
    }

    /// Exorcise [`Enumeration`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_enumeration_id_by_name(&self, name: &str) -> Option<usize> {
        self.enumeration_id_by_name
            .get(name)
            .map(|enumeration| *enumeration)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    #[inline]
    pub fn iter_enumeration(&self) -> impl Iterator<Item = Rc<RefCell<Enumeration>>> + '_ {
        let len = self.enumeration.len();
        (0..len)
            .filter(|i| self.enumeration[*i].is_some())
            .map(move |i| {
                self.enumeration[i]
                    .as_ref()
                    .map(|enumeration| enumeration.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    #[inline]
    pub fn inter_expression<F>(&mut self, expression: F) -> Rc<RefCell<Expression>>
    where
        F: Fn(usize) -> Rc<RefCell<Expression>>,
    {
        let _index = if let Some(_index) = self.expression_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.expression.push(None);
            _index
        };

        let expression = expression(_index);

        if let Some(Some(expression)) = self.expression.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *expression.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {expression:?}.");
            self.expression_free_list.push(_index);
            expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {expression:?}.");
            self.expression[_index] = Some(expression.clone());
            expression
        }
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    #[inline]
    pub fn exhume_expression(&self, id: &usize) -> Option<Rc<RefCell<Expression>>> {
        match self.expression.get(*id) {
            Some(expression) => expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    #[inline]
    pub fn exorcise_expression(&mut self, id: &usize) -> Option<Rc<RefCell<Expression>>> {
        tracing::debug!(target: "store", "exorcising expression slot: {id}.");
        let result = self.expression[*id].take();
        self.expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    #[inline]
    pub fn iter_expression(&self) -> impl Iterator<Item = Rc<RefCell<Expression>>> + '_ {
        let len = self.expression.len();
        (0..len)
            .filter(|i| self.expression[*i].is_some())
            .map(move |i| {
                self.expression[i]
                    .as_ref()
                    .map(|expression| expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ExpressionBit`] into the store.
    ///
    #[inline]
    pub fn inter_expression_bit<F>(&mut self, expression_bit: F) -> Rc<RefCell<ExpressionBit>>
    where
        F: Fn(usize) -> Rc<RefCell<ExpressionBit>>,
    {
        let _index = if let Some(_index) = self.expression_bit_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression_bit.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.expression_bit.push(None);
            _index
        };

        let expression_bit = expression_bit(_index);

        if let Some(Some(expression_bit)) = self.expression_bit.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *expression_bit.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {expression_bit:?}.");
            self.expression_bit_free_list.push(_index);
            expression_bit.clone()
        } else {
            tracing::debug!(target: "store", "interring {expression_bit:?}.");
            self.expression_bit[_index] = Some(expression_bit.clone());
            expression_bit
        }
    }

    /// Exhume (get) [`ExpressionBit`] from the store.
    ///
    #[inline]
    pub fn exhume_expression_bit(&self, id: &usize) -> Option<Rc<RefCell<ExpressionBit>>> {
        match self.expression_bit.get(*id) {
            Some(expression_bit) => expression_bit.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ExpressionBit`] from the store.
    ///
    #[inline]
    pub fn exorcise_expression_bit(&mut self, id: &usize) -> Option<Rc<RefCell<ExpressionBit>>> {
        tracing::debug!(target: "store", "exorcising expression_bit slot: {id}.");
        let result = self.expression_bit[*id].take();
        self.expression_bit_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionBit>`.
    ///
    #[inline]
    pub fn iter_expression_bit(&self) -> impl Iterator<Item = Rc<RefCell<ExpressionBit>>> + '_ {
        let len = self.expression_bit.len();
        (0..len)
            .filter(|i| self.expression_bit[*i].is_some())
            .map(move |i| {
                self.expression_bit[i]
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
    ) -> Rc<RefCell<ExpressionStatement>>
    where
        F: Fn(usize) -> Rc<RefCell<ExpressionStatement>>,
    {
        let _index = if let Some(_index) = self.expression_statement_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression_statement.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.expression_statement.push(None);
            _index
        };

        let expression_statement = expression_statement(_index);

        if let Some(Some(expression_statement)) = self.expression_statement.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *expression_statement.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {expression_statement:?}.");
            self.expression_statement_free_list.push(_index);
            expression_statement.clone()
        } else {
            tracing::debug!(target: "store", "interring {expression_statement:?}.");
            self.expression_statement[_index] = Some(expression_statement.clone());
            expression_statement
        }
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    #[inline]
    pub fn exhume_expression_statement(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<ExpressionStatement>>> {
        match self.expression_statement.get(*id) {
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
    ) -> Option<Rc<RefCell<ExpressionStatement>>> {
        tracing::debug!(target: "store", "exorcising expression_statement slot: {id}.");
        let result = self.expression_statement[*id].take();
        self.expression_statement_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    #[inline]
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<ExpressionStatement>>> + '_ {
        let len = self.expression_statement.len();
        (0..len)
            .filter(|i| self.expression_statement[*i].is_some())
            .map(move |i| {
                self.expression_statement[i]
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
    ) -> Rc<RefCell<ExternalImplementation>>
    where
        F: Fn(usize) -> Rc<RefCell<ExternalImplementation>>,
    {
        let _index = if let Some(_index) = self.external_implementation_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.external_implementation.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.external_implementation.push(None);
            _index
        };

        let external_implementation = external_implementation(_index);

        if let Some(Some(external_implementation)) =
            self.external_implementation.iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.borrow() == *external_implementation.borrow()
                } else {
                    false
                }
            })
        {
            tracing::debug!(target: "store", "found duplicate {external_implementation:?}.");
            self.external_implementation_free_list.push(_index);
            external_implementation.clone()
        } else {
            tracing::debug!(target: "store", "interring {external_implementation:?}.");
            self.external_implementation[_index] = Some(external_implementation.clone());
            external_implementation
        }
    }

    /// Exhume (get) [`ExternalImplementation`] from the store.
    ///
    #[inline]
    pub fn exhume_external_implementation(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<ExternalImplementation>>> {
        match self.external_implementation.get(*id) {
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
    ) -> Option<Rc<RefCell<ExternalImplementation>>> {
        tracing::debug!(target: "store", "exorcising external_implementation slot: {id}.");
        let result = self.external_implementation[*id].take();
        self.external_implementation_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExternalImplementation>`.
    ///
    #[inline]
    pub fn iter_external_implementation(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<ExternalImplementation>>> + '_ {
        let len = self.external_implementation.len();
        (0..len)
            .filter(|i| self.external_implementation[*i].is_some())
            .map(move |i| {
                self.external_implementation[i]
                    .as_ref()
                    .map(|external_implementation| external_implementation.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    #[inline]
    pub fn inter_field<F>(&mut self, field: F) -> Rc<RefCell<Field>>
    where
        F: Fn(usize) -> Rc<RefCell<Field>>,
    {
        let _index = if let Some(_index) = self.field_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.field.push(None);
            _index
        };

        let field = field(_index);

        let field = if let Some(Some(field)) = self.field.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *field.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {field:?}.");
            self.field_free_list.push(_index);
            field.clone()
        } else {
            tracing::debug!(target: "store", "interring {field:?}.");
            self.field[_index] = Some(field.clone());
            field
        };
        self.field_id_by_name
            .insert(field.borrow().name.to_owned(), field.borrow().id);
        field
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    #[inline]
    pub fn exhume_field(&self, id: &usize) -> Option<Rc<RefCell<Field>>> {
        match self.field.get(*id) {
            Some(field) => field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    #[inline]
    pub fn exorcise_field(&mut self, id: &usize) -> Option<Rc<RefCell<Field>>> {
        tracing::debug!(target: "store", "exorcising field slot: {id}.");
        let result = self.field[*id].take();
        self.field_free_list.push(*id);
        result
    }

    /// Exorcise [`Field`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<usize> {
        self.field_id_by_name.get(name).map(|field| *field)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    #[inline]
    pub fn iter_field(&self) -> impl Iterator<Item = Rc<RefCell<Field>>> + '_ {
        let len = self.field.len();
        (0..len)
            .filter(|i| self.field[*i].is_some())
            .map(move |i| self.field[i].as_ref().map(|field| field.clone()).unwrap())
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    #[inline]
    pub fn inter_field_access<F>(&mut self, field_access: F) -> Rc<RefCell<FieldAccess>>
    where
        F: Fn(usize) -> Rc<RefCell<FieldAccess>>,
    {
        let _index = if let Some(_index) = self.field_access_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_access.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.field_access.push(None);
            _index
        };

        let field_access = field_access(_index);

        if let Some(Some(field_access)) = self.field_access.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *field_access.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {field_access:?}.");
            self.field_access_free_list.push(_index);
            field_access.clone()
        } else {
            tracing::debug!(target: "store", "interring {field_access:?}.");
            self.field_access[_index] = Some(field_access.clone());
            field_access
        }
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    #[inline]
    pub fn exhume_field_access(&self, id: &usize) -> Option<Rc<RefCell<FieldAccess>>> {
        match self.field_access.get(*id) {
            Some(field_access) => field_access.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    #[inline]
    pub fn exorcise_field_access(&mut self, id: &usize) -> Option<Rc<RefCell<FieldAccess>>> {
        tracing::debug!(target: "store", "exorcising field_access slot: {id}.");
        let result = self.field_access[*id].take();
        self.field_access_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    #[inline]
    pub fn iter_field_access(&self) -> impl Iterator<Item = Rc<RefCell<FieldAccess>>> + '_ {
        let len = self.field_access.len();
        (0..len)
            .filter(|i| self.field_access[*i].is_some())
            .map(move |i| {
                self.field_access[i]
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
    ) -> Rc<RefCell<FieldAccessTarget>>
    where
        F: Fn(usize) -> Rc<RefCell<FieldAccessTarget>>,
    {
        let _index = if let Some(_index) = self.field_access_target_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_access_target.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.field_access_target.push(None);
            _index
        };

        let field_access_target = field_access_target(_index);

        if let Some(Some(field_access_target)) = self.field_access_target.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *field_access_target.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {field_access_target:?}.");
            self.field_access_target_free_list.push(_index);
            field_access_target.clone()
        } else {
            tracing::debug!(target: "store", "interring {field_access_target:?}.");
            self.field_access_target[_index] = Some(field_access_target.clone());
            field_access_target
        }
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    #[inline]
    pub fn exhume_field_access_target(&self, id: &usize) -> Option<Rc<RefCell<FieldAccessTarget>>> {
        match self.field_access_target.get(*id) {
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
    ) -> Option<Rc<RefCell<FieldAccessTarget>>> {
        tracing::debug!(target: "store", "exorcising field_access_target slot: {id}.");
        let result = self.field_access_target[*id].take();
        self.field_access_target_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    #[inline]
    pub fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<FieldAccessTarget>>> + '_ {
        let len = self.field_access_target.len();
        (0..len)
            .filter(|i| self.field_access_target[*i].is_some())
            .map(move |i| {
                self.field_access_target[i]
                    .as_ref()
                    .map(|field_access_target| field_access_target.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    #[inline]
    pub fn inter_field_expression<F>(&mut self, field_expression: F) -> Rc<RefCell<FieldExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<FieldExpression>>,
    {
        let _index = if let Some(_index) = self.field_expression_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_expression.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.field_expression.push(None);
            _index
        };

        let field_expression = field_expression(_index);

        if let Some(Some(field_expression)) = self.field_expression.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *field_expression.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {field_expression:?}.");
            self.field_expression_free_list.push(_index);
            field_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {field_expression:?}.");
            self.field_expression[_index] = Some(field_expression.clone());
            field_expression
        }
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_field_expression(&self, id: &usize) -> Option<Rc<RefCell<FieldExpression>>> {
        match self.field_expression.get(*id) {
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
    ) -> Option<Rc<RefCell<FieldExpression>>> {
        tracing::debug!(target: "store", "exorcising field_expression slot: {id}.");
        let result = self.field_expression[*id].take();
        self.field_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    #[inline]
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Rc<RefCell<FieldExpression>>> + '_ {
        let len = self.field_expression.len();
        (0..len)
            .filter(|i| self.field_expression[*i].is_some())
            .map(move |i| {
                self.field_expression[i]
                    .as_ref()
                    .map(|field_expression| field_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_float_literal<F>(&mut self, float_literal: F) -> Rc<RefCell<FloatLiteral>>
    where
        F: Fn(usize) -> Rc<RefCell<FloatLiteral>>,
    {
        let _index = if let Some(_index) = self.float_literal_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.float_literal.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.float_literal.push(None);
            _index
        };

        let float_literal = float_literal(_index);

        if let Some(Some(float_literal)) = self.float_literal.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *float_literal.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {float_literal:?}.");
            self.float_literal_free_list.push(_index);
            float_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {float_literal:?}.");
            self.float_literal[_index] = Some(float_literal.clone());
            float_literal
        }
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_float_literal(&self, id: &usize) -> Option<Rc<RefCell<FloatLiteral>>> {
        match self.float_literal.get(*id) {
            Some(float_literal) => float_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_float_literal(&mut self, id: &usize) -> Option<Rc<RefCell<FloatLiteral>>> {
        tracing::debug!(target: "store", "exorcising float_literal slot: {id}.");
        let result = self.float_literal[*id].take();
        self.float_literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    #[inline]
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Rc<RefCell<FloatLiteral>>> + '_ {
        let len = self.float_literal.len();
        (0..len)
            .filter(|i| self.float_literal[*i].is_some())
            .map(move |i| {
                self.float_literal[i]
                    .as_ref()
                    .map(|float_literal| float_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    #[inline]
    pub fn inter_for_loop<F>(&mut self, for_loop: F) -> Rc<RefCell<ForLoop>>
    where
        F: Fn(usize) -> Rc<RefCell<ForLoop>>,
    {
        let _index = if let Some(_index) = self.for_loop_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.for_loop.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.for_loop.push(None);
            _index
        };

        let for_loop = for_loop(_index);

        if let Some(Some(for_loop)) = self.for_loop.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *for_loop.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {for_loop:?}.");
            self.for_loop_free_list.push(_index);
            for_loop.clone()
        } else {
            tracing::debug!(target: "store", "interring {for_loop:?}.");
            self.for_loop[_index] = Some(for_loop.clone());
            for_loop
        }
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    #[inline]
    pub fn exhume_for_loop(&self, id: &usize) -> Option<Rc<RefCell<ForLoop>>> {
        match self.for_loop.get(*id) {
            Some(for_loop) => for_loop.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    #[inline]
    pub fn exorcise_for_loop(&mut self, id: &usize) -> Option<Rc<RefCell<ForLoop>>> {
        tracing::debug!(target: "store", "exorcising for_loop slot: {id}.");
        let result = self.for_loop[*id].take();
        self.for_loop_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    #[inline]
    pub fn iter_for_loop(&self) -> impl Iterator<Item = Rc<RefCell<ForLoop>>> + '_ {
        let len = self.for_loop.len();
        (0..len)
            .filter(|i| self.for_loop[*i].is_some())
            .map(move |i| {
                self.for_loop[i]
                    .as_ref()
                    .map(|for_loop| for_loop.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FormatBit`] into the store.
    ///
    #[inline]
    pub fn inter_format_bit<F>(&mut self, format_bit: F) -> Rc<RefCell<FormatBit>>
    where
        F: Fn(usize) -> Rc<RefCell<FormatBit>>,
    {
        let _index = if let Some(_index) = self.format_bit_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.format_bit.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.format_bit.push(None);
            _index
        };

        let format_bit = format_bit(_index);

        if let Some(Some(format_bit)) = self.format_bit.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *format_bit.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {format_bit:?}.");
            self.format_bit_free_list.push(_index);
            format_bit.clone()
        } else {
            tracing::debug!(target: "store", "interring {format_bit:?}.");
            self.format_bit[_index] = Some(format_bit.clone());
            format_bit
        }
    }

    /// Exhume (get) [`FormatBit`] from the store.
    ///
    #[inline]
    pub fn exhume_format_bit(&self, id: &usize) -> Option<Rc<RefCell<FormatBit>>> {
        match self.format_bit.get(*id) {
            Some(format_bit) => format_bit.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FormatBit`] from the store.
    ///
    #[inline]
    pub fn exorcise_format_bit(&mut self, id: &usize) -> Option<Rc<RefCell<FormatBit>>> {
        tracing::debug!(target: "store", "exorcising format_bit slot: {id}.");
        let result = self.format_bit[*id].take();
        self.format_bit_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FormatBit>`.
    ///
    #[inline]
    pub fn iter_format_bit(&self) -> impl Iterator<Item = Rc<RefCell<FormatBit>>> + '_ {
        let len = self.format_bit.len();
        (0..len)
            .filter(|i| self.format_bit[*i].is_some())
            .map(move |i| {
                self.format_bit[i]
                    .as_ref()
                    .map(|format_bit| format_bit.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FormatString`] into the store.
    ///
    #[inline]
    pub fn inter_format_string<F>(&mut self, format_string: F) -> Rc<RefCell<FormatString>>
    where
        F: Fn(usize) -> Rc<RefCell<FormatString>>,
    {
        let _index = if let Some(_index) = self.format_string_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.format_string.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.format_string.push(None);
            _index
        };

        let format_string = format_string(_index);

        if let Some(Some(format_string)) = self.format_string.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *format_string.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {format_string:?}.");
            self.format_string_free_list.push(_index);
            format_string.clone()
        } else {
            tracing::debug!(target: "store", "interring {format_string:?}.");
            self.format_string[_index] = Some(format_string.clone());
            format_string
        }
    }

    /// Exhume (get) [`FormatString`] from the store.
    ///
    #[inline]
    pub fn exhume_format_string(&self, id: &usize) -> Option<Rc<RefCell<FormatString>>> {
        match self.format_string.get(*id) {
            Some(format_string) => format_string.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FormatString`] from the store.
    ///
    #[inline]
    pub fn exorcise_format_string(&mut self, id: &usize) -> Option<Rc<RefCell<FormatString>>> {
        tracing::debug!(target: "store", "exorcising format_string slot: {id}.");
        let result = self.format_string[*id].take();
        self.format_string_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FormatString>`.
    ///
    #[inline]
    pub fn iter_format_string(&self) -> impl Iterator<Item = Rc<RefCell<FormatString>>> + '_ {
        let len = self.format_string.len();
        (0..len)
            .filter(|i| self.format_string[*i].is_some())
            .map(move |i| {
                self.format_string[i]
                    .as_ref()
                    .map(|format_string| format_string.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FuncGeneric`] into the store.
    ///
    #[inline]
    pub fn inter_func_generic<F>(&mut self, func_generic: F) -> Rc<RefCell<FuncGeneric>>
    where
        F: Fn(usize) -> Rc<RefCell<FuncGeneric>>,
    {
        let _index = if let Some(_index) = self.func_generic_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.func_generic.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.func_generic.push(None);
            _index
        };

        let func_generic = func_generic(_index);

        if let Some(Some(func_generic)) = self.func_generic.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *func_generic.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {func_generic:?}.");
            self.func_generic_free_list.push(_index);
            func_generic.clone()
        } else {
            tracing::debug!(target: "store", "interring {func_generic:?}.");
            self.func_generic[_index] = Some(func_generic.clone());
            func_generic
        }
    }

    /// Exhume (get) [`FuncGeneric`] from the store.
    ///
    #[inline]
    pub fn exhume_func_generic(&self, id: &usize) -> Option<Rc<RefCell<FuncGeneric>>> {
        match self.func_generic.get(*id) {
            Some(func_generic) => func_generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FuncGeneric`] from the store.
    ///
    #[inline]
    pub fn exorcise_func_generic(&mut self, id: &usize) -> Option<Rc<RefCell<FuncGeneric>>> {
        tracing::debug!(target: "store", "exorcising func_generic slot: {id}.");
        let result = self.func_generic[*id].take();
        self.func_generic_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FuncGeneric>`.
    ///
    #[inline]
    pub fn iter_func_generic(&self) -> impl Iterator<Item = Rc<RefCell<FuncGeneric>>> + '_ {
        let len = self.func_generic.len();
        (0..len)
            .filter(|i| self.func_generic[*i].is_some())
            .map(move |i| {
                self.func_generic[i]
                    .as_ref()
                    .map(|func_generic| func_generic.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    #[inline]
    pub fn inter_function<F>(&mut self, function: F) -> Rc<RefCell<Function>>
    where
        F: Fn(usize) -> Rc<RefCell<Function>>,
    {
        let _index = if let Some(_index) = self.function_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.function.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.function.push(None);
            _index
        };

        let function = function(_index);

        let function = if let Some(Some(function)) = self.function.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *function.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {function:?}.");
            self.function_free_list.push(_index);
            function.clone()
        } else {
            tracing::debug!(target: "store", "interring {function:?}.");
            self.function[_index] = Some(function.clone());
            function
        };
        self.function_id_by_name
            .insert(function.borrow().name.to_owned(), function.borrow().id);
        function
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    #[inline]
    pub fn exhume_function(&self, id: &usize) -> Option<Rc<RefCell<Function>>> {
        match self.function.get(*id) {
            Some(function) => function.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    #[inline]
    pub fn exorcise_function(&mut self, id: &usize) -> Option<Rc<RefCell<Function>>> {
        tracing::debug!(target: "store", "exorcising function slot: {id}.");
        let result = self.function[*id].take();
        self.function_free_list.push(*id);
        result
    }

    /// Exorcise [`Function`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<usize> {
        self.function_id_by_name.get(name).map(|function| *function)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    #[inline]
    pub fn iter_function(&self) -> impl Iterator<Item = Rc<RefCell<Function>>> + '_ {
        let len = self.function.len();
        (0..len)
            .filter(|i| self.function[*i].is_some())
            .map(move |i| {
                self.function[i]
                    .as_ref()
                    .map(|function| function.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`FunctionCall`] into the store.
    ///
    #[inline]
    pub fn inter_function_call<F>(&mut self, function_call: F) -> Rc<RefCell<FunctionCall>>
    where
        F: Fn(usize) -> Rc<RefCell<FunctionCall>>,
    {
        let _index = if let Some(_index) = self.function_call_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.function_call.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.function_call.push(None);
            _index
        };

        let function_call = function_call(_index);

        if let Some(Some(function_call)) = self.function_call.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *function_call.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {function_call:?}.");
            self.function_call_free_list.push(_index);
            function_call.clone()
        } else {
            tracing::debug!(target: "store", "interring {function_call:?}.");
            self.function_call[_index] = Some(function_call.clone());
            function_call
        }
    }

    /// Exhume (get) [`FunctionCall`] from the store.
    ///
    #[inline]
    pub fn exhume_function_call(&self, id: &usize) -> Option<Rc<RefCell<FunctionCall>>> {
        match self.function_call.get(*id) {
            Some(function_call) => function_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FunctionCall`] from the store.
    ///
    #[inline]
    pub fn exorcise_function_call(&mut self, id: &usize) -> Option<Rc<RefCell<FunctionCall>>> {
        tracing::debug!(target: "store", "exorcising function_call slot: {id}.");
        let result = self.function_call[*id].take();
        self.function_call_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FunctionCall>`.
    ///
    #[inline]
    pub fn iter_function_call(&self) -> impl Iterator<Item = Rc<RefCell<FunctionCall>>> + '_ {
        let len = self.function_call.len();
        (0..len)
            .filter(|i| self.function_call[*i].is_some())
            .map(move |i| {
                self.function_call[i]
                    .as_ref()
                    .map(|function_call| function_call.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XFuture`] into the store.
    ///
    #[inline]
    pub fn inter_x_future<F>(&mut self, x_future: F) -> Rc<RefCell<XFuture>>
    where
        F: Fn(usize) -> Rc<RefCell<XFuture>>,
    {
        let _index = if let Some(_index) = self.x_future_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_future.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_future.push(None);
            _index
        };

        let x_future = x_future(_index);

        if let Some(Some(x_future)) = self.x_future.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *x_future.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {x_future:?}.");
            self.x_future_free_list.push(_index);
            x_future.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_future:?}.");
            self.x_future[_index] = Some(x_future.clone());
            x_future
        }
    }

    /// Exhume (get) [`XFuture`] from the store.
    ///
    #[inline]
    pub fn exhume_x_future(&self, id: &usize) -> Option<Rc<RefCell<XFuture>>> {
        match self.x_future.get(*id) {
            Some(x_future) => x_future.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XFuture`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_future(&mut self, id: &usize) -> Option<Rc<RefCell<XFuture>>> {
        tracing::debug!(target: "store", "exorcising x_future slot: {id}.");
        let result = self.x_future[*id].take();
        self.x_future_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XFuture>`.
    ///
    #[inline]
    pub fn iter_x_future(&self) -> impl Iterator<Item = Rc<RefCell<XFuture>>> + '_ {
        let len = self.x_future.len();
        (0..len)
            .filter(|i| self.x_future[*i].is_some())
            .map(move |i| {
                self.x_future[i]
                    .as_ref()
                    .map(|x_future| x_future.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    #[inline]
    pub fn inter_grouped<F>(&mut self, grouped: F) -> Rc<RefCell<Grouped>>
    where
        F: Fn(usize) -> Rc<RefCell<Grouped>>,
    {
        let _index = if let Some(_index) = self.grouped_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.grouped.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.grouped.push(None);
            _index
        };

        let grouped = grouped(_index);

        if let Some(Some(grouped)) = self.grouped.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *grouped.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {grouped:?}.");
            self.grouped_free_list.push(_index);
            grouped.clone()
        } else {
            tracing::debug!(target: "store", "interring {grouped:?}.");
            self.grouped[_index] = Some(grouped.clone());
            grouped
        }
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    #[inline]
    pub fn exhume_grouped(&self, id: &usize) -> Option<Rc<RefCell<Grouped>>> {
        match self.grouped.get(*id) {
            Some(grouped) => grouped.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    #[inline]
    pub fn exorcise_grouped(&mut self, id: &usize) -> Option<Rc<RefCell<Grouped>>> {
        tracing::debug!(target: "store", "exorcising grouped slot: {id}.");
        let result = self.grouped[*id].take();
        self.grouped_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    #[inline]
    pub fn iter_grouped(&self) -> impl Iterator<Item = Rc<RefCell<Grouped>>> + '_ {
        let len = self.grouped.len();
        (0..len)
            .filter(|i| self.grouped[*i].is_some())
            .map(move |i| {
                self.grouped[i]
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
    ) -> Rc<RefCell<HaltAndCatchFire>>
    where
        F: Fn(usize) -> Rc<RefCell<HaltAndCatchFire>>,
    {
        let _index = if let Some(_index) = self.halt_and_catch_fire_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.halt_and_catch_fire.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.halt_and_catch_fire.push(None);
            _index
        };

        let halt_and_catch_fire = halt_and_catch_fire(_index);

        if let Some(Some(halt_and_catch_fire)) = self.halt_and_catch_fire.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *halt_and_catch_fire.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {halt_and_catch_fire:?}.");
            self.halt_and_catch_fire_free_list.push(_index);
            halt_and_catch_fire.clone()
        } else {
            tracing::debug!(target: "store", "interring {halt_and_catch_fire:?}.");
            self.halt_and_catch_fire[_index] = Some(halt_and_catch_fire.clone());
            halt_and_catch_fire
        }
    }

    /// Exhume (get) [`HaltAndCatchFire`] from the store.
    ///
    #[inline]
    pub fn exhume_halt_and_catch_fire(&self, id: &usize) -> Option<Rc<RefCell<HaltAndCatchFire>>> {
        match self.halt_and_catch_fire.get(*id) {
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
    ) -> Option<Rc<RefCell<HaltAndCatchFire>>> {
        tracing::debug!(target: "store", "exorcising halt_and_catch_fire slot: {id}.");
        let result = self.halt_and_catch_fire[*id].take();
        self.halt_and_catch_fire_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, HaltAndCatchFire>`.
    ///
    #[inline]
    pub fn iter_halt_and_catch_fire(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<HaltAndCatchFire>>> + '_ {
        let len = self.halt_and_catch_fire.len();
        (0..len)
            .filter(|i| self.halt_and_catch_fire[*i].is_some())
            .map(move |i| {
                self.halt_and_catch_fire[i]
                    .as_ref()
                    .map(|halt_and_catch_fire| halt_and_catch_fire.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    #[inline]
    pub fn inter_x_if<F>(&mut self, x_if: F) -> Rc<RefCell<XIf>>
    where
        F: Fn(usize) -> Rc<RefCell<XIf>>,
    {
        let _index = if let Some(_index) = self.x_if_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_if.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_if.push(None);
            _index
        };

        let x_if = x_if(_index);

        if let Some(Some(x_if)) = self.x_if.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *x_if.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {x_if:?}.");
            self.x_if_free_list.push(_index);
            x_if.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_if:?}.");
            self.x_if[_index] = Some(x_if.clone());
            x_if
        }
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    #[inline]
    pub fn exhume_x_if(&self, id: &usize) -> Option<Rc<RefCell<XIf>>> {
        match self.x_if.get(*id) {
            Some(x_if) => x_if.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_if(&mut self, id: &usize) -> Option<Rc<RefCell<XIf>>> {
        tracing::debug!(target: "store", "exorcising x_if slot: {id}.");
        let result = self.x_if[*id].take();
        self.x_if_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    #[inline]
    pub fn iter_x_if(&self) -> impl Iterator<Item = Rc<RefCell<XIf>>> + '_ {
        let len = self.x_if.len();
        (0..len)
            .filter(|i| self.x_if[*i].is_some())
            .map(move |i| self.x_if[i].as_ref().map(|x_if| x_if.clone()).unwrap())
    }

    /// Inter (insert) [`ImplementationBlock`] into the store.
    ///
    #[inline]
    pub fn inter_implementation_block<F>(
        &mut self,
        implementation_block: F,
    ) -> Rc<RefCell<ImplementationBlock>>
    where
        F: Fn(usize) -> Rc<RefCell<ImplementationBlock>>,
    {
        let _index = if let Some(_index) = self.implementation_block_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.implementation_block.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.implementation_block.push(None);
            _index
        };

        let implementation_block = implementation_block(_index);

        if let Some(Some(implementation_block)) = self.implementation_block.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *implementation_block.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {implementation_block:?}.");
            self.implementation_block_free_list.push(_index);
            implementation_block.clone()
        } else {
            tracing::debug!(target: "store", "interring {implementation_block:?}.");
            self.implementation_block[_index] = Some(implementation_block.clone());
            implementation_block
        }
    }

    /// Exhume (get) [`ImplementationBlock`] from the store.
    ///
    #[inline]
    pub fn exhume_implementation_block(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<ImplementationBlock>>> {
        match self.implementation_block.get(*id) {
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
    ) -> Option<Rc<RefCell<ImplementationBlock>>> {
        tracing::debug!(target: "store", "exorcising implementation_block slot: {id}.");
        let result = self.implementation_block[*id].take();
        self.implementation_block_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ImplementationBlock>`.
    ///
    #[inline]
    pub fn iter_implementation_block(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<ImplementationBlock>>> + '_ {
        let len = self.implementation_block.len();
        (0..len)
            .filter(|i| self.implementation_block[*i].is_some())
            .map(move |i| {
                self.implementation_block[i]
                    .as_ref()
                    .map(|implementation_block| implementation_block.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    #[inline]
    pub fn inter_import<F>(&mut self, import: F) -> Rc<RefCell<Import>>
    where
        F: Fn(usize) -> Rc<RefCell<Import>>,
    {
        let _index = if let Some(_index) = self.import_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.import.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.import.push(None);
            _index
        };

        let import = import(_index);

        if let Some(Some(import)) = self.import.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *import.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {import:?}.");
            self.import_free_list.push(_index);
            import.clone()
        } else {
            tracing::debug!(target: "store", "interring {import:?}.");
            self.import[_index] = Some(import.clone());
            import
        }
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    #[inline]
    pub fn exhume_import(&self, id: &usize) -> Option<Rc<RefCell<Import>>> {
        match self.import.get(*id) {
            Some(import) => import.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    #[inline]
    pub fn exorcise_import(&mut self, id: &usize) -> Option<Rc<RefCell<Import>>> {
        tracing::debug!(target: "store", "exorcising import slot: {id}.");
        let result = self.import[*id].take();
        self.import_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    #[inline]
    pub fn iter_import(&self) -> impl Iterator<Item = Rc<RefCell<Import>>> + '_ {
        let len = self.import.len();
        (0..len)
            .filter(|i| self.import[*i].is_some())
            .map(move |i| {
                self.import[i]
                    .as_ref()
                    .map(|import| import.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    #[inline]
    pub fn inter_index<F>(&mut self, index: F) -> Rc<RefCell<Index>>
    where
        F: Fn(usize) -> Rc<RefCell<Index>>,
    {
        let _index = if let Some(_index) = self.index_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.index.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.index.push(None);
            _index
        };

        let index = index(_index);

        if let Some(Some(index)) = self.index.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *index.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {index:?}.");
            self.index_free_list.push(_index);
            index.clone()
        } else {
            tracing::debug!(target: "store", "interring {index:?}.");
            self.index[_index] = Some(index.clone());
            index
        }
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    #[inline]
    pub fn exhume_index(&self, id: &usize) -> Option<Rc<RefCell<Index>>> {
        match self.index.get(*id) {
            Some(index) => index.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    #[inline]
    pub fn exorcise_index(&mut self, id: &usize) -> Option<Rc<RefCell<Index>>> {
        tracing::debug!(target: "store", "exorcising index slot: {id}.");
        let result = self.index[*id].take();
        self.index_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    #[inline]
    pub fn iter_index(&self) -> impl Iterator<Item = Rc<RefCell<Index>>> + '_ {
        let len = self.index.len();
        (0..len)
            .filter(|i| self.index[*i].is_some())
            .map(move |i| self.index[i].as_ref().map(|index| index.clone()).unwrap())
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_integer_literal<F>(&mut self, integer_literal: F) -> Rc<RefCell<IntegerLiteral>>
    where
        F: Fn(usize) -> Rc<RefCell<IntegerLiteral>>,
    {
        let _index = if let Some(_index) = self.integer_literal_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.integer_literal.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.integer_literal.push(None);
            _index
        };

        let integer_literal = integer_literal(_index);

        if let Some(Some(integer_literal)) = self.integer_literal.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *integer_literal.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {integer_literal:?}.");
            self.integer_literal_free_list.push(_index);
            integer_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {integer_literal:?}.");
            self.integer_literal[_index] = Some(integer_literal.clone());
            integer_literal
        }
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_integer_literal(&self, id: &usize) -> Option<Rc<RefCell<IntegerLiteral>>> {
        match self.integer_literal.get(*id) {
            Some(integer_literal) => integer_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_integer_literal(&mut self, id: &usize) -> Option<Rc<RefCell<IntegerLiteral>>> {
        tracing::debug!(target: "store", "exorcising integer_literal slot: {id}.");
        let result = self.integer_literal[*id].take();
        self.integer_literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    #[inline]
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Rc<RefCell<IntegerLiteral>>> + '_ {
        let len = self.integer_literal.len();
        (0..len)
            .filter(|i| self.integer_literal[*i].is_some())
            .map(move |i| {
                self.integer_literal[i]
                    .as_ref()
                    .map(|integer_literal| integer_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    #[inline]
    pub fn inter_item<F>(&mut self, item: F) -> Rc<RefCell<Item>>
    where
        F: Fn(usize) -> Rc<RefCell<Item>>,
    {
        let _index = if let Some(_index) = self.item_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.item.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.item.push(None);
            _index
        };

        let item = item(_index);

        if let Some(Some(item)) = self.item.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *item.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {item:?}.");
            self.item_free_list.push(_index);
            item.clone()
        } else {
            tracing::debug!(target: "store", "interring {item:?}.");
            self.item[_index] = Some(item.clone());
            item
        }
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    #[inline]
    pub fn exhume_item(&self, id: &usize) -> Option<Rc<RefCell<Item>>> {
        match self.item.get(*id) {
            Some(item) => item.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    #[inline]
    pub fn exorcise_item(&mut self, id: &usize) -> Option<Rc<RefCell<Item>>> {
        tracing::debug!(target: "store", "exorcising item slot: {id}.");
        let result = self.item[*id].take();
        self.item_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    #[inline]
    pub fn iter_item(&self) -> impl Iterator<Item = Rc<RefCell<Item>>> + '_ {
        let len = self.item.len();
        (0..len)
            .filter(|i| self.item[*i].is_some())
            .map(move |i| self.item[i].as_ref().map(|item| item.clone()).unwrap())
    }

    /// Inter (insert) [`Lambda`] into the store.
    ///
    #[inline]
    pub fn inter_lambda<F>(&mut self, lambda: F) -> Rc<RefCell<Lambda>>
    where
        F: Fn(usize) -> Rc<RefCell<Lambda>>,
    {
        let _index = if let Some(_index) = self.lambda_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.lambda.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.lambda.push(None);
            _index
        };

        let lambda = lambda(_index);

        if let Some(Some(lambda)) = self.lambda.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *lambda.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {lambda:?}.");
            self.lambda_free_list.push(_index);
            lambda.clone()
        } else {
            tracing::debug!(target: "store", "interring {lambda:?}.");
            self.lambda[_index] = Some(lambda.clone());
            lambda
        }
    }

    /// Exhume (get) [`Lambda`] from the store.
    ///
    #[inline]
    pub fn exhume_lambda(&self, id: &usize) -> Option<Rc<RefCell<Lambda>>> {
        match self.lambda.get(*id) {
            Some(lambda) => lambda.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Lambda`] from the store.
    ///
    #[inline]
    pub fn exorcise_lambda(&mut self, id: &usize) -> Option<Rc<RefCell<Lambda>>> {
        tracing::debug!(target: "store", "exorcising lambda slot: {id}.");
        let result = self.lambda[*id].take();
        self.lambda_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Lambda>`.
    ///
    #[inline]
    pub fn iter_lambda(&self) -> impl Iterator<Item = Rc<RefCell<Lambda>>> + '_ {
        let len = self.lambda.len();
        (0..len)
            .filter(|i| self.lambda[*i].is_some())
            .map(move |i| {
                self.lambda[i]
                    .as_ref()
                    .map(|lambda| lambda.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`LambdaParameter`] into the store.
    ///
    #[inline]
    pub fn inter_lambda_parameter<F>(&mut self, lambda_parameter: F) -> Rc<RefCell<LambdaParameter>>
    where
        F: Fn(usize) -> Rc<RefCell<LambdaParameter>>,
    {
        let _index = if let Some(_index) = self.lambda_parameter_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.lambda_parameter.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.lambda_parameter.push(None);
            _index
        };

        let lambda_parameter = lambda_parameter(_index);

        if let Some(Some(lambda_parameter)) = self.lambda_parameter.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *lambda_parameter.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {lambda_parameter:?}.");
            self.lambda_parameter_free_list.push(_index);
            lambda_parameter.clone()
        } else {
            tracing::debug!(target: "store", "interring {lambda_parameter:?}.");
            self.lambda_parameter[_index] = Some(lambda_parameter.clone());
            lambda_parameter
        }
    }

    /// Exhume (get) [`LambdaParameter`] from the store.
    ///
    #[inline]
    pub fn exhume_lambda_parameter(&self, id: &usize) -> Option<Rc<RefCell<LambdaParameter>>> {
        match self.lambda_parameter.get(*id) {
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
    ) -> Option<Rc<RefCell<LambdaParameter>>> {
        tracing::debug!(target: "store", "exorcising lambda_parameter slot: {id}.");
        let result = self.lambda_parameter[*id].take();
        self.lambda_parameter_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LambdaParameter>`.
    ///
    #[inline]
    pub fn iter_lambda_parameter(&self) -> impl Iterator<Item = Rc<RefCell<LambdaParameter>>> + '_ {
        let len = self.lambda_parameter.len();
        (0..len)
            .filter(|i| self.lambda_parameter[*i].is_some())
            .map(move |i| {
                self.lambda_parameter[i]
                    .as_ref()
                    .map(|lambda_parameter| lambda_parameter.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    #[inline]
    pub fn inter_let_statement<F>(&mut self, let_statement: F) -> Rc<RefCell<LetStatement>>
    where
        F: Fn(usize) -> Rc<RefCell<LetStatement>>,
    {
        let _index = if let Some(_index) = self.let_statement_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.let_statement.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.let_statement.push(None);
            _index
        };

        let let_statement = let_statement(_index);

        if let Some(Some(let_statement)) = self.let_statement.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *let_statement.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {let_statement:?}.");
            self.let_statement_free_list.push(_index);
            let_statement.clone()
        } else {
            tracing::debug!(target: "store", "interring {let_statement:?}.");
            self.let_statement[_index] = Some(let_statement.clone());
            let_statement
        }
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    #[inline]
    pub fn exhume_let_statement(&self, id: &usize) -> Option<Rc<RefCell<LetStatement>>> {
        match self.let_statement.get(*id) {
            Some(let_statement) => let_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    #[inline]
    pub fn exorcise_let_statement(&mut self, id: &usize) -> Option<Rc<RefCell<LetStatement>>> {
        tracing::debug!(target: "store", "exorcising let_statement slot: {id}.");
        let result = self.let_statement[*id].take();
        self.let_statement_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    #[inline]
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Rc<RefCell<LetStatement>>> + '_ {
        let len = self.let_statement.len();
        (0..len)
            .filter(|i| self.let_statement[*i].is_some())
            .map(move |i| {
                self.let_statement[i]
                    .as_ref()
                    .map(|let_statement| let_statement.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`List`] into the store.
    ///
    #[inline]
    pub fn inter_list<F>(&mut self, list: F) -> Rc<RefCell<List>>
    where
        F: Fn(usize) -> Rc<RefCell<List>>,
    {
        let _index = if let Some(_index) = self.list_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.list.push(None);
            _index
        };

        let list = list(_index);

        if let Some(Some(list)) = self.list.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *list.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {list:?}.");
            self.list_free_list.push(_index);
            list.clone()
        } else {
            tracing::debug!(target: "store", "interring {list:?}.");
            self.list[_index] = Some(list.clone());
            list
        }
    }

    /// Exhume (get) [`List`] from the store.
    ///
    #[inline]
    pub fn exhume_list(&self, id: &usize) -> Option<Rc<RefCell<List>>> {
        match self.list.get(*id) {
            Some(list) => list.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    #[inline]
    pub fn exorcise_list(&mut self, id: &usize) -> Option<Rc<RefCell<List>>> {
        tracing::debug!(target: "store", "exorcising list slot: {id}.");
        let result = self.list[*id].take();
        self.list_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    #[inline]
    pub fn iter_list(&self) -> impl Iterator<Item = Rc<RefCell<List>>> + '_ {
        let len = self.list.len();
        (0..len)
            .filter(|i| self.list[*i].is_some())
            .map(move |i| self.list[i].as_ref().map(|list| list.clone()).unwrap())
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    #[inline]
    pub fn inter_list_element<F>(&mut self, list_element: F) -> Rc<RefCell<ListElement>>
    where
        F: Fn(usize) -> Rc<RefCell<ListElement>>,
    {
        let _index = if let Some(_index) = self.list_element_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list_element.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.list_element.push(None);
            _index
        };

        let list_element = list_element(_index);

        if let Some(Some(list_element)) = self.list_element.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *list_element.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {list_element:?}.");
            self.list_element_free_list.push(_index);
            list_element.clone()
        } else {
            tracing::debug!(target: "store", "interring {list_element:?}.");
            self.list_element[_index] = Some(list_element.clone());
            list_element
        }
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    #[inline]
    pub fn exhume_list_element(&self, id: &usize) -> Option<Rc<RefCell<ListElement>>> {
        match self.list_element.get(*id) {
            Some(list_element) => list_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    #[inline]
    pub fn exorcise_list_element(&mut self, id: &usize) -> Option<Rc<RefCell<ListElement>>> {
        tracing::debug!(target: "store", "exorcising list_element slot: {id}.");
        let result = self.list_element[*id].take();
        self.list_element_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    #[inline]
    pub fn iter_list_element(&self) -> impl Iterator<Item = Rc<RefCell<ListElement>>> + '_ {
        let len = self.list_element.len();
        (0..len)
            .filter(|i| self.list_element[*i].is_some())
            .map(move |i| {
                self.list_element[i]
                    .as_ref()
                    .map(|list_element| list_element.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    #[inline]
    pub fn inter_list_expression<F>(&mut self, list_expression: F) -> Rc<RefCell<ListExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<ListExpression>>,
    {
        let _index = if let Some(_index) = self.list_expression_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list_expression.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.list_expression.push(None);
            _index
        };

        let list_expression = list_expression(_index);

        if let Some(Some(list_expression)) = self.list_expression.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *list_expression.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {list_expression:?}.");
            self.list_expression_free_list.push(_index);
            list_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {list_expression:?}.");
            self.list_expression[_index] = Some(list_expression.clone());
            list_expression
        }
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_list_expression(&self, id: &usize) -> Option<Rc<RefCell<ListExpression>>> {
        match self.list_expression.get(*id) {
            Some(list_expression) => list_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_list_expression(&mut self, id: &usize) -> Option<Rc<RefCell<ListExpression>>> {
        tracing::debug!(target: "store", "exorcising list_expression slot: {id}.");
        let result = self.list_expression[*id].take();
        self.list_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    #[inline]
    pub fn iter_list_expression(&self) -> impl Iterator<Item = Rc<RefCell<ListExpression>>> + '_ {
        let len = self.list_expression.len();
        (0..len)
            .filter(|i| self.list_expression[*i].is_some())
            .map(move |i| {
                self.list_expression[i]
                    .as_ref()
                    .map(|list_expression| list_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    #[inline]
    pub fn inter_literal<F>(&mut self, literal: F) -> Rc<RefCell<Literal>>
    where
        F: Fn(usize) -> Rc<RefCell<Literal>>,
    {
        let _index = if let Some(_index) = self.literal_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.literal.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.literal.push(None);
            _index
        };

        let literal = literal(_index);

        if let Some(Some(literal)) = self.literal.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *literal.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {literal:?}.");
            self.literal_free_list.push(_index);
            literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {literal:?}.");
            self.literal[_index] = Some(literal.clone());
            literal
        }
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    #[inline]
    pub fn exhume_literal(&self, id: &usize) -> Option<Rc<RefCell<Literal>>> {
        match self.literal.get(*id) {
            Some(literal) => literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    #[inline]
    pub fn exorcise_literal(&mut self, id: &usize) -> Option<Rc<RefCell<Literal>>> {
        tracing::debug!(target: "store", "exorcising literal slot: {id}.");
        let result = self.literal[*id].take();
        self.literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    #[inline]
    pub fn iter_literal(&self) -> impl Iterator<Item = Rc<RefCell<Literal>>> + '_ {
        let len = self.literal.len();
        (0..len)
            .filter(|i| self.literal[*i].is_some())
            .map(move |i| {
                self.literal[i]
                    .as_ref()
                    .map(|literal| literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    #[inline]
    pub fn inter_local_variable<F>(&mut self, local_variable: F) -> Rc<RefCell<LocalVariable>>
    where
        F: Fn(usize) -> Rc<RefCell<LocalVariable>>,
    {
        let _index = if let Some(_index) = self.local_variable_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.local_variable.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.local_variable.push(None);
            _index
        };

        let local_variable = local_variable(_index);

        if let Some(Some(local_variable)) = self.local_variable.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *local_variable.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {local_variable:?}.");
            self.local_variable_free_list.push(_index);
            local_variable.clone()
        } else {
            tracing::debug!(target: "store", "interring {local_variable:?}.");
            self.local_variable[_index] = Some(local_variable.clone());
            local_variable
        }
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    #[inline]
    pub fn exhume_local_variable(&self, id: &usize) -> Option<Rc<RefCell<LocalVariable>>> {
        match self.local_variable.get(*id) {
            Some(local_variable) => local_variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    #[inline]
    pub fn exorcise_local_variable(&mut self, id: &usize) -> Option<Rc<RefCell<LocalVariable>>> {
        tracing::debug!(target: "store", "exorcising local_variable slot: {id}.");
        let result = self.local_variable[*id].take();
        self.local_variable_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    #[inline]
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Rc<RefCell<LocalVariable>>> + '_ {
        let len = self.local_variable.len();
        (0..len)
            .filter(|i| self.local_variable[*i].is_some())
            .map(move |i| {
                self.local_variable[i]
                    .as_ref()
                    .map(|local_variable| local_variable.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    #[inline]
    pub fn inter_x_macro<F>(&mut self, x_macro: F) -> Rc<RefCell<XMacro>>
    where
        F: Fn(usize) -> Rc<RefCell<XMacro>>,
    {
        let _index = if let Some(_index) = self.x_macro_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_macro.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_macro.push(None);
            _index
        };

        let x_macro = x_macro(_index);

        if let Some(Some(x_macro)) = self.x_macro.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *x_macro.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {x_macro:?}.");
            self.x_macro_free_list.push(_index);
            x_macro.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_macro:?}.");
            self.x_macro[_index] = Some(x_macro.clone());
            x_macro
        }
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    #[inline]
    pub fn exhume_x_macro(&self, id: &usize) -> Option<Rc<RefCell<XMacro>>> {
        match self.x_macro.get(*id) {
            Some(x_macro) => x_macro.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_macro(&mut self, id: &usize) -> Option<Rc<RefCell<XMacro>>> {
        tracing::debug!(target: "store", "exorcising x_macro slot: {id}.");
        let result = self.x_macro[*id].take();
        self.x_macro_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    #[inline]
    pub fn iter_x_macro(&self) -> impl Iterator<Item = Rc<RefCell<XMacro>>> + '_ {
        let len = self.x_macro.len();
        (0..len)
            .filter(|i| self.x_macro[*i].is_some())
            .map(move |i| {
                self.x_macro[i]
                    .as_ref()
                    .map(|x_macro| x_macro.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Map`] into the store.
    ///
    #[inline]
    pub fn inter_map<F>(&mut self, map: F) -> Rc<RefCell<Map>>
    where
        F: Fn(usize) -> Rc<RefCell<Map>>,
    {
        let _index = if let Some(_index) = self.map_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.map.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.map.push(None);
            _index
        };

        let map = map(_index);

        if let Some(Some(map)) = self.map.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *map.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {map:?}.");
            self.map_free_list.push(_index);
            map.clone()
        } else {
            tracing::debug!(target: "store", "interring {map:?}.");
            self.map[_index] = Some(map.clone());
            map
        }
    }

    /// Exhume (get) [`Map`] from the store.
    ///
    #[inline]
    pub fn exhume_map(&self, id: &usize) -> Option<Rc<RefCell<Map>>> {
        match self.map.get(*id) {
            Some(map) => map.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Map`] from the store.
    ///
    #[inline]
    pub fn exorcise_map(&mut self, id: &usize) -> Option<Rc<RefCell<Map>>> {
        tracing::debug!(target: "store", "exorcising map slot: {id}.");
        let result = self.map[*id].take();
        self.map_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Map>`.
    ///
    #[inline]
    pub fn iter_map(&self) -> impl Iterator<Item = Rc<RefCell<Map>>> + '_ {
        let len = self.map.len();
        (0..len)
            .filter(|i| self.map[*i].is_some())
            .map(move |i| self.map[i].as_ref().map(|map| map.clone()).unwrap())
    }

    /// Inter (insert) [`MapElement`] into the store.
    ///
    #[inline]
    pub fn inter_map_element<F>(&mut self, map_element: F) -> Rc<RefCell<MapElement>>
    where
        F: Fn(usize) -> Rc<RefCell<MapElement>>,
    {
        let _index = if let Some(_index) = self.map_element_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.map_element.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.map_element.push(None);
            _index
        };

        let map_element = map_element(_index);

        if let Some(Some(map_element)) = self.map_element.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *map_element.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {map_element:?}.");
            self.map_element_free_list.push(_index);
            map_element.clone()
        } else {
            tracing::debug!(target: "store", "interring {map_element:?}.");
            self.map_element[_index] = Some(map_element.clone());
            map_element
        }
    }

    /// Exhume (get) [`MapElement`] from the store.
    ///
    #[inline]
    pub fn exhume_map_element(&self, id: &usize) -> Option<Rc<RefCell<MapElement>>> {
        match self.map_element.get(*id) {
            Some(map_element) => map_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MapElement`] from the store.
    ///
    #[inline]
    pub fn exorcise_map_element(&mut self, id: &usize) -> Option<Rc<RefCell<MapElement>>> {
        tracing::debug!(target: "store", "exorcising map_element slot: {id}.");
        let result = self.map_element[*id].take();
        self.map_element_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MapElement>`.
    ///
    #[inline]
    pub fn iter_map_element(&self) -> impl Iterator<Item = Rc<RefCell<MapElement>>> + '_ {
        let len = self.map_element.len();
        (0..len)
            .filter(|i| self.map_element[*i].is_some())
            .map(move |i| {
                self.map_element[i]
                    .as_ref()
                    .map(|map_element| map_element.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`MapExpression`] into the store.
    ///
    #[inline]
    pub fn inter_map_expression<F>(&mut self, map_expression: F) -> Rc<RefCell<MapExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<MapExpression>>,
    {
        let _index = if let Some(_index) = self.map_expression_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.map_expression.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.map_expression.push(None);
            _index
        };

        let map_expression = map_expression(_index);

        if let Some(Some(map_expression)) = self.map_expression.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *map_expression.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {map_expression:?}.");
            self.map_expression_free_list.push(_index);
            map_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {map_expression:?}.");
            self.map_expression[_index] = Some(map_expression.clone());
            map_expression
        }
    }

    /// Exhume (get) [`MapExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_map_expression(&self, id: &usize) -> Option<Rc<RefCell<MapExpression>>> {
        match self.map_expression.get(*id) {
            Some(map_expression) => map_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MapExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_map_expression(&mut self, id: &usize) -> Option<Rc<RefCell<MapExpression>>> {
        tracing::debug!(target: "store", "exorcising map_expression slot: {id}.");
        let result = self.map_expression[*id].take();
        self.map_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MapExpression>`.
    ///
    #[inline]
    pub fn iter_map_expression(&self) -> impl Iterator<Item = Rc<RefCell<MapExpression>>> + '_ {
        let len = self.map_expression.len();
        (0..len)
            .filter(|i| self.map_expression[*i].is_some())
            .map(move |i| {
                self.map_expression[i]
                    .as_ref()
                    .map(|map_expression| map_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XMatch`] into the store.
    ///
    #[inline]
    pub fn inter_x_match<F>(&mut self, x_match: F) -> Rc<RefCell<XMatch>>
    where
        F: Fn(usize) -> Rc<RefCell<XMatch>>,
    {
        let _index = if let Some(_index) = self.x_match_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_match.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_match.push(None);
            _index
        };

        let x_match = x_match(_index);

        if let Some(Some(x_match)) = self.x_match.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *x_match.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {x_match:?}.");
            self.x_match_free_list.push(_index);
            x_match.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_match:?}.");
            self.x_match[_index] = Some(x_match.clone());
            x_match
        }
    }

    /// Exhume (get) [`XMatch`] from the store.
    ///
    #[inline]
    pub fn exhume_x_match(&self, id: &usize) -> Option<Rc<RefCell<XMatch>>> {
        match self.x_match.get(*id) {
            Some(x_match) => x_match.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMatch`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_match(&mut self, id: &usize) -> Option<Rc<RefCell<XMatch>>> {
        tracing::debug!(target: "store", "exorcising x_match slot: {id}.");
        let result = self.x_match[*id].take();
        self.x_match_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMatch>`.
    ///
    #[inline]
    pub fn iter_x_match(&self) -> impl Iterator<Item = Rc<RefCell<XMatch>>> + '_ {
        let len = self.x_match.len();
        (0..len)
            .filter(|i| self.x_match[*i].is_some())
            .map(move |i| {
                self.x_match[i]
                    .as_ref()
                    .map(|x_match| x_match.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    #[inline]
    pub fn inter_method_call<F>(&mut self, method_call: F) -> Rc<RefCell<MethodCall>>
    where
        F: Fn(usize) -> Rc<RefCell<MethodCall>>,
    {
        let _index = if let Some(_index) = self.method_call_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.method_call.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.method_call.push(None);
            _index
        };

        let method_call = method_call(_index);

        if let Some(Some(method_call)) = self.method_call.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *method_call.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {method_call:?}.");
            self.method_call_free_list.push(_index);
            method_call.clone()
        } else {
            tracing::debug!(target: "store", "interring {method_call:?}.");
            self.method_call[_index] = Some(method_call.clone());
            method_call
        }
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    #[inline]
    pub fn exhume_method_call(&self, id: &usize) -> Option<Rc<RefCell<MethodCall>>> {
        match self.method_call.get(*id) {
            Some(method_call) => method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    #[inline]
    pub fn exorcise_method_call(&mut self, id: &usize) -> Option<Rc<RefCell<MethodCall>>> {
        tracing::debug!(target: "store", "exorcising method_call slot: {id}.");
        let result = self.method_call[*id].take();
        self.method_call_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    #[inline]
    pub fn iter_method_call(&self) -> impl Iterator<Item = Rc<RefCell<MethodCall>>> + '_ {
        let len = self.method_call.len();
        (0..len)
            .filter(|i| self.method_call[*i].is_some())
            .map(move |i| {
                self.method_call[i]
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
    ) -> Rc<RefCell<NamedFieldExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<NamedFieldExpression>>,
    {
        let _index = if let Some(_index) = self.named_field_expression_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.named_field_expression.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.named_field_expression.push(None);
            _index
        };

        let named_field_expression = named_field_expression(_index);

        if let Some(Some(named_field_expression)) =
            self.named_field_expression.iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.borrow() == *named_field_expression.borrow()
                } else {
                    false
                }
            })
        {
            tracing::debug!(target: "store", "found duplicate {named_field_expression:?}.");
            self.named_field_expression_free_list.push(_index);
            named_field_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {named_field_expression:?}.");
            self.named_field_expression[_index] = Some(named_field_expression.clone());
            named_field_expression
        }
    }

    /// Exhume (get) [`NamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_named_field_expression(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<NamedFieldExpression>>> {
        match self.named_field_expression.get(*id) {
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
    ) -> Option<Rc<RefCell<NamedFieldExpression>>> {
        tracing::debug!(target: "store", "exorcising named_field_expression slot: {id}.");
        let result = self.named_field_expression[*id].take();
        self.named_field_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NamedFieldExpression>`.
    ///
    #[inline]
    pub fn iter_named_field_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<NamedFieldExpression>>> + '_ {
        let len = self.named_field_expression.len();
        (0..len)
            .filter(|i| self.named_field_expression[*i].is_some())
            .map(move |i| {
                self.named_field_expression[i]
                    .as_ref()
                    .map(|named_field_expression| named_field_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    #[inline]
    pub fn inter_z_object_store<F>(&mut self, z_object_store: F) -> Rc<RefCell<ZObjectStore>>
    where
        F: Fn(usize) -> Rc<RefCell<ZObjectStore>>,
    {
        let _index = if let Some(_index) = self.z_object_store_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.z_object_store.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.z_object_store.push(None);
            _index
        };

        let z_object_store = z_object_store(_index);

        let z_object_store = if let Some(Some(z_object_store)) =
            self.z_object_store.iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.borrow() == *z_object_store.borrow()
                } else {
                    false
                }
            }) {
            tracing::debug!(target: "store", "found duplicate {z_object_store:?}.");
            self.z_object_store_free_list.push(_index);
            z_object_store.clone()
        } else {
            tracing::debug!(target: "store", "interring {z_object_store:?}.");
            self.z_object_store[_index] = Some(z_object_store.clone());
            z_object_store
        };
        self.z_object_store_id_by_name.insert(
            z_object_store.borrow().name.to_owned(),
            z_object_store.borrow().id,
        );
        z_object_store
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    #[inline]
    pub fn exhume_z_object_store(&self, id: &usize) -> Option<Rc<RefCell<ZObjectStore>>> {
        match self.z_object_store.get(*id) {
            Some(z_object_store) => z_object_store.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    #[inline]
    pub fn exorcise_z_object_store(&mut self, id: &usize) -> Option<Rc<RefCell<ZObjectStore>>> {
        tracing::debug!(target: "store", "exorcising z_object_store slot: {id}.");
        let result = self.z_object_store[*id].take();
        self.z_object_store_free_list.push(*id);
        result
    }

    /// Exorcise [`ZObjectStore`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_z_object_store_id_by_name(&self, name: &str) -> Option<usize> {
        self.z_object_store_id_by_name
            .get(name)
            .map(|z_object_store| *z_object_store)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    #[inline]
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Rc<RefCell<ZObjectStore>>> + '_ {
        let len = self.z_object_store.len();
        (0..len)
            .filter(|i| self.z_object_store[*i].is_some())
            .map(move |i| {
                self.z_object_store[i]
                    .as_ref()
                    .map(|z_object_store| z_object_store.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ObjectWrapper`] into the store.
    ///
    #[inline]
    pub fn inter_object_wrapper<F>(&mut self, object_wrapper: F) -> Rc<RefCell<ObjectWrapper>>
    where
        F: Fn(usize) -> Rc<RefCell<ObjectWrapper>>,
    {
        let _index = if let Some(_index) = self.object_wrapper_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.object_wrapper.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.object_wrapper.push(None);
            _index
        };

        let object_wrapper = object_wrapper(_index);

        if let Some(Some(object_wrapper)) = self.object_wrapper.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *object_wrapper.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {object_wrapper:?}.");
            self.object_wrapper_free_list.push(_index);
            object_wrapper.clone()
        } else {
            tracing::debug!(target: "store", "interring {object_wrapper:?}.");
            self.object_wrapper[_index] = Some(object_wrapper.clone());
            object_wrapper
        }
    }

    /// Exhume (get) [`ObjectWrapper`] from the store.
    ///
    #[inline]
    pub fn exhume_object_wrapper(&self, id: &usize) -> Option<Rc<RefCell<ObjectWrapper>>> {
        match self.object_wrapper.get(*id) {
            Some(object_wrapper) => object_wrapper.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ObjectWrapper`] from the store.
    ///
    #[inline]
    pub fn exorcise_object_wrapper(&mut self, id: &usize) -> Option<Rc<RefCell<ObjectWrapper>>> {
        tracing::debug!(target: "store", "exorcising object_wrapper slot: {id}.");
        let result = self.object_wrapper[*id].take();
        self.object_wrapper_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectWrapper>`.
    ///
    #[inline]
    pub fn iter_object_wrapper(&self) -> impl Iterator<Item = Rc<RefCell<ObjectWrapper>>> + '_ {
        let len = self.object_wrapper.len();
        (0..len)
            .filter(|i| self.object_wrapper[*i].is_some())
            .map(move |i| {
                self.object_wrapper[i]
                    .as_ref()
                    .map(|object_wrapper| object_wrapper.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    #[inline]
    pub fn inter_operator<F>(&mut self, operator: F) -> Rc<RefCell<Operator>>
    where
        F: Fn(usize) -> Rc<RefCell<Operator>>,
    {
        let _index = if let Some(_index) = self.operator_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.operator.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.operator.push(None);
            _index
        };

        let operator = operator(_index);

        if let Some(Some(operator)) = self.operator.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *operator.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {operator:?}.");
            self.operator_free_list.push(_index);
            operator.clone()
        } else {
            tracing::debug!(target: "store", "interring {operator:?}.");
            self.operator[_index] = Some(operator.clone());
            operator
        }
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    #[inline]
    pub fn exhume_operator(&self, id: &usize) -> Option<Rc<RefCell<Operator>>> {
        match self.operator.get(*id) {
            Some(operator) => operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    #[inline]
    pub fn exorcise_operator(&mut self, id: &usize) -> Option<Rc<RefCell<Operator>>> {
        tracing::debug!(target: "store", "exorcising operator slot: {id}.");
        let result = self.operator[*id].take();
        self.operator_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    #[inline]
    pub fn iter_operator(&self) -> impl Iterator<Item = Rc<RefCell<Operator>>> + '_ {
        let len = self.operator.len();
        (0..len)
            .filter(|i| self.operator[*i].is_some())
            .map(move |i| {
                self.operator[i]
                    .as_ref()
                    .map(|operator| operator.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    #[inline]
    pub fn inter_parameter<F>(&mut self, parameter: F) -> Rc<RefCell<Parameter>>
    where
        F: Fn(usize) -> Rc<RefCell<Parameter>>,
    {
        let _index = if let Some(_index) = self.parameter_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.parameter.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.parameter.push(None);
            _index
        };

        let parameter = parameter(_index);

        if let Some(Some(parameter)) = self.parameter.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *parameter.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {parameter:?}.");
            self.parameter_free_list.push(_index);
            parameter.clone()
        } else {
            tracing::debug!(target: "store", "interring {parameter:?}.");
            self.parameter[_index] = Some(parameter.clone());
            parameter
        }
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    #[inline]
    pub fn exhume_parameter(&self, id: &usize) -> Option<Rc<RefCell<Parameter>>> {
        match self.parameter.get(*id) {
            Some(parameter) => parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    #[inline]
    pub fn exorcise_parameter(&mut self, id: &usize) -> Option<Rc<RefCell<Parameter>>> {
        tracing::debug!(target: "store", "exorcising parameter slot: {id}.");
        let result = self.parameter[*id].take();
        self.parameter_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    #[inline]
    pub fn iter_parameter(&self) -> impl Iterator<Item = Rc<RefCell<Parameter>>> + '_ {
        let len = self.parameter.len();
        (0..len)
            .filter(|i| self.parameter[*i].is_some())
            .map(move |i| {
                self.parameter[i]
                    .as_ref()
                    .map(|parameter| parameter.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XPath`] into the store.
    ///
    #[inline]
    pub fn inter_x_path<F>(&mut self, x_path: F) -> Rc<RefCell<XPath>>
    where
        F: Fn(usize) -> Rc<RefCell<XPath>>,
    {
        let _index = if let Some(_index) = self.x_path_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_path.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_path.push(None);
            _index
        };

        let x_path = x_path(_index);

        if let Some(Some(x_path)) = self.x_path.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *x_path.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {x_path:?}.");
            self.x_path_free_list.push(_index);
            x_path.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_path:?}.");
            self.x_path[_index] = Some(x_path.clone());
            x_path
        }
    }

    /// Exhume (get) [`XPath`] from the store.
    ///
    #[inline]
    pub fn exhume_x_path(&self, id: &usize) -> Option<Rc<RefCell<XPath>>> {
        match self.x_path.get(*id) {
            Some(x_path) => x_path.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPath`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_path(&mut self, id: &usize) -> Option<Rc<RefCell<XPath>>> {
        tracing::debug!(target: "store", "exorcising x_path slot: {id}.");
        let result = self.x_path[*id].take();
        self.x_path_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPath>`.
    ///
    #[inline]
    pub fn iter_x_path(&self) -> impl Iterator<Item = Rc<RefCell<XPath>>> + '_ {
        let len = self.x_path.len();
        (0..len)
            .filter(|i| self.x_path[*i].is_some())
            .map(move |i| {
                self.x_path[i]
                    .as_ref()
                    .map(|x_path| x_path.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`PathElement`] into the store.
    ///
    #[inline]
    pub fn inter_path_element<F>(&mut self, path_element: F) -> Rc<RefCell<PathElement>>
    where
        F: Fn(usize) -> Rc<RefCell<PathElement>>,
    {
        let _index = if let Some(_index) = self.path_element_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.path_element.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.path_element.push(None);
            _index
        };

        let path_element = path_element(_index);

        if let Some(Some(path_element)) = self.path_element.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *path_element.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {path_element:?}.");
            self.path_element_free_list.push(_index);
            path_element.clone()
        } else {
            tracing::debug!(target: "store", "interring {path_element:?}.");
            self.path_element[_index] = Some(path_element.clone());
            path_element
        }
    }

    /// Exhume (get) [`PathElement`] from the store.
    ///
    #[inline]
    pub fn exhume_path_element(&self, id: &usize) -> Option<Rc<RefCell<PathElement>>> {
        match self.path_element.get(*id) {
            Some(path_element) => path_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`PathElement`] from the store.
    ///
    #[inline]
    pub fn exorcise_path_element(&mut self, id: &usize) -> Option<Rc<RefCell<PathElement>>> {
        tracing::debug!(target: "store", "exorcising path_element slot: {id}.");
        let result = self.path_element[*id].take();
        self.path_element_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, PathElement>`.
    ///
    #[inline]
    pub fn iter_path_element(&self) -> impl Iterator<Item = Rc<RefCell<PathElement>>> + '_ {
        let len = self.path_element.len();
        (0..len)
            .filter(|i| self.path_element[*i].is_some())
            .map(move |i| {
                self.path_element[i]
                    .as_ref()
                    .map(|path_element| path_element.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Pattern`] into the store.
    ///
    #[inline]
    pub fn inter_pattern<F>(&mut self, pattern: F) -> Rc<RefCell<Pattern>>
    where
        F: Fn(usize) -> Rc<RefCell<Pattern>>,
    {
        let _index = if let Some(_index) = self.pattern_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.pattern.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.pattern.push(None);
            _index
        };

        let pattern = pattern(_index);

        if let Some(Some(pattern)) = self.pattern.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *pattern.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {pattern:?}.");
            self.pattern_free_list.push(_index);
            pattern.clone()
        } else {
            tracing::debug!(target: "store", "interring {pattern:?}.");
            self.pattern[_index] = Some(pattern.clone());
            pattern
        }
    }

    /// Exhume (get) [`Pattern`] from the store.
    ///
    #[inline]
    pub fn exhume_pattern(&self, id: &usize) -> Option<Rc<RefCell<Pattern>>> {
        match self.pattern.get(*id) {
            Some(pattern) => pattern.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Pattern`] from the store.
    ///
    #[inline]
    pub fn exorcise_pattern(&mut self, id: &usize) -> Option<Rc<RefCell<Pattern>>> {
        tracing::debug!(target: "store", "exorcising pattern slot: {id}.");
        let result = self.pattern[*id].take();
        self.pattern_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Pattern>`.
    ///
    #[inline]
    pub fn iter_pattern(&self) -> impl Iterator<Item = Rc<RefCell<Pattern>>> + '_ {
        let len = self.pattern.len();
        (0..len)
            .filter(|i| self.pattern[*i].is_some())
            .map(move |i| {
                self.pattern[i]
                    .as_ref()
                    .map(|pattern| pattern.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XPlugin`] into the store.
    ///
    #[inline]
    pub fn inter_x_plugin<F>(&mut self, x_plugin: F) -> Rc<RefCell<XPlugin>>
    where
        F: Fn(usize) -> Rc<RefCell<XPlugin>>,
    {
        let _index = if let Some(_index) = self.x_plugin_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_plugin.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_plugin.push(None);
            _index
        };

        let x_plugin = x_plugin(_index);

        let x_plugin = if let Some(Some(x_plugin)) = self.x_plugin.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *x_plugin.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {x_plugin:?}.");
            self.x_plugin_free_list.push(_index);
            x_plugin.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_plugin:?}.");
            self.x_plugin[_index] = Some(x_plugin.clone());
            x_plugin
        };
        self.x_plugin_id_by_name
            .insert(x_plugin.borrow().name.to_owned(), x_plugin.borrow().id);
        x_plugin
    }

    /// Exhume (get) [`XPlugin`] from the store.
    ///
    #[inline]
    pub fn exhume_x_plugin(&self, id: &usize) -> Option<Rc<RefCell<XPlugin>>> {
        match self.x_plugin.get(*id) {
            Some(x_plugin) => x_plugin.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPlugin`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_plugin(&mut self, id: &usize) -> Option<Rc<RefCell<XPlugin>>> {
        tracing::debug!(target: "store", "exorcising x_plugin slot: {id}.");
        let result = self.x_plugin[*id].take();
        self.x_plugin_free_list.push(*id);
        result
    }

    /// Exorcise [`XPlugin`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_x_plugin_id_by_name(&self, name: &str) -> Option<usize> {
        self.x_plugin_id_by_name.get(name).map(|x_plugin| *x_plugin)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPlugin>`.
    ///
    #[inline]
    pub fn iter_x_plugin(&self) -> impl Iterator<Item = Rc<RefCell<XPlugin>>> + '_ {
        let len = self.x_plugin.len();
        (0..len)
            .filter(|i| self.x_plugin[*i].is_some())
            .map(move |i| {
                self.x_plugin[i]
                    .as_ref()
                    .map(|x_plugin| x_plugin.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XPrint`] into the store.
    ///
    #[inline]
    pub fn inter_x_print<F>(&mut self, x_print: F) -> Rc<RefCell<XPrint>>
    where
        F: Fn(usize) -> Rc<RefCell<XPrint>>,
    {
        let _index = if let Some(_index) = self.x_print_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_print.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_print.push(None);
            _index
        };

        let x_print = x_print(_index);

        if let Some(Some(x_print)) = self.x_print.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *x_print.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {x_print:?}.");
            self.x_print_free_list.push(_index);
            x_print.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_print:?}.");
            self.x_print[_index] = Some(x_print.clone());
            x_print
        }
    }

    /// Exhume (get) [`XPrint`] from the store.
    ///
    #[inline]
    pub fn exhume_x_print(&self, id: &usize) -> Option<Rc<RefCell<XPrint>>> {
        match self.x_print.get(*id) {
            Some(x_print) => x_print.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPrint`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_print(&mut self, id: &usize) -> Option<Rc<RefCell<XPrint>>> {
        tracing::debug!(target: "store", "exorcising x_print slot: {id}.");
        let result = self.x_print[*id].take();
        self.x_print_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPrint>`.
    ///
    #[inline]
    pub fn iter_x_print(&self) -> impl Iterator<Item = Rc<RefCell<XPrint>>> + '_ {
        let len = self.x_print.len();
        (0..len)
            .filter(|i| self.x_print[*i].is_some())
            .map(move |i| {
                self.x_print[i]
                    .as_ref()
                    .map(|x_print| x_print.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    #[inline]
    pub fn inter_range_expression<F>(&mut self, range_expression: F) -> Rc<RefCell<RangeExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<RangeExpression>>,
    {
        let _index = if let Some(_index) = self.range_expression_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.range_expression.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.range_expression.push(None);
            _index
        };

        let range_expression = range_expression(_index);

        if let Some(Some(range_expression)) = self.range_expression.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *range_expression.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {range_expression:?}.");
            self.range_expression_free_list.push(_index);
            range_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {range_expression:?}.");
            self.range_expression[_index] = Some(range_expression.clone());
            range_expression
        }
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_range_expression(&self, id: &usize) -> Option<Rc<RefCell<RangeExpression>>> {
        match self.range_expression.get(*id) {
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
    ) -> Option<Rc<RefCell<RangeExpression>>> {
        tracing::debug!(target: "store", "exorcising range_expression slot: {id}.");
        let result = self.range_expression[*id].take();
        self.range_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    #[inline]
    pub fn iter_range_expression(&self) -> impl Iterator<Item = Rc<RefCell<RangeExpression>>> + '_ {
        let len = self.range_expression.len();
        (0..len)
            .filter(|i| self.range_expression[*i].is_some())
            .map(move |i| {
                self.range_expression[i]
                    .as_ref()
                    .map(|range_expression| range_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    #[inline]
    pub fn inter_result_statement<F>(&mut self, result_statement: F) -> Rc<RefCell<ResultStatement>>
    where
        F: Fn(usize) -> Rc<RefCell<ResultStatement>>,
    {
        let _index = if let Some(_index) = self.result_statement_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.result_statement.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.result_statement.push(None);
            _index
        };

        let result_statement = result_statement(_index);

        if let Some(Some(result_statement)) = self.result_statement.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *result_statement.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {result_statement:?}.");
            self.result_statement_free_list.push(_index);
            result_statement.clone()
        } else {
            tracing::debug!(target: "store", "interring {result_statement:?}.");
            self.result_statement[_index] = Some(result_statement.clone());
            result_statement
        }
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    #[inline]
    pub fn exhume_result_statement(&self, id: &usize) -> Option<Rc<RefCell<ResultStatement>>> {
        match self.result_statement.get(*id) {
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
    ) -> Option<Rc<RefCell<ResultStatement>>> {
        tracing::debug!(target: "store", "exorcising result_statement slot: {id}.");
        let result = self.result_statement[*id].take();
        self.result_statement_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    #[inline]
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Rc<RefCell<ResultStatement>>> + '_ {
        let len = self.result_statement.len();
        (0..len)
            .filter(|i| self.result_statement[*i].is_some())
            .map(move |i| {
                self.result_statement[i]
                    .as_ref()
                    .map(|result_statement| result_statement.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    #[inline]
    pub fn inter_x_return<F>(&mut self, x_return: F) -> Rc<RefCell<XReturn>>
    where
        F: Fn(usize) -> Rc<RefCell<XReturn>>,
    {
        let _index = if let Some(_index) = self.x_return_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_return.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_return.push(None);
            _index
        };

        let x_return = x_return(_index);

        if let Some(Some(x_return)) = self.x_return.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *x_return.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {x_return:?}.");
            self.x_return_free_list.push(_index);
            x_return.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_return:?}.");
            self.x_return[_index] = Some(x_return.clone());
            x_return
        }
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    #[inline]
    pub fn exhume_x_return(&self, id: &usize) -> Option<Rc<RefCell<XReturn>>> {
        match self.x_return.get(*id) {
            Some(x_return) => x_return.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_return(&mut self, id: &usize) -> Option<Rc<RefCell<XReturn>>> {
        tracing::debug!(target: "store", "exorcising x_return slot: {id}.");
        let result = self.x_return[*id].take();
        self.x_return_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    #[inline]
    pub fn iter_x_return(&self) -> impl Iterator<Item = Rc<RefCell<XReturn>>> + '_ {
        let len = self.x_return.len();
        (0..len)
            .filter(|i| self.x_return[*i].is_some())
            .map(move |i| {
                self.x_return[i]
                    .as_ref()
                    .map(|x_return| x_return.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    #[inline]
    pub fn inter_span<F>(&mut self, span: F) -> Rc<RefCell<Span>>
    where
        F: Fn(usize) -> Rc<RefCell<Span>>,
    {
        let _index = if let Some(_index) = self.span_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.span.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.span.push(None);
            _index
        };

        let span = span(_index);

        if let Some(Some(span)) = self.span.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *span.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {span:?}.");
            self.span_free_list.push(_index);
            span.clone()
        } else {
            tracing::debug!(target: "store", "interring {span:?}.");
            self.span[_index] = Some(span.clone());
            span
        }
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    #[inline]
    pub fn exhume_span(&self, id: &usize) -> Option<Rc<RefCell<Span>>> {
        match self.span.get(*id) {
            Some(span) => span.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    #[inline]
    pub fn exorcise_span(&mut self, id: &usize) -> Option<Rc<RefCell<Span>>> {
        tracing::debug!(target: "store", "exorcising span slot: {id}.");
        let result = self.span[*id].take();
        self.span_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    #[inline]
    pub fn iter_span(&self) -> impl Iterator<Item = Rc<RefCell<Span>>> + '_ {
        let len = self.span.len();
        (0..len)
            .filter(|i| self.span[*i].is_some())
            .map(move |i| self.span[i].as_ref().map(|span| span.clone()).unwrap())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    #[inline]
    pub fn inter_statement<F>(&mut self, statement: F) -> Rc<RefCell<Statement>>
    where
        F: Fn(usize) -> Rc<RefCell<Statement>>,
    {
        let _index = if let Some(_index) = self.statement_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.statement.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.statement.push(None);
            _index
        };

        let statement = statement(_index);

        if let Some(Some(statement)) = self.statement.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *statement.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {statement:?}.");
            self.statement_free_list.push(_index);
            statement.clone()
        } else {
            tracing::debug!(target: "store", "interring {statement:?}.");
            self.statement[_index] = Some(statement.clone());
            statement
        }
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    #[inline]
    pub fn exhume_statement(&self, id: &usize) -> Option<Rc<RefCell<Statement>>> {
        match self.statement.get(*id) {
            Some(statement) => statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    #[inline]
    pub fn exorcise_statement(&mut self, id: &usize) -> Option<Rc<RefCell<Statement>>> {
        tracing::debug!(target: "store", "exorcising statement slot: {id}.");
        let result = self.statement[*id].take();
        self.statement_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    #[inline]
    pub fn iter_statement(&self) -> impl Iterator<Item = Rc<RefCell<Statement>>> + '_ {
        let len = self.statement.len();
        (0..len)
            .filter(|i| self.statement[*i].is_some())
            .map(move |i| {
                self.statement[i]
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
    ) -> Rc<RefCell<StaticMethodCall>>
    where
        F: Fn(usize) -> Rc<RefCell<StaticMethodCall>>,
    {
        let _index = if let Some(_index) = self.static_method_call_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.static_method_call.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.static_method_call.push(None);
            _index
        };

        let static_method_call = static_method_call(_index);

        if let Some(Some(static_method_call)) = self.static_method_call.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *static_method_call.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {static_method_call:?}.");
            self.static_method_call_free_list.push(_index);
            static_method_call.clone()
        } else {
            tracing::debug!(target: "store", "interring {static_method_call:?}.");
            self.static_method_call[_index] = Some(static_method_call.clone());
            static_method_call
        }
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    #[inline]
    pub fn exhume_static_method_call(&self, id: &usize) -> Option<Rc<RefCell<StaticMethodCall>>> {
        match self.static_method_call.get(*id) {
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
    ) -> Option<Rc<RefCell<StaticMethodCall>>> {
        tracing::debug!(target: "store", "exorcising static_method_call slot: {id}.");
        let result = self.static_method_call[*id].take();
        self.static_method_call_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    #[inline]
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<StaticMethodCall>>> + '_ {
        let len = self.static_method_call.len();
        (0..len)
            .filter(|i| self.static_method_call[*i].is_some())
            .map(move |i| {
                self.static_method_call[i]
                    .as_ref()
                    .map(|static_method_call| static_method_call.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StringBit`] into the store.
    ///
    #[inline]
    pub fn inter_string_bit<F>(&mut self, string_bit: F) -> Rc<RefCell<StringBit>>
    where
        F: Fn(usize) -> Rc<RefCell<StringBit>>,
    {
        let _index = if let Some(_index) = self.string_bit_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.string_bit.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.string_bit.push(None);
            _index
        };

        let string_bit = string_bit(_index);

        if let Some(Some(string_bit)) = self.string_bit.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *string_bit.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {string_bit:?}.");
            self.string_bit_free_list.push(_index);
            string_bit.clone()
        } else {
            tracing::debug!(target: "store", "interring {string_bit:?}.");
            self.string_bit[_index] = Some(string_bit.clone());
            string_bit
        }
    }

    /// Exhume (get) [`StringBit`] from the store.
    ///
    #[inline]
    pub fn exhume_string_bit(&self, id: &usize) -> Option<Rc<RefCell<StringBit>>> {
        match self.string_bit.get(*id) {
            Some(string_bit) => string_bit.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StringBit`] from the store.
    ///
    #[inline]
    pub fn exorcise_string_bit(&mut self, id: &usize) -> Option<Rc<RefCell<StringBit>>> {
        tracing::debug!(target: "store", "exorcising string_bit slot: {id}.");
        let result = self.string_bit[*id].take();
        self.string_bit_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringBit>`.
    ///
    #[inline]
    pub fn iter_string_bit(&self) -> impl Iterator<Item = Rc<RefCell<StringBit>>> + '_ {
        let len = self.string_bit.len();
        (0..len)
            .filter(|i| self.string_bit[*i].is_some())
            .map(move |i| {
                self.string_bit[i]
                    .as_ref()
                    .map(|string_bit| string_bit.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    #[inline]
    pub fn inter_string_literal<F>(&mut self, string_literal: F) -> Rc<RefCell<StringLiteral>>
    where
        F: Fn(usize) -> Rc<RefCell<StringLiteral>>,
    {
        let _index = if let Some(_index) = self.string_literal_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.string_literal.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.string_literal.push(None);
            _index
        };

        let string_literal = string_literal(_index);

        if let Some(Some(string_literal)) = self.string_literal.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *string_literal.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {string_literal:?}.");
            self.string_literal_free_list.push(_index);
            string_literal.clone()
        } else {
            tracing::debug!(target: "store", "interring {string_literal:?}.");
            self.string_literal[_index] = Some(string_literal.clone());
            string_literal
        }
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    #[inline]
    pub fn exhume_string_literal(&self, id: &usize) -> Option<Rc<RefCell<StringLiteral>>> {
        match self.string_literal.get(*id) {
            Some(string_literal) => string_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_string_literal(&mut self, id: &usize) -> Option<Rc<RefCell<StringLiteral>>> {
        tracing::debug!(target: "store", "exorcising string_literal slot: {id}.");
        let result = self.string_literal[*id].take();
        self.string_literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    #[inline]
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Rc<RefCell<StringLiteral>>> + '_ {
        let len = self.string_literal.len();
        (0..len)
            .filter(|i| self.string_literal[*i].is_some())
            .map(move |i| {
                self.string_literal[i]
                    .as_ref()
                    .map(|string_literal| string_literal.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    #[inline]
    pub fn inter_woog_struct<F>(&mut self, woog_struct: F) -> Rc<RefCell<WoogStruct>>
    where
        F: Fn(usize) -> Rc<RefCell<WoogStruct>>,
    {
        let _index = if let Some(_index) = self.woog_struct_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.woog_struct.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.woog_struct.push(None);
            _index
        };

        let woog_struct = woog_struct(_index);

        let woog_struct = if let Some(Some(woog_struct)) = self.woog_struct.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *woog_struct.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {woog_struct:?}.");
            self.woog_struct_free_list.push(_index);
            woog_struct.clone()
        } else {
            tracing::debug!(target: "store", "interring {woog_struct:?}.");
            self.woog_struct[_index] = Some(woog_struct.clone());
            woog_struct
        };
        self.woog_struct_id_by_name.insert(
            woog_struct.borrow().name.to_owned(),
            woog_struct.borrow().id,
        );
        woog_struct
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    #[inline]
    pub fn exhume_woog_struct(&self, id: &usize) -> Option<Rc<RefCell<WoogStruct>>> {
        match self.woog_struct.get(*id) {
            Some(woog_struct) => woog_struct.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    #[inline]
    pub fn exorcise_woog_struct(&mut self, id: &usize) -> Option<Rc<RefCell<WoogStruct>>> {
        tracing::debug!(target: "store", "exorcising woog_struct slot: {id}.");
        let result = self.woog_struct[*id].take();
        self.woog_struct_free_list.push(*id);
        result
    }

    /// Exorcise [`WoogStruct`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<usize> {
        self.woog_struct_id_by_name
            .get(name)
            .map(|woog_struct| *woog_struct)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    #[inline]
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Rc<RefCell<WoogStruct>>> + '_ {
        let len = self.woog_struct.len();
        (0..len)
            .filter(|i| self.woog_struct[*i].is_some())
            .map(move |i| {
                self.woog_struct[i]
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
    ) -> Rc<RefCell<StructExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<StructExpression>>,
    {
        let _index = if let Some(_index) = self.struct_expression_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_expression.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.struct_expression.push(None);
            _index
        };

        let struct_expression = struct_expression(_index);

        if let Some(Some(struct_expression)) = self.struct_expression.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *struct_expression.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {struct_expression:?}.");
            self.struct_expression_free_list.push(_index);
            struct_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {struct_expression:?}.");
            self.struct_expression[_index] = Some(struct_expression.clone());
            struct_expression
        }
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_struct_expression(&self, id: &usize) -> Option<Rc<RefCell<StructExpression>>> {
        match self.struct_expression.get(*id) {
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
    ) -> Option<Rc<RefCell<StructExpression>>> {
        tracing::debug!(target: "store", "exorcising struct_expression slot: {id}.");
        let result = self.struct_expression[*id].take();
        self.struct_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    #[inline]
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<StructExpression>>> + '_ {
        let len = self.struct_expression.len();
        (0..len)
            .filter(|i| self.struct_expression[*i].is_some())
            .map(move |i| {
                self.struct_expression[i]
                    .as_ref()
                    .map(|struct_expression| struct_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StructField`] into the store.
    ///
    #[inline]
    pub fn inter_struct_field<F>(&mut self, struct_field: F) -> Rc<RefCell<StructField>>
    where
        F: Fn(usize) -> Rc<RefCell<StructField>>,
    {
        let _index = if let Some(_index) = self.struct_field_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_field.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.struct_field.push(None);
            _index
        };

        let struct_field = struct_field(_index);

        if let Some(Some(struct_field)) = self.struct_field.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *struct_field.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {struct_field:?}.");
            self.struct_field_free_list.push(_index);
            struct_field.clone()
        } else {
            tracing::debug!(target: "store", "interring {struct_field:?}.");
            self.struct_field[_index] = Some(struct_field.clone());
            struct_field
        }
    }

    /// Exhume (get) [`StructField`] from the store.
    ///
    #[inline]
    pub fn exhume_struct_field(&self, id: &usize) -> Option<Rc<RefCell<StructField>>> {
        match self.struct_field.get(*id) {
            Some(struct_field) => struct_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructField`] from the store.
    ///
    #[inline]
    pub fn exorcise_struct_field(&mut self, id: &usize) -> Option<Rc<RefCell<StructField>>> {
        tracing::debug!(target: "store", "exorcising struct_field slot: {id}.");
        let result = self.struct_field[*id].take();
        self.struct_field_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructField>`.
    ///
    #[inline]
    pub fn iter_struct_field(&self) -> impl Iterator<Item = Rc<RefCell<StructField>>> + '_ {
        let len = self.struct_field.len();
        (0..len)
            .filter(|i| self.struct_field[*i].is_some())
            .map(move |i| {
                self.struct_field[i]
                    .as_ref()
                    .map(|struct_field| struct_field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StructGeneric`] into the store.
    ///
    #[inline]
    pub fn inter_struct_generic<F>(&mut self, struct_generic: F) -> Rc<RefCell<StructGeneric>>
    where
        F: Fn(usize) -> Rc<RefCell<StructGeneric>>,
    {
        let _index = if let Some(_index) = self.struct_generic_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_generic.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.struct_generic.push(None);
            _index
        };

        let struct_generic = struct_generic(_index);

        if let Some(Some(struct_generic)) = self.struct_generic.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *struct_generic.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {struct_generic:?}.");
            self.struct_generic_free_list.push(_index);
            struct_generic.clone()
        } else {
            tracing::debug!(target: "store", "interring {struct_generic:?}.");
            self.struct_generic[_index] = Some(struct_generic.clone());
            struct_generic
        }
    }

    /// Exhume (get) [`StructGeneric`] from the store.
    ///
    #[inline]
    pub fn exhume_struct_generic(&self, id: &usize) -> Option<Rc<RefCell<StructGeneric>>> {
        match self.struct_generic.get(*id) {
            Some(struct_generic) => struct_generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructGeneric`] from the store.
    ///
    #[inline]
    pub fn exorcise_struct_generic(&mut self, id: &usize) -> Option<Rc<RefCell<StructGeneric>>> {
        tracing::debug!(target: "store", "exorcising struct_generic slot: {id}.");
        let result = self.struct_generic[*id].take();
        self.struct_generic_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructGeneric>`.
    ///
    #[inline]
    pub fn iter_struct_generic(&self) -> impl Iterator<Item = Rc<RefCell<StructGeneric>>> + '_ {
        let len = self.struct_generic.len();
        (0..len)
            .filter(|i| self.struct_generic[*i].is_some())
            .map(move |i| {
                self.struct_generic[i]
                    .as_ref()
                    .map(|struct_generic| struct_generic.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`TupleField`] into the store.
    ///
    #[inline]
    pub fn inter_tuple_field<F>(&mut self, tuple_field: F) -> Rc<RefCell<TupleField>>
    where
        F: Fn(usize) -> Rc<RefCell<TupleField>>,
    {
        let _index = if let Some(_index) = self.tuple_field_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.tuple_field.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.tuple_field.push(None);
            _index
        };

        let tuple_field = tuple_field(_index);

        if let Some(Some(tuple_field)) = self.tuple_field.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *tuple_field.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {tuple_field:?}.");
            self.tuple_field_free_list.push(_index);
            tuple_field.clone()
        } else {
            tracing::debug!(target: "store", "interring {tuple_field:?}.");
            self.tuple_field[_index] = Some(tuple_field.clone());
            tuple_field
        }
    }

    /// Exhume (get) [`TupleField`] from the store.
    ///
    #[inline]
    pub fn exhume_tuple_field(&self, id: &usize) -> Option<Rc<RefCell<TupleField>>> {
        match self.tuple_field.get(*id) {
            Some(tuple_field) => tuple_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TupleField`] from the store.
    ///
    #[inline]
    pub fn exorcise_tuple_field(&mut self, id: &usize) -> Option<Rc<RefCell<TupleField>>> {
        tracing::debug!(target: "store", "exorcising tuple_field slot: {id}.");
        let result = self.tuple_field[*id].take();
        self.tuple_field_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TupleField>`.
    ///
    #[inline]
    pub fn iter_tuple_field(&self) -> impl Iterator<Item = Rc<RefCell<TupleField>>> + '_ {
        let len = self.tuple_field.len();
        (0..len)
            .filter(|i| self.tuple_field[*i].is_some())
            .map(move |i| {
                self.tuple_field[i]
                    .as_ref()
                    .map(|tuple_field| tuple_field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    #[inline]
    pub fn inter_type_cast<F>(&mut self, type_cast: F) -> Rc<RefCell<TypeCast>>
    where
        F: Fn(usize) -> Rc<RefCell<TypeCast>>,
    {
        let _index = if let Some(_index) = self.type_cast_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.type_cast.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.type_cast.push(None);
            _index
        };

        let type_cast = type_cast(_index);

        if let Some(Some(type_cast)) = self.type_cast.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *type_cast.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {type_cast:?}.");
            self.type_cast_free_list.push(_index);
            type_cast.clone()
        } else {
            tracing::debug!(target: "store", "interring {type_cast:?}.");
            self.type_cast[_index] = Some(type_cast.clone());
            type_cast
        }
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    #[inline]
    pub fn exhume_type_cast(&self, id: &usize) -> Option<Rc<RefCell<TypeCast>>> {
        match self.type_cast.get(*id) {
            Some(type_cast) => type_cast.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    #[inline]
    pub fn exorcise_type_cast(&mut self, id: &usize) -> Option<Rc<RefCell<TypeCast>>> {
        tracing::debug!(target: "store", "exorcising type_cast slot: {id}.");
        let result = self.type_cast[*id].take();
        self.type_cast_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    #[inline]
    pub fn iter_type_cast(&self) -> impl Iterator<Item = Rc<RefCell<TypeCast>>> + '_ {
        let len = self.type_cast.len();
        (0..len)
            .filter(|i| self.type_cast[*i].is_some())
            .map(move |i| {
                self.type_cast[i]
                    .as_ref()
                    .map(|type_cast| type_cast.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    #[inline]
    pub fn inter_unary<F>(&mut self, unary: F) -> Rc<RefCell<Unary>>
    where
        F: Fn(usize) -> Rc<RefCell<Unary>>,
    {
        let _index = if let Some(_index) = self.unary_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unary.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.unary.push(None);
            _index
        };

        let unary = unary(_index);

        if let Some(Some(unary)) = self.unary.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *unary.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {unary:?}.");
            self.unary_free_list.push(_index);
            unary.clone()
        } else {
            tracing::debug!(target: "store", "interring {unary:?}.");
            self.unary[_index] = Some(unary.clone());
            unary
        }
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    #[inline]
    pub fn exhume_unary(&self, id: &usize) -> Option<Rc<RefCell<Unary>>> {
        match self.unary.get(*id) {
            Some(unary) => unary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    #[inline]
    pub fn exorcise_unary(&mut self, id: &usize) -> Option<Rc<RefCell<Unary>>> {
        tracing::debug!(target: "store", "exorcising unary slot: {id}.");
        let result = self.unary[*id].take();
        self.unary_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    #[inline]
    pub fn iter_unary(&self) -> impl Iterator<Item = Rc<RefCell<Unary>>> + '_ {
        let len = self.unary.len();
        (0..len)
            .filter(|i| self.unary[*i].is_some())
            .map(move |i| self.unary[i].as_ref().map(|unary| unary.clone()).unwrap())
    }

    /// Inter (insert) [`Unit`] into the store.
    ///
    #[inline]
    pub fn inter_unit<F>(&mut self, unit: F) -> Rc<RefCell<Unit>>
    where
        F: Fn(usize) -> Rc<RefCell<Unit>>,
    {
        let _index = if let Some(_index) = self.unit_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unit.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.unit.push(None);
            _index
        };

        let unit = unit(_index);

        if let Some(Some(unit)) = self.unit.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *unit.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {unit:?}.");
            self.unit_free_list.push(_index);
            unit.clone()
        } else {
            tracing::debug!(target: "store", "interring {unit:?}.");
            self.unit[_index] = Some(unit.clone());
            unit
        }
    }

    /// Exhume (get) [`Unit`] from the store.
    ///
    #[inline]
    pub fn exhume_unit(&self, id: &usize) -> Option<Rc<RefCell<Unit>>> {
        match self.unit.get(*id) {
            Some(unit) => unit.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Unit`] from the store.
    ///
    #[inline]
    pub fn exorcise_unit(&mut self, id: &usize) -> Option<Rc<RefCell<Unit>>> {
        tracing::debug!(target: "store", "exorcising unit slot: {id}.");
        let result = self.unit[*id].take();
        self.unit_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unit>`.
    ///
    #[inline]
    pub fn iter_unit(&self) -> impl Iterator<Item = Rc<RefCell<Unit>>> + '_ {
        let len = self.unit.len();
        (0..len)
            .filter(|i| self.unit[*i].is_some())
            .map(move |i| self.unit[i].as_ref().map(|unit| unit.clone()).unwrap())
    }

    /// Inter (insert) [`UnnamedFieldExpression`] into the store.
    ///
    #[inline]
    pub fn inter_unnamed_field_expression<F>(
        &mut self,
        unnamed_field_expression: F,
    ) -> Rc<RefCell<UnnamedFieldExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<UnnamedFieldExpression>>,
    {
        let _index = if let Some(_index) = self.unnamed_field_expression_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unnamed_field_expression.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.unnamed_field_expression.push(None);
            _index
        };

        let unnamed_field_expression = unnamed_field_expression(_index);

        if let Some(Some(unnamed_field_expression)) =
            self.unnamed_field_expression.iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.borrow() == *unnamed_field_expression.borrow()
                } else {
                    false
                }
            })
        {
            tracing::debug!(target: "store", "found duplicate {unnamed_field_expression:?}.");
            self.unnamed_field_expression_free_list.push(_index);
            unnamed_field_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {unnamed_field_expression:?}.");
            self.unnamed_field_expression[_index] = Some(unnamed_field_expression.clone());
            unnamed_field_expression
        }
    }

    /// Exhume (get) [`UnnamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_unnamed_field_expression(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<UnnamedFieldExpression>>> {
        match self.unnamed_field_expression.get(*id) {
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
    ) -> Option<Rc<RefCell<UnnamedFieldExpression>>> {
        tracing::debug!(target: "store", "exorcising unnamed_field_expression slot: {id}.");
        let result = self.unnamed_field_expression[*id].take();
        self.unnamed_field_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, UnnamedFieldExpression>`.
    ///
    #[inline]
    pub fn iter_unnamed_field_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<UnnamedFieldExpression>>> + '_ {
        let len = self.unnamed_field_expression.len();
        (0..len)
            .filter(|i| self.unnamed_field_expression[*i].is_some())
            .map(move |i| {
                self.unnamed_field_expression[i]
                    .as_ref()
                    .map(|unnamed_field_expression| unnamed_field_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    #[inline]
    pub fn inter_x_value<F>(&mut self, x_value: F) -> Rc<RefCell<XValue>>
    where
        F: Fn(usize) -> Rc<RefCell<XValue>>,
    {
        let _index = if let Some(_index) = self.x_value_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_value.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.x_value.push(None);
            _index
        };

        let x_value = x_value(_index);

        if let Some(Some(x_value)) = self.x_value.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *x_value.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {x_value:?}.");
            self.x_value_free_list.push(_index);
            x_value.clone()
        } else {
            tracing::debug!(target: "store", "interring {x_value:?}.");
            self.x_value[_index] = Some(x_value.clone());
            x_value
        }
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    #[inline]
    pub fn exhume_x_value(&self, id: &usize) -> Option<Rc<RefCell<XValue>>> {
        match self.x_value.get(*id) {
            Some(x_value) => x_value.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_value(&mut self, id: &usize) -> Option<Rc<RefCell<XValue>>> {
        tracing::debug!(target: "store", "exorcising x_value slot: {id}.");
        let result = self.x_value[*id].take();
        self.x_value_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    #[inline]
    pub fn iter_x_value(&self) -> impl Iterator<Item = Rc<RefCell<XValue>>> + '_ {
        let len = self.x_value.len();
        (0..len)
            .filter(|i| self.x_value[*i].is_some())
            .map(move |i| {
                self.x_value[i]
                    .as_ref()
                    .map(|x_value| x_value.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    #[inline]
    pub fn inter_value_type<F>(&mut self, value_type: F) -> Rc<RefCell<ValueType>>
    where
        F: Fn(usize) -> Rc<RefCell<ValueType>>,
    {
        let _index = if let Some(_index) = self.value_type_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.value_type.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.value_type.push(None);
            _index
        };

        let value_type = value_type(_index);

        if let Some(Some(value_type)) = self.value_type.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *value_type.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {value_type:?}.");
            self.value_type_free_list.push(_index);
            value_type.clone()
        } else {
            tracing::debug!(target: "store", "interring {value_type:?}.");
            self.value_type[_index] = Some(value_type.clone());
            value_type
        }
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    #[inline]
    pub fn exhume_value_type(&self, id: &usize) -> Option<Rc<RefCell<ValueType>>> {
        match self.value_type.get(*id) {
            Some(value_type) => value_type.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    #[inline]
    pub fn exorcise_value_type(&mut self, id: &usize) -> Option<Rc<RefCell<ValueType>>> {
        tracing::debug!(target: "store", "exorcising value_type slot: {id}.");
        let result = self.value_type[*id].take();
        self.value_type_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    #[inline]
    pub fn iter_value_type(&self) -> impl Iterator<Item = Rc<RefCell<ValueType>>> + '_ {
        let len = self.value_type.len();
        (0..len)
            .filter(|i| self.value_type[*i].is_some())
            .map(move |i| {
                self.value_type[i]
                    .as_ref()
                    .map(|value_type| value_type.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    #[inline]
    pub fn inter_variable<F>(&mut self, variable: F) -> Rc<RefCell<Variable>>
    where
        F: Fn(usize) -> Rc<RefCell<Variable>>,
    {
        let _index = if let Some(_index) = self.variable_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.variable.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.variable.push(None);
            _index
        };

        let variable = variable(_index);

        if let Some(Some(variable)) = self.variable.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *variable.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {variable:?}.");
            self.variable_free_list.push(_index);
            variable.clone()
        } else {
            tracing::debug!(target: "store", "interring {variable:?}.");
            self.variable[_index] = Some(variable.clone());
            variable
        }
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    #[inline]
    pub fn exhume_variable(&self, id: &usize) -> Option<Rc<RefCell<Variable>>> {
        match self.variable.get(*id) {
            Some(variable) => variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    #[inline]
    pub fn exorcise_variable(&mut self, id: &usize) -> Option<Rc<RefCell<Variable>>> {
        tracing::debug!(target: "store", "exorcising variable slot: {id}.");
        let result = self.variable[*id].take();
        self.variable_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    #[inline]
    pub fn iter_variable(&self) -> impl Iterator<Item = Rc<RefCell<Variable>>> + '_ {
        let len = self.variable.len();
        (0..len)
            .filter(|i| self.variable[*i].is_some())
            .map(move |i| {
                self.variable[i]
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
    ) -> Rc<RefCell<VariableExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<VariableExpression>>,
    {
        let _index = if let Some(_index) = self.variable_expression_free_list.pop() {
            tracing::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.variable_expression.len();
            tracing::trace!(target: "store", "allocating block {_index}.");
            self.variable_expression.push(None);
            _index
        };

        let variable_expression = variable_expression(_index);

        if let Some(Some(variable_expression)) = self.variable_expression.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *variable_expression.borrow()
            } else {
                false
            }
        }) {
            tracing::debug!(target: "store", "found duplicate {variable_expression:?}.");
            self.variable_expression_free_list.push(_index);
            variable_expression.clone()
        } else {
            tracing::debug!(target: "store", "interring {variable_expression:?}.");
            self.variable_expression[_index] = Some(variable_expression.clone());
            variable_expression
        }
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_variable_expression(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<VariableExpression>>> {
        match self.variable_expression.get(*id) {
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
    ) -> Option<Rc<RefCell<VariableExpression>>> {
        tracing::debug!(target: "store", "exorcising variable_expression slot: {id}.");
        let result = self.variable_expression[*id].take();
        self.variable_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    #[inline]
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<VariableExpression>>> + '_ {
        let len = self.variable_expression.len();
        (0..len)
            .filter(|i| self.variable_expression[*i].is_some())
            .map(move |i| {
                self.variable_expression[i]
                    .as_ref()
                    .map(|variable_expression| variable_expression.clone())
                    .unwrap()
            })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vec-object-store-persistence"}}}
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
            for argument in &self.argument {
                if let Some(argument) = argument {
                    let path = path.join(format!("{}.json", argument.borrow().id));
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
            for a_wait in &self.a_wait {
                if let Some(a_wait) = a_wait {
                    let path = path.join(format!("{}.json", a_wait.borrow().id));
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
            for binary in &self.binary {
                if let Some(binary) = binary {
                    let path = path.join(format!("{}.json", binary.borrow().id));
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
            for block in &self.block {
                if let Some(block) = block {
                    let path = path.join(format!("{}.json", block.borrow().id));
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
            for body in &self.body {
                if let Some(body) = body {
                    let path = path.join(format!("{}.json", body.borrow().id));
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
            for boolean_literal in &self.boolean_literal {
                if let Some(boolean_literal) = boolean_literal {
                    let path = path.join(format!("{}.json", boolean_literal.borrow().id));
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
            for boolean_operator in &self.boolean_operator {
                if let Some(boolean_operator) = boolean_operator {
                    let path = path.join(format!("{}.json", boolean_operator.borrow().id));
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
            for call in &self.call {
                if let Some(call) = call {
                    let path = path.join(format!("{}.json", call.borrow().id));
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
            for char_literal in &self.char_literal {
                if let Some(char_literal) = char_literal {
                    let path = path.join(format!("{}.json", char_literal.borrow().id));
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
            for comparison in &self.comparison {
                if let Some(comparison) = comparison {
                    let path = path.join(format!("{}.json", comparison.borrow().id));
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
            for data_structure in &self.data_structure {
                if let Some(data_structure) = data_structure {
                    let path = path.join(format!("{}.json", data_structure.borrow().id));
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
            for dwarf_source_file in &self.dwarf_source_file {
                if let Some(dwarf_source_file) = dwarf_source_file {
                    let path = path.join(format!("{}.json", dwarf_source_file.borrow().id));
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
            for enum_field in &self.enum_field {
                if let Some(enum_field) = enum_field {
                    let path = path.join(format!("{}.json", enum_field.borrow().id));
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
            for enum_generic in &self.enum_generic {
                if let Some(enum_generic) = enum_generic {
                    let path = path.join(format!("{}.json", enum_generic.borrow().id));
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
            for enum_generic_type in &self.enum_generic_type {
                if let Some(enum_generic_type) = enum_generic_type {
                    let path = path.join(format!("{}.json", enum_generic_type.borrow().id));
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
            for enumeration in &self.enumeration {
                if let Some(enumeration) = enumeration {
                    let path = path.join(format!("{}.json", enumeration.borrow().id));
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
            for expression in &self.expression {
                if let Some(expression) = expression {
                    let path = path.join(format!("{}.json", expression.borrow().id));
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
            for expression_bit in &self.expression_bit {
                if let Some(expression_bit) = expression_bit {
                    let path = path.join(format!("{}.json", expression_bit.borrow().id));
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
            for expression_statement in &self.expression_statement {
                if let Some(expression_statement) = expression_statement {
                    let path = path.join(format!("{}.json", expression_statement.borrow().id));
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
            for external_implementation in &self.external_implementation {
                if let Some(external_implementation) = external_implementation {
                    let path = path.join(format!("{}.json", external_implementation.borrow().id));
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
            for field in &self.field {
                if let Some(field) = field {
                    let path = path.join(format!("{}.json", field.borrow().id));
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
            for field_access in &self.field_access {
                if let Some(field_access) = field_access {
                    let path = path.join(format!("{}.json", field_access.borrow().id));
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
            for field_access_target in &self.field_access_target {
                if let Some(field_access_target) = field_access_target {
                    let path = path.join(format!("{}.json", field_access_target.borrow().id));
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
            for field_expression in &self.field_expression {
                if let Some(field_expression) = field_expression {
                    let path = path.join(format!("{}.json", field_expression.borrow().id));
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
            for float_literal in &self.float_literal {
                if let Some(float_literal) = float_literal {
                    let path = path.join(format!("{}.json", float_literal.borrow().id));
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
            for for_loop in &self.for_loop {
                if let Some(for_loop) = for_loop {
                    let path = path.join(format!("{}.json", for_loop.borrow().id));
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
            for format_bit in &self.format_bit {
                if let Some(format_bit) = format_bit {
                    let path = path.join(format!("{}.json", format_bit.borrow().id));
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
            for format_string in &self.format_string {
                if let Some(format_string) = format_string {
                    let path = path.join(format!("{}.json", format_string.borrow().id));
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
            for func_generic in &self.func_generic {
                if let Some(func_generic) = func_generic {
                    let path = path.join(format!("{}.json", func_generic.borrow().id));
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
            for function in &self.function {
                if let Some(function) = function {
                    let path = path.join(format!("{}.json", function.borrow().id));
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
            for function_call in &self.function_call {
                if let Some(function_call) = function_call {
                    let path = path.join(format!("{}.json", function_call.borrow().id));
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
            for x_future in &self.x_future {
                if let Some(x_future) = x_future {
                    let path = path.join(format!("{}.json", x_future.borrow().id));
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
            for grouped in &self.grouped {
                if let Some(grouped) = grouped {
                    let path = path.join(format!("{}.json", grouped.borrow().id));
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
            for halt_and_catch_fire in &self.halt_and_catch_fire {
                if let Some(halt_and_catch_fire) = halt_and_catch_fire {
                    let path = path.join(format!("{}.json", halt_and_catch_fire.borrow().id));
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
            for x_if in &self.x_if {
                if let Some(x_if) = x_if {
                    let path = path.join(format!("{}.json", x_if.borrow().id));
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
            for implementation_block in &self.implementation_block {
                if let Some(implementation_block) = implementation_block {
                    let path = path.join(format!("{}.json", implementation_block.borrow().id));
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
            for import in &self.import {
                if let Some(import) = import {
                    let path = path.join(format!("{}.json", import.borrow().id));
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
            for index in &self.index {
                if let Some(index) = index {
                    let path = path.join(format!("{}.json", index.borrow().id));
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
            for integer_literal in &self.integer_literal {
                if let Some(integer_literal) = integer_literal {
                    let path = path.join(format!("{}.json", integer_literal.borrow().id));
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
            for item in &self.item {
                if let Some(item) = item {
                    let path = path.join(format!("{}.json", item.borrow().id));
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
            for lambda in &self.lambda {
                if let Some(lambda) = lambda {
                    let path = path.join(format!("{}.json", lambda.borrow().id));
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
            for lambda_parameter in &self.lambda_parameter {
                if let Some(lambda_parameter) = lambda_parameter {
                    let path = path.join(format!("{}.json", lambda_parameter.borrow().id));
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
            for let_statement in &self.let_statement {
                if let Some(let_statement) = let_statement {
                    let path = path.join(format!("{}.json", let_statement.borrow().id));
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
            for list in &self.list {
                if let Some(list) = list {
                    let path = path.join(format!("{}.json", list.borrow().id));
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
            for list_element in &self.list_element {
                if let Some(list_element) = list_element {
                    let path = path.join(format!("{}.json", list_element.borrow().id));
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
            for list_expression in &self.list_expression {
                if let Some(list_expression) = list_expression {
                    let path = path.join(format!("{}.json", list_expression.borrow().id));
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
            for literal in &self.literal {
                if let Some(literal) = literal {
                    let path = path.join(format!("{}.json", literal.borrow().id));
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
            for local_variable in &self.local_variable {
                if let Some(local_variable) = local_variable {
                    let path = path.join(format!("{}.json", local_variable.borrow().id));
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
            for x_macro in &self.x_macro {
                if let Some(x_macro) = x_macro {
                    let path = path.join(format!("{}.json", x_macro.borrow().id));
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
            for map in &self.map {
                if let Some(map) = map {
                    let path = path.join(format!("{}.json", map.borrow().id));
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
            for map_element in &self.map_element {
                if let Some(map_element) = map_element {
                    let path = path.join(format!("{}.json", map_element.borrow().id));
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
            for map_expression in &self.map_expression {
                if let Some(map_expression) = map_expression {
                    let path = path.join(format!("{}.json", map_expression.borrow().id));
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
            for x_match in &self.x_match {
                if let Some(x_match) = x_match {
                    let path = path.join(format!("{}.json", x_match.borrow().id));
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
            for method_call in &self.method_call {
                if let Some(method_call) = method_call {
                    let path = path.join(format!("{}.json", method_call.borrow().id));
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
            for named_field_expression in &self.named_field_expression {
                if let Some(named_field_expression) = named_field_expression {
                    let path = path.join(format!("{}.json", named_field_expression.borrow().id));
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
            for z_object_store in &self.z_object_store {
                if let Some(z_object_store) = z_object_store {
                    let path = path.join(format!("{}.json", z_object_store.borrow().id));
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
            for object_wrapper in &self.object_wrapper {
                if let Some(object_wrapper) = object_wrapper {
                    let path = path.join(format!("{}.json", object_wrapper.borrow().id));
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
            for operator in &self.operator {
                if let Some(operator) = operator {
                    let path = path.join(format!("{}.json", operator.borrow().id));
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
            for parameter in &self.parameter {
                if let Some(parameter) = parameter {
                    let path = path.join(format!("{}.json", parameter.borrow().id));
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
            for x_path in &self.x_path {
                if let Some(x_path) = x_path {
                    let path = path.join(format!("{}.json", x_path.borrow().id));
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
            for path_element in &self.path_element {
                if let Some(path_element) = path_element {
                    let path = path.join(format!("{}.json", path_element.borrow().id));
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
            for pattern in &self.pattern {
                if let Some(pattern) = pattern {
                    let path = path.join(format!("{}.json", pattern.borrow().id));
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
            for x_plugin in &self.x_plugin {
                if let Some(x_plugin) = x_plugin {
                    let path = path.join(format!("{}.json", x_plugin.borrow().id));
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
            for x_print in &self.x_print {
                if let Some(x_print) = x_print {
                    let path = path.join(format!("{}.json", x_print.borrow().id));
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
            for range_expression in &self.range_expression {
                if let Some(range_expression) = range_expression {
                    let path = path.join(format!("{}.json", range_expression.borrow().id));
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
            for result_statement in &self.result_statement {
                if let Some(result_statement) = result_statement {
                    let path = path.join(format!("{}.json", result_statement.borrow().id));
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
            for x_return in &self.x_return {
                if let Some(x_return) = x_return {
                    let path = path.join(format!("{}.json", x_return.borrow().id));
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
            for span in &self.span {
                if let Some(span) = span {
                    let path = path.join(format!("{}.json", span.borrow().id));
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
            for statement in &self.statement {
                if let Some(statement) = statement {
                    let path = path.join(format!("{}.json", statement.borrow().id));
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
            for static_method_call in &self.static_method_call {
                if let Some(static_method_call) = static_method_call {
                    let path = path.join(format!("{}.json", static_method_call.borrow().id));
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
            for string_bit in &self.string_bit {
                if let Some(string_bit) = string_bit {
                    let path = path.join(format!("{}.json", string_bit.borrow().id));
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
            for string_literal in &self.string_literal {
                if let Some(string_literal) = string_literal {
                    let path = path.join(format!("{}.json", string_literal.borrow().id));
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
            for woog_struct in &self.woog_struct {
                if let Some(woog_struct) = woog_struct {
                    let path = path.join(format!("{}.json", woog_struct.borrow().id));
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
            for struct_expression in &self.struct_expression {
                if let Some(struct_expression) = struct_expression {
                    let path = path.join(format!("{}.json", struct_expression.borrow().id));
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
            for struct_field in &self.struct_field {
                if let Some(struct_field) = struct_field {
                    let path = path.join(format!("{}.json", struct_field.borrow().id));
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
            for struct_generic in &self.struct_generic {
                if let Some(struct_generic) = struct_generic {
                    let path = path.join(format!("{}.json", struct_generic.borrow().id));
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
            for tuple_field in &self.tuple_field {
                if let Some(tuple_field) = tuple_field {
                    let path = path.join(format!("{}.json", tuple_field.borrow().id));
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
            for type_cast in &self.type_cast {
                if let Some(type_cast) = type_cast {
                    let path = path.join(format!("{}.json", type_cast.borrow().id));
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
            for unary in &self.unary {
                if let Some(unary) = unary {
                    let path = path.join(format!("{}.json", unary.borrow().id));
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
            for unit in &self.unit {
                if let Some(unit) = unit {
                    let path = path.join(format!("{}.json", unit.borrow().id));
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
            for unnamed_field_expression in &self.unnamed_field_expression {
                if let Some(unnamed_field_expression) = unnamed_field_expression {
                    let path = path.join(format!("{}.json", unnamed_field_expression.borrow().id));
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
            for x_value in &self.x_value {
                if let Some(x_value) = x_value {
                    let path = path.join(format!("{}.json", x_value.borrow().id));
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
            for value_type in &self.value_type {
                if let Some(value_type) = value_type {
                    let path = path.join(format!("{}.json", value_type.borrow().id));
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
            for variable in &self.variable {
                if let Some(variable) = variable {
                    let path = path.join(format!("{}.json", variable.borrow().id));
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
            for variable_expression in &self.variable_expression {
                if let Some(variable_expression) = variable_expression {
                    let path = path.join(format!("{}.json", variable_expression.borrow().id));
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
                let argument: Rc<RefCell<Argument>> = serde_json::from_reader(reader)?;
                store
                    .argument
                    .insert(argument.borrow().id, Some(argument.clone()));
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
                let a_wait: Rc<RefCell<AWait>> = serde_json::from_reader(reader)?;
                store
                    .a_wait
                    .insert(a_wait.borrow().id, Some(a_wait.clone()));
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
                let binary: Rc<RefCell<Binary>> = serde_json::from_reader(reader)?;
                store
                    .binary
                    .insert(binary.borrow().id, Some(binary.clone()));
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
                let block: Rc<RefCell<Block>> = serde_json::from_reader(reader)?;
                store.block.insert(block.borrow().id, Some(block.clone()));
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
                let body: Rc<RefCell<Body>> = serde_json::from_reader(reader)?;
                store.body.insert(body.borrow().id, Some(body.clone()));
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
                let boolean_literal: Rc<RefCell<BooleanLiteral>> = serde_json::from_reader(reader)?;
                store
                    .boolean_literal
                    .insert(boolean_literal.borrow().id, Some(boolean_literal.clone()));
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
                let boolean_operator: Rc<RefCell<BooleanOperator>> =
                    serde_json::from_reader(reader)?;
                store
                    .boolean_operator
                    .insert(boolean_operator.borrow().id, Some(boolean_operator.clone()));
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
                let call: Rc<RefCell<Call>> = serde_json::from_reader(reader)?;
                store.call.insert(call.borrow().id, Some(call.clone()));
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
                let char_literal: Rc<RefCell<CharLiteral>> = serde_json::from_reader(reader)?;
                store
                    .char_literal
                    .insert(char_literal.borrow().id, Some(char_literal.clone()));
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
                let comparison: Rc<RefCell<Comparison>> = serde_json::from_reader(reader)?;
                store
                    .comparison
                    .insert(comparison.borrow().id, Some(comparison.clone()));
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
                let data_structure: Rc<RefCell<DataStructure>> = serde_json::from_reader(reader)?;
                store
                    .data_structure
                    .insert(data_structure.borrow().id, Some(data_structure.clone()));
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
                let dwarf_source_file: Rc<RefCell<DwarfSourceFile>> =
                    serde_json::from_reader(reader)?;
                store.dwarf_source_file.insert(
                    dwarf_source_file.borrow().id,
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
                let enum_field: Rc<RefCell<EnumField>> = serde_json::from_reader(reader)?;
                store
                    .enum_field
                    .insert(enum_field.borrow().id, Some(enum_field.clone()));
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
                let enum_generic: Rc<RefCell<EnumGeneric>> = serde_json::from_reader(reader)?;
                store
                    .enum_generic
                    .insert(enum_generic.borrow().id, Some(enum_generic.clone()));
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
                let enum_generic_type: Rc<RefCell<EnumGenericType>> =
                    serde_json::from_reader(reader)?;
                store.enum_generic_type.insert(
                    enum_generic_type.borrow().id,
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
                let enumeration: Rc<RefCell<Enumeration>> = serde_json::from_reader(reader)?;
                store.enumeration_id_by_name.insert(
                    enumeration.borrow().name.to_owned(),
                    enumeration.borrow().id,
                );
                store
                    .enumeration
                    .insert(enumeration.borrow().id, Some(enumeration.clone()));
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
                let expression: Rc<RefCell<Expression>> = serde_json::from_reader(reader)?;
                store
                    .expression
                    .insert(expression.borrow().id, Some(expression.clone()));
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
                let expression_bit: Rc<RefCell<ExpressionBit>> = serde_json::from_reader(reader)?;
                store
                    .expression_bit
                    .insert(expression_bit.borrow().id, Some(expression_bit.clone()));
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
                let expression_statement: Rc<RefCell<ExpressionStatement>> =
                    serde_json::from_reader(reader)?;
                store.expression_statement.insert(
                    expression_statement.borrow().id,
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
                let external_implementation: Rc<RefCell<ExternalImplementation>> =
                    serde_json::from_reader(reader)?;
                store.external_implementation.insert(
                    external_implementation.borrow().id,
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
                let field: Rc<RefCell<Field>> = serde_json::from_reader(reader)?;
                store
                    .field_id_by_name
                    .insert(field.borrow().name.to_owned(), field.borrow().id);
                store.field.insert(field.borrow().id, Some(field.clone()));
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
                let field_access: Rc<RefCell<FieldAccess>> = serde_json::from_reader(reader)?;
                store
                    .field_access
                    .insert(field_access.borrow().id, Some(field_access.clone()));
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
                let field_access_target: Rc<RefCell<FieldAccessTarget>> =
                    serde_json::from_reader(reader)?;
                store.field_access_target.insert(
                    field_access_target.borrow().id,
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
                let field_expression: Rc<RefCell<FieldExpression>> =
                    serde_json::from_reader(reader)?;
                store
                    .field_expression
                    .insert(field_expression.borrow().id, Some(field_expression.clone()));
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
                let float_literal: Rc<RefCell<FloatLiteral>> = serde_json::from_reader(reader)?;
                store
                    .float_literal
                    .insert(float_literal.borrow().id, Some(float_literal.clone()));
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
                let for_loop: Rc<RefCell<ForLoop>> = serde_json::from_reader(reader)?;
                store
                    .for_loop
                    .insert(for_loop.borrow().id, Some(for_loop.clone()));
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
                let format_bit: Rc<RefCell<FormatBit>> = serde_json::from_reader(reader)?;
                store
                    .format_bit
                    .insert(format_bit.borrow().id, Some(format_bit.clone()));
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
                let format_string: Rc<RefCell<FormatString>> = serde_json::from_reader(reader)?;
                store
                    .format_string
                    .insert(format_string.borrow().id, Some(format_string.clone()));
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
                let func_generic: Rc<RefCell<FuncGeneric>> = serde_json::from_reader(reader)?;
                store
                    .func_generic
                    .insert(func_generic.borrow().id, Some(func_generic.clone()));
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
                let function: Rc<RefCell<Function>> = serde_json::from_reader(reader)?;
                store
                    .function_id_by_name
                    .insert(function.borrow().name.to_owned(), function.borrow().id);
                store
                    .function
                    .insert(function.borrow().id, Some(function.clone()));
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
                let function_call: Rc<RefCell<FunctionCall>> = serde_json::from_reader(reader)?;
                store
                    .function_call
                    .insert(function_call.borrow().id, Some(function_call.clone()));
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
                let x_future: Rc<RefCell<XFuture>> = serde_json::from_reader(reader)?;
                store
                    .x_future
                    .insert(x_future.borrow().id, Some(x_future.clone()));
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
                let grouped: Rc<RefCell<Grouped>> = serde_json::from_reader(reader)?;
                store
                    .grouped
                    .insert(grouped.borrow().id, Some(grouped.clone()));
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
                let halt_and_catch_fire: Rc<RefCell<HaltAndCatchFire>> =
                    serde_json::from_reader(reader)?;
                store.halt_and_catch_fire.insert(
                    halt_and_catch_fire.borrow().id,
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
                let x_if: Rc<RefCell<XIf>> = serde_json::from_reader(reader)?;
                store.x_if.insert(x_if.borrow().id, Some(x_if.clone()));
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
                let implementation_block: Rc<RefCell<ImplementationBlock>> =
                    serde_json::from_reader(reader)?;
                store.implementation_block.insert(
                    implementation_block.borrow().id,
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
                let import: Rc<RefCell<Import>> = serde_json::from_reader(reader)?;
                store
                    .import
                    .insert(import.borrow().id, Some(import.clone()));
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
                let index: Rc<RefCell<Index>> = serde_json::from_reader(reader)?;
                store.index.insert(index.borrow().id, Some(index.clone()));
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
                let integer_literal: Rc<RefCell<IntegerLiteral>> = serde_json::from_reader(reader)?;
                store
                    .integer_literal
                    .insert(integer_literal.borrow().id, Some(integer_literal.clone()));
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
                let item: Rc<RefCell<Item>> = serde_json::from_reader(reader)?;
                store.item.insert(item.borrow().id, Some(item.clone()));
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
                let lambda: Rc<RefCell<Lambda>> = serde_json::from_reader(reader)?;
                store
                    .lambda
                    .insert(lambda.borrow().id, Some(lambda.clone()));
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
                let lambda_parameter: Rc<RefCell<LambdaParameter>> =
                    serde_json::from_reader(reader)?;
                store
                    .lambda_parameter
                    .insert(lambda_parameter.borrow().id, Some(lambda_parameter.clone()));
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
                let let_statement: Rc<RefCell<LetStatement>> = serde_json::from_reader(reader)?;
                store
                    .let_statement
                    .insert(let_statement.borrow().id, Some(let_statement.clone()));
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
                let list: Rc<RefCell<List>> = serde_json::from_reader(reader)?;
                store.list.insert(list.borrow().id, Some(list.clone()));
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
                let list_element: Rc<RefCell<ListElement>> = serde_json::from_reader(reader)?;
                store
                    .list_element
                    .insert(list_element.borrow().id, Some(list_element.clone()));
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
                let list_expression: Rc<RefCell<ListExpression>> = serde_json::from_reader(reader)?;
                store
                    .list_expression
                    .insert(list_expression.borrow().id, Some(list_expression.clone()));
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
                let literal: Rc<RefCell<Literal>> = serde_json::from_reader(reader)?;
                store
                    .literal
                    .insert(literal.borrow().id, Some(literal.clone()));
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
                let local_variable: Rc<RefCell<LocalVariable>> = serde_json::from_reader(reader)?;
                store
                    .local_variable
                    .insert(local_variable.borrow().id, Some(local_variable.clone()));
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
                let x_macro: Rc<RefCell<XMacro>> = serde_json::from_reader(reader)?;
                store
                    .x_macro
                    .insert(x_macro.borrow().id, Some(x_macro.clone()));
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
                let map: Rc<RefCell<Map>> = serde_json::from_reader(reader)?;
                store.map.insert(map.borrow().id, Some(map.clone()));
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
                let map_element: Rc<RefCell<MapElement>> = serde_json::from_reader(reader)?;
                store
                    .map_element
                    .insert(map_element.borrow().id, Some(map_element.clone()));
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
                let map_expression: Rc<RefCell<MapExpression>> = serde_json::from_reader(reader)?;
                store
                    .map_expression
                    .insert(map_expression.borrow().id, Some(map_expression.clone()));
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
                let x_match: Rc<RefCell<XMatch>> = serde_json::from_reader(reader)?;
                store
                    .x_match
                    .insert(x_match.borrow().id, Some(x_match.clone()));
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
                let method_call: Rc<RefCell<MethodCall>> = serde_json::from_reader(reader)?;
                store
                    .method_call
                    .insert(method_call.borrow().id, Some(method_call.clone()));
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
                let named_field_expression: Rc<RefCell<NamedFieldExpression>> =
                    serde_json::from_reader(reader)?;
                store.named_field_expression.insert(
                    named_field_expression.borrow().id,
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
                let z_object_store: Rc<RefCell<ZObjectStore>> = serde_json::from_reader(reader)?;
                store.z_object_store_id_by_name.insert(
                    z_object_store.borrow().name.to_owned(),
                    z_object_store.borrow().id,
                );
                store
                    .z_object_store
                    .insert(z_object_store.borrow().id, Some(z_object_store.clone()));
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
                let object_wrapper: Rc<RefCell<ObjectWrapper>> = serde_json::from_reader(reader)?;
                store
                    .object_wrapper
                    .insert(object_wrapper.borrow().id, Some(object_wrapper.clone()));
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
                let operator: Rc<RefCell<Operator>> = serde_json::from_reader(reader)?;
                store
                    .operator
                    .insert(operator.borrow().id, Some(operator.clone()));
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
                let parameter: Rc<RefCell<Parameter>> = serde_json::from_reader(reader)?;
                store
                    .parameter
                    .insert(parameter.borrow().id, Some(parameter.clone()));
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
                let x_path: Rc<RefCell<XPath>> = serde_json::from_reader(reader)?;
                store
                    .x_path
                    .insert(x_path.borrow().id, Some(x_path.clone()));
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
                let path_element: Rc<RefCell<PathElement>> = serde_json::from_reader(reader)?;
                store
                    .path_element
                    .insert(path_element.borrow().id, Some(path_element.clone()));
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
                let pattern: Rc<RefCell<Pattern>> = serde_json::from_reader(reader)?;
                store
                    .pattern
                    .insert(pattern.borrow().id, Some(pattern.clone()));
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
                let x_plugin: Rc<RefCell<XPlugin>> = serde_json::from_reader(reader)?;
                store
                    .x_plugin_id_by_name
                    .insert(x_plugin.borrow().name.to_owned(), x_plugin.borrow().id);
                store
                    .x_plugin
                    .insert(x_plugin.borrow().id, Some(x_plugin.clone()));
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
                let x_print: Rc<RefCell<XPrint>> = serde_json::from_reader(reader)?;
                store
                    .x_print
                    .insert(x_print.borrow().id, Some(x_print.clone()));
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
                let range_expression: Rc<RefCell<RangeExpression>> =
                    serde_json::from_reader(reader)?;
                store
                    .range_expression
                    .insert(range_expression.borrow().id, Some(range_expression.clone()));
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
                let result_statement: Rc<RefCell<ResultStatement>> =
                    serde_json::from_reader(reader)?;
                store
                    .result_statement
                    .insert(result_statement.borrow().id, Some(result_statement.clone()));
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
                let x_return: Rc<RefCell<XReturn>> = serde_json::from_reader(reader)?;
                store
                    .x_return
                    .insert(x_return.borrow().id, Some(x_return.clone()));
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
                let span: Rc<RefCell<Span>> = serde_json::from_reader(reader)?;
                store.span.insert(span.borrow().id, Some(span.clone()));
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
                let statement: Rc<RefCell<Statement>> = serde_json::from_reader(reader)?;
                store
                    .statement
                    .insert(statement.borrow().id, Some(statement.clone()));
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
                let static_method_call: Rc<RefCell<StaticMethodCall>> =
                    serde_json::from_reader(reader)?;
                store.static_method_call.insert(
                    static_method_call.borrow().id,
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
                let string_bit: Rc<RefCell<StringBit>> = serde_json::from_reader(reader)?;
                store
                    .string_bit
                    .insert(string_bit.borrow().id, Some(string_bit.clone()));
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
                let string_literal: Rc<RefCell<StringLiteral>> = serde_json::from_reader(reader)?;
                store
                    .string_literal
                    .insert(string_literal.borrow().id, Some(string_literal.clone()));
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
                let woog_struct: Rc<RefCell<WoogStruct>> = serde_json::from_reader(reader)?;
                store.woog_struct_id_by_name.insert(
                    woog_struct.borrow().name.to_owned(),
                    woog_struct.borrow().id,
                );
                store
                    .woog_struct
                    .insert(woog_struct.borrow().id, Some(woog_struct.clone()));
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
                let struct_expression: Rc<RefCell<StructExpression>> =
                    serde_json::from_reader(reader)?;
                store.struct_expression.insert(
                    struct_expression.borrow().id,
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
                let struct_field: Rc<RefCell<StructField>> = serde_json::from_reader(reader)?;
                store
                    .struct_field
                    .insert(struct_field.borrow().id, Some(struct_field.clone()));
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
                let struct_generic: Rc<RefCell<StructGeneric>> = serde_json::from_reader(reader)?;
                store
                    .struct_generic
                    .insert(struct_generic.borrow().id, Some(struct_generic.clone()));
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
                let tuple_field: Rc<RefCell<TupleField>> = serde_json::from_reader(reader)?;
                store
                    .tuple_field
                    .insert(tuple_field.borrow().id, Some(tuple_field.clone()));
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
                let type_cast: Rc<RefCell<TypeCast>> = serde_json::from_reader(reader)?;
                store
                    .type_cast
                    .insert(type_cast.borrow().id, Some(type_cast.clone()));
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
                let unary: Rc<RefCell<Unary>> = serde_json::from_reader(reader)?;
                store.unary.insert(unary.borrow().id, Some(unary.clone()));
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
                let unit: Rc<RefCell<Unit>> = serde_json::from_reader(reader)?;
                store.unit.insert(unit.borrow().id, Some(unit.clone()));
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
                let unnamed_field_expression: Rc<RefCell<UnnamedFieldExpression>> =
                    serde_json::from_reader(reader)?;
                store.unnamed_field_expression.insert(
                    unnamed_field_expression.borrow().id,
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
                let x_value: Rc<RefCell<XValue>> = serde_json::from_reader(reader)?;
                store
                    .x_value
                    .insert(x_value.borrow().id, Some(x_value.clone()));
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
                let value_type: Rc<RefCell<ValueType>> = serde_json::from_reader(reader)?;
                store
                    .value_type
                    .insert(value_type.borrow().id, Some(value_type.clone()));
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
                let variable: Rc<RefCell<Variable>> = serde_json::from_reader(reader)?;
                store
                    .variable
                    .insert(variable.borrow().id, Some(variable.clone()));
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
                let variable_expression: Rc<RefCell<VariableExpression>> =
                    serde_json::from_reader(reader)?;
                store.variable_expression.insert(
                    variable_expression.borrow().id,
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
