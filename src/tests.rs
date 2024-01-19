use crate::Matrix;

#[test]
fn general() {
    let mut a = Matrix::empty(3, 2);
    let mut b = Matrix::empty(2, 3);
    let mut c = Matrix::empty(2, 2);

    a.set(&[1.,2.,3.,4.,5.,6.]);
    b.set(&[7.,8.,9.,10.,11.,12.]);

    a.multiply(&b, &mut c);

    c.log();
}