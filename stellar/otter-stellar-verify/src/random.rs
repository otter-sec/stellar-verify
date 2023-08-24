#[cfg(any(kani, feature = "kani"))]
pub(crate) fn random() -> [u8; 32] {
    let arr = [0u8; 32];
    for _ in 0..32 {
        kani::any::<u8>();
    }
    arr
}

#[cfg(not(any(kani, feature = "kani")))]
pub(crate) fn random<const N: usize>() -> [u8; N] {
    let mut arr = [0u8; N];
    let timestamp = get_timestamp();

    for (i, elem) in arr.iter_mut().enumerate().take(N) {
        *elem = (timestamp >> (i % 8 * 8)) as u8;
    }

    arr
}

fn get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u64
}
