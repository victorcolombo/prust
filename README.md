## PRust: (P)ersistent & Immutable Data Structures in (Rust)

This library houses a collection of immutable and persistent data structures, inspired by the standard libraries found in Haskell, OCaml, Closure and Okasaki's *Purely Functional Data Structures* book.

It does NOT contain:
- Unsafe memory access (no `unsafe` use)
- Methods taking mutable references
- External dependencies

### What's Prust Good For?

**Persistent Data Structures**: Ever wanted to undo an operation on a data structure? With persistent data structures, you can access prior versions, essentially maintaining a history of change made as needed.

```rust
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
```

**Immutable Data Structures**: Data structures, once created, do not change. Instead of modifying, a new "view" is created.

```rust
// deque: [2, 1]
let deque: Deque<i32> = Deque::empty().push_front(1).push_front(2);

// Even with pop_back, deque is still unaltered, since it's immutable
let (value, _) = deque.pop_back().unwrap();
assert_eq!(*value, 1);
assert_eq!(deque.length(), 2);

// The "new" deque with pop_back is returned as part of the call
let (value, deque_updated) = deque.pop_front().unwrap();
assert_eq!(*value, 2);
assert_eq!(deque_updated.length(), 1);
```

**Multithreading Friendliness**: Thanks to their immutability, Prust's data structures are inherently thread-safe. See more on section below.

### Avaliable data structures

- Trie (aka Prefix Tree)
- Hash Map / Hash Set (based on Trie)
- AVL tree
- Ordered Map / Ordered Set (based on AVL)
- Stack (aka Cons List)
- Deque

### Thread Safety

For performance reasons, thread safety is opt in. To enable the `thread_safe` feature, add the following to your `Cargo.toml`:
```toml
[dependencies.prust_lib]
version = "version"
features = ["thread_safe"]
```

This switches the reference counting from `std::rc::Rc` to `std::sync::Arc`.

### How Does Prust Work?

Instead of in-place updates, whenever a mutable-like operation is invoked (e.g., adding a value to a set), Prust returns a "copy" of the new updated structure, leaving the original untouched. This ensures both persistence (by retaining prior versions) and immutability (since the original remains unchanged).

This is acomplished by reusing most parts of the original structure and only building new pieces relevant to the updated version.

A colorary is that cloning these structures is efficient as they are usually represented by only a few pointers.