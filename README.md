# dp_rust
Rust procedural macro to apply memoization to pure functions.

This crate is still in beta. Report issues if possible.

## Usage
_For an explanationon memoization, go to explanation_

Implementing memoization is simple, but takes some time and adds
boilerplate.

### `dp` attribute
Just take the original fn and add `#[dp]` at the beggining.
```rs
fn main(){
  assert_eq!(102334155, fib(40));
}

#[dp]
fn fib(n: i32) -> i32{
  if n < 2{
    return n;
  }
  (fib(n-1) + fib(n-2)) % 1_000_000_007
}
```
The function must not be inside of `impl`.

Using the dp macro over a non pure function is Undefinied Behaviour.
Note that pure does not mean `const`, eg. you may use for loops despite
them being forbidden in `const`.

All arguments must implement `Copy`. In some cases non-`Copy` arguments
can be used emulated with extra arguments.

### Extra arguments
In case it is needed, extra inmutable arguments can be included with
the `#[dp_extra]` attribute.

```rs
fn main(){
  assert_eq!(
    knapsack(vec![3, 4, 5, 6, 10], vec![2, 3, 4, 5, 9], 5, 10),
    13
  );
}

#[dp]
#[dp_extra(values: Vec<i32>, weights: Vec<i32>)]
fn knapsack(values: Vec<i32>, weight: Vec<i32>){
  if n == 0 {
    return 0;
  }
  let mut ans = knapsack(n - 1, k);
  if k >= weights[n - 1] {
    ans = std::cmp::max(ans, knapsack(n - 1, k - weights[n - 1]) + values[n - 1]);
  }
  return ans;

}
```

The order is first all the extra arguments, and then all function
arguments, given in order of appearance.

### Default arguments
The `#[dp_default]` attribute is still on the works. It is intended to
remove auxilliary arguments that should default in the final function.

```rs
use std::cmp::min;

fn main(){
  assert_eq!(edit_distance("movie", "love"), 2);
}

#[dp]
#[dp_extra(a: str, b: str)]
#[dp_default(i=a.len(); j=b.len())]
fn edit_distance(i: usize, j: usize){
  if i == 0{
    return j;
  }
  if j == 0{
    return i
  }
  let mut ans = min(edit_distance(i-1, j), edit_distance(i, j-1))+1;
  if a.as_bytes()[i] == b.bytes()[j]{
    ans = min(ans, edit_distance(i-1, j-1));
  }
  return ans;
}
```

## Explanation
Memoization is a technique that allows pure functions overlapping
subproblems to be optimized by saving the answer and never recalculating
them.

As an example, take the fibonacci function, given by the recurrence:

$f(n) = f(n-1) + f(n-2)$
where $f(0) = 0$ and $f(1) = 1$

As the function is only ever allowed to return end the recursive calls
with an answer of at most 1, it can be shown that the number of
recursive calls is at least the n-th fibonacci number, which grows
exponentially. But given the first n fibonacci numbers, calculating the
n+1-th takes just two memory lookups and a sum, now making the problem
linear.

Note that if a constant function loops doesn't work, eg:
$f(n) = f(n+1) + f(n-1)$

Some problems, despite using constant non-looping functions the number
of overlapping subproblems is close to zero, like in backtracking. Here
memoization doesn't help and will add overhead, making it even worse.
