const TOTAL_STATES: usize = 500;
const INVALID_STATE: usize = TOTAL_STATES;
const CHARSET_LEN: usize = 26;
fn get_index(c: char) -> usize {
    assert!(c >= 'a' && c <= 'z');
    c as usize - 'a' as usize
}

pub struct ACAutomaton {
    g: [[usize; CHARSET_LEN]; TOTAL_STATES],
    out: [usize; TOTAL_STATES],
    f: [usize; TOTAL_STATES],
}

impl ACAutomaton {
    pub fn new() -> Self {
        Self {
            g: [[INVALID_STATE; CHARSET_LEN]; TOTAL_STATES],
            out: [0; TOTAL_STATES],
            f: [INVALID_STATE; TOTAL_STATES],
        }
    }

    pub fn build(&mut self, words: &[&str]) {
        let mut state = 1;
        for (i, word) in words.iter().enumerate() {
            let mut current = 0;
            for c in word.chars() {
                let i = get_index(c);
                if self.g[current][i] == INVALID_STATE {
                    self.g[current][i] = state;
                    state += 1;
                }
                current = self.g[current][i];
            }
            self.out[current] |= 1 << i;
        }

        let mut queue = std::collections::VecDeque::new();
        for i in 0..CHARSET_LEN {
            if self.g[0][i] == INVALID_STATE {
                self.g[0][i] = 0;
            } else {
                self.f[self.g[0][i]] = 0;
                queue.push_back(self.g[0][i]);
            }
        }

        while let Some(state) = queue.pop_front() {
            for i in 0..CHARSET_LEN {
                let next = self.g[state][i];
                if next != INVALID_STATE {
                    let mut failure = self.f[state];
                    while self.g[failure][i] == INVALID_STATE {
                        failure = self.f[failure];
                    }
                    failure = self.g[failure][i];

                    self.f[next] = failure;
                    self.out[next] |= self.out[failure];
                    queue.push_back(next);
                }
            }
        }

        #[cfg(test)]
        self.dump(state);
    }

    fn next_state(&self, mut current: usize, c: char) -> usize {
        let i = get_index(c);
        while self.g[current][i] == INVALID_STATE {
            current = self.f[current]
        }
        self.g[current][i]
    }

    pub fn search(&self, text: &str, words: &[&str]) -> Vec<(usize, usize, usize)> {
        let mut result = Vec::new();

        let mut current = 0;
        for (i, c) in text.chars().enumerate() {
            current = self.next_state(current, c);

            if self.out[current] == 0 {
                continue;
            }

            for (j, word) in words.iter().enumerate() {
                if self.out[current] & (1 << j) != 0 {
                    result.push((j, i - word.len() + 1, i));
                }
            }
        }

        result
    }

    pub fn starts_with(&self, text: &str) -> bool {
        let mut current = 0;
        for c in text.chars() {
            let i = get_index(c);
            if self.g[current][i] == INVALID_STATE {
                return false;
            }
            current = self.g[current][i];
        }
        true
    }

    #[cfg(test)]
    fn dump(&self, total_states: usize) {
        self.dump_g(total_states);
        println!("");
        self.dump_f(total_states);
    }

    #[cfg(test)]
    fn dump_g(&self, total_states: usize) {
        print!("   ");
        "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .for_each(|c| print!("{c}  "));
        println!("");
        for i in 0..total_states {
            print!("{i:02} ");
            for j in 0..CHARSET_LEN {
                match self.g[i][j] {
                    0 | INVALID_STATE => print!("   "),
                    s => print!("{s:02} "),
                }
            }
            println!("");
        }
    }

    #[cfg(test)]
    fn dump_f(&self, total_states: usize) {
        for i in 0..total_states {
            print!("{i:02} ");
            match self.f[i] {
                0 | INVALID_STATE => print!("   "),
                s => print!("{s:02} "),
            }
            println!("");
        }
    }
}
