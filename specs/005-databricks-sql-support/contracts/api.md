# API Contracts: Enhanced Databricks SQL Syntax Support

**Feature**: Enhanced Databricks SQL Syntax Support
**Date**: 2026-01-04
**Phase**: Phase 1 - Design

## Overview

This feature extends the existing SQL import API to support Databricks dialect. No new public API functions are introduced - the feature enhances existing `SQLImporter` functionality.

## Public API (No Changes)

### SQLImporter

**Module**: `data_modelling_sdk::import::sql`

**Existing API** (unchanged):

```rust
pub struct SQLImporter {
    pub dialect: String,
}

impl SQLImporter {
    pub fn new(dialect: &str) -> Self;
    pub fn parse(&self, sql: &str) -> Result<ImportResult>;
    pub fn parse_liquibase(&self, sql: &str) -> Result<ImportResult>;
}
```

**Enhancement**: The `dialect` parameter now accepts `"databricks"` as a valid value.

**Dialect Values**:
- `"postgres"` or `"postgresql"` - PostgreSQL dialect
- `"mysql"` - MySQL dialect
- `"sqlite"` - SQLite dialect
- `"generic"` - Generic SQL dialect (default)
- `"databricks"` - **NEW** Databricks SQL dialect with IDENTIFIER() and variable reference support

## Internal API (New)

### DatabricksDialect

**Module**: `src/import/sql.rs` (internal, not exported)

**Type**: Struct implementing `sqlparser::dialect::Dialect`

```rust
struct DatabricksDialect;

impl Dialect for DatabricksDialect {
    // Override trait methods for Databricks-specific behavior
}
```

**Responsibilities**:
- Recognize `:` as valid in identifiers (for variable references)
- Handle Databricks-specific identifier quoting
- Customize parser behavior for Databricks SQL patterns

## Function Contracts

### SQLImporter::new()

**Signature**: `pub fn new(dialect: &str) -> Self`

**Preconditions**:
- `dialect` is a non-empty string

**Postconditions**:
- Returns `SQLImporter` with specified dialect
- If `dialect == "databricks"`, Databricks-specific parsing will be enabled

**Side Effects**: None

**Errors**: None (invalid dialects default to "generic")

### SQLImporter::parse()

**Signature**: `pub fn parse(&self, sql: &str) -> Result<ImportResult>`

**Preconditions**:
- `sql` is a valid UTF-8 string
- If `self.dialect == "databricks"`, SQL may contain Databricks-specific syntax

**Postconditions**:
- Returns `ImportResult` containing:
  - Successfully parsed tables in `tables` field
  - Tables requiring name resolution in `tables_requiring_name` (if IDENTIFIER() contains only variables)
  - Parse errors in `errors` field (if parsing fails)
- Databricks-specific syntax patterns are handled:
  - `IDENTIFIER()` function calls are recognized and table names extracted
  - Variable references in type definitions are replaced with fallback types
  - Variable references in COMMENT/TBLPROPERTIES are handled gracefully

**Side Effects**: None

**Errors**:
- Returns `Ok(ImportResult)` with errors in `errors` field if parsing fails
- Error messages include context about Databricks-specific syntax when applicable

### Preprocessing Functions (Internal)

**Module**: `src/import/sql.rs` (internal, not exported)

**Functions**:
- `preprocess_databricks_sql(sql: &str) -> (String, PreprocessingState)` - Preprocesses SQL to handle Databricks syntax
- `extract_identifier_table_name(expr: &str) -> Option<String>` - Extracts table name from IDENTIFIER() expression
- `replace_variables_in_types(sql: &str) -> String` - Replaces variable references in type definitions

**Contracts**: Internal implementation details, not part of public API

## Error Contracts

### ImportError::ParseError

**When**: SQL parsing fails due to syntax errors

**Message Format**: `"Parse error: {details}"`

**Enhancements for Databricks**:
- When Databricks syntax is detected but generic dialect is used, error suggests using `dialect="databricks"`
- When IDENTIFIER() parsing fails, error includes the problematic expression
- When variable references cause parse errors, error indicates the location and suggests preprocessing

**Example Messages**:
- `"Parse error: Expected: column name or constraint definition, found: : at Line: 1, Column: 39. This appears to be Databricks SQL syntax. Try using dialect='databricks'."`
- `"Parse error: IDENTIFIER() expression contains only variables. Table name cannot be determined: IDENTIFIER(:catalog)"`

## Test Contracts

### Unit Tests

**Location**: `tests/import_tests.rs` or `src/import/sql.rs` (module tests)

**Test Cases**:
1. `test_databricks_identifier_with_literal()` - IDENTIFIER('table') parses correctly
2. `test_databricks_identifier_with_variable()` - IDENTIFIER(:var) creates placeholder table
3. `test_databricks_identifier_with_concatenation()` - IDENTIFIER(:var || '.schema.table') extracts table name
4. `test_databricks_variable_in_struct()` - STRUCT<field: :type> replaces variable with STRING
5. `test_databricks_variable_in_array()` - ARRAY<:type> replaces variable with STRING
6. `test_databricks_nested_variables()` - ARRAY<STRUCT<field: :type>> handles nested patterns
7. `test_databricks_comment_variable()` - COMMENT ':var' handles variable gracefully
8. `test_databricks_tblproperties_variable()` - TBLPROPERTIES ('key' = ':var') handles variable
9. `test_databricks_backward_compatibility()` - Existing dialects still work
10. `test_databricks_error_messages()` - Error messages include helpful context

### Integration Tests

**Location**: `tests/import_tests.rs`

**Test Cases**:
1. `test_databricks_full_example()` - Complete Databricks SQL DDL from GitHub issue #13
2. `test_databricks_mixed_sql()` - Databricks SQL mixed with standard SQL
3. `test_databricks_performance()` - Performance is within 10% of standard SQL parsing

## Backward Compatibility

**Guarantee**: All existing API contracts remain unchanged. Adding Databricks support does not modify behavior for existing dialects.

**Verification**:
- All existing tests for PostgreSQL, MySQL, SQLite, Generic dialects must pass
- No changes to `ImportResult`, `TableData`, `ColumnData` structures
- No changes to error types or error handling patterns

## Feature Flag

**Cargo Feature**: `databricks-dialect` (already defined in Cargo.toml)

**Behavior**:
- When feature is enabled: Databricks dialect is available
- When feature is disabled: Attempting to use `dialect="databricks"` may default to generic dialect or return an error (implementation detail)

**Note**: Feature flag may not be strictly necessary if Databricks support adds no new dependencies, but it's already defined in Cargo.toml and should be respected.
