# Tasks: Enhanced Databricks SQL Syntax Support

**Input**: Design documents from `/specs/005-databricks-sql-support/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are included per Constitution requirements (all features MUST include appropriate test coverage).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- Paths shown below assume single project structure per plan.md

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Verify Rust project structure and existing SQL import module in src/import/sql.rs
- [X] T002 [P] Verify sqlparser 0.53 and regex 1.0 dependencies are available in Cargo.toml
- [X] T003 [P] Verify databricks-dialect feature flag exists in Cargo.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 Create DatabricksDialect struct in src/import/sql.rs implementing sqlparser::dialect::Dialect trait
- [X] T005 [P] Implement is_identifier_start() and is_identifier_part() methods in DatabricksDialect to recognize ':' as valid in identifiers
- [X] T006 [P] Implement is_delimited_identifier_start() method in DatabricksDialect for backtick-quoted identifiers
- [X] T007 Create PreprocessingState struct in src/import/sql.rs to track preprocessing transformations
- [X] T008 Add "databricks" case to SQLImporter::dialect_impl() method in src/import/sql.rs to return Box<DatabricksDialect>

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Import Databricks SQL with IDENTIFIER() Function (Priority: P1) 🎯 MVP

**Goal**: Enable parsing of Databricks SQL DDL statements containing IDENTIFIER() function calls for dynamic table name construction. Users can import SQL with patterns like `CREATE TABLE IDENTIFIER(:catalog || '.schema.table')` and the parser will extract table names correctly.

**Independent Test**: Import a CREATE TABLE statement containing `IDENTIFIER(:catalog || '.schema.table')` and verify that the table name is correctly extracted and the import succeeds without parse errors.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T009 [P] [US1] Add unit test test_databricks_identifier_with_literal() in src/import/sql.rs for IDENTIFIER('table') pattern
- [X] T010 [P] [US1] Add unit test test_databricks_identifier_with_variable() in src/import/sql.rs for IDENTIFIER(:var) pattern
- [X] T011 [P] [US1] Add unit test test_databricks_identifier_with_concatenation() in src/import/sql.rs for IDENTIFIER(:var || '.schema.table') pattern
- [X] T012 [P] [US1] Add integration test test_databricks_identifier_basic() in tests/import_tests.rs for basic IDENTIFIER() import scenarios

### Implementation for User Story 1

- [X] T013 [US1] Implement preprocess_identifier_expressions() function in src/import/sql.rs to replace IDENTIFIER() calls with placeholder table names
- [X] T014 [US1] Implement extract_identifier_table_name() function in src/import/sql.rs to extract table names from IDENTIFIER() expressions containing string literals
- [X] T015 [US1] Implement handle_identifier_variables() function in src/import/sql.rs to handle IDENTIFIER() expressions containing only variables (create placeholder names)
- [X] T016 [US1] Integrate IDENTIFIER() preprocessing into SQLImporter::parse() method in src/import/sql.rs when dialect is "databricks"
- [X] T017 [US1] Update parse_create_table() method in src/import/sql.rs to handle placeholder table names and extract actual names from PreprocessingState
- [X] T018 [US1] Add logic to populate tables_requiring_name field in ImportResult when IDENTIFIER() contains only variables

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Users can import Databricks SQL with IDENTIFIER() functions.

---

## Phase 4: User Story 2 - Import Databricks SQL with Variable References in Type Definitions (Priority: P1)

**Goal**: Enable parsing of Databricks SQL containing variable references within STRUCT and ARRAY type definitions. Users can import SQL with patterns like `STRUCT<field: :variable_type>` or `ARRAY<:element_type>` without parse errors.

**Independent Test**: Import a CREATE TABLE statement containing `STRUCT<field: :variable_type>` or `ARRAY<:element_type>` and verify that the parser handles the variable reference gracefully, replacing it with STRING fallback type.

### Tests for User Story 2

- [ ] T019 [P] [US2] Add unit test test_databricks_variable_in_struct() in src/import/sql.rs for STRUCT<field: :type> pattern
- [ ] T020 [P] [US2] Add unit test test_databricks_variable_in_array() in src/import/sql.rs for ARRAY<:type> pattern
- [ ] T021 [P] [US2] Add unit test test_databricks_nested_variables() in src/import/sql.rs for ARRAY<STRUCT<field: :type>> nested patterns
- [ ] T022 [P] [US2] Add integration test test_databricks_variables_in_types() in tests/import_tests.rs for variable references in type definitions

### Implementation for User Story 2

- [ ] T023 [US2] Implement replace_variables_in_struct_types() function in src/import/sql.rs to replace :variable_type in STRUCT field types with STRING
- [ ] T024 [US2] Implement replace_variables_in_array_types() function in src/import/sql.rs to replace :variable_type in ARRAY element types with STRING
- [ ] T025 [US2] Implement replace_nested_variables() function in src/import/sql.rs to handle nested patterns like ARRAY<STRUCT<field: :type>> recursively
- [ ] T026 [US2] Integrate variable replacement preprocessing into preprocess_databricks_sql() function in src/import/sql.rs
- [ ] T027 [US2] Add validation to ensure replaced types pass validate_data_type() checks in src/import/sql.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Users can import Databricks SQL with IDENTIFIER() functions and variable references in type definitions.

---

## Phase 5: User Story 3 - Import Databricks SQL with Variable References in Metadata Clauses (Priority: P2)

**Goal**: Enable graceful handling of variable references in COMMENT clauses and TBLPROPERTIES. Users can import SQL with patterns like `COMMENT ':variable'` or `TBLPROPERTIES ('key' = ':variable')` without errors.

**Independent Test**: Import a CREATE TABLE statement containing `COMMENT ':variable'` or `TBLPROPERTIES ('key' = ':variable')` and verify that the parser handles the variable references appropriately.

### Tests for User Story 3

- [X] T028 [P] [US3] Add unit test test_databricks_comment_variable() in src/import/sql.rs for COMMENT ':var' pattern
- [X] T029 [P] [US3] Add unit test test_databricks_tblproperties_variable() in src/import/sql.rs for TBLPROPERTIES ('key' = ':var') pattern
- [X] T030 [P] [US3] Add integration test test_databricks_metadata_variables() in tests/import_tests.rs for variable references in metadata clauses

### Implementation for User Story 3

- [X] T031 [US3] Implement replace_variables_in_comment() function in src/import/sql.rs to replace :variable in COMMENT clauses with placeholder text
- [X] T032 [US3] Implement replace_variables_in_tblproperties() function in src/import/sql.rs to replace :variable in TBLPROPERTIES values with placeholder string
- [X] T033 [US3] Integrate metadata variable replacement into preprocess_databricks_sql() function in src/import/sql.rs
- [X] T034 [US3] Ensure COMMENT and TBLPROPERTIES structure is preserved after variable replacement

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently. Users can import Databricks SQL with IDENTIFIER() functions, variable references in types, and variable references in metadata.

---

## Phase 6: User Story 4 - Import Databricks SQL with Variable References in Column Definitions (Priority: P2)

**Goal**: Enable handling of variable references in column type definitions. Users can import SQL with patterns like `column_name :variable STRING` without errors.

**Independent Test**: Import a CREATE TABLE statement containing a column definition with a variable reference and verify that the parser handles it appropriately.

### Tests for User Story 4

- [X] T035 [P] [US4] Add unit test test_databricks_column_variable() in src/import/sql.rs for column_name :variable TYPE pattern
- [X] T036 [P] [US4] Add integration test test_databricks_column_variables() in tests/import_tests.rs for multiple columns with variable references

### Implementation for User Story 4

- [X] T037 [US4] Implement replace_variables_in_column_definitions() function in src/import/sql.rs to remove or handle variable references in column type definitions
- [X] T038 [US4] Integrate column variable replacement into preprocess_databricks_sql() function in src/import/sql.rs
- [X] T039 [US4] Ensure column names and types are preserved correctly after variable removal

**Checkpoint**: All user stories should now be independently functional. Users can import Databricks SQL with all supported patterns.

---

## Phase 6.5: User Story 5 - Import Databricks SQL with Views and Materialized Views (Priority: P2)

**Purpose**: Support CREATE VIEW and CREATE MATERIALIZED VIEW statements in Databricks SQL imports

### Tests for User Story 5

- [X] T053 [P] [US5] Add unit test test_databricks_create_view() in src/import/sql.rs for CREATE VIEW statements
- [X] T054 [P] [US5] Add unit test test_databricks_create_materialized_view() in src/import/sql.rs for CREATE MATERIALIZED VIEW statements
- [X] T055 [P] [US5] Add unit test test_databricks_view_with_identifier() in src/import/sql.rs for CREATE VIEW IDENTIFIER(...) pattern
- [X] T056 [P] [US5] Add integration test test_databricks_views_and_tables() in tests/import_tests.rs for mixed tables and views

### Implementation for User Story 5

- [X] T057 [US5] Extend SQLImporter::parse() method in src/import/sql.rs to handle Statement::CreateView AST nodes
- [X] T058 [US5] Implement parse_create_view() method in src/import/sql.rs to extract view name and definition
- [X] T059 [US5] Extend SQLImporter::parse() method to handle CREATE MATERIALIZED VIEW statements (preprocessed to CREATE VIEW)
- [X] T060 [US5] Implement preprocess_materialized_views() function to convert MATERIALIZED VIEW to CREATE VIEW for sqlparser compatibility
- [X] T061 [US5] Ensure IDENTIFIER() preprocessing works for VIEW and MATERIALIZED VIEW names
- [X] T062 [US5] Ensure variable reference preprocessing works for VIEW and MATERIALIZED VIEW column definitions (views use same preprocessing pipeline)
- [X] T063 [US5] Views are imported as TableData entities (no separate ViewData needed - views are table-like for data modeling purposes)

**Checkpoint**: Views and materialized views can now be imported alongside tables, providing complete schema coverage.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T040 [P] Implement enhanced error messages in src/import/sql.rs that suggest using dialect="databricks" when Databricks syntax is detected but generic dialect is used (Note: Error detection can be enhanced in future)
- [X] T041 [P] Add error context to ImportError::ParseError messages in src/import/sql.rs indicating which Databricks pattern caused the failure (Note: Basic error messages implemented)
- [X] T042 [P] Add integration test test_databricks_backward_compatibility() in tests/import_tests.rs to verify existing dialects (PostgreSQL, MySQL, SQLite, Generic) still work
- [X] T043 [P] Add integration test test_databricks_full_example() in tests/import_tests.rs using complete Databricks SQL DDL from GitHub issue #13
- [X] T044 [P] Add integration test test_databricks_mixed_sql() in tests/import_tests.rs for Databricks SQL mixed with standard SQL
- [ ] T045 [P] Add performance test test_databricks_performance() in tests/import_tests.rs to verify parsing is within 10% of standard SQL performance (Optional - can be added later)
- [X] T046 [P] Update SQLImporter documentation in src/import/sql.rs to document "databricks" dialect option
- [X] T047 [P] Add doctest examples in src/import/sql.rs for Databricks SQL import usage
- [X] T064 [P] Run cargo fmt --all -- --check to ensure code formatting compliance
- [X] T065 [P] Run cargo clippy --all-targets --all-features -- -D warnings to ensure linting compliance
- [ ] T066 [P] Run cargo audit to verify security compliance (Optional - requires cargo-audit)
- [X] T067 [P] Run cargo test --all-features to verify all tests pass
- [ ] T068 [P] Validate quickstart.md examples work correctly (Manual validation - examples are documented)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories (can run in parallel with US1)
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories (can run in parallel with US1/US2)
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories (can run in parallel with US1/US2/US3)

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Preprocessing functions before integration
- Integration before validation
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Preprocessing functions within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Add unit test test_databricks_identifier_with_literal() in src/import/sql.rs"
Task: "Add unit test test_databricks_identifier_with_variable() in src/import/sql.rs"
Task: "Add unit test test_databricks_identifier_with_concatenation() in src/import/sql.rs"
Task: "Add integration test test_databricks_identifier_basic() in tests/import_tests.rs"

# After tests are written, launch preprocessing functions in parallel:
Task: "Implement preprocess_identifier_expressions() function in src/import/sql.rs"
Task: "Implement extract_identifier_table_name() function in src/import/sql.rs"
Task: "Implement handle_identifier_variables() function in src/import/sql.rs"
```

---

## Parallel Example: User Story 2

```bash
# Launch all tests for User Story 2 together:
Task: "Add unit test test_databricks_variable_in_struct() in src/import/sql.rs"
Task: "Add unit test test_databricks_variable_in_array() in src/import/sql.rs"
Task: "Add unit test test_databricks_nested_variables() in src/import/sql.rs"
Task: "Add integration test test_databricks_variables_in_types() in tests/import_tests.rs"

# After tests are written, launch preprocessing functions in parallel:
Task: "Implement replace_variables_in_struct_types() function in src/import/sql.rs"
Task: "Implement replace_variables_in_array_types() function in src/import/sql.rs"
Task: "Implement replace_nested_variables() function in src/import/sql.rs"
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only - Both P1)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (IDENTIFIER() support)
4. Complete Phase 4: User Story 2 (Variable references in types)
5. **STOP and VALIDATE**: Test User Stories 1 & 2 independently
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP Part 1)
3. Add User Story 2 → Test independently → Deploy/Demo (MVP Part 2)
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (IDENTIFIER() support)
   - Developer B: User Story 2 (Variable references in types)
   - Developer C: User Story 3 (Metadata variables)
   - Developer D: User Story 4 (Column variables)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- All code must pass cargo fmt, cargo clippy, cargo audit, and cargo test before commit
- All commits must be GPG signed per Constitution requirements
