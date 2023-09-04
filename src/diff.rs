use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// The final result of Diff
///
/// So the dev can know When to update
/// ,insert or delete item
#[derive(Debug, PartialEq)]
pub enum Difference<T> {
    /// Update the existing Data
    ///
    /// First T is target and second is
    /// the update (new object)
    Update(T, T),

    /// New Data
    Insert(T),

    /// Removed Data
    Remove(T),
}
/// Finds the difference between Two Vectors
pub struct Diff<'a, T>
where
    T: Hash + Eq + PartialEq,
{
    target: HashMap<&'a str, T>,
    new: &'a [(&'a str, T)],
}

impl<'a, T> Diff<'a, T>
where
    T: Hash + Eq + PartialEq + Sized + Ord + Debug,
{
    /// Creates a new Diff Object
    pub fn new(target: HashMap<&'a str, T>, new: &'a [(&'a str, T)]) -> Self {
        Self { new, target }
    }

    pub fn diff(&self) -> Vec<Difference<&T>> {
        let mut result = vec![];

        for (name, object) in self.new {
            match self.target.get(name) {
                Some(i) => {
                    if i != object {
                        result.push(Difference::Update(i, object));
                    }
                }
                None => result.push(Difference::Insert(object)),
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, Ord, Eq, PartialEq, Debug, PartialOrd)]
    struct TestData<'a> {
        name: &'a str,
        value: &'a str,
    }

    #[test]
    fn test_diff() {
        let target = HashMap::from([
            (
                "Hello",
                TestData {
                    name: "Hello",
                    value: "true",
                },
            ),
            (
                "OkMa",
                TestData {
                    name: "OOK",
                    value: "OOK",
                },
            ),
        ]);

        let new = vec![
            (
                "Hello",
                TestData {
                    name: "Hello",
                    value: "false",
                },
            ),
            (
                "Ok",
                TestData {
                    name: "Ok",
                    value: "test",
                },
            ),
        ];

        let diff = Diff::new(target, &new);
        let result = diff.diff();

        let expected = vec![
            Difference::Update(
                &TestData {
                    name: "Hello",
                    value: "true",
                },
                &TestData {
                    name: "Hello",
                    value: "false",
                },
            ),
            Difference::Insert(&TestData {
                name: "Ok",
                value: "test",
            }),
        ];

        
        assert_eq!(expected, result);
    }
}
