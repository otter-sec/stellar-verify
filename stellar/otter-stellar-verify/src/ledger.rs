use crate::BytesN;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct Ledger {}

impl Ledger {
    pub fn sequence(&self) -> u32 {
        u32::MAX
    }

    pub fn protocol_version(&self) -> u32 {
        u32::MAX
    }

    pub fn max_live_until_ledger(&self) -> u32 {
        #[cfg(kani)]
        {
            kani::any()
        }
        #[cfg(not(kani))]
        {
            u32::MAX
        }
    }

    // Always return current timestamp as of the time of writing this code.
    pub fn timestamp(&self) -> u64 {
        1701348881
    }

    // Always return the same value to avoid concurrency issues
    pub fn network_id(&self) -> BytesN<32> {
        BytesN::from_array(&[1; 32])
    }
}
