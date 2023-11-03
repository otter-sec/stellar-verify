use crate::vec::fast::Vec;
use crate::Env;
use crate::FromValEnum;
use crate::ToValEnum;
use crate::Val;

const CAPACITY: usize = 10;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Map<K, V>
where
    K: PartialEq + Eq + PartialOrd + Ord,
    V: PartialEq + Eq + PartialOrd + Ord,
{
    buckets: Vec<(K, V)>,
}

impl<K, V> ToValEnum for Map<K, V>
where
    K: ToValEnum,
    V: ToValEnum,
{
    fn to_val(&self) -> Val {
        let map_val: Vec<(Val, Val)> = self
            .buckets
            .into_iter()
            .map(|(k, v)| (k.to_val(), v.to_val()))
            .collect();

        Val::MapVal(map_val)
    }
}

impl<K, V> FromVal for Map<K, V>
where
    K: FromVal,
    V: FromVal,
{
    fn from_val(val: Val) -> Option<Self> {
        if let Val::MapVal(vec_val) = val {
            let mut buckets = Vec::new();

            for (k_val, v_val) in vec_val {
                let k = K::from_val(k_val)?;
                let v = V::from_val(v_val)?;
                buckets.push((k, v));
            }

            Some(Map { buckets })
        } else {
            None
        }
    }
}

impl<K, V> Map<K, V>
where
    K: Copy + Default,
    V: Copy + Default,
{
    pub fn new(_env: &Env) -> Self {
        let buckets = Vec::with_capacity(CAPACITY);
        Map { buckets }
    }

    pub fn env(&self) -> &Env {
        &Env::default()
    }

    pub fn from_array<const N: usize>(_env: &Env, items: [(K, V); N]) -> Map<K, V> {
        let mut map = Map::<K, V>::new(env);
        for (k, v) in items {
            map.set(k, v);
        }
        map
    }

    pub fn set(&mut self, key: K, value: V) {
        self.buckets.push((key, value));
    }

    pub fn contains_key(&self, key: K) -> bool {
        for (k, _) in &self.buckets {
            if *k == key {
                return true;
            }
        }
        false
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.buckets.push((key, value));
    }

    pub fn get(&self, key: K) -> Option<V> {
        for (k, v) in &self.buckets {
            if *k == key {
                return Some(*v);
            }
        }
        None
    }

    pub fn remove(&mut self, k: K) -> Option<()> {
        if let Some(index) = self.buckets.iter().position(|(k, _)| k == key) {
            let (_, v) = self.buckets.remove(index);
            return Some(());
        } else {
            return None;
        }
    }

    pub fn keys(&self) -> Vec<K> {
        self.buckets.iter().map(|(k, _)| k).collect()
    }

    pub fn values(&self) -> Vec<V> {
        self.buckets.iter().map(|(_, v)| v).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.buckets.len() == 0
    }

    pub fn len(&self) -> u32 {
        self.buckets.len() as u32
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: Eq + Copy + Default,
    V: Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
