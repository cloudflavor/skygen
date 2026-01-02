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
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tokio::fs;

fn generate_cargo_config(config: Config) -> Result<String> {
    let cargo_tpl = crate::TEMPLATES
        .get_file("cargo.toml.tera")
        .with_context(|| "failed to open cargo tera template")?;
    let contents = cargo_tpl
        .contents_utf8()
        .with_context(|| "failed to read cargo tera template")?;

    let mut tera = tera::Tera::default();
    tera.add_raw_template("cargo.toml.tera", contents)?;

    let mut context = tera::Context::new();
    context.insert("crate_name", &config.name);
    context.insert("description", &config.description);
    context.insert("version", &config.version);
    context.insert("edition", &config.edition);
    context.insert("lib_status", &config.lib_status);
    context.insert("keywords", &config.keywords);
    context.insert("authors", &config.authors);

    let render = tera
        .render("cargo.toml.tera", &context)
        .with_context(|| "failed to render tera template")?;

    Ok(render)
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
