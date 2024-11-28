use num::{Num, ToPrimitive};

use crate::helper::arrays::{padded_vec_to, transpose_table, table_indices_to_counts};
use crate::helper::math::{bin_to_u8, ciel_div, max_always, min_always};
use crate::helper::charset::subdiv_chars::*;


fn padded_range<T: Num + PartialOrd + Copy + ToPrimitive>(points: &Vec<(T, T)>, padding: f64) -> ((f64, f64), (f64, f64)) {
    let min_x = match min_always(&points.iter().map(|i| i.0).collect(), T::zero()).to_f64() {Some(val) => val, None => 0.};
    let max_x = match max_always(&points.iter().map(|i| i.0).collect(), T::zero()).to_f64() {Some(val) => val, None => 0.};
    let dif_x = max_x - min_x;
    let min_y = match min_always(&points.iter().map(|i| i.1).collect(), T::zero()).to_f64() {Some(val) => val, None => 0.};
    let max_y = match max_always(&points.iter().map(|i| i.1).collect(), T::zero()).to_f64() {Some(val) => val, None => 0.};
    let dif_y = max_y - min_y;

    ((min_x - padding * dif_x, max_x + padding * dif_x), (min_y - padding * dif_y, max_y + padding * dif_y))
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

    println!("{arr:?}");

    (0..y_size).map(|j|
        (0..x_size).map(|i| {
            // arr[y..yn][x..xn] defines the subarray for the character at (i, j)
            let (x, y) = (chrsize.0 * i, chrsize.1 * j);
            let (xn, yn) = (chrsize.0 * (i + 1), chrsize.1 * (j + 1));

            println!("{x}-{xn} and {y}-{yn}");
            
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

pub fn scatter_plot_ranged_string<T: ToPrimitive + Copy + PartialEq>(points: &Vec<(T, T)>, range: ((f64, f64), (f64, f64)), size: (u32, u32)) -> String {
    let bool_arr: Vec<Vec<bool>> = table_indices_to_counts(&points, range, size)
        .into_iter()
        .map(|i| 
            i.into_iter()
            .map(|j| j != 0)
            .collect()
        ).collect();
    
    let charset = determine_char_set(&points, range, size);

    return bool_arr_plot_string_custom_charset(&bool_arr, size, charset)
}

pub fn scatter_plot_string<T>(points: &Vec<(T, T)>, size: (u32, u32)) -> String
where
    T: Num + PartialOrd + Copy + ToPrimitive
{
    scatter_plot_ranged_string(points, padded_range(points, 0.1), size)
}