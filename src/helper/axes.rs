use crate::helper::{
    charset::axes_chars,
    math::{subdivide, min_always, max_always},
    arrays::pad_table,
};

fn string_to_char_table(s: &String) -> Vec<Vec<char>> {
    s.split('\n').map(|line| line.chars().collect()).collect()
}

fn pad_str_right(s: String, chr: char, l: usize) -> String {
    let chrs: String = (0..(l - s.len())).map(|_i| chr).collect();
    let mut o= s;
    o.push_str(&chrs);
    o
}

fn pad_str_left(s: String, chr: char, l: usize) -> String {
    let mut chrs: String = (0..(l - s.len())).map(|_i| chr).collect();
    chrs.push_str(&s);
    chrs
}

fn generate_single_axis_label(rge: (f64, f64), num_labels: u32, num_len: usize, pad_right: bool) -> Vec<String> {
    // If any range labels are NAN
    let any_nan = rge.0.is_nan() || rge.1.is_nan();
    let any_inf = rge.0.is_infinite() || rge.1.is_infinite();
    if any_nan || any_inf {
        let lab = if any_nan {"NaN"} else {"Inf"};
        let lab = String::from(&lab[0..std::cmp::min(lab.len(), num_len)]);

        return (0..num_labels).map(|_i| lab.clone()).collect();
    }

    let label_values = subdivide(rge.0, rge.1, num_labels);

    // Number of values too big / too small to fit in decimal notation
    let mut num_too_large: u32 = 0;
    let mut num_too_small: u32 = 0;

    for val in &label_values {
        let mut reamining_num_len = num_len;

        // Account for Neg
        if *val < 0. {reamining_num_len -= 1}
        let abs_val = val.abs();

        // Size Checks
        if abs_val > 10u32.pow(reamining_num_len as u32) as f64 {
            // 10 ^ i gives smallest number with len(digits) == i + 1, all smaller nums must have len(digits) <= i
            num_too_large += 1
        }
        if abs_val < 10.0_f64.powi(-(reamining_num_len as i32 - 3)) as f64 {
            // 10 ^ -(i - 3) gives largest number that allows for 2 sig figs in decimals with i total charachters (including 0.)
            num_too_small += 1
        }
    }

    // > 1 too small nums because of zero and things like that
    if num_too_large > 0 {num_too_small = 0}
    let use_sci_not = num_too_large > 0 || num_too_small > 1;

    let mut labels: Vec<String> = Vec::new();
    if use_sci_not {
        for val in &label_values {

            // Adjust num_len
            let mut reamining_num_len = num_len;
            if *val < 0. {reamining_num_len -= 1} // Account for neg sign
            if num_too_small > 0 {reamining_num_len -= 1} // Account for neg sign in exp

            labels.push(format!("{:.precision$e}", val, precision = reamining_num_len.max(4) - 4));
        }
    } else {
        for val in &label_values {
            // Adjust num_len
            let mut reamining_num_len = num_len;
            if *val < 0. {reamining_num_len -= 1} // Account for neg sign

            if label_values.iter().all(|v| v - (*v as i32 as f64) < 1e-16) {
                reamining_num_len = 2;
            }

            // Technically the -2 shouldn't be here, but it makes labels usually shorter. We'll see if I should remove it
            labels.push(format!("{:.precision$}", val, precision = reamining_num_len - 2));

        }

    }

    let max_len = labels.iter().map(|lab| lab.len()).max().unwrap_or(0usize);
    if pad_right {
        labels = labels.into_iter().map(|lab| pad_str_right(lab, ' ', max_len)).collect();
    } else {
        labels = labels.into_iter().map(|lab| pad_str_left(lab, ' ', max_len)).collect();
    }

    let trailing_zeros: usize;
    if labels.iter().all(|l| l.contains(".")) {
        trailing_zeros = min_always(&labels.iter().map(|l| l.chars().rev().take_while(|&c| c == '0').count()).collect(), 0);
    } else {
        trailing_zeros = 0;
    }
    labels.into_iter().map(|l| l[..(l.len() - trailing_zeros)].to_string()).collect()
    
}

pub(crate) fn add_axes(s: &String, range: ((f64, f64), (f64, f64))) -> String {
    let tab = string_to_char_table(s);

    let tab_height = tab.len() as u32;
    let tab_width = if tab_height > 0 {tab[0].len() as u32} else {0};
    
    let x_spacing = (tab_width / 3).clamp(1, 7); // Spacing must result in at least 3 labels
    let x_num_labels = 1 + (tab_width - 1) / x_spacing;
    let x_num_length = (x_spacing - 2).min(6) as usize; // On the x axis, labels must not overlap
    let x_adjusted_range = (range.0.0, (range.0.1 - range.0.0) * (x_spacing * (x_num_labels - 1)) as f64 / (tab_width - 1) as f64 + range.0.0);
    let x_labels = generate_single_axis_label(x_adjusted_range, x_num_labels, x_num_length, true);
    // let x_label_length = max_always(&x_labels.iter().map(|l| l.len()).collect(), 0); // unused

    let y_spacing = (tab_height / 3).clamp(1, 5);
    let y_num_labels = 1 + (tab_height - 1) / y_spacing;
    let y_num_length = 6 as usize;
    let y_adjusted_range = (range.1.0, (range.1.1 - range.1.0) * (y_spacing * (y_num_labels - 1)) as f64 / (tab_height - 1) as f64 + range.1.0);
    let y_labels = generate_single_axis_label(y_adjusted_range, y_num_labels, y_num_length, false);
    let y_label_length = max_always(&y_labels.iter().map(|l| l.len()).collect(), 0);

    let mut o = pad_table(&tab, ' ', ((y_label_length as i32 + 1, 2 * x_spacing as i32 + 1), (0, 2)));
    let o_height = tab_height as usize + 2;
    let o_width = tab_width as usize + 2 * y_label_length + 3 as usize;

    // Add in the axes
    (y_label_length..(tab_width + 1) as usize + y_label_length).for_each(|i| o[o_height - 2][i] = axes_chars::HORIZONTAL);
    (0..(o_height - 2)).for_each(|i| o[i][y_label_length] = axes_chars::VERTICAL);
    o[o_height - 2][y_label_length] = axes_chars::CORNER;

    for i in 0..x_num_labels {
        o[o_height - 2][(i * x_spacing) as usize + y_label_length + 1] = axes_chars::CROSS;

        x_labels[i as usize]
        .chars()
        .enumerate()
        .for_each(|(j, c)|
            if (i * x_spacing) as usize + j + y_label_length + 1 < o_width {
                o[o_height - 1][(i * x_spacing) as usize + j + y_label_length + 1] = c
            }
        );
    }

    for i in 0..y_num_labels {
        o[o_height - (i * y_spacing) as usize - 3][y_label_length] = axes_chars::CROSS;

        y_labels[i as usize]
        .chars()
        .enumerate()
        .for_each(|(j, c)| 
            o[o_height - (i * y_spacing) as usize - 3][j] = c
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

pub(crate) fn add_title(s: &String, title: String) -> String {
    let mut o = title;
    o.push('\n');
    o.push_str(s);
    o
}

pub(crate) fn add_opt_axes_and_opt_titles(s: &String, range: ((f64, f64), (f64, f64)), include_axes: bool, title: Option<&str>) -> String {
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