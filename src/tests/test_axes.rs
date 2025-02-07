#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::helper::axes::*;

#[allow(dead_code)]
fn format_nums_test_general(nums: Vec<f64>, max_len: usize, expected: Option<Vec<&str>>) {
    let res = format_nums(&nums, max_len);
    println!("{max_len} | {res:?}");
    if expected.is_none(){
        assert!(res.is_none())
    } else {
        assert!(res.clone().unwrap().iter().all(|s| s.len() <= max_len));
        assert_eq!(res, Some(expected.unwrap().into_iter().map(|s| s.to_string()).collect::<Vec<String>>()));
    }
}

#[test]
fn format_nums_test_1() {
    format_nums_test_general(vec![123.45, 67.89], 5, Some(vec!["123.5", "67.89"]));
}

#[test]
fn format_nums_test_2() {
    format_nums_test_general(vec![123.45, 67.89], 4, Some(vec!["123", "67.9"]));
}

#[test]
fn format_nums_test_3() {
    format_nums_test_general(vec![0.0235, 0.4567, 1.2345], 5, Some(vec!["0.023", "0.456", "1.234"]));
}

#[test]
fn format_nums_test_4() {
    format_nums_test_general(vec![123456789.0, 987654321.0], 10, Some(vec!["123456789", "987654321"]));
}

#[test]
fn format_nums_test_5() {
    format_nums_test_general(vec![1.23, 4.56], 3, Some(vec!["1.2", "4.6"]));
}

#[test]
fn format_nums_test_6() {
    format_nums_test_general(vec![123.45, 67.89], 2, None);
}

#[test]
fn format_nums_test_7() {
    format_nums_test_general(vec![0.00123, 456.], 4, Some(vec!["0.00", "456"]));
}

#[test]
fn format_nums_test_8() {
    format_nums_test_general(vec![1000.0, 9999.9], 4, Some(vec!["1000", "9999"]));
}

#[test]
fn format_nums_test_9() {
    format_nums_test_general(vec![0.1, 1.0, 10.0], 2, Some(vec!["0.", "1", "10"]));
}

#[test]
fn add_axes_test_1() {
    let plot = (0..=9usize).map(|i| (0..32usize).map(|_j| format!("{}", 9 - i)).collect()).collect::<Vec<String>>().join("\n");

    let out = add_axes(&plot, ((0., 10.), (0., 9.)));
    let exp = String::from(
"      │99999999999999999999999999999999
7.650 ┼88888888888888888888888888888888
      │77777777777777777777777777777777
5.850 ┼66666666666666666666666666666666
      │55555555555555555555555555555555
4.050 ┼44444444444444444444444444444444
      │33333333333333333333333333333333
2.250 ┼22222222222222222222222222222222
      │11111111111111111111111111111111
0.450 ┼00000000000000000000000000000000
      └┼─────┼─────┼─────┼─────┼───────
       0.156 2.031 3.906 5.781 7.656   ");
    println!("{}", out);
    assert_eq!(out, exp);
}

#[test]
fn add_axes_test_2() {
    let plot = (0..=8usize).map(|i| (0..48usize).map(|j| format!("{}", (j as i32 - i as i32).clamp(0, 9))).collect()).collect::<Vec<String>>().join("\n");

    let out = add_axes(&plot, ((0., 10.), (0., 8.)));
    let exp = String::from(
"7.556 ┼012345678999999999999999999999999999999999999999
      │001234567899999999999999999999999999999999999999
5.778 ┼000123456789999999999999999999999999999999999999
      │000012345678999999999999999999999999999999999999
4.000 ┼000001234567899999999999999999999999999999999999
      │000000123456789999999999999999999999999999999999
2.222 ┼000000012345678999999999999999999999999999999999
      │000000001234567899999999999999999999999999999999
0.444 ┼000000000123456789999999999999999999999999999999
      └┼─────┼─────┼─────┼─────┼─────┼─────┼───────────
       0.104 1.354 2.604 3.854 5.104 6.354 7.604       ");
            println!("{}", out);
            assert_eq!(out, exp);
}

#[test]
fn test_add_opt_axes_and_opt_titles_1() {
    let r = String::new();
    let l = add_opt_axes_and_opt_titles(&r, ((0., 0.), (0., 0.)), false, Some("test"));
    let e = String::from("test\n");

    assert_eq!(l, e);
}