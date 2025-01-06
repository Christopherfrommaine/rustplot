#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::helper::axes::*;

#[test]
fn add_axes_test_1() {
    let plot = (0..=9usize).map(|i| (0..32usize).map(|_j| format!("{}", 9 - i)).collect()).collect::<Vec<String>>().join("\n");

    let out = add_axes(&plot, ((0., 10.), (0., 9.)));
    let exp = String::from(
"9┼99999999999999999999999999999999
 │88888888888888888888888888888888
 │77777777777777777777777777777777
6┼66666666666666666666666666666666
 │55555555555555555555555555555555
 │44444444444444444444444444444444
3┼33333333333333333333333333333333
 │22222222222222222222222222222222
 │11111111111111111111111111111111
0┼00000000000000000000000000000000
 └┼──────┼──────┼──────┼──────┼───
  0.000  2.258  4.516  6.774  9.02");
    println!("{}", out);
    assert_eq!(out, exp);
}

#[test]
fn add_axes_test_2() {
    let plot = (0..=8usize).map(|i| (0..32usize).map(|_j| format!("{}", 8 - i)).collect()).collect::<Vec<String>>().join("\n");

    let out = add_axes(&plot, ((0., 10.), (0., 8.)));
    let exp = String::from(
" │88888888888888888888888888888888
 │77777777777777777777777777777777
6┼66666666666666666666666666666666
 │55555555555555555555555555555555
 │44444444444444444444444444444444
3┼33333333333333333333333333333333
 │22222222222222222222222222222222
 │11111111111111111111111111111111
0┼00000000000000000000000000000000
 └┼──────┼──────┼──────┼──────┼───
  0.000  2.258  4.516  6.774  9.02");
            println!("{}", out);
            assert_eq!(out, exp);
}

#[test]
fn test_add_opt_axes_and_opt_titles_1() {
    let r = String::new();
    let l = add_opt_axes_and_opt_titles(&r, ((0., 0.), (0., 0.)), false, &Some(String::from("test")));
    let e = String::from("test\n");

    assert_eq!(l, e);
}