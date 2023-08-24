#[cfg(not(any(kani, feature = "kani")))]
use rand::Rng;

#[cfg(any(kani, feature = "kani"))]
pub(crate) fn random() -> [u8; 32] {
    let mut arr = [0u8; 32];
    for i in 0..32 {
        arr[i] = kani::any::<u8>();
    }
    arr
}

#[cfg(not(any(kani, feature = "kani")))]
pub(crate) fn random<const N: usize>() -> [u8; N] {
    let mut rng = rand::thread_rng();
    let mut arr = [0u8; N];

    for elem in arr.iter_mut() {
        *elem = rng.gen();
    }

    arr
}
