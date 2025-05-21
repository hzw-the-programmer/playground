pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut pre = 1;
    let mut ret = 1;
    let mut dec = 1;
    let mut inc = 1;
    for i in 1..n {
        if ratings[i] >= ratings[i - 1] {
            pre = if ratings[i] > ratings[i - 1] {
                pre + 1
            } else {
                1
            };
            ret += pre;
            inc = pre;
            dec = 0;
        } else {
            pre = 1;

            dec += 1;
            if dec == inc {
                dec += 1;
            }

            ret += dec;
        }
    }

    ret
}
