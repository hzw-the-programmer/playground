pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();
    for i in 0..n {
        let mut total = 0;
        let mut j = i;
        while j < i + n {
            total += gas[j % n];
            if total >= cost[j % n] {
                total -= cost[j % n];
            } else {
                break;
            }
            j += 1;
        }
        if j == i + n {
            return i as _;
        }
    }
    -1
}
