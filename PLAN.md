# Feature Plan: Data Decision Log (DDL) & Knowledge Base

## Overview

This plan implements two complementary features for the Data Modelling SDK:

1. **Data Decision Log (DDL)** - MADR-compliant decision tracking system
2. **Knowledge Base (KB)** - Domain-partitioned knowledge repository

Both features follow the SDK's core principle: **Git YAML as the master record** with database sync for querying.

---

## Architecture

```
workspace/
├── workspace.yaml                              # Workspace config (relationships embedded)
├── decisions.yaml                              # Decision log index (references individual ADRs)
├── decisions/                                  # Individual decision records
│   ├── ADR-0001-use-odcs-format.md            # Exported Markdown for GitHub readability
│   └── {workspace}_{domain}_adr-0001.madr.yaml # Source YAML (master record)
├── knowledge.yaml                              # Knowledge base index
├── knowledge/                                  # Individual knowledge articles
│   ├── KB-0001-data-classification-guide.md   # Exported Markdown for GitHub readability
│   └── {workspace}_{domain}_kb-0001.kb.yaml   # Source YAML (master record)
├── {workspace}_{domain}_{resource}.odcs.yaml  # ODCS files (existing)
├── {workspace}_{domain}_{resource}.odps.yaml  # ODPS files (existing)
└── {workspace}_{domain}_{resource}.cads.yaml  # CADS files (existing)
```

### Key Principles

1. **YAML is Master** - All decisions and knowledge stored as `.madr.yaml` and `.kb.yaml`
2. **Markdown for Reading** - CLI exports to Markdown for GitHub/offline readability
3. **Database for Querying** - Sync to DuckDB/PostgreSQL for fast search and filtering
4. **Domain Partitioning** - Both DDL and KB support domain-level organization
5. **Asset Linking** - Decisions and knowledge can reference ODCS/ODPS/CADS assets

---

## Part 1: Data Decision Log (DDL)

### 1.1 Data Model

#### Decision Status Lifecycle

```
Proposed → Accepted → [Deprecated | Superseded]
```

#### Decision Categories (from feature request)

- Architecture
- DataDesign
- Workflow
- Model
- Governance
- Security
- Performance
- Compliance
- Infrastructure
- Tooling

#### Rust Model (`src/models/decision.rs`)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Decision status in lifecycle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DecisionStatus {
    Proposed,
    Accepted,
    Deprecated,
    Superseded,
}

/// Decision category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DecisionCategory {
    Architecture,
    DataDesign,
    Workflow,
    Model,
    Governance,
    Security,
    Performance,
    Compliance,
    Infrastructure,
    Tooling,
}

/// Option considered during decision making
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionOption {
    pub name: String,
    pub description: Option<String>,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub selected: bool,
}

/// Driver/reason for the decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionDriver {
    pub description: String,
    pub priority: Option<String>,  // high, medium, low
}

/// Link to an asset (table, relationship, product, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetLink {
    pub asset_type: String,  // odcs, odps, cads, relationship
    pub asset_id: Uuid,
    pub asset_name: String,
    pub relationship: Option<String>,  // affects, implements, deprecates
}

/// Compliance assessment for the decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComplianceAssessment {
    pub regulatory_impact: Option<String>,
    pub privacy_assessment: Option<String>,
    pub security_assessment: Option<String>,
    pub frameworks: Vec<String>,  // GDPR, SOC2, HIPAA, etc.
}

/// MADR-compliant Decision Record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Decision {
    pub id: Uuid,
    pub number: u32,  // ADR-0001, ADR-0002, etc.
    pub title: String,
    pub status: DecisionStatus,
    pub category: DecisionCategory,
    pub domain: Option<String>,  // Domain this decision belongs to

    // MADR template fields
    pub date: DateTime<Utc>,
    pub deciders: Vec<String>,
    pub context: String,
    pub drivers: Vec<DecisionDriver>,
    pub options: Vec<DecisionOption>,
    pub decision: String,
    pub consequences: String,

    // Linking
    pub linked_assets: Vec<AssetLink>,
    pub supersedes: Option<Uuid>,
    pub superseded_by: Option<Uuid>,

    // Compliance (from feature request)
    pub compliance: Option<ComplianceAssessment>,

    // Confirmation tracking (from feature request)
    pub confirmation_date: Option<DateTime<Utc>>,
    pub confirmation_notes: Option<String>,

    // Standard metadata
    pub tags: Vec<crate::models::Tag>,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### 1.2 YAML Format (`.madr.yaml`)

Example: `enterprise_sales_adr-0001.madr.yaml`

```yaml
id: 550e8400-e29b-41d4-a716-446655440000
number: 1
title: "Use ODCS v3.1.0 for all data contracts"
status: accepted
category: datadesign
domain: sales
date: 2026-01-07T10:00:00Z
deciders:
  - data-architecture@company.com
  - platform-team@company.com

context: |
  We need a standard format for defining data contracts across
  all domains. Multiple teams are creating schemas in different
  formats leading to inconsistency.

drivers:
  - description: "Need for schema consistency across teams"
    priority: high
  - description: "Support for quality rules and SLAs"
    priority: high
  - description: "Industry standard adoption"
    priority: medium

options:
  - name: "ODCS v3.1.0"
    description: "Open Data Contract Standard"
    pros:
      - "Industry standard"
      - "Rich metadata support"
      - "Quality rules built-in"
    cons:
      - "Learning curve for teams"
    selected: true

  - name: "Custom JSON Schema"
    description: "Internal schema format"
    pros:
      - "Full control"
    cons:
      - "Maintenance burden"
      - "No community support"
    selected: false

decision: |
  We will adopt ODCS v3.1.0 as the standard format for all
  data contracts. All existing schemas will be migrated.

consequences: |
  Positive:
  - Consistent contracts across domains
  - Better tooling support
  - Industry alignment

  Negative:
  - Initial migration effort required
  - Team training needed

linked_assets:
  - asset_type: odcs
    asset_id: 550e8400-e29b-41d4-a716-446655440001
    asset_name: orders
    relationship: implements

compliance:
  regulatory_impact: "GDPR compliant metadata fields supported"
  privacy_assessment: "PII tagging available via classification field"
  security_assessment: "Encryption metadata supported"
  frameworks:
    - GDPR
    - SOC2

tags:
  - data-contracts
  - "Environment:Production"

created_at: 2026-01-07T10:00:00Z
updated_at: 2026-01-07T10:00:00Z
```

### 1.3 Markdown Export Format

Example: `decisions/ADR-0001-use-odcs-format.md`

```markdown
# ADR-0001: Use ODCS v3.1.0 for all data contracts

| Property | Value |
|----------|-------|
| **Status** | Accepted |
| **Category** | Data Design |
| **Domain** | Sales |
| **Date** | 2026-01-07 |
| **Deciders** | data-architecture@company.com, platform-team@company.com |

## Context

We need a standard format for defining data contracts across
all domains. Multiple teams are creating schemas in different
formats leading to inconsistency.

## Decision Drivers

1. **[High]** Need for schema consistency across teams
2. **[High]** Support for quality rules and SLAs
3. **[Medium]** Industry standard adoption

## Considered Options

### Option 1: ODCS v3.1.0 (Selected)

Open Data Contract Standard

**Pros:**
- Industry standard
- Rich metadata support
- Quality rules built-in

**Cons:**
- Learning curve for teams

### Option 2: Custom JSON Schema

Internal schema format

**Pros:**
- Full control

**Cons:**
- Maintenance burden
- No community support

## Decision

We will adopt ODCS v3.1.0 as the standard format for all
data contracts. All existing schemas will be migrated.

## Consequences

**Positive:**
- Consistent contracts across domains
- Better tooling support
- Industry alignment

**Negative:**
- Initial migration effort required
- Team training needed

## Linked Assets

| Type | Name | Relationship |
|------|------|--------------|
| ODCS | orders | implements |

## Compliance

- **Regulatory Impact:** GDPR compliant metadata fields supported
- **Privacy Assessment:** PII tagging available via classification field
- **Security Assessment:** Encryption metadata supported
- **Frameworks:** GDPR, SOC2

---

*Tags: data-contracts, Environment:Production*

*Created: 2026-01-07 | Updated: 2026-01-07*
```

### 1.4 Database Schema

```sql
-- Decision log table
CREATE TABLE IF NOT EXISTS decisions (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    domain_id UUID REFERENCES domains(id),
    number INTEGER NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    category TEXT NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    deciders JSON,
    context TEXT NOT NULL,
    drivers JSON,
    options JSON,
    decision TEXT NOT NULL,
    consequences TEXT,
    linked_assets JSON,
    supersedes UUID REFERENCES decisions(id),
    superseded_by UUID REFERENCES decisions(id),
    compliance JSON,
    confirmation_date TIMESTAMPTZ,
    confirmation_notes TEXT,
    tags JSON,
    notes TEXT,
    yaml_file_path TEXT,
    yaml_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(workspace_id, number)
);

-- Indexes for fast querying
CREATE INDEX IF NOT EXISTS idx_decisions_workspace ON decisions(workspace_id);
CREATE INDEX IF NOT EXISTS idx_decisions_domain ON decisions(domain_id);
CREATE INDEX IF NOT EXISTS idx_decisions_status ON decisions(status);
CREATE INDEX IF NOT EXISTS idx_decisions_category ON decisions(category);
CREATE INDEX IF NOT EXISTS idx_decisions_date ON decisions(date DESC);
CREATE INDEX IF NOT EXISTS idx_decisions_number ON decisions(workspace_id, number);
```

### 1.5 CLI Commands

```bash
# Create a new decision (interactive or with flags)
data-modelling-cli decision new "Use ODCS v3.1.0" \
    --category datadesign \
    --domain sales \
    --workspace .

# List decisions with filtering
data-modelling-cli decision list \
    --status accepted \
    --category architecture \
    --domain sales \
    --workspace .

# Update decision status
data-modelling-cli decision status ADR-0001 accepted \
    --workspace .

# Supersede a decision
data-modelling-cli decision supersede ADR-0001 \
    --by ADR-0005 \
    --workspace .

# Link decision to an asset
data-modelling-cli decision link ADR-0001 \
    --asset-type odcs \
    --asset-id <uuid> \
    --relationship implements \
    --workspace .

# Export decisions to Markdown
data-modelling-cli decision export \
    --output ./decisions \
    --format markdown \
    --workspace .

# Export single decision to Markdown
data-modelling-cli decision export ADR-0001 \
    --output ./decisions/ADR-0001.md \
    --format markdown \
    --workspace .

# Show decision details
data-modelling-cli decision show ADR-0001 \
    --workspace .
```

---

## Part 2: Knowledge Base (KB)

### 2.1 Data Model

#### Knowledge Article Types

- Guide
- Standard
- Reference
- Glossary
- HowTo
- Troubleshooting
- Policy
- Template

#### Rust Model (`src/models/knowledge.rs`)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Knowledge article type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum KnowledgeType {
    Guide,
    Standard,
    Reference,
    Glossary,
    HowTo,
    Troubleshooting,
    Policy,
    Template,
}

/// Knowledge article status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum KnowledgeStatus {
    Draft,
    Published,
    Archived,
    Deprecated,
}

/// Related article reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedArticle {
    pub article_id: Uuid,
    pub article_number: String,
    pub title: String,
    pub relationship: String,  // related, prerequisite, supersedes
}

/// Knowledge Base Article
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeArticle {
    pub id: Uuid,
    pub number: String,  // KB-0001, KB-0002, etc.
    pub title: String,
    pub article_type: KnowledgeType,
    pub status: KnowledgeStatus,
    pub domain: Option<String>,  // Domain this article belongs to

    // Content
    pub summary: String,
    pub content: String,  // Markdown content

    // Authorship
    pub author: String,
    pub reviewers: Vec<String>,
    pub last_reviewed: Option<DateTime<Utc>>,
    pub review_frequency: Option<String>,  // monthly, quarterly, yearly

    // Classification
    pub audience: Vec<String>,  // developers, architects, analysts
    pub skill_level: Option<String>,  // beginner, intermediate, advanced

    // Linking
    pub linked_assets: Vec<AssetLink>,  // Reuse from decision.rs
    pub linked_decisions: Vec<Uuid>,
    pub related_articles: Vec<RelatedArticle>,

    // Standard metadata
    pub tags: Vec<crate::models::Tag>,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### 2.2 YAML Format (`.kb.yaml`)

Example: `enterprise_sales_kb-0001.kb.yaml`

```yaml
id: 660e8400-e29b-41d4-a716-446655440000
number: KB-0001
title: "Data Classification Guide for Sales Domain"
article_type: guide
status: published
domain: sales

summary: |
  This guide explains how to properly classify data within the
  Sales domain according to company data governance policies.

content: |
  ## Overview

  Data classification is essential for ensuring proper handling
  of sensitive information...

  ## Classification Levels

  ### Public
  Data that can be freely shared...

  ### Internal
  Data for internal use only...

  ### Confidential
  Sensitive business data...

  ### Restricted
  Highly sensitive data requiring special handling...

  ## Classification Process

  1. Identify the data elements
  2. Determine sensitivity level
  3. Apply appropriate tags
  4. Document in ODCS file

author: data-governance@company.com
reviewers:
  - security@company.com
  - compliance@company.com
last_reviewed: 2026-01-01T00:00:00Z
review_frequency: quarterly

audience:
  - data-engineers
  - data-architects
  - analysts
skill_level: intermediate

linked_assets:
  - asset_type: odcs
    asset_id: 550e8400-e29b-41d4-a716-446655440001
    asset_name: customer_pii
    relationship: documents

linked_decisions:
  - 550e8400-e29b-41d4-a716-446655440000  # ADR-0001

related_articles:
  - article_id: 660e8400-e29b-41d4-a716-446655440001
    article_number: KB-0002
    title: "PII Handling Procedures"
    relationship: related

tags:
  - data-governance
  - classification
  - "Domain:Sales"

created_at: 2026-01-01T10:00:00Z
updated_at: 2026-01-07T10:00:00Z
```

### 2.3 Markdown Export Format

Example: `knowledge/KB-0001-data-classification-guide.md`

```markdown
# KB-0001: Data Classification Guide for Sales Domain

| Property | Value |
|----------|-------|
| **Type** | Guide |
| **Status** | Published |
| **Domain** | Sales |
| **Author** | data-governance@company.com |
| **Last Reviewed** | 2026-01-01 |
| **Review Frequency** | Quarterly |
| **Audience** | data-engineers, data-architects, analysts |
| **Skill Level** | Intermediate |

## Summary

This guide explains how to properly classify data within the
Sales domain according to company data governance policies.

---

## Overview

Data classification is essential for ensuring proper handling
of sensitive information...

## Classification Levels

### Public
Data that can be freely shared...

### Internal
Data for internal use only...

### Confidential
Sensitive business data...

### Restricted
Highly sensitive data requiring special handling...

## Classification Process

1. Identify the data elements
2. Determine sensitivity level
3. Apply appropriate tags
4. Document in ODCS file

---

## Linked Assets

| Type | Name | Relationship |
|------|------|--------------|
| ODCS | customer_pii | documents |

## Linked Decisions

- ADR-0001

## Related Articles

| Article | Title | Relationship |
|---------|-------|--------------|
| KB-0002 | PII Handling Procedures | related |

---

*Reviewers: security@company.com, compliance@company.com*

*Tags: data-governance, classification, Domain:Sales*

*Created: 2026-01-01 | Updated: 2026-01-07*
```

### 2.4 Database Schema

```sql
-- Knowledge base table
CREATE TABLE IF NOT EXISTS knowledge_articles (
    id UUID PRIMARY KEY,
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    domain_id UUID REFERENCES domains(id),
    number TEXT NOT NULL,
    title TEXT NOT NULL,
    article_type TEXT NOT NULL,
    status TEXT NOT NULL,
    summary TEXT NOT NULL,
    content TEXT NOT NULL,
    author TEXT NOT NULL,
    reviewers JSON,
    last_reviewed TIMESTAMPTZ,
    review_frequency TEXT,
    audience JSON,
    skill_level TEXT,
    linked_assets JSON,
    linked_decisions JSON,
    related_articles JSON,
    tags JSON,
    notes TEXT,
    yaml_file_path TEXT,
    yaml_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(workspace_id, number)
);

-- Indexes for fast querying
CREATE INDEX IF NOT EXISTS idx_knowledge_workspace ON knowledge_articles(workspace_id);
CREATE INDEX IF NOT EXISTS idx_knowledge_domain ON knowledge_articles(domain_id);
CREATE INDEX IF NOT EXISTS idx_knowledge_type ON knowledge_articles(article_type);
CREATE INDEX IF NOT EXISTS idx_knowledge_status ON knowledge_articles(status);
CREATE INDEX IF NOT EXISTS idx_knowledge_author ON knowledge_articles(author);
CREATE INDEX IF NOT EXISTS idx_knowledge_number ON knowledge_articles(workspace_id, number);

-- Full-text search (PostgreSQL)
-- CREATE INDEX IF NOT EXISTS idx_knowledge_content_fts ON knowledge_articles
--     USING GIN(to_tsvector('english', title || ' ' || summary || ' ' || content));
```

### 2.5 CLI Commands

```bash
# Create a new knowledge article
data-modelling-cli knowledge new "Data Classification Guide" \
    --type guide \
    --domain sales \
    --author data-governance@company.com \
    --workspace .

# List knowledge articles
data-modelling-cli knowledge list \
    --type guide \
    --status published \
    --domain sales \
    --workspace .

# Update article status
data-modelling-cli knowledge status KB-0001 published \
    --workspace .

# Link article to asset
data-modelling-cli knowledge link KB-0001 \
    --asset-type odcs \
    --asset-id <uuid> \
    --relationship documents \
    --workspace .

# Link article to decision
data-modelling-cli knowledge link-decision KB-0001 \
    --decision ADR-0001 \
    --workspace .

# Export knowledge base to Markdown
data-modelling-cli knowledge export \
    --output ./knowledge \
    --format markdown \
    --workspace .

# Export single article to Markdown
data-modelling-cli knowledge export KB-0001 \
    --output ./knowledge/KB-0001.md \
    --format markdown \
    --workspace .

# Search knowledge base
data-modelling-cli knowledge search "classification" \
    --domain sales \
    --workspace .

# Show article details
data-modelling-cli knowledge show KB-0001 \
    --workspace .
```

---

## Part 3: Index Files

### 3.1 decisions.yaml

Central index of all decisions in the workspace:

```yaml
# Decision Log Index
# Auto-generated - do not edit manually

schema_version: "1.0"
last_updated: 2026-01-07T10:00:00Z

decisions:
  - number: 1
    id: 550e8400-e29b-41d4-a716-446655440000
    title: "Use ODCS v3.1.0 for all data contracts"
    status: accepted
    category: datadesign
    domain: sales
    file: enterprise_sales_adr-0001.madr.yaml

  - number: 2
    id: 550e8400-e29b-41d4-a716-446655440001
    title: "Adopt medallion architecture"
    status: proposed
    category: architecture
    domain: null
    file: enterprise_adr-0002.madr.yaml

next_number: 3
```

### 3.2 knowledge.yaml

Central index of all knowledge articles:

```yaml
# Knowledge Base Index
# Auto-generated - do not edit manually

schema_version: "1.0"
last_updated: 2026-01-07T10:00:00Z

articles:
  - number: "KB-0001"
    id: 660e8400-e29b-41d4-a716-446655440000
    title: "Data Classification Guide for Sales Domain"
    article_type: guide
    status: published
    domain: sales
    file: enterprise_sales_kb-0001.kb.yaml

  - number: "KB-0002"
    id: 660e8400-e29b-41d4-a716-446655440001
    title: "PII Handling Procedures"
    article_type: standard
    status: published
    domain: null
    file: enterprise_kb-0002.kb.yaml

next_number: 3
```

---

## Part 4: Sync Integration

### 4.1 SyncEngine Updates

Extend `SyncEngine` to handle decisions and knowledge articles:

```rust
impl<B: DatabaseBackend> SyncEngine<B> {
    pub async fn sync_workspace(
        &self,
        workspace: &Workspace,
        tables: &[Table],
        relationships: &[Relationship],
        domains: &[Domain],
        decisions: &[Decision],      // NEW
        knowledge: &[KnowledgeArticle], // NEW
        force: bool,
    ) -> DatabaseResult<SyncResult>;
}
```

### 4.2 ModelLoader Updates

Extend `ModelLoader` to load decisions and knowledge:

```rust
impl<B: StorageBackend> ModelLoader<B> {
    pub async fn load_decisions(&self, workspace_path: &str)
        -> Result<Vec<Decision>, StorageError>;

    pub async fn load_knowledge(&self, workspace_path: &str)
        -> Result<Vec<KnowledgeArticle>, StorageError>;
}
```

### 4.3 ModelSaver Updates

Extend `ModelSaver` to save decisions and knowledge:

```rust
impl<B: StorageBackend> ModelSaver<B> {
    pub async fn save_decision(&self, workspace_path: &str, decision: &Decision)
        -> Result<(), StorageError>;

    pub async fn save_knowledge(&self, workspace_path: &str, article: &KnowledgeArticle)
        -> Result<(), StorageError>;

    pub async fn update_decision_index(&self, workspace_path: &str, decisions: &[Decision])
        -> Result<(), StorageError>;

    pub async fn update_knowledge_index(&self, workspace_path: &str, articles: &[KnowledgeArticle])
        -> Result<(), StorageError>;
}
```

---

## Part 5: Export Module

### 5.1 Markdown Exporter

New module `src/export/markdown.rs`:

```rust
pub struct MarkdownExporter;

impl MarkdownExporter {
    /// Export a decision to MADR-compliant Markdown
    pub fn export_decision(&self, decision: &Decision) -> Result<String, ExportError>;

    /// Export a knowledge article to Markdown
    pub fn export_knowledge(&self, article: &KnowledgeArticle) -> Result<String, ExportError>;

    /// Export all decisions to a directory
    pub fn export_decisions_to_dir(&self, decisions: &[Decision], output_dir: &Path)
        -> Result<(), ExportError>;

    /// Export all knowledge articles to a directory
    pub fn export_knowledge_to_dir(&self, articles: &[KnowledgeArticle], output_dir: &Path)
        -> Result<(), ExportError>;
}
```

---

## Part 6: Asset Type Extensions

### 6.1 Extend AssetType enum

```rust
pub enum AssetType {
    // Existing
    Workspace,
    Relationships,  // Legacy - now embedded in workspace
    Odcs,
    Odps,
    Cads,
    Bpmn,
    Dmn,
    Openapi,

    // New
    Decision,      // .madr.yaml files
    Knowledge,     // .kb.yaml files
    DecisionIndex, // decisions.yaml
    KnowledgeIndex, // knowledge.yaml
}
```

---

## Implementation Phases

### Phase 1: Core Models (Priority: High)
- Create `src/models/decision.rs`
- Create `src/models/knowledge.rs`
- Extend `src/models/mod.rs` with new exports
- Add `AssetLink` shared type

### Phase 2: Import/Export (Priority: High)
- Create `src/import/decision.rs`
- Create `src/import/knowledge.rs`
- Create `src/export/markdown.rs`
- Extend importers/exporters

### Phase 3: Storage Integration (Priority: High)
- Extend `ModelLoader` with decision/knowledge loading
- Extend `ModelSaver` with decision/knowledge saving
- Add index file management
- Extend `AssetType` enum

### Phase 4: Database Schema (Priority: Medium)
- Add `decisions` table to `schema.rs`
- Add `knowledge_articles` table to `schema.rs`
- Add indexes for querying

### Phase 5: Sync Engine (Priority: Medium)
- Extend `SyncEngine` for decisions
- Extend `SyncEngine` for knowledge
- Add change detection for new file types

### Phase 6: CLI Commands (Priority: Medium)
- Create `src/cli/commands/decision.rs`
- Create `src/cli/commands/knowledge.rs`
- Extend `main.rs` with new commands
- Add interactive mode for creation

### Phase 7: Validation (Priority: Low)
- Create `src/validation/decision.rs`
- Create `src/validation/knowledge.rs`
- Validate asset links
- Validate supersession chains

### Phase 8: Testing & Documentation (Priority: Medium)
- Unit tests for models
- Integration tests for CLI
- Update LLM.txt
- Update README.md
- Add example files

---

## File Changes Summary

### New Files
```
src/models/decision.rs
src/models/knowledge.rs
src/import/decision.rs
src/import/knowledge.rs
src/export/markdown.rs
src/cli/commands/decision.rs
src/cli/commands/knowledge.rs
src/validation/decision.rs
src/validation/knowledge.rs
tests/decision_tests.rs
tests/knowledge_tests.rs
schemas/decision-schema.json
schemas/knowledge-schema.json
examples/decisions.yaml
examples/knowledge.yaml
examples/enterprise_sales_adr-0001.madr.yaml
examples/enterprise_sales_kb-0001.kb.yaml
```

### Modified Files
```
src/models/mod.rs                 # Add decision, knowledge exports
src/models/workspace.rs           # Add AssetType variants
src/import/mod.rs                 # Add decision, knowledge exports
src/export/mod.rs                 # Add markdown export
src/model/loader.rs               # Add load_decisions, load_knowledge
src/model/saver.rs                # Add save_decision, save_knowledge
src/database/schema.rs            # Add decisions, knowledge_articles tables
src/database/sync.rs              # Extend sync for new types
src/cli/main.rs                   # Add decision, knowledge commands
src/cli/commands/mod.rs           # Add new command modules
LLM.txt                           # Document new features
README.md                         # Update documentation
Cargo.toml                        # Version bump
```

---

## Part 7: JSON Schema Validation

Following the existing SDK pattern (see `src/cli/validation.rs`), both formats require JSON Schema definitions for CLI validation.

### 7.1 Decision Schema (`schemas/decision-schema.json`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/pixie79/data-modelling-sdk/schemas/decision-schema.json",
  "title": "MADR Decision Record",
  "description": "Schema for MADR-compliant decision records (.madr.yaml)",
  "type": "object",
  "required": ["id", "number", "title", "status", "category", "date", "context", "decision"],
  "properties": {
    "id": {
      "type": "string",
      "format": "uuid",
      "description": "Unique identifier for the decision"
    },
    "number": {
      "type": "integer",
      "minimum": 1,
      "description": "Sequential decision number (ADR-0001, ADR-0002, etc.)"
    },
    "title": {
      "type": "string",
      "minLength": 1,
      "maxLength": 200,
      "description": "Short title describing the decision"
    },
    "status": {
      "type": "string",
      "enum": ["proposed", "accepted", "deprecated", "superseded"],
      "description": "Current status of the decision"
    },
    "category": {
      "type": "string",
      "enum": ["architecture", "datadesign", "workflow", "model", "governance", "security", "performance", "compliance", "infrastructure", "tooling"],
      "description": "Category of the decision"
    },
    "domain": {
      "type": ["string", "null"],
      "description": "Domain this decision belongs to"
    },
    "date": {
      "type": "string",
      "format": "date-time",
      "description": "Date the decision was made"
    },
    "deciders": {
      "type": "array",
      "items": { "type": "string" },
      "description": "People or teams who made the decision"
    },
    "context": {
      "type": "string",
      "minLength": 1,
      "description": "Problem statement and context for the decision"
    },
    "drivers": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["description"],
        "properties": {
          "description": { "type": "string" },
          "priority": { "type": "string", "enum": ["high", "medium", "low"] }
        }
      },
      "description": "Reasons driving this decision"
    },
    "options": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["name", "selected"],
        "properties": {
          "name": { "type": "string" },
          "description": { "type": "string" },
          "pros": { "type": "array", "items": { "type": "string" } },
          "cons": { "type": "array", "items": { "type": "string" } },
          "selected": { "type": "boolean" }
        }
      },
      "description": "Options considered"
    },
    "decision": {
      "type": "string",
      "minLength": 1,
      "description": "The decision that was made"
    },
    "consequences": {
      "type": "string",
      "description": "Positive and negative consequences of the decision"
    },
    "linked_assets": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["asset_type", "asset_id", "asset_name"],
        "properties": {
          "asset_type": { "type": "string", "enum": ["odcs", "odps", "cads", "relationship"] },
          "asset_id": { "type": "string", "format": "uuid" },
          "asset_name": { "type": "string" },
          "relationship": { "type": "string", "enum": ["affects", "implements", "deprecates"] }
        }
      }
    },
    "supersedes": {
      "type": ["string", "null"],
      "format": "uuid",
      "description": "ID of the decision this supersedes"
    },
    "superseded_by": {
      "type": ["string", "null"],
      "format": "uuid",
      "description": "ID of the decision that superseded this"
    },
    "compliance": {
      "type": "object",
      "properties": {
        "regulatory_impact": { "type": "string" },
        "privacy_assessment": { "type": "string" },
        "security_assessment": { "type": "string" },
        "frameworks": { "type": "array", "items": { "type": "string" } }
      }
    },
    "confirmation_date": {
      "type": ["string", "null"],
      "format": "date-time"
    },
    "confirmation_notes": {
      "type": ["string", "null"]
    },
    "tags": {
      "type": "array",
      "items": { "type": "string" }
    },
    "notes": {
      "type": ["string", "null"]
    },
    "created_at": {
      "type": "string",
      "format": "date-time"
    },
    "updated_at": {
      "type": "string",
      "format": "date-time"
    }
  }
}
```

### 7.2 Knowledge Schema (`schemas/knowledge-schema.json`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/pixie79/data-modelling-sdk/schemas/knowledge-schema.json",
  "title": "Knowledge Base Article",
  "description": "Schema for knowledge base articles (.kb.yaml)",
  "type": "object",
  "required": ["id", "number", "title", "article_type", "status", "summary", "content", "author"],
  "properties": {
    "id": {
      "type": "string",
      "format": "uuid",
      "description": "Unique identifier for the article"
    },
    "number": {
      "type": "string",
      "pattern": "^KB-[0-9]{4}$",
      "description": "Article number (KB-0001, KB-0002, etc.)"
    },
    "title": {
      "type": "string",
      "minLength": 1,
      "maxLength": 200,
      "description": "Article title"
    },
    "article_type": {
      "type": "string",
      "enum": ["guide", "standard", "reference", "glossary", "howto", "troubleshooting", "policy", "template"],
      "description": "Type of knowledge article"
    },
    "status": {
      "type": "string",
      "enum": ["draft", "published", "archived", "deprecated"],
      "description": "Publication status"
    },
    "domain": {
      "type": ["string", "null"],
      "description": "Domain this article belongs to"
    },
    "summary": {
      "type": "string",
      "minLength": 1,
      "maxLength": 500,
      "description": "Brief summary of the article"
    },
    "content": {
      "type": "string",
      "minLength": 1,
      "description": "Full article content in Markdown"
    },
    "author": {
      "type": "string",
      "minLength": 1,
      "description": "Article author (email or name)"
    },
    "reviewers": {
      "type": "array",
      "items": { "type": "string" },
      "description": "List of reviewers"
    },
    "last_reviewed": {
      "type": ["string", "null"],
      "format": "date-time",
      "description": "Date of last review"
    },
    "review_frequency": {
      "type": ["string", "null"],
      "enum": ["monthly", "quarterly", "yearly", null],
      "description": "How often the article should be reviewed"
    },
    "audience": {
      "type": "array",
      "items": { "type": "string" },
      "description": "Target audience for the article"
    },
    "skill_level": {
      "type": ["string", "null"],
      "enum": ["beginner", "intermediate", "advanced", null],
      "description": "Required skill level"
    },
    "linked_assets": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["asset_type", "asset_id", "asset_name"],
        "properties": {
          "asset_type": { "type": "string", "enum": ["odcs", "odps", "cads", "relationship"] },
          "asset_id": { "type": "string", "format": "uuid" },
          "asset_name": { "type": "string" },
          "relationship": { "type": "string" }
        }
      }
    },
    "linked_decisions": {
      "type": "array",
      "items": { "type": "string", "format": "uuid" },
      "description": "UUIDs of related decisions"
    },
    "related_articles": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["article_id", "article_number", "title", "relationship"],
        "properties": {
          "article_id": { "type": "string", "format": "uuid" },
          "article_number": { "type": "string" },
          "title": { "type": "string" },
          "relationship": { "type": "string", "enum": ["related", "prerequisite", "supersedes"] }
        }
      }
    },
    "tags": {
      "type": "array",
      "items": { "type": "string" }
    },
    "notes": {
      "type": ["string", "null"]
    },
    "created_at": {
      "type": "string",
      "format": "date-time"
    },
    "updated_at": {
      "type": "string",
      "format": "date-time"
    }
  }
}
```

### 7.3 Index Schemas

#### decisions-index-schema.json

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/pixie79/data-modelling-sdk/schemas/decisions-index-schema.json",
  "title": "Decision Log Index",
  "type": "object",
  "required": ["schema_version", "decisions", "next_number"],
  "properties": {
    "schema_version": { "type": "string" },
    "last_updated": { "type": "string", "format": "date-time" },
    "decisions": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["number", "id", "title", "status", "category", "file"],
        "properties": {
          "number": { "type": "integer" },
          "id": { "type": "string", "format": "uuid" },
          "title": { "type": "string" },
          "status": { "type": "string" },
          "category": { "type": "string" },
          "domain": { "type": ["string", "null"] },
          "file": { "type": "string" }
        }
      }
    },
    "next_number": { "type": "integer", "minimum": 1 }
  }
}
```

#### knowledge-index-schema.json

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/pixie79/data-modelling-sdk/schemas/knowledge-index-schema.json",
  "title": "Knowledge Base Index",
  "type": "object",
  "required": ["schema_version", "articles", "next_number"],
  "properties": {
    "schema_version": { "type": "string" },
    "last_updated": { "type": "string", "format": "date-time" },
    "articles": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["number", "id", "title", "article_type", "status", "file"],
        "properties": {
          "number": { "type": "string" },
          "id": { "type": "string", "format": "uuid" },
          "title": { "type": "string" },
          "article_type": { "type": "string" },
          "status": { "type": "string" },
          "domain": { "type": ["string", "null"] },
          "file": { "type": "string" }
        }
      }
    },
    "next_number": { "type": "integer", "minimum": 1 }
  }
}
```

### 7.4 CLI Validation Integration

Update `src/cli/validation.rs`:

```rust
/// Validate a decision file against the Decision JSON Schema
#[cfg(feature = "schema-validation")]
pub fn validate_decision(content: &str) -> Result<(), CliError> {
    use jsonschema::Validator;
    use serde_json::Value;

    let schema_content = include_str!("../../schemas/decision-schema.json");
    let schema: Value = serde_json::from_str(schema_content)
        .map_err(|e| CliError::ValidationError(format!("Failed to load decision schema: {}", e)))?;

    let validator = Validator::new(&schema)
        .map_err(|e| CliError::ValidationError(format!("Failed to compile decision schema: {}", e)))?;

    let data: Value = serde_yaml::from_str(content)
        .map_err(|e| CliError::ValidationError(format!("Failed to parse YAML: {}", e)))?;

    if let Err(error) = validator.validate(&data) {
        let error_msg = format_validation_error(&error, "Decision");
        return Err(CliError::ValidationError(error_msg));
    }

    Ok(())
}

/// Validate a knowledge article against the Knowledge JSON Schema
#[cfg(feature = "schema-validation")]
pub fn validate_knowledge(content: &str) -> Result<(), CliError> {
    use jsonschema::Validator;
    use serde_json::Value;

    let schema_content = include_str!("../../schemas/knowledge-schema.json");
    let schema: Value = serde_json::from_str(schema_content)
        .map_err(|e| CliError::ValidationError(format!("Failed to load knowledge schema: {}", e)))?;

    let validator = Validator::new(&schema)
        .map_err(|e| CliError::ValidationError(format!("Failed to compile knowledge schema: {}", e)))?;

    let data: Value = serde_yaml::from_str(content)
        .map_err(|e| CliError::ValidationError(format!("Failed to parse YAML: {}", e)))?;

    if let Err(error) = validator.validate(&data) {
        let error_msg = format_validation_error(&error, "Knowledge");
        return Err(CliError::ValidationError(error_msg));
    }

    Ok(())
}
```

Update `src/cli/commands/validate.rs`:

```rust
pub fn handle_validate(format: &str, input: &str) -> Result<(), CliError> {
    let content = load_input(input)?;

    match format {
        "odcs" => validate_odcs(&content)?,
        "odcl" => validate_odcl(&content)?,
        "odps" => validate_odps(&content)?,
        "cads" => validate_cads(&content)?,
        "decision" => validate_decision(&content)?,   // NEW
        "knowledge" => validate_knowledge(&content)?, // NEW
        "openapi" => validate_openapi(&content)?,
        "protobuf" => validate_protobuf(&content)?,
        "avro" => validate_avro(&content)?,
        "json-schema" => validate_json_schema(&content)?,
        "sql" => validate_sql(&content)?,
        _ => {
            return Err(CliError::InvalidArgument(format!(
                "Unknown format: {}",
                format
            )));
        }
    }

    println!("Validation successful");
    Ok(())
}
```

### 7.5 Import Validation

Both importers should validate against schemas before importing:

```rust
// src/import/decision.rs
impl DecisionImporter {
    pub fn import(&mut self, content: &str) -> Result<Decision, ImportError> {
        // Validate against schema first
        #[cfg(feature = "schema-validation")]
        {
            crate::cli::validation::validate_decision(content)
                .map_err(|e| ImportError::ValidationFailed(e.to_string()))?;
        }

        // Parse and return
        let decision: Decision = serde_yaml::from_str(content)
            .map_err(|e| ImportError::ParseError(e.to_string()))?;

        Ok(decision)
    }
}

// src/import/knowledge.rs
impl KnowledgeImporter {
    pub fn import(&mut self, content: &str) -> Result<KnowledgeArticle, ImportError> {
        // Validate against schema first
        #[cfg(feature = "schema-validation")]
        {
            crate::cli::validation::validate_knowledge(content)
                .map_err(|e| ImportError::ValidationFailed(e.to_string()))?;
        }

        // Parse and return
        let article: KnowledgeArticle = serde_yaml::from_str(content)
            .map_err(|e| ImportError::ParseError(e.to_string()))?;

        Ok(article)
    }
}
```

---

## Implementation Phases

### Phase 1: Core Models (Priority: High)
- Create `src/models/decision.rs`
- Create `src/models/knowledge.rs`
- Extend `src/models/mod.rs` with new exports
- Add `AssetLink` shared type

### Phase 2: JSON Schemas (Priority: High)
- Create `schemas/decision-schema.json`
- Create `schemas/knowledge-schema.json`
- Create `schemas/decisions-index-schema.json`
- Create `schemas/knowledge-index-schema.json`
- Update `schemas/README.md`

### Phase 3: Import/Export with Validation (Priority: High)
- Create `src/import/decision.rs` with schema validation
- Create `src/import/knowledge.rs` with schema validation
- Create `src/export/markdown.rs`
- Extend importers/exporters

### Phase 4: CLI Validation Integration (Priority: High)
- Add `validate_decision()` to `src/cli/validation.rs`
- Add `validate_knowledge()` to `src/cli/validation.rs`
- Add `decision` and `knowledge` to `ValidateFormatArg` enum
- Update `handle_validate()` in `src/cli/commands/validate.rs`

### Phase 5: Storage Integration (Priority: High)
- Extend `ModelLoader` with decision/knowledge loading
- Extend `ModelSaver` with decision/knowledge saving
- Add index file management
- Extend `AssetType` enum

### Phase 6: Database Schema (Priority: Medium)
- Add `decisions` table to `schema.rs`
- Add `knowledge_articles` table to `schema.rs`
- Add indexes for querying

### Phase 7: Sync Engine (Priority: Medium)
- Extend `SyncEngine` for decisions
- Extend `SyncEngine` for knowledge
- Add change detection for new file types

### Phase 8: CLI Commands (Priority: Medium)
- Create `src/cli/commands/decision.rs`
- Create `src/cli/commands/knowledge.rs`
- Extend `main.rs` with new commands
- Add interactive mode for creation

### Phase 9: Business Logic Validation (Priority: Low)
- Create `src/validation/decision.rs` (supersession chains, status transitions)
- Create `src/validation/knowledge.rs` (related article refs, decision links)
- Validate asset links exist

### Phase 10: Testing & Documentation (Priority: Medium)
- Unit tests for models
- Integration tests for CLI
- Schema validation tests
- Update LLM.txt
- Update README.md
- Add example files

---

## File Changes Summary

### New Files
```
src/models/decision.rs
src/models/knowledge.rs
src/import/decision.rs
src/import/knowledge.rs
src/export/markdown.rs
src/cli/commands/decision.rs
src/cli/commands/knowledge.rs
src/validation/decision.rs
src/validation/knowledge.rs
tests/decision_tests.rs
tests/knowledge_tests.rs
schemas/decision-schema.json
schemas/knowledge-schema.json
schemas/decisions-index-schema.json
schemas/knowledge-index-schema.json
examples/decisions.yaml
examples/knowledge.yaml
examples/enterprise_sales_adr-0001.madr.yaml
examples/enterprise_sales_kb-0001.kb.yaml
```

### Modified Files
```
src/models/mod.rs                 # Add decision, knowledge exports
src/models/workspace.rs           # Add AssetType variants
src/import/mod.rs                 # Add decision, knowledge exports
src/export/mod.rs                 # Add markdown export
src/model/loader.rs               # Add load_decisions, load_knowledge
src/model/saver.rs                # Add save_decision, save_knowledge
src/database/schema.rs            # Add decisions, knowledge_articles tables
src/database/sync.rs              # Extend sync for new types
src/cli/main.rs                   # Add decision, knowledge commands, validate formats
src/cli/commands/mod.rs           # Add new command modules
src/cli/commands/validate.rs      # Add decision, knowledge validation
src/cli/validation.rs             # Add validate_decision, validate_knowledge
schemas/README.md                 # Document new schemas
LLM.txt                           # Document new features
README.md                         # Update documentation
Cargo.toml                        # Version bump
```

---

## Success Criteria

1. **YAML Master Record** - All decisions and knowledge stored as YAML in Git
2. **Markdown Export** - CLI can export to readable Markdown for GitHub
3. **JSON Schema Validation** - Both formats validate against JSON Schema on import
4. **CLI Validation** - `data-modelling-cli validate decision` and `validate knowledge` work
5. **Database Sync** - Changes sync to DuckDB/PostgreSQL for querying
6. **Domain Partitioning** - Both DDL and KB support domain-level organization
7. **Asset Linking** - Decisions and knowledge can reference other assets
8. **MADR Compliance** - Decision format follows MADR template
9. **CLI Usability** - All operations available via CLI
10. **Query Support** - Can query via `data-modelling-cli query`
