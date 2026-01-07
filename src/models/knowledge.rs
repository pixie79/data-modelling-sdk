//! Knowledge Base model for domain-partitioned knowledge articles
//!
//! Implements the Knowledge Base (KB) feature for storing and organizing
//! documentation, guides, standards, and other knowledge resources.
//!
//! ## File Format
//!
//! Knowledge articles are stored as `.kb.yaml` files following the naming convention:
//! `{workspace}_{domain}_kb-{number}.kb.yaml`
//!
//! ## Example
//!
//! ```yaml
//! id: 660e8400-e29b-41d4-a716-446655440000
//! number: "KB-0001"
//! title: "Data Classification Guide"
//! article_type: guide
//! status: published
//! domain: sales
//! summary: |
//!   This guide explains data classification.
//! content: |
//!   ## Overview
//!   Data classification is essential...
//! author: data-governance@company.com
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Tag;
use super::decision::AssetLink;

/// Knowledge article type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum KnowledgeType {
    /// How-to guide or tutorial
    Guide,
    /// Standard or specification
    Standard,
    /// Reference documentation
    Reference,
    /// Glossary of terms
    Glossary,
    /// Step-by-step how-to
    HowTo,
    /// Troubleshooting guide
    Troubleshooting,
    /// Policy document
    Policy,
    /// Template or boilerplate
    Template,
}

impl Default for KnowledgeType {
    fn default() -> Self {
        Self::Guide
    }
}

impl std::fmt::Display for KnowledgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnowledgeType::Guide => write!(f, "Guide"),
            KnowledgeType::Standard => write!(f, "Standard"),
            KnowledgeType::Reference => write!(f, "Reference"),
            KnowledgeType::Glossary => write!(f, "Glossary"),
            KnowledgeType::HowTo => write!(f, "How-To"),
            KnowledgeType::Troubleshooting => write!(f, "Troubleshooting"),
            KnowledgeType::Policy => write!(f, "Policy"),
            KnowledgeType::Template => write!(f, "Template"),
        }
    }
}

/// Knowledge article status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum KnowledgeStatus {
    /// Article is being drafted
    Draft,
    /// Article is published and active
    Published,
    /// Article is archived (historical reference)
    Archived,
    /// Article is deprecated (should not be used)
    Deprecated,
}

impl Default for KnowledgeStatus {
    fn default() -> Self {
        Self::Draft
    }
}

impl std::fmt::Display for KnowledgeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnowledgeStatus::Draft => write!(f, "Draft"),
            KnowledgeStatus::Published => write!(f, "Published"),
            KnowledgeStatus::Archived => write!(f, "Archived"),
            KnowledgeStatus::Deprecated => write!(f, "Deprecated"),
        }
    }
}

/// Review frequency for knowledge articles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReviewFrequency {
    /// Review monthly
    Monthly,
    /// Review quarterly
    Quarterly,
    /// Review yearly
    Yearly,
}

impl std::fmt::Display for ReviewFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewFrequency::Monthly => write!(f, "Monthly"),
            ReviewFrequency::Quarterly => write!(f, "Quarterly"),
            ReviewFrequency::Yearly => write!(f, "Yearly"),
        }
    }
}

/// Skill level required for the article
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SkillLevel {
    /// For beginners
    Beginner,
    /// For intermediate users
    Intermediate,
    /// For advanced users
    Advanced,
}

impl std::fmt::Display for SkillLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillLevel::Beginner => write!(f, "Beginner"),
            SkillLevel::Intermediate => write!(f, "Intermediate"),
            SkillLevel::Advanced => write!(f, "Advanced"),
        }
    }
}

/// Relationship type between articles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ArticleRelationship {
    /// General related article
    Related,
    /// Prerequisite article (should be read first)
    Prerequisite,
    /// This article supersedes the referenced one
    Supersedes,
}

impl std::fmt::Display for ArticleRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArticleRelationship::Related => write!(f, "Related"),
            ArticleRelationship::Prerequisite => write!(f, "Prerequisite"),
            ArticleRelationship::Supersedes => write!(f, "Supersedes"),
        }
    }
}

/// Reference to a related article
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedArticle {
    /// UUID of the related article
    pub article_id: Uuid,
    /// Article number (e.g., "KB-0001")
    pub article_number: String,
    /// Article title
    pub title: String,
    /// Type of relationship
    pub relationship: ArticleRelationship,
}

impl RelatedArticle {
    /// Create a new related article reference
    pub fn new(
        article_id: Uuid,
        article_number: impl Into<String>,
        title: impl Into<String>,
        relationship: ArticleRelationship,
    ) -> Self {
        Self {
            article_id,
            article_number: article_number.into(),
            title: title.into(),
            relationship,
        }
    }
}

/// Knowledge Base Article
///
/// Represents a knowledge article that can be categorized by domain,
/// type, and audience.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeArticle {
    /// Unique identifier for the article
    pub id: Uuid,
    /// Article number (KB-0001, KB-0002, etc.)
    pub number: String,
    /// Article title
    pub title: String,
    /// Type of article
    pub article_type: KnowledgeType,
    /// Publication status
    pub status: KnowledgeStatus,
    /// Domain this article belongs to (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    // Content
    /// Brief summary of the article
    pub summary: String,
    /// Full article content in Markdown
    pub content: String,

    // Authorship
    /// Article author (email or name)
    pub author: String,
    /// List of reviewers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<String>,
    /// Date of last review
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reviewed: Option<DateTime<Utc>>,
    /// How often the article should be reviewed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_frequency: Option<ReviewFrequency>,

    // Classification
    /// Target audience for the article
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<String>,
    /// Required skill level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_level: Option<SkillLevel>,

    // Linking
    /// Assets referenced by this article
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_assets: Vec<AssetLink>,
    /// UUIDs of related decisions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_decisions: Vec<Uuid>,
    /// Related articles
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_articles: Vec<RelatedArticle>,

    // Standard metadata
    /// Tags for categorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
}

impl KnowledgeArticle {
    /// Create a new knowledge article with required fields
    pub fn new(
        number: u32,
        title: impl Into<String>,
        summary: impl Into<String>,
        content: impl Into<String>,
        author: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        let number_str = format!("KB-{:04}", number);
        Self {
            id: Self::generate_id(number),
            number: number_str,
            title: title.into(),
            article_type: KnowledgeType::Guide,
            status: KnowledgeStatus::Draft,
            domain: None,
            summary: summary.into(),
            content: content.into(),
            author: author.into(),
            reviewers: Vec::new(),
            last_reviewed: None,
            review_frequency: None,
            audience: Vec::new(),
            skill_level: None,
            linked_assets: Vec::new(),
            linked_decisions: Vec::new(),
            related_articles: Vec::new(),
            tags: Vec::new(),
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Generate a deterministic UUID for an article based on its number
    pub fn generate_id(number: u32) -> Uuid {
        // Use UUID v5 with a namespace for knowledge articles
        let namespace = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(); // URL namespace
        let name = format!("knowledge:{}", number);
        Uuid::new_v5(&namespace, name.as_bytes())
    }

    /// Parse the numeric part of the article number
    pub fn parse_number(&self) -> Option<u32> {
        self.number.strip_prefix("KB-").and_then(|s| s.parse().ok())
    }

    /// Set the article type
    pub fn with_type(mut self, article_type: KnowledgeType) -> Self {
        self.article_type = article_type;
        self.updated_at = Utc::now();
        self
    }

    /// Set the article status
    pub fn with_status(mut self, status: KnowledgeStatus) -> Self {
        self.status = status;
        self.updated_at = Utc::now();
        self
    }

    /// Set the domain
    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self.updated_at = Utc::now();
        self
    }

    /// Add a reviewer
    pub fn add_reviewer(mut self, reviewer: impl Into<String>) -> Self {
        self.reviewers.push(reviewer.into());
        self.updated_at = Utc::now();
        self
    }

    /// Set review frequency
    pub fn with_review_frequency(mut self, frequency: ReviewFrequency) -> Self {
        self.review_frequency = Some(frequency);
        self.updated_at = Utc::now();
        self
    }

    /// Add an audience
    pub fn add_audience(mut self, audience: impl Into<String>) -> Self {
        self.audience.push(audience.into());
        self.updated_at = Utc::now();
        self
    }

    /// Set skill level
    pub fn with_skill_level(mut self, level: SkillLevel) -> Self {
        self.skill_level = Some(level);
        self.updated_at = Utc::now();
        self
    }

    /// Add an asset link
    pub fn add_asset_link(mut self, link: AssetLink) -> Self {
        self.linked_assets.push(link);
        self.updated_at = Utc::now();
        self
    }

    /// Link to a decision
    pub fn link_decision(mut self, decision_id: Uuid) -> Self {
        if !self.linked_decisions.contains(&decision_id) {
            self.linked_decisions.push(decision_id);
            self.updated_at = Utc::now();
        }
        self
    }

    /// Add a related article
    pub fn add_related_article(mut self, article: RelatedArticle) -> Self {
        self.related_articles.push(article);
        self.updated_at = Utc::now();
        self
    }

    /// Add a tag
    pub fn add_tag(mut self, tag: Tag) -> Self {
        self.tags.push(tag);
        self.updated_at = Utc::now();
        self
    }

    /// Mark the article as reviewed
    pub fn mark_reviewed(&mut self) {
        self.last_reviewed = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Generate the YAML filename for this article
    pub fn filename(&self, workspace_name: &str) -> String {
        let number = self.parse_number().unwrap_or(0);
        match &self.domain {
            Some(domain) => format!(
                "{}_{}_kb-{:04}.kb.yaml",
                sanitize_name(workspace_name),
                sanitize_name(domain),
                number
            ),
            None => format!("{}_kb-{:04}.kb.yaml", sanitize_name(workspace_name), number),
        }
    }

    /// Generate the Markdown filename for this article
    pub fn markdown_filename(&self) -> String {
        let slug = slugify(&self.title);
        format!("{}-{}.md", self.number, slug)
    }

    /// Import from YAML
    pub fn from_yaml(yaml_content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml_content)
    }

    /// Export to YAML
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

/// Knowledge article index entry for the knowledge.yaml file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeIndexEntry {
    /// Article number (e.g., "KB-0001")
    pub number: String,
    /// Article UUID
    pub id: Uuid,
    /// Article title
    pub title: String,
    /// Article type
    pub article_type: KnowledgeType,
    /// Article status
    pub status: KnowledgeStatus,
    /// Domain (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Filename of the article YAML file
    pub file: String,
}

impl From<&KnowledgeArticle> for KnowledgeIndexEntry {
    fn from(article: &KnowledgeArticle) -> Self {
        Self {
            number: article.number.clone(),
            id: article.id,
            title: article.title.clone(),
            article_type: article.article_type.clone(),
            status: article.status.clone(),
            domain: article.domain.clone(),
            file: String::new(), // Set by caller
        }
    }
}

/// Knowledge base index (knowledge.yaml)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeIndex {
    /// Schema version
    pub schema_version: String,
    /// Last update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>,
    /// List of articles
    #[serde(default)]
    pub articles: Vec<KnowledgeIndexEntry>,
    /// Next available article number
    pub next_number: u32,
}

impl Default for KnowledgeIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl KnowledgeIndex {
    /// Create a new empty knowledge index
    pub fn new() -> Self {
        Self {
            schema_version: "1.0".to_string(),
            last_updated: Some(Utc::now()),
            articles: Vec::new(),
            next_number: 1,
        }
    }

    /// Add an article to the index
    pub fn add_article(&mut self, article: &KnowledgeArticle, filename: String) {
        let mut entry = KnowledgeIndexEntry::from(article);
        entry.file = filename;

        // Remove existing entry with same number if present
        self.articles.retain(|a| a.number != article.number);
        self.articles.push(entry);

        // Sort by number
        self.articles.sort_by(|a, b| a.number.cmp(&b.number));

        // Update next number
        if let Some(num) = article.parse_number()
            && num >= self.next_number
        {
            self.next_number = num + 1;
        }

        self.last_updated = Some(Utc::now());
    }

    /// Get the next available article number
    pub fn get_next_number(&self) -> u32 {
        self.next_number
    }

    /// Find an article by number
    pub fn find_by_number(&self, number: &str) -> Option<&KnowledgeIndexEntry> {
        self.articles.iter().find(|a| a.number == number)
    }

    /// Import from YAML
    pub fn from_yaml(yaml_content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml_content)
    }

    /// Export to YAML
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

/// Sanitize a name for use in filenames
fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            ' ' | '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            _ => c,
        })
        .collect::<String>()
        .to_lowercase()
}

/// Create a URL-friendly slug from a title
fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
        .chars()
        .take(50) // Limit slug length
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_article_new() {
        let article = KnowledgeArticle::new(
            1,
            "Data Classification Guide",
            "This guide explains classification",
            "## Overview\n\nData classification is important...",
            "data-governance@example.com",
        );

        assert_eq!(article.number, "KB-0001");
        assert_eq!(article.title, "Data Classification Guide");
        assert_eq!(article.status, KnowledgeStatus::Draft);
        assert_eq!(article.article_type, KnowledgeType::Guide);
    }

    #[test]
    fn test_knowledge_article_builder_pattern() {
        let article = KnowledgeArticle::new(1, "Test", "Summary", "Content", "author@example.com")
            .with_type(KnowledgeType::Standard)
            .with_status(KnowledgeStatus::Published)
            .with_domain("sales")
            .add_reviewer("reviewer@example.com")
            .with_review_frequency(ReviewFrequency::Quarterly)
            .add_audience("data-engineers")
            .with_skill_level(SkillLevel::Intermediate);

        assert_eq!(article.article_type, KnowledgeType::Standard);
        assert_eq!(article.status, KnowledgeStatus::Published);
        assert_eq!(article.domain, Some("sales".to_string()));
        assert_eq!(article.reviewers.len(), 1);
        assert_eq!(article.review_frequency, Some(ReviewFrequency::Quarterly));
        assert_eq!(article.audience.len(), 1);
        assert_eq!(article.skill_level, Some(SkillLevel::Intermediate));
    }

    #[test]
    fn test_knowledge_article_id_generation() {
        let id1 = KnowledgeArticle::generate_id(1);
        let id2 = KnowledgeArticle::generate_id(1);
        let id3 = KnowledgeArticle::generate_id(2);

        // Same number should generate same ID
        assert_eq!(id1, id2);
        // Different numbers should generate different IDs
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_knowledge_article_filename() {
        let article = KnowledgeArticle::new(1, "Test", "Summary", "Content", "author@example.com");
        assert_eq!(article.filename("enterprise"), "enterprise_kb-0001.kb.yaml");

        let article_with_domain = article.with_domain("sales");
        assert_eq!(
            article_with_domain.filename("enterprise"),
            "enterprise_sales_kb-0001.kb.yaml"
        );
    }

    #[test]
    fn test_knowledge_article_markdown_filename() {
        let article = KnowledgeArticle::new(
            1,
            "Data Classification Guide",
            "Summary",
            "Content",
            "author@example.com",
        );
        let filename = article.markdown_filename();
        assert!(filename.starts_with("KB-0001-"));
        assert!(filename.ends_with(".md"));
    }

    #[test]
    fn test_knowledge_article_yaml_roundtrip() {
        let article = KnowledgeArticle::new(
            1,
            "Test Article",
            "Test summary",
            "Test content",
            "author@example.com",
        )
        .with_status(KnowledgeStatus::Published)
        .with_domain("test");

        let yaml = article.to_yaml().unwrap();
        let parsed = KnowledgeArticle::from_yaml(&yaml).unwrap();

        assert_eq!(article.id, parsed.id);
        assert_eq!(article.title, parsed.title);
        assert_eq!(article.status, parsed.status);
        assert_eq!(article.domain, parsed.domain);
    }

    #[test]
    fn test_knowledge_article_parse_number() {
        let article = KnowledgeArticle::new(42, "Test", "Summary", "Content", "author");
        assert_eq!(article.parse_number(), Some(42));
    }

    #[test]
    fn test_knowledge_index() {
        let mut index = KnowledgeIndex::new();
        assert_eq!(index.get_next_number(), 1);

        let article1 =
            KnowledgeArticle::new(1, "First", "Summary", "Content", "author@example.com");
        index.add_article(&article1, "test_kb-0001.kb.yaml".to_string());

        assert_eq!(index.articles.len(), 1);
        assert_eq!(index.get_next_number(), 2);

        let article2 =
            KnowledgeArticle::new(2, "Second", "Summary", "Content", "author@example.com");
        index.add_article(&article2, "test_kb-0002.kb.yaml".to_string());

        assert_eq!(index.articles.len(), 2);
        assert_eq!(index.get_next_number(), 3);
    }

    #[test]
    fn test_related_article() {
        let related = RelatedArticle::new(
            Uuid::new_v4(),
            "KB-0002",
            "PII Handling",
            ArticleRelationship::Related,
        );

        assert_eq!(related.article_number, "KB-0002");
        assert_eq!(related.relationship, ArticleRelationship::Related);
    }

    #[test]
    fn test_knowledge_type_display() {
        assert_eq!(format!("{}", KnowledgeType::Guide), "Guide");
        assert_eq!(format!("{}", KnowledgeType::Standard), "Standard");
        assert_eq!(format!("{}", KnowledgeType::HowTo), "How-To");
    }

    #[test]
    fn test_knowledge_status_display() {
        assert_eq!(format!("{}", KnowledgeStatus::Draft), "Draft");
        assert_eq!(format!("{}", KnowledgeStatus::Published), "Published");
        assert_eq!(format!("{}", KnowledgeStatus::Archived), "Archived");
    }
}
