// Given two parameters, an integer array nums and an integer a, remove all occurrences of a in nums in-place. 
// The order of the elements may be changed.

// Then return the number of elements in nums which are not equal to a.

// Consider the number of elements in nums which are not equal to a be k. To get accepted, you need to do the following things:

// Change the array nums such that the first k elements of nums contain the elements which are not equal to a. 
// The remaining elements of nums are not important as well as the size of nums.
// Return k.

pub enum RemoveXError {
    EmptyArray,
}

pub fn remove_x<'a>(nums: &'a mut Vec<i32>, a: i32) -> Result<i32, RemoveXError> {

    if nums.is_empty() {
        return Err(RemoveXError::EmptyArray);
    }

    let mut k = 0;
    let mut indices = vec![];
    
    for (index, num) in nums.iter_mut().enumerate() {
        if *num == a {
            k += 1;
            indices.push(index);
        }
    }

    for index in indices.iter().rev() {
        nums.remove(*index);
    }

    Ok(k)
}