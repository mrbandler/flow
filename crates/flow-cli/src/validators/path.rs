#![allow(dead_code)]

use std::path::PathBuf;

use flow_core::Space;
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

/// Path validator that checks if a given path is a valid Flow space.
#[derive(Debug, Clone)]
pub struct PathIsValidSpaceValidator;

impl PathIsValidSpaceValidator {
    pub const fn new() -> Self {
        Self
    }

    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl StringValidator for PathIsValidSpaceValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        let path = PathBuf::from(input);

        if !path.is_dir() {
            let msg = ErrorMessage::Custom("The specified path is not a directory.".into());
            return Ok(Validation::Invalid(msg));
        }

        let is_valid = Space::is_valid(&path);
        if !is_valid {
            let msg = ErrorMessage::Custom("The specified path is not a valid Flow space.".into());
            return Ok(Validation::Invalid(msg));
        }

        Ok(Validation::Valid)
    }
}
