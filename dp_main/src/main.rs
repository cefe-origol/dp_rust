mod dp_trait;
extern crate dp_lib;

use dp_lib::*;
use num_bigint::{BigInt, ToBigInt};

fn main() {
    println!("{}", fib(1000_i32.to_bigint().unwrap()));
    println!(
        "{}",
        knapsack(vec![3, 4, 5, 6, 10], vec![2, 3, 4, 5, 9], 5, 10)
    );
}

#[dp]
fn fib(n: BigInt) -> BigInt {
    if n.clone() < 2_i32.to_bigint().unwrap() {
        return n;
    }
    (fib(n.clone() - 1_i32.to_bigint().unwrap()) + fib(n.clone() - 2_i32.to_bigint().unwrap())) % 1_000_000_009_i32.to_bigint().unwrap()
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
