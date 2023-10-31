use crate::{Address, Bytes, BytesN, Env, IntoVal};

pub struct Deployer {
    env: Env,
}

impl Deployer {
    pub(crate) fn new(env: &Env) -> Deployer {
        Deployer { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn with_current_contract(
        &self,
        salt: impl IntoVal<Env, BytesN<32>>,
    ) -> DeployerWithAddress {
        DeployerWithAddress {
            _env: self.env.clone(),
            _address: self.env.current_contract_address(),
            _salt: salt.into_val(&self.env),
        }
    }

    pub fn with_address(
        &self,
        address: Address,
        salt: impl IntoVal<Env, BytesN<32>>,
    ) -> DeployerWithAddress {
        DeployerWithAddress {
            _env: self.env.clone(),
            _address: address,
            _salt: salt.into_val(&self.env),
        }
    }

    pub fn with_stellar_asset(
        &self,
        serialized_asset: impl IntoVal<Env, Bytes>,
    ) -> DeployerWithAsset {
        DeployerWithAsset {
            _env: self.env.clone(),
            _serialized_asset: serialized_asset.into_val(&self.env),
        }
    }

    pub fn upload_contract_wasm(&self, _contract_wasm: impl IntoVal<Env, Bytes>) -> BytesN<32> {
        kani::any()
    }

    pub fn update_current_contract_wasm(&self, _wasm_hash: impl IntoVal<Env, BytesN<32>>) {}
}

pub struct DeployerWithAddress {
    _env: Env,
    _address: Address,
    _salt: BytesN<32>,
}

impl DeployerWithAddress {
    pub fn deployed_address(&self) -> Address {
        kani::any()
    }

    pub fn deploy(&self, _wasm_hash: impl IntoVal<Env, BytesN<32>>) -> Address {
        kani::any()
    }
}

pub struct DeployerWithAsset {
    _env: Env,
    _serialized_asset: Bytes,
}

impl DeployerWithAsset {
    pub fn deployed_address(&self) -> Address {
        kani::any()
    }

    pub fn deploy(&self) -> Address {
        kani::any()
    }
}
