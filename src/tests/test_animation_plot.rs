#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::plots::animation_plot::*;

#[test]
fn animation_plot_test_1 () {
    // only for debugging

    let ani: Vec<Vec<Vec<(u8, u8, u8)>>> = (0..30).map(|t|
        (0..480).map(|i|
            (0..720).map(|j|
                (
                    (63. * (0.01 * i as f64 + 0.1 * t as f64 + 0.01 * j as f64).sin() + 2.) as u8,
                    (63. * (0.02 * j as f64 + 1. * t as f64).sin() + (0.1 * i as f64).sin() + 2.) as u8,
                    (63. * (0.4 * i as f64).sin() + (0.04 * j as f64).sin() + 2.) as u8,
                )
            ).collect()
        ).collect()
    ).collect();
    println!("Finished");

    animation_plot(&ani).set_rel_path("src\\tests\\0_ani_plot_test_output".to_string()).set_framerate(10).set_overwrite(true).save();

    
}
