pub fn test() {}

trait Buf {
    fn remaining(&self) -> usize;
    fn chunk(&self) -> &[u8];
    fn advance(&mut self, cnt: usize);
}

impl Buf for &[u8] {
    fn remaining(&self) -> usize {
        // let i: i32 = self;
        self.len()
    }

    fn chunk(&self) -> &[u8] {
        // let i: i32 = self;
        self
    }

    fn advance(&mut self, cnt: usize) {
        // let i: i32 = self;
        *self = &self[cnt..];
    }
}
