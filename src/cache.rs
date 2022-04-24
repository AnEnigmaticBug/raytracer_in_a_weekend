use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Index;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cache<T> {
    idxs: HashMap<String, usize>,
    items: Vec<T>,
}

impl<T> Cache<T> {
    pub fn new() -> Self {
        Self {
            idxs: HashMap::new(),
            items: vec![],
        }
    }

    /// Adds an item along with its name and returns an index that can be used
    /// to retrieve the item later.
    ///
    /// WARNING: Two items can't share the same name.
    pub fn add<K: Clone + Debug + Into<String>>(&mut self, name: K, item: T) -> usize {
        let new_idx = self.items.len();
        assert!(
            self.idxs.insert(name.clone().into(), new_idx).is_none(),
            "Item with name {:?} already exists",
            name
        );
        self.items.push(item);
        new_idx
    }

    /// Retrieves the index for the item with the given name.
    ///
    /// There's *no* method that retrieves the item directly for a given name.
    /// This is intentional. Name-based lookups should be cumbersome. Callers
    /// should use indexes for much faster lookups.
    pub fn idx_for(&self, name: &str) -> Option<usize> {
        self.idxs.get(name).copied()
    }
}

impl<T> Default for Cache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for Cache<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Cache;

    #[test]
    fn adding_an_item_one_time_is_allowed() {
        let mut cache = Cache::new();
        cache.add("unique", 1);
    }

    #[test]
    #[should_panic(expected = "Item with name \"same\" already exists")]
    fn adding_an_item_multiple_times_isnt_allowed() {
        let mut cache = Cache::new();
        cache.add("same", 1);
        cache.add("same", 2);
    }

    #[test]
    fn added_item_can_be_retrieved_using_idx() {
        let mut cache = Cache::new();
        let idx = cache.add("name", 1822);
        assert_eq!(1822, cache[idx]);
    }

    #[test]
    fn added_items_idx_can_be_retrieved_using_name() {
        let mut cache = Cache::new();
        let idx = cache.add("name", 1822);
        assert_eq!(Some(idx), cache.idx_for("name"));
    }
}
