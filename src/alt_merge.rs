// You are given two strings word1 and word2. 
// Merge the strings by adding letters in alternating order, starting with word1. 
// If a string is longer than the other, append the additional letters onto the end of the merged string.

pub enum MergeError {

}

// Return the merged string.
pub fn merge_strings(first: String, second: String) -> Result<String, MergeError> {
    //output
    let mut ending_str: String = String::new();
    //Tracking
    let mut counter: usize = 0;
    let mut first_tracker: usize = 0;
    let mut second_tracker: usize = 0;

    let max_length = std::cmp::max(first.len(), second.len());

    while counter < max_length {
        if first_tracker < first.len() {
            ending_str.push(first.chars().nth(first_tracker).unwrap());
            first_tracker += 1;
        }

        if second_tracker < second.len() {
            ending_str.push(second.chars().nth(second_tracker).unwrap());
            second_tracker += 1;
        }

        counter += 1;
    }

    Ok(ending_str)
    
}