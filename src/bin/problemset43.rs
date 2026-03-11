struct Solution;
impl Solution {
    fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let n = num1.len();
        let m = num2.len();
        let mut result = vec![0; n + m];

        let a: Vec<u8> = num1.bytes().collect();
        let b: Vec<u8> = num2.bytes().collect();

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                let mul = (a[i] - b'0') as i32 * (b[j] - b'0') as i32;
                let sum = mul + result[i + j + 1];

                result[i + j + 1] = sum % 10;
                result[i + j] += sum / 10;
            }
        }

        result
            .into_iter()
            .skip_while(|&x| x == 0)
            .map(|x| (x as u8 + b'0') as char)
            .collect()
    }
}

fn main() {
    println!(
        "{}",
        Solution::multiply(
            String::from("498828660196"),
            String::from("167141802233061013023557397451289113296441069")
        )
    );
}
