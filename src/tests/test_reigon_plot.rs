#[allow(unused_imports)]
use crate::plots::reigon_plot::reigon_plot;

#[test]
fn reigon_plot_test_1() {
    let p = |x: f64, y: f64| (x.powi(2) + y.powi(2)).sqrt() <= 0.7;

    let o = reigon_plot(p)
        .set_domain_and_range(((-1., 1.), (-1., 1.)))
        .set_size((30, 10))
        .set_axes(false)
        .as_string();

    let e = 
"                              
                              
           ▗▄▄▄▄▄▖            
       ▗▄▄▄▟█████▙▄▄▄▖        
      ▗▟█████████████▙▖       
      ▐███████████████▌       
      ▝▜█████████████▛▘       
       ▝▀▀▀▜█████▛▀▀▀▘        
           ▝▀▀▀▀▀▘            
                              ";

    println!("{}", o);

    assert_eq!(o, e);
}