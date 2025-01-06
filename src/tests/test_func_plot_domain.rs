#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::helper::func_plot_domain::*;

#[test]
fn determine_plot_domain_test_1() {
    let f = |x: f64| 1. + x.powi(2);
    let o = determine_plot_domain(f);
    println!("test 1: {o:?}");
    let acceptable_domain = ((-0.1, -10.), (0.1, 10.));
    assert!(o.0 < acceptable_domain.0.0 && o.0 > acceptable_domain.0.1 && o.1 > acceptable_domain.1.0 && 0.1 < acceptable_domain.1.1);
}

#[test]
fn determine_plot_domain_test_2() {
    let f = |x: f64| 1. + x + x.powi(2) - 3. * x.powi(3) + x.powi(4);
    let o = determine_plot_domain(f);
    println!("test 2: {o:?}");
    let acceptable_domain = ((-0.1, -10.), (0.1, 10.));
    assert!(o.0 < acceptable_domain.0.0 && o.0 > acceptable_domain.0.1 && o.1 > acceptable_domain.1.0 && 0.1 < acceptable_domain.1.1);
}

#[test]
fn determine_plot_domain_test_3() {
    let f = |x: f64| (x.exp() / (1. + x.exp()));
    let o = determine_plot_domain(f);
    println!("test 3: {o:?}");
    let acceptable_domain = ((-0.1, -100.), (0.1, 100.));
    assert!(o.0 < acceptable_domain.0.0 && o.0 > acceptable_domain.0.1 && o.1 > acceptable_domain.1.0 && 0.1 < acceptable_domain.1.1);
}

#[test]
fn determine_plot_domain_test_4() {
    let f = |x: f64| x.sin();
    let o = determine_plot_domain(f);
    println!("test 4: {o:?}");
    let acceptable_domain = ((-3., -30.), (3., 30.));
    assert!(o.0 < acceptable_domain.0.0 && o.0 > acceptable_domain.0.1 && o.1 > acceptable_domain.1.0 && 0.1 < acceptable_domain.1.1);
}

#[test]
fn determine_plot_domain_test_5() {
    let f = |x: f64| if x <= 0. { 1. / x } else if x <= 9. { 1. + 0.2 * x } else { 3. };
    let o = determine_plot_domain(f);
    println!("test 5: {o:?}");
    let acceptable_domain = ((-1., -20.), (1., 20.));
    assert!(o.0 < acceptable_domain.0.0 && o.0 > acceptable_domain.0.1 && o.1 > acceptable_domain.1.0 && 0.1 < acceptable_domain.1.1);
}

#[test]
fn determine_plot_domain_test_6() {
    let f = |x: f64| 2. * x;
    let o = determine_plot_domain(f);
    println!("test 6: {o:?}");
    let acceptable_domain = ((-0.1, -10.), (0.1, 10.));
    assert!(o.0 < acceptable_domain.0.0 && o.0 > acceptable_domain.0.1 && o.1 > acceptable_domain.1.0 && 0.1 < acceptable_domain.1.1);
}

#[test]
fn determine_plot_domain_test_7() {
    let f = |_x: f64| 5.;
    let o = determine_plot_domain(f);
    println!("test 7: {o:?}");
    let acceptable_domain = ((-0.1, -10.), (0.1, 10.));
    assert!(o.0 < acceptable_domain.0.0 && o.0 > acceptable_domain.0.1 && o.1 > acceptable_domain.1.0 && 0.1 < acceptable_domain.1.1);
}

#[test]
fn determine_plot_domain_test_8() {
    let f = |x: f64| if x != 0. { 1. / x } else { 1. };
    let o = determine_plot_domain(f);
    println!("test 8: {o:?}");
    let acceptable_domain = ((-0.1, -10.), (0.1, 10.));
    assert!(o.0 < acceptable_domain.0.0 && o.0 > acceptable_domain.0.1 && o.1 > acceptable_domain.1.0 && 0.1 < acceptable_domain.1.1);
}

#[test]
fn determine_plot_domain_test_9() {
    let f = |x: f64| if x != 0. { (1. / x).sin() } else { 1. };
    let o = determine_plot_domain(f);
    println!("test 9: {o:?}");
    let acceptable_domain = ((-0.1, -10.), (0.1, 10.));
    assert!(o.0 < acceptable_domain.0.0 && o.0 > acceptable_domain.0.1 && o.1 > acceptable_domain.1.0 && 0.1 < acceptable_domain.1.1);
}

