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

use anyhow::Result;
use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;

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

arg_enum! {
    #[derive(Debug)]
    pub enum BindingKind {
        Wit,
        Http,
    }
}

#[derive(StructOpt)]
pub struct GenerateArgs {
    #[structopt(short = "i", long = "input")]
    pub input: String,

    #[structopt(short = "o", long = "output")]
    pub output: PathBuf,

    #[structopt(long = "kind", default_value = "wit", possible_values = &BindingKind::variants(), case_insensitive = true)]
    pub kind: BindingKind,
}

pub mod generate {
    use super::*;

    pub async fn run_generate(args: GenerateArgs) -> Result<()> {
        let spec = if args.input.starts_with("http://") || args.input.starts_with("https://") {
            reqwest::get(&args.input).await?.bytes().await?.to_vec()
        } else {
            tokio::fs::read(&args.input).await?
        };

        tokio::fs::create_dir_all(&args.output).await?;
        let out_file = args.output.join("spec.bin");
        tokio::fs::write(out_file, spec).await?;

        tracing::info!("generated {:?} bindings to {:?}", args.kind, args.output);
        Ok(())
    }
}
