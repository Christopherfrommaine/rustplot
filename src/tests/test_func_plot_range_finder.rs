#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::helper::func_plot_range_finder::*;

#[test]
fn determine_plot_range_test_1() {
    let f = |x: f64| 1. + x.powi(2);
    let o = determine_plot_range(f);
    println!("test 1: {o:?}");
    assert_eq!(o, (0., 0.));  // Leave blank for now 
}

#[test]
fn determine_plot_range_test_2() {
    let f = |x: f64| 1. + x + x.powi(2) - 3. * x.powi(3) + x.powi(4);
    let o = determine_plot_range(f);
    println!("test 2: {o:?}");
    assert_eq!(o, (0., 0.));  // Leave blank for now 
}

#[test]
fn determine_plot_range_test_3() {
    let f = |x: f64| (x.exp() / (1. + x.exp())) + 0.001 * x; // The linear term is needed because of precision errors
    let o = determine_plot_range(f);
    println!("test 3: {o:?}");
    assert_eq!(o, (0., 0.));  // Leave blank for now 
}

#[test]
fn determine_plot_range_test_4() {
    let f = |x: f64| x.sin();
    let o = determine_plot_range(f);
    println!("test 4: {o:?}");
    assert_eq!(o, (0., 0.));  // Leave blank for now 
}

#[test]
fn determine_plot_range_test_5() {
    let f = |x: f64| if x <= 0. { x } else if x <= 9. { 1. + 0.2 * x } else { 3. };
    let o = determine_plot_range(f);
    println!("test 5: {o:?}");
    assert_eq!(o, (0., 0.));  // Leave blank for now 
}

#[test]
fn determine_plot_range_test_6() {
    let f = |x: f64| 2. * x;
    let o = determine_plot_range(f);
    println!("test 6: {o:?}");
    assert_eq!(o, (0., 0.));  // Leave blank for now 
}

#[test]
fn determine_plot_range_test_7() {
    let f = |_x: f64| 5.;
    let o = determine_plot_range(f);
    println!("test 7: {o:?}");
    assert_eq!(o, (0., 0.));  // Leave blank for now 
}

#[test]
fn determine_plot_range_test_8() {
    let f = |x: f64| if x != 0. { 1. / x } else { 1. };
    let o = determine_plot_range(f);
    println!("test 8: {o:?}");
    assert_eq!(o, (0., 0.));  // Leave blank for now 
}

#[test]
fn determine_plot_range_test_9() {
    let f = |x: f64| if x != 0. { x.sin().sin() } else { 1. };
    let o = determine_plot_range(f);
    println!("test 9: {o:?}");
    assert_eq!(o, (0., 0.));  // Leave blank for now 
}

