struct Solution;

impl Solution {
    pub fn roman_to_int(mut s: String) -> i32 {
        let mut num: i32 = 0;
        let mapping = vec![
            (1000, "M"),
            // (900, "CM"),
            (500, "D"),
            // (400, "CD"),
            (100, "C"),
            // (90, "XC"),
            (50, "L"),
            // (40, "XL"),
            (10, "X"),
            // (9, "IX"),
            (5, "V"),
            // (4, "IV"),
            (1, "I"),
        ];
        s = s.replace("CM", "DCCCC");
        s = s.replace("CD", "CCCC");
        s = s.replace("XC", "LXXXX");
        s = s.replace("XL", "XXXX");
        s = s.replace("IX", "VIIII");
        s = s.replace("IV", "IIII");
        for (value, roman) in mapping {
            for c in s.chars() {
                if c.to_string() == roman {
                    num += value;
                }
            }
        }
        return num;
    }
}

fn main() {
    println!("{}", Solution::roman_to_int("III".to_string()))
}
