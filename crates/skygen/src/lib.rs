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
pub mod transformer;

use anyhow::{anyhow, Context, Result};
use include_dir::{include_dir, Dir};
use openapiv3::OpenAPI;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Deserializer;
use serde_yaml::Value as YamlValue;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use transformer::{
    clamp_overflowing_numeric_literals, fix_json_large_numbers, fix_yaml_large_numbers,
};

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
    if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(data) {
        let fixed = fix_json_large_numbers(json_val);

        return from_json_value(fixed).with_context(|| "failed to deserialize OpenAPI from JSON");
    }

    if let Some(sanitized) = clamp_overflowing_numeric_literals(data) {
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&sanitized) {
            let fixed = fix_json_large_numbers(json_val);
            return from_json_value(fixed)
                .with_context(|| "failed to deserialize OpenAPI from JSON (sanitized)");
        }
    }

    let yaml_val = match serde_yaml::from_str::<serde_yaml::Value>(data) {
        Ok(val) => val,
        Err(original_err) => {
            if let Some(sanitized) = clamp_overflowing_numeric_literals(data) {
                serde_yaml::from_str::<serde_yaml::Value>(&sanitized).with_context(|| {
                    "failed to parse YAML even after clamping overflowing integers"
                })?
            } else {
                return Err(original_err).with_context(|| "failed to parse YAML");
            }
        }
    };
    let normalized_yaml = normalize_yaml_keys(yaml_val);
    let fixed_yaml = fix_yaml_large_numbers(normalized_yaml);
    let json_val: serde_json::Value =
        serde_json::to_value(fixed_yaml).with_context(|| "failed to convert YAML to JSON value")?;
    let fixed_json = fix_json_large_numbers(json_val);

    from_json_value(fixed_json).with_context(|| "failed to deserialize OpenAPI from YAML->JSON")
}

fn normalize_yaml_keys(value: YamlValue) -> YamlValue {
    match value {
        YamlValue::Mapping(map) => {
            let normalized = map
                .into_iter()
                .map(|(key, val)| {
                    let normalized_key = match key {
                        YamlValue::String(s) => s,
                        YamlValue::Number(n) => n.to_string(),
                        YamlValue::Bool(b) => b.to_string(),
                        YamlValue::Null => "null".to_string(),
                        other => serde_yaml::to_string(&other)
                            .unwrap_or_else(|_| format!("{other:?}"))
                            .trim()
                            .to_string(),
                    };
                    (YamlValue::String(normalized_key), normalize_yaml_keys(val))
                })
                .collect();
            YamlValue::Mapping(normalized)
        }
        YamlValue::Sequence(seq) => {
            YamlValue::Sequence(seq.into_iter().map(normalize_yaml_keys).collect())
        }
        other => other,
    }
}

fn from_json_value<T>(value: serde_json::Value) -> Result<T>
where
    T: DeserializeOwned,
{
    let json_str = serde_json::to_string(&value)?;
    let mut deserializer = Deserializer::from_str(&json_str);
    serde_path_to_error::deserialize(&mut deserializer).map_err(|error| {
        let path = error.path().to_string();
        anyhow!(
            "{} at {}",
            error,
            if path.is_empty() { "<root>" } else { &path }
        )
    })
}
