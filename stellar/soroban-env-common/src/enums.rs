use crate::{Symbol, Vec};

// Define a EnumVal type which stores Symbol and Val
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumType {
    pub variant: Symbol,
    pub value: Vec<u8>,
}
