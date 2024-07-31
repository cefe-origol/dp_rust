mod dp_trait;
extern crate dp_lib;

use dp_lib::*;

fn main() {
    println!("{}", fib(40));
    println!(
        "{}",
        knapsack(vec![3, 4, 5, 6, 10], vec![2, 3, 4, 5, 9], 5, 10)
    );
}

#[dp]
fn fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    (fib(n - 1) + fib(n - 2)) % 1_000_000_009
}

#[dp]
#[dp_extra(values: Vec<i32>, weights: Vec<i32>)]
//#[dp_default(n = values.len())]
fn knapsack(n: usize, k: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut ans = knapsack(n - 1, k);
    if k >= weights[n - 1] {
        ans = std::cmp::max(ans, knapsack(n - 1, k - weights[n - 1]) + values[n - 1]);
    }
    return ans;
}
