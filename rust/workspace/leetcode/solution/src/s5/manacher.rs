// https://www.geeksforgeeks.org/manachers-algorithm-linear-time-longest-palindromic-substring-part-1/
pub fn longest_palindrome(s: String) -> String {
    //插入特殊标记
    let mut t = Vec::with_capacity((s.len() << 1) + 3);
    t.push('^');
    s.chars().for_each(|x| {
        t.push('#');
        t.push(x);
    });
    t.push('#');
    t.push('$');

    // // 定义一个奇回文串的回文半径 = (长度+1)/2，即保留回文中心，去掉一侧后的剩余字符串的长度
    let mut halflen = vec![0; t.len() - 2];
    halflen[1] = 1;
    // boxR 表示当前右边界下标最大的回文子串的右边界下标+1
    // boxM 为该回文子串的中心位置，二者的关系为 r=mid+halfLen[mid]
    let mut box_m = 0;
    let mut box_r = 0;
    let (mut c, mut r) = (0, 0);
    for i in 2..halflen.len() {
        // 回文半径
        let mut hl = 1;
        if i < box_r {
            hl = halflen[box_m * 2 - i].min(box_r - i);
        }
        while t[i - hl] == t[i + hl] {
            hl += 1;
            box_m = i;
            box_r = i + hl;
        }
        halflen[i] = hl;
        if hl > r {
            c = i;
            r = hl;
        }
    }
    // s.chars().skip((c-r+1)/2).take(r-1).collect()
    s[(c - r + 1) / 2..(c + r - 1) / 2].to_string()
}
