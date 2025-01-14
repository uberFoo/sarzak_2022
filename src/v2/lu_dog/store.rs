//! v2::lu_dog Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-definition"}}}
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use ordered_hash_map::OrderedHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog::types::{
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
    argument: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Argument>>>>>,
    a_wait: Rc<RefCell<HashMap<Uuid, Rc<RefCell<AWait>>>>>,
    binary: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Binary>>>>>,
    block: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Block>>>>>,
    body: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Body>>>>>,
    boolean_literal: Rc<RefCell<HashMap<Uuid, Rc<RefCell<BooleanLiteral>>>>>,
    boolean_operator: Rc<RefCell<HashMap<Uuid, Rc<RefCell<BooleanOperator>>>>>,
    call: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Call>>>>>,
    char_literal: Rc<RefCell<HashMap<Uuid, Rc<RefCell<CharLiteral>>>>>,
    comparison: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Comparison>>>>>,
    data_structure: Rc<RefCell<HashMap<Uuid, Rc<RefCell<DataStructure>>>>>,
    dwarf_source_file: Rc<RefCell<HashMap<Uuid, Rc<RefCell<DwarfSourceFile>>>>>,
    enum_field: Rc<RefCell<HashMap<Uuid, Rc<RefCell<EnumField>>>>>,
    enum_generic: Rc<RefCell<HashMap<Uuid, Rc<RefCell<EnumGeneric>>>>>,
    enum_generic_type: Rc<RefCell<HashMap<Uuid, Rc<RefCell<EnumGenericType>>>>>,
    enumeration: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Enumeration>>>>>,
    enumeration_id_by_name: Rc<RefCell<HashMap<String, Uuid>>>,
    expression: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Expression>>>>>,
    expression_bit: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ExpressionBit>>>>>,
    expression_statement: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ExpressionStatement>>>>>,
    external_implementation: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ExternalImplementation>>>>>,
    field: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Field>>>>>,
    field_id_by_name: Rc<RefCell<HashMap<String, Uuid>>>,
    field_access: Rc<RefCell<HashMap<Uuid, Rc<RefCell<FieldAccess>>>>>,
    field_access_target: Rc<RefCell<HashMap<Uuid, Rc<RefCell<FieldAccessTarget>>>>>,
    field_expression: Rc<RefCell<HashMap<Uuid, Rc<RefCell<FieldExpression>>>>>,
    float_literal: Rc<RefCell<HashMap<Uuid, Rc<RefCell<FloatLiteral>>>>>,
    for_loop: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ForLoop>>>>>,
    format_bit: Rc<RefCell<HashMap<Uuid, Rc<RefCell<FormatBit>>>>>,
    format_string: Rc<RefCell<HashMap<Uuid, Rc<RefCell<FormatString>>>>>,
    func_generic: Rc<RefCell<HashMap<Uuid, Rc<RefCell<FuncGeneric>>>>>,
    function: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Function>>>>>,
    function_id_by_name: Rc<RefCell<HashMap<String, Uuid>>>,
    function_call: Rc<RefCell<HashMap<Uuid, Rc<RefCell<FunctionCall>>>>>,
    x_future: Rc<RefCell<HashMap<Uuid, Rc<RefCell<XFuture>>>>>,
    grouped: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Grouped>>>>>,
    halt_and_catch_fire: Rc<RefCell<HashMap<Uuid, Rc<RefCell<HaltAndCatchFire>>>>>,
    x_if: Rc<RefCell<HashMap<Uuid, Rc<RefCell<XIf>>>>>,
    implementation_block: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ImplementationBlock>>>>>,
    import: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Import>>>>>,
    index: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Index>>>>>,
    integer_literal: Rc<RefCell<HashMap<Uuid, Rc<RefCell<IntegerLiteral>>>>>,
    item: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Item>>>>>,
    lambda: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Lambda>>>>>,
    lambda_parameter: Rc<RefCell<HashMap<Uuid, Rc<RefCell<LambdaParameter>>>>>,
    let_statement: Rc<RefCell<HashMap<Uuid, Rc<RefCell<LetStatement>>>>>,
    list: Rc<RefCell<HashMap<Uuid, Rc<RefCell<List>>>>>,
    list_element: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ListElement>>>>>,
    list_expression: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ListExpression>>>>>,
    literal: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Literal>>>>>,
    local_variable: Rc<RefCell<HashMap<Uuid, Rc<RefCell<LocalVariable>>>>>,
    x_macro: Rc<RefCell<HashMap<Uuid, Rc<RefCell<XMacro>>>>>,
    map: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Map>>>>>,
    map_element: Rc<RefCell<HashMap<Uuid, Rc<RefCell<MapElement>>>>>,
    map_expression: Rc<RefCell<HashMap<Uuid, Rc<RefCell<MapExpression>>>>>,
    x_match: Rc<RefCell<HashMap<Uuid, Rc<RefCell<XMatch>>>>>,
    method_call: Rc<RefCell<HashMap<Uuid, Rc<RefCell<MethodCall>>>>>,
    named_field_expression: Rc<RefCell<HashMap<Uuid, Rc<RefCell<NamedFieldExpression>>>>>,
    z_object_store: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ZObjectStore>>>>>,
    z_object_store_id_by_name: Rc<RefCell<HashMap<String, Uuid>>>,
    object_wrapper: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ObjectWrapper>>>>>,
    operator: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Operator>>>>>,
    parameter: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Parameter>>>>>,
    x_path: Rc<RefCell<HashMap<Uuid, Rc<RefCell<XPath>>>>>,
    path_element: Rc<RefCell<HashMap<Uuid, Rc<RefCell<PathElement>>>>>,
    pattern: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Pattern>>>>>,
    x_plugin: Rc<RefCell<HashMap<Uuid, Rc<RefCell<XPlugin>>>>>,
    x_plugin_id_by_name: Rc<RefCell<HashMap<String, Uuid>>>,
    x_print: Rc<RefCell<HashMap<Uuid, Rc<RefCell<XPrint>>>>>,
    range_expression: Rc<RefCell<HashMap<Uuid, Rc<RefCell<RangeExpression>>>>>,
    result_statement: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ResultStatement>>>>>,
    x_return: Rc<RefCell<HashMap<Uuid, Rc<RefCell<XReturn>>>>>,
    span: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Span>>>>>,
    statement: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Statement>>>>>,
    static_method_call: Rc<RefCell<HashMap<Uuid, Rc<RefCell<StaticMethodCall>>>>>,
    string_bit: Rc<RefCell<HashMap<Uuid, Rc<RefCell<StringBit>>>>>,
    string_literal: Rc<RefCell<HashMap<Uuid, Rc<RefCell<StringLiteral>>>>>,
    woog_struct: Rc<RefCell<HashMap<Uuid, Rc<RefCell<WoogStruct>>>>>,
    woog_struct_id_by_name: Rc<RefCell<HashMap<String, Uuid>>>,
    struct_expression: Rc<RefCell<HashMap<Uuid, Rc<RefCell<StructExpression>>>>>,
    struct_field: Rc<RefCell<HashMap<Uuid, Rc<RefCell<StructField>>>>>,
    struct_generic: Rc<RefCell<HashMap<Uuid, Rc<RefCell<StructGeneric>>>>>,
    tuple_field: Rc<RefCell<HashMap<Uuid, Rc<RefCell<TupleField>>>>>,
    type_cast: Rc<RefCell<HashMap<Uuid, Rc<RefCell<TypeCast>>>>>,
    unary: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Unary>>>>>,
    unit: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Unit>>>>>,
    unnamed_field_expression: Rc<RefCell<HashMap<Uuid, Rc<RefCell<UnnamedFieldExpression>>>>>,
    x_value: Rc<RefCell<HashMap<Uuid, Rc<RefCell<XValue>>>>>,
    value_type: Rc<RefCell<HashMap<Uuid, Rc<RefCell<ValueType>>>>>,
    variable: Rc<RefCell<HashMap<Uuid, Rc<RefCell<Variable>>>>>,
    variable_expression: Rc<RefCell<HashMap<Uuid, Rc<RefCell<VariableExpression>>>>>,
}

impl Clone for ObjectStore {
    fn clone(&self) -> Self {
        ObjectStore {
            argument: Rc::new(RefCell::new(self.argument.borrow().clone())),
            a_wait: Rc::new(RefCell::new(self.a_wait.borrow().clone())),
            binary: Rc::new(RefCell::new(self.binary.borrow().clone())),
            block: Rc::new(RefCell::new(self.block.borrow().clone())),
            body: Rc::new(RefCell::new(self.body.borrow().clone())),
            boolean_literal: Rc::new(RefCell::new(self.boolean_literal.borrow().clone())),
            boolean_operator: Rc::new(RefCell::new(self.boolean_operator.borrow().clone())),
            call: Rc::new(RefCell::new(self.call.borrow().clone())),
            char_literal: Rc::new(RefCell::new(self.char_literal.borrow().clone())),
            comparison: Rc::new(RefCell::new(self.comparison.borrow().clone())),
            data_structure: Rc::new(RefCell::new(self.data_structure.borrow().clone())),
            dwarf_source_file: Rc::new(RefCell::new(self.dwarf_source_file.borrow().clone())),
            enum_field: Rc::new(RefCell::new(self.enum_field.borrow().clone())),
            enum_generic: Rc::new(RefCell::new(self.enum_generic.borrow().clone())),
            enum_generic_type: Rc::new(RefCell::new(self.enum_generic_type.borrow().clone())),
            enumeration: Rc::new(RefCell::new(self.enumeration.borrow().clone())),
            enumeration_id_by_name: Rc::new(RefCell::new(
                self.enumeration_id_by_name.borrow().clone(),
            )),
            expression: Rc::new(RefCell::new(self.expression.borrow().clone())),
            expression_bit: Rc::new(RefCell::new(self.expression_bit.borrow().clone())),
            expression_statement: Rc::new(RefCell::new(self.expression_statement.borrow().clone())),
            external_implementation: Rc::new(RefCell::new(
                self.external_implementation.borrow().clone(),
            )),
            field: Rc::new(RefCell::new(self.field.borrow().clone())),
            field_id_by_name: Rc::new(RefCell::new(self.field_id_by_name.borrow().clone())),
            field_access: Rc::new(RefCell::new(self.field_access.borrow().clone())),
            field_access_target: Rc::new(RefCell::new(self.field_access_target.borrow().clone())),
            field_expression: Rc::new(RefCell::new(self.field_expression.borrow().clone())),
            float_literal: Rc::new(RefCell::new(self.float_literal.borrow().clone())),
            for_loop: Rc::new(RefCell::new(self.for_loop.borrow().clone())),
            format_bit: Rc::new(RefCell::new(self.format_bit.borrow().clone())),
            format_string: Rc::new(RefCell::new(self.format_string.borrow().clone())),
            func_generic: Rc::new(RefCell::new(self.func_generic.borrow().clone())),
            function: Rc::new(RefCell::new(self.function.borrow().clone())),
            function_id_by_name: Rc::new(RefCell::new(self.function_id_by_name.borrow().clone())),
            function_call: Rc::new(RefCell::new(self.function_call.borrow().clone())),
            x_future: Rc::new(RefCell::new(self.x_future.borrow().clone())),
            grouped: Rc::new(RefCell::new(self.grouped.borrow().clone())),
            halt_and_catch_fire: Rc::new(RefCell::new(self.halt_and_catch_fire.borrow().clone())),
            x_if: Rc::new(RefCell::new(self.x_if.borrow().clone())),
            implementation_block: Rc::new(RefCell::new(self.implementation_block.borrow().clone())),
            import: Rc::new(RefCell::new(self.import.borrow().clone())),
            index: Rc::new(RefCell::new(self.index.borrow().clone())),
            integer_literal: Rc::new(RefCell::new(self.integer_literal.borrow().clone())),
            item: Rc::new(RefCell::new(self.item.borrow().clone())),
            lambda: Rc::new(RefCell::new(self.lambda.borrow().clone())),
            lambda_parameter: Rc::new(RefCell::new(self.lambda_parameter.borrow().clone())),
            let_statement: Rc::new(RefCell::new(self.let_statement.borrow().clone())),
            list: Rc::new(RefCell::new(self.list.borrow().clone())),
            list_element: Rc::new(RefCell::new(self.list_element.borrow().clone())),
            list_expression: Rc::new(RefCell::new(self.list_expression.borrow().clone())),
            literal: Rc::new(RefCell::new(self.literal.borrow().clone())),
            local_variable: Rc::new(RefCell::new(self.local_variable.borrow().clone())),
            x_macro: Rc::new(RefCell::new(self.x_macro.borrow().clone())),
            map: Rc::new(RefCell::new(self.map.borrow().clone())),
            map_element: Rc::new(RefCell::new(self.map_element.borrow().clone())),
            map_expression: Rc::new(RefCell::new(self.map_expression.borrow().clone())),
            x_match: Rc::new(RefCell::new(self.x_match.borrow().clone())),
            method_call: Rc::new(RefCell::new(self.method_call.borrow().clone())),
            named_field_expression: Rc::new(RefCell::new(
                self.named_field_expression.borrow().clone(),
            )),
            z_object_store: Rc::new(RefCell::new(self.z_object_store.borrow().clone())),
            z_object_store_id_by_name: Rc::new(RefCell::new(
                self.z_object_store_id_by_name.borrow().clone(),
            )),
            object_wrapper: Rc::new(RefCell::new(self.object_wrapper.borrow().clone())),
            operator: Rc::new(RefCell::new(self.operator.borrow().clone())),
            parameter: Rc::new(RefCell::new(self.parameter.borrow().clone())),
            x_path: Rc::new(RefCell::new(self.x_path.borrow().clone())),
            path_element: Rc::new(RefCell::new(self.path_element.borrow().clone())),
            pattern: Rc::new(RefCell::new(self.pattern.borrow().clone())),
            x_plugin: Rc::new(RefCell::new(self.x_plugin.borrow().clone())),
            x_plugin_id_by_name: Rc::new(RefCell::new(self.x_plugin_id_by_name.borrow().clone())),
            x_print: Rc::new(RefCell::new(self.x_print.borrow().clone())),
            range_expression: Rc::new(RefCell::new(self.range_expression.borrow().clone())),
            result_statement: Rc::new(RefCell::new(self.result_statement.borrow().clone())),
            x_return: Rc::new(RefCell::new(self.x_return.borrow().clone())),
            span: Rc::new(RefCell::new(self.span.borrow().clone())),
            statement: Rc::new(RefCell::new(self.statement.borrow().clone())),
            static_method_call: Rc::new(RefCell::new(self.static_method_call.borrow().clone())),
            string_bit: Rc::new(RefCell::new(self.string_bit.borrow().clone())),
            string_literal: Rc::new(RefCell::new(self.string_literal.borrow().clone())),
            woog_struct: Rc::new(RefCell::new(self.woog_struct.borrow().clone())),
            woog_struct_id_by_name: Rc::new(RefCell::new(
                self.woog_struct_id_by_name.borrow().clone(),
            )),
            struct_expression: Rc::new(RefCell::new(self.struct_expression.borrow().clone())),
            struct_field: Rc::new(RefCell::new(self.struct_field.borrow().clone())),
            struct_generic: Rc::new(RefCell::new(self.struct_generic.borrow().clone())),
            tuple_field: Rc::new(RefCell::new(self.tuple_field.borrow().clone())),
            type_cast: Rc::new(RefCell::new(self.type_cast.borrow().clone())),
            unary: Rc::new(RefCell::new(self.unary.borrow().clone())),
            unit: Rc::new(RefCell::new(self.unit.borrow().clone())),
            unnamed_field_expression: Rc::new(RefCell::new(
                self.unnamed_field_expression.borrow().clone(),
            )),
            x_value: Rc::new(RefCell::new(self.x_value.borrow().clone())),
            value_type: Rc::new(RefCell::new(self.value_type.borrow().clone())),
            variable: Rc::new(RefCell::new(self.variable.borrow().clone())),
            variable_expression: Rc::new(RefCell::new(self.variable_expression.borrow().clone())),
        }
    }
}
impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            argument: Rc::new(RefCell::new(HashMap::default())),
            a_wait: Rc::new(RefCell::new(HashMap::default())),
            binary: Rc::new(RefCell::new(HashMap::default())),
            block: Rc::new(RefCell::new(HashMap::default())),
            body: Rc::new(RefCell::new(HashMap::default())),
            boolean_literal: Rc::new(RefCell::new(HashMap::default())),
            boolean_operator: Rc::new(RefCell::new(HashMap::default())),
            call: Rc::new(RefCell::new(HashMap::default())),
            char_literal: Rc::new(RefCell::new(HashMap::default())),
            comparison: Rc::new(RefCell::new(HashMap::default())),
            data_structure: Rc::new(RefCell::new(HashMap::default())),
            dwarf_source_file: Rc::new(RefCell::new(HashMap::default())),
            enum_field: Rc::new(RefCell::new(HashMap::default())),
            enum_generic: Rc::new(RefCell::new(HashMap::default())),
            enum_generic_type: Rc::new(RefCell::new(HashMap::default())),
            enumeration: Rc::new(RefCell::new(HashMap::default())),
            enumeration_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            expression: Rc::new(RefCell::new(HashMap::default())),
            expression_bit: Rc::new(RefCell::new(HashMap::default())),
            expression_statement: Rc::new(RefCell::new(HashMap::default())),
            external_implementation: Rc::new(RefCell::new(HashMap::default())),
            field: Rc::new(RefCell::new(HashMap::default())),
            field_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            field_access: Rc::new(RefCell::new(HashMap::default())),
            field_access_target: Rc::new(RefCell::new(HashMap::default())),
            field_expression: Rc::new(RefCell::new(HashMap::default())),
            float_literal: Rc::new(RefCell::new(HashMap::default())),
            for_loop: Rc::new(RefCell::new(HashMap::default())),
            format_bit: Rc::new(RefCell::new(HashMap::default())),
            format_string: Rc::new(RefCell::new(HashMap::default())),
            func_generic: Rc::new(RefCell::new(HashMap::default())),
            function: Rc::new(RefCell::new(HashMap::default())),
            function_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            function_call: Rc::new(RefCell::new(HashMap::default())),
            x_future: Rc::new(RefCell::new(HashMap::default())),
            grouped: Rc::new(RefCell::new(HashMap::default())),
            halt_and_catch_fire: Rc::new(RefCell::new(HashMap::default())),
            x_if: Rc::new(RefCell::new(HashMap::default())),
            implementation_block: Rc::new(RefCell::new(HashMap::default())),
            import: Rc::new(RefCell::new(HashMap::default())),
            index: Rc::new(RefCell::new(HashMap::default())),
            integer_literal: Rc::new(RefCell::new(HashMap::default())),
            item: Rc::new(RefCell::new(HashMap::default())),
            lambda: Rc::new(RefCell::new(HashMap::default())),
            lambda_parameter: Rc::new(RefCell::new(HashMap::default())),
            let_statement: Rc::new(RefCell::new(HashMap::default())),
            list: Rc::new(RefCell::new(HashMap::default())),
            list_element: Rc::new(RefCell::new(HashMap::default())),
            list_expression: Rc::new(RefCell::new(HashMap::default())),
            literal: Rc::new(RefCell::new(HashMap::default())),
            local_variable: Rc::new(RefCell::new(HashMap::default())),
            x_macro: Rc::new(RefCell::new(HashMap::default())),
            map: Rc::new(RefCell::new(HashMap::default())),
            map_element: Rc::new(RefCell::new(HashMap::default())),
            map_expression: Rc::new(RefCell::new(HashMap::default())),
            x_match: Rc::new(RefCell::new(HashMap::default())),
            method_call: Rc::new(RefCell::new(HashMap::default())),
            named_field_expression: Rc::new(RefCell::new(HashMap::default())),
            z_object_store: Rc::new(RefCell::new(HashMap::default())),
            z_object_store_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            object_wrapper: Rc::new(RefCell::new(HashMap::default())),
            operator: Rc::new(RefCell::new(HashMap::default())),
            parameter: Rc::new(RefCell::new(HashMap::default())),
            x_path: Rc::new(RefCell::new(HashMap::default())),
            path_element: Rc::new(RefCell::new(HashMap::default())),
            pattern: Rc::new(RefCell::new(HashMap::default())),
            x_plugin: Rc::new(RefCell::new(HashMap::default())),
            x_plugin_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            x_print: Rc::new(RefCell::new(HashMap::default())),
            range_expression: Rc::new(RefCell::new(HashMap::default())),
            result_statement: Rc::new(RefCell::new(HashMap::default())),
            x_return: Rc::new(RefCell::new(HashMap::default())),
            span: Rc::new(RefCell::new(HashMap::default())),
            statement: Rc::new(RefCell::new(HashMap::default())),
            static_method_call: Rc::new(RefCell::new(HashMap::default())),
            string_bit: Rc::new(RefCell::new(HashMap::default())),
            string_literal: Rc::new(RefCell::new(HashMap::default())),
            woog_struct: Rc::new(RefCell::new(HashMap::default())),
            woog_struct_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            struct_expression: Rc::new(RefCell::new(HashMap::default())),
            struct_field: Rc::new(RefCell::new(HashMap::default())),
            struct_generic: Rc::new(RefCell::new(HashMap::default())),
            tuple_field: Rc::new(RefCell::new(HashMap::default())),
            type_cast: Rc::new(RefCell::new(HashMap::default())),
            unary: Rc::new(RefCell::new(HashMap::default())),
            unit: Rc::new(RefCell::new(HashMap::default())),
            unnamed_field_expression: Rc::new(RefCell::new(HashMap::default())),
            x_value: Rc::new(RefCell::new(HashMap::default())),
            value_type: Rc::new(RefCell::new(HashMap::default())),
            variable: Rc::new(RefCell::new(HashMap::default())),
            variable_expression: Rc::new(RefCell::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    pub fn merge(&mut self, other: &ObjectStore) {
        self.argument
            .borrow_mut()
            .extend(other.argument.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.a_wait
            .borrow_mut()
            .extend(other.a_wait.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.binary
            .borrow_mut()
            .extend(other.binary.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.block
            .borrow_mut()
            .extend(other.block.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.body
            .borrow_mut()
            .extend(other.body.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.boolean_literal.borrow_mut().extend(
            other
                .boolean_literal
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.boolean_operator.borrow_mut().extend(
            other
                .boolean_operator
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.call
            .borrow_mut()
            .extend(other.call.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.char_literal.borrow_mut().extend(
            other
                .char_literal
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.comparison.borrow_mut().extend(
            other
                .comparison
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.data_structure.borrow_mut().extend(
            other
                .data_structure
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.dwarf_source_file.borrow_mut().extend(
            other
                .dwarf_source_file
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enum_field.borrow_mut().extend(
            other
                .enum_field
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enum_generic.borrow_mut().extend(
            other
                .enum_generic
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enum_generic_type.borrow_mut().extend(
            other
                .enum_generic_type
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enumeration.borrow_mut().extend(
            other
                .enumeration
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.enumeration_id_by_name.borrow_mut().extend(
            other
                .enumeration_id_by_name
                .borrow()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.expression.borrow_mut().extend(
            other
                .expression
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.expression_bit.borrow_mut().extend(
            other
                .expression_bit
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.expression_statement.borrow_mut().extend(
            other
                .expression_statement
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.external_implementation.borrow_mut().extend(
            other
                .external_implementation
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.field
            .borrow_mut()
            .extend(other.field.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.field_id_by_name.borrow_mut().extend(
            other
                .field_id_by_name
                .borrow()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.field_access.borrow_mut().extend(
            other
                .field_access
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.field_access_target.borrow_mut().extend(
            other
                .field_access_target
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.field_expression.borrow_mut().extend(
            other
                .field_expression
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.float_literal.borrow_mut().extend(
            other
                .float_literal
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.for_loop
            .borrow_mut()
            .extend(other.for_loop.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.format_bit.borrow_mut().extend(
            other
                .format_bit
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.format_string.borrow_mut().extend(
            other
                .format_string
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.func_generic.borrow_mut().extend(
            other
                .func_generic
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.function
            .borrow_mut()
            .extend(other.function.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.function_id_by_name.borrow_mut().extend(
            other
                .function_id_by_name
                .borrow()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.function_call.borrow_mut().extend(
            other
                .function_call
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_future
            .borrow_mut()
            .extend(other.x_future.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.grouped
            .borrow_mut()
            .extend(other.grouped.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.halt_and_catch_fire.borrow_mut().extend(
            other
                .halt_and_catch_fire
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_if
            .borrow_mut()
            .extend(other.x_if.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.implementation_block.borrow_mut().extend(
            other
                .implementation_block
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.import
            .borrow_mut()
            .extend(other.import.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.index
            .borrow_mut()
            .extend(other.index.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.integer_literal.borrow_mut().extend(
            other
                .integer_literal
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.item
            .borrow_mut()
            .extend(other.item.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.lambda
            .borrow_mut()
            .extend(other.lambda.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.lambda_parameter.borrow_mut().extend(
            other
                .lambda_parameter
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.let_statement.borrow_mut().extend(
            other
                .let_statement
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.list
            .borrow_mut()
            .extend(other.list.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.list_element.borrow_mut().extend(
            other
                .list_element
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.list_expression.borrow_mut().extend(
            other
                .list_expression
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.literal
            .borrow_mut()
            .extend(other.literal.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.local_variable.borrow_mut().extend(
            other
                .local_variable
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_macro
            .borrow_mut()
            .extend(other.x_macro.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.map
            .borrow_mut()
            .extend(other.map.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.map_element.borrow_mut().extend(
            other
                .map_element
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.map_expression.borrow_mut().extend(
            other
                .map_expression
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_match
            .borrow_mut()
            .extend(other.x_match.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.method_call.borrow_mut().extend(
            other
                .method_call
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.named_field_expression.borrow_mut().extend(
            other
                .named_field_expression
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.z_object_store.borrow_mut().extend(
            other
                .z_object_store
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.z_object_store_id_by_name.borrow_mut().extend(
            other
                .z_object_store_id_by_name
                .borrow()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.object_wrapper.borrow_mut().extend(
            other
                .object_wrapper
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.operator
            .borrow_mut()
            .extend(other.operator.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.parameter.borrow_mut().extend(
            other
                .parameter
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_path
            .borrow_mut()
            .extend(other.x_path.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.path_element.borrow_mut().extend(
            other
                .path_element
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.pattern
            .borrow_mut()
            .extend(other.pattern.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.x_plugin
            .borrow_mut()
            .extend(other.x_plugin.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.x_plugin_id_by_name.borrow_mut().extend(
            other
                .x_plugin_id_by_name
                .borrow()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.x_print
            .borrow_mut()
            .extend(other.x_print.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.range_expression.borrow_mut().extend(
            other
                .range_expression
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.result_statement.borrow_mut().extend(
            other
                .result_statement
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_return
            .borrow_mut()
            .extend(other.x_return.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.span
            .borrow_mut()
            .extend(other.span.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.statement.borrow_mut().extend(
            other
                .statement
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.static_method_call.borrow_mut().extend(
            other
                .static_method_call
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.string_bit.borrow_mut().extend(
            other
                .string_bit
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.string_literal.borrow_mut().extend(
            other
                .string_literal
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.woog_struct.borrow_mut().extend(
            other
                .woog_struct
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.woog_struct_id_by_name.borrow_mut().extend(
            other
                .woog_struct_id_by_name
                .borrow()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        self.struct_expression.borrow_mut().extend(
            other
                .struct_expression
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.struct_field.borrow_mut().extend(
            other
                .struct_field
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.struct_generic.borrow_mut().extend(
            other
                .struct_generic
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.tuple_field.borrow_mut().extend(
            other
                .tuple_field
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.type_cast.borrow_mut().extend(
            other
                .type_cast
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.unary
            .borrow_mut()
            .extend(other.unary.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.unit
            .borrow_mut()
            .extend(other.unit.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.unnamed_field_expression.borrow_mut().extend(
            other
                .unnamed_field_expression
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.x_value
            .borrow_mut()
            .extend(other.x_value.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.value_type.borrow_mut().extend(
            other
                .value_type
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
        self.variable
            .borrow_mut()
            .extend(other.variable.borrow().iter().map(|(k, v)| (*k, v.clone())));
        self.variable_expression.borrow_mut().extend(
            other
                .variable_expression
                .borrow()
                .iter()
                .map(|(k, v)| (*k, v.clone())),
        );
    }
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument(&mut self, argument: Rc<RefCell<Argument>>) {
        let read = argument.borrow();
        self.argument.borrow_mut().insert(read.id, argument.clone());
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &Uuid) -> Option<Rc<RefCell<Argument>>> {
        self.argument
            .borrow()
            .get(id)
            .map(|argument| argument.clone())
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub fn exorcise_argument(&mut self, id: &Uuid) -> Option<Rc<RefCell<Argument>>> {
        self.argument
            .borrow_mut()
            .remove(id)
            .map(|argument| argument.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = Rc<RefCell<Argument>>> + '_ {
        let values: Vec<Rc<RefCell<Argument>>> = self
            .argument
            .borrow()
            .values()
            .map(|argument| argument.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`AWait`] into the store.
    ///
    pub fn inter_a_wait(&mut self, a_wait: Rc<RefCell<AWait>>) {
        let read = a_wait.borrow();
        self.a_wait.borrow_mut().insert(read.id, a_wait.clone());
    }

    /// Exhume (get) [`AWait`] from the store.
    ///
    pub fn exhume_a_wait(&self, id: &Uuid) -> Option<Rc<RefCell<AWait>>> {
        self.a_wait.borrow().get(id).map(|a_wait| a_wait.clone())
    }

    /// Exorcise (remove) [`AWait`] from the store.
    ///
    pub fn exorcise_a_wait(&mut self, id: &Uuid) -> Option<Rc<RefCell<AWait>>> {
        self.a_wait
            .borrow_mut()
            .remove(id)
            .map(|a_wait| a_wait.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AWait>`.
    ///
    pub fn iter_a_wait(&self) -> impl Iterator<Item = Rc<RefCell<AWait>>> + '_ {
        let values: Vec<Rc<RefCell<AWait>>> = self
            .a_wait
            .borrow()
            .values()
            .map(|a_wait| a_wait.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Rc<RefCell<Binary>>) {
        let read = binary.borrow();
        self.binary.borrow_mut().insert(read.id, binary.clone());
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<Rc<RefCell<Binary>>> {
        self.binary.borrow().get(id).map(|binary| binary.clone())
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &Uuid) -> Option<Rc<RefCell<Binary>>> {
        self.binary
            .borrow_mut()
            .remove(id)
            .map(|binary| binary.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = Rc<RefCell<Binary>>> + '_ {
        let values: Vec<Rc<RefCell<Binary>>> = self
            .binary
            .borrow()
            .values()
            .map(|binary| binary.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Rc<RefCell<Block>>) {
        let read = block.borrow();
        self.block.borrow_mut().insert(read.id, block.clone());
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<Rc<RefCell<Block>>> {
        self.block.borrow().get(id).map(|block| block.clone())
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &Uuid) -> Option<Rc<RefCell<Block>>> {
        self.block
            .borrow_mut()
            .remove(id)
            .map(|block| block.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = Rc<RefCell<Block>>> + '_ {
        let values: Vec<Rc<RefCell<Block>>> = self
            .block
            .borrow()
            .values()
            .map(|block| block.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Body`] into the store.
    ///
    pub fn inter_body(&mut self, body: Rc<RefCell<Body>>) {
        let read = body.borrow();
        self.body.borrow_mut().insert(read.id, body.clone());
    }

    /// Exhume (get) [`Body`] from the store.
    ///
    pub fn exhume_body(&self, id: &Uuid) -> Option<Rc<RefCell<Body>>> {
        self.body.borrow().get(id).map(|body| body.clone())
    }

    /// Exorcise (remove) [`Body`] from the store.
    ///
    pub fn exorcise_body(&mut self, id: &Uuid) -> Option<Rc<RefCell<Body>>> {
        self.body.borrow_mut().remove(id).map(|body| body.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Body>`.
    ///
    pub fn iter_body(&self) -> impl Iterator<Item = Rc<RefCell<Body>>> + '_ {
        let values: Vec<Rc<RefCell<Body>>> = self
            .body
            .borrow()
            .values()
            .map(|body| body.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal(&mut self, boolean_literal: Rc<RefCell<BooleanLiteral>>) {
        let read = boolean_literal.borrow();
        self.boolean_literal
            .borrow_mut()
            .insert(read.id, boolean_literal.clone());
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &Uuid) -> Option<Rc<RefCell<BooleanLiteral>>> {
        self.boolean_literal
            .borrow()
            .get(id)
            .map(|boolean_literal| boolean_literal.clone())
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub fn exorcise_boolean_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<BooleanLiteral>>> {
        self.boolean_literal
            .borrow_mut()
            .remove(id)
            .map(|boolean_literal| boolean_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Rc<RefCell<BooleanLiteral>>> + '_ {
        let values: Vec<Rc<RefCell<BooleanLiteral>>> = self
            .boolean_literal
            .borrow()
            .values()
            .map(|boolean_literal| boolean_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub fn inter_boolean_operator(&mut self, boolean_operator: Rc<RefCell<BooleanOperator>>) {
        let read = boolean_operator.borrow();
        self.boolean_operator
            .borrow_mut()
            .insert(read.id, boolean_operator.clone());
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub fn exhume_boolean_operator(&self, id: &Uuid) -> Option<Rc<RefCell<BooleanOperator>>> {
        self.boolean_operator
            .borrow()
            .get(id)
            .map(|boolean_operator| boolean_operator.clone())
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub fn exorcise_boolean_operator(&mut self, id: &Uuid) -> Option<Rc<RefCell<BooleanOperator>>> {
        self.boolean_operator
            .borrow_mut()
            .remove(id)
            .map(|boolean_operator| boolean_operator.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = Rc<RefCell<BooleanOperator>>> + '_ {
        let values: Vec<Rc<RefCell<BooleanOperator>>> = self
            .boolean_operator
            .borrow()
            .values()
            .map(|boolean_operator| boolean_operator.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Rc<RefCell<Call>>) {
        let read = call.borrow();
        self.call.borrow_mut().insert(read.id, call.clone());
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<Rc<RefCell<Call>>> {
        self.call.borrow().get(id).map(|call| call.clone())
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &Uuid) -> Option<Rc<RefCell<Call>>> {
        self.call.borrow_mut().remove(id).map(|call| call.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = Rc<RefCell<Call>>> + '_ {
        let values: Vec<Rc<RefCell<Call>>> = self
            .call
            .borrow()
            .values()
            .map(|call| call.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`CharLiteral`] into the store.
    ///
    pub fn inter_char_literal(&mut self, char_literal: Rc<RefCell<CharLiteral>>) {
        let read = char_literal.borrow();
        self.char_literal
            .borrow_mut()
            .insert(read.id, char_literal.clone());
    }

    /// Exhume (get) [`CharLiteral`] from the store.
    ///
    pub fn exhume_char_literal(&self, id: &Uuid) -> Option<Rc<RefCell<CharLiteral>>> {
        self.char_literal
            .borrow()
            .get(id)
            .map(|char_literal| char_literal.clone())
    }

    /// Exorcise (remove) [`CharLiteral`] from the store.
    ///
    pub fn exorcise_char_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<CharLiteral>>> {
        self.char_literal
            .borrow_mut()
            .remove(id)
            .map(|char_literal| char_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, CharLiteral>`.
    ///
    pub fn iter_char_literal(&self) -> impl Iterator<Item = Rc<RefCell<CharLiteral>>> + '_ {
        let values: Vec<Rc<RefCell<CharLiteral>>> = self
            .char_literal
            .borrow()
            .values()
            .map(|char_literal| char_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub fn inter_comparison(&mut self, comparison: Rc<RefCell<Comparison>>) {
        let read = comparison.borrow();
        self.comparison
            .borrow_mut()
            .insert(read.id, comparison.clone());
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub fn exhume_comparison(&self, id: &Uuid) -> Option<Rc<RefCell<Comparison>>> {
        self.comparison
            .borrow()
            .get(id)
            .map(|comparison| comparison.clone())
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub fn exorcise_comparison(&mut self, id: &Uuid) -> Option<Rc<RefCell<Comparison>>> {
        self.comparison
            .borrow_mut()
            .remove(id)
            .map(|comparison| comparison.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub fn iter_comparison(&self) -> impl Iterator<Item = Rc<RefCell<Comparison>>> + '_ {
        let values: Vec<Rc<RefCell<Comparison>>> = self
            .comparison
            .borrow()
            .values()
            .map(|comparison| comparison.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`DataStructure`] into the store.
    ///
    pub fn inter_data_structure(&mut self, data_structure: Rc<RefCell<DataStructure>>) {
        let read = data_structure.borrow();
        self.data_structure
            .borrow_mut()
            .insert(read.id, data_structure.clone());
    }

    /// Exhume (get) [`DataStructure`] from the store.
    ///
    pub fn exhume_data_structure(&self, id: &Uuid) -> Option<Rc<RefCell<DataStructure>>> {
        self.data_structure
            .borrow()
            .get(id)
            .map(|data_structure| data_structure.clone())
    }

    /// Exorcise (remove) [`DataStructure`] from the store.
    ///
    pub fn exorcise_data_structure(&mut self, id: &Uuid) -> Option<Rc<RefCell<DataStructure>>> {
        self.data_structure
            .borrow_mut()
            .remove(id)
            .map(|data_structure| data_structure.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DataStructure>`.
    ///
    pub fn iter_data_structure(&self) -> impl Iterator<Item = Rc<RefCell<DataStructure>>> + '_ {
        let values: Vec<Rc<RefCell<DataStructure>>> = self
            .data_structure
            .borrow()
            .values()
            .map(|data_structure| data_structure.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub fn inter_dwarf_source_file(&mut self, dwarf_source_file: Rc<RefCell<DwarfSourceFile>>) {
        let read = dwarf_source_file.borrow();
        self.dwarf_source_file
            .borrow_mut()
            .insert(read.id, dwarf_source_file.clone());
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &Uuid) -> Option<Rc<RefCell<DwarfSourceFile>>> {
        self.dwarf_source_file
            .borrow()
            .get(id)
            .map(|dwarf_source_file| dwarf_source_file.clone())
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub fn exorcise_dwarf_source_file(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<DwarfSourceFile>>> {
        self.dwarf_source_file
            .borrow_mut()
            .remove(id)
            .map(|dwarf_source_file| dwarf_source_file.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<DwarfSourceFile>>> + '_ {
        let values: Vec<Rc<RefCell<DwarfSourceFile>>> = self
            .dwarf_source_file
            .borrow()
            .values()
            .map(|dwarf_source_file| dwarf_source_file.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`EnumField`] into the store.
    ///
    pub fn inter_enum_field(&mut self, enum_field: Rc<RefCell<EnumField>>) {
        let read = enum_field.borrow();
        self.enum_field
            .borrow_mut()
            .insert(read.id, enum_field.clone());
    }

    /// Exhume (get) [`EnumField`] from the store.
    ///
    pub fn exhume_enum_field(&self, id: &Uuid) -> Option<Rc<RefCell<EnumField>>> {
        self.enum_field
            .borrow()
            .get(id)
            .map(|enum_field| enum_field.clone())
    }

    /// Exorcise (remove) [`EnumField`] from the store.
    ///
    pub fn exorcise_enum_field(&mut self, id: &Uuid) -> Option<Rc<RefCell<EnumField>>> {
        self.enum_field
            .borrow_mut()
            .remove(id)
            .map(|enum_field| enum_field.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumField>`.
    ///
    pub fn iter_enum_field(&self) -> impl Iterator<Item = Rc<RefCell<EnumField>>> + '_ {
        let values: Vec<Rc<RefCell<EnumField>>> = self
            .enum_field
            .borrow()
            .values()
            .map(|enum_field| enum_field.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`EnumGeneric`] into the store.
    ///
    pub fn inter_enum_generic(&mut self, enum_generic: Rc<RefCell<EnumGeneric>>) {
        let read = enum_generic.borrow();
        self.enum_generic
            .borrow_mut()
            .insert(read.id, enum_generic.clone());
    }

    /// Exhume (get) [`EnumGeneric`] from the store.
    ///
    pub fn exhume_enum_generic(&self, id: &Uuid) -> Option<Rc<RefCell<EnumGeneric>>> {
        self.enum_generic
            .borrow()
            .get(id)
            .map(|enum_generic| enum_generic.clone())
    }

    /// Exorcise (remove) [`EnumGeneric`] from the store.
    ///
    pub fn exorcise_enum_generic(&mut self, id: &Uuid) -> Option<Rc<RefCell<EnumGeneric>>> {
        self.enum_generic
            .borrow_mut()
            .remove(id)
            .map(|enum_generic| enum_generic.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGeneric>`.
    ///
    pub fn iter_enum_generic(&self) -> impl Iterator<Item = Rc<RefCell<EnumGeneric>>> + '_ {
        let values: Vec<Rc<RefCell<EnumGeneric>>> = self
            .enum_generic
            .borrow()
            .values()
            .map(|enum_generic| enum_generic.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`EnumGenericType`] into the store.
    ///
    pub fn inter_enum_generic_type(&mut self, enum_generic_type: Rc<RefCell<EnumGenericType>>) {
        let read = enum_generic_type.borrow();
        self.enum_generic_type
            .borrow_mut()
            .insert(read.id, enum_generic_type.clone());
    }

    /// Exhume (get) [`EnumGenericType`] from the store.
    ///
    pub fn exhume_enum_generic_type(&self, id: &Uuid) -> Option<Rc<RefCell<EnumGenericType>>> {
        self.enum_generic_type
            .borrow()
            .get(id)
            .map(|enum_generic_type| enum_generic_type.clone())
    }

    /// Exorcise (remove) [`EnumGenericType`] from the store.
    ///
    pub fn exorcise_enum_generic_type(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<EnumGenericType>>> {
        self.enum_generic_type
            .borrow_mut()
            .remove(id)
            .map(|enum_generic_type| enum_generic_type.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGenericType>`.
    ///
    pub fn iter_enum_generic_type(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<EnumGenericType>>> + '_ {
        let values: Vec<Rc<RefCell<EnumGenericType>>> = self
            .enum_generic_type
            .borrow()
            .values()
            .map(|enum_generic_type| enum_generic_type.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    pub fn inter_enumeration(&mut self, enumeration: Rc<RefCell<Enumeration>>) {
        let read = enumeration.borrow();
        self.enumeration_id_by_name
            .borrow_mut()
            .insert(read.name.clone(), read.id);
        self.enumeration
            .borrow_mut()
            .insert(read.id, enumeration.clone());
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    pub fn exhume_enumeration(&self, id: &Uuid) -> Option<Rc<RefCell<Enumeration>>> {
        self.enumeration
            .borrow()
            .get(id)
            .map(|enumeration| enumeration.clone())
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    pub fn exorcise_enumeration(&mut self, id: &Uuid) -> Option<Rc<RefCell<Enumeration>>> {
        self.enumeration
            .borrow_mut()
            .remove(id)
            .map(|enumeration| enumeration.clone())
    }

    /// Exhume [`Enumeration`] id from the store by name.
    ///
    pub fn exhume_enumeration_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.enumeration_id_by_name
            .borrow()
            .get(name)
            .map(|enumeration| *enumeration)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    pub fn iter_enumeration(&self) -> impl Iterator<Item = Rc<RefCell<Enumeration>>> + '_ {
        let values: Vec<Rc<RefCell<Enumeration>>> = self
            .enumeration
            .borrow()
            .values()
            .map(|enumeration| enumeration.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Rc<RefCell<Expression>>) {
        let read = expression.borrow();
        self.expression
            .borrow_mut()
            .insert(read.id, expression.clone());
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<Rc<RefCell<Expression>>> {
        self.expression
            .borrow()
            .get(id)
            .map(|expression| expression.clone())
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<Expression>>> {
        self.expression
            .borrow_mut()
            .remove(id)
            .map(|expression| expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = Rc<RefCell<Expression>>> + '_ {
        let values: Vec<Rc<RefCell<Expression>>> = self
            .expression
            .borrow()
            .values()
            .map(|expression| expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ExpressionBit`] into the store.
    ///
    pub fn inter_expression_bit(&mut self, expression_bit: Rc<RefCell<ExpressionBit>>) {
        let read = expression_bit.borrow();
        self.expression_bit
            .borrow_mut()
            .insert(read.id, expression_bit.clone());
    }

    /// Exhume (get) [`ExpressionBit`] from the store.
    ///
    pub fn exhume_expression_bit(&self, id: &Uuid) -> Option<Rc<RefCell<ExpressionBit>>> {
        self.expression_bit
            .borrow()
            .get(id)
            .map(|expression_bit| expression_bit.clone())
    }

    /// Exorcise (remove) [`ExpressionBit`] from the store.
    ///
    pub fn exorcise_expression_bit(&mut self, id: &Uuid) -> Option<Rc<RefCell<ExpressionBit>>> {
        self.expression_bit
            .borrow_mut()
            .remove(id)
            .map(|expression_bit| expression_bit.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionBit>`.
    ///
    pub fn iter_expression_bit(&self) -> impl Iterator<Item = Rc<RefCell<ExpressionBit>>> + '_ {
        let values: Vec<Rc<RefCell<ExpressionBit>>> = self
            .expression_bit
            .borrow()
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
        expression_statement: Rc<RefCell<ExpressionStatement>>,
    ) {
        let read = expression_statement.borrow();
        self.expression_statement
            .borrow_mut()
            .insert(read.id, expression_statement.clone());
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(
        &self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<ExpressionStatement>>> {
        self.expression_statement
            .borrow()
            .get(id)
            .map(|expression_statement| expression_statement.clone())
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub fn exorcise_expression_statement(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<ExpressionStatement>>> {
        self.expression_statement
            .borrow_mut()
            .remove(id)
            .map(|expression_statement| expression_statement.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<ExpressionStatement>>> + '_ {
        let values: Vec<Rc<RefCell<ExpressionStatement>>> = self
            .expression_statement
            .borrow()
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
        external_implementation: Rc<RefCell<ExternalImplementation>>,
    ) {
        let read = external_implementation.borrow();
        self.external_implementation
            .borrow_mut()
            .insert(read.id, external_implementation.clone());
    }

    /// Exhume (get) [`ExternalImplementation`] from the store.
    ///
    pub fn exhume_external_implementation(
        &self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<ExternalImplementation>>> {
        self.external_implementation
            .borrow()
            .get(id)
            .map(|external_implementation| external_implementation.clone())
    }

    /// Exorcise (remove) [`ExternalImplementation`] from the store.
    ///
    pub fn exorcise_external_implementation(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<ExternalImplementation>>> {
        self.external_implementation
            .borrow_mut()
            .remove(id)
            .map(|external_implementation| external_implementation.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExternalImplementation>`.
    ///
    pub fn iter_external_implementation(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<ExternalImplementation>>> + '_ {
        let values: Vec<Rc<RefCell<ExternalImplementation>>> = self
            .external_implementation
            .borrow()
            .values()
            .map(|external_implementation| external_implementation.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Rc<RefCell<Field>>) {
        let read = field.borrow();
        self.field_id_by_name
            .borrow_mut()
            .insert(read.name.clone(), read.id);
        self.field.borrow_mut().insert(read.id, field.clone());
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<Rc<RefCell<Field>>> {
        self.field.borrow().get(id).map(|field| field.clone())
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &Uuid) -> Option<Rc<RefCell<Field>>> {
        self.field
            .borrow_mut()
            .remove(id)
            .map(|field| field.clone())
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.field_id_by_name.borrow().get(name).map(|field| *field)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = Rc<RefCell<Field>>> + '_ {
        let values: Vec<Rc<RefCell<Field>>> = self
            .field
            .borrow()
            .values()
            .map(|field| field.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access(&mut self, field_access: Rc<RefCell<FieldAccess>>) {
        let read = field_access.borrow();
        self.field_access
            .borrow_mut()
            .insert(read.id, field_access.clone());
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &Uuid) -> Option<Rc<RefCell<FieldAccess>>> {
        self.field_access
            .borrow()
            .get(id)
            .map(|field_access| field_access.clone())
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub fn exorcise_field_access(&mut self, id: &Uuid) -> Option<Rc<RefCell<FieldAccess>>> {
        self.field_access
            .borrow_mut()
            .remove(id)
            .map(|field_access| field_access.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = Rc<RefCell<FieldAccess>>> + '_ {
        let values: Vec<Rc<RefCell<FieldAccess>>> = self
            .field_access
            .borrow()
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
        field_access_target: Rc<RefCell<FieldAccessTarget>>,
    ) {
        let read = field_access_target.borrow();
        self.field_access_target
            .borrow_mut()
            .insert(read.id, field_access_target.clone());
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub fn exhume_field_access_target(&self, id: &Uuid) -> Option<Rc<RefCell<FieldAccessTarget>>> {
        self.field_access_target
            .borrow()
            .get(id)
            .map(|field_access_target| field_access_target.clone())
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub fn exorcise_field_access_target(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<FieldAccessTarget>>> {
        self.field_access_target
            .borrow_mut()
            .remove(id)
            .map(|field_access_target| field_access_target.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<FieldAccessTarget>>> + '_ {
        let values: Vec<Rc<RefCell<FieldAccessTarget>>> = self
            .field_access_target
            .borrow()
            .values()
            .map(|field_access_target| field_access_target.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression(&mut self, field_expression: Rc<RefCell<FieldExpression>>) {
        let read = field_expression.borrow();
        self.field_expression
            .borrow_mut()
            .insert(read.id, field_expression.clone());
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &Uuid) -> Option<Rc<RefCell<FieldExpression>>> {
        self.field_expression
            .borrow()
            .get(id)
            .map(|field_expression| field_expression.clone())
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub fn exorcise_field_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<FieldExpression>>> {
        self.field_expression
            .borrow_mut()
            .remove(id)
            .map(|field_expression| field_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Rc<RefCell<FieldExpression>>> + '_ {
        let values: Vec<Rc<RefCell<FieldExpression>>> = self
            .field_expression
            .borrow()
            .values()
            .map(|field_expression| field_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal(&mut self, float_literal: Rc<RefCell<FloatLiteral>>) {
        let read = float_literal.borrow();
        self.float_literal
            .borrow_mut()
            .insert(read.id, float_literal.clone());
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &Uuid) -> Option<Rc<RefCell<FloatLiteral>>> {
        self.float_literal
            .borrow()
            .get(id)
            .map(|float_literal| float_literal.clone())
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub fn exorcise_float_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<FloatLiteral>>> {
        self.float_literal
            .borrow_mut()
            .remove(id)
            .map(|float_literal| float_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Rc<RefCell<FloatLiteral>>> + '_ {
        let values: Vec<Rc<RefCell<FloatLiteral>>> = self
            .float_literal
            .borrow()
            .values()
            .map(|float_literal| float_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub fn inter_for_loop(&mut self, for_loop: Rc<RefCell<ForLoop>>) {
        let read = for_loop.borrow();
        self.for_loop.borrow_mut().insert(read.id, for_loop.clone());
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub fn exhume_for_loop(&self, id: &Uuid) -> Option<Rc<RefCell<ForLoop>>> {
        self.for_loop
            .borrow()
            .get(id)
            .map(|for_loop| for_loop.clone())
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub fn exorcise_for_loop(&mut self, id: &Uuid) -> Option<Rc<RefCell<ForLoop>>> {
        self.for_loop
            .borrow_mut()
            .remove(id)
            .map(|for_loop| for_loop.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub fn iter_for_loop(&self) -> impl Iterator<Item = Rc<RefCell<ForLoop>>> + '_ {
        let values: Vec<Rc<RefCell<ForLoop>>> = self
            .for_loop
            .borrow()
            .values()
            .map(|for_loop| for_loop.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FormatBit`] into the store.
    ///
    pub fn inter_format_bit(&mut self, format_bit: Rc<RefCell<FormatBit>>) {
        let read = format_bit.borrow();
        self.format_bit
            .borrow_mut()
            .insert(read.id, format_bit.clone());
    }

    /// Exhume (get) [`FormatBit`] from the store.
    ///
    pub fn exhume_format_bit(&self, id: &Uuid) -> Option<Rc<RefCell<FormatBit>>> {
        self.format_bit
            .borrow()
            .get(id)
            .map(|format_bit| format_bit.clone())
    }

    /// Exorcise (remove) [`FormatBit`] from the store.
    ///
    pub fn exorcise_format_bit(&mut self, id: &Uuid) -> Option<Rc<RefCell<FormatBit>>> {
        self.format_bit
            .borrow_mut()
            .remove(id)
            .map(|format_bit| format_bit.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FormatBit>`.
    ///
    pub fn iter_format_bit(&self) -> impl Iterator<Item = Rc<RefCell<FormatBit>>> + '_ {
        let values: Vec<Rc<RefCell<FormatBit>>> = self
            .format_bit
            .borrow()
            .values()
            .map(|format_bit| format_bit.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FormatString`] into the store.
    ///
    pub fn inter_format_string(&mut self, format_string: Rc<RefCell<FormatString>>) {
        let read = format_string.borrow();
        self.format_string
            .borrow_mut()
            .insert(read.id, format_string.clone());
    }

    /// Exhume (get) [`FormatString`] from the store.
    ///
    pub fn exhume_format_string(&self, id: &Uuid) -> Option<Rc<RefCell<FormatString>>> {
        self.format_string
            .borrow()
            .get(id)
            .map(|format_string| format_string.clone())
    }

    /// Exorcise (remove) [`FormatString`] from the store.
    ///
    pub fn exorcise_format_string(&mut self, id: &Uuid) -> Option<Rc<RefCell<FormatString>>> {
        self.format_string
            .borrow_mut()
            .remove(id)
            .map(|format_string| format_string.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FormatString>`.
    ///
    pub fn iter_format_string(&self) -> impl Iterator<Item = Rc<RefCell<FormatString>>> + '_ {
        let values: Vec<Rc<RefCell<FormatString>>> = self
            .format_string
            .borrow()
            .values()
            .map(|format_string| format_string.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FuncGeneric`] into the store.
    ///
    pub fn inter_func_generic(&mut self, func_generic: Rc<RefCell<FuncGeneric>>) {
        let read = func_generic.borrow();
        self.func_generic
            .borrow_mut()
            .insert(read.id, func_generic.clone());
    }

    /// Exhume (get) [`FuncGeneric`] from the store.
    ///
    pub fn exhume_func_generic(&self, id: &Uuid) -> Option<Rc<RefCell<FuncGeneric>>> {
        self.func_generic
            .borrow()
            .get(id)
            .map(|func_generic| func_generic.clone())
    }

    /// Exorcise (remove) [`FuncGeneric`] from the store.
    ///
    pub fn exorcise_func_generic(&mut self, id: &Uuid) -> Option<Rc<RefCell<FuncGeneric>>> {
        self.func_generic
            .borrow_mut()
            .remove(id)
            .map(|func_generic| func_generic.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FuncGeneric>`.
    ///
    pub fn iter_func_generic(&self) -> impl Iterator<Item = Rc<RefCell<FuncGeneric>>> + '_ {
        let values: Vec<Rc<RefCell<FuncGeneric>>> = self
            .func_generic
            .borrow()
            .values()
            .map(|func_generic| func_generic.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Rc<RefCell<Function>>) {
        let read = function.borrow();
        self.function_id_by_name
            .borrow_mut()
            .insert(read.name.clone(), read.id);
        self.function.borrow_mut().insert(read.id, function.clone());
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<Rc<RefCell<Function>>> {
        self.function
            .borrow()
            .get(id)
            .map(|function| function.clone())
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &Uuid) -> Option<Rc<RefCell<Function>>> {
        self.function
            .borrow_mut()
            .remove(id)
            .map(|function| function.clone())
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.function_id_by_name
            .borrow()
            .get(name)
            .map(|function| *function)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = Rc<RefCell<Function>>> + '_ {
        let values: Vec<Rc<RefCell<Function>>> = self
            .function
            .borrow()
            .values()
            .map(|function| function.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`FunctionCall`] into the store.
    ///
    pub fn inter_function_call(&mut self, function_call: Rc<RefCell<FunctionCall>>) {
        let read = function_call.borrow();
        self.function_call
            .borrow_mut()
            .insert(read.id, function_call.clone());
    }

    /// Exhume (get) [`FunctionCall`] from the store.
    ///
    pub fn exhume_function_call(&self, id: &Uuid) -> Option<Rc<RefCell<FunctionCall>>> {
        self.function_call
            .borrow()
            .get(id)
            .map(|function_call| function_call.clone())
    }

    /// Exorcise (remove) [`FunctionCall`] from the store.
    ///
    pub fn exorcise_function_call(&mut self, id: &Uuid) -> Option<Rc<RefCell<FunctionCall>>> {
        self.function_call
            .borrow_mut()
            .remove(id)
            .map(|function_call| function_call.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FunctionCall>`.
    ///
    pub fn iter_function_call(&self) -> impl Iterator<Item = Rc<RefCell<FunctionCall>>> + '_ {
        let values: Vec<Rc<RefCell<FunctionCall>>> = self
            .function_call
            .borrow()
            .values()
            .map(|function_call| function_call.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XFuture`] into the store.
    ///
    pub fn inter_x_future(&mut self, x_future: Rc<RefCell<XFuture>>) {
        let read = x_future.borrow();
        self.x_future.borrow_mut().insert(read.id, x_future.clone());
    }

    /// Exhume (get) [`XFuture`] from the store.
    ///
    pub fn exhume_x_future(&self, id: &Uuid) -> Option<Rc<RefCell<XFuture>>> {
        self.x_future
            .borrow()
            .get(id)
            .map(|x_future| x_future.clone())
    }

    /// Exorcise (remove) [`XFuture`] from the store.
    ///
    pub fn exorcise_x_future(&mut self, id: &Uuid) -> Option<Rc<RefCell<XFuture>>> {
        self.x_future
            .borrow_mut()
            .remove(id)
            .map(|x_future| x_future.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XFuture>`.
    ///
    pub fn iter_x_future(&self) -> impl Iterator<Item = Rc<RefCell<XFuture>>> + '_ {
        let values: Vec<Rc<RefCell<XFuture>>> = self
            .x_future
            .borrow()
            .values()
            .map(|x_future| x_future.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub fn inter_grouped(&mut self, grouped: Rc<RefCell<Grouped>>) {
        let read = grouped.borrow();
        self.grouped.borrow_mut().insert(read.id, grouped.clone());
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub fn exhume_grouped(&self, id: &Uuid) -> Option<Rc<RefCell<Grouped>>> {
        self.grouped.borrow().get(id).map(|grouped| grouped.clone())
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub fn exorcise_grouped(&mut self, id: &Uuid) -> Option<Rc<RefCell<Grouped>>> {
        self.grouped
            .borrow_mut()
            .remove(id)
            .map(|grouped| grouped.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub fn iter_grouped(&self) -> impl Iterator<Item = Rc<RefCell<Grouped>>> + '_ {
        let values: Vec<Rc<RefCell<Grouped>>> = self
            .grouped
            .borrow()
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
        halt_and_catch_fire: Rc<RefCell<HaltAndCatchFire>>,
    ) {
        let read = halt_and_catch_fire.borrow();
        self.halt_and_catch_fire
            .borrow_mut()
            .insert(read.id, halt_and_catch_fire.clone());
    }

    /// Exhume (get) [`HaltAndCatchFire`] from the store.
    ///
    pub fn exhume_halt_and_catch_fire(&self, id: &Uuid) -> Option<Rc<RefCell<HaltAndCatchFire>>> {
        self.halt_and_catch_fire
            .borrow()
            .get(id)
            .map(|halt_and_catch_fire| halt_and_catch_fire.clone())
    }

    /// Exorcise (remove) [`HaltAndCatchFire`] from the store.
    ///
    pub fn exorcise_halt_and_catch_fire(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<HaltAndCatchFire>>> {
        self.halt_and_catch_fire
            .borrow_mut()
            .remove(id)
            .map(|halt_and_catch_fire| halt_and_catch_fire.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, HaltAndCatchFire>`.
    ///
    pub fn iter_halt_and_catch_fire(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<HaltAndCatchFire>>> + '_ {
        let values: Vec<Rc<RefCell<HaltAndCatchFire>>> = self
            .halt_and_catch_fire
            .borrow()
            .values()
            .map(|halt_and_catch_fire| halt_and_catch_fire.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub fn inter_x_if(&mut self, x_if: Rc<RefCell<XIf>>) {
        let read = x_if.borrow();
        self.x_if.borrow_mut().insert(read.id, x_if.clone());
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub fn exhume_x_if(&self, id: &Uuid) -> Option<Rc<RefCell<XIf>>> {
        self.x_if.borrow().get(id).map(|x_if| x_if.clone())
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub fn exorcise_x_if(&mut self, id: &Uuid) -> Option<Rc<RefCell<XIf>>> {
        self.x_if.borrow_mut().remove(id).map(|x_if| x_if.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub fn iter_x_if(&self) -> impl Iterator<Item = Rc<RefCell<XIf>>> + '_ {
        let values: Vec<Rc<RefCell<XIf>>> = self
            .x_if
            .borrow()
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
        implementation_block: Rc<RefCell<ImplementationBlock>>,
    ) {
        let read = implementation_block.borrow();
        self.implementation_block
            .borrow_mut()
            .insert(read.id, implementation_block.clone());
    }

    /// Exhume (get) [`ImplementationBlock`] from the store.
    ///
    pub fn exhume_implementation_block(
        &self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<ImplementationBlock>>> {
        self.implementation_block
            .borrow()
            .get(id)
            .map(|implementation_block| implementation_block.clone())
    }

    /// Exorcise (remove) [`ImplementationBlock`] from the store.
    ///
    pub fn exorcise_implementation_block(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<ImplementationBlock>>> {
        self.implementation_block
            .borrow_mut()
            .remove(id)
            .map(|implementation_block| implementation_block.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ImplementationBlock>`.
    ///
    pub fn iter_implementation_block(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<ImplementationBlock>>> + '_ {
        let values: Vec<Rc<RefCell<ImplementationBlock>>> = self
            .implementation_block
            .borrow()
            .values()
            .map(|implementation_block| implementation_block.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import(&mut self, import: Rc<RefCell<Import>>) {
        let read = import.borrow();
        self.import.borrow_mut().insert(read.id, import.clone());
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &Uuid) -> Option<Rc<RefCell<Import>>> {
        self.import.borrow().get(id).map(|import| import.clone())
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub fn exorcise_import(&mut self, id: &Uuid) -> Option<Rc<RefCell<Import>>> {
        self.import
            .borrow_mut()
            .remove(id)
            .map(|import| import.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = Rc<RefCell<Import>>> + '_ {
        let values: Vec<Rc<RefCell<Import>>> = self
            .import
            .borrow()
            .values()
            .map(|import| import.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub fn inter_index(&mut self, index: Rc<RefCell<Index>>) {
        let read = index.borrow();
        self.index.borrow_mut().insert(read.id, index.clone());
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub fn exhume_index(&self, id: &Uuid) -> Option<Rc<RefCell<Index>>> {
        self.index.borrow().get(id).map(|index| index.clone())
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub fn exorcise_index(&mut self, id: &Uuid) -> Option<Rc<RefCell<Index>>> {
        self.index
            .borrow_mut()
            .remove(id)
            .map(|index| index.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub fn iter_index(&self) -> impl Iterator<Item = Rc<RefCell<Index>>> + '_ {
        let values: Vec<Rc<RefCell<Index>>> = self
            .index
            .borrow()
            .values()
            .map(|index| index.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal(&mut self, integer_literal: Rc<RefCell<IntegerLiteral>>) {
        let read = integer_literal.borrow();
        self.integer_literal
            .borrow_mut()
            .insert(read.id, integer_literal.clone());
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &Uuid) -> Option<Rc<RefCell<IntegerLiteral>>> {
        self.integer_literal
            .borrow()
            .get(id)
            .map(|integer_literal| integer_literal.clone())
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub fn exorcise_integer_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<IntegerLiteral>>> {
        self.integer_literal
            .borrow_mut()
            .remove(id)
            .map(|integer_literal| integer_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Rc<RefCell<IntegerLiteral>>> + '_ {
        let values: Vec<Rc<RefCell<IntegerLiteral>>> = self
            .integer_literal
            .borrow()
            .values()
            .map(|integer_literal| integer_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Rc<RefCell<Item>>) {
        let read = item.borrow();
        self.item.borrow_mut().insert(read.id, item.clone());
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<Rc<RefCell<Item>>> {
        self.item.borrow().get(id).map(|item| item.clone())
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &Uuid) -> Option<Rc<RefCell<Item>>> {
        self.item.borrow_mut().remove(id).map(|item| item.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = Rc<RefCell<Item>>> + '_ {
        let values: Vec<Rc<RefCell<Item>>> = self
            .item
            .borrow()
            .values()
            .map(|item| item.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Lambda`] into the store.
    ///
    pub fn inter_lambda(&mut self, lambda: Rc<RefCell<Lambda>>) {
        let read = lambda.borrow();
        self.lambda.borrow_mut().insert(read.id, lambda.clone());
    }

    /// Exhume (get) [`Lambda`] from the store.
    ///
    pub fn exhume_lambda(&self, id: &Uuid) -> Option<Rc<RefCell<Lambda>>> {
        self.lambda.borrow().get(id).map(|lambda| lambda.clone())
    }

    /// Exorcise (remove) [`Lambda`] from the store.
    ///
    pub fn exorcise_lambda(&mut self, id: &Uuid) -> Option<Rc<RefCell<Lambda>>> {
        self.lambda
            .borrow_mut()
            .remove(id)
            .map(|lambda| lambda.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Lambda>`.
    ///
    pub fn iter_lambda(&self) -> impl Iterator<Item = Rc<RefCell<Lambda>>> + '_ {
        let values: Vec<Rc<RefCell<Lambda>>> = self
            .lambda
            .borrow()
            .values()
            .map(|lambda| lambda.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`LambdaParameter`] into the store.
    ///
    pub fn inter_lambda_parameter(&mut self, lambda_parameter: Rc<RefCell<LambdaParameter>>) {
        let read = lambda_parameter.borrow();
        self.lambda_parameter
            .borrow_mut()
            .insert(read.id, lambda_parameter.clone());
    }

    /// Exhume (get) [`LambdaParameter`] from the store.
    ///
    pub fn exhume_lambda_parameter(&self, id: &Uuid) -> Option<Rc<RefCell<LambdaParameter>>> {
        self.lambda_parameter
            .borrow()
            .get(id)
            .map(|lambda_parameter| lambda_parameter.clone())
    }

    /// Exorcise (remove) [`LambdaParameter`] from the store.
    ///
    pub fn exorcise_lambda_parameter(&mut self, id: &Uuid) -> Option<Rc<RefCell<LambdaParameter>>> {
        self.lambda_parameter
            .borrow_mut()
            .remove(id)
            .map(|lambda_parameter| lambda_parameter.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LambdaParameter>`.
    ///
    pub fn iter_lambda_parameter(&self) -> impl Iterator<Item = Rc<RefCell<LambdaParameter>>> + '_ {
        let values: Vec<Rc<RefCell<LambdaParameter>>> = self
            .lambda_parameter
            .borrow()
            .values()
            .map(|lambda_parameter| lambda_parameter.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement(&mut self, let_statement: Rc<RefCell<LetStatement>>) {
        let read = let_statement.borrow();
        self.let_statement
            .borrow_mut()
            .insert(read.id, let_statement.clone());
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &Uuid) -> Option<Rc<RefCell<LetStatement>>> {
        self.let_statement
            .borrow()
            .get(id)
            .map(|let_statement| let_statement.clone())
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub fn exorcise_let_statement(&mut self, id: &Uuid) -> Option<Rc<RefCell<LetStatement>>> {
        self.let_statement
            .borrow_mut()
            .remove(id)
            .map(|let_statement| let_statement.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Rc<RefCell<LetStatement>>> + '_ {
        let values: Vec<Rc<RefCell<LetStatement>>> = self
            .let_statement
            .borrow()
            .values()
            .map(|let_statement| let_statement.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub fn inter_list(&mut self, list: Rc<RefCell<List>>) {
        let read = list.borrow();
        self.list.borrow_mut().insert(read.id, list.clone());
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &Uuid) -> Option<Rc<RefCell<List>>> {
        self.list.borrow().get(id).map(|list| list.clone())
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub fn exorcise_list(&mut self, id: &Uuid) -> Option<Rc<RefCell<List>>> {
        self.list.borrow_mut().remove(id).map(|list| list.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = Rc<RefCell<List>>> + '_ {
        let values: Vec<Rc<RefCell<List>>> = self
            .list
            .borrow()
            .values()
            .map(|list| list.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub fn inter_list_element(&mut self, list_element: Rc<RefCell<ListElement>>) {
        let read = list_element.borrow();
        self.list_element
            .borrow_mut()
            .insert(read.id, list_element.clone());
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub fn exhume_list_element(&self, id: &Uuid) -> Option<Rc<RefCell<ListElement>>> {
        self.list_element
            .borrow()
            .get(id)
            .map(|list_element| list_element.clone())
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub fn exorcise_list_element(&mut self, id: &Uuid) -> Option<Rc<RefCell<ListElement>>> {
        self.list_element
            .borrow_mut()
            .remove(id)
            .map(|list_element| list_element.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub fn iter_list_element(&self) -> impl Iterator<Item = Rc<RefCell<ListElement>>> + '_ {
        let values: Vec<Rc<RefCell<ListElement>>> = self
            .list_element
            .borrow()
            .values()
            .map(|list_element| list_element.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub fn inter_list_expression(&mut self, list_expression: Rc<RefCell<ListExpression>>) {
        let read = list_expression.borrow();
        self.list_expression
            .borrow_mut()
            .insert(read.id, list_expression.clone());
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub fn exhume_list_expression(&self, id: &Uuid) -> Option<Rc<RefCell<ListExpression>>> {
        self.list_expression
            .borrow()
            .get(id)
            .map(|list_expression| list_expression.clone())
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub fn exorcise_list_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<ListExpression>>> {
        self.list_expression
            .borrow_mut()
            .remove(id)
            .map(|list_expression| list_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub fn iter_list_expression(&self) -> impl Iterator<Item = Rc<RefCell<ListExpression>>> + '_ {
        let values: Vec<Rc<RefCell<ListExpression>>> = self
            .list_expression
            .borrow()
            .values()
            .map(|list_expression| list_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub fn inter_literal(&mut self, literal: Rc<RefCell<Literal>>) {
        let read = literal.borrow();
        self.literal.borrow_mut().insert(read.id, literal.clone());
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &Uuid) -> Option<Rc<RefCell<Literal>>> {
        self.literal.borrow().get(id).map(|literal| literal.clone())
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub fn exorcise_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<Literal>>> {
        self.literal
            .borrow_mut()
            .remove(id)
            .map(|literal| literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = Rc<RefCell<Literal>>> + '_ {
        let values: Vec<Rc<RefCell<Literal>>> = self
            .literal
            .borrow()
            .values()
            .map(|literal| literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable(&mut self, local_variable: Rc<RefCell<LocalVariable>>) {
        let read = local_variable.borrow();
        self.local_variable
            .borrow_mut()
            .insert(read.id, local_variable.clone());
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &Uuid) -> Option<Rc<RefCell<LocalVariable>>> {
        self.local_variable
            .borrow()
            .get(id)
            .map(|local_variable| local_variable.clone())
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub fn exorcise_local_variable(&mut self, id: &Uuid) -> Option<Rc<RefCell<LocalVariable>>> {
        self.local_variable
            .borrow_mut()
            .remove(id)
            .map(|local_variable| local_variable.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Rc<RefCell<LocalVariable>>> + '_ {
        let values: Vec<Rc<RefCell<LocalVariable>>> = self
            .local_variable
            .borrow()
            .values()
            .map(|local_variable| local_variable.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub fn inter_x_macro(&mut self, x_macro: Rc<RefCell<XMacro>>) {
        let read = x_macro.borrow();
        self.x_macro.borrow_mut().insert(read.id, x_macro.clone());
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub fn exhume_x_macro(&self, id: &Uuid) -> Option<Rc<RefCell<XMacro>>> {
        self.x_macro.borrow().get(id).map(|x_macro| x_macro.clone())
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub fn exorcise_x_macro(&mut self, id: &Uuid) -> Option<Rc<RefCell<XMacro>>> {
        self.x_macro
            .borrow_mut()
            .remove(id)
            .map(|x_macro| x_macro.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub fn iter_x_macro(&self) -> impl Iterator<Item = Rc<RefCell<XMacro>>> + '_ {
        let values: Vec<Rc<RefCell<XMacro>>> = self
            .x_macro
            .borrow()
            .values()
            .map(|x_macro| x_macro.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Map`] into the store.
    ///
    pub fn inter_map(&mut self, map: Rc<RefCell<Map>>) {
        let read = map.borrow();
        self.map.borrow_mut().insert(read.id, map.clone());
    }

    /// Exhume (get) [`Map`] from the store.
    ///
    pub fn exhume_map(&self, id: &Uuid) -> Option<Rc<RefCell<Map>>> {
        self.map.borrow().get(id).map(|map| map.clone())
    }

    /// Exorcise (remove) [`Map`] from the store.
    ///
    pub fn exorcise_map(&mut self, id: &Uuid) -> Option<Rc<RefCell<Map>>> {
        self.map.borrow_mut().remove(id).map(|map| map.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Map>`.
    ///
    pub fn iter_map(&self) -> impl Iterator<Item = Rc<RefCell<Map>>> + '_ {
        let values: Vec<Rc<RefCell<Map>>> =
            self.map.borrow().values().map(|map| map.clone()).collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`MapElement`] into the store.
    ///
    pub fn inter_map_element(&mut self, map_element: Rc<RefCell<MapElement>>) {
        let read = map_element.borrow();
        self.map_element
            .borrow_mut()
            .insert(read.id, map_element.clone());
    }

    /// Exhume (get) [`MapElement`] from the store.
    ///
    pub fn exhume_map_element(&self, id: &Uuid) -> Option<Rc<RefCell<MapElement>>> {
        self.map_element
            .borrow()
            .get(id)
            .map(|map_element| map_element.clone())
    }

    /// Exorcise (remove) [`MapElement`] from the store.
    ///
    pub fn exorcise_map_element(&mut self, id: &Uuid) -> Option<Rc<RefCell<MapElement>>> {
        self.map_element
            .borrow_mut()
            .remove(id)
            .map(|map_element| map_element.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MapElement>`.
    ///
    pub fn iter_map_element(&self) -> impl Iterator<Item = Rc<RefCell<MapElement>>> + '_ {
        let values: Vec<Rc<RefCell<MapElement>>> = self
            .map_element
            .borrow()
            .values()
            .map(|map_element| map_element.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`MapExpression`] into the store.
    ///
    pub fn inter_map_expression(&mut self, map_expression: Rc<RefCell<MapExpression>>) {
        let read = map_expression.borrow();
        self.map_expression
            .borrow_mut()
            .insert(read.id, map_expression.clone());
    }

    /// Exhume (get) [`MapExpression`] from the store.
    ///
    pub fn exhume_map_expression(&self, id: &Uuid) -> Option<Rc<RefCell<MapExpression>>> {
        self.map_expression
            .borrow()
            .get(id)
            .map(|map_expression| map_expression.clone())
    }

    /// Exorcise (remove) [`MapExpression`] from the store.
    ///
    pub fn exorcise_map_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<MapExpression>>> {
        self.map_expression
            .borrow_mut()
            .remove(id)
            .map(|map_expression| map_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MapExpression>`.
    ///
    pub fn iter_map_expression(&self) -> impl Iterator<Item = Rc<RefCell<MapExpression>>> + '_ {
        let values: Vec<Rc<RefCell<MapExpression>>> = self
            .map_expression
            .borrow()
            .values()
            .map(|map_expression| map_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XMatch`] into the store.
    ///
    pub fn inter_x_match(&mut self, x_match: Rc<RefCell<XMatch>>) {
        let read = x_match.borrow();
        self.x_match.borrow_mut().insert(read.id, x_match.clone());
    }

    /// Exhume (get) [`XMatch`] from the store.
    ///
    pub fn exhume_x_match(&self, id: &Uuid) -> Option<Rc<RefCell<XMatch>>> {
        self.x_match.borrow().get(id).map(|x_match| x_match.clone())
    }

    /// Exorcise (remove) [`XMatch`] from the store.
    ///
    pub fn exorcise_x_match(&mut self, id: &Uuid) -> Option<Rc<RefCell<XMatch>>> {
        self.x_match
            .borrow_mut()
            .remove(id)
            .map(|x_match| x_match.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMatch>`.
    ///
    pub fn iter_x_match(&self) -> impl Iterator<Item = Rc<RefCell<XMatch>>> + '_ {
        let values: Vec<Rc<RefCell<XMatch>>> = self
            .x_match
            .borrow()
            .values()
            .map(|x_match| x_match.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub fn inter_method_call(&mut self, method_call: Rc<RefCell<MethodCall>>) {
        let read = method_call.borrow();
        self.method_call
            .borrow_mut()
            .insert(read.id, method_call.clone());
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &Uuid) -> Option<Rc<RefCell<MethodCall>>> {
        self.method_call
            .borrow()
            .get(id)
            .map(|method_call| method_call.clone())
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub fn exorcise_method_call(&mut self, id: &Uuid) -> Option<Rc<RefCell<MethodCall>>> {
        self.method_call
            .borrow_mut()
            .remove(id)
            .map(|method_call| method_call.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = Rc<RefCell<MethodCall>>> + '_ {
        let values: Vec<Rc<RefCell<MethodCall>>> = self
            .method_call
            .borrow()
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
        named_field_expression: Rc<RefCell<NamedFieldExpression>>,
    ) {
        let read = named_field_expression.borrow();
        self.named_field_expression
            .borrow_mut()
            .insert(read.id, named_field_expression.clone());
    }

    /// Exhume (get) [`NamedFieldExpression`] from the store.
    ///
    pub fn exhume_named_field_expression(
        &self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<NamedFieldExpression>>> {
        self.named_field_expression
            .borrow()
            .get(id)
            .map(|named_field_expression| named_field_expression.clone())
    }

    /// Exorcise (remove) [`NamedFieldExpression`] from the store.
    ///
    pub fn exorcise_named_field_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<NamedFieldExpression>>> {
        self.named_field_expression
            .borrow_mut()
            .remove(id)
            .map(|named_field_expression| named_field_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NamedFieldExpression>`.
    ///
    pub fn iter_named_field_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<NamedFieldExpression>>> + '_ {
        let values: Vec<Rc<RefCell<NamedFieldExpression>>> = self
            .named_field_expression
            .borrow()
            .values()
            .map(|named_field_expression| named_field_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store(&mut self, z_object_store: Rc<RefCell<ZObjectStore>>) {
        let read = z_object_store.borrow();
        self.z_object_store_id_by_name
            .borrow_mut()
            .insert(read.name.clone(), read.id);
        self.z_object_store
            .borrow_mut()
            .insert(read.id, z_object_store.clone());
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &Uuid) -> Option<Rc<RefCell<ZObjectStore>>> {
        self.z_object_store
            .borrow()
            .get(id)
            .map(|z_object_store| z_object_store.clone())
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub fn exorcise_z_object_store(&mut self, id: &Uuid) -> Option<Rc<RefCell<ZObjectStore>>> {
        self.z_object_store
            .borrow_mut()
            .remove(id)
            .map(|z_object_store| z_object_store.clone())
    }

    /// Exhume [`ZObjectStore`] id from the store by name.
    ///
    pub fn exhume_z_object_store_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.z_object_store_id_by_name
            .borrow()
            .get(name)
            .map(|z_object_store| *z_object_store)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Rc<RefCell<ZObjectStore>>> + '_ {
        let values: Vec<Rc<RefCell<ZObjectStore>>> = self
            .z_object_store
            .borrow()
            .values()
            .map(|z_object_store| z_object_store.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ObjectWrapper`] into the store.
    ///
    pub fn inter_object_wrapper(&mut self, object_wrapper: Rc<RefCell<ObjectWrapper>>) {
        let read = object_wrapper.borrow();
        self.object_wrapper
            .borrow_mut()
            .insert(read.id, object_wrapper.clone());
    }

    /// Exhume (get) [`ObjectWrapper`] from the store.
    ///
    pub fn exhume_object_wrapper(&self, id: &Uuid) -> Option<Rc<RefCell<ObjectWrapper>>> {
        self.object_wrapper
            .borrow()
            .get(id)
            .map(|object_wrapper| object_wrapper.clone())
    }

    /// Exorcise (remove) [`ObjectWrapper`] from the store.
    ///
    pub fn exorcise_object_wrapper(&mut self, id: &Uuid) -> Option<Rc<RefCell<ObjectWrapper>>> {
        self.object_wrapper
            .borrow_mut()
            .remove(id)
            .map(|object_wrapper| object_wrapper.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectWrapper>`.
    ///
    pub fn iter_object_wrapper(&self) -> impl Iterator<Item = Rc<RefCell<ObjectWrapper>>> + '_ {
        let values: Vec<Rc<RefCell<ObjectWrapper>>> = self
            .object_wrapper
            .borrow()
            .values()
            .map(|object_wrapper| object_wrapper.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator(&mut self, operator: Rc<RefCell<Operator>>) {
        let read = operator.borrow();
        self.operator.borrow_mut().insert(read.id, operator.clone());
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub fn exhume_operator(&self, id: &Uuid) -> Option<Rc<RefCell<Operator>>> {
        self.operator
            .borrow()
            .get(id)
            .map(|operator| operator.clone())
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub fn exorcise_operator(&mut self, id: &Uuid) -> Option<Rc<RefCell<Operator>>> {
        self.operator
            .borrow_mut()
            .remove(id)
            .map(|operator| operator.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub fn iter_operator(&self) -> impl Iterator<Item = Rc<RefCell<Operator>>> + '_ {
        let values: Vec<Rc<RefCell<Operator>>> = self
            .operator
            .borrow()
            .values()
            .map(|operator| operator.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Rc<RefCell<Parameter>>) {
        let read = parameter.borrow();
        self.parameter
            .borrow_mut()
            .insert(read.id, parameter.clone());
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<Rc<RefCell<Parameter>>> {
        self.parameter
            .borrow()
            .get(id)
            .map(|parameter| parameter.clone())
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Rc<RefCell<Parameter>>> {
        self.parameter
            .borrow_mut()
            .remove(id)
            .map(|parameter| parameter.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Rc<RefCell<Parameter>>> + '_ {
        let values: Vec<Rc<RefCell<Parameter>>> = self
            .parameter
            .borrow()
            .values()
            .map(|parameter| parameter.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XPath`] into the store.
    ///
    pub fn inter_x_path(&mut self, x_path: Rc<RefCell<XPath>>) {
        let read = x_path.borrow();
        self.x_path.borrow_mut().insert(read.id, x_path.clone());
    }

    /// Exhume (get) [`XPath`] from the store.
    ///
    pub fn exhume_x_path(&self, id: &Uuid) -> Option<Rc<RefCell<XPath>>> {
        self.x_path.borrow().get(id).map(|x_path| x_path.clone())
    }

    /// Exorcise (remove) [`XPath`] from the store.
    ///
    pub fn exorcise_x_path(&mut self, id: &Uuid) -> Option<Rc<RefCell<XPath>>> {
        self.x_path
            .borrow_mut()
            .remove(id)
            .map(|x_path| x_path.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPath>`.
    ///
    pub fn iter_x_path(&self) -> impl Iterator<Item = Rc<RefCell<XPath>>> + '_ {
        let values: Vec<Rc<RefCell<XPath>>> = self
            .x_path
            .borrow()
            .values()
            .map(|x_path| x_path.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`PathElement`] into the store.
    ///
    pub fn inter_path_element(&mut self, path_element: Rc<RefCell<PathElement>>) {
        let read = path_element.borrow();
        self.path_element
            .borrow_mut()
            .insert(read.id, path_element.clone());
    }

    /// Exhume (get) [`PathElement`] from the store.
    ///
    pub fn exhume_path_element(&self, id: &Uuid) -> Option<Rc<RefCell<PathElement>>> {
        self.path_element
            .borrow()
            .get(id)
            .map(|path_element| path_element.clone())
    }

    /// Exorcise (remove) [`PathElement`] from the store.
    ///
    pub fn exorcise_path_element(&mut self, id: &Uuid) -> Option<Rc<RefCell<PathElement>>> {
        self.path_element
            .borrow_mut()
            .remove(id)
            .map(|path_element| path_element.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, PathElement>`.
    ///
    pub fn iter_path_element(&self) -> impl Iterator<Item = Rc<RefCell<PathElement>>> + '_ {
        let values: Vec<Rc<RefCell<PathElement>>> = self
            .path_element
            .borrow()
            .values()
            .map(|path_element| path_element.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Pattern`] into the store.
    ///
    pub fn inter_pattern(&mut self, pattern: Rc<RefCell<Pattern>>) {
        let read = pattern.borrow();
        self.pattern.borrow_mut().insert(read.id, pattern.clone());
    }

    /// Exhume (get) [`Pattern`] from the store.
    ///
    pub fn exhume_pattern(&self, id: &Uuid) -> Option<Rc<RefCell<Pattern>>> {
        self.pattern.borrow().get(id).map(|pattern| pattern.clone())
    }

    /// Exorcise (remove) [`Pattern`] from the store.
    ///
    pub fn exorcise_pattern(&mut self, id: &Uuid) -> Option<Rc<RefCell<Pattern>>> {
        self.pattern
            .borrow_mut()
            .remove(id)
            .map(|pattern| pattern.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Pattern>`.
    ///
    pub fn iter_pattern(&self) -> impl Iterator<Item = Rc<RefCell<Pattern>>> + '_ {
        let values: Vec<Rc<RefCell<Pattern>>> = self
            .pattern
            .borrow()
            .values()
            .map(|pattern| pattern.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XPlugin`] into the store.
    ///
    pub fn inter_x_plugin(&mut self, x_plugin: Rc<RefCell<XPlugin>>) {
        let read = x_plugin.borrow();
        self.x_plugin_id_by_name
            .borrow_mut()
            .insert(read.name.clone(), read.id);
        self.x_plugin.borrow_mut().insert(read.id, x_plugin.clone());
    }

    /// Exhume (get) [`XPlugin`] from the store.
    ///
    pub fn exhume_x_plugin(&self, id: &Uuid) -> Option<Rc<RefCell<XPlugin>>> {
        self.x_plugin
            .borrow()
            .get(id)
            .map(|x_plugin| x_plugin.clone())
    }

    /// Exorcise (remove) [`XPlugin`] from the store.
    ///
    pub fn exorcise_x_plugin(&mut self, id: &Uuid) -> Option<Rc<RefCell<XPlugin>>> {
        self.x_plugin
            .borrow_mut()
            .remove(id)
            .map(|x_plugin| x_plugin.clone())
    }

    /// Exhume [`XPlugin`] id from the store by name.
    ///
    pub fn exhume_x_plugin_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.x_plugin_id_by_name
            .borrow()
            .get(name)
            .map(|x_plugin| *x_plugin)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPlugin>`.
    ///
    pub fn iter_x_plugin(&self) -> impl Iterator<Item = Rc<RefCell<XPlugin>>> + '_ {
        let values: Vec<Rc<RefCell<XPlugin>>> = self
            .x_plugin
            .borrow()
            .values()
            .map(|x_plugin| x_plugin.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XPrint`] into the store.
    ///
    pub fn inter_x_print(&mut self, x_print: Rc<RefCell<XPrint>>) {
        let read = x_print.borrow();
        self.x_print.borrow_mut().insert(read.id, x_print.clone());
    }

    /// Exhume (get) [`XPrint`] from the store.
    ///
    pub fn exhume_x_print(&self, id: &Uuid) -> Option<Rc<RefCell<XPrint>>> {
        self.x_print.borrow().get(id).map(|x_print| x_print.clone())
    }

    /// Exorcise (remove) [`XPrint`] from the store.
    ///
    pub fn exorcise_x_print(&mut self, id: &Uuid) -> Option<Rc<RefCell<XPrint>>> {
        self.x_print
            .borrow_mut()
            .remove(id)
            .map(|x_print| x_print.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPrint>`.
    ///
    pub fn iter_x_print(&self) -> impl Iterator<Item = Rc<RefCell<XPrint>>> + '_ {
        let values: Vec<Rc<RefCell<XPrint>>> = self
            .x_print
            .borrow()
            .values()
            .map(|x_print| x_print.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub fn inter_range_expression(&mut self, range_expression: Rc<RefCell<RangeExpression>>) {
        let read = range_expression.borrow();
        self.range_expression
            .borrow_mut()
            .insert(read.id, range_expression.clone());
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub fn exhume_range_expression(&self, id: &Uuid) -> Option<Rc<RefCell<RangeExpression>>> {
        self.range_expression
            .borrow()
            .get(id)
            .map(|range_expression| range_expression.clone())
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub fn exorcise_range_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<RangeExpression>>> {
        self.range_expression
            .borrow_mut()
            .remove(id)
            .map(|range_expression| range_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub fn iter_range_expression(&self) -> impl Iterator<Item = Rc<RefCell<RangeExpression>>> + '_ {
        let values: Vec<Rc<RefCell<RangeExpression>>> = self
            .range_expression
            .borrow()
            .values()
            .map(|range_expression| range_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement(&mut self, result_statement: Rc<RefCell<ResultStatement>>) {
        let read = result_statement.borrow();
        self.result_statement
            .borrow_mut()
            .insert(read.id, result_statement.clone());
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &Uuid) -> Option<Rc<RefCell<ResultStatement>>> {
        self.result_statement
            .borrow()
            .get(id)
            .map(|result_statement| result_statement.clone())
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub fn exorcise_result_statement(&mut self, id: &Uuid) -> Option<Rc<RefCell<ResultStatement>>> {
        self.result_statement
            .borrow_mut()
            .remove(id)
            .map(|result_statement| result_statement.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Rc<RefCell<ResultStatement>>> + '_ {
        let values: Vec<Rc<RefCell<ResultStatement>>> = self
            .result_statement
            .borrow()
            .values()
            .map(|result_statement| result_statement.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub fn inter_x_return(&mut self, x_return: Rc<RefCell<XReturn>>) {
        let read = x_return.borrow();
        self.x_return.borrow_mut().insert(read.id, x_return.clone());
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub fn exhume_x_return(&self, id: &Uuid) -> Option<Rc<RefCell<XReturn>>> {
        self.x_return
            .borrow()
            .get(id)
            .map(|x_return| x_return.clone())
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub fn exorcise_x_return(&mut self, id: &Uuid) -> Option<Rc<RefCell<XReturn>>> {
        self.x_return
            .borrow_mut()
            .remove(id)
            .map(|x_return| x_return.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub fn iter_x_return(&self) -> impl Iterator<Item = Rc<RefCell<XReturn>>> + '_ {
        let values: Vec<Rc<RefCell<XReturn>>> = self
            .x_return
            .borrow()
            .values()
            .map(|x_return| x_return.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub fn inter_span(&mut self, span: Rc<RefCell<Span>>) {
        let read = span.borrow();
        self.span.borrow_mut().insert(read.id, span.clone());
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub fn exhume_span(&self, id: &Uuid) -> Option<Rc<RefCell<Span>>> {
        self.span.borrow().get(id).map(|span| span.clone())
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub fn exorcise_span(&mut self, id: &Uuid) -> Option<Rc<RefCell<Span>>> {
        self.span.borrow_mut().remove(id).map(|span| span.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub fn iter_span(&self) -> impl Iterator<Item = Rc<RefCell<Span>>> + '_ {
        let values: Vec<Rc<RefCell<Span>>> = self
            .span
            .borrow()
            .values()
            .map(|span| span.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Rc<RefCell<Statement>>) {
        let read = statement.borrow();
        self.statement
            .borrow_mut()
            .insert(read.id, statement.clone());
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<Rc<RefCell<Statement>>> {
        self.statement
            .borrow()
            .get(id)
            .map(|statement| statement.clone())
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &Uuid) -> Option<Rc<RefCell<Statement>>> {
        self.statement
            .borrow_mut()
            .remove(id)
            .map(|statement| statement.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = Rc<RefCell<Statement>>> + '_ {
        let values: Vec<Rc<RefCell<Statement>>> = self
            .statement
            .borrow()
            .values()
            .map(|statement| statement.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call(&mut self, static_method_call: Rc<RefCell<StaticMethodCall>>) {
        let read = static_method_call.borrow();
        self.static_method_call
            .borrow_mut()
            .insert(read.id, static_method_call.clone());
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &Uuid) -> Option<Rc<RefCell<StaticMethodCall>>> {
        self.static_method_call
            .borrow()
            .get(id)
            .map(|static_method_call| static_method_call.clone())
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub fn exorcise_static_method_call(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<StaticMethodCall>>> {
        self.static_method_call
            .borrow_mut()
            .remove(id)
            .map(|static_method_call| static_method_call.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<StaticMethodCall>>> + '_ {
        let values: Vec<Rc<RefCell<StaticMethodCall>>> = self
            .static_method_call
            .borrow()
            .values()
            .map(|static_method_call| static_method_call.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StringBit`] into the store.
    ///
    pub fn inter_string_bit(&mut self, string_bit: Rc<RefCell<StringBit>>) {
        let read = string_bit.borrow();
        self.string_bit
            .borrow_mut()
            .insert(read.id, string_bit.clone());
    }

    /// Exhume (get) [`StringBit`] from the store.
    ///
    pub fn exhume_string_bit(&self, id: &Uuid) -> Option<Rc<RefCell<StringBit>>> {
        self.string_bit
            .borrow()
            .get(id)
            .map(|string_bit| string_bit.clone())
    }

    /// Exorcise (remove) [`StringBit`] from the store.
    ///
    pub fn exorcise_string_bit(&mut self, id: &Uuid) -> Option<Rc<RefCell<StringBit>>> {
        self.string_bit
            .borrow_mut()
            .remove(id)
            .map(|string_bit| string_bit.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringBit>`.
    ///
    pub fn iter_string_bit(&self) -> impl Iterator<Item = Rc<RefCell<StringBit>>> + '_ {
        let values: Vec<Rc<RefCell<StringBit>>> = self
            .string_bit
            .borrow()
            .values()
            .map(|string_bit| string_bit.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal(&mut self, string_literal: Rc<RefCell<StringLiteral>>) {
        let read = string_literal.borrow();
        self.string_literal
            .borrow_mut()
            .insert(read.id, string_literal.clone());
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &Uuid) -> Option<Rc<RefCell<StringLiteral>>> {
        self.string_literal
            .borrow()
            .get(id)
            .map(|string_literal| string_literal.clone())
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub fn exorcise_string_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<StringLiteral>>> {
        self.string_literal
            .borrow_mut()
            .remove(id)
            .map(|string_literal| string_literal.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Rc<RefCell<StringLiteral>>> + '_ {
        let values: Vec<Rc<RefCell<StringLiteral>>> = self
            .string_literal
            .borrow()
            .values()
            .map(|string_literal| string_literal.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct(&mut self, woog_struct: Rc<RefCell<WoogStruct>>) {
        let read = woog_struct.borrow();
        self.woog_struct_id_by_name
            .borrow_mut()
            .insert(read.name.clone(), read.id);
        self.woog_struct
            .borrow_mut()
            .insert(read.id, woog_struct.clone());
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &Uuid) -> Option<Rc<RefCell<WoogStruct>>> {
        self.woog_struct
            .borrow()
            .get(id)
            .map(|woog_struct| woog_struct.clone())
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub fn exorcise_woog_struct(&mut self, id: &Uuid) -> Option<Rc<RefCell<WoogStruct>>> {
        self.woog_struct
            .borrow_mut()
            .remove(id)
            .map(|woog_struct| woog_struct.clone())
    }

    /// Exhume [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.woog_struct_id_by_name
            .borrow()
            .get(name)
            .map(|woog_struct| *woog_struct)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Rc<RefCell<WoogStruct>>> + '_ {
        let values: Vec<Rc<RefCell<WoogStruct>>> = self
            .woog_struct
            .borrow()
            .values()
            .map(|woog_struct| woog_struct.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub fn inter_struct_expression(&mut self, struct_expression: Rc<RefCell<StructExpression>>) {
        let read = struct_expression.borrow();
        self.struct_expression
            .borrow_mut()
            .insert(read.id, struct_expression.clone());
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &Uuid) -> Option<Rc<RefCell<StructExpression>>> {
        self.struct_expression
            .borrow()
            .get(id)
            .map(|struct_expression| struct_expression.clone())
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub fn exorcise_struct_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<StructExpression>>> {
        self.struct_expression
            .borrow_mut()
            .remove(id)
            .map(|struct_expression| struct_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<StructExpression>>> + '_ {
        let values: Vec<Rc<RefCell<StructExpression>>> = self
            .struct_expression
            .borrow()
            .values()
            .map(|struct_expression| struct_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StructField`] into the store.
    ///
    pub fn inter_struct_field(&mut self, struct_field: Rc<RefCell<StructField>>) {
        let read = struct_field.borrow();
        self.struct_field
            .borrow_mut()
            .insert(read.id, struct_field.clone());
    }

    /// Exhume (get) [`StructField`] from the store.
    ///
    pub fn exhume_struct_field(&self, id: &Uuid) -> Option<Rc<RefCell<StructField>>> {
        self.struct_field
            .borrow()
            .get(id)
            .map(|struct_field| struct_field.clone())
    }

    /// Exorcise (remove) [`StructField`] from the store.
    ///
    pub fn exorcise_struct_field(&mut self, id: &Uuid) -> Option<Rc<RefCell<StructField>>> {
        self.struct_field
            .borrow_mut()
            .remove(id)
            .map(|struct_field| struct_field.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructField>`.
    ///
    pub fn iter_struct_field(&self) -> impl Iterator<Item = Rc<RefCell<StructField>>> + '_ {
        let values: Vec<Rc<RefCell<StructField>>> = self
            .struct_field
            .borrow()
            .values()
            .map(|struct_field| struct_field.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`StructGeneric`] into the store.
    ///
    pub fn inter_struct_generic(&mut self, struct_generic: Rc<RefCell<StructGeneric>>) {
        let read = struct_generic.borrow();
        self.struct_generic
            .borrow_mut()
            .insert(read.id, struct_generic.clone());
    }

    /// Exhume (get) [`StructGeneric`] from the store.
    ///
    pub fn exhume_struct_generic(&self, id: &Uuid) -> Option<Rc<RefCell<StructGeneric>>> {
        self.struct_generic
            .borrow()
            .get(id)
            .map(|struct_generic| struct_generic.clone())
    }

    /// Exorcise (remove) [`StructGeneric`] from the store.
    ///
    pub fn exorcise_struct_generic(&mut self, id: &Uuid) -> Option<Rc<RefCell<StructGeneric>>> {
        self.struct_generic
            .borrow_mut()
            .remove(id)
            .map(|struct_generic| struct_generic.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructGeneric>`.
    ///
    pub fn iter_struct_generic(&self) -> impl Iterator<Item = Rc<RefCell<StructGeneric>>> + '_ {
        let values: Vec<Rc<RefCell<StructGeneric>>> = self
            .struct_generic
            .borrow()
            .values()
            .map(|struct_generic| struct_generic.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`TupleField`] into the store.
    ///
    pub fn inter_tuple_field(&mut self, tuple_field: Rc<RefCell<TupleField>>) {
        let read = tuple_field.borrow();
        self.tuple_field
            .borrow_mut()
            .insert(read.id, tuple_field.clone());
    }

    /// Exhume (get) [`TupleField`] from the store.
    ///
    pub fn exhume_tuple_field(&self, id: &Uuid) -> Option<Rc<RefCell<TupleField>>> {
        self.tuple_field
            .borrow()
            .get(id)
            .map(|tuple_field| tuple_field.clone())
    }

    /// Exorcise (remove) [`TupleField`] from the store.
    ///
    pub fn exorcise_tuple_field(&mut self, id: &Uuid) -> Option<Rc<RefCell<TupleField>>> {
        self.tuple_field
            .borrow_mut()
            .remove(id)
            .map(|tuple_field| tuple_field.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TupleField>`.
    ///
    pub fn iter_tuple_field(&self) -> impl Iterator<Item = Rc<RefCell<TupleField>>> + '_ {
        let values: Vec<Rc<RefCell<TupleField>>> = self
            .tuple_field
            .borrow()
            .values()
            .map(|tuple_field| tuple_field.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast(&mut self, type_cast: Rc<RefCell<TypeCast>>) {
        let read = type_cast.borrow();
        self.type_cast
            .borrow_mut()
            .insert(read.id, type_cast.clone());
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub fn exhume_type_cast(&self, id: &Uuid) -> Option<Rc<RefCell<TypeCast>>> {
        self.type_cast
            .borrow()
            .get(id)
            .map(|type_cast| type_cast.clone())
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub fn exorcise_type_cast(&mut self, id: &Uuid) -> Option<Rc<RefCell<TypeCast>>> {
        self.type_cast
            .borrow_mut()
            .remove(id)
            .map(|type_cast| type_cast.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub fn iter_type_cast(&self) -> impl Iterator<Item = Rc<RefCell<TypeCast>>> + '_ {
        let values: Vec<Rc<RefCell<TypeCast>>> = self
            .type_cast
            .borrow()
            .values()
            .map(|type_cast| type_cast.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub fn inter_unary(&mut self, unary: Rc<RefCell<Unary>>) {
        let read = unary.borrow();
        self.unary.borrow_mut().insert(read.id, unary.clone());
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub fn exhume_unary(&self, id: &Uuid) -> Option<Rc<RefCell<Unary>>> {
        self.unary.borrow().get(id).map(|unary| unary.clone())
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub fn exorcise_unary(&mut self, id: &Uuid) -> Option<Rc<RefCell<Unary>>> {
        self.unary
            .borrow_mut()
            .remove(id)
            .map(|unary| unary.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub fn iter_unary(&self) -> impl Iterator<Item = Rc<RefCell<Unary>>> + '_ {
        let values: Vec<Rc<RefCell<Unary>>> = self
            .unary
            .borrow()
            .values()
            .map(|unary| unary.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Unit`] into the store.
    ///
    pub fn inter_unit(&mut self, unit: Rc<RefCell<Unit>>) {
        let read = unit.borrow();
        self.unit.borrow_mut().insert(read.id, unit.clone());
    }

    /// Exhume (get) [`Unit`] from the store.
    ///
    pub fn exhume_unit(&self, id: &Uuid) -> Option<Rc<RefCell<Unit>>> {
        self.unit.borrow().get(id).map(|unit| unit.clone())
    }

    /// Exorcise (remove) [`Unit`] from the store.
    ///
    pub fn exorcise_unit(&mut self, id: &Uuid) -> Option<Rc<RefCell<Unit>>> {
        self.unit.borrow_mut().remove(id).map(|unit| unit.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unit>`.
    ///
    pub fn iter_unit(&self) -> impl Iterator<Item = Rc<RefCell<Unit>>> + '_ {
        let values: Vec<Rc<RefCell<Unit>>> = self
            .unit
            .borrow()
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
        unnamed_field_expression: Rc<RefCell<UnnamedFieldExpression>>,
    ) {
        let read = unnamed_field_expression.borrow();
        self.unnamed_field_expression
            .borrow_mut()
            .insert(read.id, unnamed_field_expression.clone());
    }

    /// Exhume (get) [`UnnamedFieldExpression`] from the store.
    ///
    pub fn exhume_unnamed_field_expression(
        &self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<UnnamedFieldExpression>>> {
        self.unnamed_field_expression
            .borrow()
            .get(id)
            .map(|unnamed_field_expression| unnamed_field_expression.clone())
    }

    /// Exorcise (remove) [`UnnamedFieldExpression`] from the store.
    ///
    pub fn exorcise_unnamed_field_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<UnnamedFieldExpression>>> {
        self.unnamed_field_expression
            .borrow_mut()
            .remove(id)
            .map(|unnamed_field_expression| unnamed_field_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, UnnamedFieldExpression>`.
    ///
    pub fn iter_unnamed_field_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<UnnamedFieldExpression>>> + '_ {
        let values: Vec<Rc<RefCell<UnnamedFieldExpression>>> = self
            .unnamed_field_expression
            .borrow()
            .values()
            .map(|unnamed_field_expression| unnamed_field_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value(&mut self, x_value: Rc<RefCell<XValue>>) {
        let read = x_value.borrow();
        self.x_value.borrow_mut().insert(read.id, x_value.clone());
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &Uuid) -> Option<Rc<RefCell<XValue>>> {
        self.x_value.borrow().get(id).map(|x_value| x_value.clone())
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &Uuid) -> Option<Rc<RefCell<XValue>>> {
        self.x_value
            .borrow_mut()
            .remove(id)
            .map(|x_value| x_value.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = Rc<RefCell<XValue>>> + '_ {
        let values: Vec<Rc<RefCell<XValue>>> = self
            .x_value
            .borrow()
            .values()
            .map(|x_value| x_value.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub fn inter_value_type(&mut self, value_type: Rc<RefCell<ValueType>>) {
        let read = value_type.borrow();
        self.value_type
            .borrow_mut()
            .insert(read.id, value_type.clone());
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &Uuid) -> Option<Rc<RefCell<ValueType>>> {
        self.value_type
            .borrow()
            .get(id)
            .map(|value_type| value_type.clone())
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub fn exorcise_value_type(&mut self, id: &Uuid) -> Option<Rc<RefCell<ValueType>>> {
        self.value_type
            .borrow_mut()
            .remove(id)
            .map(|value_type| value_type.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = Rc<RefCell<ValueType>>> + '_ {
        let values: Vec<Rc<RefCell<ValueType>>> = self
            .value_type
            .borrow()
            .values()
            .map(|value_type| value_type.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Rc<RefCell<Variable>>) {
        let read = variable.borrow();
        self.variable.borrow_mut().insert(read.id, variable.clone());
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<Rc<RefCell<Variable>>> {
        self.variable
            .borrow()
            .get(id)
            .map(|variable| variable.clone())
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &Uuid) -> Option<Rc<RefCell<Variable>>> {
        self.variable
            .borrow_mut()
            .remove(id)
            .map(|variable| variable.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = Rc<RefCell<Variable>>> + '_ {
        let values: Vec<Rc<RefCell<Variable>>> = self
            .variable
            .borrow()
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
        variable_expression: Rc<RefCell<VariableExpression>>,
    ) {
        let read = variable_expression.borrow();
        self.variable_expression
            .borrow_mut()
            .insert(read.id, variable_expression.clone());
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(&self, id: &Uuid) -> Option<Rc<RefCell<VariableExpression>>> {
        self.variable_expression
            .borrow()
            .get(id)
            .map(|variable_expression| variable_expression.clone())
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub fn exorcise_variable_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<VariableExpression>>> {
        self.variable_expression
            .borrow_mut()
            .remove(id)
            .map(|variable_expression| variable_expression.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<VariableExpression>>> + '_ {
        let values: Vec<Rc<RefCell<VariableExpression>>> = self
            .variable_expression
            .borrow()
            .values()
            .map(|variable_expression| variable_expression.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-persistence"}}}
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
            for argument in self.argument.borrow().values() {
                let path = path.join(format!("{}.json", argument.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &argument)?;
            }
        }

        // Persist Await.
        {
            let path = path.join("a_wait");
            fs::create_dir_all(&path)?;
            for a_wait in self.a_wait.borrow().values() {
                let path = path.join(format!("{}.json", a_wait.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &a_wait)?;
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary in self.binary.borrow().values() {
                let path = path.join(format!("{}.json", binary.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &binary)?;
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block in self.block.borrow().values() {
                let path = path.join(format!("{}.json", block.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &block)?;
            }
        }

        // Persist Body.
        {
            let path = path.join("body");
            fs::create_dir_all(&path)?;
            for body in self.body.borrow().values() {
                let path = path.join(format!("{}.json", body.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &body)?;
            }
        }

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal in self.boolean_literal.borrow().values() {
                let path = path.join(format!("{}.json", boolean_literal.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &boolean_literal)?;
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator in self.boolean_operator.borrow().values() {
                let path = path.join(format!("{}.json", boolean_operator.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &boolean_operator)?;
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call in self.call.borrow().values() {
                let path = path.join(format!("{}.json", call.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &call)?;
            }
        }

        // Persist Char Literal.
        {
            let path = path.join("char_literal");
            fs::create_dir_all(&path)?;
            for char_literal in self.char_literal.borrow().values() {
                let path = path.join(format!("{}.json", char_literal.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &char_literal)?;
            }
        }

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison in self.comparison.borrow().values() {
                let path = path.join(format!("{}.json", comparison.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &comparison)?;
            }
        }

        // Persist Data Structure.
        {
            let path = path.join("data_structure");
            fs::create_dir_all(&path)?;
            for data_structure in self.data_structure.borrow().values() {
                let path = path.join(format!("{}.json", data_structure.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &data_structure)?;
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file in self.dwarf_source_file.borrow().values() {
                let path = path.join(format!("{}.json", dwarf_source_file.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &dwarf_source_file)?;
            }
        }

        // Persist Enum Field.
        {
            let path = path.join("enum_field");
            fs::create_dir_all(&path)?;
            for enum_field in self.enum_field.borrow().values() {
                let path = path.join(format!("{}.json", enum_field.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enum_field)?;
            }
        }

        // Persist Enum Generic.
        {
            let path = path.join("enum_generic");
            fs::create_dir_all(&path)?;
            for enum_generic in self.enum_generic.borrow().values() {
                let path = path.join(format!("{}.json", enum_generic.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enum_generic)?;
            }
        }

        // Persist Enum Generic Type.
        {
            let path = path.join("enum_generic_type");
            fs::create_dir_all(&path)?;
            for enum_generic_type in self.enum_generic_type.borrow().values() {
                let path = path.join(format!("{}.json", enum_generic_type.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enum_generic_type)?;
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration in self.enumeration.borrow().values() {
                let path = path.join(format!("{}.json", enumeration.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enumeration)?;
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression in self.expression.borrow().values() {
                let path = path.join(format!("{}.json", expression.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &expression)?;
            }
        }

        // Persist Expression Bit.
        {
            let path = path.join("expression_bit");
            fs::create_dir_all(&path)?;
            for expression_bit in self.expression_bit.borrow().values() {
                let path = path.join(format!("{}.json", expression_bit.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &expression_bit)?;
            }
        }

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement in self.expression_statement.borrow().values() {
                let path = path.join(format!("{}.json", expression_statement.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &expression_statement)?;
            }
        }

        // Persist External Implementation.
        {
            let path = path.join("external_implementation");
            fs::create_dir_all(&path)?;
            for external_implementation in self.external_implementation.borrow().values() {
                let path = path.join(format!("{}.json", external_implementation.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &external_implementation)?;
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field in self.field.borrow().values() {
                let path = path.join(format!("{}.json", field.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field)?;
            }
        }

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access in self.field_access.borrow().values() {
                let path = path.join(format!("{}.json", field_access.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field_access)?;
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target in self.field_access_target.borrow().values() {
                let path = path.join(format!("{}.json", field_access_target.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field_access_target)?;
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression in self.field_expression.borrow().values() {
                let path = path.join(format!("{}.json", field_expression.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field_expression)?;
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal in self.float_literal.borrow().values() {
                let path = path.join(format!("{}.json", float_literal.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &float_literal)?;
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop in self.for_loop.borrow().values() {
                let path = path.join(format!("{}.json", for_loop.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &for_loop)?;
            }
        }

        // Persist Format Bit.
        {
            let path = path.join("format_bit");
            fs::create_dir_all(&path)?;
            for format_bit in self.format_bit.borrow().values() {
                let path = path.join(format!("{}.json", format_bit.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &format_bit)?;
            }
        }

        // Persist Format String.
        {
            let path = path.join("format_string");
            fs::create_dir_all(&path)?;
            for format_string in self.format_string.borrow().values() {
                let path = path.join(format!("{}.json", format_string.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &format_string)?;
            }
        }

        // Persist Func Generic.
        {
            let path = path.join("func_generic");
            fs::create_dir_all(&path)?;
            for func_generic in self.func_generic.borrow().values() {
                let path = path.join(format!("{}.json", func_generic.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &func_generic)?;
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function in self.function.borrow().values() {
                let path = path.join(format!("{}.json", function.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &function)?;
            }
        }

        // Persist Function Call.
        {
            let path = path.join("function_call");
            fs::create_dir_all(&path)?;
            for function_call in self.function_call.borrow().values() {
                let path = path.join(format!("{}.json", function_call.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &function_call)?;
            }
        }

        // Persist Future.
        {
            let path = path.join("x_future");
            fs::create_dir_all(&path)?;
            for x_future in self.x_future.borrow().values() {
                let path = path.join(format!("{}.json", x_future.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_future)?;
            }
        }

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped in self.grouped.borrow().values() {
                let path = path.join(format!("{}.json", grouped.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &grouped)?;
            }
        }

        // Persist Halt and Catch Fire.
        {
            let path = path.join("halt_and_catch_fire");
            fs::create_dir_all(&path)?;
            for halt_and_catch_fire in self.halt_and_catch_fire.borrow().values() {
                let path = path.join(format!("{}.json", halt_and_catch_fire.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &halt_and_catch_fire)?;
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if in self.x_if.borrow().values() {
                let path = path.join(format!("{}.json", x_if.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_if)?;
            }
        }

        // Persist Implementation Block.
        {
            let path = path.join("implementation_block");
            fs::create_dir_all(&path)?;
            for implementation_block in self.implementation_block.borrow().values() {
                let path = path.join(format!("{}.json", implementation_block.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &implementation_block)?;
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import in self.import.borrow().values() {
                let path = path.join(format!("{}.json", import.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &import)?;
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index in self.index.borrow().values() {
                let path = path.join(format!("{}.json", index.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &index)?;
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal in self.integer_literal.borrow().values() {
                let path = path.join(format!("{}.json", integer_literal.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &integer_literal)?;
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item in self.item.borrow().values() {
                let path = path.join(format!("{}.json", item.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &item)?;
            }
        }

        // Persist Lambda.
        {
            let path = path.join("lambda");
            fs::create_dir_all(&path)?;
            for lambda in self.lambda.borrow().values() {
                let path = path.join(format!("{}.json", lambda.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &lambda)?;
            }
        }

        // Persist Lambda Parameter.
        {
            let path = path.join("lambda_parameter");
            fs::create_dir_all(&path)?;
            for lambda_parameter in self.lambda_parameter.borrow().values() {
                let path = path.join(format!("{}.json", lambda_parameter.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &lambda_parameter)?;
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement in self.let_statement.borrow().values() {
                let path = path.join(format!("{}.json", let_statement.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &let_statement)?;
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list in self.list.borrow().values() {
                let path = path.join(format!("{}.json", list.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &list)?;
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element in self.list_element.borrow().values() {
                let path = path.join(format!("{}.json", list_element.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &list_element)?;
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression in self.list_expression.borrow().values() {
                let path = path.join(format!("{}.json", list_expression.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &list_expression)?;
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal in self.literal.borrow().values() {
                let path = path.join(format!("{}.json", literal.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &literal)?;
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable in self.local_variable.borrow().values() {
                let path = path.join(format!("{}.json", local_variable.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &local_variable)?;
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro in self.x_macro.borrow().values() {
                let path = path.join(format!("{}.json", x_macro.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_macro)?;
            }
        }

        // Persist Map.
        {
            let path = path.join("map");
            fs::create_dir_all(&path)?;
            for map in self.map.borrow().values() {
                let path = path.join(format!("{}.json", map.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &map)?;
            }
        }

        // Persist Map Element.
        {
            let path = path.join("map_element");
            fs::create_dir_all(&path)?;
            for map_element in self.map_element.borrow().values() {
                let path = path.join(format!("{}.json", map_element.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &map_element)?;
            }
        }

        // Persist Map Expression.
        {
            let path = path.join("map_expression");
            fs::create_dir_all(&path)?;
            for map_expression in self.map_expression.borrow().values() {
                let path = path.join(format!("{}.json", map_expression.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &map_expression)?;
            }
        }

        // Persist Match.
        {
            let path = path.join("x_match");
            fs::create_dir_all(&path)?;
            for x_match in self.x_match.borrow().values() {
                let path = path.join(format!("{}.json", x_match.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_match)?;
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call in self.method_call.borrow().values() {
                let path = path.join(format!("{}.json", method_call.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &method_call)?;
            }
        }

        // Persist Named Field Expression.
        {
            let path = path.join("named_field_expression");
            fs::create_dir_all(&path)?;
            for named_field_expression in self.named_field_expression.borrow().values() {
                let path = path.join(format!("{}.json", named_field_expression.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &named_field_expression)?;
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store in self.z_object_store.borrow().values() {
                let path = path.join(format!("{}.json", z_object_store.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &z_object_store)?;
            }
        }

        // Persist Object Wrapper.
        {
            let path = path.join("object_wrapper");
            fs::create_dir_all(&path)?;
            for object_wrapper in self.object_wrapper.borrow().values() {
                let path = path.join(format!("{}.json", object_wrapper.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &object_wrapper)?;
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator in self.operator.borrow().values() {
                let path = path.join(format!("{}.json", operator.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &operator)?;
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter in self.parameter.borrow().values() {
                let path = path.join(format!("{}.json", parameter.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &parameter)?;
            }
        }

        // Persist Path.
        {
            let path = path.join("x_path");
            fs::create_dir_all(&path)?;
            for x_path in self.x_path.borrow().values() {
                let path = path.join(format!("{}.json", x_path.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_path)?;
            }
        }

        // Persist Path Element.
        {
            let path = path.join("path_element");
            fs::create_dir_all(&path)?;
            for path_element in self.path_element.borrow().values() {
                let path = path.join(format!("{}.json", path_element.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &path_element)?;
            }
        }

        // Persist Pattern.
        {
            let path = path.join("pattern");
            fs::create_dir_all(&path)?;
            for pattern in self.pattern.borrow().values() {
                let path = path.join(format!("{}.json", pattern.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &pattern)?;
            }
        }

        // Persist Plugin.
        {
            let path = path.join("x_plugin");
            fs::create_dir_all(&path)?;
            for x_plugin in self.x_plugin.borrow().values() {
                let path = path.join(format!("{}.json", x_plugin.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_plugin)?;
            }
        }

        // Persist Print.
        {
            let path = path.join("x_print");
            fs::create_dir_all(&path)?;
            for x_print in self.x_print.borrow().values() {
                let path = path.join(format!("{}.json", x_print.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_print)?;
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression in self.range_expression.borrow().values() {
                let path = path.join(format!("{}.json", range_expression.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &range_expression)?;
            }
        }

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement in self.result_statement.borrow().values() {
                let path = path.join(format!("{}.json", result_statement.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &result_statement)?;
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return in self.x_return.borrow().values() {
                let path = path.join(format!("{}.json", x_return.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_return)?;
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span in self.span.borrow().values() {
                let path = path.join(format!("{}.json", span.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &span)?;
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement in self.statement.borrow().values() {
                let path = path.join(format!("{}.json", statement.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &statement)?;
            }
        }

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call in self.static_method_call.borrow().values() {
                let path = path.join(format!("{}.json", static_method_call.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &static_method_call)?;
            }
        }

        // Persist String Bit.
        {
            let path = path.join("string_bit");
            fs::create_dir_all(&path)?;
            for string_bit in self.string_bit.borrow().values() {
                let path = path.join(format!("{}.json", string_bit.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &string_bit)?;
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal in self.string_literal.borrow().values() {
                let path = path.join(format!("{}.json", string_literal.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &string_literal)?;
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct in self.woog_struct.borrow().values() {
                let path = path.join(format!("{}.json", woog_struct.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &woog_struct)?;
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression in self.struct_expression.borrow().values() {
                let path = path.join(format!("{}.json", struct_expression.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &struct_expression)?;
            }
        }

        // Persist Struct Field.
        {
            let path = path.join("struct_field");
            fs::create_dir_all(&path)?;
            for struct_field in self.struct_field.borrow().values() {
                let path = path.join(format!("{}.json", struct_field.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &struct_field)?;
            }
        }

        // Persist Struct Generic.
        {
            let path = path.join("struct_generic");
            fs::create_dir_all(&path)?;
            for struct_generic in self.struct_generic.borrow().values() {
                let path = path.join(format!("{}.json", struct_generic.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &struct_generic)?;
            }
        }

        // Persist Tuple Field.
        {
            let path = path.join("tuple_field");
            fs::create_dir_all(&path)?;
            for tuple_field in self.tuple_field.borrow().values() {
                let path = path.join(format!("{}.json", tuple_field.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &tuple_field)?;
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast in self.type_cast.borrow().values() {
                let path = path.join(format!("{}.json", type_cast.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &type_cast)?;
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary in self.unary.borrow().values() {
                let path = path.join(format!("{}.json", unary.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &unary)?;
            }
        }

        // Persist Unit.
        {
            let path = path.join("unit");
            fs::create_dir_all(&path)?;
            for unit in self.unit.borrow().values() {
                let path = path.join(format!("{}.json", unit.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &unit)?;
            }
        }

        // Persist Unnamed Field Expression.
        {
            let path = path.join("unnamed_field_expression");
            fs::create_dir_all(&path)?;
            for unnamed_field_expression in self.unnamed_field_expression.borrow().values() {
                let path = path.join(format!("{}.json", unnamed_field_expression.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &unnamed_field_expression)?;
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value in self.x_value.borrow().values() {
                let path = path.join(format!("{}.json", x_value.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_value)?;
            }
        }

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type in self.value_type.borrow().values() {
                let path = path.join(format!("{}.json", value_type.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &value_type)?;
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable in self.variable.borrow().values() {
                let path = path.join(format!("{}.json", variable.borrow().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &variable)?;
            }
        }

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression in self.variable_expression.borrow().values() {
                let path = path.join(format!("{}.json", variable_expression.borrow().id));
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
                let argument: Rc<RefCell<Argument>> = serde_json::from_reader(reader)?;
                store
                    .argument
                    .borrow_mut()
                    .insert(argument.borrow().id, argument.clone());
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
                    .borrow_mut()
                    .insert(a_wait.borrow().id, a_wait.clone());
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
                    .borrow_mut()
                    .insert(binary.borrow().id, binary.clone());
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
                store
                    .block
                    .borrow_mut()
                    .insert(block.borrow().id, block.clone());
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
                store
                    .body
                    .borrow_mut()
                    .insert(body.borrow().id, body.clone());
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
                    .borrow_mut()
                    .insert(boolean_literal.borrow().id, boolean_literal.clone());
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
                    .borrow_mut()
                    .insert(boolean_operator.borrow().id, boolean_operator.clone());
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
                store
                    .call
                    .borrow_mut()
                    .insert(call.borrow().id, call.clone());
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
                    .borrow_mut()
                    .insert(char_literal.borrow().id, char_literal.clone());
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
                    .borrow_mut()
                    .insert(comparison.borrow().id, comparison.clone());
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
                    .borrow_mut()
                    .insert(data_structure.borrow().id, data_structure.clone());
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
                store
                    .dwarf_source_file
                    .borrow_mut()
                    .insert(dwarf_source_file.borrow().id, dwarf_source_file.clone());
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
                    .borrow_mut()
                    .insert(enum_field.borrow().id, enum_field.clone());
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
                    .borrow_mut()
                    .insert(enum_generic.borrow().id, enum_generic.clone());
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
                store
                    .enum_generic_type
                    .borrow_mut()
                    .insert(enum_generic_type.borrow().id, enum_generic_type.clone());
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
                store
                    .enumeration_id_by_name
                    .borrow_mut()
                    .insert(enumeration.borrow().name.clone(), enumeration.borrow().id);
                store
                    .enumeration
                    .borrow_mut()
                    .insert(enumeration.borrow().id, enumeration.clone());
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
                    .borrow_mut()
                    .insert(expression.borrow().id, expression.clone());
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
                    .borrow_mut()
                    .insert(expression_bit.borrow().id, expression_bit.clone());
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
                store.expression_statement.borrow_mut().insert(
                    expression_statement.borrow().id,
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
                let external_implementation: Rc<RefCell<ExternalImplementation>> =
                    serde_json::from_reader(reader)?;
                store.external_implementation.borrow_mut().insert(
                    external_implementation.borrow().id,
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
                let field: Rc<RefCell<Field>> = serde_json::from_reader(reader)?;
                store
                    .field_id_by_name
                    .borrow_mut()
                    .insert(field.borrow().name.clone(), field.borrow().id);
                store
                    .field
                    .borrow_mut()
                    .insert(field.borrow().id, field.clone());
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
                    .borrow_mut()
                    .insert(field_access.borrow().id, field_access.clone());
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
                store
                    .field_access_target
                    .borrow_mut()
                    .insert(field_access_target.borrow().id, field_access_target.clone());
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
                    .borrow_mut()
                    .insert(field_expression.borrow().id, field_expression.clone());
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
                    .borrow_mut()
                    .insert(float_literal.borrow().id, float_literal.clone());
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
                    .borrow_mut()
                    .insert(for_loop.borrow().id, for_loop.clone());
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
                    .borrow_mut()
                    .insert(format_bit.borrow().id, format_bit.clone());
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
                    .borrow_mut()
                    .insert(format_string.borrow().id, format_string.clone());
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
                    .borrow_mut()
                    .insert(func_generic.borrow().id, func_generic.clone());
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
                    .borrow_mut()
                    .insert(function.borrow().name.clone(), function.borrow().id);
                store
                    .function
                    .borrow_mut()
                    .insert(function.borrow().id, function.clone());
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
                    .borrow_mut()
                    .insert(function_call.borrow().id, function_call.clone());
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
                    .borrow_mut()
                    .insert(x_future.borrow().id, x_future.clone());
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
                    .borrow_mut()
                    .insert(grouped.borrow().id, grouped.clone());
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
                store
                    .halt_and_catch_fire
                    .borrow_mut()
                    .insert(halt_and_catch_fire.borrow().id, halt_and_catch_fire.clone());
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
                store
                    .x_if
                    .borrow_mut()
                    .insert(x_if.borrow().id, x_if.clone());
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
                store.implementation_block.borrow_mut().insert(
                    implementation_block.borrow().id,
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
                let import: Rc<RefCell<Import>> = serde_json::from_reader(reader)?;
                store
                    .import
                    .borrow_mut()
                    .insert(import.borrow().id, import.clone());
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
                store
                    .index
                    .borrow_mut()
                    .insert(index.borrow().id, index.clone());
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
                    .borrow_mut()
                    .insert(integer_literal.borrow().id, integer_literal.clone());
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
                store
                    .item
                    .borrow_mut()
                    .insert(item.borrow().id, item.clone());
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
                    .borrow_mut()
                    .insert(lambda.borrow().id, lambda.clone());
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
                    .borrow_mut()
                    .insert(lambda_parameter.borrow().id, lambda_parameter.clone());
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
                    .borrow_mut()
                    .insert(let_statement.borrow().id, let_statement.clone());
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
                store
                    .list
                    .borrow_mut()
                    .insert(list.borrow().id, list.clone());
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
                    .borrow_mut()
                    .insert(list_element.borrow().id, list_element.clone());
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
                    .borrow_mut()
                    .insert(list_expression.borrow().id, list_expression.clone());
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
                    .borrow_mut()
                    .insert(literal.borrow().id, literal.clone());
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
                    .borrow_mut()
                    .insert(local_variable.borrow().id, local_variable.clone());
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
                    .borrow_mut()
                    .insert(x_macro.borrow().id, x_macro.clone());
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
                store.map.borrow_mut().insert(map.borrow().id, map.clone());
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
                    .borrow_mut()
                    .insert(map_element.borrow().id, map_element.clone());
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
                    .borrow_mut()
                    .insert(map_expression.borrow().id, map_expression.clone());
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
                    .borrow_mut()
                    .insert(x_match.borrow().id, x_match.clone());
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
                    .borrow_mut()
                    .insert(method_call.borrow().id, method_call.clone());
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
                store.named_field_expression.borrow_mut().insert(
                    named_field_expression.borrow().id,
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
                let z_object_store: Rc<RefCell<ZObjectStore>> = serde_json::from_reader(reader)?;
                store.z_object_store_id_by_name.borrow_mut().insert(
                    z_object_store.borrow().name.clone(),
                    z_object_store.borrow().id,
                );
                store
                    .z_object_store
                    .borrow_mut()
                    .insert(z_object_store.borrow().id, z_object_store.clone());
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
                    .borrow_mut()
                    .insert(object_wrapper.borrow().id, object_wrapper.clone());
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
                    .borrow_mut()
                    .insert(operator.borrow().id, operator.clone());
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
                    .borrow_mut()
                    .insert(parameter.borrow().id, parameter.clone());
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
                    .borrow_mut()
                    .insert(x_path.borrow().id, x_path.clone());
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
                    .borrow_mut()
                    .insert(path_element.borrow().id, path_element.clone());
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
                    .borrow_mut()
                    .insert(pattern.borrow().id, pattern.clone());
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
                    .borrow_mut()
                    .insert(x_plugin.borrow().name.clone(), x_plugin.borrow().id);
                store
                    .x_plugin
                    .borrow_mut()
                    .insert(x_plugin.borrow().id, x_plugin.clone());
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
                    .borrow_mut()
                    .insert(x_print.borrow().id, x_print.clone());
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
                    .borrow_mut()
                    .insert(range_expression.borrow().id, range_expression.clone());
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
                    .borrow_mut()
                    .insert(result_statement.borrow().id, result_statement.clone());
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
                    .borrow_mut()
                    .insert(x_return.borrow().id, x_return.clone());
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
                store
                    .span
                    .borrow_mut()
                    .insert(span.borrow().id, span.clone());
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
                    .borrow_mut()
                    .insert(statement.borrow().id, statement.clone());
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
                store
                    .static_method_call
                    .borrow_mut()
                    .insert(static_method_call.borrow().id, static_method_call.clone());
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
                    .borrow_mut()
                    .insert(string_bit.borrow().id, string_bit.clone());
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
                    .borrow_mut()
                    .insert(string_literal.borrow().id, string_literal.clone());
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
                store
                    .woog_struct_id_by_name
                    .borrow_mut()
                    .insert(woog_struct.borrow().name.clone(), woog_struct.borrow().id);
                store
                    .woog_struct
                    .borrow_mut()
                    .insert(woog_struct.borrow().id, woog_struct.clone());
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
                store
                    .struct_expression
                    .borrow_mut()
                    .insert(struct_expression.borrow().id, struct_expression.clone());
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
                    .borrow_mut()
                    .insert(struct_field.borrow().id, struct_field.clone());
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
                    .borrow_mut()
                    .insert(struct_generic.borrow().id, struct_generic.clone());
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
                    .borrow_mut()
                    .insert(tuple_field.borrow().id, tuple_field.clone());
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
                    .borrow_mut()
                    .insert(type_cast.borrow().id, type_cast.clone());
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
                store
                    .unary
                    .borrow_mut()
                    .insert(unary.borrow().id, unary.clone());
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
                store
                    .unit
                    .borrow_mut()
                    .insert(unit.borrow().id, unit.clone());
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
                store.unnamed_field_expression.borrow_mut().insert(
                    unnamed_field_expression.borrow().id,
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
                let x_value: Rc<RefCell<XValue>> = serde_json::from_reader(reader)?;
                store
                    .x_value
                    .borrow_mut()
                    .insert(x_value.borrow().id, x_value.clone());
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
                    .borrow_mut()
                    .insert(value_type.borrow().id, value_type.clone());
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
                    .borrow_mut()
                    .insert(variable.borrow().id, variable.clone());
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
                store
                    .variable_expression
                    .borrow_mut()
                    .insert(variable_expression.borrow().id, variable_expression.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
