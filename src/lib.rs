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

use anyhow::Result;
use std::path::{Path, PathBuf};
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
    #[structopt(short = "s", long = "spec-file")]
    pub schema: String,

    /// The output directory where the generated bindings will be placed
    #[structopt(short = "o", long = "output-dir")]
    pub output: PathBuf,

    #[structopt(long = "kind", default_value = "wit", possible_values = &BindingKind::variants(), case_insensitive = true)]
    pub kind: BindingKind,
}
