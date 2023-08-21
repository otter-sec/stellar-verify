use crate::{bytes::BytesN, env::Env};

#[derive(Clone)]
pub struct Ledger(Env);

impl Ledger {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Ledger {
        Ledger(env.clone())
    }

    /// Returns the version of the protocol that the ledger created with.
    pub fn protocol_version(&self) -> u32 {
        todo!()
    }

    /// Returns the sequence number of the ledger.
    ///
    /// The sequence number is a unique number for each ledger
    /// that is sequential, incremented by one for each new ledger.
    pub fn sequence(&self) -> u32 {
        todo!()
    }

    /// Returns a unix timestamp for when the ledger was closed.
    ///
    /// The timestamp is the number of seconds, excluding leap seconds,
    /// that have elapsed since unix epoch. Unix epoch is January 1st, 1970,
    /// at 00:00:00 UTC.
    pub fn timestamp(&self) -> u64 {
        todo!()
    }

    /// Returns the network identifier.
    ///
    /// This is SHA-256 hash of the network passphrase, for example
    /// for the Public Network this returns:
    /// > SHA256(Public Global Stellar Network ; September 2015)
    ///
    /// Returns for the Test Network:
    /// > SHA256(Test SDF Network ; September 2015)
    pub fn network_id(&self) -> BytesN<32> {
        todo!()
    }
}
