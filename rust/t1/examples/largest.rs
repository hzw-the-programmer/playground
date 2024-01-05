fn main() {
    let nums = vec![1, 2, 3];
    // let n: i32 = v;
    let largest = largest(&nums);
    // let n: i32 = v;
    println!("&nums[0]={:?}", &nums[0] as *const i32);
    println!("&nums[2]={:?}", &nums[2] as *const i32);
    println!("largest ={:?}", largest as *const i32);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
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

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
