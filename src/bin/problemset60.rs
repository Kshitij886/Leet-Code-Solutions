struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut num = 0;
        let mut arr: Vec<i32> = Vec::new();
        for i in 1..n + 1 {
            arr.push(i);
        }

        // while k > 0 {
        //     num = 0;
        //     for i in 1..n + 1 {
        //         num = num * 10 + i;
        //     }
        //     k -= 1;
        // }
        return num.to_string();
    }
}

fn main() {
    let n = 3;
    let mut k = 3;
    println!("Solution is {}", Solution::get_permutation(n, k));
}
