use std::path::PathBuf;

use clap::Args;
use flow_core::Space;
use inquire::Text;
use miette::IntoDiagnostic;
use serde::Serialize;

use crate::{commands::Command, common::GlobalArgs, errors::Error, extensions::PathExt};

#[derive(Args, Debug, Clone)]
pub struct Arguments {
    #[command(flatten)]
    pub global: GlobalArgs,

    pub path: Option<PathBuf>,

    #[arg(short, long)]
    pub name: Option<String>,
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

    fn globals(&self) -> &GlobalArgs {
        &self.args.global
    }

    async fn interactive(&mut self) -> miette::Result<()> {
        if self.args.path.is_none() {
            self.args.global.info("Entering interactive mode");

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
            .ok_or_else(|| Error::MissingArgument("path".to_string()))?;
        let name = self
            .args
            .name
            .take()
            .ok_or_else(|| Error::MissingArgument("name".to_string()))?;

        let _ = Space::init(&path, &name).await?;

        Ok(Output { name, path })
    }

    fn finalize(&self, output: &Self::Output) {
        self.globals().success("Graph initialized successfully");
        self.globals().blank();
        self.globals().kv("Name", &output.name);
        self.globals().kv("Path", output.path.normalize());
    }
}
