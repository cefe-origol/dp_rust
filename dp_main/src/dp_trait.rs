pub trait DP<T: Copy + std::fmt::Debug, U: Clone> {
    fn eval(&mut self, args: T) -> U {
        match self.get(&args) {
            Some(Some(result)) => return result.clone(),
            Some(None) => panic!("Value {:?} from accessed before solution was found", args),
            None => {
                self.insert(args, None);
                let ans = self.solve(args);
                self.insert(args, Some(ans.clone()));
                return ans;
            }
        }
    }
    fn get(&self, args: &T) -> Option<&Option<U>>;
    fn insert(&mut self, k: T, v: Option<U>);
    fn solve(&mut self, args: T) -> U;
}
