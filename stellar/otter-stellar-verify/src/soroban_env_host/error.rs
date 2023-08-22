use std::fmt::Debug;

use crate::soroban_env_common::error::Error;

#[derive(Clone, Debug)]
pub struct HostError {
    pub error: Error,
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <HostError as Debug>::fmt(self, f)
    }
}

impl std::error::Error for HostError {}
