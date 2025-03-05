pub fn performance(calories: &[i32], k: usize, lower: i32, upper: i32) -> i32 {
    let mut score = 0;
    let n = calories.len();

    let mut sum = calories.iter().take(k).sum::<i32>();
    if sum < lower {
        score -= 1;
    } else if sum > upper {
        score += 1;
    }

    for i in k..n {
        sum += calories[i] - calories[i - k];
        if sum < lower {
            score -= 1;
        } else if sum > upper {
            score += 1;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, performance(&[1, 2, 3, 4, 5], 1, 3, 3));
        assert_eq!(1, performance(&[3, 2], 2, 0, 1));
        assert_eq!(0, performance(&[6, 5, 0, 0], 2, 1, 5));
    }
}
