// You are given an integer array nums. You are initially positioned at the array's first index, 
// and each element in the array represents your maximum jump length at that position.

// Return true if you can reach the last index, or false otherwise.

pub fn can_reach(jumps: Vec<i32>) -> bool {

    if jumps.len() == 0 {
        return false
    }

    let mut is_possible = true;
    let mut dex = 0;

    while dex < jumps.len() - 1 {

        if jumps[dex] == 0 && dex < jumps.len() - 1 {
            is_possible = false;
            break;
        }

       dex += jumps[dex] as usize;
    }

    is_possible
}