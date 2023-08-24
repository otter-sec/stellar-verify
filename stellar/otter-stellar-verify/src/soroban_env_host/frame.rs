use crate::{types::ScVal, Val};

use super::{
    error::HostError,
    xdr::{HostFunction, HostFunctionType},
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
        // let hf_type = hf.discriminant();
        // match hf {
        //     HostFunction::CreateContract(args) => self.create_contract_internal(None, args),
        //     _ => todo!("Implement invoke_function_raw for {:?}", hf_type),
        // }
        Ok(Val::default())
    }

    pub fn invoke_function(&self, hf: HostFunction) -> Result<ScVal, HostError> {
        let rv = self.invoke_function_raw(hf)?;
        todo!("Convert Val to ScVal")
    }
}
