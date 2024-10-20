use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

// Structs to line up with the QuestConfig structure in rqst.toml
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct QuestConfig {
    title: String,
    author: String,
    repo: String,
    stages: Vec<Stage>,
    read_only: Option<Vec<String>>,
    r#final: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stage {
    label: String,
    description: String,
    // What are the other required parts of a stage? branch_name?
}

fn validate_quest_config(config: &QuestConfig) -> Result<()> {
    // First check for empty fields
    if config.title.trim().is_empty() {
        anyhow::bail!("Validation Error: title cannot be empty.");
    }
    if config.author.trim().is_empty() {
        anyhow::bail!("Validation Error: author cannot be empty.");
    }
    if config.repo.trim().is_empty() {
        anyhow::bail!("Validation Error: repo cannot be empty.");
    }
    if config.stages.is_empty() {
        anyhow::bail!("Validation Error: at least one stage must be defined.");
    }

    for (i, stage) in config.stages.iter().enumerate() {
        if stage.label.trim().is_empty() {
            anyhow::bail!("Validation Error: stage {} label cannot be empty.", i + 1);
        }
        if stage.description.trim().is_empty() {
            anyhow::bail!(
                "Validation Error: stage {} description cannot be empty.",
                i + 1
            );
        }
    }

    // Read-only field is optional, so no validation needed right?
    // Final field is optional, so no validation needed right?

    println!("Validation passed.");
    Ok(())
}

fn read_quest_config(file_path: &str) -> Result<QuestConfig> {
    let content =
        fs::read_to_string(Path::new(file_path)).context("Failed to read rqst.toml file")?;
    let config: QuestConfig = toml::from_str(&content).context("Failed to parse rqst.toml")?;
    Ok(config)
}

fn main() -> Result<()> {
    // Path to the rqst.toml file
    let file_path = "rqst.toml"; // Not sure where this should actually go

    let config = read_quest_config(file_path).context("Error loading quest config")?;

    validate_quest_config(&config)?;

    Ok(())
}
