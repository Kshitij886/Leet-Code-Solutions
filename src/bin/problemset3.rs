#![allow(unused)]
use std::collections::HashSet;

struct Solution;
impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut set = HashSet::<char>::new();
        let mut left = 0;
        let mut max = 0;
        for right in 0..chars.len() {
            while set.contains(&chars[right]) {
                set.remove(&chars[left]);
                left += 1;
            }
            set.insert(chars[right]);
            max = max.max((right - left + 1) as i32)
        }

        max
    }
}

fn main() {
    let sub_string = "bbbbb".to_string();
    println!("ans: {}", Solution::length_of_longest_substring(sub_string));
}
