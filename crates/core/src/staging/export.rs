//! Export staging data to production catalogs
//!
//! This module provides functionality to export data from local Iceberg tables
//! to production catalogs like Unity Catalog, AWS Glue, and S3 Tables.
//!
//! # Example
//!
//! ```ignore
//! use data_modelling_core::staging::export::{ExportConfig, ExportTarget, export_to_catalog};
//!
//! let config = ExportConfig {
//!     target: ExportTarget::Unity {
//!         endpoint: "https://workspace.cloud.databricks.com".to_string(),
//!         token: "dapi...".to_string(),
//!         catalog: "main".to_string(),
//!         schema: "staging".to_string(),
//!     },
//!     table_name: "raw_json".to_string(),
//!     overwrite: false,
//! };
//!
//! let result = export_to_catalog(&source_table, &catalog, &config).await?;
//! println!("Exported {} files ({} bytes)", result.files_exported, result.bytes_exported);
//! ```

use serde::{Deserialize, Serialize};

use super::catalog::{CatalogError, CatalogResult};

/// Target catalog for export
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExportTarget {
    /// Databricks Unity Catalog
    Unity {
        /// Unity Catalog endpoint URL
        endpoint: String,
        /// OAuth or PAT token
        token: String,
        /// Target catalog name
        catalog: String,
        /// Target schema name
        schema: String,
    },

    /// AWS Glue Data Catalog
    Glue {
        /// AWS region
        region: String,
        /// Target database name
        database: String,
        /// Optional AWS profile
        profile: Option<String>,
    },

    /// AWS S3 Tables
    S3Tables {
        /// S3 Tables bucket ARN
        arn: String,
        /// AWS region
        region: String,
        /// Optional AWS profile
        profile: Option<String>,
    },

    /// Local filesystem (for testing/development)
    Local {
        /// Base path for exported files
        path: String,
    },
}

/// Configuration for export operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Target catalog
    pub target: ExportTarget,
    /// Target table name
    pub table_name: String,
    /// Whether to overwrite existing table
    pub overwrite: bool,
    /// Optional partition filter (only export specific partitions)
    pub partition_filter: Option<String>,
}

/// Result of an export operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    /// Number of data files exported
    pub files_exported: usize,
    /// Total bytes exported
    pub bytes_exported: u64,
    /// Target location where files were written
    pub target_location: String,
    /// Whether a new table was created
    pub table_created: bool,
}

/// Export data from a local Iceberg table to a production catalog
///
/// This function:
/// 1. Reads data files from the source Iceberg table
/// 2. Copies Parquet files to the target storage location
/// 3. Registers the table in the target catalog
///
/// For Unity Catalog and Glue, this creates an external table pointing
/// to the exported Parquet files. For S3 Tables, it creates a managed table.
#[cfg(feature = "iceberg")]
pub async fn export_to_catalog(
    source_table: &super::iceberg_table::IcebergTable,
    _source_catalog: &super::IcebergCatalog,
    config: &ExportConfig,
) -> CatalogResult<ExportResult> {
    match &config.target {
        ExportTarget::Local { path } => export_to_local(source_table, path, config).await,
        ExportTarget::Unity { .. } => {
            // Unity Catalog export requires additional setup
            Err(CatalogError::ConfigError(
                "Unity Catalog export not yet implemented. Use local export for now.".to_string(),
            ))
        }
        ExportTarget::Glue { .. } => {
            // Glue export requires AWS SDK integration
            Err(CatalogError::ConfigError(
                "Glue export not yet implemented. Use local export for now.".to_string(),
            ))
        }
        ExportTarget::S3Tables { .. } => {
            // S3 Tables export requires AWS SDK integration
            Err(CatalogError::ConfigError(
                "S3 Tables export not yet implemented. Use local export for now.".to_string(),
            ))
        }
    }
}

/// Export to local filesystem (copies Parquet files)
#[cfg(feature = "iceberg")]
async fn export_to_local(
    source_table: &super::iceberg_table::IcebergTable,
    target_path: &str,
    config: &ExportConfig,
) -> CatalogResult<ExportResult> {
    use std::fs;
    use std::path::Path;

    let source_location = source_table.location();
    let source_data_dir = Path::new(source_location).join("data");

    // Create target directory
    let target_dir = Path::new(target_path).join(&config.table_name).join("data");
    fs::create_dir_all(&target_dir).map_err(|e| {
        CatalogError::IoError(format!(
            "Failed to create target directory {}: {}",
            target_dir.display(),
            e
        ))
    })?;

    let mut files_exported = 0;
    let mut bytes_exported = 0u64;

    // Copy all Parquet files from source to target
    if source_data_dir.exists() {
        for entry in fs::read_dir(&source_data_dir).map_err(|e| {
            CatalogError::IoError(format!(
                "Failed to read source directory {}: {}",
                source_data_dir.display(),
                e
            ))
        })? {
            let entry = entry.map_err(|e| CatalogError::IoError(e.to_string()))?;
            let path = entry.path();

            if path.extension().map(|e| e == "parquet").unwrap_or(false) {
                let file_name = path.file_name().unwrap();
                let target_file = target_dir.join(file_name);

                let metadata = fs::metadata(&path).map_err(|e| {
                    CatalogError::IoError(format!(
                        "Failed to get metadata for {}: {}",
                        path.display(),
                        e
                    ))
                })?;

                fs::copy(&path, &target_file).map_err(|e| {
                    CatalogError::IoError(format!(
                        "Failed to copy {} to {}: {}",
                        path.display(),
                        target_file.display(),
                        e
                    ))
                })?;

                files_exported += 1;
                bytes_exported += metadata.len();

                tracing::info!(
                    "Exported {} ({} bytes)",
                    file_name.to_string_lossy(),
                    metadata.len()
                );
            }
        }
    }

    Ok(ExportResult {
        files_exported,
        bytes_exported,
        target_location: target_dir.display().to_string(),
        table_created: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_target_serialize_unity() {
        let target = ExportTarget::Unity {
            endpoint: "https://workspace.cloud.databricks.com".to_string(),
            token: "dapi123".to_string(),
            catalog: "main".to_string(),
            schema: "staging".to_string(),
        };

        let json = serde_json::to_string(&target).unwrap();
        assert!(json.contains("unity"));
        assert!(json.contains("databricks.com"));
        assert!(json.contains("main"));
    }

    #[test]
    fn test_export_target_serialize_glue() {
        let target = ExportTarget::Glue {
            region: "us-east-1".to_string(),
            database: "staging_db".to_string(),
            profile: Some("production".to_string()),
        };

        let json = serde_json::to_string(&target).unwrap();
        assert!(json.contains("glue"));
        assert!(json.contains("us-east-1"));
        assert!(json.contains("staging_db"));
    }

    #[test]
    fn test_export_target_serialize_s3_tables() {
        let target = ExportTarget::S3Tables {
            arn: "arn:aws:s3tables:us-east-1:123456789:bucket/my-bucket".to_string(),
            region: "us-east-1".to_string(),
            profile: None,
        };

        let json = serde_json::to_string(&target).unwrap();
        assert!(json.contains("s3_tables"));
        assert!(json.contains("arn:aws:s3tables"));
    }

    #[test]
    fn test_export_target_serialize_local() {
        let target = ExportTarget::Local {
            path: "/tmp/export".to_string(),
        };

        let json = serde_json::to_string(&target).unwrap();
        assert!(json.contains("local"));
        assert!(json.contains("/tmp/export"));
    }

    #[test]
    fn test_export_config_serialize() {
        let config = ExportConfig {
            target: ExportTarget::Local {
                path: "./export".to_string(),
            },
            table_name: "raw_json".to_string(),
            overwrite: true,
            partition_filter: Some("partition = '2024-01'".to_string()),
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("raw_json"));
        assert!(json.contains("overwrite"));
        assert!(json.contains("partition_filter"));
    }

    #[test]
    fn test_export_result_serialize() {
        let result = ExportResult {
            files_exported: 10,
            bytes_exported: 1024 * 1024 * 100, // 100 MB
            target_location: "s3://bucket/path".to_string(),
            table_created: true,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("10"));
        assert!(json.contains("104857600"));
        assert!(json.contains("s3://bucket/path"));
    }

    #[test]
    fn test_export_target_deserialize() {
        let json = r#"{
            "type": "unity",
            "endpoint": "https://test.databricks.com",
            "token": "secret",
            "catalog": "prod",
            "schema": "analytics"
        }"#;

        let target: ExportTarget = serde_json::from_str(json).unwrap();
        match target {
            ExportTarget::Unity {
                endpoint,
                catalog,
                schema,
                ..
            } => {
                assert_eq!(endpoint, "https://test.databricks.com");
                assert_eq!(catalog, "prod");
                assert_eq!(schema, "analytics");
            }
            _ => panic!("Expected Unity target"),
        }
    }
}
