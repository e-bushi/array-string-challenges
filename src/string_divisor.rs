// For two strings s and t, we say "t divides s" if and only if s = t + t + t + ... + t + t 
// (i.e., t is concatenated with itself one or more times).

// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.

//Basically return the largest repeating string found in both strings

pub enum StringDivisorError {
    NoCommonDivisor,
    EmptyString
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

pub fn string_divisor(first: String, second: String) -> Result<String, StringDivisorError> {

    if first.is_empty() || second.is_empty() {
        return Err(StringDivisorError::EmptyString);
    }

    let is_repeating = |s: &str, sub: &str| -> bool {
        if s.len() % sub.len() != 0 {
            return false
        }

        let repetition = s.len() / sub.len();

        sub.repeat(repetition) == s
    };


    let potential_len = greatest_common_divisor(first.len(), second.len());
    println!("Here is the potential length: {}", potential_len);
    let potential_divisor = &first[0..potential_len];
    println!("Here is the potential divisor: {}", potential_divisor);
    if is_repeating(&first, potential_divisor) && is_repeating(&second, potential_divisor) {
        Ok(potential_divisor.to_string())
    } else {
        Err(StringDivisorError::NoCommonDivisor)
    }

}