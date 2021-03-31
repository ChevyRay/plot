use crate::line;
use crate::quad_bezier_seg;
use math::Int2;

pub fn cubic_bezier_seg<P: Into<Int2>, F: FnMut(Int2)>(p0: P, p1: P, p2: P, p3: P, mut plot: F) {
    fn fp(p: Int2) -> (f64, f64) {
        (p.x as f64, p.y as f64)
    }
    fn pt(x: f64, y: f64) -> (i32, i32) {
        (x as i32, y as i32)
    }
    fn fabs(x: f64) -> f64 {
        x.abs()
    }
    fn floor(x: f64) -> f64 {
        x.floor()
    }
    let (mut x0, mut y0) = fp(p0.into());
    let (mut x1, mut y1) = fp(p1.into());
    let (mut x2, y2) = fp(p2.into());
    let (mut x3, mut y3) = fp(p3.into());
    let mut leg = 1;
    let mut sx = (x3 - x0).signum();
    let mut sy = (y3 - y0).signum();
    let xc = -fabs(x0 + x1 - x2 - x3);
    let xa = xc - 4.0 * sx * (x1 - x2);
    let mut xb = sx * (x0 - x1 - x2 + x3);
    let yc = -fabs(y0 + y1 - y2 - y3);
    let ya = yc - 4.0 * sy * (y1 - y2);
    let mut yb = sy * (y0 - y1 - y2 + y3);
    const EP: f64 = 0.01;

    assert!((x1 - x0) * (x2 - x3) < EP && ((x3 - x0) * (x1 - x2) < EP || xb * xb < xa * xc + EP));
    assert!((y1 - y0) * (y2 - y3) < EP && ((y3 - y0) * (y1 - y2) < EP || yb * yb < ya * yc + EP));

    if xa == 0.0 && ya == 0.0 {
        sx = floor((3.0 * x1 - x0 + 1.0) / 2.0);
        sy = floor((3.0 * y1 - y0 + 1.0) / 2.0);
        quad_bezier_seg(pt(x0, y0), pt(sx, sy), pt(x3, y3), plot);
        return;
    }

    x1 = (x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0) + 1.0;
    x2 = (x2 - x3) * (x2 - x3) + (y2 - y3) * (y2 - y3) + 1.0;

    loop {
        let mut ab = xa * yb - xb * ya;
        let mut ac = xa * yc - xc * ya;
        let mut bc = xb * yc - xc * yb;
        let mut ex = ab * (ab + ac - 3.0 * bc) + ac * ac;
        let ff = (ex <= 0.0)
            .then(|| (1.0 + 1024.0 / x1).sqrt())
            .unwrap_or(1.0);
        ab *= ff;
        ac *= ff;
        bc *= ff;
        ex *= ff * ff;
        let mut xy = 9.0 * (ab + ac + bc) / 8.0;
        let mut cb = 8.0 * (xa - ya);
        let mut dx = 27.0 * (8.0 * ab * (yb * yb - ya * yc) + ex * (ya + 2.0 * yb + yc)) / 64.0
            - ya * ya * (xy - ya);
        let mut dy = 27.0 * (8.0 * ab * (xb * xb - xa * xc) - ex * (xa + 2.0 * xb + xc)) / 64.0
            - xa * xa * (xy + xa);
        let mut xx = 3.0
            * (3.0 * ab * (3.0 * yb * yb - ya * ya - 2.0 * ya * yc)
                - ya * (3.0 * ac * (ya + yb) + ya * cb))
            / 4.0;
        let mut yy = 3.0
            * (3.0 * ab * (3.0 * xb * xb - xa * xa - 2.0 * xa * xc)
                - xa * (3.0 * ac * (xa + xb) + xa * cb))
            / 4.0;
        xy = xa * ya * (6.0 * ab + 6.0 * ac - 3.0 * bc + cb);
        ac = ya * ya;
        cb = xa * xa;
        xy = 3.0 * (xy + 9.0 * ff * (cb * yb * yc - xb * xc * ac) - 18.0 * xb * yb * ab) / 8.0;
        if ex < 0.0 {
            dx = -dx;
            dy = -dy;
            xx = -xx;
            yy = -yy;
            xy = -xy;
            ac = -ac;
            cb = -cb;
        }
        ab = 6.0 * ya * ac;
        ac = -6.0 * xa * ac;
        bc = 6.0 * ya * cb;
        cb = -6.0 * xa * cb;
        dx += xy;
        ex = dx + dy;
        dy += xy;

        let mut use_ep = false;
        let mut fx = ff;
        let mut fy = ff;
        'outer: while x0 != x3 && y0 != y3 {
            plot(pt(x0, y0).into());
            loop {
                let ep = if use_ep { EP } else { xy };
                if dx > ep || dy < ep {
                    break 'outer;
                }
                y1 = 2.0 * ex - dy;
                if 2.0 * ex >= dx {
                    fx -= 1.0;
                    dx += xx;
                    ex += dx;
                    xy += ac;
                    dy += xy;
                    yy += bc;
                    xx += ab;
                }
                if y1 <= 0.0 {
                    fy -= 1.0;
                    dy += yy;
                    ex += dy;
                    xy += bc;
                    dx += xy;
                    xx += ac;
                    yy += cb;
                }
                if fx <= 0.0 || fy <= 0.0 {
                    break;
                }
            }
            if 2.0 * fx <= ff {
                x0 += floor(sx);
                fx += ff;
            }
            if 2.0 * fy <= ff {
                y0 += floor(sy);
                fy += ff;
            }
            if !use_ep && dx < 0.0 && dy > 0.0 {
                use_ep = true;
            }
        }
        xx = x0;
        x0 = x3;
        x3 = floor(xx);
        sx = -sx;
        xb = -xb;
        yy = y0;
        y0 = y3;
        y3 = floor(yy);
        sy = -sy;
        yb = -yb;
        x1 = x2;
        leg -= 1;
        if leg < 0 {
            break;
        }
    }
    line(pt(x0, y0), pt(x3, y3), plot);
}

pub fn cubic_bezier<P: Into<Int2>, F: FnMut(Int2)>(p0: P, p1: P, p2: P, p3: P, mut plot: F) {
    fn fp(p: Int2) -> (f64, f64) {
        (p.x as f64, p.y as f64)
    }
    fn pt(x: f64, y: f64) -> (i32, i32) {
        (x as i32, y as i32)
    }
    fn fabs(x: f64) -> f64 {
        x.abs()
    }
    fn floor(x: f64) -> f64 {
        x.floor()
    }
    let (mut x0, mut y0) = fp(p0.into());
    let (x1, y1) = fp(p1.into());
    let (x2, y2) = fp(p2.into());
    let (mut x3, mut y3) = fp(p3.into());

    let xc = x0 + x1 - x2 - x3;
    let xa = xc - 4.0 * (x1 - x2);
    let xb = x0 - x1 - x2 + x3;
    let xd = xb + 4.0 * (x1 + x2);
    let yc = y0 + y1 - y2 - y3;
    let ya = yc - 4.0 * (y1 - y2);
    let yb = y0 - y1 - y2 + y3;
    let yd = yb + 4.0 * (y1 + y2);

    let mut fx0 = x0;
    let mut fy0 = y0;

    let mut t1 = xb * xb - xa * xc;
    let mut t = [0.0; 5];
    let mut n = 0;

    if xa == 0.0 {
        if xc.abs() < 2.0 * xb.abs() {
            t[n] = xc / (2.0 * xb);
            n += 1;
        }
    } else if t1 > 0.0 {
        let t2 = t1.sqrt();
        t1 = (xb - t2) / xa;
        if fabs(t1) < 1.0 {
            t[n] = t1;
            n += 1;
        }
        t1 = (xb + t2) / xa;
        if fabs(t1) < 1.0 {
            t[n] = t1;
            n += 1;
        }
    }

    t1 = yb * yb - ya * yc;

    if ya == 0.0 {
        if yc.abs() < 2.0 * yb.abs() {
            t[n] = yc / (2.0 * yb);
            n += 1;
        }
    } else if t1 > 0.0 {
        let t2 = t1.sqrt();
        t1 = (yb - t2) / ya;
        if fabs(t1) < 1.0 {
            t[n] = t1;
            n += 1;
        }
        t1 = (yb + t2) / ya;
        if fabs(t1) < 1.0 {
            t[n] = t1;
            n += 1;
        }
    }

    let mut i = 1;
    while i < n {
        t1 = t[i - 1];
        if t1 > t[i] {
            t[i - 1] = t[i];
            t[i] = t1;
            i = 0;
        }
        i += 1;
    }

    t1 = -1.0;
    t[n] = 1.0;

    i = 0;
    while i <= n {
        let t2 = t[i];
        let mut fx1 =
            (t1 * (t1 * xb - 2.0 * xc) - t2 * (t1 * (t1 * xa - 2.0 * xb) + xc) + xd) / 8.0 - fx0;
        let mut fy1 =
            (t1 * (t1 * yb - 2.0 * yc) - t2 * (t1 * (t1 * ya - 2.0 * yb) + yc) + yd) / 8.0 - fy0;
        let mut fx2 =
            (t2 * (t2 * xb - 2.0 * xc) - t1 * (t2 * (t2 * xa - 2.0 * xb) + xc) + xd) / 8.0 - fx0;
        let mut fy2 =
            (t2 * (t2 * yb - 2.0 * yc) - t1 * (t2 * (t2 * ya - 2.0 * yb) + yc) + yd) / 8.0 - fy0;
        let fx3 = (t2 * (t2 * (3.0 * xb - t2 * xa) - 3.0 * xc) + xd) / 8.0;
        let fy3 = (t2 * (t2 * (3.0 * yb - t2 * ya) - 3.0 * yc) + yd) / 8.0;
        fx0 -= fx3;
        fy0 -= fy3;
        x3 = floor(fx3 + 0.5);
        y3 = floor(fy3 + 0.5);
        if fx0 != 0.0 {
            fx0 = (x0 - x3) / fx0;
            fx1 *= fx0;
            fx2 *= fx0;
        }
        if fy0 != 0.0 {
            fy0 = (y0 - y3) / fy0;
            fy1 *= fy0;
            fy2 *= fy0;
        }
        if x0 != x3 || y0 != y3 {
            cubic_bezier_seg(
                pt(x0, y0),
                pt(x0 + fx1, y0 + fy1),
                pt(x0 + fx2, y0 + fy2),
                pt(x3, y3),
                &mut plot,
            );
        }
        x0 = x3;
        y0 = y3;
        fx0 = fx3;
        fy0 = fy3;
        t1 = t2;
        i += 1;
    }
}
