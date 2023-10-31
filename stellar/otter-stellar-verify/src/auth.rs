use crate::{Address, BytesN, Symbol, Val, Vec};

#[derive(Clone)]
pub enum Context {
    Contract(ContractContext),
    CreateContractHostFn(CreateContractHostFnContext),
}

#[derive(Clone)]
pub struct ContractContext {
    pub contract: Address,
    pub fn_name: Symbol,
    pub args: Vec<Val>,
}

#[derive(Clone)]
pub struct CreateContractHostFnContext {
    pub executable: ContractExecutable,
    pub salt: BytesN<32>,
}

/// Contract executable used for creating a new contract and used in
/// `CreateContractHostFnContext`.
#[derive(Clone)]
pub enum ContractExecutable {
    Wasm(BytesN<32>),
}
