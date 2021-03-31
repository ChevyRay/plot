mod circle;
mod cubic_bezier;
mod ellipsis;
mod line;
mod quad_bezier;

pub use circle::circle;
pub use cubic_bezier::cubic_bezier;
pub use ellipsis::ellipsis;
pub use line::line;
pub use quad_bezier::quad_bezier;

#[test]
fn test_me() {
    use grid::{Grid, VecGrid};
    use std::fmt::Write;

    let mut g = VecGrid::<char>::new(50, 50, '.');

    cubic_bezier((0, 0), (99, 49), (-49, 49), (49, 0), |p| {
        g.set(p, 'X');
    });

    let mut str = String::new();
    for y in 0..g.height() {
        for x in 0..g.width() {
            write!(str, "{}", g.get((x, y)).unwrap());
        }
        writeln!(str);
    }
    println!("{}", str);
}
