struct Solution;

// time complexity > 5ms
// impl Solution {
//     pub fn roman_to_int(mut s: String) -> i32 {
//         let mut num: i32 = 0;
//         let mapping = vec![
//             (1000, "M"),
//             // (900, "CM"),
//             (500, "D"),
//             // (400, "CD"),
//             (100, "C"),
//             // (90, "XC"),
//             (50, "L"),
//             // (40, "XL"),
//             (10, "X"),
//             // (9, "IX"),
//             (5, "V"),
//             // (4, "IV"),
//             (1, "I"),
//         ];
//         s = s.replace("CM", "DCCCC");
//         s = s.replace("CD", "CCCC");
//         s = s.replace("XC", "LXXXX");
//         s = s.replace("XL", "XXXX");
//         s = s.replace("IX", "VIIII");
//         s = s.replace("IV", "IIII");
//         for (value, roman) in mapping {
//             for c in s.chars() {
//                 if c.to_string() == roman {
//                     num += value;
//                 }
//             }
//         }
//         return num;
//     }
// }

// time complexity = 0ms
impl Solution {
    fn roman_to_int(s: String) -> i32 {
        let mut num: i32 = 0;
        let mut prev: i32 = 0;
        for c in s.chars().rev() {
            match c {
                'I' => {
                    if prev > 1 {
                        num -= 1;
                    } else {
                        num += 1;
                    }
                    prev = 1;
                }
                'V' => {
                    if prev > 5 {
                        num -= 5;
                    } else {
                        num += 5;
                    }
                    prev = 5;
                }
                'X' => {
                    if prev > 10 {
                        num -= 10;
                    } else {
                        num += 10;
                    }
                    prev = 10;
                }
                'L' => {
                    if prev > 50 {
                        num -= 50;
                    } else {
                        num += 50;
                    }
                    prev = 50;
                }
                'C' => {
                    if prev > 100 {
                        num -= 100;
                    } else {
                        num += 100;
                    }
                    prev = 100;
                }
                'D' => {
                    if prev > 500 {
                        num -= 500;
                    } else {
                        num += 500;
                    }
                    prev = 500;
                }
                'M' => {
                    if prev > 1000 {
                        num -= 1000;
                    } else {
                        num += 1000;
                    }
                    prev = 1000;
                }
                _ => num += 0,
            }
        }
        return num;
    }
}
fn main() {
    println!("{}", Solution::roman_to_int("LVIII".to_string()))
}
