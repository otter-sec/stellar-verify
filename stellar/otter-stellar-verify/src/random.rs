#[cfg(not(any(kani, feature = "kani")))]
use rand::Rng;

#[cfg(not(any(kani, feature = "kani")))]
pub(crate) fn random<const N: usize>() -> [u8; N] {
    let mut rng = rand::thread_rng();
    let mut arr = [0u8; N];

    for elem in arr.iter_mut() {
        *elem = rng.gen();
    }

    arr
}
