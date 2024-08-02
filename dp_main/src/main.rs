mod dp_trait;
extern crate dp_lib;

use dp_lib::*;
use num_bigint::{BigInt, ToBigInt};

fn main() {
    println!("{}", fib(1000_i32.to_bigint().unwrap()));
    println!(
        "{}",
        knapsack(vec![3, 4, 5, 6, 10], vec![2, 3, 4, 5, 9], 10)
    );
    println!("{}", edit_distance("love".into(), "movie".into()))
}

#[dp]
fn fib(n: BigInt) -> BigInt {
    if n.clone() < 2_i32.to_bigint().unwrap() {
        return n;
    }
    (fib(n.clone() - 1_i32.to_bigint().unwrap()) + fib(n.clone() - 2_i32.to_bigint().unwrap()))
        % 1_000_000_007_i32.to_bigint().unwrap()
}

#[dp]
#[dp_extra(values: Vec<i32>, weights: Vec<i32>)]
#[dp_default(n = values.len())]
fn knapsack(n: usize, k: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut ans = knapsack(n - 1, k);
    if k >= weights[n - 1] {
        ans = std::cmp::max(ans, knapsack(n - 1, k - weights[n - 1]) + values[n - 1]);
    }
    ans
}

#[dp]
#[dp_extra(a: String, b: String)]
#[dp_default(i=a.len(); j=b.len())]
fn edit_distance(i: usize, j: usize) -> usize {
    if i == 0 {
        return j;
    }
    if j == 0 {
        return i;
    }
    let mut ans = std::cmp::min(edit_distance(i - 1, j), edit_distance(i, j - 1)) + 1;
    if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] {
        ans = std::cmp::min(ans, edit_distance(i - 1, j - 1));
    } else {
        ans = std::cmp::min(ans, edit_distance(i - 1, j - 1) + 1);
    }
    ans
}
