# Quickstart: Enhanced Databricks SQL Syntax Support

**Feature**: Enhanced Databricks SQL Syntax Support
**Date**: 2026-01-04
**Phase**: Phase 1 - Design

## Overview

This guide demonstrates how to use the enhanced SQL import functionality to import Databricks SQL DDL statements containing `IDENTIFIER()` functions and variable references.

## Prerequisites

- Rust project with `data-modelling-sdk` dependency
- `databricks-dialect` feature enabled (if using feature flags)

## Basic Usage

### Importing Databricks SQL with IDENTIFIER() Function

```rust
use data_modelling_sdk::import::sql::SQLImporter;

// Create importer with Databricks dialect
let importer = SQLImporter::new("databricks");

// SQL with IDENTIFIER() function
let sql = r#"
CREATE TABLE IF NOT EXISTS IDENTIFIER(:catalog || '.schema.example_table') (
  id STRING COMMENT 'Unique identifier',
  name STRING COMMENT 'Name of the record'
)
USING DELTA;
"#;

// Parse the SQL
let result = importer.parse(sql)?;

// Check results
assert_eq!(result.tables.len(), 1);
let table = &result.tables[0];
assert_eq!(table.name.as_deref(), Some("schema.example_table"));
assert_eq!(table.columns.len(), 2);
```

### Handling Variable References in Type Definitions

```rust
use data_modelling_sdk::import::sql::SQLImporter;

let importer = SQLImporter::new("databricks");

// SQL with variable references in STRUCT types
let sql = r#"
CREATE TABLE example (
  id STRING,
  metadata STRUCT<
    key: STRING,
    value: :value_type,  -- Variable reference replaced with STRING
    timestamp: TIMESTAMP
  >,
  items ARRAY<:element_type>  -- Variable reference replaced with STRING
);
"#;

let result = importer.parse(sql)?;
let table = &result.tables[0];

// Variable references are replaced with STRING fallback type
assert_eq!(table.columns[1].data_type, "STRUCT<key: STRING, value: STRING, timestamp: TIMESTAMP>");
assert_eq!(table.columns[2].data_type, "ARRAY<STRING>");
```

### Handling Complex Nested Patterns

```rust
use data_modelling_sdk::import::sql::SQLImporter;

let importer = SQLImporter::new("databricks");

// SQL with nested STRUCT/ARRAY containing variables
let sql = r#"
CREATE TABLE example (
  rulesTriggered ARRAY<STRUCT<
    id: STRING,
    name: STRING,
    alertOperation: STRUCT<
      name: STRING,
      revert: :variable_type,  -- Variable in nested STRUCT
      timestamp: TIMESTAMP
    >
  >>
);
"#;

let result = importer.parse(sql)?;
let table = &result.tables[0];

// Nested variables are replaced recursively
let expected_type = "ARRAY<STRUCT<id: STRING, name: STRING, alertOperation: STRUCT<name: STRING, revert: STRING, timestamp: TIMESTAMP>>>";
assert_eq!(table.columns[0].data_type, expected_type);
```

### Handling IDENTIFIER() with Only Variables

```rust
use data_modelling_sdk::import::sql::SQLImporter;

let importer = SQLImporter::new("databricks");

// SQL with IDENTIFIER() containing only variables (no literals)
let sql = r#"
CREATE TABLE IDENTIFIER(:table_name) (
  id STRING
);
"#;

let result = importer.parse(sql)?;

// Table is added to tables_requiring_name
assert_eq!(result.tables_requiring_name.len(), 1);
assert_eq!(result.tables.len(), 1);

// Table has placeholder name
let table = &result.tables[0];
assert!(table.name.as_deref().unwrap().starts_with("__databricks_table_"));
```

### Error Handling

```rust
use data_modelling_sdk::import::sql::SQLImporter;

let importer = SQLImporter::new("databricks");

// SQL with syntax error
let sql = "CREATE TABLE example (id STRING,);";  // Trailing comma

let result = importer.parse(sql)?;

// Errors are captured in result.errors
if !result.errors.is_empty() {
    for error in &result.errors {
        eprintln!("Parse error: {}", error);
    }
}
```

## Common Patterns

### Pattern 1: Catalog-Schema-Table Pattern

```rust
// IDENTIFIER(:catalog || '.schema.table')
// Extracted as: "schema.table"
```

### Pattern 2: Simple Variable Reference

```rust
// IDENTIFIER(:table_name)
// Creates placeholder table name, added to tables_requiring_name
```

### Pattern 3: Multiple Concatenations

```rust
// IDENTIFIER(:catalog || '.schema.' || :table || '.suffix')
// Extracted as: "schema.{placeholder}.suffix" or full placeholder if all variables
```

### Pattern 4: Variable in STRUCT Field Type

```rust
// STRUCT<field: :variable_type>
// Replaced with: STRUCT<field: STRING>
```

### Pattern 5: Variable in ARRAY Element Type

```rust
// ARRAY<:element_type>
// Replaced with: ARRAY<STRING>
```

### Pattern 6: Variable in COMMENT

```rust
// COMMENT ':variable'
// Replaced with: COMMENT '[Databricks variable: :variable]'
```

### Pattern 7: Variable in TBLPROPERTIES

```rust
// TBLPROPERTIES ('key' = ':variable')
// Replaced with: TBLPROPERTIES ('key' = '[variable]')
```

## Migration from Frontend Preprocessing

If you're currently using frontend preprocessing (temporary workaround), you can migrate to native SDK support:

**Before** (frontend preprocessing):
```typescript
const preprocessedSQL = preprocessDatabricksSQL(sql);
const result = await sdk.importSQL(preprocessedSQL, "generic");
```

**After** (native SDK support):
```rust
let importer = SQLImporter::new("databricks");
let result = importer.parse(sql)?;
```

**Benefits**:
- No preprocessing step required
- More accurate parsing
- Better error messages
- Consistent behavior across platforms

## Best Practices

1. **Always specify dialect**: Use `SQLImporter::new("databricks")` when importing Databricks SQL
2. **Check tables_requiring_name**: Handle tables with placeholder names that need user input
3. **Review error messages**: Error messages include helpful context about Databricks syntax
4. **Validate results**: Verify that variable replacements (STRING fallback) are acceptable for your use case
5. **Test edge cases**: Test with your specific Databricks SQL patterns before production use

## Troubleshooting

### Issue: Parse errors with "Expected: >, found: :"

**Cause**: Variable reference in STRUCT/ARRAY type definition not handled

**Solution**: Ensure you're using `dialect="databricks"`. Variables are automatically replaced with STRING fallback.

### Issue: Table name is placeholder

**Cause**: IDENTIFIER() expression contains only variables (no string literals)

**Solution**: Check `result.tables_requiring_name` and provide table name manually, or modify SQL to include literal parts in IDENTIFIER().

### Issue: Performance degradation

**Cause**: Preprocessing overhead for complex SQL

**Solution**: Preprocessing is optimized, but very large SQL files (>10k lines) may see slight slowdown. Consider splitting into smaller imports.

## Examples from GitHub Issue #13

### Full Example with All Patterns

```rust
use data_modelling_sdk::import::sql::SQLImporter;

let importer = SQLImporter::new("databricks");

let sql = r#"
CREATE TABLE IF NOT EXISTS IDENTIFIER(:catalog_name || '.schema.example_table') (
  id STRING COMMENT 'Unique identifier for each record.',
  name STRING COMMENT 'Name of the record.',

  rulesTriggered ARRAY<STRUCT<
    id: STRING,
    name: STRING,
    priorityOrder: INT,
    group: STRING,
    sound: STRING,
    alertOperation: STRUCT<
      name: STRING,
      field: STRING,
      revert: :variable_type,
      timestamp: TIMESTAMP
    >
  >>,

  metadata STRUCT<
    key: STRING,
    value: :value_type,
    timestamp: TIMESTAMP,
    nested: STRUCT<
      field1: :nested_type,
      field2: STRING
    >
  >,

  items ARRAY<:element_type>,
  status :status_type STRING,
  created_at TIMESTAMP,
  updated_at TIMESTAMP
)
USING DELTA
COMMENT ':table_comment_variable'
TBLPROPERTIES (
  'key1' = ':variable_value',
  'key2' = 'static_value'
)
CLUSTER BY (id);
"#;

let result = importer.parse(sql)?;

// All patterns handled successfully
assert_eq!(result.tables.len(), 1);
assert!(result.errors.is_empty());
```

This example demonstrates all supported Databricks patterns working together in a single import.
