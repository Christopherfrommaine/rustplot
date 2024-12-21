#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::plots::image_plot::*;

#[test]
fn image_plot_test_1 () {
    // only for debugging

    let img: Vec<Vec<(u8, u8, u8)>> = (0..1080).map(|i|
        (0..1920).map(|j|
            (
                (63. * (0.003 * i as f64).powi(3).sin() + (0.07 * j as f64).sin() + 2.) as u8,
                (63. * ((0.02 * j as f64 + 0.1 * i as f64) * 0.03).powi(3).sin() + 2.) as u8,
                (63. * (0.4 * i as f64).sin() + (0.04 * j as f64).sin() + 2.) as u8,
            )
        ).collect()
    ).collect();

    image_plot(img).set_rel_path("src\\tests\\0_image_plot_test_output".to_string()).save();

    assert!(true)
}

#[test]
fn image_plot_test_2 () {
    //only for debugging

    let img: Vec<Vec<(u8, u8, u8)>> = (0..30).map(|i|
        (0..60).map(|j|
            (
                (63. * (0.1 * i as f64).sin() + (0.1 * j as f64).sin() + 2.) as u8,
                (63. * (0.2 * i as f64).sin() + (0.2 * j as f64).sin() + 2.) as u8,
                (63. * (0.4 * i as f64).sin() + (0.4 * j as f64).sin() + 2.) as u8,
            )
        ).collect()
    ).collect();

    image_plot(img).print();

    assert!(true)
}