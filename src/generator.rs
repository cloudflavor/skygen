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

use anyhow::Context;
use anyhow::Result;
use openapiv3::{Components, OpenAPI, Operation, Parameter, Paths};
use serde_json::Number;
use serde_json::Value as JsonVal;
use serde_yaml;
use serde_yaml::Value as YamlVal;
use std::collections::HashMap;
use std::path::Path;

async fn deserialize_data(data: &str) -> Result<OpenAPI> {
    let json_str = match serde_json::from_str::<serde_json::Value>(data) {
        Ok(json_val) => {
            let fixed_json = fix_json_large_numbers(json_val);
            serde_json::to_string(&fixed_json).with_context(|| "failed to deserialize json")?
        }
        Err(e) => {
            let yaml_val = serde_yaml::from_str::<serde_yaml::Value>(data)
                .with_context(|| "failed to deserialize yaml")?;
            let fixed_yaml = fix_yaml_large_numbers(yaml_val);
            serde_json::to_string(&fixed_yaml)
                .with_context(|| format!("failed to deseriale json {e}"))?
        }
    };
    println!("{:#?}", json_str);

    let openapi: OpenAPI =
        serde_json::from_str(&json_str).with_context(|| "failed to deserialize open api schema")?;
    Ok(openapi)
}

pub fn fix_json_large_numbers(val: JsonVal) -> JsonVal {
    match val {
        JsonVal::Number(n) => {
            let n_str = n.to_string();
            match n_str.parse::<u128>() {
                Ok(parsed) if parsed > u64::MAX as u128 => JsonVal::Number(Number::from(u64::MAX)),
                Ok(parsed) => JsonVal::Number(Number::from(parsed as u64)),
                Err(_) => JsonVal::Number(Number::from(u64::MAX)), // fallback
            }
        }
        JsonVal::String(s) => {
            if let Ok(parsed) = s.parse::<u128>() {
                if parsed > u64::MAX as u128 {
                    JsonVal::Number(Number::from(u64::MAX))
                } else {
                    JsonVal::Number(Number::from(parsed as u64))
                }
            } else {
                JsonVal::String(s)
            }
        }
        JsonVal::Array(arr) => {
            JsonVal::Array(arr.into_iter().map(fix_json_large_numbers).collect())
        }
        JsonVal::Object(obj) => {
            let fixed = obj
                .into_iter()
                .map(|(k, v)| (k, fix_json_large_numbers(v)))
                .collect();
            JsonVal::Object(fixed)
        }
        other => other,
    }
}

fn fix_yaml_large_numbers(val: YamlVal) -> YamlVal {
    match val {
        serde_yaml::Value::Number(n) => {
            let n_str = n.to_string();
            match n_str.parse::<u128>() {
                Ok(parsed) if parsed > u64::MAX as u128 => {
                    serde_yaml::Value::Number(serde_yaml::Number::from(u64::MAX))
                }
                Ok(parsed) => serde_yaml::Value::Number(serde_yaml::Number::from(parsed as u64)),
                Err(_) => {
                    // Fallback: something weird, just replace with u64::MAX
                    serde_yaml::Value::Number(serde_yaml::Number::from(u64::MAX))
                }
            }
        }
        serde_yaml::Value::String(s) => {
            if let Ok(parsed) = s.parse::<u128>() {
                if parsed > u64::MAX as u128 {
                    serde_yaml::Value::Number(serde_yaml::Number::from(u64::MAX))
                } else {
                    serde_yaml::Value::Number(serde_yaml::Number::from(parsed as u64))
                }
            } else {
                serde_yaml::Value::String(s)
            }
        }
        serde_yaml::Value::Sequence(seq) => {
            serde_yaml::Value::Sequence(seq.into_iter().map(fix_yaml_large_numbers).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let fixed = map
                .into_iter()
                .map(|(k, v)| (k, fix_yaml_large_numbers(v)))
                .collect();
            serde_yaml::Value::Mapping(fixed)
        }
        other => other,
    }
}

pub async fn generate(schema: impl AsRef<Path>, output_dir: impl AsRef<Path>) -> Result<()> {
    match tokio::fs::read_to_string(schema).await {
        Ok(content) => {
            let openapi: OpenAPI = deserialize_data(content.as_str()).await?;
            walk_paths(openapi.paths).await?;
            if let Some(components) = openapi.components {
                walk_schemas(components).await?;
            }
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

async fn walk_paths(paths: Paths) -> Result<()> {
    let mut grouped_by_tag: HashMap<String, Vec<(String, Operation)>> = HashMap::new();

    for (path, item) in paths.paths {
        if let Some(item) = item.into_item() {
            for (method, op_opt) in [
                ("get", item.get),
                ("post", item.post),
                ("put", item.put),
                ("delete", item.delete),
                ("patch", item.patch),
                ("options", item.options),
            ] {
                if let Some(op) = op_opt {
                    for tag in &op.tags {
                        grouped_by_tag
                            .entry(tag.clone())
                            .or_default()
                            .push((path.clone(), op.clone()));
                    }
                }
            }

            let params: Vec<Parameter> = item
                .parameters
                .into_iter()
                .filter_map(|param| param.into_item())
                .collect();
        }
    }
    println!("{:?}", grouped_by_tag);
    Ok(())
}

async fn walk_schemas(schemas: Components) -> Result<()> {
    Ok(())
}
