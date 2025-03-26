//! Helper file for adding axes, axes labels, and titles to existing plot strings

use crate::helper::{
    charset::axes_chars,
    math::{min_always, max_always},
    arrays::pad_table,
};

/// Splits a single \n-seperated string into a table of charachters
fn string_to_char_table(s: &str) -> Vec<Vec<char>> {
    s.split('\n').map(|line| line.chars().collect()).collect()
}

/// Formats a list of numbers to be displayed with the specified number of digits.
/// 
/// All numbers will be displayed in a consistent format across the list.
/// 
/// Returns an option representing if a number is storable in such a length.
/// 
/// # Arguments
/// 
/// * `nums` - The numbers to be formatted.
/// * `max_len` - The maximum allowable length that the string of outputs will take.
/// 
/// # Examples
///
/// `10000` formatted with 5 digits will be `"10000"'
/// 
/// '10000' formatted with 4 digits will be `"1.E4"`
/// 
/// '10000' formatted with 3 digits will be `None`
/// 
/// # Notes
/// Uses decimal notation, integer notation, and scientific notation, and
/// preference is greedy in that order.
/// 
pub(crate) fn format_nums(nums: &Vec<f64>, max_len: usize) -> Option<Vec<String>> {
    let min = min_always(nums, 0.);
    let max = max_always(nums, 0.);

    // Decimal Form
    // The length of a decimal number num is num.log10().floor() + 1
    // Plus one more is a negative sign is needed
    if (max < 0. || max.log10().floor() + 1. <= max_len as f64)
        && (min > 0. || min.abs().log10().floor() + 2. <= max_len as f64) {

        // Decimal works!
        let mut o = Vec::new();
        for x in nums {
            let len: f64;
            if x == &0. {
                len = max_len as f64;
            } else if x < &0. {
                len = max_len as f64 - (x.abs().log10().floor() + 3.);
            } else {
                len = max_len as f64 - (x.log10().floor() + 2.);
            }
            let fmlen = if len < 0. || !len.is_finite() {max_len} else {len as usize};

            let fm = format!("{:.1$}", x, fmlen);

            let trimmed_fm = fm.chars().take(max_len).collect::<String>();

            o.push(trimmed_fm);
        }

        return Some(o);
    }

    // Integer Form
    // Similar first two checks as decimal
    // Must check that all indices are unique
    if (max < 0. || max.log10().floor() + 1. <= max_len as f64)
        && (min > 0. || min.abs().log10().floor() + 2. <= max_len as f64)
        && nums.iter().zip(nums.iter().skip(1)).all(|(l, r)| (*l as u32).to_string() != (*r as u32).to_string()) {
        
        // Integer works!
        return Some(nums.iter().map(|x| (x.round() as u32).to_string()).collect());
    }

    // Scientific Notation
    // Check that all numbers have few enoguh mantissa and exp digits
    let mut largest_mantissa = 0;
    if nums.iter().all(|x| {
        let digit_add = (x < &0.) as i32 + (x.abs() < 1.) as i32;
        let expo = 2 + x.log10().floor().log10().floor() as i32;
        largest_mantissa = std::cmp::min(largest_mantissa, max_len as i32 - expo - digit_add);
        expo + digit_add + 1 < max_len as i32
    }) {
        if largest_mantissa <= 0 {largest_mantissa = 1} // Something's gone wrong, but its okay...
        if largest_mantissa == 1 {largest_mantissa = 2} // Because of a -1 term in format

        return Some(nums.iter().map(|x| format!("{:.*E}", largest_mantissa as usize - 2, x)).collect());
    }

    return None; // More digits are needed to represent these numbers in any format.
}

/// Determine the numebr of ticks on the x axis
fn kf(n: f64, min_sep: f64, max_sep: f64, min_ticks: f64) -> f64 {
    if n <= min_ticks * min_sep {
        min_ticks
    } else {
        (n / min_sep) - ((n - min_ticks * min_sep) / max_sep)
    }
}

/// Determines the number of ticks on the y axis
fn kfy(n: f64, sep_amount: f64) -> f64 {
    if n < sep_amount {
        n
    } else {
        (n / sep_amount).ceil()
    }
}

/// Generates the numbers and labels for a single axis.
/// 
/// ll value is Some for the vertical axis, representing the spacing between numbers.
fn single_axes_labels(n: usize, range: (f64, f64), ll: Option<usize>) -> (usize, Vec<String>) {
    // number of ticks (k) and seperation amount (s)
    let mut k = if ll.is_none() {kf(n as f64, 4., 8., 2.)} else {kfy(n as f64, 2.)} as usize;
    let mut s = ((n - 1) / k) + 1 - if ll.is_none() {1} else {0};

    for _ in 0..s {
        let nums_c: Vec<f64> = (0..k).map(|i| i as f64 * s as f64 + 0.5).collect();
        let nums_u: Vec<f64> = nums_c.iter().map(|x| range.0 + x * (range.1 - range.0) / n as f64).collect();

        // uses ll or s depending on vertical vs horizontal
        let labs = format_nums(&nums_u, ll.unwrap_or(s - 1));

        if let Some(v) = labs {
            return (s, v)
        } else {
            s += 1;
            k = n / s;
        }
    }

    return (s, vec!["err".to_string()]);
}

/// Adds axes to an input string
pub(crate) fn add_axes(s: &str, range: ((f64, f64), (f64, f64))) -> String {
    let tab = string_to_char_table(s);

    let tab_height = tab.len();
    let tab_width = if tab_height > 0 {tab[0].len()} else {0};
    
    let (x_spacing, x_labels) = single_axes_labels(tab_width as usize, range.0, None);
    let x_num_labels = x_labels.len();

    let default_y_label_length = 5;
    let (y_label_sep, y_labels) = single_axes_labels(tab_height as usize, range.1, Some(default_y_label_length));
    let y_num_labels = y_labels.len();
    let y_label_len = y_labels.iter().map(|s| s.len()).max().unwrap();
    
    let mut o = pad_table(&tab, ' ', ((y_label_len as i32 + 2, x_spacing as i32), (0, 2)));
    let o_height = tab_height as usize + 2;
    let o_width = tab_width as usize + x_spacing as usize;

    // Add in the axes
    ((y_label_len + 1)..(tab_width + y_label_len + 2)).for_each(|i| o[o_height - 2][i] = axes_chars::HORIZONTAL); // X
    (0..(o_height - 2)).for_each(|i| o[i][y_label_len + 1] = axes_chars::VERTICAL); // Y
    o[o_height - 2][y_label_len + 1] = axes_chars::CORNER; // O

    for i in 0..x_num_labels {
        let x_pos = (i * x_spacing) as usize + y_label_len + 2;

        o[o_height - 2][x_pos] = axes_chars::CROSS;

        x_labels[i as usize]
        .chars()
        .enumerate()
        .for_each(|(j, c)|
            if (i * x_spacing) as usize + j + y_label_len + 1 < o_width {
                o[o_height - 1][x_pos + j] = c
            }
        );
    }

    for i in 0..y_num_labels {
        let y_pos = o_height - 3 - i * y_label_sep;

        o[y_pos][y_label_len + 1] = axes_chars::CROSS;

        y_labels[i as usize]
        .chars()
        .enumerate()
        .for_each(|(j, c)| 
            o[y_pos][j] = c
        );
    }

    let trailing_spaces = min_always(&o.iter().map(|r| r.iter().rev().take_while(|c: &&char| **c == ' ').count()).collect(), 0);

    o
    .into_iter()
    .map(|r|
        r[..r.len() - trailing_spaces]
        .into_iter().collect()
    ).collect::<Vec<String>>()
    .join("\n")
}

/// Adds a title to an input string
pub(crate) fn add_title(s: &String, title: String) -> String {
    let mut o = title;
    o.push('\n');
    o.push_str(s);
    o
}

/// Formats a plot to add a title and axes, depending on options.
/// 
/// # Arguments
/// 
/// * `s` - The input plot.
/// * `range` - The range of numbers over which the input is plotted.
/// * `include_axes` - Whether or not to add axes and axes labels to the plot
/// * `title` - An optional title to be added to the plot.
/// 
/// # Notes
/// 
/// Axes labels are generated to be as short as possible while
/// still being able to display every number on the plot. See 
/// `pub(crate) format_nums` for full documentation and
/// implementation of how labels are formatted.
/// 
pub fn add_opt_axes_and_opt_titles(s: &String, range: ((f64, f64), (f64, f64)), include_axes: bool, title: Option<&str>) -> String {
    let mut o = String::new();

    if include_axes {
        o.push_str(&add_axes(s, range));
    } else {
        o.push_str(&s);
    }

    match title {
        Some(t) => o = add_title(&o, t.to_string()),
        None => ()
    }

    o
}