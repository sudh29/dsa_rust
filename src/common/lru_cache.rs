use std::collections::HashMap;
use std::hash::Hash;

/// Node in the doubly-linked arena for LRU eviction tracking.
#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    val: V,
    prev: Option<usize>,
    next: Option<usize>,
}

/// A fixed-capacity generic Least Recently Used (LRU) Cache in 100% safe Rust.
///
/// # Complexity
/// - `get`: $O(1)$ amortized
/// - `put`: $O(1)$ amortized
/// - Space: $O(\text{capacity})$
///
/// # Examples
/// ```
/// use dsa_rust::common::lru_cache::LruCache;
///
/// let mut cache = LruCache::new(2);
/// cache.put(1, "one");
/// cache.put(2, "two");
/// assert_eq!(cache.get(&1), Some(&"one"));
///
/// cache.put(3, "three"); // Evicts key 2
/// assert_eq!(cache.get(&2), None);
/// assert_eq!(cache.get(&3), Some(&"three"));
/// ```
#[derive(Debug, Clone)]
pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>,
    head: Option<usize>, // Most recently used
    tail: Option<usize>, // Least recently used
    free: Vec<usize>,
}

impl<K: Hash + Eq + Clone, V> LruCache<K, V> {
    /// Creates a new `LruCache` with the specified maximum `capacity`.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than zero");
        LruCache {
            capacity,
            map: HashMap::with_capacity(capacity),
            nodes: Vec::with_capacity(capacity),
            head: None,
            tail: None,
            free: Vec::new(),
        }
    }

    /// Retrieves a reference to the value corresponding to the key, updating access order.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let &idx = self.map.get(key)?;
        self.move_to_head(idx);
        Some(&self.nodes[idx].val)
    }

    /// Inserts a key-value pair into the cache, evicting the least recently used element if full.
    pub fn put(&mut self, key: K, val: V) {
        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].val = val;
            self.move_to_head(idx);
            return;
        }

        if self.map.len() >= self.capacity {
            self.evict();
        }

        let idx = if let Some(free_idx) = self.free.pop() {
            self.nodes[free_idx] = Node {
                key: key.clone(),
                val,
                prev: None,
                next: self.head,
            };
            free_idx
        } else {
            let new_idx = self.nodes.len();
            self.nodes.push(Node {
                key: key.clone(),
                val,
                prev: None,
                next: self.head,
            });
            new_idx
        };

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(idx);
        }
        self.head = Some(idx);
        if self.tail.is_none() {
            self.tail = Some(idx);
        }
        self.map.insert(key, idx);
    }

    fn move_to_head(&mut self, idx: usize) {
        if self.head == Some(idx) {
            return;
        }

        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;

        if let Some(p) = prev {
            self.nodes[p].next = next;
        }
        if let Some(n) = next {
            self.nodes[n].prev = prev;
        }
        if self.tail == Some(idx) {
            self.tail = prev;
        }

        self.nodes[idx].prev = None;
        self.nodes[idx].next = self.head;

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(idx);
        }
        self.head = Some(idx);
    }

    fn evict(&mut self) {
        let tail_idx = match self.tail {
            Some(idx) => idx,
            None => return,
        };

        let prev = self.nodes[tail_idx].prev;
        if let Some(p) = prev {
            self.nodes[p].next = None;
            self.tail = Some(p);
        } else {
            self.head = None;
            self.tail = None;
        }

        let key = &self.nodes[tail_idx].key;
        self.map.remove(key);
        self.free.push(tail_idx);
    }

    /// Returns the current number of elements in the cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_operations() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get(&1), Some(&10));

        // Adding 3 should evict key 2 because key 1 was recently accessed
        cache.put(3, 30);
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&10));
        assert_eq!(cache.get(&3), Some(&30));

        // Overwrite existing key
        cache.put(1, 100);
        assert_eq!(cache.get(&1), Some(&100));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_capacity_one() {
        let mut cache = LruCache::new(1);
        cache.put("a", 1);
        assert_eq!(cache.get(&"a"), Some(&1));
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
    }
}
