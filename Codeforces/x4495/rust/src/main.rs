use std::io;
use std::cmp::max;
//use std::io::prelude::*;


fn main() {
    let nd = get_ints();
    let mut nums = get_ints();
    nums.sort();

    println!("{}", min_points(nd, nums));

}


fn min_points(nd : Vec<i32>, nums : Vec<i32>) -> i32 {
    let n = nd[0] as usize;
    let d = nd[1] ;

    let mut high : usize = 0;
    let mut ans : usize = 0;

    for  low in 0..n {
        while  high < n && nums[high] - nums[low] <= d {
            ans = max(ans, high - low);
            high += 1;
        }
    }

    (n-ans-1) as i32

}


fn get_ints() -> Vec<i32> {
    let mut input_text = String::new();
    io::stdin().read_line( &mut input_text )
               .expect( "could not read line" );
    let numbers : Vec<i32> = input_text.trim()
                                           .split_whitespace()
                                           .map(|x| x.parse::<i32>().unwrap())
                                           .collect();

    numbers
}