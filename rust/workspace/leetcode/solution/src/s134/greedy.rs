pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut res = -1;
    let mut acc = 0;
    for i in 0..gas.len() {
        let remain = gas[i] - cost[i];
        total += remain;
        if remain >= 0 && res == -1 {
            res = i as i32;
            acc = remain;
        } else {
            acc += remain;
            if acc < 0 {
                res = -1;
            }
        }
    }

    if total < 0 {
        res = -1;
    }
    res
}
