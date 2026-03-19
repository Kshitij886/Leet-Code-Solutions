struct Solution;

impl Solution {
    fn is_match(s: String, p: String) -> bool {
        return false;
    }
}

fn main() {
    let s = String::from("aaaaaaaaaaaaab");
    let p = String::from("a*a*a*a*a*a*a*a*a*a*b");
    println!("{}", Solution::is_match(s, p));
}
