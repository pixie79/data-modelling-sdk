# Feature Specification: Enhanced Databricks SQL Syntax Support

**Feature Branch**: `005-databricks-sql-support`
**Created**: 2026-01-04
**Status**: Draft
**Input**: User description: "Enhanced Databricks SQL Syntax Support"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Import Databricks SQL with IDENTIFIER() Function (Priority: P1)

A data engineer working with Databricks needs to import SQL DDL statements that use the `IDENTIFIER()` function for dynamic table name construction. Currently, these imports fail with parse errors, preventing them from using the data modeling tool with their existing Databricks SQL schemas.

**Why this priority**: This is the most common Databricks pattern and is blocking users from importing real-world Databricks SQL schemas. Without this, users cannot use the tool with Databricks at all.

**Independent Test**: Can be fully tested by importing a CREATE TABLE statement containing `IDENTIFIER(:catalog || '.schema.table')` and verifying that the table name is correctly extracted and the import succeeds without parse errors.

**Acceptance Scenarios**:

1. **Given** a user has a Databricks SQL DDL with `CREATE TABLE IDENTIFIER(:catalog || '.schema.table')`, **When** they import the SQL, **Then** the parser recognizes the IDENTIFIER() function, extracts the table name, and successfully imports the table definition
2. **Given** a user has a Databricks SQL DDL with `CREATE TABLE IDENTIFIER(:variable)`, **When** they import the SQL, **Then** the parser handles the variable reference and imports the table with a placeholder or resolved name
3. **Given** a user has a Databricks SQL DDL with `CREATE TABLE IDENTIFIER('literal_table_name')`, **When** they import the SQL, **Then** the parser extracts the literal table name and imports successfully
4. **Given** a user has a Databricks SQL DDL with complex concatenation like `IDENTIFIER(:var || '.schema.table' || '.suffix')`, **When** they import the SQL, **Then** the parser handles the concatenation expression and extracts the table name

---

### User Story 2 - Import Databricks SQL with Variable References in Type Definitions (Priority: P1)

A data engineer needs to import Databricks SQL that contains variable references within STRUCT and ARRAY type definitions. These patterns are common in Databricks schemas but currently cause parse errors at specific positions (e.g., Line 5, Column 7).

**Why this priority**: This is equally critical as Story 1 because many real-world Databricks schemas use variable references in type definitions. The parser fails with "Expected: >, found: :" errors, completely blocking imports.

**Independent Test**: Can be fully tested by importing a CREATE TABLE statement containing `STRUCT<field: :variable_type>` or `ARRAY<:element_type>` and verifying that the parser handles the variable reference gracefully, replacing it with an appropriate fallback type or placeholder.

**Acceptance Scenarios**:

1. **Given** a user has a Databricks SQL DDL with `STRUCT<key: :variable_type, value: STRING>`, **When** they import the SQL, **Then** the parser recognizes the variable reference in the STRUCT field type and handles it without parse errors
2. **Given** a user has a Databricks SQL DDL with `ARRAY<:element_type>`, **When** they import the SQL, **Then** the parser recognizes the variable reference in the ARRAY element type and handles it without parse errors
3. **Given** a user has a Databricks SQL DDL with nested patterns like `ARRAY<STRUCT<field: :nested_type>>`, **When** they import the SQL, **Then** the parser handles the nested variable reference without parse errors
4. **Given** a user has a Databricks SQL DDL with variable references in deeply nested STRUCT definitions, **When** they import the SQL, **Then** the parser handles all nested variable references correctly

---

### User Story 3 - Import Databricks SQL with Variable References in Metadata Clauses (Priority: P2)

A data engineer needs to import Databricks SQL that contains variable references in COMMENT clauses and TBLPROPERTIES. While these don't cause parse failures, they should be handled gracefully to preserve the intent of the original SQL.

**Why this priority**: Lower priority than Stories 1 and 2 because these don't cause parse failures, but they are common in Databricks SQL and should be handled for completeness.

**Independent Test**: Can be fully tested by importing a CREATE TABLE statement containing `COMMENT ':variable'` or `TBLPROPERTIES ('key' = ':variable')` and verifying that the parser handles the variable references appropriately.

**Acceptance Scenarios**:

1. **Given** a user has a Databricks SQL DDL with `COMMENT ':comment_variable'`, **When** they import the SQL, **Then** the parser handles the variable reference in the COMMENT clause without errors
2. **Given** a user has a Databricks SQL DDL with `TBLPROPERTIES ('key' = ':variable_value')`, **When** they import the SQL, **Then** the parser handles the variable reference in TBLPROPERTIES without errors
3. **Given** a user has a Databricks SQL DDL with multiple variable references in TBLPROPERTIES, **When** they import the SQL, **Then** the parser handles all variable references correctly

---

### User Story 4 - Import Databricks SQL with Variable References in Column Definitions (Priority: P2)

A data engineer needs to import Databricks SQL that contains variable references in column type definitions (e.g., `column_name :variable TYPE`). These patterns should be handled to allow successful imports.

**Why this priority**: Lower priority than Stories 1 and 2, but still important for completeness and handling edge cases in real-world Databricks SQL.

**Independent Test**: Can be fully tested by importing a CREATE TABLE statement containing a column definition with a variable reference and verifying that the parser handles it appropriately.

**Acceptance Scenarios**:

1. **Given** a user has a Databricks SQL DDL with `column_name :variable STRING`, **When** they import the SQL, **Then** the parser removes or handles the variable reference while preserving the column name and type
2. **Given** a user has a Databricks SQL DDL with multiple columns containing variable references, **When** they import the SQL, **Then** the parser handles all variable references correctly

---

### User Story 5 - Import Databricks SQL with Views and Materialized Views (Priority: P2)

A data engineer needs to import Databricks SQL that contains CREATE VIEW and CREATE MATERIALIZED VIEW statements. Currently, only CREATE TABLE statements are supported, but views are an important part of Databricks schemas and should be imported to provide a complete picture of the data model.

**Why this priority**: Views and materialized views are commonly used in Databricks for data abstraction, security, and performance optimization. Supporting them completes the SQL import functionality and allows users to import complete Databricks schemas including both tables and views.

**Independent Test**: Can be fully tested by importing CREATE VIEW and CREATE MATERIALIZED VIEW statements and verifying that they are parsed and imported correctly, with their definitions preserved.

**Acceptance Scenarios**:

1. **Given** a user has a Databricks SQL DDL with `CREATE VIEW view_name AS SELECT ...`, **When** they import the SQL, **Then** the parser recognizes the VIEW statement and imports it as a table-like entity with the view definition preserved
2. **Given** a user has a Databricks SQL DDL with `CREATE MATERIALIZED VIEW view_name AS SELECT ...`, **When** they import the SQL, **Then** the parser recognizes the MATERIALIZED VIEW statement and imports it with appropriate metadata indicating it's a materialized view
3. **Given** a user has a Databricks SQL DDL with views using IDENTIFIER() function calls, **When** they import the SQL, **Then** the parser handles IDENTIFIER() in view names using the same preprocessing logic as tables
4. **Given** a user has a Databricks SQL DDL with views containing variable references in type definitions, **When** they import the SQL, **Then** the parser handles variable references using the same preprocessing logic as tables
5. **Given** a user has a Databricks SQL DDL with both tables and views, **When** they import the SQL, **Then** the parser imports both types of entities correctly

---

### Edge Cases

- What happens when IDENTIFIER() contains only a variable with no concatenation or literal parts?
- What happens when variable references appear in deeply nested STRUCT definitions (3+ levels)?
- What happens when string concatenation in IDENTIFIER() contains multiple variables?
- How does the system handle malformed IDENTIFIER() expressions (missing parentheses, invalid syntax)?
- What happens when variable references appear in unsupported contexts (e.g., constraint definitions)?
- How does the system handle Databricks SQL mixed with standard SQL in the same import?
- What happens when the same SQL is imported multiple times with different variable values?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST recognize and parse `IDENTIFIER()` function calls in table name positions within CREATE TABLE statements
- **FR-002**: System MUST extract table names from `IDENTIFIER()` expressions that contain string literals
- **FR-003**: System MUST handle variable references (`:variable_name`) in `IDENTIFIER()` function expressions
- **FR-004**: System MUST support string concatenation (`||` operator) within `IDENTIFIER()` expressions
- **FR-005**: System MUST handle variable references in STRUCT field type definitions (e.g., `STRUCT<field: :variable_type>`)
- **FR-006**: System MUST handle variable references in ARRAY element type definitions (e.g., `ARRAY<:element_type>`)
- **FR-007**: System MUST handle variable references in nested type patterns (e.g., `ARRAY<STRUCT<field: :nested_type>>`)
- **FR-008**: System MUST handle variable references in COMMENT clauses
- **FR-009**: System MUST handle variable references in TBLPROPERTIES clauses
- **FR-010**: System MUST handle variable references in column type definitions
- **FR-011**: System MUST recognize and parse CREATE VIEW statements in Databricks SQL
- **FR-012**: System MUST recognize and parse CREATE MATERIALIZED VIEW statements in Databricks SQL
- **FR-013**: System MUST support IDENTIFIER() function calls in VIEW and MATERIALIZED VIEW names
- **FR-014**: System MUST support variable references in VIEW and MATERIALIZED VIEW column definitions
- **FR-015**: System MUST provide clear error messages when parsing fails due to unsupported Databricks syntax patterns
- **FR-016**: System MUST maintain backward compatibility with existing SQL dialects (PostgreSQL, MySQL, SQLite, Generic)
- **FR-017**: System MUST allow users to specify Databricks as a SQL dialect option for imports
- **FR-018**: System MUST handle variable references gracefully by either substituting with fallback types or preserving them as placeholders

### Key Entities *(include if feature involves data)*

- **SQL Import Result**: Represents the outcome of importing SQL, containing successfully parsed tables, tables requiring additional information, and any parse errors encountered
- **Table Definition**: Represents a parsed table structure with name, columns, and metadata extracted from CREATE TABLE statements
- **Variable Reference**: Represents a placeholder (`:variable_name`) found in SQL that may need resolution or substitution
- **Identifier Expression**: Represents an `IDENTIFIER()` function call that may contain literals, variables, or concatenation expressions

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can successfully import Databricks SQL DDL statements containing `IDENTIFIER()` functions with string concatenation without parse errors (target: 100% success rate for supported patterns)
- **SC-002**: Users can successfully import Databricks SQL DDL statements containing variable references in STRUCT and ARRAY type definitions without parse errors (target: 100% success rate for supported patterns)
- **SC-003**: Users can successfully import complex nested Databricks SQL patterns (ARRAY<STRUCT<:variable>>) without parse errors (target: 100% success rate for supported patterns)
- **SC-004**: Import success rate for Databricks SQL schemas improves from 0% (currently blocked) to at least 95% for real-world Databricks SQL patterns
- **SC-005**: Error messages clearly indicate when Databricks-specific syntax is encountered but not supported, helping users understand what needs to be adjusted (target: 100% of parse errors include context about the unsupported pattern)
- **SC-006**: Existing SQL imports (PostgreSQL, MySQL, SQLite, Generic) continue to work without regression (target: 100% backward compatibility maintained)
- **SC-007**: Users can complete a Databricks SQL import in the same time as standard SQL imports (target: no more than 10% performance degradation compared to standard SQL parsing)
- **SC-008**: Users can successfully import CREATE VIEW and CREATE MATERIALIZED VIEW statements from Databricks SQL without parse errors (target: 100% success rate for supported view patterns)

## Assumptions

- Variable references in Databricks SQL can be handled by replacing them with appropriate fallback types (e.g., `STRING` for type variables) or preserving them as placeholders, as actual variable substitution is not required for the import use case
- Users will specify "databricks" as the dialect when importing Databricks SQL, making Databricks-specific parsing opt-in
- The primary use case is importing table schemas for data modeling purposes, not executing SQL, so variable resolution is not required
- Complex nested STRUCT and ARRAY patterns with variables are common in real-world Databricks schemas and must be supported
- String concatenation in IDENTIFIER() expressions typically follows patterns like `:variable || '.schema.table'` or `'catalog.' || :schema || '.table'`
- Error messages should guide users but don't need to provide automatic fixes or suggestions

## Dependencies

- SQL parser library must support extensible dialect definitions
- Existing SQL import infrastructure must support dialect-specific parsing logic
- Test suite must include comprehensive Databricks SQL examples covering all supported patterns

## Out of Scope

- Actual variable substitution/resolution (variables are handled as placeholders or replaced with fallback types)
- Support for other Databricks-specific SQL features beyond IDENTIFIER() and variable references
- Execution or validation of imported SQL against a Databricks instance
- Support for Databricks-specific data types beyond what's needed for parsing
- Performance optimization beyond maintaining reasonable import times
- Frontend preprocessing workaround removal (this will be handled separately after SDK implementation)
