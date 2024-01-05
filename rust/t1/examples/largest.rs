fn main() {
    let nums = vec![1, 2, 3];
    // let n: i32 = v;
    let largest = largest(&nums);
    // let n: i32 = v;
    println!("&nums[0]={:?}", &nums[0] as *const i32);
    println!("&nums[2]={:?}", &nums[2] as *const i32);
    println!("largest ={:?}", largest as *const i32);
}

fn largest(nums: &[i32]) -> &i32 {
    let mut largest = &nums[0];
    for num in nums {
        if num > largest {
            largest = num;
        }
    }
    largest
}
