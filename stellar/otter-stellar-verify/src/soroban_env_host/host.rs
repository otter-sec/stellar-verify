use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct LedgerInfo {
    pub protocol_version: u32,
    pub sequence_number: u32,
    pub timestamp: u64,
    pub network_id: [u8; 32],
    pub base_reserve: u32,
    pub min_temp_entry_expiration: u32,
    pub min_persistent_entry_expiration: u32,
    pub max_entry_expiration: u32,
}

#[derive(Clone, Default)]
pub(crate) struct HostImpl {}

#[derive(Clone)]
pub struct Host(pub(crate) Rc<HostImpl>);

#[allow(clippy::derivable_impls)]
impl Default for Host {
    fn default() -> Self {
        Self(Default::default())
    }
}
