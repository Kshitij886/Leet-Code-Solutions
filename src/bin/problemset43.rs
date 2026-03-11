struct Solution;
impl Solution {
    fn multiply(num1: String, num2: String) -> String {
        let nums = vec![('1',1),('2',2), ('3',3),('4',4),('5',5),('5',6),('7',7),('8',8),('9',9),('0',0)];
        let mut num3: i32 = 0;
        let mut num4: i32 = 0;
        for (str, num) in &nums {
            for n in num1.chars() {
                if &n == str {
                    num3 = num3 *10 + num;
                }
            }
        }
        for (str, num) in &nums {
            for n in num2.chars() {
                if &n == str {
                    num4 = num4 *10 + num;
                }
            }
        }
        return (num3 * num4).to_string();
    }
}

fn main() {
    println!("{}", Solution::multiply(String::from("98"), String::from("9")));
}
