use std::path::PathBuf;

use clap::Args;
use flow_core::{Config, Space};
use inquire::Text;
use miette::IntoDiagnostic;
use serde::Serialize;

use crate::{commands::Command, common::OutputArgs};
use flow_common::PathExt;
use flow_errors::CliError;

#[derive(Args, Debug, Clone)]
pub struct Arguments {
    #[command(flatten)]
    pub output: OutputArgs,

    /// Path to initialize the space at
    pub path: Option<PathBuf>,

    /// Name of the space (defaults to the directory name)
    #[arg(short, long)]
    pub name: Option<String>,

    /// Flag, to not register the space
    #[arg(long)]
    pub no_register: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Output {
    pub name: String,
    pub path: PathBuf,
}

pub struct Init {
    args: Arguments,
}

impl Command for Init {
    type Args = Arguments;
    type Output = Output;

    fn new(args: Self::Args) -> Self {
        Self { args }
    }

    fn output_args(&self) -> &OutputArgs {
        &self.args.output
    }

    async fn interactive(&mut self) -> miette::Result<()> {
        if self.args.path.is_none() {
            self.printer().info("Entering interactive mode");

            let path_input = Text::new("Path:")
                .with_default(".")
                .with_help_message("Path where the space will be initialized")
                .prompt()
                .into_diagnostic()?;

            self.args.path = Some(PathBuf::from(path_input));
        }

        if self.args.name.is_none() {
            let mut name_prompt = Text::new("Name:").with_help_message("Name of the space");
            let test = self
                .args
                .path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str());
            if let Some(name_default) = test {
                name_prompt = name_prompt.with_default(name_default);
            }

            let name_input = name_prompt.prompt().into_diagnostic()?;
            if !name_input.trim().is_empty() {
                self.args.name = Some(name_input);
            }
        }

        Ok(())
    }

    async fn execute(&mut self) -> miette::Result<Self::Output> {
        let path = self
            .args
            .path
            .take()
            .ok_or_else(|| CliError::MissingArgument("path".to_string()))?;

        let path_name = path.file_name().and_then(|n| n.to_str());
        let name = self
            .args
            .name
            .take()
            .or_else(|| path_name.map(String::from))
            .ok_or_else(|| CliError::MissingArgument("name".to_string()))?;

        let space = Space::init(&path, &name).await?;
        if !self.args.no_register {
            let mut config = Config::load().await?;

            config.register(&space).await?;
            if config.active().is_none() {
                config.set_active(space.name()).await?;
            }
        }

        Ok(Output { name, path })
    }

    fn finalize(&self, output: &Self::Output) {
        let printer = self.printer();

        printer.success("Space initialized");
        printer.blank();
        printer.kv("Name", &output.name);
        printer.kv("Path", output.path.normalize());
    }
}
