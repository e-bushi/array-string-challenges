// Given n non-negative integers representing an elevation map where the width of each bar is 1, 
// compute how much water it can trap after raining.

pub enum ElevationError {
    NotEnoughWalls,
}

pub fn max_rainwater(elevation: Vec<i32>) -> Result<i32, ElevationError> {
    if elevation.is_empty() || elevation.len() < 3 {
        return Err(ElevationError::NotEnoughWalls)
    }

    let mut total_rain_water = 0;
    let n = elevation.len();
    
    // For each element, find the maximum level of water it can trap
    for i in 1..n-1 {
        // Find maximum element on its left
        
        println!("Left slice: {:?}", &elevation[0..i]);
        let left_max = elevation[0..i].iter().max().unwrap();
        
        
        // Find maximum element on its right
        println!("Right slice: {:?}", &elevation[i+1..n]);
        let right_max = elevation[i+1..n].iter().max().unwrap();
        
        // Add the difference between smaller height and current height
        let min_height = left_max.min(right_max);
        println!("Left Max: {:?}", left_max);
        println!("Right Max: {:?} \n", right_max);
        if *min_height > elevation[i] {
            let difference = min_height - elevation[i];
            println!("Difference: {:?}\n", difference);
            total_rain_water += min_height - elevation[i];
        }
    }

    Ok(total_rain_water)
}


pub fn max_rainwater_optimized(elevation: Vec<i32>) -> Result<i32, ElevationError> {
    if elevation.is_empty() || elevation.len() < 3 {
        return Err(ElevationError::NotEnoughWalls);
    }

    let mut left = 0;
    let mut right = elevation.len() - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut total_rain_water = 0;

    while left < right {
        if elevation[left] < elevation[right] {
            if elevation[left] >= left_max {
                left_max = elevation[left];
            } else {
                total_rain_water += left_max - elevation[left];
            }
            left += 1;
        } else {
            if elevation[right] >= right_max {
                right_max = elevation[right];
            } else {
                total_rain_water += right_max - elevation[right];
            }
            right -= 1;
        }
    }

    Ok(total_rain_water)
}