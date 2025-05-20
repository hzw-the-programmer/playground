pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();
    let mut i = 0;
    while i < n {
        let mut remain = 0;
        let mut cnt = 0;
        while cnt < n {
            let j = (i + cnt) % n;
            remain += gas[j] - cost[j];
            if remain < 0 {
                break;
            }
            cnt += 1;
        }
        if cnt == n {
            return i as i32;
        }
        i += cnt + 1;
    }

    -1
}

// pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//     let n = gas.len();
//     let mut i = 0;
//     while i < n {
//         let mut remain = 0;
//         let mut j = i;
//         while j < i + n {
//             remain += gas[j % n] - cost[j % n];
//             if remain < 0 {
//                 break;
//             }
//             j += 1;
//         }
//         if j == i + n {
//             return i as i32;
//         }
//         i = j + 1;
//     }

//     -1
// }
