struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut num: i32 = 0;
        let mapping = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        for c in s.chars() {}
        return num;
    }
}

fn main() {
    println!("{}", Solution::roman_to_int("III".to_string()))
}
