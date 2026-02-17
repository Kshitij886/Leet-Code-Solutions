struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let vec: Vec<u32> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        if
        for i in 0..vec.len() {
            let place = 10u32.pow(i as u32);
        }
        return String::from("hello");
    }
}

fn main() {
    println!("{}", Solution::int_to_roman(23));
}
