#![allow(unused_imports)] use std::fs::create_dir;

// imports are used, but doesn't detect it?
use crate::helper::charset::subdiv_chars::*;
use crate::plots::scatter_plot::*;

#[test]
fn bool_arr_plot_test_1() {
    let range = (4, 3);
    let chrset = vec![' ', '#'];

    let arr = vec![vec![true, true, true, false], vec![true, true, false, true], vec![true, false, true, true]];
    
    let res = bool_arr_plot_string_custom_charset(&arr, range, (chrset, (1, 1)));

    assert_eq!(res, "### \n## #\n# ##");
}

#[test]
fn bool_arr_plot_test_2() {
    let range = (4, 3);
    let chrset = vec![' ', 'l', 'r', 'b'];

    let arr = vec![vec![true, true, true, false], vec![true, true, false, true], vec![true, false, true, true]];
    
    let res = bool_arr_plot_string_custom_charset(&arr, range, (chrset, (2, 1)));

    assert_eq!(res, "bl\nbr\nlb")
}

#[test]
fn bool_arr_plot_test_3() {
    let range = (3, 3);
    let chrset = vec![' ', 'l', 'r', 'b'];

    let arr = vec![vec![true, true, false], vec![true, false, true], vec![false, true, true]];
    
    let res = bool_arr_plot_string_custom_charset(&arr, range, (chrset, (2, 1)));
    println!("{}", res);

    assert_eq!(res, "b \nll\nrl")
}

#[test]
fn bool_arr_plot_test_4() {
    let range = (3, 3);
    let chrset = vec![' ', 'u', 'l', 'b'];

    let arr = vec![
        vec![true, true, false],
        vec![true, false, true],
        vec![false, true, true]];
    
    let res = bool_arr_plot_string_custom_charset(&arr, range, (chrset, (1, 2)));
    println!("{}", res);

    assert_eq!(res, "bul\n uu")

}

#[test]
fn bool_arr_plot_test_5() {
    let range = (3, 3);
    let chrset = dots_two_by_four();

    let arr = vec![
        vec![true, true, false],
        vec![true, false, true],
        vec![false, true, true]];
    
    let res = bool_arr_plot_string_custom_charset(&arr, range, (chrset, (2, 4)));
    println!("{}", res);

    assert_eq!(res, "⠫⠆")
}

#[test]
fn determine_char_set_test_1() {
    let mat = vec![(1, 1)];
    let res = determine_char_set(&mat, ((0., 10.), (0., 10.)), (10, 10));

    assert_eq!(res.0, dots_one_by_one())
}

#[test]
fn determine_char_set_test_2() {
    let mat = vec![(1, 1), (1, 1), (1, 3), (1, 1), (4, 5), (1, 1)];
    let res = determine_char_set(&mat, ((0., 10.), (0., 10.)), (10, 10));

    println!("{}", res.0.iter().collect::<String>());

    assert_eq!(res.0, blocks_two_by_two())
}

#[test]
fn determine_char_set_test_3() {
    let mat = vec![(1, 1); 1000];
    let res = determine_char_set(&mat, ((0., 10.), (0., 10.)), (10, 10));

    assert_eq!(res.0, dots_two_by_four())
}

#[test]
fn determine_char_set_test_4() {
    let mat: Vec<(i32, i32)> = Vec::new();
    let res = determine_char_set(&mat, ((0., 10.), (0., 10.)), (10, 10));

    assert_eq!(res.0, dots_one_by_one())
}

#[test]
fn scatter_plot_test_1() {
    let pts = vec![(1, 2), (3, 4), (5, 6)];
    let res = scatter_plot(&pts)
        .set_size((5, 5))
        .set_axes(false)
        .as_string();
    let exp = String::from(
"●    
     
  ●  
     
    ●");

    println!("{res}");

    assert_eq!(res, exp);
}

#[test]
fn scatter_plot_test_2() {
    let pts = vec![(1, 1), (2, 2), (1, 1), (1, 1)];
    let res = scatter_plot(&pts)
        .set_size((10, 10))
        .set_axes(false)
        .as_string();
    let exp = String::from(
"▗         
          
          
          
          
          
          
          
          
         ▘");

    println!("{res}");

    assert_eq!(res, exp);
}

#[test]
fn scatter_plot_test_3() {
    let pts = vec![(1, 2), (3, 4), (5, 6), (5, 6), (7, 8), (8, 9)];
    let res = scatter_plot(&pts)
        .set_size((1, 1))
        .set_axes(false)
        .as_string();
    let exp = String::from("⢣");

    println!("{res}");

    assert_eq!(res, exp);
}

#[test]
fn scatter_plot_test_4() {
    let pts = vec![(1, 2), (3, 4), (5, 6)];
    let res = scatter_plot(&pts)
        .set_size((20, 20))
        .set_axes(false)
        .as_string();
    assert!(res.chars().any(|i| i == '●'));
    assert!(res.len() == 425);
}

#[test]
fn scatter_plot_test_5() {
    let pts = (0..20).flat_map(|i| (0..20).map(move |j| (i, j))).collect();
    let res = scatter_plot(&pts)
        .set_size((5, 5))
        .set_axes(false)
        .set_chars((crate::helper::charset::subdiv_chars::blocks_two_by_two(), (2, 2)))
        .as_string();
    let exp = String::from(
"█████
█████
█████
█████
█████");
    
    println!("{}", res);

    assert_eq!(res, exp);
}


#[test]
fn scatter_plot_test_6() {
    let pts = (0..100).flat_map(|i| (0..100).map(move |j| (i, j))).collect();
    let res = scatter_plot(&pts)
        .set_size((5, 5))
        .set_axes(false)
        .as_string();
    let exp = String::from(
"⣶⣶⣶⣶⣶
⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿
⠿⠿⠿⠿⠿");
    
    println!("{}", res);

    assert_eq!(res, exp);
}

#[test]
fn char_set_test_1() {
    // Unused -- only used for manually displaying and checking
    let chs = crate::helper::charset::subdiv_chars::dots_two_by_four();
    (0..chs.len()).for_each(|i| println!("{:008b} | {}", i, chs[i]));
    assert!(true);
}

