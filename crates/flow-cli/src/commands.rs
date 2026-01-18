use miette::Result;

use crate::common::GlobalArgs;

pub mod init;

pub trait Command: Sized {
    type Args;
    type Output: serde::Serialize;

    fn new(args: Self::Args) -> Self;

    fn globals(&self) -> &GlobalArgs;

    async fn interactive(&mut self) -> Result<()>;

    async fn execute(&mut self) -> Result<Self::Output>;

    fn finalize(&self, output: &Self::Output);

    async fn run(&mut self) -> Result<()> {
        if !self.globals().json {
            self.interactive().await?;
        }

        let output = self.execute().await?;
        if self.globals().json {
            self.globals().json(&output)?;
        } else {
            self.finalize(&output);
        }

        Ok(())
    }
}
