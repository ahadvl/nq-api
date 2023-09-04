use std::{collections::HashSet, hash::Hash};

pub enum Difference<T> {
    Update(T),
    Insert(T),
    Remove(T),
}

/// Finds the difference between Two Vectors
pub struct Diff<'a, T>
where
    T: Hash + Eq + PartialEq + Sized,
{
    target_vec: &'a [T],
    new_vec: &'a [T],
}

impl<'a, T> Diff<'a, T>
where
    T: Hash + Eq + PartialEq + Sized,
{
    /// Creates a new Diff Object
    pub fn new(target_vec: &'a [T], new_vec: &'a [T]) -> Self {
        Self {
            new_vec,
            target_vec,
        }
    }

    /// Collect ony the Unique items (skip unchaged items)
    fn get_unique_items(&self) -> HashSet<&T> {
        let mut set: HashSet<&T> = HashSet::new();

        for item in self.target_vec {
            set.insert(item);
        }

        for item in self.new_vec {
            set.insert(item);
        }

        set
    }

    pub fn diff(&self) -> Vec<Difference<T>> {
        let unique_items = self.get_unique_items();

        todo!()
    }
}
