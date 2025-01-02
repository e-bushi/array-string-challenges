
// Given an array nums of size n, return the majority element.
// The majority element is the element that appears more than ⌊n / 2⌋ times. 
// You may assume that the majority element always exists in the array.

//If there are multiple majority elements, return the smallest one.
use std::collections::HashMap;
pub fn majority_element(nums: &mut [i32]) -> i32 {

    nums.sort();

    let mut counts = HashMap::new();
    for num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    let majority_element = counts.iter().max_by_key(|entry| entry.1).unwrap().0;

    return **majority_element;
}
