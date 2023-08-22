use crate::{
    types::{Hash, ScAddress, ScVal},
    Val,
};

use super::{
    error::HostError,
    xdr::{
        ContractExecutable, CreateContractArgs, HostFunction, HostFunctionType, ScErrorCode,
        ScErrorType,
    },
    Host,
};

pub(crate) enum ContractReentryMode {
    /// Re-entry is completely prohibited.
    Prohibited,
    /// Re-entry is allowed, but only directly into the same contract (i.e. it's
    /// possible for a contract to do a self-call via host).
    SelfAllowed,
    /// Re-entry is fully allowed.
    Allowed,
}

#[derive(Clone)]
pub(crate) enum Frame {
    HostFunction(HostFunctionType),
}

impl Host {
    fn invoke_function_raw(&self, hf: HostFunction) -> Result<Val, HostError> {
        let hf_type = hf.discriminant();
        match hf {
            HostFunction::CreateContract(args) => {
                todo!("Host::invoke_function_raw: CreateContract")
            }
            _ => {}
        }
        Ok(Val::default())
    }

    pub fn invoke_function(&self, hf: HostFunction) -> Result<ScVal, HostError> {
        todo!();
    }
}
