use std::{collections::HashMap, vec};

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements = HashMap::new();
    for i in 0..nums.len() {
        let complement = target - nums[i];
        if let Some(j) = complements.get(&complement) {
            if *j != i {
                return vec![*j as i32, i as i32];
            }
        }
        complements.insert(nums[i], i);
    }
    vec![]
}

fn main() {
    // You can add test cases here
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("Input: nums = {:?}, target = {}", nums, target);
    println!("Output: {:?}", two_sum(nums, target));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);

        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2]);

        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}