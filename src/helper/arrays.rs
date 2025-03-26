//! Helper file for working with, modifying, and manipulating arrays, tables, and vectors

use std::collections::HashSet;
use crate::helper::math::{non_nan_type::*, subdivide};
use num::ToPrimitive;

/// Takes a list of table indexes and returns the counts for each cell of the table.
/// 
/// Bins float-values so that the total range fits within the output size.
///
/// # Arguments
///
/// * `points` - Indices into the table.
/// * `range` - The range of values which the points take.
/// * `size` - The desired size of the output table.
///
/// # Example
///
/// ```
/// use cgrustplot::helper::arrays::table_indices_to_counts;
/// // indices at (0, 0), (0, 0), and (1, 1)
/// let result = table_indices_to_counts(&vec![(0., 0.), (1., 1.), (0., 0.)], ((0., 1.), (0., 1.)), (2, 2));
/// assert_eq!(result, vec![[2, 0], [0, 1]]);
/// ```
/// 
/// # Notes
/// 
/// Nan-valued input indices are ignored.
/// 
pub fn table_indices_to_counts<T: ToPrimitive + PartialEq>(points: &Vec<(T, T)>, range: ((f64, f64), (f64, f64)), size: (u32, u32)) -> Vec<Vec<u32>> {
    let pts: Vec<&(T, T)> = points.iter().filter(|i| i.0 == i.0 && i.1 == i.1).collect();

    let ptsxf64: Vec<f64> = pts.iter().map(|i| i.0.to_f64().unwrap()).collect();
    let ptsyf64: Vec<f64> = pts.iter().map(|i| i.1.to_f64().unwrap()).collect();

    let ptsxbin: Vec<u32> = bin_vec_bounded(&ptsxf64, size.0, range.0);
    let ptsybin: Vec<u32> = bin_vec_bounded(&ptsyf64, size.1, range.1);

    let mut o: Vec<Vec<u32>> = vec![vec![0u32; size.0 as usize]; size.1 as usize];
    (0..pts.len()).for_each(|i| o[ptsybin[i] as usize][ptsxbin[i] as usize] += 1);
    
    o
}

/// Transposes a table (2D-array).
/// 
/// The output values reference the values of the original table.
///
/// # Example
///
/// ```
/// use cgrustplot::helper::arrays::transpose_table;
/// let binding = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
/// let result = transpose_table(&binding);
/// assert_eq!(result, vec![vec![&1, &4, &7], vec![&2, &5, &8], vec![&3, &6, &9]]);
/// ```
/// 
/// # Notes
/// Input array must be rectangular and must have a non-zero size.
/// 
pub fn transpose_table<T>(arr: &Vec<Vec<T>>) -> Vec<Vec<&T>> {
    // Rectangular Array
    assert!(arr.iter().all(|i| i.len() == arr[0].len()));

    (0..arr[0].len())
    .map(|j|
        (0..arr.len())
        .map(|i|
            &arr[i][j]
        ).collect()
    ).collect()
}

/// Finds distinct elements in a table (2D-array).
/// 
/// That is, if you have a table of elements, it will search through to 
/// create a list of the unique elements.
/// 
/// It wraps the output list elements in a NonNanWrapper, which represents
/// arbitrary floats whose values cannot be NaN
/// (and thus have nice properties like Ord, Hash, etc.)
/// 
/// # Example
///
/// ```text
/// let result = distinct_in_table_non_nan(&vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
/// assert_eq!(result, (1..10).map(|i| NonNanWrapper::from(i)).collect());
/// ```
/// 
/// # Notes
/// 
/// Nan-values are removed and ignored.
pub(crate) fn distinct_in_table_non_nan<T: PartialOrd + Copy>(arr: &Vec<Vec<T>>) -> Vec<NonNanWrapper<T>> {
    arr
    .iter()
    .flat_map(|i| {
        let mut o = i.clone();
        o.retain(|i| i == i);
        o.iter()
        .map(|j|
            NonNanWrapper::from(*j)
        ).collect::<Vec<NonNanWrapper<T>>>()
    }).collect::<HashSet<NonNanWrapper<T>>>()
    .into_iter()
    .collect()
}

/// Bins the values of a vec.
/// 
/// Creates a number of bins, then returns the index of the bin into which each value in the vec first fits.
/// Ignores NaN values.
pub(crate) fn bin_vec_bounded(v: &Vec<f64>, bins: u32, range: (f64, f64)) -> Vec<u32> {
    // Bounds for the bins
    let subdivisions = subdivide(range.0, range.1, (bins + 1) as u32);

    // Place each value into the index of the first bin it fits in
    v.into_iter().map(|j| {
        match (0..subdivisions.len() - 1).into_iter()
            .position(|i| subdivisions[i] <= *j && *j <= subdivisions[i + 1])
            {Some(val) => val as u32, None => 0} // 0 case not needed
    }).collect()
}
pub(crate) fn bin_arr_bounded(arr: &Vec<Vec<f64>>, bins: u32, range: (f64, f64)) -> Vec<Vec<u32>> {
    arr.into_iter().map(|row| bin_vec_bounded(row, bins, range)).collect()
}

/// Extends a vector with an element to a specified length.
/// 
/// i.e. it pads it in-place.
/// 
/// # Arguments
/// 
/// * `v` - The vector to be padded.
/// * `n` - The desired length.
/// * `el` - The element with which to pad the vector.
/// 
/// # Example
///
/// ```
/// use cgrustplot::helper::arrays::pad_vec_to;
/// let mut result = vec![1, 2, 3];
/// pad_vec_to(&mut result, 5, 0);
/// assert_eq!(result, vec![1, 2, 3, 0, 0]);
/// ```
/// 
pub fn pad_vec_to<T: Clone>(v: &mut Vec<T>, n: usize, el: T) {
    v.extend(vec![el; n - v.len()]);
}

/// Pads a vector with an element to a specified length.
/// 
/// Takes ownership of the vector and returns a new vector.
/// 
/// # Arguments
/// 
/// * `v` - The vector to be padded.
/// * `n` - The desired length.
/// * `el` - The element with which to pad the vector.
/// 
/// # Example
///
/// ```
/// use cgrustplot::helper::arrays::padded_vec_to;
/// let result = padded_vec_to(vec![1, 2, 3], 5, 0);
/// assert_eq!(result, vec![1, 2, 3, 0, 0]);
/// ```
/// 
/// # Notes
/// 
/// Internally just calls pad_vec_to, so the in-place version is slightly preffered
pub fn padded_vec_to<T: Clone>(v: Vec<T>, n: usize, el: T) -> Vec<T> {
    let mut u = v;
    pad_vec_to(&mut u, n, el);
    u
}

/// Pads a table with an element by some specified dimensions.
/// 
/// Clones the table while doing so, so may not be efficient.
/// 
/// # Arguments
/// 
/// * `tab` - The table to be padded.
/// * `el` - The element with which to pad the vector.
/// * `((left, right), (top, bottom))` - The amount to pad in each direction.
/// 
/// # Example
///
/// ```
/// use cgrustplot::helper::arrays::pad_table;
/// let result = pad_table(&vec![vec![1, 2], vec![3, 4]], 0, ((0, 2), (1, 1)));
/// assert_eq!(result, vec![vec![0, 0, 0, 0], vec![1, 2, 0, 0], vec![3, 4, 0, 0], vec![0, 0, 0, 0]]);
/// ```
/// 
/// # Notes
/// 
/// Unlike padded_vec, pad_table pads BY a dimension, not TO a specified dimension.
/// 
/// Input table must be rectangular, but *may* have a size of zero.
/// 
pub fn pad_table<T: Clone>(tab: &Vec<Vec<T>>, el: T, padding: ((i32, i32), (i32, i32))) -> Vec<Vec<T>> {
    let ((left, right), (top, bottom)) = padding;

    let height = tab.len() as i32;
    let width = if height > 0 {tab[0].len() as i32} else {0};

    let rectagular = tab.iter().all(|r| r.len() as i32 == width);
    assert!(rectagular);
    
    (0..(top + tab.len() as i32 + bottom)).map(|i|
        (0..(left + tab[0].len() as i32 + right)).map(|j|
            if i - top < 0 || j - left < 0 || i - top >= tab.len() as i32 || j - left >= tab[0].len() as i32 {
                el.clone()
            } else {
                tab[(i - top) as usize][(j - left) as usize].clone()
            }
        ).collect()
    ).collect()
}