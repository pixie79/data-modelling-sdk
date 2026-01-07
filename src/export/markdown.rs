//! Markdown exporter for Decision and Knowledge articles
//!
//! Exports Decision and KnowledgeArticle models to Markdown format
//! for easy reading on GitHub and other platforms.

use crate::export::ExportError;
use crate::models::decision::{Decision, DecisionStatus, DriverPriority};
use crate::models::knowledge::{KnowledgeArticle, KnowledgeStatus, KnowledgeType};

/// Markdown exporter for decisions and knowledge articles
pub struct MarkdownExporter;

impl MarkdownExporter {
    /// Create a new Markdown exporter instance
    pub fn new() -> Self {
        Self
    }

    /// Export a decision to MADR-compliant Markdown format
    ///
    /// # Arguments
    ///
    /// * `decision` - The Decision to export
    ///
    /// # Returns
    ///
    /// A Markdown string following MADR template format
    pub fn export_decision(&self, decision: &Decision) -> Result<String, ExportError> {
        let mut md = String::new();

        // Title with status badge
        let status_badge = match decision.status {
            DecisionStatus::Proposed => "🟡 Proposed",
            DecisionStatus::Accepted => "🟢 Accepted",
            DecisionStatus::Deprecated => "🔴 Deprecated",
            DecisionStatus::Superseded => "⚫ Superseded",
        };

        md.push_str(&format!(
            "# ADR-{:04}: {}\n\n",
            decision.number, decision.title
        ));

        // Metadata table
        md.push_str("| Property | Value |\n");
        md.push_str("|----------|-------|\n");
        md.push_str(&format!("| **Status** | {} |\n", status_badge));
        md.push_str(&format!("| **Category** | {} |\n", decision.category));
        if let Some(domain) = &decision.domain {
            md.push_str(&format!("| **Domain** | {} |\n", domain));
        }
        md.push_str(&format!(
            "| **Date** | {} |\n",
            decision.date.format("%Y-%m-%d")
        ));
        if !decision.deciders.is_empty() {
            md.push_str(&format!(
                "| **Deciders** | {} |\n",
                decision.deciders.join(", ")
            ));
        }
        md.push('\n');

        // Context section
        md.push_str("## Context\n\n");
        md.push_str(&decision.context);
        md.push_str("\n\n");

        // Decision Drivers section (if any)
        if !decision.drivers.is_empty() {
            md.push_str("## Decision Drivers\n\n");
            for driver in &decision.drivers {
                let priority_str = match &driver.priority {
                    Some(DriverPriority::High) => " *(High Priority)*",
                    Some(DriverPriority::Medium) => " *(Medium Priority)*",
                    Some(DriverPriority::Low) => " *(Low Priority)*",
                    None => "",
                };
                md.push_str(&format!("- {}{}\n", driver.description, priority_str));
            }
            md.push('\n');
        }

        // Considered Options section (if any)
        if !decision.options.is_empty() {
            md.push_str("## Considered Options\n\n");
            for option in &decision.options {
                let selected_marker = if option.selected { " ✓" } else { "" };
                md.push_str(&format!("### {}{}\n\n", option.name, selected_marker));

                if let Some(desc) = &option.description {
                    md.push_str(desc);
                    md.push_str("\n\n");
                }

                if !option.pros.is_empty() {
                    md.push_str("**Pros:**\n");
                    for pro in &option.pros {
                        md.push_str(&format!("- ✅ {}\n", pro));
                    }
                    md.push('\n');
                }

                if !option.cons.is_empty() {
                    md.push_str("**Cons:**\n");
                    for con in &option.cons {
                        md.push_str(&format!("- ❌ {}\n", con));
                    }
                    md.push('\n');
                }
            }
        }

        // Decision section
        md.push_str("## Decision\n\n");
        md.push_str(&decision.decision);
        md.push_str("\n\n");

        // Consequences section (if any)
        if let Some(consequences) = &decision.consequences {
            md.push_str("## Consequences\n\n");
            md.push_str(consequences);
            md.push_str("\n\n");
        }

        // Linked Assets section (if any)
        if !decision.linked_assets.is_empty() {
            md.push_str("## Linked Assets\n\n");
            md.push_str("| Asset Type | Asset Name | Relationship |\n");
            md.push_str("|------------|------------|---------------|\n");
            for link in &decision.linked_assets {
                let rel_str = link
                    .relationship
                    .as_ref()
                    .map(|r| format!("{:?}", r))
                    .unwrap_or_else(|| "-".to_string());
                md.push_str(&format!(
                    "| {} | {} | {} |\n",
                    link.asset_type, link.asset_name, rel_str
                ));
            }
            md.push('\n');
        }

        // Supersession info
        if let Some(supersedes) = &decision.supersedes {
            md.push_str(&format!(
                "> **Note:** This decision supersedes `{}`\n\n",
                supersedes
            ));
        }
        if let Some(superseded_by) = &decision.superseded_by {
            md.push_str(&format!(
                "> **Warning:** This decision has been superseded by `{}`\n\n",
                superseded_by
            ));
        }

        // Compliance section (if any)
        if let Some(compliance) = &decision.compliance {
            md.push_str("## Compliance Assessment\n\n");
            if let Some(reg) = &compliance.regulatory_impact {
                md.push_str(&format!("**Regulatory Impact:** {}\n\n", reg));
            }
            if let Some(priv_assess) = &compliance.privacy_assessment {
                md.push_str(&format!("**Privacy Assessment:** {}\n\n", priv_assess));
            }
            if let Some(sec_assess) = &compliance.security_assessment {
                md.push_str(&format!("**Security Assessment:** {}\n\n", sec_assess));
            }
            if !compliance.frameworks.is_empty() {
                md.push_str(&format!(
                    "**Frameworks:** {}\n\n",
                    compliance.frameworks.join(", ")
                ));
            }
        }

        // Tags (if any)
        if !decision.tags.is_empty() {
            let tags_str: Vec<String> = decision.tags.iter().map(|t| format!("`{}`", t)).collect();
            md.push_str(&format!("**Tags:** {}\n\n", tags_str.join(" ")));
        }

        // Footer with timestamps
        md.push_str("---\n\n");
        md.push_str(&format!(
            "*Created: {} | Last Updated: {}*\n",
            decision.created_at.format("%Y-%m-%d %H:%M UTC"),
            decision.updated_at.format("%Y-%m-%d %H:%M UTC")
        ));

        if let Some(conf_date) = &decision.confirmation_date {
            md.push_str(&format!(
                "\n*Last Confirmed: {}*",
                conf_date.format("%Y-%m-%d")
            ));
            if let Some(notes) = &decision.confirmation_notes {
                md.push_str(&format!(" - {}", notes));
            }
            md.push('\n');
        }

        Ok(md)
    }

    /// Export a knowledge article to Markdown format
    ///
    /// # Arguments
    ///
    /// * `article` - The KnowledgeArticle to export
    ///
    /// # Returns
    ///
    /// A Markdown string
    pub fn export_knowledge(&self, article: &KnowledgeArticle) -> Result<String, ExportError> {
        let mut md = String::new();

        // Title with type badge
        let type_badge = match article.article_type {
            KnowledgeType::Guide => "📖 Guide",
            KnowledgeType::Standard => "📋 Standard",
            KnowledgeType::Reference => "📚 Reference",
            KnowledgeType::Glossary => "📝 Glossary",
            KnowledgeType::HowTo => "🔧 How-To",
            KnowledgeType::Troubleshooting => "🔍 Troubleshooting",
            KnowledgeType::Policy => "⚖️ Policy",
            KnowledgeType::Template => "📄 Template",
        };

        let status_badge = match article.status {
            KnowledgeStatus::Draft => "🟡 Draft",
            KnowledgeStatus::Published => "🟢 Published",
            KnowledgeStatus::Archived => "📦 Archived",
            KnowledgeStatus::Deprecated => "🔴 Deprecated",
        };

        md.push_str(&format!("# {}: {}\n\n", article.number, article.title));

        // Metadata table
        md.push_str("| Property | Value |\n");
        md.push_str("|----------|-------|\n");
        md.push_str(&format!("| **Type** | {} |\n", type_badge));
        md.push_str(&format!("| **Status** | {} |\n", status_badge));
        if let Some(domain) = &article.domain {
            md.push_str(&format!("| **Domain** | {} |\n", domain));
        }
        md.push_str(&format!("| **Author** | {} |\n", article.author));
        if let Some(skill) = &article.skill_level {
            md.push_str(&format!("| **Skill Level** | {} |\n", skill));
        }
        if !article.audience.is_empty() {
            md.push_str(&format!(
                "| **Audience** | {} |\n",
                article.audience.join(", ")
            ));
        }
        md.push('\n');

        // Summary section
        md.push_str("## Summary\n\n");
        md.push_str(&article.summary);
        md.push_str("\n\n");

        // Main content (already in Markdown)
        md.push_str(&article.content);
        md.push_str("\n\n");

        // Linked Decisions section (if any)
        if !article.linked_decisions.is_empty() {
            md.push_str("## Related Decisions\n\n");
            for decision_id in &article.linked_decisions {
                md.push_str(&format!("- `{}`\n", decision_id));
            }
            md.push('\n');
        }

        // Linked Assets section (if any)
        if !article.linked_assets.is_empty() {
            md.push_str("## Linked Assets\n\n");
            md.push_str("| Asset Type | Asset Name | Relationship |\n");
            md.push_str("|------------|------------|---------------|\n");
            for link in &article.linked_assets {
                let rel_str = link
                    .relationship
                    .as_ref()
                    .map(|r| format!("{:?}", r))
                    .unwrap_or_else(|| "-".to_string());
                md.push_str(&format!(
                    "| {} | {} | {} |\n",
                    link.asset_type, link.asset_name, rel_str
                ));
            }
            md.push('\n');
        }

        // Related Articles section (if any)
        if !article.related_articles.is_empty() {
            md.push_str("## Related Articles\n\n");
            for related in &article.related_articles {
                md.push_str(&format!(
                    "- **{}**: {} ({})\n",
                    related.article_number, related.title, related.relationship
                ));
            }
            md.push('\n');
        }

        // Tags (if any)
        if !article.tags.is_empty() {
            let tags_str: Vec<String> = article.tags.iter().map(|t| format!("`{}`", t)).collect();
            md.push_str(&format!("**Tags:** {}\n\n", tags_str.join(" ")));
        }

        // Footer with review info
        md.push_str("---\n\n");

        if !article.reviewers.is_empty() {
            md.push_str(&format!(
                "*Reviewers: {}*\n\n",
                article.reviewers.join(", ")
            ));
        }

        if let Some(last_reviewed) = &article.last_reviewed {
            md.push_str(&format!(
                "*Last Reviewed: {}*",
                last_reviewed.format("%Y-%m-%d")
            ));
            if let Some(freq) = &article.review_frequency {
                md.push_str(&format!(" (Review Frequency: {})", freq));
            }
            md.push_str("\n\n");
        }

        md.push_str(&format!(
            "*Created: {} | Last Updated: {}*\n",
            article.created_at.format("%Y-%m-%d %H:%M UTC"),
            article.updated_at.format("%Y-%m-%d %H:%M UTC")
        ));

        Ok(md)
    }

    /// Export decisions to a directory as Markdown files
    ///
    /// # Arguments
    ///
    /// * `decisions` - The decisions to export
    /// * `dir_path` - Directory to export to (e.g., "decisions/")
    ///
    /// # Returns
    ///
    /// A Result with the number of files exported, or an ExportError
    pub fn export_decisions_to_directory(
        &self,
        decisions: &[Decision],
        dir_path: &std::path::Path,
    ) -> Result<usize, ExportError> {
        // Create directory if it doesn't exist
        if !dir_path.exists() {
            std::fs::create_dir_all(dir_path)
                .map_err(|e| ExportError::IoError(format!("Failed to create directory: {}", e)))?;
        }

        let mut count = 0;
        for decision in decisions {
            let filename = decision.markdown_filename();
            let path = dir_path.join(&filename);
            let md = self.export_decision(decision)?;
            std::fs::write(&path, md).map_err(|e| {
                ExportError::IoError(format!("Failed to write {}: {}", filename, e))
            })?;
            count += 1;
        }

        Ok(count)
    }

    /// Export knowledge articles to a directory as Markdown files
    ///
    /// # Arguments
    ///
    /// * `articles` - The articles to export
    /// * `dir_path` - Directory to export to (e.g., "knowledge/")
    ///
    /// # Returns
    ///
    /// A Result with the number of files exported, or an ExportError
    pub fn export_knowledge_to_directory(
        &self,
        articles: &[KnowledgeArticle],
        dir_path: &std::path::Path,
    ) -> Result<usize, ExportError> {
        // Create directory if it doesn't exist
        if !dir_path.exists() {
            std::fs::create_dir_all(dir_path)
                .map_err(|e| ExportError::IoError(format!("Failed to create directory: {}", e)))?;
        }

        let mut count = 0;
        for article in articles {
            let filename = article.markdown_filename();
            let path = dir_path.join(&filename);
            let md = self.export_knowledge(article)?;
            std::fs::write(&path, md).map_err(|e| {
                ExportError::IoError(format!("Failed to write {}: {}", filename, e))
            })?;
            count += 1;
        }

        Ok(count)
    }

    /// Export knowledge articles organized by domain
    ///
    /// Creates subdirectories for each domain.
    ///
    /// # Arguments
    ///
    /// * `articles` - The articles to export
    /// * `base_dir` - Base directory (e.g., "knowledge/")
    ///
    /// # Returns
    ///
    /// A Result with the number of files exported, or an ExportError
    pub fn export_knowledge_by_domain(
        &self,
        articles: &[KnowledgeArticle],
        base_dir: &std::path::Path,
    ) -> Result<usize, ExportError> {
        // Create base directory if it doesn't exist
        if !base_dir.exists() {
            std::fs::create_dir_all(base_dir)
                .map_err(|e| ExportError::IoError(format!("Failed to create directory: {}", e)))?;
        }

        let mut count = 0;
        for article in articles {
            // Determine subdirectory based on domain
            let subdir = if let Some(domain) = &article.domain {
                base_dir.join(domain)
            } else {
                base_dir.join("general")
            };

            if !subdir.exists() {
                std::fs::create_dir_all(&subdir).map_err(|e| {
                    ExportError::IoError(format!("Failed to create directory: {}", e))
                })?;
            }

            let filename = article.markdown_filename();
            let path = subdir.join(&filename);
            let md = self.export_knowledge(article)?;
            std::fs::write(&path, md).map_err(|e| {
                ExportError::IoError(format!("Failed to write {}: {}", filename, e))
            })?;
            count += 1;
        }

        Ok(count)
    }

    /// Generate a decisions index page in Markdown
    ///
    /// Creates a summary page listing all decisions with links.
    pub fn generate_decisions_index(&self, decisions: &[Decision]) -> String {
        let mut md = String::new();

        md.push_str("# Architecture Decision Records\n\n");
        md.push_str("This directory contains all Architecture Decision Records (ADRs) for this project.\n\n");

        // Group by status
        let accepted: Vec<_> = decisions
            .iter()
            .filter(|d| d.status == DecisionStatus::Accepted)
            .collect();
        let proposed: Vec<_> = decisions
            .iter()
            .filter(|d| d.status == DecisionStatus::Proposed)
            .collect();
        let deprecated: Vec<_> = decisions
            .iter()
            .filter(|d| d.status == DecisionStatus::Deprecated)
            .collect();
        let superseded: Vec<_> = decisions
            .iter()
            .filter(|d| d.status == DecisionStatus::Superseded)
            .collect();

        // Summary table
        md.push_str("## Summary\n\n");
        md.push_str(&format!(
            "| Status | Count |\n|--------|-------|\n| 🟢 Accepted | {} |\n| 🟡 Proposed | {} |\n| 🔴 Deprecated | {} |\n| ⚫ Superseded | {} |\n\n",
            accepted.len(), proposed.len(), deprecated.len(), superseded.len()
        ));

        // Decision list
        md.push_str("## Decisions\n\n");
        md.push_str("| Number | Title | Status | Category | Date |\n");
        md.push_str("|--------|-------|--------|----------|------|\n");

        for decision in decisions {
            let status_icon = match decision.status {
                DecisionStatus::Proposed => "🟡",
                DecisionStatus::Accepted => "🟢",
                DecisionStatus::Deprecated => "🔴",
                DecisionStatus::Superseded => "⚫",
            };
            let filename = decision.markdown_filename();
            md.push_str(&format!(
                "| [ADR-{:04}]({}) | {} | {} | {} | {} |\n",
                decision.number,
                filename,
                decision.title,
                status_icon,
                decision.category,
                decision.date.format("%Y-%m-%d")
            ));
        }

        md
    }

    /// Generate a knowledge index page in Markdown
    ///
    /// Creates a summary page listing all articles with links.
    pub fn generate_knowledge_index(&self, articles: &[KnowledgeArticle]) -> String {
        let mut md = String::new();

        md.push_str("# Knowledge Base\n\n");
        md.push_str("This directory contains all Knowledge Base articles for this project.\n\n");

        // Group by domain
        let mut domains: std::collections::HashMap<String, Vec<&KnowledgeArticle>> =
            std::collections::HashMap::new();
        for article in articles {
            let domain = article
                .domain
                .clone()
                .unwrap_or_else(|| "General".to_string());
            domains.entry(domain).or_default().push(article);
        }

        // Articles by domain
        let mut domain_keys: Vec<_> = domains.keys().collect();
        domain_keys.sort();

        for domain in domain_keys {
            let domain_articles = &domains[domain];
            md.push_str(&format!("## {}\n\n", domain));

            md.push_str("| Number | Title | Type | Status |\n");
            md.push_str("|--------|-------|------|--------|\n");

            for article in domain_articles.iter() {
                let type_icon = match article.article_type {
                    KnowledgeType::Guide => "📖",
                    KnowledgeType::Standard => "📋",
                    KnowledgeType::Reference => "📚",
                    KnowledgeType::Glossary => "📝",
                    KnowledgeType::HowTo => "🔧",
                    KnowledgeType::Troubleshooting => "🔍",
                    KnowledgeType::Policy => "⚖️",
                    KnowledgeType::Template => "📄",
                };
                let status_icon = match article.status {
                    KnowledgeStatus::Draft => "🟡",
                    KnowledgeStatus::Published => "🟢",
                    KnowledgeStatus::Archived => "📦",
                    KnowledgeStatus::Deprecated => "🔴",
                };
                let filename = article.markdown_filename();
                let link_path = if article.domain.is_some() {
                    format!("{}/{}", domain.to_lowercase(), filename)
                } else {
                    format!("general/{}", filename)
                };
                md.push_str(&format!(
                    "| [{}]({}) | {} | {} | {} |\n",
                    article.number, link_path, article.title, type_icon, status_icon
                ));
            }

            md.push('\n');
        }

        md
    }
}

impl Default for MarkdownExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::decision::{DecisionCategory, DecisionDriver, DecisionOption};

    #[test]
    fn test_export_decision_markdown() {
        let decision = Decision::new(
            1,
            "Use ODCS Format for Data Contracts",
            "We need a standard format for defining data contracts across teams.",
            "Use ODCS v3.1.0 as our data contract format.",
        )
        .with_status(DecisionStatus::Accepted)
        .with_category(DecisionCategory::DataDesign)
        .with_domain("platform")
        .add_driver(DecisionDriver::with_priority(
            "Need standardization",
            DriverPriority::High,
        ))
        .add_option(DecisionOption::with_details(
            "ODCS",
            "Open Data Contract Standard",
            vec!["Industry standard".to_string()],
            vec!["Learning curve".to_string()],
            true,
        ))
        .with_consequences("All teams must migrate to ODCS format.");

        let exporter = MarkdownExporter::new();
        let result = exporter.export_decision(&decision);
        assert!(result.is_ok());

        let md = result.unwrap();
        assert!(md.contains("# ADR-0001: Use ODCS Format for Data Contracts"));
        assert!(md.contains("🟢 Accepted"));
        assert!(md.contains("## Context"));
        assert!(md.contains("## Decision Drivers"));
        assert!(md.contains("## Considered Options"));
        assert!(md.contains("## Decision"));
        assert!(md.contains("## Consequences"));
    }

    #[test]
    fn test_export_knowledge_markdown() {
        let article = KnowledgeArticle::new(
            1,
            "Data Classification Guide",
            "This guide explains how to classify data.",
            "## Introduction\n\nData classification is important...",
            "data-governance@example.com",
        )
        .with_status(KnowledgeStatus::Published)
        .with_domain("governance");

        let exporter = MarkdownExporter::new();
        let result = exporter.export_knowledge(&article);
        assert!(result.is_ok());

        let md = result.unwrap();
        assert!(md.contains("# KB-0001: Data Classification Guide"));
        assert!(md.contains("🟢 Published"));
        assert!(md.contains("## Summary"));
        assert!(md.contains("## Introduction"));
    }

    #[test]
    fn test_generate_decisions_index() {
        let decisions = vec![
            Decision::new(1, "First Decision", "Context", "Decision")
                .with_status(DecisionStatus::Accepted),
            Decision::new(2, "Second Decision", "Context", "Decision")
                .with_status(DecisionStatus::Proposed),
        ];

        let exporter = MarkdownExporter::new();
        let index = exporter.generate_decisions_index(&decisions);

        assert!(index.contains("# Architecture Decision Records"));
        assert!(index.contains("ADR-0001"));
        assert!(index.contains("ADR-0002"));
        assert!(index.contains("🟢 Accepted | 1"));
        assert!(index.contains("🟡 Proposed | 1"));
    }
}
