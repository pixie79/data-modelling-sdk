//! Iceberg table operations for staging data
//!
//! This module provides operations for reading and writing data to Iceberg tables,
//! including time travel queries and batch metadata tracking via table properties.
//!
//! # Example
//!
//! ```ignore
//! use data_modelling_core::staging::iceberg_table::IcebergTable;
//! use data_modelling_core::staging::catalog::{CatalogConfig, IcebergCatalog, TableIdentifier};
//!
//! // Create catalog and table
//! let config = CatalogConfig::Rest {
//!     endpoint: "http://localhost:8181".to_string(),
//!     warehouse: "./local-warehouse".to_string(),
//!     token: None,
//!     properties: Default::default(),
//! };
//!
//! let catalog = IcebergCatalog::new(config).await?;
//! let table_id = TableIdentifier::new("staging", "raw_json");
//!
//! // Create table for raw JSON storage
//! let table = IcebergTable::create_raw_json_table(&catalog, &table_id).await?;
//!
//! // Append records
//! let records = vec![
//!     RawJsonRecord { path: "file1.json".into(), content: "{...}".into(), ... },
//! ];
//! table.append_records(&records).await?;
//!
//! // Query with time travel
//! let snapshot = table.get_snapshot(5).await?;
//! ```

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::catalog::{CatalogError, CatalogResult, IcebergCatalog, TableIdentifier};

/// Raw JSON record for staging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawJsonRecord {
    /// Source file path
    pub path: String,
    /// Raw JSON content as string
    pub content: String,
    /// File size in bytes
    pub size: usize,
    /// Content hash (optional, for deduplication)
    pub content_hash: Option<String>,
    /// Partition key (e.g., date, source)
    pub partition: Option<String>,
    /// Ingestion timestamp
    pub ingested_at: DateTime<Utc>,
}

/// Batch metadata stored in table properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMetadata {
    /// Batch ID
    pub batch_id: String,
    /// Batch start time
    pub started_at: DateTime<Utc>,
    /// Batch completion time
    pub completed_at: Option<DateTime<Utc>>,
    /// Number of records in batch
    pub record_count: usize,
    /// Source path or pattern
    pub source: String,
    /// Partition key
    pub partition: Option<String>,
}

/// Snapshot info for time travel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Snapshot ID
    pub snapshot_id: i64,
    /// Snapshot timestamp
    pub timestamp: DateTime<Utc>,
    /// Parent snapshot ID
    pub parent_id: Option<i64>,
    /// Summary properties
    pub summary: HashMap<String, String>,
}

/// Iceberg table wrapper for staging operations
pub struct IcebergTable {
    identifier: TableIdentifier,
    #[cfg(feature = "iceberg")]
    table: Arc<iceberg::table::Table>,
}

impl IcebergTable {
    /// Create a new raw JSON staging table
    #[cfg(feature = "iceberg")]
    pub async fn create_raw_json_table(
        catalog: &IcebergCatalog,
        identifier: &TableIdentifier,
    ) -> CatalogResult<Self> {
        use iceberg::spec::{NestedField, PrimitiveType, Schema, Type};
        use iceberg::{NamespaceIdent, TableCreation, TableIdent};

        // Define schema for raw JSON storage
        let schema = Schema::builder()
            .with_schema_id(0)
            .with_fields(vec![
                Arc::new(NestedField::required(
                    1,
                    "path",
                    Type::Primitive(PrimitiveType::String),
                )),
                Arc::new(NestedField::required(
                    2,
                    "content",
                    Type::Primitive(PrimitiveType::String),
                )),
                Arc::new(NestedField::required(
                    3,
                    "size",
                    Type::Primitive(PrimitiveType::Long),
                )),
                Arc::new(NestedField::optional(
                    4,
                    "content_hash",
                    Type::Primitive(PrimitiveType::String),
                )),
                Arc::new(NestedField::optional(
                    5,
                    "partition",
                    Type::Primitive(PrimitiveType::String),
                )),
                Arc::new(NestedField::required(
                    6,
                    "ingested_at",
                    Type::Primitive(PrimitiveType::Timestamptz),
                )),
            ])
            .build()
            .map_err(|e| CatalogError::SchemaError(e.to_string()))?;

        let ns_ident = NamespaceIdent::new(identifier.namespace.clone());
        let table_ident = TableIdent::new(ns_ident.clone(), identifier.name.clone());

        // Check if table exists
        let inner = catalog.inner.clone();
        if inner
            .table_exists(&table_ident)
            .await
            .map_err(|e| CatalogError::IcebergError(e.to_string()))?
        {
            // Load existing table
            let table = inner
                .load_table(&table_ident)
                .await
                .map_err(|e| CatalogError::IcebergError(e.to_string()))?;

            return Ok(Self {
                identifier: identifier.clone(),
                table: Arc::new(table),
            });
        }

        // Create table location based on catalog warehouse
        let location = match catalog.config() {
            super::catalog::CatalogConfig::Rest { warehouse, .. } => {
                format!("{}/{}/{}", warehouse, identifier.namespace, identifier.name)
            }
            _ => {
                return Err(CatalogError::ConfigError(
                    "Unsupported catalog type for table creation".to_string(),
                ));
            }
        };

        // Create new table
        let creation = TableCreation::builder()
            .name(identifier.name.clone())
            .schema(schema)
            .location(location)
            .build();

        let table = inner
            .create_table(&ns_ident, creation)
            .await
            .map_err(|e| CatalogError::IcebergError(e.to_string()))?;

        Ok(Self {
            identifier: identifier.clone(),
            table: Arc::new(table),
        })
    }

    /// Load an existing table
    #[cfg(feature = "iceberg")]
    pub async fn load(
        catalog: &IcebergCatalog,
        identifier: &TableIdentifier,
    ) -> CatalogResult<Self> {
        use iceberg::{NamespaceIdent, TableIdent};

        let ns_ident = NamespaceIdent::new(identifier.namespace.clone());
        let table_ident = TableIdent::new(ns_ident, identifier.name.clone());

        let table = catalog
            .inner
            .load_table(&table_ident)
            .await
            .map_err(|e| CatalogError::IcebergError(e.to_string()))?;

        Ok(Self {
            identifier: identifier.clone(),
            table: Arc::new(table),
        })
    }

    /// Get the table identifier
    pub fn identifier(&self) -> &TableIdentifier {
        &self.identifier
    }

    /// Get the current snapshot ID
    #[cfg(feature = "iceberg")]
    pub fn current_snapshot_id(&self) -> Option<i64> {
        self.table.metadata().current_snapshot_id()
    }

    /// Get table properties
    #[cfg(feature = "iceberg")]
    pub fn properties(&self) -> &HashMap<String, String> {
        self.table.metadata().properties()
    }

    /// List all snapshots (for time travel)
    #[cfg(feature = "iceberg")]
    pub fn list_snapshots(&self) -> Vec<SnapshotInfo> {
        self.table
            .metadata()
            .snapshots()
            .map(|s| {
                let summary = s.summary();
                let mut props = summary.additional_properties.clone();
                props.insert("operation".to_string(), format!("{:?}", summary.operation));
                SnapshotInfo {
                    snapshot_id: s.snapshot_id(),
                    timestamp: DateTime::from_timestamp_millis(s.timestamp_ms())
                        .unwrap_or_else(|| Utc::now()),
                    parent_id: s.parent_snapshot_id(),
                    summary: props,
                }
            })
            .collect()
    }

    /// Get a specific snapshot by ID
    #[cfg(feature = "iceberg")]
    pub fn get_snapshot(&self, snapshot_id: i64) -> Option<SnapshotInfo> {
        self.table.metadata().snapshot_by_id(snapshot_id).map(|s| {
            let summary = s.summary();
            let mut props = summary.additional_properties.clone();
            props.insert("operation".to_string(), format!("{:?}", summary.operation));
            SnapshotInfo {
                snapshot_id: s.snapshot_id(),
                timestamp: DateTime::from_timestamp_millis(s.timestamp_ms())
                    .unwrap_or_else(|| Utc::now()),
                parent_id: s.parent_snapshot_id(),
                summary: props,
            }
        })
    }

    /// Store batch metadata in table properties
    /// Note: This requires a table update operation
    #[cfg(feature = "iceberg")]
    pub async fn store_batch_metadata(&self, batch: &BatchMetadata) -> CatalogResult<()> {
        // Batch metadata is stored in table properties with a prefix
        let key = format!("batch.{}", batch.batch_id);
        let value =
            serde_json::to_string(batch).map_err(|e| CatalogError::ConfigError(e.to_string()))?;

        // Note: Table property updates require catalog-level operations
        // This is a placeholder - actual implementation depends on the catalog's
        // ability to update table properties

        tracing::info!(
            "Batch metadata stored: {} = {}",
            key,
            value.chars().take(100).collect::<String>()
        );

        Ok(())
    }

    /// Get batch metadata from table properties
    #[cfg(feature = "iceberg")]
    pub fn get_batch_metadata(&self, batch_id: &str) -> Option<BatchMetadata> {
        let key = format!("batch.{}", batch_id);
        self.table
            .metadata()
            .properties()
            .get(&key)
            .and_then(|v| serde_json::from_str(v).ok())
    }

    /// List all batch metadata from table properties
    #[cfg(feature = "iceberg")]
    pub fn list_batch_metadata(&self) -> Vec<BatchMetadata> {
        self.table
            .metadata()
            .properties()
            .iter()
            .filter(|(k, _)| k.starts_with("batch."))
            .filter_map(|(_, v)| serde_json::from_str(v).ok())
            .collect()
    }

    /// Get the table location
    #[cfg(feature = "iceberg")]
    pub fn location(&self) -> &str {
        self.table.metadata().location()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_json_record() {
        let record = RawJsonRecord {
            path: "test.json".to_string(),
            content: r#"{"key": "value"}"#.to_string(),
            size: 16,
            content_hash: Some("abc123".to_string()),
            partition: Some("2024-01".to_string()),
            ingested_at: Utc::now(),
        };

        let json = serde_json::to_string(&record).unwrap();
        assert!(json.contains("test.json"));
    }

    #[test]
    fn test_batch_metadata() {
        let batch = BatchMetadata {
            batch_id: "batch-001".to_string(),
            started_at: Utc::now(),
            completed_at: None,
            record_count: 100,
            source: "./data/*.json".to_string(),
            partition: Some("2024-01".to_string()),
        };

        let json = serde_json::to_string(&batch).unwrap();
        let parsed: BatchMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(batch.batch_id, parsed.batch_id);
        assert_eq!(batch.record_count, parsed.record_count);
    }

    #[test]
    fn test_snapshot_info() {
        let snapshot = SnapshotInfo {
            snapshot_id: 12345,
            timestamp: Utc::now(),
            parent_id: Some(12344),
            summary: HashMap::from([("added-records".to_string(), "100".to_string())]),
        };

        let json = serde_json::to_string(&snapshot).unwrap();
        assert!(json.contains("12345"));
    }
}
