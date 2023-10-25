#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct Ledger {}

impl Ledger {
    pub fn sequence(&self) -> u32 {
        u32::MAX
    }
}
