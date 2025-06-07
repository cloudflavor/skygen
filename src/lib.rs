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
