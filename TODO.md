# TODO: Data Decision Log & Knowledge Base Implementation

## Overview

This document tracks all tasks for implementing the Data Decision Log (DDL) and Knowledge Base (KB) features. Tasks are organized by phase and priority.

**Related Document:** [PLAN.md](./PLAN.md) - Detailed implementation plan

---

## Phase 1: Core Models (Priority: High) - COMPLETED

### 1.1 Decision Model
- [x] Create `src/models/decision.rs`
  - [x] Define `DecisionStatus` enum (Proposed, Accepted, Deprecated, Superseded)
  - [x] Define `DecisionCategory` enum (Architecture, DataDesign, Workflow, etc.)
  - [x] Define `DecisionOption` struct (name, description, pros, cons, selected)
  - [x] Define `DecisionDriver` struct (description, priority)
  - [x] Define `AssetLink` struct (asset_type, asset_id, asset_name, relationship)
  - [x] Define `ComplianceAssessment` struct (regulatory, privacy, security, frameworks)
  - [x] Define `Decision` struct with all MADR fields
  - [x] Implement `Decision::new()` constructor
  - [x] Implement `Decision::generate_id()` using UUID v5
  - [x] Implement `Decision::from_yaml()` and `Decision::to_yaml()`
  - [x] Add serde serialization with proper field naming
  - [x] Add unit tests for Decision model

### 1.2 Knowledge Model
- [x] Create `src/models/knowledge.rs`
  - [x] Define `KnowledgeType` enum (Guide, Standard, Reference, etc.)
  - [x] Define `KnowledgeStatus` enum (Draft, Published, Archived, Deprecated)
  - [x] Define `RelatedArticle` struct (article_id, article_number, title, relationship)
  - [x] Define `KnowledgeArticle` struct with all fields
  - [x] Implement `KnowledgeArticle::new()` constructor
  - [x] Implement `KnowledgeArticle::generate_id()` using UUID v5
  - [x] Implement `KnowledgeArticle::from_yaml()` and `KnowledgeArticle::to_yaml()`
  - [x] Add serde serialization with proper field naming
  - [x] Add unit tests for KnowledgeArticle model

### 1.3 Index Models
- [x] Create `DecisionIndex` struct in `src/models/decision.rs`
  - [x] Define `DecisionIndexEntry` (number, id, title, status, category, domain, file)
  - [x] Implement serialization for `decisions.yaml`
- [x] Create `KnowledgeIndex` struct in `src/models/knowledge.rs`
  - [x] Define `KnowledgeIndexEntry` (number, id, title, type, status, domain, file)
  - [x] Implement serialization for `knowledge.yaml`

### 1.4 Module Integration
- [x] Update `src/models/mod.rs`
  - [x] Add `pub mod decision;`
  - [x] Add `pub mod knowledge;`
  - [x] Re-export all public types
  - [x] Move `AssetLink` to shared location (or keep in decision.rs and re-export)

---

## Phase 2: Asset Type Extensions (Priority: High) - COMPLETED

### 2.1 Extend AssetType
- [x] Update `src/models/workspace.rs`
  - [x] Add `Decision` variant to `AssetType` enum
  - [x] Add `Knowledge` variant to `AssetType` enum
  - [x] Add `DecisionIndex` variant to `AssetType` enum
  - [x] Add `KnowledgeIndex` variant to `AssetType` enum
  - [x] Update `extension()` method for new types
  - [x] Update `from_filename()` method for new types
  - [x] Update `supported_extensions()` for new types
  - [x] Add unit tests for new asset types

---

## Phase 3: JSON Schemas (Priority: High) - COMPLETED

JSON Schemas must be created **before** import modules since importers validate against them.

### 3.1 Decision Schema
- [x] Create `schemas/decision-schema.json`
  - [x] Add `$schema` and `$id` metadata
  - [x] Define required fields: `id`, `number`, `title`, `status`, `category`, `date`, `context`, `decision`
  - [x] Add `status` enum: `proposed`, `accepted`, `deprecated`, `superseded`
  - [x] Add `category` enum: `architecture`, `datadesign`, `workflow`, `model`, `governance`, `security`, `performance`, `compliance`, `infrastructure`, `tooling`
  - [x] Add `drivers` array schema with `description` and `priority` enum
  - [x] Add `options` array schema with `name`, `description`, `pros`, `cons`, `selected`
  - [x] Add `linked_assets` array schema with `asset_type` enum
  - [x] Add `compliance` object schema
  - [x] Add UUID format validation for `id`, `supersedes`, `superseded_by`
  - [x] Add date-time format validation for `date`, `created_at`, `updated_at`
  - [x] Add string length constraints (title max 200 chars)

### 3.2 Knowledge Schema
- [x] Create `schemas/knowledge-schema.json`
  - [x] Add `$schema` and `$id` metadata
  - [x] Define required fields: `id`, `number`, `title`, `article_type`, `status`, `summary`, `content`, `author`
  - [x] Add `article_type` enum: `guide`, `standard`, `reference`, `glossary`, `howto`, `troubleshooting`, `policy`, `template`
  - [x] Add `status` enum: `draft`, `published`, `archived`, `deprecated`
  - [x] Add `review_frequency` enum: `monthly`, `quarterly`, `yearly`
  - [x] Add `skill_level` enum: `beginner`, `intermediate`, `advanced`
  - [x] Add `linked_assets` array schema
  - [x] Add `linked_decisions` array of UUIDs
  - [x] Add `related_articles` array schema with relationship enum
  - [x] Add `number` pattern validation: `^KB-[0-9]{4}$`
  - [x] Add UUID format validation
  - [x] Add date-time format validation

### 3.3 Index Schemas
- [x] Create `schemas/decisions-index-schema.json`
  - [x] Define required fields: `schema_version`, `decisions`, `next_number`
  - [x] Add `decisions` array item schema
- [x] Create `schemas/knowledge-index-schema.json`
  - [x] Define required fields: `schema_version`, `articles`, `next_number`
  - [x] Add `articles` array item schema

### 3.4 Schema Documentation
- [x] Update `schemas/README.md`
  - [x] Add decision schema documentation
  - [x] Add knowledge schema documentation
  - [x] Add index schema documentation
  - [x] Add validation examples

---

## Phase 4: CLI Validation Integration (Priority: High) - COMPLETED

### 4.1 Validation Functions
- [x] Update `src/cli/validation.rs`
  - [x] Add `validate_decision()` function
    - [x] Load schema with `include_str!("../../schemas/decision-schema.json")`
    - [x] Parse YAML content to JSON Value
    - [x] Validate using `jsonschema::Validator`
    - [x] Format errors with path information using `format_validation_error()`
  - [x] Add `validate_knowledge()` function
    - [x] Load schema with `include_str!("../../schemas/knowledge-schema.json")`
    - [x] Parse YAML content to JSON Value
    - [x] Validate using `jsonschema::Validator`
    - [x] Format errors with path information
  - [x] Add `validate_decision_index()` function (optional)
  - [x] Add `validate_knowledge_index()` function (optional)

### 4.2 CLI Validate Command
- [x] Update `src/cli/main.rs`
  - [x] Add `Decision` variant to `ValidateFormatArg` enum
  - [x] Add `Knowledge` variant to `ValidateFormatArg` enum
- [x] Update `src/cli/commands/validate.rs`
  - [x] Add `"decision"` case to `handle_validate()` match
  - [x] Add `"knowledge"` case to `handle_validate()` match
  - [x] Import `validate_decision` and `validate_knowledge` functions

### 4.3 Validation Tests
- [ ] Create validation tests
  - [ ] Test valid decision YAML passes validation
  - [ ] Test invalid decision YAML fails with correct error
  - [ ] Test valid knowledge YAML passes validation
  - [ ] Test invalid knowledge YAML fails with correct error
  - [ ] Test missing required fields are detected
  - [ ] Test invalid enum values are detected

---

## Phase 5: Import Modules with Validation (Priority: High) - COMPLETED

### 5.1 Decision Importer
- [x] Create `src/import/decision.rs`
  - [x] Implement `DecisionImporter` struct
  - [x] Implement `import()` method for single YAML file
    - [x] Call `validate_decision()` first (when `schema-validation` feature enabled)
    - [x] Parse YAML to `Decision` struct
    - [x] Return `ImportError::ValidationFailed` on schema errors
  - [x] Implement `import_index()` method for decisions.yaml
  - [x] Handle UUID parsing and generation
  - [x] Handle date parsing
  - [x] Add `ImportError` variants for decision-specific errors
  - [x] Add unit tests

### 5.2 Knowledge Importer
- [x] Create `src/import/knowledge.rs`
  - [x] Implement `KnowledgeImporter` struct
  - [x] Implement `import()` method for single YAML file
    - [x] Call `validate_knowledge()` first (when `schema-validation` feature enabled)
    - [x] Parse YAML to `KnowledgeArticle` struct
    - [x] Return `ImportError::ValidationFailed` on schema errors
  - [x] Implement `import_index()` method for knowledge.yaml
  - [x] Handle Markdown content parsing
  - [x] Add `ImportError` variants for knowledge-specific errors
  - [x] Add unit tests

### 5.3 Module Integration
- [x] Update `src/import/mod.rs`
  - [x] Add `pub mod decision;`
  - [x] Add `pub mod knowledge;`
  - [x] Re-export `DecisionImporter` and `KnowledgeImporter`

---

## Phase 6: Export Modules (Priority: High) - COMPLETED

### 6.1 Decision Exporter (YAML)
- [x] Create `src/export/decision.rs`
  - [x] Implement `DecisionExporter` struct
  - [x] Implement `export()` method for single decision to YAML
  - [x] Implement `export_index()` method for decisions.yaml
  - [x] Handle proper YAML formatting
  - [x] Add unit tests

### 6.2 Knowledge Exporter (YAML)
- [x] Create `src/export/knowledge.rs`
  - [x] Implement `KnowledgeExporter` struct
  - [x] Implement `export()` method for single article to YAML
  - [x] Implement `export_index()` method for knowledge.yaml
  - [x] Handle proper YAML formatting
  - [x] Add unit tests

### 6.3 Markdown Exporter
- [x] Create `src/export/markdown.rs`
  - [x] Implement `MarkdownExporter` struct
  - [x] Implement `export_decision()` method
    - [x] Generate MADR-compliant header table
    - [x] Format context section
    - [x] Format decision drivers with priority
    - [x] Format options with pros/cons
    - [x] Format decision and consequences
    - [x] Format linked assets table
    - [x] Format compliance section
    - [x] Format footer with tags and dates
  - [x] Implement `export_knowledge()` method
    - [x] Generate header table
    - [x] Format summary section
    - [x] Include Markdown content as-is
    - [x] Format linked assets table
    - [x] Format related articles table
    - [x] Format footer with reviewers, tags, dates
  - [x] Implement `export_decisions_to_dir()` method
  - [x] Implement `export_knowledge_to_dir()` method
  - [x] Generate filenames: `ADR-NNNN-slug.md`, `KB-NNNN-slug.md`
  - [x] Add unit tests

### 6.4 Module Integration
- [x] Update `src/export/mod.rs`
  - [x] Add `pub mod decision;`
  - [x] Add `pub mod knowledge;`
  - [x] Add `pub mod markdown;`
  - [x] Re-export exporters

---

## Phase 7: Storage Integration (Priority: High) - COMPLETED

### 7.1 Model Loader Extensions
- [x] Update `src/model/loader.rs`
  - [x] Add `load_decisions()` method
    - [x] Load all `.madr.yaml` files from workspace
    - [x] Parse using DecisionImporter
    - [x] Return `DecisionLoadResult` with decisions and errors
  - [x] Add `load_decision_index()` method
    - [x] Load `decisions.yaml`
    - [x] Return `DecisionIndex`
  - [x] Add `load_knowledge()` method
    - [x] Load all `.kb.yaml` files from workspace
    - [x] Parse using KnowledgeImporter
    - [x] Return `KnowledgeLoadResult` with articles and errors
  - [x] Add `load_knowledge_index()` method
    - [x] Load `knowledge.yaml`
    - [x] Return `KnowledgeIndex`
  - [x] Add `load_decisions_by_domain()` method for domain filtering
  - [x] Add `load_knowledge_by_domain()` method for domain filtering
  - [x] Add `DecisionLoadResult` and `KnowledgeLoadResult` structs
  - [x] Add `DecisionLoadError` and `KnowledgeLoadError` structs

### 7.2 Model Saver Extensions
- [x] Update `src/model/saver.rs`
  - [x] Add `save_decision()` method
    - [x] Generate filename using naming convention
    - [x] Save to `{workspace}_{domain}_adr-{number}.madr.yaml`
    - [x] Handle domain-less decisions
  - [x] Add `save_decision_index()` method
    - [x] Save `decisions.yaml` with all decision references
  - [x] Add `save_knowledge()` method
    - [x] Generate filename using naming convention
    - [x] Save to `{workspace}_{domain}_kb-{number}.kb.yaml`
  - [x] Add `save_knowledge_index()` method
    - [x] Save `knowledge.yaml` with all article references
  - [x] Add `export_decision_markdown()` method
    - [x] Export to `decisions/ADR-NNNN-slug.md`
  - [x] Add `export_knowledge_markdown()` method
    - [x] Export to `knowledge/KB-NNNN-slug.md`
  - [x] Add `export_all_decisions_markdown()` method
  - [x] Add `export_all_knowledge_markdown()` method

---

## Phase 8: Database Schema (Priority: Medium) - COMPLETED

### 8.1 Schema Definitions
- [x] Update `src/database/schema.rs`
  - [x] Add `decisions` table definition
    - [x] All columns (id, workspace_id, domain_id, number, title, status, category, etc.)
    - [x] Foreign key to workspaces
    - [x] Foreign key to domains (nullable)
    - [x] Unique constraint on (workspace_id, number)
  - [x] Add `knowledge_articles` table definition
    - [x] All columns (id, workspace_id, domain_id, number, title, article_type, status, etc.)
    - [x] Foreign key to workspaces
    - [x] Foreign key to domains (nullable)
    - [x] Unique constraint on (workspace_id, number)
  - [x] Add index definitions for both tables
    - [x] Indexes for workspace, domain, status, category, date, number queries
  - [x] Add SQL modules for UPSERT/SELECT/DELETE
    - [x] `decision_sql` module with UPSERT, SELECT_BY_*, DELETE, COUNT, MAX_NUMBER
    - [x] `knowledge_sql` module with UPSERT, SELECT_BY_*, SEARCH_CONTENT, DELETE, COUNT, MAX_NUMBER
  - [x] Update `SCHEMA_VERSION` constant to 2
  - [x] Update `drop_all_tables_sql` to include new tables

### 8.2 Schema Tests
- [x] Add tests for decision SQL constants
- [x] Add tests for knowledge SQL constants

---

## Phase 9: Sync Engine (Priority: Medium) - COMPLETED

### 9.1 Decision Sync
- [x] Update `src/database/sync.rs`
  - [x] Add `sync_decisions()` method to SyncEngine
  - [x] Add `sync_decisions()` method to DatabaseBackend trait
  - [x] Add `export_decisions()` method to DatabaseBackend trait
  - [x] Add decision sync to `sync_workspace_full()` flow

### 9.2 Knowledge Sync
- [x] Update `src/database/sync.rs`
  - [x] Add `sync_knowledge()` method to SyncEngine
  - [x] Add `sync_knowledge()` method to DatabaseBackend trait
  - [x] Add `export_knowledge()` method to DatabaseBackend trait
  - [x] Add knowledge sync to `sync_workspace_full()` flow

### 9.3 Sync Result Updates
- [x] Extend `SyncResult` struct
  - [x] Add `decisions_synced: usize`
  - [x] Add `knowledge_synced: usize`
  - [x] Update `total_synced()` to include decisions and knowledge
- [x] Extend `SyncStatus` struct
  - [x] Add `decision_count: usize`
  - [x] Add `knowledge_count: usize`

### 9.4 Export Methods
- [x] Add `export_workspace_full()` to return all data including decisions/knowledge
- [x] Add `export_decisions()` wrapper method
- [x] Add `export_knowledge()` wrapper method

---

## Phase 10: CLI Commands (Priority: Medium) - COMPLETED

### 10.1 Decision Commands
- [x] Create `src/cli/commands/decision.rs`
  - [x] Define argument structs (DecisionNewArgs, DecisionListArgs, etc.)
  - [x] Implement `handle_decision_new()`
    - [x] Parse arguments and category
    - [x] Generate UUID and number
    - [x] Create Decision struct
    - [x] Save YAML file
    - [x] Update decisions.yaml index
    - [x] Optionally export Markdown
  - [x] Implement `handle_decision_list()`
    - [x] Load decisions from workspace
    - [x] Apply filters (status, category, domain)
    - [x] Display table/json/csv output
  - [x] Implement `handle_decision_status()`
    - [x] Load decision by number
    - [x] Update status
    - [x] Save YAML file
    - [x] Update index
  - [x] Implement `handle_decision_export()`
    - [x] Load decisions
    - [x] Export to Markdown
    - [x] Create decisions/ directory
    - [x] Optionally generate index
  - [x] Implement `handle_decision_show()`
    - [x] Load decision
    - [x] Display yaml/markdown/json output
  - [x] Add helper functions (parse_category, parse_status, etc.)
  - [x] Add unit tests

### 10.2 Knowledge Commands
- [x] Create `src/cli/commands/knowledge.rs`
  - [x] Define argument structs (KnowledgeNewArgs, KnowledgeListArgs, etc.)
  - [x] Implement `handle_knowledge_new()`
    - [x] Parse arguments and article type
    - [x] Generate UUID and number
    - [x] Create KnowledgeArticle struct
    - [x] Save YAML file
    - [x] Update knowledge.yaml index
    - [x] Optionally export Markdown
  - [x] Implement `handle_knowledge_list()`
    - [x] Load articles from workspace
    - [x] Apply filters (type, status, domain, author)
    - [x] Display table/json/csv output
  - [x] Implement `handle_knowledge_status()`
    - [x] Load article by number
    - [x] Update status
    - [x] Save YAML file
    - [x] Update index
  - [x] Implement `handle_knowledge_export()`
    - [x] Load articles
    - [x] Export to Markdown
    - [x] Create knowledge/ directory
    - [x] Support by-domain organization
    - [x] Optionally generate index
  - [x] Implement `handle_knowledge_search()`
    - [x] Search title, summary, content, tags
    - [x] Display results
  - [x] Implement `handle_knowledge_show()`
    - [x] Load article
    - [x] Display yaml/markdown/json output
  - [x] Add helper functions (parse_article_type, parse_status, etc.)
  - [x] Add unit tests

### 10.3 Module Integration
- [x] Update `src/cli/commands/mod.rs`
  - [x] Add `pub mod decision;`
  - [x] Add `pub mod knowledge;`

Note: CLI main.rs integration with subcommands can be done in a future iteration.
The command handler functions are ready to be wired up.

---

## Phase 11: Business Logic Validation (Priority: Low)

### 11.1 Decision Validation
- [ ] Create `src/validation/decision.rs`
  - [ ] Validate required fields (title, context, decision)
  - [ ] Validate status transitions
  - [ ] Validate supersession chains (no cycles)
  - [ ] Validate asset links (assets exist)
  - [ ] Validate unique decision numbers
  - [ ] Add error types

### 11.2 Knowledge Validation
- [ ] Create `src/validation/knowledge.rs`
  - [ ] Validate required fields (title, summary, content, author)
  - [ ] Validate status transitions
  - [ ] Validate related article references exist
  - [ ] Validate asset links (assets exist)
  - [ ] Validate decision links (decisions exist)
  - [ ] Validate unique article numbers
  - [ ] Add error types

### 11.3 Module Integration
- [ ] Update `src/validation/mod.rs`
  - [ ] Add `pub mod decision;`
  - [ ] Add `pub mod knowledge;`

---

## Phase 12: Testing (Priority: Medium)

### 12.1 Unit Tests
- [ ] Create `tests/decision_tests.rs`
  - [ ] Test Decision model creation
  - [ ] Test serialization/deserialization
  - [ ] Test ID generation
  - [ ] Test import/export
- [ ] Create `tests/knowledge_tests.rs`
  - [ ] Test KnowledgeArticle model creation
  - [ ] Test serialization/deserialization
  - [ ] Test ID generation
  - [ ] Test import/export
- [ ] Create `tests/markdown_export_tests.rs`
  - [ ] Test decision Markdown export
  - [ ] Test knowledge Markdown export
  - [ ] Test file generation

### 12.2 Integration Tests
- [ ] Add decision CLI integration tests
  - [ ] Test `decision new`
  - [ ] Test `decision list`
  - [ ] Test `decision status`
  - [ ] Test `decision export`
- [ ] Add knowledge CLI integration tests
  - [ ] Test `knowledge new`
  - [ ] Test `knowledge list`
  - [ ] Test `knowledge status`
  - [ ] Test `knowledge export`
- [ ] Add database sync integration tests
  - [ ] Test decision sync
  - [ ] Test knowledge sync
  - [ ] Test change detection
- [ ] Add schema validation tests
  - [ ] Test valid YAML passes validation
  - [ ] Test invalid YAML fails with helpful errors
  - [ ] Test CLI `validate decision` command
  - [ ] Test CLI `validate knowledge` command

---

## Phase 13: Documentation (Priority: Medium)

### 13.1 LLM.txt Updates
- [ ] Update `LLM.txt`
  - [ ] Add decision model documentation
  - [ ] Add knowledge model documentation
  - [ ] Add CLI command documentation
  - [ ] Add file structure documentation
  - [ ] Update directory structure diagram

### 13.2 README Updates
- [ ] Update `README.md`
  - [ ] Add feature description
  - [ ] Add CLI usage examples
  - [ ] Add YAML format examples

### 13.3 Example Files
- [ ] Create `examples/decisions.yaml`
- [ ] Create `examples/knowledge.yaml`
- [ ] Create `examples/enterprise_sales_adr-0001.madr.yaml`
- [ ] Create `examples/enterprise_sales_kb-0001.kb.yaml`
- [ ] Create `examples/decisions/ADR-0001-use-odcs-format.md`
- [ ] Create `examples/knowledge/KB-0001-data-classification-guide.md`

---

## Phase 14: Version Bump & Release (Priority: Low)

### 14.1 Version Update
- [ ] Update `Cargo.toml` version
- [ ] Update `CHANGELOG.md`
- [ ] Tag release

---

## Task Summary

| Phase | Description | Tasks | Priority | Est. Effort |
|-------|-------------|-------|----------|-------------|
| 1 | Core Models | 15 | High | 2 days |
| 2 | Asset Type Extensions | 4 | High | 0.5 days |
| 3 | JSON Schemas | 12 | High | 1 day |
| 4 | CLI Validation Integration | 8 | High | 0.5 days |
| 5 | Import Modules with Validation | 10 | High | 1 day |
| 6 | Export Modules | 12 | High | 1.5 days |
| 7 | Storage Integration | 10 | High | 1.5 days |
| 8 | Database Schema | 6 | Medium | 1 day |
| 9 | Sync Engine | 8 | Medium | 1 day |
| 10 | CLI Commands | 20 | Medium | 2 days |
| 11 | Business Logic Validation | 8 | Low | 1 day |
| 12 | Testing | 16 | Medium | 1.5 days |
| 13 | Documentation | 8 | Medium | 1 day |
| 14 | Release | 3 | Low | 0.5 days |

**Total Estimated Effort:** ~15 days

---

## Definition of Done

A task is complete when:
1. Code is written and compiles without warnings
2. Unit tests pass
3. Integration tests pass (if applicable)
4. Code follows existing patterns and conventions
5. Documentation is updated (if applicable)
6. Code is reviewed and approved

---

## Notes

- **JSON Schemas are High Priority** - Must be created before importers since validation happens on import
- Start with Phases 1-7 (Core functionality) before CLI commands
- Database sync can be implemented after CLI works with files only
- Markdown export is the key differentiator for GitHub readability
- Consider adding `--auto-export-md` flag to `decision new` and `knowledge new`
- Follow existing validation pattern in `src/cli/validation.rs` using `jsonschema` crate
- Use `include_str!()` to embed schemas at compile time (same as ODCS, ODPS, etc.)
