# Data Model: Enhanced Databricks SQL Syntax Support

**Feature**: Enhanced Databricks SQL Syntax Support
**Date**: 2026-01-04
**Phase**: Phase 1 - Design

## Overview

This feature extends the existing SQL import data model to support Databricks-specific syntax patterns. No new entities are introduced - the feature enhances the parsing logic for existing `ImportResult`, `TableData`, and `ColumnData` entities.

## Existing Entities (Enhanced)

### ImportResult

**Purpose**: Represents the outcome of importing SQL, containing successfully parsed tables and any parse errors.

**Fields** (unchanged):
- `tables: Vec<TableData>` - Tables extracted from the import
- `tables_requiring_name: Vec<TableRequiringName>` - Tables that require name input (for SQL imports with unnamed tables)
- `errors: Vec<ImportError>` - Parse errors/warnings
- `ai_suggestions: Option<Vec<serde_json::Value>>` - Whether AI suggestions are available

**Enhancements**:
- When Databricks SQL contains `IDENTIFIER(:variable)` with only variables (no literals), tables may be added to `tables_requiring_name` with placeholder names
- Error messages in `errors` will include context about Databricks-specific syntax when parsing fails

### TableData

**Purpose**: Represents a parsed table structure with name, columns, and metadata extracted from CREATE TABLE statements.

**Fields** (unchanged):
- `table_index: usize` - Index of table in import
- `name: Option<String>` - Table name (may be None if extracted from IDENTIFIER() with only variables)
- `columns: Vec<ColumnData>` - Column definitions

**Enhancements**:
- Table names extracted from `IDENTIFIER()` expressions will be normalized (e.g., `IDENTIFIER(:catalog || '.schema.table')` → `schema.table`)
- If IDENTIFIER() contains only variables, table name will be a placeholder like `__databricks_table_0__` and table added to `tables_requiring_name`

### ColumnData

**Purpose**: Represents a column definition extracted from SQL.

**Fields** (unchanged):
- `name: String` - Column name
- `data_type: String` - Column data type
- `nullable: bool` - Whether column allows NULL
- `primary_key: bool` - Whether column is part of primary key
- `description: Option<String>` - Column description/documentation
- `quality: Option<Vec<HashMap<String, serde_json::Value>>>` - Quality rules
- `ref_path: Option<String>` - JSON Schema $ref reference path

**Enhancements**:
- When variable references in type definitions are replaced (e.g., `STRUCT<field: :variable_type>` → `STRUCT<field: STRING>`), the `data_type` field will contain the fallback type
- Original variable references are not preserved (per spec assumption that variable substitution is not required)

## New Internal Entities

### DatabricksDialect

**Purpose**: Custom SQL dialect implementation for Databricks-specific syntax.

**Type**: Struct implementing `sqlparser::dialect::Dialect` trait

**Responsibilities**:
- Recognize `:` as valid in identifiers (for variable references)
- Handle backtick-quoted identifiers
- Customize identifier parsing behavior for Databricks SQL

**Location**: `src/import/sql.rs` (internal to module)

### PreprocessingState

**Purpose**: Tracks preprocessing transformations applied to SQL for later reference.

**Type**: Internal struct (not exposed in public API)

**Fields**:
- `identifier_replacements: Vec<(String, String)>` - Maps placeholder table names to original IDENTIFIER() expressions
- `variable_replacements: Vec<(String, String)>` - Tracks variable references replaced in type definitions

**Location**: `src/import/sql.rs` (internal to module)

## Relationships

```
SQLImporter
  ├── Uses DatabricksDialect (when dialect="databricks")
  ├── Produces ImportResult
  │     ├── Contains Vec<TableData>
  │     │     └── Contains Vec<ColumnData>
  │     └── Contains Vec<ImportError>
  └── Uses PreprocessingState (internal, during parsing)
```

## Validation Rules

### Table Name Validation

- Table names extracted from `IDENTIFIER()` expressions must pass existing `validate_table_name()` checks
- Placeholder table names (for variables-only IDENTIFIER()) are exempt from validation but marked in `tables_requiring_name`

### Column Type Validation

- Data types with variable references replaced (e.g., `STRING` fallback) must pass existing `validate_data_type()` checks
- Nested STRUCT/ARRAY types with replaced variables must be valid SQL type syntax

### Error Handling

- Parse errors must include context about which Databricks pattern caused the failure
- Errors should suggest using "databricks" dialect if Databricks syntax is detected but generic dialect is used

## State Transitions

### SQL Import Flow

```
1. User provides SQL + dialect="databricks"
   ↓
2. SQLImporter::parse() called
   ↓
3. Preprocessing applied (if Databricks dialect):
   - Replace IDENTIFIER() expressions
   - Replace variable references in types
   - Replace variables in COMMENT/TBLPROPERTIES
   ↓
4. sqlparser::Parser::parse_sql() called with DatabricksDialect
   ↓
5. AST parsed into Statement::CreateTable nodes
   ↓
6. parse_create_table() extracts TableData
   ↓
7. ImportResult returned with tables and errors
```

## Notes

- No database schema changes required (parsing-only feature)
- No new public API types introduced
- All enhancements are internal to SQL import logic
- Backward compatibility maintained - existing imports unchanged
