use math::{int2, Int2, IntRect};

pub fn ellipsis<R: Into<IntRect>, F: FnMut(Int2)>(rect: R, mut plot: F) {
    let rect = rect.into();
    let mut p0 = rect.min();
    let mut p1 = rect.max();
    let a = (p1.x - p0.x).abs();
    let b = (p1.y - p0.y).abs();
    let b1 = b & 1;
    let mut d = int2(4 * (1 - a) * b * b, 4 * (b1 + 1) * a * a);
    let mut err = d.x + d.y + b1 * a * a;
    if p0.x > p1.x {
        p0.x = p1.x;
        p1.x += a;
    }
    if p0.y > p1.y {
        p0.y = p1.y;
    }
    p0.y += (b + 1) / 2;
    p1.y = p0.y - b1;
    let a = a * a * 8;
    let b1 = 8 * b * b;
    loop {
        plot(int2(p1.x, p0.y));
        plot(int2(p0.x, p0.y));
        plot(int2(p0.x, p1.y));
        plot(int2(p1.x, p1.y));
        let e2 = 2 * err;
        if e2 >= d.x {
            p0.x += 1;
            p1.x -= 1;
            d.x += b1;
            err += d.x;
        }
        if e2 <= d.y {
            p0.y += 1;
            p1.y -= 1;
            d.y += a;
            err += d.y;
        }
        if p0.x > p1.x {
            break;
        }
    }
    while p0.y - p1.y < b {
        plot(int2(p0.x - 1, p0.y));
        plot(int2(p1.x + 1, p0.y));
        p0.y += 1;
        plot(int2(p0.x - 1, p1.y));
        plot(int2(p1.x + 1, p1.y));
        p1.y -= 1;
    }
}
