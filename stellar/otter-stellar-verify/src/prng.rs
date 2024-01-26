#[cfg(any(kani, feature = "kani"))]
use core::ops::{Bound, RangeBounds};

use crate::Bytes;
use soroban_env_common::{Env, ToValEnum, Val};

pub struct Prng {
    env: Env,
}

impl Prng {
    pub(crate) fn new(env: &Env) -> Prng {
        Prng { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn seed(&self, _seed: Bytes) {}

    #[cfg(any(kani, feature = "kani"))]
    pub fn u64_in_range(&self, r: impl RangeBounds<u64>) -> u64 {
        let start_bound = match r.start_bound() {
            Bound::Included(b) => *b,
            Bound::Excluded(b) => *b + 1,
            Bound::Unbounded => 0,
        };
        let end_bound = match r.end_bound() {
            Bound::Included(b) => *b,
            Bound::Excluded(b) => *b - 1,
            Bound::Unbounded => u64::MAX,
        };

        let val = kani::any::<u64>();
        kani::assume(val >= start_bound && val <= end_bound);
        val
    }

    pub fn shuffle<V>(&self, v: Vec<V>) -> Vec<Val>
    where
        V: ToValEnum,
    {
        v.into_iter().map(|item| item.into_val()).collect()
    }
}
