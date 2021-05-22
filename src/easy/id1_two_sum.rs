// // https://rustgym.com/leetcode/1

use std::collections::HashMap;

pub struct TwoSum;

impl TwoSum {
    fn _check_input(nums: &Vec<i32>) {
        if nums.len() < 2 {
            panic!("'nums' must have at least two items.")
        }
    }
    // Time Complexity: O(n^2)
    // For each element, we try to find its complement by looping through the rest of array which takes O(n)O(n) time.
    // Space complexity: O(1)
    fn brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::_check_input(&nums);

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        panic!("No items sum to the target.")
    }
    // Time Complexity: O(2n) -> O(n)
    // The list is traversed two times but the look up time is reduced by the hash map to O(1).
    // Space Complexity: O(n)
    // Space is been traded for time.
    fn two_pass_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::_check_input(&nums);

        let mut hm: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (i, num) in nums.iter().enumerate() {
            hm.insert(*num, i as i32);
        }

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&stored_index) = hm.get(&complement) {
                if stored_index != i as i32 {
                    return vec![i as i32, stored_index];
                }
            }
        }

        panic!("No items sum to the target.")
    }
    // Time Complexity: O(n)
    // The list is traversed only once. Each look up in the hash map costs O(1).
    // Space Complexity: O(n)
    // Like in the previous fn, the hashmap stores at most n elements.
    fn one_pass_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::_check_input(&nums);

        let mut prev: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(p) = prev.get(&complement) {
                return vec![*p as i32, i as i32];
            }

            prev.insert(*num, i as i32);
        }

        panic!("No items sum to the target")
    }
}

#[test]
fn test_brute_force() {
    assert_eq!(TwoSum::brute_force(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test_two_pass_hash_map() {
    assert_eq!(TwoSum::two_pass_hash_map(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test_one_pass_hash_map() {
    assert_eq!(TwoSum::one_pass_hash_map(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
