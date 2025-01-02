// You are given two integer arrays nums1 and nums2, sorted in ascending order, and two integers m and n, 
// representing the number of elements in nums1 and nums2 respectively.

// Merge nums1 and nums2 into a single array sorted in ascending order.

// The final sorted array should not be returned by the function, 
// but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, 
// where the first m elements denote the elements that should be merged, 
// and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

pub enum MergeError { 
    EmptyArray,
    OutOfBounds
}

pub fn merge<'a>(a: &'a mut Vec<i32>, b: &'a Vec<i32>, m: usize, n: usize) -> Result<&'a Vec<i32>, MergeError> {
    if a.is_empty() || b.is_empty() {
        return Err(MergeError::EmptyArray);
    }

    if m + n != a.len() {
        return Err(MergeError::OutOfBounds);
    }

    let mut first_zero_index = a.iter().position(|&x| x == 0).unwrap();

    println!("first_zero_index: {}", first_zero_index);

    let mut b_index = 0;

    for integer in a.iter_mut() {
        if *integer == 0 {

            if b_index >= b.len() {
                break;
            }

            *integer = b[b_index];
            b_index += 1;
        }
    }

    a.sort();

    Ok(a)
}
