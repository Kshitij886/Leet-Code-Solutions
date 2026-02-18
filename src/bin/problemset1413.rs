struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut start_val = 1;
        let mut found = false;
        let mut ans = 1;
        while !found {
            let mut index = 0;
            for i in &nums {
                if (start_val + i) <= 0 {
                    start_val = start_val + 1;
                    ans += 1;
                    break;
                } else {
                    index += 1;
                    start_val = start_val + i;
                }
            }
            if &index == &nums.len() {
                found = true;
            } else {
                start_val = ans;
            }
        }
        return ans;
    }
}

fn main() {
    let vec: Vec<i32> = vec![1, -2, -3];
    println!("{}", Solution::min_start_value(vec))
}
