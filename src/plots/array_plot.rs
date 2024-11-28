use std::collections::HashMap;
use bytemuck::Pod;

use crate::helper::math::{*, non_nan_type::*};
use crate::helper::arrays::{distinct_in_table_non_nan, bin_arr_bounded};
use crate::helper::charset::{gradient_chars::*, NULL_STR};

/// Determines which ascii shading charachter set to use based on the number of unique charachters.
fn choose_charachter_set(num_distinct: u32) -> Vec<String> {
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

/// Bins the values of a table (2D array).
/// 
/// Creates a number of bins, then returns the index of the bin into which each value in the table first fits.
fn bin_arr(arr: &Vec<Vec<f64>>, bins: u32) -> Vec<Vec<u32>> {
    bin_arr_bounded(arr, bins, (
        // min and max non-nan over the 2D array
        min_always(&(arr.iter().map(
            |i| min_always(i, 0.)
        ).collect::<Vec<f64>>()), 0.),
        
        max_always(&(arr.iter().map(
            |i| max_always(i, 0.)
        ).collect::<Vec<f64>>()), 0.),
    ))
}

/// Primary logic for an array plot.
/// 
/// Creates a mapping of elements to charachters, then replaces elements with charachters according to the mapping.
fn array_plot_string_custom_chars<T>(arr: &Vec<Vec<T>>, chrs: Vec<String>) -> String 
where
    T: PartialOrd + Copy + Pod,
{
    // di is distinct non-NaN integers in the table
    let mut di = distinct_in_table_non_nan(arr);
    di.sort_unstable();
    
    // Select di.len() unique (usually) charachters
    let chars: Vec<&str> = subdivide_round(0, chrs.len() as i32 - 1, di.len() as u32)
        .into_iter()
        .map(|i| chrs[i as usize].as_str())
        .collect::<Vec<&str>>();

    // Map from every integer to a corresponding char
    let charmap: HashMap<NonNanWrapper<T>, &str> = di.into_iter().zip(chars.into_iter()).collect();

    // Map each in table to corresponding char
    arr.into_iter().map(|i| {
        i.into_iter().map(|j| {
            // If non-nan, get from charmap, else null character
            if j == j {
                charmap.get(&NonNanWrapper::from(*j)).unwrap()
            } else {NULL_STR} // Only for NaN
        }).collect::<String>()
    }).collect::<Vec<String>>()
    .join("\n")
}

pub fn array_plot_string<T>(arr: &Vec<Vec<T>>) -> String
where
    T: PartialOrd + Copy + Pod,
{
    array_plot_string_custom_chars(arr, choose_charachter_set(distinct_in_table_non_nan(&arr).len() as u32))
}

pub fn array_plot<T>(arr: &Vec<Vec<T>>)
where
    T: PartialOrd + Copy + Pod,
{
    println!("{}", array_plot_string(arr));
}

pub fn density_plot_string(arr: &Vec<Vec<f64>>, bins: u32) -> String {
    array_plot_string(&bin_arr(arr, bins))
}

pub fn density_plot(arr: &Vec<Vec<f64>>, bins: u32) {
    println!("{}", density_plot_string(arr, bins));
}
