struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        for i in 0..arr.len() {
            for j in 0..arr.len() {
                if i != j && (arr[i] == 2 * arr[j] || arr[j] == 2 * arr[i]) {
                    return true;
                }
            }
        }
        return false;
    }
}

fn main() {
    let arr = vec![-20, 8, -6, -14, 0, -19, 14, 4];
    println!("{}", Solution::check_if_exist(arr))
}
