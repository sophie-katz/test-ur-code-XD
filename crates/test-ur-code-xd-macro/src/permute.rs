// Copyright (c) 2023 Sophie Katz
//
// This file is part of test-ur-code-XD.
//
// test-ur-code-XD is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// test-ur-code-XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with test-ur-code-XD. If
// not, see <https://www.gnu.org/licenses/>.

use std::{collections::HashMap, hash::Hash};

fn take_first_key_and_values<KeyType: Clone + Eq + Hash, ValueType>(
    unpermuted: &mut HashMap<KeyType, Vec<ValueType>>,
) -> (KeyType, Vec<ValueType>) {
    let first_key = unpermuted.keys().next().unwrap().clone();
    let first_values = unpermuted.remove(&first_key).unwrap();

    (first_key, first_values)
}

pub fn permute_maps<KeyType: Clone + Eq + Hash, ValueType: Clone>(
    mut unpermuted: HashMap<KeyType, Vec<ValueType>>,
) -> Vec<HashMap<KeyType, ValueType>> {
    let mut permuted: Vec<HashMap<KeyType, ValueType>> = Vec::new();

    if unpermuted.is_empty() {
        permuted
    } else if unpermuted.len() == 1 {
        let (first_key, first_values) = take_first_key_and_values(&mut unpermuted);

        for value in first_values {
            let mut permutation = HashMap::new();
            permutation.insert(first_key.clone(), value);

            permuted.push(permutation);
        }

        permuted
    } else {
        let (first_key, first_values) = take_first_key_and_values(&mut unpermuted);

        let next_permutations = permute_maps(unpermuted);

        for next_permutation in next_permutations.into_iter() {
            for value in &first_values {
                let mut permutation = next_permutation.clone();

                permutation.insert(first_key.clone(), value.clone());

                permuted.push(permutation);
            }
        }

        permuted
    }
}
