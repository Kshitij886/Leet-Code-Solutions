#![allow(unused)]
struct Solution;

impl Solution {
    fn my_atoi(s: String) -> i32 {
        let mut num: i64 = 0;
        let mut sign: i64 = 1;

        let mut chars = s.trim().chars().peekable();

        if let Some(&c) = chars.peek() {
            if c == '-' {
                sign = -1;
                chars.next();
            } else if c == '+' {
                chars.next();
            }
        }

        while let Some(c) = chars.next() {
            if let Some(digit) = c.to_digit(10) {
                let digit = digit as i64;

                if num > (i32::MAX as i64 - digit) / 10 {
                    return if sign == 1 { i32::MAX } else { i32::MIN };
                }

                num = num * 10 + digit;
            } else {
                break;
            }
        }

        (num * sign) as i32
    }
}

fn main() {
    let s = "2147483648".to_string();
    println!("Ans {}", Solution::my_atoi(s));
}
