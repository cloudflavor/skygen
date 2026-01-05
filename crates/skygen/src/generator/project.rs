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
use tera::{Context as TeraContext, Tera};
use tokio::fs;

pub struct RenderPlan {
    template: &'static str,
    out_rel: &'static str,
    extra: fn(&mut TeraContext),
}

fn noop(_: &mut tera::Context) {}

async fn render_templates(
    tera: &tera::Tera,
    root: &Path,
    base: &tera::Context,
    plans: &[RenderPlan],
) -> Result<()> {
    for p in plans {
        let mut ctx = base.clone();
        (p.extra)(&mut ctx);

        let data = tera.render(p.template, &ctx)?;
        let out = root.join(p.out_rel);

        fs::write(out, data).await?;
    }

    Ok(())
}

pub async fn bootstrap_lib(config: &Config, out_dir: impl AsRef<Path>) -> Result<()> {
    create_dirs(out_dir.as_ref())
        .await
        .with_context(|| "failed to create project directories")?;

    let mut tera = Tera::default();
    for name in ["cargo.toml.tera", "lib.rs.tera", "mod.rs.tera"] {
        let f = crate::TEMPLATES
            .get_file(name)
            .with_context(|| "failed to fetch template")?;
        tera.add_raw_template(
            name,
            f.contents_utf8()
                .with_context(|| "failed to fetch utf8 contents from template")?,
        )?;
    }

    let mut base_ctx = TeraContext::new();
    base_ctx.insert("config", config);

    let plans = [
        RenderPlan {
            template: "cargo.toml.tera",
            out_rel: "Cargo.toml",
            extra: noop,
        },
        RenderPlan {
            template: "lib.rs.tera",
            out_rel: "src/lib.rs",
            extra: noop,
        },
        RenderPlan {
            template: "mod.rs.tera",
            out_rel: "src/apis/mod.rs",
            extra: noop,
        },
        RenderPlan {
            template: "mod.rs.tera",
            out_rel: "src/models/mod.rs",
            extra: noop,
        },
    ];

    render_templates(&tera, out_dir.as_ref(), &base_ctx, &plans).await?;

    Ok(())
}

async fn create_dirs(root_dir: &Path) -> Result<()> {
    let src_dir = root_dir.join("src");

    for path in [&src_dir, &src_dir.join("apis"), &src_dir.join("models")] {
        fs::create_dir_all(path).await?;
    }

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
