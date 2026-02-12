struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x as i64;
        let mut rev = 0;
        while num != 0 {
            rev = rev * 10 + num % 10;
            num = num / 10;
        }
        if rev > i32::MAX.into() || rev < i32::MIN.into() {
            return 0;
        }
        return rev as i32;
    }
}
fn main() {
    println!("{}", Solution::reverse(-2147483648));
}
