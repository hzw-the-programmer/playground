use std::ops::BitAnd;

#[derive(PartialEq, Debug)]
struct WrappedVecBool(Vec<bool>);

impl BitAnd for WrappedVecBool {
    type Output = Self;

    fn bitand(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs.iter().zip(rhs.iter()).map(|(f, s)| *f & *s).collect())
    }
}

fn main() {
    let v1 = WrappedVecBool(vec![true, true, false, false]);
    let v2 = WrappedVecBool(vec![true, false, true, false]);
    let v3 = WrappedVecBool(vec![true, false, false, false]);
    assert_eq!(v1 & v2, v3);
}
