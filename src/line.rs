use math::{int2, Int2};

pub fn line<P: Into<Int2>, F: FnMut(Int2)>(p0: P, p1: P, mut plot: F) {
    let mut p0 = p0.into();
    let p1 = p1.into();
    let s = (p1 - p0).sign();
    let d = (p1 - p0).abs() * int2(1, -1);
    let mut err = d.x + d.y;
    plot(p0);
    while p0 != p1 {
        let e2 = err * 2;
        if e2 >= d.y {
            err += d.y;
            p0.x += s.x;
        }
        if e2 <= d.x {
            err += d.x;
            p0.y += s.y;
        }
        plot(p0);
    }
}

#[test]
fn test_line() {
    line((1, 1), (-1, -1), |p| {
        println!("{}", p);
    })
}
