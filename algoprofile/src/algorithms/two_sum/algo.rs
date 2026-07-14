use std::collections::HashMap;
use algoprofile_macros::profile_algorithm;

// Profile with explicit sizes
#[profile_algorithm(sizes = [10, 100, 1000, 10000], iterations = 5)]
pub fn algo(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }
        map.insert(*num, i);
    }
    vec![]
}

// Profile with gradual range from 1 to 100
#[profile_algorithm(range = (1, 100), iterations = 3)]
pub fn algo_gradual(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }
        map.insert(*num, i);
    }
    vec![]
}

#[profile_algorithm(range = (1, 1000, 1), iterations = 5)]
pub fn algo_custom_step(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }
        map.insert(*num, i);
    }
    vec![]
}
