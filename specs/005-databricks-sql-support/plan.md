# Implementation Plan: Enhanced Databricks SQL Syntax Support

**Branch**: `005-databricks-sql-support` | **Date**: 2026-01-04 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/005-databricks-sql-support/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Enable the SQL parser to recognize and handle Databricks-specific syntax patterns, including `IDENTIFIER()` function calls with variable references and string concatenation, as well as variable references in STRUCT/ARRAY type definitions, COMMENT clauses, and TBLPROPERTIES. This will allow users to import real-world Databricks SQL DDL statements that are currently blocked by parse errors.

Technical approach: Extend the sqlparser crate's `Dialect` trait to create a `DatabricksDialect` that recognizes Databricks-specific syntax patterns. Implement preprocessing/post-processing logic to handle variable references that cannot be parsed directly, replacing them with fallback types or placeholders.

## Technical Context

**Language/Version**: Rust 1.75+ (Rust 2024 edition)
**Primary Dependencies**: sqlparser 0.53, regex 1.0
**Storage**: N/A (parsing only, no storage operations)
**Testing**: cargo test (unit tests, integration tests, doctests)
**Target Platform**: All platforms (native, WASM) - SQL parsing is platform-agnostic
**Project Type**: Single library project (SDK)
**Performance Goals**: Parse Databricks SQL at similar speed to standard SQL (target: <10% performance degradation)
**Constraints**: Must maintain backward compatibility with existing SQL dialects (PostgreSQL, MySQL, SQLite, Generic). Must not break existing imports.
**Scale/Scope**: Handle typical Databricks SQL DDL statements (hundreds to thousands of lines). Support complex nested STRUCT/ARRAY patterns up to 5+ levels deep.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Verify compliance with Data Modelling SDK Constitution principles:

- **Commit Requirements**: ✅ Code MUST build successfully before commit. All commits MUST be GPG signed.
- **Code Quality & Security**: ✅ Plan includes security audit (`cargo audit`), formatting (`cargo fmt`), linting (`cargo clippy`) checks. sqlparser 0.53 is latest stable version compatible with project.
- **Storage Abstraction**: ✅ N/A - No storage operations added in this feature.
- **Feature Flags**: ✅ Databricks dialect support will be behind existing `databricks-dialect` Cargo feature flag (already defined in Cargo.toml).
- **Testing Requirements**: ✅ Plan includes unit tests for DatabricksDialect, integration tests for SQL import with Databricks SQL, and doctests for public API methods.
- **Import/Export Patterns**: ✅ Follows existing SQL importer pattern in `src/import/sql.rs`. No new importer trait needed.
- **Error Handling**: ✅ Uses existing `ImportError` type from `src/import/mod.rs`. Structured error types already in place.

**Constitution Compliance**: ✅ All principles satisfied. No violations.

## Project Structure

### Documentation (this feature)

```text
specs/005-databricks-sql-support/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── import/
│   ├── sql.rs           # Extend SQLImporter with DatabricksDialect support
│   └── mod.rs           # Export DatabricksDialect if needed
└── [other existing modules]

tests/
├── import_tests.rs      # Add Databricks SQL import test cases
└── [other existing tests]
```

**Structure Decision**: Single project structure. Databricks dialect support extends existing SQL import functionality in `src/import/sql.rs`. No new modules needed - add `DatabricksDialect` implementation and extend `SQLImporter::dialect_impl()` to handle "databricks" dialect string.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations - all constitution principles satisfied.
