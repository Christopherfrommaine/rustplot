use std::collections::HashMap;
use bytemuck::Pod;

use crate::helper::math::{*, non_nan_type::*};
use crate::helper::arrays::{bin_arr_bounded, distinct_in_table, distinct_in_table_non_nan};
use crate::helper::charset::{gradient_chars::*, NULL_STR};

/// Determines which ascii shading charachter set to use based on the number of unique charachters.
/// Acts as a default argument for ArrayPlots
pub(crate) fn choose_charachter_set(num_distinct: u32) -> Vec<String> {
    if num_distinct <= binary_chars().len() as u32 {
        return binary_chars();
    } else if num_distinct <= shade_chars().len() as u32 {
        return shade_chars();
    } else if num_distinct <= ascii_chars().len() as u32 {
        return ascii_chars();
    } else {
        return ascii_chars_large();
    }
}

/// Builds elements of an array plot.
/// 
/// This struct allows the user to set various values of the plot, such as
/// title, axes, custom charachter sets, etc.
/// 
/// Internally then uses .build() to convert it's values from Option<T> to T,
/// and finally plots with .as_string() or .print() from those values.
pub struct ArrayPlotBuilder<T: PartialOrd + Copy + Pod> {
    data: Vec<Vec<T>>,
    title: Option<String>,
    axes: Option<bool>,
    chars: Option<Vec<String>>,
}

/// Internal struct representing built values.
pub(crate) struct ArrayPlot<T: PartialOrd + Copy + Pod> {
    data: Vec<Vec<T>>,
    title: Option<String>,
    axes: bool,
    chars: Vec<String>,
    string: Option<String>,
}

impl<T: PartialOrd + Copy + Pod> ArrayPlotBuilder<T> {
    /// Create an array plot from a table of data.
    fn from(data: &Vec<Vec<T>>) -> ArrayPlotBuilder<T> {
        ArrayPlotBuilder {
            data: data.clone(),
            title: None,
            axes: None,
            chars: None,
        }
    }

    pub fn set_title(&mut self, title: String) -> &mut ArrayPlotBuilder<T> {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut ArrayPlotBuilder<T> {
        self.axes = Some(do_axes);
        self
    }

    pub fn set_chars(&mut self, chars: Vec<String>) -> &mut ArrayPlotBuilder<T> {
        self.chars = Some(chars);
        self
    }

    fn build(self) -> ArrayPlot<T> {
        ArrayPlot {
            // chars must be set before data because chars borrows and data takes ownership
            chars: match self.chars {
                Some(o) => o,
                None => choose_charachter_set(
                    distinct_in_table_non_nan(&self.data)
                    .len() as u32
                )
            },
            data: self.data,
            title: self.title,
            axes: match self.axes {
                Some(o) => o,
                None => true
            },
            string: None,
        }
        
    }

    /// Returns the plotted data as a string
    pub fn as_string(self) -> String {
        self.build().as_string()
    }

    /// Displays the plotted data with println
    pub fn print(self) {
        self.build().print();
    }
}

impl<T: PartialOrd + Copy + Pod> ArrayPlot<T> {
    fn as_string(&self) -> String {
        // di is distinct non-NaN integers in the table
        let mut di = distinct_in_table_non_nan(&self.data);
        di.sort_unstable();
        
        // Select di.len() unique (usually) charachters
        let ref_chars: Vec<&str> = subdivide_round(0, self.chars.len() as i32 - 1, di.len() as u32)
            .into_iter()
            .map(|i| self.chars[i as usize].as_str())
            .collect::<Vec<&str>>();

        // Map from every integer to a corresponding char
        let charmap: HashMap<NonNanWrapper<T>, &str> = di.into_iter().zip(ref_chars.into_iter()).collect();

        // Map each in table to corresponding char
        self.data.iter().map(|i| {
            i.iter().map(|j| {
                // If non-nan, get from charmap, else null character
                if j == j {
                    charmap.get(&NonNanWrapper::from(*j)).unwrap()
                } else {NULL_STR} // Only for NaN
            }).collect::<String>()
        }).collect::<Vec<String>>()
        .join("\n")
    }

    fn print(&self) {
        println!("{}", self.as_string());
    }
}

pub fn array_plot<T: PartialOrd + Copy + Pod>(data: &Vec<Vec<T>>) -> ArrayPlotBuilder<T> {
    ArrayPlotBuilder::from(&data)
}

pub fn density_plot(arr: &Vec<Vec<f64>>, bins: u32) -> ArrayPlotBuilder<u32> {
    ArrayPlotBuilder::from(
        // Bins a table of f64
        &bin_arr_bounded(arr, bins, (
            // min and max non-nan over the 2D array
            min_always(&(arr.iter().map(
                |i| min_always(i, 0.)
            ).collect::<Vec<f64>>()), 0.),
            
            max_always(&(arr.iter().map(
                |i| max_always(i, 0.)
            ).collect::<Vec<f64>>()), 0.),
        ))
    )
}