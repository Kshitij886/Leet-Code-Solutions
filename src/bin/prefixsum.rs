


fn main() {
    println!("hello");
    prefix_sum();
}


fn prefix_sum() -> Vec<i32>{
    let  arr = vec![1,2,3,4,5];
    let mut prefix_sum_arr: Vec<i32> = Vec::new();
    prefix_sum_arr.push(arr[0]);
    for i in 1..arr.len() {
        prefix_sum_arr.push(prefix_sum_arr[i-1] + arr[i]);
    }
    return prefix_sum_arr;
}