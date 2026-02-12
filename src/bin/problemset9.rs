struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut num = x as i64;
        if num < 0 {
            return false;
        }
        let mut rev = 0;
        while num != 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }
        if rev > i32::MAX.into() || rev < i32::MIN.into() {
            return false;
        }
        if rev as i32 == x { return true } else { false }
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(10))
}
