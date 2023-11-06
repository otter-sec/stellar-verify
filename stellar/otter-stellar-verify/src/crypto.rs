use crate::{Bytes, BytesN, Env};

pub struct Crypto {
    env: Env,
}

impl Crypto {
    pub(crate) fn new(env: &Env) -> Crypto {
        Crypto { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn sha256(&self, _data: &Bytes) -> BytesN<32> {
        BytesN([0; 32])
    }

    pub fn keccak256(&self, _data: &Bytes) -> BytesN<32> {
        BytesN([0; 32])
    }

    pub fn ed25519_verify(
        &self,
        _public_key: &BytesN<32>,
        _message: &Bytes,
        _signature: &BytesN<64>,
    ) {
        // nop
    }

    pub fn secp256k1_recover(
        &self,
        _message_digest: &BytesN<32>,
        _signature: &BytesN<64>,
        _recorvery_id: u32,
    ) -> BytesN<65> {
        BytesN([0; 65])
    }
}
