use crate::{Symbol, Val};

// Define a EnumVal type which stores Symbol and Val
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumType {
    pub variant: Symbol,
    pub value: Option<Box<Val>>,
}
