use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("Filesystem error: {0}")]
    #[diagnostic(code(flow::io_error), url(docsrs))]
    Io(#[from] std::io::Error),

    #[error("Missing argument: {0}")]
    #[diagnostic(code(flow::missing_argument), url(docsrs))]
    MissingArgument(String),
}
