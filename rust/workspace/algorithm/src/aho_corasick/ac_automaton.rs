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
}

impl ACAutomaton {
    pub fn new() -> Self {
        Self {
            g: [[INVALID_STATE; CHARSET_LEN]; TOTAL_STATES],
            out: [0; TOTAL_STATES],
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
}
