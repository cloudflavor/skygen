// Copyright 2025 Cloudflavor GmbH

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod generator;
pub mod ir;
pub mod transformers;

use anyhow::{anyhow, Result};
use include_dir::{include_dir, Dir};
use openapiv3::OpenAPI;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use structopt::StructOpt;


pub static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/templates");

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(
        short,
        long,
        default_value = "info",
        possible_values = &["trace", "debug", "info", "warn", "error"]
    )]
    pub log_level: tracing::Level,

    #[structopt(subcommand)]
    pub commands: Commands,
}

#[derive(StructOpt)]
pub enum Commands {
    Generate(GenerateArgs),
}

#[derive(StructOpt)]
pub struct GenerateArgs {
    /// OpenAPIv3 Spec file to generate the SDK from
    #[structopt(short = "s", long = "spec-file")]
    pub schema: String,

    /// The output directory where the generated bindings will be placed
    #[structopt(short = "o", long = "output-dir")]
    pub output: PathBuf,

    /// Skygen config for generating the SDK.
    #[structopt(short = "c", long = "config")]
    pub config: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    name: String,
    version: String,
    description: String,
    keywords: Vec<String>,
    api_url: String,
    authors: Vec<String>,
}

pub async fn read_config(config_file: impl AsRef<Path>) -> Result<Config> {
    let resp = tokio::fs::read_to_string(&config_file).await?;
    let config: Config = toml::from_str(resp.as_str())?;

    Ok(config)
}

pub(crate) async fn deserialize_data(data: &str) -> Result<OpenAPI> {
    if let Ok(json) = serde_json::from_str(data) {
        return Ok(json);
    }
    if let Ok(yaml) = serde_yaml::from_str(data) {
        return Ok(yaml);
    }
    Err(anyhow!("failed to deserialize OpenAPI spec"))
}


