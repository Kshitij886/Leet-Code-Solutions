struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let vec1 = vec!["I", "V", "X", "L", "C", "D", "M"];
        let vec: Vec<u32> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        for i in 0..vec.len() {
            let place = 10u32.pow(i as u32);
        }
        return String::from("hello");
    }
}

fn main() {
    println!("{}", Solution::int_to_roman(23));
}

// I	1
// V	5
// X	10
// L	50
// C	100
// D	500
// M	1000
