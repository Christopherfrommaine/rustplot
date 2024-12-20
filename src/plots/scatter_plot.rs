use num::ToPrimitive;

use crate::helper::{
    arrays::{padded_vec_to, transpose_table, table_indices_to_counts},
    math::{bin_to_u8, ciel_div, max_always, min_always, pad_range},
    charset::subdiv_chars::*,
    axes::add_opt_axes_and_opt_titles,
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
    } else if mean_v <= 2. || max_v * ciel_div(pts.len(), 20) as f64 <= 4. {
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

    (0..y_size).map(|j|
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


/// Builds elements of a scatter plot.
/// 
/// This struct allows the user to set various values of the plot, such as
/// title, axes, custom charachter sets, etc.
/// 
/// Internally then uses .build() to convert it's values from Option<T> to T,
/// and finally plots with .as_string() or .print() from those values.
pub struct ScatterPlotBuilder<T: PartialOrd + Copy + ToPrimitive> {
    data: Vec<(T, T)>,
    range: Option<((f64, f64), (f64, f64))>,
    padding: Option<f64>,
    size: Option<(u32, u32)>,
    title: Option<String>,
    axes: Option<bool>,
    chars: Option<(Vec<char>, (u32, u32))>,
}

/// Internal struct representing built values.
pub(crate) struct ScatterPlot<T: PartialOrd + Copy + ToPrimitive> {
    data: Vec<(T, T)>,
    range: ((f64, f64), (f64, f64)),
    size: (u32, u32),
    title: Option<String>,
    axes: bool,
    chars: (Vec<char>, (u32, u32)),
}

impl<T: PartialOrd + Copy + ToPrimitive> ScatterPlotBuilder<T> {
    /// Create an array plot from a table of data.
    fn from(data: &Vec<(T, T)>) -> ScatterPlotBuilder<T> {
        ScatterPlotBuilder {
            data: data.clone(),
            range: None,
            padding: None,
            size: None,
            title: None,
            axes: None,
            chars: None,
        }
    }

    pub fn set_range(&mut self, range: ((f64, f64), (f64, f64))) -> &mut ScatterPlotBuilder<T> {
        self.range = Some(range);
        self
    }

    pub fn set_padding(&mut self, padding: f64) -> &mut ScatterPlotBuilder<T> {
        self.padding = Some(padding);
        self
    }

    pub fn set_size(&mut self, size: (u32, u32)) -> &mut ScatterPlotBuilder<T> {
        self.size = Some(size);
        self
    }

    pub fn set_title(&mut self, title: String) -> &mut ScatterPlotBuilder<T> {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut ScatterPlotBuilder<T> {
        self.axes = Some(do_axes);
        self
    }

    pub fn set_chars(&mut self, chars: (Vec<char>, (u32, u32))) -> &mut ScatterPlotBuilder<T> {
        self.chars = Some(chars);
        self
    }

    fn build(&mut self) -> ScatterPlot<T> {
        // Padding must go before range, as default arg for range is based on padding
        self.set_padding(
            match self.padding {
                Some(o) => o,
                None => 0.1,
            }
        );
        self.set_range(
            match self.range {
                Some(o) => o,
                None => padded_point_range(&self.data, self.padding.unwrap())
            }
        );
        self.set_size(
            match self.size {
                Some(o) => o,
                None => (30, 50),
            }
        );
        self.set_chars(
            match &self.chars {
                Some(o) => o.clone(),
                None => determine_char_set(&self.data, self.range.unwrap(), self.size.unwrap())
            }
        );
        self.set_axes(
            match self.axes {
                Some(o) => o,
                None => true,
            }
        );
        
        ScatterPlot {
            data: self.data.clone(),
            range: self.range.unwrap(),
            size: self.size.unwrap(),
            title: self.title.clone(),
            axes: self.axes.unwrap(),
            chars: self.chars.clone().unwrap(),
        }
    }

    /// Returns the plotted data as a string
    pub fn as_string(&mut self) -> String {
        self.build().as_string()
    }

    /// Displays the plotted data with println
    pub fn print(&mut self) {
        self.build().print();
    }
}

impl<T: PartialOrd + Copy + ToPrimitive> ScatterPlot<T> {
    fn plot(&self) -> String {
        let bool_arr: Vec<Vec<bool>> = table_indices_to_counts(&self.data, self.range, self.size)
            .into_iter()
            .map(|i| 
                i.into_iter()
                .map(|j| j != 0)
                .collect()
            ).collect();

        bool_arr_plot_string_custom_charset(&bool_arr, self.size, self.chars.clone())
    }

    fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), self.range, self.axes, &self.title)
    }

    fn print(&self) {
        println!("{}", self.as_string());
    }
}

pub fn scatter_plot<T: PartialOrd + Copy + ToPrimitive>(points: &Vec<(T, T)>) -> ScatterPlotBuilder<T> {
    ScatterPlotBuilder::from(points)
}