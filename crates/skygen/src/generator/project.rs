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

use crate::Config;
use anyhow::Result;
use std::path::Path;
use std::process::Command;
use tokio::fs;

fn generate_cargo_config(config: Config) -> Result<String> {
    let deps = [r#""#];
    let kw = [
        config.keywords,
        vec![
            "api".to_string(),
            "openapiv3".to_string(),
            "openapi".to_string(),
            "sdk".to_string(),
        ],
    ]
    .concat();
    let kw_str = serde_json::to_string(&kw)?;
    let authors = serde_json::to_string(&config.authors)?;
    let cargo = format!(
        r#"[package]
name = "{name}" 
edition = "{edition}"
version = "{version}"
authors = {authors}
license = "Apache-2.0"
description = "{description}"
repository = "https://cloudflavor.io/projects/skygen"
readme = "README.md"

[badge.maintenance]
status = "{status}"

categories = [
    "web-programming::http-client", 
    "api-bindings"
]

keywords = {key_words}

[dependencies]
"#,
        name = config.name,
        edition = config.edition.unwrap_or("2021".to_string()),
        version = config.version,
        authors = authors,
        description = config.description,
        status = config.lib_status,
        key_words = kw_str,
    );

    Ok(cargo)
}

pub async fn create_scaffolding(root_dir: impl AsRef<Path>, config: Config) -> Result<()> {
    let src_dir = root_dir.as_ref().join("src");

    for path in [&src_dir, &src_dir.join("apis"), &src_dir.join("models")] {
        fs::create_dir_all(path).await?;
    }

    for path in [
        &src_dir.join("lib.rs"),
        &src_dir.join("apis").join("mod.rs"),
        &src_dir.join("models").join("mod.rs"),
    ] {
        fs::File::create(&path).await?;
    }

    let toml_data = generate_cargo_config(config)?;

    fs::write(root_dir.as_ref().join("Cargo.toml"), &toml_data).await?;

    Ok(())
}

pub fn format_crate(path: impl AsRef<Path>) -> Result<()> {
    Command::new("cargo")
        .arg("format")
        .current_dir(path)
        .status()?;

    Ok(())
}

pub fn format_cargo(path: impl AsRef<Path>) -> Result<()> {
    Command::new("taplo").current_dir(&path).status()?;

    Ok(())
}
