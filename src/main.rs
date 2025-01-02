// mod merge;
// mod remove_x;
// mod remove_dups;
// mod majority;
mod best_time;
// mod jump_game;
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

    let a = vec![14, 8, 3, 4, 4, 6, 14, 2, 13, 10];
    let max_profit = best_time::max_profit(&a);

    match max_profit {
        Ok(vals) => println!("Buy Price: {}, Sell Price: {}, Max Profit: {}", a[vals.0], &a[vals.1], vals.2),
        Err(_) => println!("{}", "No Profit"),
    }

    // let a = vec![3,2,1,0,4];
    // let val = jump_game::can_reach(a);
    // println!("{}", val);

}