/*

struct DP<T: std::fmt::Debug + Eq + Hash + Copy, U: Copy> {
    memo: HashMap<T, Option<U>>,
    solve: dyn Fn(T) -> U,
}

impl<T: std::fmt::Debug + Eq + Hash + Copy, U: Copy> DP<T, U> {
    fn eval(&mut self, args: T) -> U {
        match self.get(args) {
            DpOption::Some(result) => return result,
            DpOption::Repeat => panic!("Value {:?} from accessed before solution was found", args),
            DpOption::None => {
                self.set(args, None);
                let ans = (self.solve)(args);
                self.set(args, Some(ans));
                return ans;
            }
        }
    }
    fn get(&self, n: T) -> DpOption<U> {
        match self.memo.get(&n).copied() {
            Some(Some(t)) => DpOption::Some(t),
            Some(None) => DpOption::Repeat,
            None => DpOption::None,
        }
    }
    fn set(&mut self, k: T, v: Option<U>) {
        self.memo.insert(k, v);
    }

    /*fn new(f: &dyn Fn(T) -> U) -> Self {
        Self {
            memo: HashMap::default(),
            solve: f,
        }
    }*/
}*/
