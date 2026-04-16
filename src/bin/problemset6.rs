#![allow(unused)]
struct Solution;

impl Solution {
    fn convert(s: String, n: i32) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        if len == 1 || len == 2 {
            return s;
        }
        let diff = 2 * (n - 1);
        let mut sub_string = Vec::<char>::new();
        for i in 0..chars.len() {
            let mut indx = diff * i as i32;
            if indx > len as i32 {
                indx = indx % len as i32;
            }
            sub_string.push(chars[indx as usize]);
        }
        return sub_string.into_iter().collect();
    }
}
fn main() {
    let s: String = String::from("PAYPALISHIRING");
    let n = 3;
    println!("ans: {}", Solution::convert(s, n));
}
