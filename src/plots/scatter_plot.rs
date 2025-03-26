//! # Scatter Plot
//! Displays scatter plot of a list of given points.
//! 
//! # Functions
//! 
//! * `scatter_plot` - Generates a RegionPlotBuilder from a predicate.
//! * `list_as_points` - Enumerates a list to generate 2D points. (e.g. [8, 3, 4, 6] -> [(0, 8), (1, 3), (2, 4), (3, 6)]).
//! 

use num::ToPrimitive;
use rayon::prelude::*;

use crate::helper::{
    arrays::{padded_vec_to, table_indices_to_counts, transpose_table},
    axes::add_opt_axes_and_opt_titles,
    charset::subdiv_chars::*,
    mat_plot_lib::pyplot,
    math::{bin_to_u8, ciel_div, max_always, min_always, pad_range},
    file::save_to_file,
    rendering::RenderableTextBuilder,
};

/// Pads a range by a ratio of it's width
fn pad_point_range(points: &Vec<(f64, f64)>, padding: f64) -> ((f64, f64), (f64, f64)) {
    (
        pad_range((min_always(&points.iter().map(|i| i.0).collect(), 0.),
            max_always(&points.iter().map(|i| i.0).collect(), 0.)), padding),
        pad_range((min_always(&points.iter().map(|i| i.1).collect(), 0.),
            max_always(&points.iter().map(|i| i.1).collect(), 0.)), padding)
    )
}

pub(crate) fn padded_point_range<T: PartialOrd + Copy + ToPrimitive>(points: &Vec<(T, T)>, padding: f64) -> ((f64, f64), (f64, f64)) {
    pad_point_range(
        &points
            .iter()
            .map(|t|
                (match t.0.to_f64() {Some(val) => val, None => 0.},
                match t.1.to_f64() {Some(val) => val, None => 0.})
            ).collect::<Vec<(f64, f64)>>(),
        padding
    )
}

pub(crate) fn determine_char_set<T: ToPrimitive + PartialEq>(points: &Vec<(T, T)>, range: ((f64, f64), (f64, f64)), size: (u32, u32)) -> (Vec<char>, (u32, u32)) {   
    let pts: Vec<&(T, T)> = points.iter().filter(|i| i.0 == i.0 && i.1 == i.1).collect();

    let v: Vec<f64> = table_indices_to_counts(&points, range, size).into_iter().flatten().map(|i| i as f64).collect();

    let mean_v: f64 = v.iter().sum::<f64>() / v.len() as f64;
    let max_v: f64 = max_always(&v, 0.);

    if mean_v <= 1. && max_v * ciel_div(pts.len(), 20) as f64 <= 2. {
        (dots_one_by_one(), (1, 1))
    } else if mean_v <= 1.5 || max_v * ciel_div(pts.len(), 10) as f64 <= 4. {
        (blocks_two_by_two(), (2, 2))
    } else {
        (dots_two_by_four(), (2, 4))
    }
}


pub(crate) fn bool_arr_plot_string_custom_charset(arr: &Vec<Vec<bool>>, range: (u32, u32), charset: (Vec<char>, (u32, u32))) -> String {
    // Dimensions of arr should be equal to (range.0, range.1)

    let chrs = charset.0;
    let chrsize = charset.1;
    let x_size = ciel_div(range.0, chrsize.0);
    let y_size = ciel_div(range.1, chrsize.1);

    // Valid binary representing charachter set
    assert_eq!(chrs.len() as u32, 1u32 << (chrsize.0 * chrsize.1));

    (0..y_size).into_par_iter().map(|j|
        (0..x_size).map(|i| {
            // arr[y..yn][x..xn] defines the subarray for the character at (i, j)
            let (x, y) = (chrsize.0 * i, chrsize.1 * j);
            let (xn, yn) = (chrsize.0 * (i + 1), chrsize.1 * (j + 1));
            
            chrs[
                // Determine the index of the charachter in chrs based on binary representation of points
                bin_to_u8(
                    // Transpose the subarray from (row, col) to (col, row), because charachters are stored in binary (col, row) order
                transpose_table(
                        // Padding the table to dimensions a multiple of the charset size
                    &padded_vec_to(
                            arr[y as usize..(yn as usize).clamp(0, arr.len())]
                            .iter()
                            .map(|row| padded_vec_to(
                                row[x as usize..(xn as usize).clamp(0, row.len())].to_vec(),
                                chrsize.0 as usize,
                                false)
                            )
                            .collect::<Vec<Vec<bool>>>(),

                            chrsize.1 as usize,
                            vec![false; chrsize.0 as usize],
                        )
                    )
                    // Flatten and extract into a single list of binary
                    .into_iter()
                    .flatten()
                    .map(|i| *i)
                    .collect::<Vec<bool>>()
                ) as usize
            ]
        }).collect::<String>()
    ).collect::<Vec<String>>()
    .join("\n")
}


/// Builder for a Scatter Plot
/// Set various options for plotting the points.
/// 
/// # Options
/// 
/// * `data` - Input points.
/// * `domain_and_range` - Domain and range over which to plot the region. Default is computed.
/// * `padding` - Proportion of domain and range to pad the plot with. Default is 0.1.
/// * `size` - Dimensions (in characters) of the outputted plot. Default is (60, 30).
/// * `title` - Optional title for the plot. Default is None.
/// * `axes` - Whether or not to display axes and axes labels. Default is true.
/// * `chars` - Charset to be used for plotting. Any set in `cgrustplot::helper::charset::subdiv_chars` works. Default is computed.
/// 
#[derive(Clone)]
pub struct ScatterPlotBuilder<'a, T: PartialOrd + Copy + ToPrimitive + std::fmt::Debug> {
    data: &'a Vec<(T, T)>,
    domain_and_range: Option<((f64, f64), (f64, f64))>,
    padding: Option<f64>,
    size: Option<(u32, u32)>,
    title: Option<&'a str>,
    axes: Option<bool>,
    chars: Option<(Vec<char>, (u32, u32))>,
}

/// Internal struct representing built values.
struct ScatterPlot<'a, T: PartialOrd + Copy + ToPrimitive + std::fmt::Debug> {
    data: &'a Vec<(T, T)>,
    domain_and_range: ((f64, f64), (f64, f64)),
    size: (u32, u32),
    title: Option<&'a str>,
    axes: bool,
    chars: (Vec<char>, (u32, u32)),
}

impl<'a, T: PartialOrd + Copy + ToPrimitive + std::fmt::Debug> ScatterPlotBuilder<'a, T> {
    /// Create an array plot from a table of data.
    fn from<'b: 'a>(data: &'b Vec<(T, T)>) -> Self {
        ScatterPlotBuilder {
            data: data,
            domain_and_range: None,
            padding: None,
            size: None,
            title: None,
            axes: None,
            chars: None,
        }
    }

    pub fn set_range(&mut self, range: ((f64, f64), (f64, f64))) -> &mut Self {
        self.domain_and_range = Some(range);
        self
    }

    pub fn set_padding(&mut self, padding: f64) -> &mut Self {
        self.padding = Some(padding);
        self
    }

    pub fn set_size(&mut self, size: (u32, u32)) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn set_title<'b: 'a>(&mut self, title: &'b str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut Self {
        self.axes = Some(do_axes);
        self
    }

    /// In addition to the chars, it also needs the dimensions of the charset.
    /// If it's named "something_x_by_y", then set the dimensions to be (x, y).
    /// 
    /// e.g. dots_two_by_four should be input as .set_chars((dots_two_by_four(), (2, 4)))
    pub fn set_chars(&mut self, chars: (Vec<char>, (u32, u32))) -> &mut Self {
        self.chars = Some(chars);
        self
    }

    fn build(&self) -> ScatterPlot<T> {
        // Padding must go before range, as default arg for range is based on padding
        let padding = self.padding.unwrap_or(0.1);
        let domain_and_range = self.domain_and_range.unwrap_or_else(|| padded_point_range(&self.data, padding));
        let size = self.size.unwrap_or((60, 30));
        let chars = self.chars.clone().unwrap_or_else(|| determine_char_set(&self.data, domain_and_range, size));  // Cloned value is moved into built variant, so the clone would be needed anyway
        
        ScatterPlot {
            data: self.data,
            domain_and_range,
            size: size,
            title: self.title,
            axes: self.axes.unwrap_or(true),
            chars: chars,
        }
    }

    /// Returns the plotted data as a string
    pub fn as_string(&self) -> String {
        self.build().as_string()
    }

    /// Displays the plotted data with println
    pub fn print(&self) {
        self.build().print();
    }

    /// Saves the text content of a plot to a file
    pub fn save(&self, path: &str) {
        save_to_file(&self.build().as_string(), path);
    }

    /// Returns a rendered text builder to render a string
    pub fn as_image(&self) -> RenderableTextBuilder {
        RenderableTextBuilder::from(self.build().as_string())
    }

    /// Displays the plot's data using pyplot
    pub fn pyplot(&self) {
        self.build().pyplot(None);
    }

    /// Saves the plot's data using pyplot
    pub fn save_pyplot(&self, path: &str) {
        self.build().pyplot(Some(path));
    }

    /// Returns the unformatted text content of a plot
    #[allow(dead_code)]
    pub(crate) fn plot(&self) -> String {
        self.build().plot()
    }

}

impl<'a, T: PartialOrd + Copy + ToPrimitive + std::fmt::Debug> ScatterPlot<'a, T> {
    fn plot(&self) -> String {
        let bool_arr: Vec<Vec<bool>> = table_indices_to_counts(&self.data, self.domain_and_range, (self.size.0 * self.chars.1.0, self.size.1 * self.chars.1.1))
            .into_par_iter()
            .map(|i| 
                i.into_iter()
                .map(|j| j != 0)
                .collect()
            ).collect();

        bool_arr_plot_string_custom_charset(&bool_arr, (self.size.0 * self.chars.1.0, self.size.1 * self.chars.1.1), self.chars.clone())
    }

    fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), self.domain_and_range, self.axes, self.title)
    }

    fn print(&self) {
        println!("{}", self.as_string());
    }

    fn pyplot(&self, path: Option<&str>) {
        let x_data: Vec<T> = self.data.iter().map(|p| p.0).collect();
        let y_data: Vec<T> = self.data.iter().map(|p| p.1).collect();
        let command = format!("scatter({x_data:?}, {y_data:?})");

        pyplot(&command, self.title, Some(self.axes), Some(self.domain_and_range), path);
    }
}

/// Displays a 2D region which satisfies a given predicate.
/// 
/// # Example
/// 
/// ```
/// use cgrustplot::plots::scatter_plot::scatter_plot;
/// 
/// let points = vec![(0., 0.), (1., 4.), (2., 8.), (1.2, 3.1)];
/// scatter_plot(&points).set_size((30, 10)).print();
/// 
/// // Standard Output:
/// //       │  ●                           
/// // 7.360 ┼                              
/// //       │                              
/// // 5.440 ┼                              
/// //       │              ●  ●            
/// // 3.520 ┼                              
/// //       │                              
/// // 1.600 ┼                              
/// //       │                              
/// // -0.32 ┼                           ●  
/// //       └┼──────┼──────┼──────┼────────
/// //        -0.160 0.4000 0.9600 1.5200   
/// ```
/// 
/// # Options
/// 
/// * `data` - Input points.
/// * `domain_and_range` - Domain and range over which to plot the region. Default is computed.
/// * `padding` - Proportion of domain and range to pad the plot with. Default is 0.1.
/// * `size` - Dimensions (in characters) of the outputted plot. Default is (60, 30).
/// * `title` - Optional title for the plot. Default is None.
/// * `axes` - Whether or not to display axes and axes labels. Default is true.
/// * `chars` - Charset to be used for plotting. Any set in `cgrustplot::helper::charset::subdiv_chars` works. Default is computed.
/// 
pub fn scatter_plot<'a, T: PartialOrd + Copy + ToPrimitive + std::fmt::Debug>(points: &'a Vec<(T, T)>) -> ScatterPlotBuilder<'a, T> {
    ScatterPlotBuilder::from(points)
}

/// Enumerates a list to generate 2D points.
/// 
/// # Example
/// ```
/// use cgrustplot::plots::scatter_plot::list_as_points;
/// 
/// let list = vec![8, 3, 4, 6];
/// assert_eq!(list_as_points(&list), vec![(0., 8.), (1., 3.), (2., 4.), (3., 6.)]);
/// ``````
pub fn list_as_points<T: ToPrimitive>(points: &Vec<T>) -> Vec<(f64, f64)> {
    points.iter().enumerate().map(|(i, p)| (i as f64, p.to_f64().unwrap())).collect()
}