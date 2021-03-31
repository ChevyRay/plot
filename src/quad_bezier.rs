use crate::line;
use math::Int2;

fn quad_bezier_seg<P: Into<Int2>, F: FnMut(Int2)>(p0: P, p1: P, p2: P, mut plot: F) {
    fn lp(p: Int2) -> (i64, i64) {
        (p.x as i64, p.y as i64)
    }
    fn pt(x: i64, y: i64) -> (i32, i32) {
        (x as i32, y as i32)
    }
    let (mut x0, mut y0) = lp(p0.into());
    let (x1, mut y1) = lp(p1.into());
    let (mut x2, mut y2) = lp(p2.into());
    let (mut sx, mut sy) = (x2 - x1, y2 - y1);
    let (mut xx, mut yy) = (x0 - x1, y0 - y1);
    let mut cur = xx * sy - yy * sx;
    assert!(xx * sx <= 0 && yy * sy <= 0);
    if sx * sx + sy * sy > xx * xx + yy * yy {
        x2 = x0;
        x0 = sx + x1;
        y2 = y0;
        y0 = sy + y1;
        cur = -cur;
    }
    if cur != 0 {
        xx += sx;
        yy += sy;
        sx = if x0 < x2 { 1 } else { -1 };
        sy = if y0 < y2 { 1 } else { -1 };
        xx *= sx;
        yy *= sy;
        let mut xy = 2 * xx * yy;
        xx *= xx;
        yy *= yy;
        if cur * sx * sy < 0 {
            xx = -xx;
            yy = -yy;
            xy = -xy;
            cur = -cur;
        }
        let mut dx = 4 * sy * cur * (x1 - x0) + xx - xy;
        let mut dy = 4 * sx * cur * (y0 - y1) + yy - xy;
        xx += xx;
        yy += yy;
        let mut err = dx + dy + xy;
        loop {
            plot(pt(x0, y0).into());
            if x0 == x2 && y0 == y0 {
                return;
            }
            y1 = if 2 * err < dx { 1 } else { 0 };
            if 2 * err > dy {
                x0 += sx;
                dx -= xy;
                dy += yy;
                err += dy;
            }
            if y1 > 0 {
                y0 += sy;
                dy -= xy;
                dx += xx;
                err += dx;
            }
            if dy >= dx {
                break;
            }
        }
    }
    line(pt(x0, y0), pt(x2, y2), plot);
}

pub fn quad_bezier<P: Into<Int2>, F: FnMut(Int2)>(p0: P, p1: P, p2: P, mut plot: F) {
    fn lp(p: Int2) -> (i64, i64) {
        (p.x as i64, p.y as i64)
    }
    fn pt(x: i64, y: i64) -> (i32, i32) {
        (x as i32, y as i32)
    }
    fn f(x: i64) -> f64 {
        x as f64
    }
    fn fabs(x: f64) -> i64 {
        x.abs() as i64
    }
    fn floor(x: f64) -> i64 {
        x.floor() as i64
    }
    let (mut x0, mut y0) = lp(p0.into());
    let (mut x1, mut y1) = lp(p1.into());
    let (mut x2, mut y2) = lp(p2.into());
    let x = x0 - x1;
    let y = y0 - y1;
    let mut t = f(x0 - 2 * x1 + x2);
    if x * (x2 - x1) > 0 {
        if y * (y2 - y1) > 0 {
            if fabs(f(y0 - 2 * y1 + y2) / t * f(x)) > y.abs() {
                x0 = x2;
                x2 = x + x1;
                y0 = y2;
                y2 = y + y1;
            }
        }
        t = f(x0 - x1) / t;
        let r = (1.0 - t) * ((1.0 - t) * f(y0) + 2.0 * t * f(y1)) + t * t * f(y2);
        t = f(x0 * x2 - x1 * x1) * t / f(x0 - x1);
        let x = floor(t + 0.5);
        let y = floor(r + 0.5);
        let r = f(y1 - y0) * (t - f(x0)) / f(x1 - x0) + f(y0);
        quad_bezier_seg(pt(x0, y0), pt(x, floor(r + 0.5)), pt(x, y), &mut plot);
        let r = f(y1 - y2) * (t - f(x2)) / f(x1 - x2) + f(y2);
        x0 = x;
        x1 = x;
        y0 = y;
        y1 = floor(r + 0.5);
    }
    if (y0 - y1) * (y2 - y1) > 0 {
        t = f(y0 - 2 * y1 + y2);
        t = f(y0 - y1) / t;
        let r = (1.0 - t) * ((1.0 - t) * f(x0) + 2.0 * t * f(x1)) + t * t * f(x2);
        t = f(y0 * y2 - y1 * y1) * t / f(y0 - y1);
        let x = floor(r + 0.5);
        let y = floor(t + 0.5);
        let r = f(x1 - x0) * (t - f(y0)) / f(y1 - y0) + f(x0);
        quad_bezier_seg(pt(x0, y0), pt(floor(r + 0.5), y), pt(x, y), &mut plot);
        let r = f(x1 - x2) * (t - f(y2)) / f(y1 - y2) + f(x2);
        x0 = x;
        x1 = floor(r + 0.5);
        y0 = y;
        y1 = y;
    }
    quad_bezier_seg(pt(x0, y0), pt(x1, y1), pt(x2, y2), plot);
}
