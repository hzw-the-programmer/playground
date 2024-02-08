use ndarray::{arr2, aview0, aview1, Axis};

fn main() {
    let a = arr2(&[[1., 2., 3.], [4., 5., 6.]]);
    assert!(
        a.sum_axis(Axis(0)) == aview1(&[5., 7., 9.])
            && a.sum_axis(Axis(1)) == aview1(&[6., 15.])
            && a.sum_axis(Axis(0)).sum_axis(Axis(0)) == aview0(&21.)
            && a.sum_axis(Axis(0)).sum_axis(Axis(0)) == aview0(&a.sum())
    );
}
