use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::trie::Trie;

#[derive(Clone)]
pub struct HashMap<K, V = ()> {
    trie: Trie<bool, KeyValue<K, V>>,
    phantom: PhantomData<K>,
}

pub type HashSet<K> = HashMap<K, ()>;

#[derive(Clone, Debug)]
struct KeyValue<K, V> {
    key: K,
    value: Option<V>,
}

impl<K: PartialEq, V> PartialEq for KeyValue<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        HashMap {
            trie: Trie::empty_store(),
            phantom: PhantomData,
        }
    }
}

impl<K: Hash + PartialEq> HashMap<K> {
    pub fn insert(&self, value: K) -> Self {
        self.put(value, ())
    }
    pub fn search(&self, value: &K) -> bool {
        self.get(value).is_some()
    }
}

impl<K: Hash + PartialEq, V> HashMap<K, V> {
    pub fn put(&self, key: K, value: V) -> Self {
        Self {
            trie: self.trie.insert_store(
                Self::get_bits(&key),
                KeyValue {
                    key,
                    value: Some(value),
                },
            ),
            phantom: PhantomData,
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        let store = self.trie.get_store(Self::get_bits(k))?;
        let store_cloned: Vec<_> = (*store).to_vec();
        store_cloned
            .iter()
            .find(|KeyValue { key, .. }| k == key)
            .and_then(|kv| kv.value.as_ref())
    }

    pub fn delete(&self, key: K) -> Option<Self> {
        self.trie
            .delete_store(Self::get_bits(&key), &KeyValue { key, value: None })
            .map(|trie| HashMap {
                trie,
                phantom: PhantomData,
            })
    }

    fn get_bits(key: &K) -> Vec<bool> {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        let hash = s.finish();
        (0..64).map(|i| hash & (1u64 << i) > 0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_retrieve_values_set() {
        let m1 = HashMap::new();
        let m2 = m1.insert(1238).insert(-1).insert(1238);
        assert!(m2.search(&1238));
        assert!(!m1.search(&-1));
        assert!(!m2.delete(1238).unwrap().search(&1238))
    }

    #[test]
    fn insert_and_retrieve_values() {
        let m1 = HashMap::new();
        let m2 = m1.put(1238, 1).put(-1, 10);
        assert_eq!(m2.get(&1238), Some(&1));
        assert_eq!(m1.get(&-1), None);
    }

    #[test]
    fn handle_hash_collisions() {
        #[derive(PartialEq, Clone)]
        struct K {
            x: i8,
        }

        impl Hash for K {
            fn hash<H: Hasher>(&self, _: &mut H) {}
        }

        let m = HashMap::new().put(K { x: 1 }, 1).put(K { x: -1 }, 10);
        assert_eq!(m.get(&K { x: 1 }), Some(&1));
        assert_eq!(m.get(&K { x: -1 }), Some(&10));
    }

    #[test]
    fn delete_entries() {
        #[derive(PartialEq, Clone)]
        struct K {
            x: i8,
        }

        impl Hash for K {
            fn hash<H: Hasher>(&self, _: &mut H) {}
        }

        let m = HashMap::new()
            .put(K { x: 1 }, 1)
            .put(K { x: -1 }, 10)
            .delete(K { x: 1 });
        assert!(m.is_some());
        let m2 = m.unwrap();
        assert_eq!(m2.get(&K { x: 1 }), None);
        assert_eq!(m2.get(&K { x: -1 }), Some(&10));
    }
}
