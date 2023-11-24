use std::cmp::max;

use crate::RefCounter;

pub enum AVL<K, V = ()> {
    Empty,
    Node {
        key: RefCounter<K>,
        value: RefCounter<V>,
        left: RefCounter<AVL<K, V>>,
        right: RefCounter<AVL<K, V>>,
    },
}

pub type OrderedMap<K, V> = AVL<K, V>;
pub type OrderedSet<K> = AVL<K>;

impl<K, V> Clone for AVL<K, V> {
    fn clone(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Node {
                key,
                value,
                left,
                right,
            } => Self::Node {
                key: key.clone(),
                value: value.clone(),
                left: left.clone(),
                right: right.clone(),
            },
        }
    }
}

impl<K: Ord> AVL<K> {
    pub fn insert(&self, value: K) -> Self {
        self.put(value, ())
    }
    pub fn search(&self, value: &K) -> bool {
        self.find(value).is_some()
    }
}

impl<K: Ord, V> AVL<K, V> {
    pub fn empty() -> AVL<K, V> {
        AVL::Empty
    }
    pub fn is_empty(&self) -> bool {
        matches!(self, AVL::Empty)
    }
    fn height(&self) -> i64 {
        match self {
            AVL::Empty => 0,
            AVL::Node {
                key: _,
                value: _,
                left,
                right,
            } => 1 + max(&left.height(), &right.height()),
        }
    }
    fn diff(&self) -> i64 {
        match self {
            AVL::Empty => 0,
            AVL::Node {
                key: _,
                value: _,
                left,
                right,
            } => left.height() - right.height(),
        }
    }
    pub fn find(&self, target_value: &K) -> Option<&V> {
        match self {
            AVL::Empty => Option::None,
            AVL::Node {
                key,
                value,
                left,
                right,
            } => match target_value.cmp(key) {
                std::cmp::Ordering::Less => left.find(target_value),
                std::cmp::Ordering::Equal => Option::Some(value.as_ref()),
                std::cmp::Ordering::Greater => right.find(target_value),
            },
        }
    }
    fn right_rotation(&self) -> AVL<K, V> {
        if let AVL::Node {
            key: x,
            value: vx,
            left: lt,
            right: t3,
        } = self
        {
            if let AVL::Node {
                key: y,
                value: vy,
                left: t1,
                right: t2,
            } = (*lt).as_ref()
            {
                return AVL::Node {
                    key: y.clone(),
                    left: t1.clone(),
                    value: vy.clone(),
                    right: RefCounter::new(AVL::Node {
                        key: x.clone(),
                        value: vx.clone(),
                        left: t2.clone(),
                        right: t3.clone(),
                    }),
                };
            }
        }
        self.clone()
    }
    fn right_fix(&self) -> AVL<K, V> {
        if let AVL::Node {
            key: x,
            value: vx,
            left: t1,
            right: t2,
        } = self
        {
            if t1.diff() == -1 {
                return AVL::Node {
                    key: x.clone(),
                    value: vx.clone(),
                    left: RefCounter::new(t1.left_rotation()),
                    right: t2.clone(),
                }
                .right_rotation();
            } else {
                return self.right_rotation();
            }
        }
        self.clone()
    }
    fn left_rotation(&self) -> AVL<K, V> {
        if let AVL::Node {
            key: x,
            value: vx,
            left: t1,
            right: rt,
        } = self
        {
            if let AVL::Node {
                key: y,
                value: vy,
                left: t2,
                right: t3,
            } = (*rt).as_ref()
            {
                return AVL::Node {
                    key: y.clone(),
                    value: vy.clone(),
                    left: RefCounter::new(AVL::Node {
                        key: x.clone(),
                        value: vx.clone(),
                        left: t1.clone(),
                        right: t2.clone(),
                    }),
                    right: t3.clone(),
                };
            }
        }
        self.clone()
    }
    fn left_fix(&self) -> AVL<K, V> {
        if let AVL::Node {
            key: x,
            value: vx,
            left: t1,
            right: t2,
        } = self
        {
            if t2.diff() == 1 {
                return AVL::Node {
                    key: x.clone(),
                    value: vx.clone(),
                    left: t1.clone(),
                    right: RefCounter::new(t2.right_rotation()),
                }
                .left_rotation();
            } else {
                return self.left_rotation();
            }
        }
        self.clone()
    }
    fn fix(&self) -> AVL<K, V> {
        match self.diff() {
            2 => self.right_fix(),
            -2 => self.left_fix(),
            _ => self.clone(),
        }
    }
    pub fn put(&self, key: K, value: V) -> AVL<K, V> {
        self.put_rc(RefCounter::new(key), RefCounter::new(value))
    }
    fn put_rc(&self, key_rc: RefCounter<K>, value_rc: RefCounter<V>) -> AVL<K, V> {
        match self {
            AVL::Empty => AVL::Node {
                key: key_rc,
                value: value_rc,
                left: RefCounter::new(AVL::Empty),
                right: RefCounter::new(AVL::Empty),
            },
            AVL::Node {
                key,
                value,
                left,
                right,
            } => match key_rc.cmp(key) {
                std::cmp::Ordering::Less => AVL::Node {
                    key: key.clone(),
                    value: value.clone(),
                    left: RefCounter::new(left.put_rc(key_rc, value_rc)),
                    right: right.clone(),
                }
                .fix(),
                std::cmp::Ordering::Equal => AVL::Node {
                    key: key_rc,
                    value: value_rc,
                    left: left.clone(),
                    right: right.clone(),
                },
                std::cmp::Ordering::Greater => AVL::Node {
                    key: key.clone(),
                    value: value.clone(),
                    left: left.clone(),
                    right: RefCounter::new(right.put_rc(key_rc, value_rc)),
                }
                .fix(),
            },
        }
    }
    pub fn delete(&self, target_key: &K) -> AVL<K, V> {
        match self {
            AVL::Empty => AVL::Empty,
            AVL::Node {
                key,
                value,
                left,
                right,
            } => {
                match target_key.cmp(key) {
                    std::cmp::Ordering::Less => {
                        let left_deleted = left.delete(target_key);
                        AVL::Node {
                            key: key.clone(),
                            value: value.clone(),
                            left: RefCounter::new(left_deleted),
                            right: right.clone(),
                        }
                        .fix()
                    }
                    std::cmp::Ordering::Equal => {
                        // Node with only one child or no child
                        if left.is_empty() {
                            return right.as_ref().clone();
                        } else if right.is_empty() {
                            return left.as_ref().clone();
                        }

                        // Node with two children, get the inorder predecessor (maximum value in the left subtree)
                        let inorder_predecessor = left.find_max();
                        if let Some((pred_key, pred_value)) = inorder_predecessor {
                            let left_deleted = left.delete(&pred_key);
                            AVL::Node {
                                key: pred_key.clone(),
                                value: pred_value.clone(),
                                left: RefCounter::new(left_deleted),
                                right: right.clone(),
                            }
                            .fix()
                        } else {
                            self.clone()
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        let right_deleted = right.delete(target_key);
                        AVL::Node {
                            key: key.clone(),
                            value: value.clone(),
                            left: left.clone(),
                            right: RefCounter::new(right_deleted),
                        }
                        .fix()
                    }
                }
            }
        }
    }

    fn find_max(&self) -> Option<(RefCounter<K>, RefCounter<V>)> {
        match self {
            AVL::Empty => None,
            AVL::Node {
                key,
                value,
                left: _,
                right,
            } => {
                if right.is_empty() {
                    Some((key.clone(), value.clone()))
                } else {
                    right.find_max()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avl_set() {
        let l = AVL::empty().insert(1).insert(2).insert(3).insert(4);
        let l2 = l.clone().insert(5);
        for i in 1..=4 {
            assert!(l.search(&i));
            assert!(l2.search(&i));
        }
        assert!(!l.search(&5));
        assert!(l2.search(&5));
    }

    #[test]
    fn test_avl_map() {
        let l = AVL::empty().put(1, 999);
        let l2 = l.clone().put(1, 123).put(2, 3);
        assert_eq!(l.find(&1), Some(&999));
        assert_eq!(l2.find(&1), Some(&123));
        assert!(l.find(&2).is_none());
        assert!(l2.find(&2).is_some());
    }

    #[test]
    fn test_avl_delete() {
        let l = AVL::empty()
            .insert(1)
            .insert(2)
            .insert(3)
            .insert(4)
            .insert(5);
        let l = l.delete(&3);
        assert!(!l.search(&3));
        assert!(l.search(&1));
        assert!(l.search(&2));
        assert!(l.search(&4));
        assert!(l.search(&5));
    }
}
