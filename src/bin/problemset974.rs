struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut prefix_sum = 0;
        let mut remainder_map = HashMap::new();

        remainder_map.insert(0, 1);

        for num in nums {
            prefix_sum += num;

            let mut remainder = prefix_sum % k;
            if remainder < 0 {
                remainder += k;
            }

            if let Some(&freq) = remainder_map.get(&remainder) {
                count += freq;
            }

            *remainder_map.entry(remainder).or_insert(0) += 1;
        }

        count
    }
}

fn main() {
    let ans = vec![-2];
    let k = 9;
    println!("{}", Solution::subarrays_div_by_k(ans, k));
}
