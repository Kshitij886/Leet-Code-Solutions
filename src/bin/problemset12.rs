struct Solution {}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        if num == 0 {
            return "".to_string();
        }
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
        let mut result = String::new();
        for (val, roman) in mapping {
            while num >= val {
                result.push_str(roman);
                num -= val;
            }
        }
        return result;
    }
}

fn main() {
    println!("{}", Solution::int_to_roman(3749));
}

// I	1
// V	5
// X	10
// L	50
// C	100
// D	500
// M	1000
