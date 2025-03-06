#![allow(unused_imports)] use std::fs::create_dir;

// imports are used, but doesn't detect it?
use crate::plots::*;
use crate::helper::file::get_current_dir;
use std::env::var;
use std::fs::exists;

#[test]
fn save_array_test_1() {
    if var("NO_EXTERNAL_COMMANDS").is_ok() {return;}

    let d: Vec<Vec<u32>> = (0..100).map(|y| (0..60).map(|x| ((0.2 * ((2 * x) as f64 + y as f64)).sin() * 10.) as u32).collect()).collect();
    let path = get_current_dir() + "testoutput/save_array_test_1.txt";
    array_plot::array_plot(&d).save(&path);
    
    assert!(exists(path).is_ok());
}

#[test]
fn save_array_image_test_1() {
    if var("NO_EXTERNAL_COMMANDS").is_ok() {return;}

    let d: Vec<Vec<u32>> = (0..100).map(|y| (0..60).map(|x| ((0.2 * ((2 * x) as f64 + y as f64)).sin() * 10.) as u32).collect()).collect();
    let path = get_current_dir() + "testoutput/save_array_image_test_1.png";
    array_plot::array_plot(&d).as_image().save(&path);
    
    assert!(exists(path).is_ok());
}

#[test]
fn save_function_test_1() {
    if var("NO_EXTERNAL_COMMANDS").is_ok() {return;}

    let d = |x: f64| 0.01 * x * x * x + x.sin();
    let path = get_current_dir() + "testoutput/save_function_test_1.txt";
    function_plot::function_plot(&d).save(&path);
    
    assert!(exists(path).is_ok());
}

#[test]
fn save_function_image_test_1() {
    if var("NO_EXTERNAL_COMMANDS").is_ok() {return;}

    let d = |x: f64| 0.01 * x * x * x + x.sin();
    let path = get_current_dir() + "testoutput/save_function_image_test_1.png";
    function_plot::function_plot(&d).as_image().save(&path);
    
    assert!(exists(path).is_ok());
}


#[test]
fn save_region_test_1() {
    if var("NO_EXTERNAL_COMMANDS").is_ok() {return;}

    let d = |x: f64, y: f64| (x.sin() + y.sin()) > 0.;
    let path = get_current_dir() + "testoutput/save_region_test_1.txt";
    region_plot::region_plot(&d).set_domain_and_range(((-10., 10.), (-10., 10.))).save(&path);
    
    assert!(exists(path).is_ok());
}

#[test]
fn save_region_image_test_1() {
    if var("NO_EXTERNAL_COMMANDS").is_ok() {return;}

    let d = |x: f64, y: f64| (x.sin() + y.sin()) > 0.;
    let path = get_current_dir() + "testoutput/save_region_image_test_1.png";
    region_plot::region_plot(&d).set_domain_and_range(((-10., 10.), (-10., 10.))).as_image().save(&path);
    
    assert!(exists(path).is_ok());
}

#[test]
fn save_scatter_test_1() {
    if var("NO_EXTERNAL_COMMANDS").is_ok() {return;}

    let d: Vec<(f64, f64)> = (0..100).map(|x| (x as f64, (x as f64).sin())).collect();
    let path = get_current_dir() + "testoutput/save_scatter_test_1.txt";
    scatter_plot::scatter_plot(&d).save(&path);
    
    assert!(exists(path).is_ok());
}

#[test]
fn save_scatter_image_test_1() {
    if var("NO_EXTERNAL_COMMANDS").is_ok() {return;}

    let d: Vec<(f64, f64)> = (0..100).map(|x| (x as f64, (x as f64).sin())).collect();
    let path = get_current_dir() + "testoutput/save_scatter_image_test_1.png";
    scatter_plot::scatter_plot(&d).as_image().save(&path);
    
    assert!(exists(path).is_ok());
}

