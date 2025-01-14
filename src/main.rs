// use candy::candy_distribution;
// mod merge;
// mod remove_x;
// mod remove_dups;
// mod majority;
// mod best_time;
// mod jump_game;
// mod candy;
// mod rain_water;
// mod alt_merge;
mod string_divisor;
fn main() {
    println!("Hello, world!");

    // let mut a = vec![1, 2, 3, 0, 0, 0];
    // let b = vec![2, 5, 6];
    // let _ = merge::merge(&mut a, &b, 3, 3);

    // let mut a = vec![1, 2, 3, 3, 5, 7];
    // let _ = remove_x::remove_x(&mut a, 3);

    // let mut a = vec![1, 2, 3, 3, 5, 7, 2, 11, 4, 1];
    // let _ = remove_dups::remove_dups(&mut a);

    // let mut a = vec![1, 2, 3, 3, 5, 7, 2, 11, 4, 1];
    // let val = majority::majority_element(&mut a);

    // let prices = vec![7, 1, 5, 3, 6, 4];
    // let val = best_time::max_profit(prices);

    // match val {
    //     Ok(profit) => println!("{}", profit),
    //     Err(e) => println!("NA"),
    // }

    // let a = vec![14, 8, 3, 4, 4, 6, 14, 2, 13, 10];
    // let max_profit = best_time::max_profit(&a);

    // match max_profit {
    //     Ok(vals) => println!("Buy Price: {}, Sell Price: {}, Max Profit: {}", a[vals.0], &a[vals.1], vals.2),
    //     Err(_) => println!("{}", "No Profit"),
    // }

    // let a = vec![3,2,1,0,4];
    // let val = jump_game::can_reach(a);
    // println!("{}", val);

    // let children = vec![1, 0, 2];
    // let total_distribution = candy_distribution(children);

    // match total_distribution {
    //     Ok(distro) => println!("Total: {}, Distro: {:?}", distro.0, distro.1),
    //     _ => println!("Something went wrong")
    // }

    // let wall_sizes = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    // let rain_water_collected = max_rainwater(wall_sizes);

    // match rain_water_collected {
    //     Ok(amount) => println!("Total Rain: {}", amount),
    //     _ => println!("Something went wrong")
    // }

    // let word1 = "abc".to_string();
    // let word2 = "pqr".to_string();
    // let merged = alt_merge::merge_strings(word1, word2);
    // match merged {
    //     Ok(merged_str) => println!("{}", merged_str),
    //     Err(_) => println!("There was an Error Merging"),
    // }

    let word1 = "abcabcabcabc".to_string();
    let word2 = "abc".to_string();
    let divisor = string_divisor::string_divisor(word1, word2);
    match divisor {
        Ok(divisor) => println!("Here is the divisor: {}", divisor),
        Err(_) => println!("There was an Error Merging"),
    }

}
