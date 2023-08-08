pub mod avl;
#[cfg(feature = "thread_safe")]
pub type RefCounter<T> = std::sync::Arc<T>;

#[cfg(not(feature = "thread_safe"))]
pub type RefCounter<T> = std::rc::Rc<T>;

pub mod deque;
pub mod hashmap;
pub mod list;
pub mod trie;
