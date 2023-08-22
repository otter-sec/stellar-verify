#[cfg(any(kani, feature = "kani"))]
pub(crate) fn random() -> [u8; 32] {
    let mut arr = [0u8; 32];
    for _ in 0..32 {
        kani::any::<u8>();
    }
    arr
}

#[cfg(not(any(kani, feature = "kani")))]
pub(crate) fn random() -> [u8; 32] {
    [0u8; 32]
}
