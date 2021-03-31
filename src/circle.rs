use math::{int2, Int2};

pub fn circle<P: Into<Int2>, F: FnMut(Int2)>(m: P, mut r: i32, mut plot: F) {
    let m = m.into();
    let mut p = int2(-r, 0);
    let mut err = 2 - 2 * r;
    loop {
        plot(int2(m.x - p.x, m.y + p.y));
        plot(int2(m.x - p.y, m.y - p.x));
        plot(int2(m.x + p.x, m.y - p.y));
        plot(int2(m.x + p.y, m.y + p.x));
        r = err;
        if r > p.x {
            p.x += 1;
            err += p.x * 2 + 1;
        }
        if r <= p.y {
            p.y += 1;
            err += p.y * 2 + 1;
        }
        if p.x >= 0 {
            break;
        }
    }
}
