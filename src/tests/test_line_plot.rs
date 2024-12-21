#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::plots::line_plot::*;

#[test]
fn test_line_plot_1() {
    // Used for debugging

    let d: Vec<(f64, f64)> = (0..100).map(|i: i32| (i as f64, i.pow(2) as f64 / 10.)).collect();

    let o = line_plot(d).as_string();

    println!("{}", o);

    assert!(true);
}
