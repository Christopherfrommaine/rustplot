#![allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::plots::*;
use std::{env::var, vec};

#[test]
fn test_array_plot_0() {
    if var("PYPLOT_TESTS").is_err() {return;}

    let vec_mat: Vec<Vec<f64>> =
        (0..50).map(|i| {
            (0..50).map(|j|
                (((i - 25) * (i - 25) + (j - 25) * (j - 25)) as f64).sqrt()
            ).collect()
        }).collect();
    
    let binned = array_plot::bin_arr(&vec_mat, 8);

    array_plot::array_plot(&binned).pyplot();
}

#[test]
fn test_array_plot_00() {
    if var("PYPLOT_TESTS").is_err() {return;}

    let vec_mat: Vec<Vec<f64>> =
        (0..50).map(|i| {
            (0..50).map(|j|
                (((i - 25) * (i - 25) + (j - 25) * (j - 25)) as f64).sqrt()
            ).collect()
        }).collect();
    
    array_plot::array_plot(&vec_mat).pyplot();
}

#[test]
fn test_func_plot_0() {
    if var("PYPLOT_TESTS").is_err() {return;}

    let f = |x: f64| x.powi(3);

    function_plot::function_plot(&f).pyplot();
}

#[test]
fn test_func_plot_00() {
    if var("PYPLOT_TESTS").is_err() {return;}

    let f = |x: f64| (3. * x).sin();

    function_plot::function_plot(&f).pyplot();
}

#[test]
fn test_image_plot_0() {
    if var("PYPLOT_TESTS").is_err() {return;}

    let img: Vec<Vec<(u8, u8, u8)>> = (0..1080).map(|i|
        (0..1920).map(|j|
            (
                (63. * (0.003 * i as f64).powi(3).sin() + (0.07 * j as f64).sin() + 2.) as u8,
                (63. * ((0.02 * j as f64 + 0.1 * i as f64) * 0.03).powi(3).sin() + 2.) as u8,
                (63. * (0.4 * i as f64).sin() + (0.04 * j as f64).sin() + 2.) as u8,
            )
        ).collect()
    ).collect();

    image_plot::image_plot(&img).pyplot();
}

#[test]
fn test_line_plot_0() {
    if var("PYPLOT_TESTS").is_err() {return;}

    let d: Vec<(f64, f64)> = (-20..20).map(|i: i32| (i as f64, i.pow(2) as f64 / 10000.)).collect();

    let plt = line_plot::line_plot(&d);

    plt.pyplot();
}

#[test]
fn test_region_plot_0() {
    if var("PYPLOT_TESTS").is_err() {return;}

    let p = |x: f64, y: f64| (x.powi(2) + y.powi(2)).sqrt() <= 0.7;

    region_plot::region_plot(&p)
        .set_domain_and_range(((-1., 1.), (-1., 1.)))
        .pyplot();
}

#[test]
fn test_scatter_plot_0() {
    if var("PYPLOT_TESTS").is_err() {return;}

    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let data: Vec<(f64, f64)> = (0..100).map(|_|
            (rng.random_range(0.0..60.0), rng.random_range(0.0..30.0))
        ).collect();

    scatter_plot::scatter_plot(&data)
    .set_size((30, 10))
    .set_range(((0., 60.), (0., 30.)))
    .set_padding(0.)
    .pyplot();
}