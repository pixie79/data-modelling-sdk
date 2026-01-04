# Research: Enhanced Databricks SQL Syntax Support

**Feature**: Enhanced Databricks SQL Syntax Support
**Date**: 2026-01-04
**Phase**: Phase 0 - Research

## Research Questions

### 1. How to extend sqlparser Dialect trait for Databricks-specific syntax?

**Decision**: Create a custom `DatabricksDialect` struct that implements the `Dialect` trait from sqlparser crate. The trait provides methods to customize identifier parsing, keyword recognition, and operator handling.

**Rationale**:
- sqlparser 0.53 provides a `Dialect` trait that allows customizing parser behavior
- Existing dialects (PostgreSqlDialect, MySqlDialect, SQLiteDialect) serve as reference implementations
- The trait methods allow overriding identifier parsing to recognize `IDENTIFIER()` function calls
- Can customize operator recognition to handle `||` concatenation in identifier contexts

**Alternatives considered**:
- **Preprocessing approach**: Strip/replace Databricks syntax before parsing (rejected - loses semantic information, harder to maintain)
- **Post-processing approach**: Parse with generic dialect then fix AST (rejected - too complex, error-prone)
- **Fork sqlparser**: Create custom fork with Databricks support (rejected - maintenance burden, version drift)

**Implementation approach**:
- Create `DatabricksDialect` struct in `src/import/sql.rs`
- Implement `Dialect` trait methods:
  - `is_identifier_start()` and `is_identifier_part()` - recognize `:` as valid in identifiers for variable references
  - `is_delimited_identifier_start()` - handle backtick-quoted identifiers
  - Override identifier parsing to recognize `IDENTIFIER(...)` as a special function call
- Use preprocessing for variable references in type definitions (STRUCT/ARRAY) since these cannot be parsed directly by sqlparser

### 2. How to handle IDENTIFIER() function calls with variable references?

**Decision**: Implement a two-phase approach:
1. **Preprocessing**: Replace `IDENTIFIER(:variable || '.schema.table')` patterns with placeholder table names before parsing
2. **Post-processing**: Extract actual table name from original SQL if literal parts exist, otherwise use placeholder

**Rationale**:
- sqlparser cannot parse `IDENTIFIER()` function calls directly in table name position
- Variable references (`:variable`) are not standard SQL and cannot be parsed as identifiers
- String concatenation (`||`) in identifier context is Databricks-specific
- Preprocessing allows parsing to succeed while preserving information for later extraction

**Alternatives considered**:
- **Extend sqlparser AST**: Add custom AST nodes for IDENTIFIER() (rejected - requires forking sqlparser)
- **Custom parser**: Write custom parser for Databricks SQL (rejected - too complex, maintenance burden)
- **Regex-based extraction**: Extract table names with regex before parsing (rejected - fragile, doesn't handle all cases)

**Implementation approach**:
- Preprocess SQL to replace `IDENTIFIER(expression)` with placeholder table name like `__databricks_table_0__`
- Use regex to extract the expression from IDENTIFIER() for later processing
- If expression contains string literals, extract and construct table name
- If expression contains only variables, use placeholder and mark table as requiring name resolution

### 3. How to handle variable references in STRUCT/ARRAY type definitions?

**Decision**: Preprocess variable references in type definitions by replacing them with fallback types (e.g., `STRING` for type variables) before parsing.

**Rationale**:
- sqlparser expects valid SQL types in STRUCT/ARRAY definitions
- Variable references like `:variable_type` are not valid SQL types
- Replacing with fallback types allows parsing to succeed
- The fallback type choice (STRING) is reasonable for most use cases

**Alternatives considered**:
- **Custom type system**: Extend sqlparser to support variable types (rejected - too invasive)
- **Skip parsing**: Mark columns with variable types as "unknown" (rejected - loses too much information)
- **User-provided type mapping**: Require users to provide variable substitutions (rejected - adds complexity, not required per spec)

**Implementation approach**:
- Preprocess SQL to replace `:variable_type` in STRUCT field types with `STRING`
- Replace `:variable_type` in ARRAY element types with `STRING`
- Handle nested patterns recursively (e.g., `ARRAY<STRUCT<field: :type>>`)
- Preserve original SQL in metadata for reference if needed

### 4. How to handle variable references in COMMENT and TBLPROPERTIES?

**Decision**: Preprocess variable references in COMMENT and TBLPROPERTIES by replacing them with placeholder values or removing them.

**Rationale**:
- COMMENT and TBLPROPERTIES are metadata, not structural
- Variable references here don't block parsing but should be handled gracefully
- Replacing with placeholders preserves the structure while indicating variables were present

**Alternatives considered**:
- **Leave as-is**: Don't preprocess (rejected - variables in strings may cause parsing issues)
- **Remove entirely**: Strip COMMENT/TBLPROPERTIES with variables (rejected - loses metadata)
- **Require substitution**: Force users to provide variable values (rejected - adds complexity)

**Implementation approach**:
- Replace `:variable` in COMMENT clauses with placeholder text like `"[Databricks variable: :variable]"`
- Replace `:variable` in TBLPROPERTIES values with placeholder string `"[variable]"`
- Preserve keys and structure, only replace variable values

### 5. What fallback types to use for variable references?

**Decision**: Use `STRING` as the default fallback type for all variable references in type contexts.

**Rationale**:
- STRING is the most general type in Databricks SQL
- Most variable references in real-world schemas resolve to STRING or compatible types
- Using a single fallback type simplifies implementation
- Users can manually correct types after import if needed

**Alternatives considered**:
- **VARIANT/ANY**: Use a generic variant type (rejected - not standard SQL, may not be supported)
- **User-configurable**: Allow users to specify fallback types (rejected - adds complexity, not required)
- **Context-aware**: Infer type from context (rejected - too complex, error-prone)

**Implementation approach**:
- Replace all `:variable_type` with `STRING` in type definitions
- Document this behavior in error messages and user-facing documentation
- Consider adding a note in imported table metadata indicating variables were replaced

### 6. How to maintain backward compatibility?

**Decision**: Make Databricks dialect opt-in via dialect string parameter. Existing dialects (postgres, mysql, sqlite, generic) continue to work unchanged.

**Rationale**:
- Users must explicitly specify "databricks" dialect to enable Databricks-specific parsing
- Existing SQL imports continue to use their respective dialects
- No changes to default behavior
- Preprocessing only applies when Databricks dialect is selected

**Alternatives considered**:
- **Auto-detect**: Automatically detect Databricks SQL (rejected - too error-prone, may misclassify)
- **Always enable**: Enable Databricks support for all dialects (rejected - may break existing imports)

**Implementation approach**:
- Add "databricks" case to `SQLImporter::dialect_impl()` method
- Return `Box<DatabricksDialect>` when dialect is "databricks"
- Keep all existing dialect handling unchanged

## Technical Constraints

1. **sqlparser limitations**: Cannot parse `IDENTIFIER()` function calls or variable references directly
2. **AST limitations**: sqlparser AST doesn't support custom nodes for Databricks-specific syntax
3. **Backward compatibility**: Must not break existing SQL imports
4. **Performance**: Preprocessing should not significantly impact parse performance

## Dependencies

- **sqlparser 0.53**: Already in use, provides Dialect trait for extension
- **regex 1.0**: Already in use, needed for preprocessing patterns
- **No new dependencies**: All required crates already available

## Implementation Strategy

1. **Phase 1**: Create `DatabricksDialect` struct implementing `Dialect` trait
2. **Phase 2**: Implement preprocessing functions for:
   - IDENTIFIER() function calls
   - Variable references in type definitions
   - Variable references in COMMENT/TBLPROPERTIES
3. **Phase 3**: Integrate DatabricksDialect into SQLImporter
4. **Phase 4**: Add comprehensive test cases covering all supported patterns
5. **Phase 5**: Update documentation and error messages

## Open Questions Resolved

All technical questions have been resolved. No remaining `NEEDS CLARIFICATION` markers.
