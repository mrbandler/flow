use miette::Result;

use crate::{common::OutputArgs, printer::Printer};

pub mod init;

pub trait Command: Sized {
    type Args;
    type Output: serde::Serialize;

    fn new(args: Self::Args) -> Self;

    fn output_args(&self) -> &OutputArgs;

    fn printer(&self) -> Printer {
        self.output_args().printer()
    }

    async fn interactive(&mut self) -> Result<()>;

    async fn execute(&mut self) -> Result<Self::Output>;

    fn finalize(&self, output: &Self::Output);

    async fn run(&mut self) -> Result<()> {
        if !self.output_args().json {
            self.interactive().await?;
        }

        let output = self.execute().await?;
        if self.output_args().json {
            self.printer().json(&output)?;
        } else {
            self.finalize(&output);
        }

        Ok(())
    }
}
