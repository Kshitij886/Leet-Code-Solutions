struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        if n > 9 || n < 1 || {
            return "".to_string();
        }
        return String::from("hello");
    }
}

fn main() {
    let n = 3;
    let k = 3;
    println!("Solution is {}", Solution::get_permutation(n, k));
}
