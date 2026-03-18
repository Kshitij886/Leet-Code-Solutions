#![allow(unused)]
struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;
        let mut num = n;

        let mut seen = HashSet::new();

        while num != 1 && !seen.contains(&num) {
            seen.insert(num);

            let mut sum = 0;
            let mut temp = num;

            while temp > 0 {
                let digit = temp % 10;
                sum += digit * digit;
                temp /= 10;
            }

            num = sum;
        }

        return num == 1;
    }
}
fn main() {
    let num: i32 = 19;
    println!("Ans is {}", Solution::is_happy(num));
}
