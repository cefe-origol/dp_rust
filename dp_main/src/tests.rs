#[cfg(test)]
mod tests {
    use super::dp_trait::DP;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Fib {
        memo: HashMap<i32, Option<i32>>,
    }

    impl DP<i32, i32> for Fib {
        fn get(&self, n: &i32) -> Option<&Option<i32>> {
            self.memo.get(n)
        }
        fn insert(&mut self, k: i32, v: Option<i32>) {
            self.memo.insert(k, v);
        }

        fn solve(&mut self, n: i32) -> i32 {
            let mut solve = |x| self.eval(x);
            if n == 0 || n == 1 {
                return 1;
            }
            solve(n - 1) + solve(n - 2)
        }
    }

    #[test]
    fn fib() {
        let mut f = Fib::default();
        for i in 10..=2 {
            assert_eq!(f.eval(i), f.eval(i - 1) + f.eval(i - 2));
        }
    }

    #[derive(Default, Debug)]
    struct FibString {
        memo: HashMap<i32, Option<String>>,
    }

    impl DP<i32, String> for FibString {
        fn get(&self, n: &i32) -> Option<&Option<String>> {
            self.memo.get(&n)
        }
        fn insert(&mut self, k: i32, v: Option<String>) {
            self.memo.insert(k, v);
        }

        fn solve(&mut self, n: i32) -> String {
            let mut solve = |x| self.eval(x);
            if n == 0 {
                return String::from("0");
            }
            if n == 1 {
                return String::from("1");
            }
            solve(n - 1) + &solve(n - 2)
        }
    }

    fn fib_string(n: i32) -> String {
        if n == 0 {
            return String::from("0");
        }
        if n == 1 {
            return String::from("1");
        }
        fib_string(n - 1) + &fib_string(n - 2)
    }

    #[test]
    fn fib_string_test() {
        let mut f = FibString::default();
        for i in 0..10 {
            assert_eq!(fib_string(i), f.eval(i));
        }
    }

    struct EditDistanceExp {
        a: Vec<u8>,
        b: Vec<u8>,
    }
    impl EditDistanceExp {
        fn solve(&self, i: usize, j: usize) -> usize {
            if i == 0 {
                return j;
            }
            if j == 0 {
                return i;
            }
            let ans = std::cmp::min(self.solve(i - 1, j) + 1, self.solve(i, j - 1) + 1);
            std::cmp::min(
                ans,
                self.solve(i - 1, j - 1) + (if self.a[i - 1] == self.b[j - 1] { 0 } else { 1 }),
            )
        }
    }

    struct EditDistanceDP {
        memo: HashMap<(usize, usize), Option<usize>>,
        a: Vec<u8>,
        b: Vec<u8>,
    }
    impl DP<(usize, usize), usize> for EditDistanceDP {
        fn solve(&mut self, args: (usize, usize)) -> usize {
            let (i, j) = args;
            let mut solve = |x, y| self.eval((x, y));
            if i == 0 {
                return j;
            }
            if j == 0 {
                return i;
            }
            let ans = std::cmp::min(solve(i - 1, j) + 1, solve(i, j - 1) + 1);
            std::cmp::min(
                ans,
                solve(i - 1, j - 1) + (if self.a[i - 1] == self.b[j - 1] { 0 } else { 1 }),
            )
        }
        fn get(&self, args: &(usize, usize)) -> Option<&Option<usize>> {
            self.memo.get(args)
        }
        fn insert(&mut self, k: (usize, usize), v: Option<usize>) {
            self.memo.insert(k, v);
        }
    }

    #[test]
    fn edit_distance() {
        let wordlist = ["aurora", "cinema", "parkour", "algorithm"];
        for i in 0..wordlist.len() {
            for j in i + 1..wordlist.len() {
                let mut t1 = EditDistanceDP {
                    memo: Default::default(),
                    a: wordlist[i].try_into().unwrap(),
                    b: wordlist[j].try_into().unwrap(),
                };
                let t2 = EditDistanceExp {
                    a: wordlist[i].try_into().unwrap(),
                    b: wordlist[j].try_into().unwrap(),
                };
                let (x, y) = (wordlist[i].len(), wordlist[j].len());
                assert_eq!(t1.solve((x, y)), t2.solve(x, y));
            }
        }
    }
}
