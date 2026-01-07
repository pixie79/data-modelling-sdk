//! Validate command implementation

use crate::cli::error::CliError;
use crate::cli::validation::{
    validate_avro, validate_cads, validate_decision, validate_decisions_index,
    validate_json_schema, validate_knowledge, validate_knowledge_index, validate_odcl,
    validate_odcs, validate_odps, validate_openapi, validate_protobuf, validate_sql,
};
use std::io::Read;
use std::path::PathBuf;

/// Load input content from file or stdin
fn load_input(input: &str) -> Result<String, CliError> {
    if input == "-" {
        // Read from stdin
        let mut content = String::new();
        std::io::stdin()
            .read_to_string(&mut content)
            .map_err(|e| CliError::InvalidArgument(format!("Failed to read stdin: {}", e)))?;
        Ok(content)
    } else {
        // Read from file
        let path = PathBuf::from(input);
        std::fs::read_to_string(&path).map_err(|e| CliError::FileReadError(path, e.to_string()))
    }
}

/// Handle the validate command
pub fn handle_validate(format: &str, input: &str) -> Result<(), CliError> {
    let content = load_input(input)?;

    match format {
        "odcs" => validate_odcs(&content)?,
        "odcl" => validate_odcl(&content)?,
        "odps" => validate_odps(&content)?,
        "cads" => validate_cads(&content)?,
        "openapi" => validate_openapi(&content)?,
        "protobuf" => validate_protobuf(&content)?,
        "avro" => validate_avro(&content)?,
        "json-schema" => validate_json_schema(&content)?,
        "sql" => validate_sql(&content)?,
        "decision" => validate_decision(&content)?,
        "knowledge" => validate_knowledge(&content)?,
        "decisions-index" => validate_decisions_index(&content)?,
        "knowledge-index" => validate_knowledge_index(&content)?,
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
