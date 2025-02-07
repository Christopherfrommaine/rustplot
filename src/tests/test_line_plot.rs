#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::plots::line_plot::*;

#[test]
fn test_line_plot_1() {
    // Used for debugging

    let d: Vec<(f64, f64)> = (0..100).map(|i: i32| (i as f64, i.pow(2) as f64 / 10.)).collect();

    println!("d: {d:?}");

    line_plot(&d)
        .set_axes(false)
        .plot();

    assert!(true);
}

#[test]
fn test_line_plot_2() {
    // Used for debugging

    let d: Vec<(f64, f64)> = (-100..100).map(|i: i32| (i as f64, i.pow(3) as f64 / 10000.)).collect();

    line_plot(&d).plot();

    assert!(true);
}