struct Solution;

impl Solution {
    fn is_match(s: String, p: String) -> bool {
        let mut new_str = String::from("");
        let mut prev_str: char = ' ';
        if s.len() != p.len() && (!p.contains("*") || !p.contains(".")) {
            return false;
        }
        for c in s.chars() {
            for b in p.chars() {
                if b == '.' {
                    new_str.push(c);
                } else if b == '*' {
                    new_str.push(prev_str);
                } else if b == c {
                    new_str.push(c);
                }
                prev_str = c;
            }
        }
        if new_str == s {
            return true;
        }
        return false;
    }
}

fn main() {
    let s = String::from("aa");
    let p = String::from("a*");
    println!("{}", Solution::is_match(s, p));
}
