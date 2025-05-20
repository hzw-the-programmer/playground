pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut inc = 0;
    let mut dec = 0;
    let mut total = 0;

    let check = |mut inc, mut dec| {
        let mut delta = 0;
        if dec == 0 {
            delta += (inc + 1) * inc / 2;
        } else {
            if inc == 0 {
                delta += (dec + 1) * dec / 2;
            } else {
                if dec > inc {
                    inc -= 1;
                } else {
                    dec -= 1;
                }
                delta += (dec + 1) * dec / 2;
                delta += (inc + 1) * inc / 2;
            }
        }
        delta
    };

    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            if dec > 0 {
                total += check(inc, dec);
                dec = 0;
                inc = 0;
            }
            inc += 1;
        } else if ratings[i] < ratings[i - 1] {
            dec += 1;
        } else {
            if dec == 0 {
                total += check(inc, dec);
                inc = 0;
            } else if dec == 1 {
                if inc != 0 {
                    dec = 0;
                    total += check(inc, dec);
                    inc = 0;
                } else {
                    total += check(inc, dec);
                    dec = 0;
                }
            }
        }
    }

    total += check(inc, dec);

    n as i32 + total
}
