#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::plots::func_plot::*;

#[test]
fn test_func_plot_1() {
    // Not a great test, but it's more of a visual thing.

    let f = |x: f64| 1. + (x.powi(2));
    
    let o = function_plot(&f).set_axes(false).as_string();
    println!("{}", o);

    assert!(o.contains("_") && o.contains("―") && o.contains("╱"));
}

#[test]
fn test_func_plot_2() {
    // Not a great test, but it's more of a visual thing.

    let f = |x: f64| x;
    
    let o = 
        function_plot(&f)
        .set_domain((0., 1.))
        .set_title("Linear Plot Test")
        .set_domain_padding(0.)
        .set_range((0., 1.5))
        .set_range_padding(0.)
        .as_string();
    println!("{}", o);

    assert!(o.contains("_") && o.contains("―"));
}