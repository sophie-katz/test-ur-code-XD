// Copyright (c) 2023 Sophie Katz
//
// This file is part of test ur code XD.
//
// test ur code XD is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// test ur code XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with test ur code XD. If
// not, see <https://www.gnu.org/licenses/>.

use std::{collections::HashMap, hash::Hash};

/// Takes the first key and value from a [`HashMap`] and returns them as a tuple.
///
/// It also removes the key from the hash map.
///
/// # Example
///
/// ```ignore
/// // Create a hash map with some values
/// let hash_map = HashMap::new();
///
/// hash_map.insert("a", 1);
/// hash_map.insert("b", 2);
/// hash_map.insert("c", 3);
///
/// // Take the first key-value pair
/// match take_first_key_and_value(&mut hash_map) {
///     None => panic!("the hash map is empty"),
///     Some((first_key, first_value)) => {
///        // We're not sure which key we'll get, because hash maps are unordered
///     }
/// }
///
/// // There are now only two left
/// assert_eq!(hash_map.len(), 2);
/// ```
///
/// # Returns
///
/// * `Some((first_key, first_value))` if there is at least one key in the hash map
/// * `None` if the hash map is empty
fn take_first_key_and_value<KeyType: Clone + Eq + Hash, ValueType>(
    hash_map: &mut HashMap<KeyType, ValueType>,
) -> Option<(KeyType, ValueType)> {
    let first_key = hash_map.keys().next()?.clone();

    let first_value = hash_map.remove(&first_key)?;

    Some((first_key, first_value))
}

/// Helper method that actually does the recursion
fn permute_map_of_vecs_helper<KeyType: 'static + Clone + Eq + Hash, ValueType: 'static + Clone>(
    mut map_of_vecs: HashMap<KeyType, Vec<ValueType>>,
) -> Vec<HashMap<KeyType, ValueType>> {
    if let Some((first_key, first_vec)) = take_first_key_and_value(&mut map_of_vecs) {
        if map_of_vecs.is_empty() {
            first_vec
                .iter()
                .map(move |value| HashMap::from([(first_key.clone(), value.clone())]))
                .collect()
        } else {
            permute_map_of_vecs_helper(map_of_vecs)
                .into_iter()
                .flat_map(|map| {
                    let first_key = first_key.clone();

                    first_vec.iter().map(move |value| {
                        let mut map = map.clone();

                        map.insert(first_key.clone(), value.clone());

                        map
                    })
                })
                .collect()
        }
    } else {
        Vec::new()
    }
}

/// Permutes a hash map of vectors into a vector of hash maps.
///
/// If there is a hash map of vectors like this:
///
/// ```json
/// {
///     "a": [1, 2, 3],
///     "b": [4, 5, 6],
///     "c": [7, 8, 9],
/// }
/// ```
///
/// It will permute them into:
///
/// ```json
/// [
///     {"a": 1, "b": 4, "c": 7},
///     {"a": 2, "b": 4, "c": 7},
///     {"a": 3, "b": 4, "c": 7},
///     // ...
///     {"a": 1, "b": 6, "c": 9},
///     {"a": 2, "b": 6, "c": 9},
///     {"a": 3, "b": 6, "c": 9},
/// ]
/// ```
///
/// # Details
///
/// * If any of the vectors is empty, it will be ignored and the key will not be included in the
///   resulting hash maps.
/// * If there are no keys, it will return an empty vector.
/// * Because the size of the resulting array is the product of the sizes of all non-empty vectors,
///   the result can become quite large.
pub fn permute_map_of_vecs<KeyType: 'static + Clone + Eq + Hash, ValueType: 'static + Clone>(
    map_of_vecs: HashMap<KeyType, Vec<ValueType>>,
) -> Vec<HashMap<KeyType, ValueType>> {
    permute_map_of_vecs_helper(
        map_of_vecs
            .into_iter()
            .filter(|(_, value)| !value.is_empty())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_first_key_and_value_empty() {
        let mut hash_map: HashMap<String, String> = HashMap::new();

        let first_key_and_value = take_first_key_and_value(&mut hash_map);

        assert!(first_key_and_value.is_none());

        assert!(hash_map.is_empty());
    }

    #[test]
    fn take_first_key_and_value_one() {
        let mut hash_map: HashMap<String, String> =
            HashMap::from([("k0".to_owned(), "v0".to_owned())]);

        let first_key_and_value = take_first_key_and_value(&mut hash_map);

        assert_eq!(
            first_key_and_value,
            Some(("k0".to_owned(), "v0".to_owned()))
        );

        assert!(hash_map.is_empty());
    }

    #[test]
    fn take_first_key_and_value_two() {
        let mut hash_map: HashMap<String, String> = HashMap::from([
            ("k0".to_owned(), "v0".to_owned()),
            ("k1".to_owned(), "v1".to_owned()),
        ]);

        let first_key_and_value = take_first_key_and_value(&mut hash_map);

        assert!(first_key_and_value.is_some());

        assert!(hash_map.len() == 1);

        if first_key_and_value.as_ref().unwrap().0 == "k0" {
            assert!(first_key_and_value.as_ref().unwrap().1 == "v0");
            assert!(hash_map.contains_key("k1"));
            assert!(hash_map["k1"] == "v1");
        } else if first_key_and_value.as_ref().unwrap().0 == "k1" {
            assert!(first_key_and_value.as_ref().unwrap().1 == "v1");
            assert!(hash_map.contains_key("k0"));
            assert!(hash_map["k0"] == "v0");
        } else {
            panic!();
        }
    }

    #[test]
    fn empty() {
        let map_of_vecs: HashMap<String, Vec<String>> = HashMap::new();

        let vec_of_maps = permute_map_of_vecs(map_of_vecs);

        assert!(vec_of_maps.is_empty());
    }

    #[test]
    fn one_empty_key() {
        let map_of_vecs: HashMap<String, Vec<String>> =
            HashMap::from([("k0".to_owned(), Vec::new())]);

        let vec_of_maps = permute_map_of_vecs(map_of_vecs);

        assert!(vec_of_maps.is_empty());
    }

    #[test]
    fn one_key_one_value() {
        let map_of_vecs: HashMap<String, Vec<String>> =
            HashMap::from([("k0".to_owned(), vec!["v0".to_owned()])]);

        let vec_of_maps = permute_map_of_vecs(map_of_vecs);

        assert_eq!(vec_of_maps.len(), 1);
        assert_eq!(vec_of_maps[0].len(), 1);
        assert_eq!(vec_of_maps[0]["k0"], "v0");
    }

    #[test]
    fn one_key_two_values() {
        let map_of_vecs: HashMap<String, Vec<String>> =
            HashMap::from([("k0".to_owned(), vec!["v0".to_owned(), "v1".to_owned()])]);

        let vec_of_maps = permute_map_of_vecs(map_of_vecs);

        assert_eq!(vec_of_maps.len(), 2);
        assert_eq!(vec_of_maps[0].len(), 1);
        assert_eq!(vec_of_maps[0]["k0"], "v0");
        assert_eq!(vec_of_maps[1].len(), 1);
        assert_eq!(vec_of_maps[1]["k0"], "v1");
    }

    #[test]
    fn two_keys_one_value_and_one_empty() {
        let map_of_vecs: HashMap<String, Vec<String>> = HashMap::from([
            ("k0".to_owned(), vec!["v0".to_owned()]),
            ("k1".to_owned(), Vec::new()),
        ]);

        let vec_of_maps = permute_map_of_vecs(map_of_vecs);

        assert_eq!(vec_of_maps.len(), 1);
        assert_eq!(vec_of_maps[0].len(), 1);
        assert_eq!(vec_of_maps[0]["k0"], "v0");
    }

    #[test]
    fn two_keys_one_with_two_values_and_one_empty() {
        let map_of_vecs: HashMap<String, Vec<String>> = HashMap::from([
            ("k0".to_owned(), vec!["v0".to_owned(), "v1".to_owned()]),
            ("k1".to_owned(), Vec::new()),
        ]);

        let vec_of_maps = permute_map_of_vecs(map_of_vecs);

        assert_eq!(vec_of_maps.len(), 2);
        assert_eq!(vec_of_maps[0].len(), 1);
        assert_eq!(vec_of_maps[0]["k0"], "v0");
        assert_eq!(vec_of_maps[1].len(), 1);
        assert_eq!(vec_of_maps[1]["k0"], "v1");
    }

    #[test]
    fn two_keys_one_value_and_one_value() {
        let map_of_vecs: HashMap<String, Vec<String>> = HashMap::from([
            ("k0".to_owned(), vec!["v0".to_owned()]),
            ("k1".to_owned(), vec!["v1".to_owned()]),
        ]);

        let vec_of_maps = permute_map_of_vecs(map_of_vecs);

        assert_eq!(vec_of_maps.len(), 1);
        assert_eq!(vec_of_maps[0].len(), 2);
        assert_eq!(vec_of_maps[0]["k0"], "v0");
        assert_eq!(vec_of_maps[0]["k1"], "v1");
    }

    #[test]
    fn two_keys_one_with_two_values_and_one_value() {
        let map_of_vecs: HashMap<String, Vec<String>> = HashMap::from([
            ("k0".to_owned(), vec!["v0".to_owned(), "v1".to_owned()]),
            ("k1".to_owned(), vec!["v2".to_owned()]),
        ]);

        let vec_of_maps = permute_map_of_vecs(map_of_vecs);

        assert_eq!(vec_of_maps.len(), 2);
        assert_eq!(vec_of_maps[0].len(), 2);
        assert_eq!(vec_of_maps[0]["k0"], "v0");
        assert_eq!(vec_of_maps[0]["k1"], "v2");
        assert_eq!(vec_of_maps[1].len(), 2);
        assert_eq!(vec_of_maps[1]["k0"], "v1");
        assert_eq!(vec_of_maps[1]["k1"], "v2");
    }

    #[test]
    fn two_keys_one_with_two_values_and_one_with_two_values() {
        let map_of_vecs: HashMap<String, Vec<String>> = HashMap::from([
            ("k0".to_owned(), vec!["v0".to_owned(), "v1".to_owned()]),
            ("k1".to_owned(), vec!["v2".to_owned(), "v3".to_owned()]),
        ]);

        let vec_of_maps = permute_map_of_vecs(map_of_vecs);

        assert_eq!(vec_of_maps.len(), 4);
        assert_eq!(vec_of_maps[0].len(), 2);
        assert_eq!(vec_of_maps[1].len(), 2);
        assert_eq!(vec_of_maps[2].len(), 2);
        assert_eq!(vec_of_maps[3].len(), 2);
    }
}
