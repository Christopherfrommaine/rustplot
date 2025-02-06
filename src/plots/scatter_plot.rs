use num::ToPrimitive;

use crate::helper::{
    arrays::{padded_vec_to, table_indices_to_counts, transpose_table},
    axes_original::add_opt_axes_and_opt_titles,
    charset::subdiv_chars::*,
    mat_plot_lib::pyplot,
    math::{bin_to_u8, ciel_div, max_always, min_always, pad_range}
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
pub struct ScatterPlotBuilder<'a, T: PartialOrd + Copy + ToPrimitive + std::fmt::Debug> {
    data: &'a Vec<(T, T)>,
    range: Option<((f64, f64), (f64, f64))>,
    padding: Option<f64>,
    size: Option<(u32, u32)>,
    title: Option<&'a str>,
    axes: Option<bool>,
    chars: Option<(Vec<char>, (u32, u32))>,
}

/// Internal struct representing built values.
struct ScatterPlot<'a, T: PartialOrd + Copy + ToPrimitive + std::fmt::Debug> {
    data: &'a Vec<(T, T)>,
    range: ((f64, f64), (f64, f64)),
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
            range: None,
            padding: None,
            size: None,
            title: None,
            axes: None,
            chars: None,
        }
    }

    pub fn set_range(&mut self, range: ((f64, f64), (f64, f64))) -> &mut Self {
        self.range = Some(range);
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

    pub fn set_chars(&mut self, chars: (Vec<char>, (u32, u32))) -> &mut Self {
        self.chars = Some(chars);
        self
    }

    fn build(&mut self) -> ScatterPlot<T> {
        // Padding must go before range, as default arg for range is based on padding
        if self.padding.is_none() {
            self.set_padding(0.1);
        }

        if self.range.is_none() {
            self.set_range(padded_point_range(&self.data, self.padding.unwrap()));
        }
        if self.size.is_none() {
            self.set_size((30, 50));
        }
        if self.chars.is_none() {
            self.set_chars(determine_char_set(&self.data, self.range.unwrap(), self.size.unwrap()));
        }
        if self.axes.is_none() {
            self.set_axes(true);
        }
        
        ScatterPlot {
            data: self.data,
            range: self.range.unwrap(),
            size: self.size.unwrap(),
            title: self.title,
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

    pub fn plot(&mut self) -> String {
        self.build().plot()
    }

    pub fn pyplot(&mut self) {
        self.build().pyplot(None);
    }

    pub fn save_pyplot(&mut self, path: &str) {
        self.build().pyplot(Some(path));
    }

}

impl<'a, T: PartialOrd + Copy + ToPrimitive + std::fmt::Debug> ScatterPlot<'a, T> {
    fn plot(&self) -> String {
        let bool_arr: Vec<Vec<bool>> = table_indices_to_counts(&self.data, self.range, (self.size.0 * self.chars.1.0, self.size.1 * self.chars.1.1))
            .into_iter()
            .map(|i| 
                i.into_iter()
                .map(|j| j != 0)
                .collect()
            ).collect();

        bool_arr_plot_string_custom_charset(&bool_arr, (self.size.0 * self.chars.1.0, self.size.1 * self.chars.1.1), self.chars.clone())
    }

    fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), self.range, self.axes, self.title)
    }

    fn print(&self) {
        println!("{}", self.as_string());
    }

    fn pyplot(&self, path: Option<&str>) {
        let x_data: Vec<T> = self.data.iter().map(|p| p.0).collect();
        let y_data: Vec<T> = self.data.iter().map(|p| p.1).collect();
        let command = format!("scatter({x_data:?}, {y_data:?})");

        pyplot(&command, self.title, Some(self.axes), Some(self.range), path);
    }
}

pub fn scatter_plot<'a, T: PartialOrd + Copy + ToPrimitive + std::fmt::Debug>(points: &'a Vec<(T, T)>) -> ScatterPlotBuilder<'a, T> {
    ScatterPlotBuilder::from(points)
}

pub fn list_as_points<T: ToPrimitive>(points: &Vec<T>) -> Vec<(f64, f64)> {
    points.iter().enumerate().map(|(i, p)| (i as f64, p.to_f64().unwrap())).collect()
}