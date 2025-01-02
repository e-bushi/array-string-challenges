// Given an integer array nums sorted in ascending order, 
// remove the duplicates so that each every element appears only once. 
// The relative order of the elements should be kept the same.

// Return k: the number of unique elements in nums. And return the modified array.
use std::collections::HashSet;

pub enum RemoveDupsError {
    EmptyArray,
}

pub fn remove_dups<'a>(nums: &'a mut Vec<i32>) -> Result<(&'a mut Vec<i32>, i32), RemoveDupsError> {

    if nums.is_empty() {
        return Err(RemoveDupsError::EmptyArray);
    }

    let mut unique_elements = HashSet::new();

    nums.retain(|&num| unique_elements.insert(num));

    Ok((nums, unique_elements.len() as i32))
}
