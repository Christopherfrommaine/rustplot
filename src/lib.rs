//! This is a plotting util for rust which uses ascii/unicode art to display graphs, plots, and images in the standard output, in a string, or saved to a file.
//! 
//! Check out the documentation for more information on each of the plot types and options.
//! 
//! # Basic Syntax / Example Usage
//! 
//! Each plot file contains a public function by the name of that plot. For example,
//! `pub fn array_plot<T>(data: &Vec<Vec<T>>) -> ArrayPlotBuilder<T>`
//! creates an ArrayPlotBuilder instance.
//! 
//! You can then set options for it such as title, axes, output size, and more (depending on the type of plot). Finally, you can call .print() or .as_string() or .pyplot() to print it to the standard output, or return the plot as a string, or display an interactive window with matplotlib, respectively.
//! 
//! For example:
//! ```
//! use cgrustplot::{
//!     plots::array_plot::{array_plot, bin_arr},
//!     helper::charset::gradient_chars::shade_chars,
//! };
//! 
//! // table of sin(x + y)
//! let my_data: Vec<Vec<f64>> = (0..10).map(|x| {(0..30).map(|y| (x as f64 + y as f64).sin()).collect()}).collect();
//! 
//! array_plot(&bin_arr(&my_data, 4)) // Bins the array to have four values
//!     .set_title("A Plot of my Data:")  // Sets the title of the plot
//!     .set_axes(true) // Turns on the axes for the plot
//!     .set_chars(shade_chars()) // uses unicode shade characters
//!     .print()  // Displays the plot
//! ```
//! 
//! And the output of the above code is this plot:
//! 
//! *NOTE: some plots may not display correctly without a monowidth font, such as on github*
//! 
//! ```text
//! A Plot of my Data:
//!       │▓██▓  ░██▓   ▓██░  ▓██░  ░██▓ 
//! 8.500 ┼██▓  ░██▓   ▓██░  ▓██░  ░██▓  
//!       │█▓  ░██▓   ▓██░  ▓██░  ░██▓  ░
//! 6.500 ┼▓  ░██▓   ▓██░  ▓██░  ░██▓  ░█
//!       │  ░██▓   ▓██░  ▓██░  ░██▓  ░██
//! 4.500 ┼ ░██▓   ▓██░  ▓██░  ░██▓  ░███
//!       │░██▓   ▓██░  ▓██░  ░██▓  ░███░
//! 2.500 ┼██▓   ▓██░  ▓██░  ░██▓  ░███░ 
//!       │█▓   ▓██░  ▓██░  ░██▓  ░███░  
//! 0.500 ┼▓   ▓██░  ▓██░  ░██▓  ░███░  ▓
//!       └┼──────┼──────┼──────┼────────
//!        0.5000 7.5000 14.500 21.500   
//! ```
//! # Plot Types
//! In no particular order, here are some of the various types of plots that can be used
//! (more plots than listed here may already be available. Check the /src/plots/ for a full list while in development).
//! 
//! ## Array Plot
//! Filename: array_plot.rs
//! 
//! Displays a table with varying brightness across it's characters.
//! Takes in a `&Vec<Vec<T>>`, representing the brightness value at each point.
//! 
//! The dimensions of the output string are equal to the dimensions of the input table, so if you give a 15x12 grid of values, the output will be about 15x12 characters in size. (Note that axes and title will change this).
//! 
//! ### Options
//! `title: Option<&str>` Sets the title to be displayed above the output. Default is None, which has no displayed title.
//! 
//! `axes: bool` Selects whether to turn on or off the axes. Axes display in units of the number of characters. Default is true
//! 
//! `chars: Vec<String>` The character set to be used. Defaults are based on `rustplot::helper::charset::gradient_chars`, depending on the number of distinct values in the table.
//! 
//! `bins: Option<u32>` Only for plots of f64. It bins close-together datapoints and plots based on those. Not so much a plot option as much as a transformation, as it actually creates a new struct of `<u32>` instead of `<f64>`.
//! 
//! ### Example
//! 
//! Code:
//! ```
//! use cgrustplot::plots::array_plot::array_plot;
//! 
//! let data: Vec<Vec<i32>> = (-5..=5)
//! 	.map(|i: i32|
//! 		(-15..=15).map(|j: i32|
//! 			i.pow(2) + j.pow(2)
//! 		).collect()
//! 	).collect();
//! 
//! array_plot(&data)
//!     .print()
//! ```
//! 
//! Output:
//! ```text
//! 10.50 ┼@%%#**++=---:::::::---=++**#%%@
//!       │@%##**+==--::.....::--==+**##%@
//! 8.500 ┼@%##*++=--::.......::--=++*##%@
//!       │@%##*+==-::...   ...::-==+*##%@
//! 6.500 ┼@%#**+==-::..     ..::-==+**#%@
//!       │%%#**+=--::..     ..::--=+**#%%
//! 4.500 ┼@%#**+==-::..     ..::-==+**#%@
//!       │@%##*+==-::...   ...::-==+*##%@
//! 2.500 ┼@%##*++=--::.......::--=++*##%@
//!       │@%##**+==--::.....::--==+**##%@
//! 0.500 ┼@%%#**++=---:::::::---=++**#%%@
//!       └┼──────┼──────┼──────┼─────────
//!        0.5000 7.5000 14.500 21.500   
//! ```
//! 
//! ## Function Plot
//! Filename: func_plot.rs
//! 
//! Displays the output of a numerically-valued function over a domain.
//! Takes in a `Fn(U) -> V` for types which can be cast to and from f64, respectively.
//! 
//! ### Options
//! `domain: (f64, f64)` Sets the domain (i.e. min and max x values) over which to plot the function. If no domain is selected, `rustplot::helper::func_plot_domain::determine_plot_domain` will be used as a default. By a variety of heuristic methods, it is usually able to determine a domain over which some useful behavior can be observed.
//! 
//! `range: (f64, f64)` Sets the range (i.e. min and max y values) over which to plot the function. Default comes from the min and max values of the function within it's domain.
//! 
//! `domain_padding: f64` Pads the domain by some percentage. For example, with padding of 0.01, the domain (0, 10) gets turned into (-0.1, 10.1).
//! 
//! `range_padding: f64` Pads the range by some percentage.
//! 
//! `size: (u32, u32)` Sets the size of the output of the plot, measured in number of characters.
//! 
//! `title: Option<&str>`
//! 
//! `axes: bool`
//! 
//! ### Example
//! 
//! Code:
//! ```
//! use cgrustplot::plots::function_plot::function_plot;
//! 
//! let f = |x: f64| x.powi(3);
//! 
//! function_plot(&f)
//!     .set_size((30, 10))
//!     .print();
//! ```
//! 
//! Output:
//! ```text
//!       │                           _‾ 
//! 1.293 ┼                          /   
//!       │                        _‾    
//! 0.554 ┼                     _―‾      
//!       │         _――――――――――‾         
//! -0.18 ┼      _―‾                     
//!       │    _‾                        
//! -0.92 ┼   /                          
//!       │  /                           
//! -1.66 ┼ /                            
//!       └┼──────┼──────┼──────┼────────
//!        -1.339 -0.692 -0.046 0.6004   
//! ```
//! 
//! ## Scatter Plot
//! Filename: scatter_plot.rs
//! 
//! Displays a scatter plot from a given set of points `Vec<(f64, f64)>`.
//! 
//! ### Options
//! `range: ((f64, f64), (f64, f64))` Sets the domain as well as range (i.e. min and max x and values) over which to plot the function. Sorry about the inconsistency between range here and in other places. It may be fixed in the future.
//! 
//! `padding: f64` Pads both the domain and range by some percentage.
//! 
//! `size: (u32, u32)`
//! 
//! `title: Option<&str>`
//! 
//! `axes: bool`
//! 
//! `chars: (Vec<char>, (u32, u32))` The character set to be used. Defaults are based on `rustplot::helper::charset::subdiv_chars`, depending on the number of unique points that would be overwritten at each size. (i.e. it tries not to show two points in only a single character). You must specify the dimension of each character. For example, braille characters allow you to plot a grid of 2x4 dots for each character, so you must pass in (2, 4) in addition to the character set.
//! 
//! ### Example
//! Code:
//! ```
//! use cgrustplot::plots::scatter_plot::scatter_plot;
//! use cgrustplot::helper::charset::subdiv_chars::dots_two_by_four;
//! use rand::{Rng, SeedableRng, rngs::StdRng};
//! 
//! // Generate some random data points within ((0, 60), (0, 30))
//! let mut rng: StdRng = SeedableRng::seed_from_u64(0);
//! let data: Vec<(f64, f64)> = (0..100).map(|_|
//!         (rng.gen_range(0.0..60.0), rng.gen_range(0.0..30.0))
//!     ).collect();
//! 
//! scatter_plot(&data)
//!     .set_size((30, 10))
//!     .set_chars((dots_two_by_four(), (2, 4)))
//!     .set_range(((0., 60.), (0., 30.)))
//!     .set_padding(0.)
//!     .print();
//! ```
//! 
//! Output:
//! ```text
//!       │⠔⡈⢀⠀⠂⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠂⢁⠀⠀⠂⠀⠀⠀⡀⠀⠀⠄
//! 25.50 ┼⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⢀⠀⠀⠁⠐⠀⠁⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐
//!       │⠀⠀⠀⠄⠀⠄⠀⠠⠀⠀⡀⠈⠀⠈⠀⠀⠀⠀⠠⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀
//! 19.50 ┼⠄⠀⠁⠀⠀⠀⠁⠀⠡⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠈⠀⠀⠐⠀⠀
//!       │⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀
//! 13.50 ┼⢀⠀⠀⠀⠀⠁⠣⠀⠀⠀⠠⠄⠀⠀⠰⠀⠀⢂⠀⠀⠀⠀⠀⠁⠀⠀⣀⠀⠀⠀
//!       │⠠⢀⠀⠀⠅⠠⠀⠀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠁⠈⠀⠀⠀⠀⠠⠀⠐⠀⣀⠠⠠
//! 7.500 ┼⠀⠀⠈⠂⠀⠀⢀⠡⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⡀⠠⠀⠀⠀⠀⠀⠀⠀⠀
//!       │⠀⠐⠀⠀⠐⠀⠀⠀⠀⠀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⢀⠄⠁⠄⠄⠀⠀⠀
//! 1.500 ┼⠀⢀⡀⠄⠁⠀⢀⠀⠀⠀⠀⠀⠀⠈⠀⢀⠀⠀⠀⠀⠀⢀⠄⠀⠀⠁⠀⠀⠀⠀
//!       └┼──────┼──────┼──────┼────────
//!        1.0000 15.000 29.000 43.000   
//! ```
//! 
//! 
//! ## Region Plot
//! Filename: Region_plot.rs
//! 
//! Displays the region over which a predicate `pred: Fn(f64, f64) -> bool` is true.
//! 
//! ### Options
//! `range: ((f64, f64), (f64, f64))` Sets the domain as well as range (i.e. min and max x and values) over which to plot the predicate
//! 
//! `padding: f64`
//! 
//! `size: (u32, u32)`
//! 
//! `title: Option<&str>`
//! 
//! `axes: bool`
//! 
//! ### Example
//! Code:
//! ```
//! use cgrustplot::plots::region_plot::region_plot;
//! 
//! let p = |x: f64, y: f64| (x.powi(2) + y.powi(2)).sqrt() <= 0.7;
//! 
//! region_plot(&p)
//!     .set_domain_and_range(((-1., 1.), (-1., 1.)))
//!     .set_size((30, 10))
//!     .print();
//! ```
//! 
//! Output:
//! ```text
//!       │                         
//! 0.840 ┼                         
//!       │          ▄▄▄▄▖          
//! 0.360 ┼      ▗▟█████████▄       
//!       │     ▗████████████▙      
//! -0.12 ┼     ▜█████████████▘     
//!       │      ▜███████████▘      
//! -0.60 ┼       ▝▀▀████▛▀▀        
//!       │                         
//! -1.08 ┼                         
//!       └┼─────┼─────┼─────┼──────
//!        -1.15 -0.57 0.000 0.576  
//! ```

//! # Structure
//! 
//! This crate is structured into plots, helper functions, and tests.
//! 
//! The `plots` module contains all the plot types.
//! 
//! The `helper` module contains extra functions used throughout the crate.
//! 
//! The 'tests` module (private) implemenents unit tests and full tests for plots and helpers.
//! 
//! # Contribution and Development
//! 
//! So far, this has been a completely solo project. If you have anything to add, or would like to
//! help out in any way, just reach out or submit your changes with a pull request.
//! 
//! In order to properly run tests involving saving files, create a directory called "testoutput"
//! in the workspace folder.
//! 
pub mod helper;
pub mod plots;
mod tests;
