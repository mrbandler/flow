#![allow(dead_code)]

use std::path::PathBuf;

use inquire::{
    validator::{ErrorMessage, StringValidator, Validation},
    CustomUserError,
};

/// Path validator that checks if a given path already exists.
#[derive(Debug, Clone)]
pub struct PathAlreadyExistsValidator;

impl PathAlreadyExistsValidator {
    pub const fn new() -> Self {
        Self
    }

    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl StringValidator for PathAlreadyExistsValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        let path = PathBuf::from(input);

        // Check if the path already exists
        if path.exists() {
            let msg = ErrorMessage::Custom("The specified path already exists.".into());
            return Ok(Validation::Invalid(msg));
        }

        Ok(Validation::Valid)
    }
}
