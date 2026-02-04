#![allow(dead_code)]

use flow_core::Config;
use inquire::{
    validator::{ErrorMessage, StringValidator, Validation},
    CustomUserError,
};

/// Name validator that checks if a given name is already registered in the config.
#[derive(Debug, Clone)]
pub struct NameAlreadyRegisteredValidator<'a> {
    config: &'a Config,
}

impl<'a> NameAlreadyRegisteredValidator<'a> {
    pub const fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn boxed(config: &'a Config) -> Box<Self> {
        Box::new(Self::new(config))
    }
}

impl StringValidator for NameAlreadyRegisteredValidator<'_> {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        let exists = self.config.spaces().iter().any(|s| s.name == input);

        if exists {
            let msg = ErrorMessage::Custom("A space with this name is already registered.".into());
            return Ok(Validation::Invalid(msg));
        }

        Ok(Validation::Valid)
    }
}
