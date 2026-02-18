struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut vec: Vec<i32> = Vec::new();
        for n in nums {
            if n % k == 0 {
                vec.push(n);
            }
        }

        return vec.len() as i32;
    }
}
fn main() {
    let ans = vec![4, 5, 0, -2, -3, 1];
    let k = 5;
    println!("{}", Solution::subarrays_div_by_k(ans, k));
}
