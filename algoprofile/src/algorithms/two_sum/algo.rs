pub mod two_sum {
    use algoprofile_macros::{profile, AlgoInput};
    use std::collections::HashMap;

    #[derive(AlgoInput, Clone)]
    pub struct RegularArgs {
        #[n_size(0, 100)]
        #[element_range(-1000,1000)]
        pub nums: Vec<i64>,

        #[element_range(0, 100)]
        pub target: i64,
    }

    /*
     uses hashmap
    */
    #[profile]
    pub fn algo(args: RegularArgs) -> Vec<i64> {
        let mut map: HashMap<i64, usize> = HashMap::new();

        for (i, num) in args.nums.iter().enumerate() {
            if let Some(&j) = map.get(&(args.target - num)) {
                return vec![j as i64, i as i64];
            }
            map.insert(*num, i);
        }
        vec![]
    }
}
