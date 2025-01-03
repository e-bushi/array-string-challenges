// There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.

// You are giving candies to these children subjected to the following requirements:

// Each child must have at least one candy.
// Children with a higher rating get more candies than their neighbors.
// Return the minimum number of candies you need to have to distribute the candies to the children.

use std::collections::HashMap;

pub enum CandyError {
    NoChildren
}

pub fn candy_distribution(children: Vec<i32>) -> Result<(i32, HashMap<usize, i32>), CandyError> {

    //Output
    //Minimum #
    let mut distribution: HashMap<usize, i32> = children
    .iter()
    .enumerate()
    .fold(HashMap::new(), | mut map, (dex, _) | {
        map
        .entry(dex)
        .or_insert(1);

        map
    });

    let mut left: usize = 0;
    let mut right: usize = 1;

    while right <= children.len() - 1 {

        if children[left] < children[right] {
            *distribution.get_mut(&right).unwrap() += 1;
        } else if children[left] > children[right] {
            *distribution.get_mut(&left).unwrap() += 1;
        }

        left += 1;
        right += 1;
    }

    let max_distribution = distribution.values().fold(0, |acc, val| acc + val);

    Ok((max_distribution, distribution))
}