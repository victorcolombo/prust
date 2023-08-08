use crate::RefCounter;
pub struct Trie<T = u8, U = bool> {
    pub(crate) stored_value: Vec<RefCounter<U>>,
    pub(crate) adjecent_nodes: Vec<(T, RefCounter<Trie<T, U>>)>,
}

impl<T: Clone, U> Clone for Trie<T, U> {
    fn clone(&self) -> Self {
        Self {
            stored_value: self.stored_value.clone(),
            adjecent_nodes: self.adjecent_nodes.clone(),
        }
    }
}

impl<T: PartialEq + Clone, U> Trie<T, U> {
    pub(crate) fn empty_store() -> Trie<T, U> {
        Trie {
            stored_value: Vec::new(),
            adjecent_nodes: Vec::new(),
        }
    }
    pub fn empty() -> Trie<T, U> {
        Trie {
            stored_value: Vec::new(),
            adjecent_nodes: Vec::new(),
        }
    }
    pub fn insert_store<Slc: AsRef<[T]>>(&self, value: Slc, store: U) -> Self {
        let value_ref = value.as_ref();
        let mut new_trie = self.clone();
        if value_ref.is_empty() {
            new_trie.stored_value.push(RefCounter::new(store));
            return new_trie;
        }
        let head = &value_ref[0];
        let tail = &value_ref[1..];
        for (k, v) in new_trie.adjecent_nodes.iter_mut() {
            if k == head {
                *v = RefCounter::new(v.insert_store(tail, store));
                return new_trie;
            }
        }
        new_trie.adjecent_nodes.push((
            head.clone(),
            RefCounter::new(Trie::empty_store().insert_store(tail, store)),
        ));
        new_trie
    }
    pub fn get_store<Slc: AsRef<[T]>>(&self, value: Slc) -> Option<Box<[&U]>> {
        let value_ref = value.as_ref();
        if value_ref.is_empty() {
            let mut vr = Vec::new();
            for v in self.stored_value.iter() {
                vr.push(v.as_ref());
            }
            if vr.is_empty() {
                return Option::None;
            }
            return Option::Some(vr.into_boxed_slice());
        }
        let head = &value_ref[0];
        let tail = &value_ref[1..];
        for (k, v) in &self.adjecent_nodes {
            if k == head {
                return v.get_store(tail);
            }
        }
        return Option::None;
    }
}

impl<T: PartialEq + Clone, U: PartialEq> Trie<T, U> {
    pub fn delete_store<Slc: AsRef<[T]>>(&self, value: Slc, store: &U) -> Option<Self> {
        let value_ref = value.as_ref();
        let mut new_trie = self.clone();
        if value_ref.is_empty() {
            new_trie.stored_value.retain(|v| {
                let retain = v.as_ref() != store;
                retain
            });
            if self.stored_value.len() == new_trie.stored_value.len() {
                return Option::None;
            } else {
                return Option::Some(new_trie);
            }
        }
        let head = &value_ref[0];
        let tail = &value_ref[1..];
        for (k, v) in new_trie.adjecent_nodes.iter_mut() {
            if k == head {
                let subt = v.delete_store(tail, store)?;
                *v = RefCounter::new(subt);
                return Option::Some(new_trie);
            }
        }
        return Option::None;
    }
}

impl<T: PartialEq + Copy> Trie<T> {
    pub fn insert<Slc: AsRef<[T]>>(&self, value: Slc) -> Self {
        self.insert_store(value, true)
    }
    pub fn search<Slc: AsRef<[T]>>(&self, value: Slc) -> bool {
        self.get_store(value).is_some()
    }
    pub fn delete<Slc: AsRef<[T]>>(&self, value: Slc) -> Option<Self> {
        self.delete_store(value, &true)
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn test_trie_store() {
        let t = Trie::empty_store().insert_store("aab", 123);
        let t2 = t.insert_store("adc", 459);
        let boxed_array: Box<[&i32]> = Box::new([&123]);
        let boxed_array_2: Box<[&i32]> = Box::new([&459]);
        assert_eq!(t.get_store("aab"), Option::Some(boxed_array.clone()));
        assert!(t.get_store("adc").is_none());
        assert_eq!(t2.get_store("aab"), Option::Some(boxed_array));
        assert_eq!(t2.get_store("adc"), Option::Some(boxed_array_2));
    }

    #[test]
    fn test_trie_persistance_simple() {
        let t = Trie::empty().insert("aab").insert("adc");
        assert!(t.search("aab"));
        assert!(t.search("adc"));
    }

    #[test]
    fn test_trie_persistance() {
        let vs = vec!["aab", "adc", "acd", "dca"];
        let snapshots: Vec<_> = vs
            .iter()
            .scan(Trie::empty(), |tree, value| {
                *tree = tree.insert(value);
                Option::Some(tree.clone())
            })
            .collect();
        for (index, tree) in snapshots.iter().enumerate() {
            let found = vs
                .iter()
                .map(|s| tree.search(s))
                .filter(|found| *found == true)
                .count();
            assert_eq!(found, index + 1);
        }
    }

    #[test]
    fn test_search_present() {
        let v = vec![1, 5, 9];
        let not_v = vec![1, 15, 9];
        let t = Trie::empty().insert(&v);
        assert!(t.search(v));
        assert!(!t.search(not_v));
    }

    #[test]
    fn test_search_absent() {
        let s = "test";
        let not_s = "tett";
        let t = Trie::empty().insert(s);
        assert!(t.search(s));
        assert!(!t.search(not_s));
    }

    #[test]
    fn test_trie_deletion() {
        let t = Trie::empty().insert("aab").delete("aab");
        assert!(t.is_some());
        assert_eq!(t.unwrap().search("aab"), false);
        let t2 = Trie::empty();
        assert!(t2.delete("a").is_none());
    }

    #[test]
    fn test_insert_empty_string() {
        let t = Trie::empty().insert("");
        assert!(t.search(""));
    }

    #[test]
    fn test_multiple_values_for_same_key() {
        let t = Trie::empty_store()
            .insert_store("key", 1)
            .insert_store("key", 2);
        let values = t.get_store("key").unwrap();
        assert!(values.contains(&&1) && values.contains(&&2));
    }

    #[test]
    fn test_delete_internal_node() {
        let t = Trie::empty()
            .insert("abc")
            .insert("ab")
            .delete("ab")
            .unwrap();
        assert!(!t.search("ab"));
        assert!(t.search("abc"));
    }

    #[test]
    fn test_persistence_after_delete() {
        let t1 = Trie::empty().insert("key");
        let t2 = t1.delete("key").unwrap_or_else(|| t1.clone());
        assert!(t1.search("key"));
        assert!(!t2.search("key"));
    }

    #[test]
    fn test_search_nonexistent_key() {
        let t = Trie::empty().insert("key");
        assert!(!t.search("not_key"));
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let t = Trie::empty().insert("key");
        assert!(t.delete("not_key").is_none());
    }

    #[test]
    fn test_readme() {
        // Insert words
        let mut trie = Trie::empty().insert("apple").insert("app").insert("banana");

        // Snapshot the current trie. This operation is lightweight, allocating only a couple of bytes long.
        let snapshot = trie.clone();

        // Insert more words
        trie = trie.insert("grape").insert("banana-split");

        // Check for words in current trie
        assert_eq!(trie.search("grape"), true);

        // Restore trie to a previous of moment in time
        trie = snapshot;

        // Word was not present at snapshop moment
        assert_eq!(trie.search("grape"), false);
    }
}
