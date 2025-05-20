pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut inc = 0;
    let mut dec = 0;
    let mut total = 0;
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            if dec > 0 {
                if inc == 0 {
                    total += (dec + 1) * dec / 2;
                } else {
                    if dec > inc {
                        inc -= 1;
                    } else {
                        dec -= 1;
                    }
                    total += (dec + 1) * dec / 2;
                    total += (inc + 1) * inc / 2;
                }
                dec = 0;
                inc = 0;
            }
            inc += 1;
        } else if ratings[i] < ratings[i - 1] {
            dec += 1;
        }
    }

    if dec > 0 {
        if inc == 0 {
            total += (dec + 1) * dec / 2;
        } else {
            if dec > inc {
                inc -= 1;
            } else {
                dec -= 1;
            }
            total += (dec + 1) * dec / 2;
            total += (inc + 1) * inc / 2;
        }
    } else {
        total += (inc + 1) * inc / 2;
    }

    (n + total) as _
}
