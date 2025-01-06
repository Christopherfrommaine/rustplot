use std::collections::HashSet;
use bytemuck::Pod;
use crate::helper::math::{non_nan_type::*, subdivide};
use num::ToPrimitive;


/// Takes [(0, 0), (1, 1), (0, 0)] to [[2, 0], [0, 1]].
/// Output.dims == size, and points are binned by range
/// NaN is ignored
pub(crate) fn table_indices_to_counts<T: ToPrimitive + PartialEq>(points: &Vec<(T, T)>, range: ((f64, f64), (f64, f64)), size: (u32, u32)) -> Vec<Vec<u32>> {
    let pts: Vec<&(T, T)> = points.iter().filter(|i| i.0 == i.0 && i.1 == i.1).collect();

    let ptsxf64: Vec<f64> = pts.iter().map(|i| i.0.to_f64().unwrap()).collect();
    let ptsyf64: Vec<f64> = pts.iter().map(|i| i.1.to_f64().unwrap()).collect();

    let ptsxbin: Vec<u32> = bin_vec_bounded(&ptsxf64, size.0, range.0);
    let ptsybin: Vec<u32> = bin_vec_bounded(&ptsyf64, size.1, range.1);

    let mut o: Vec<Vec<u32>> = vec![vec![0u32; size.0 as usize]; size.1 as usize];
    (0..pts.len()).for_each(|i| o[ptsybin[i] as usize][ptsxbin[i] as usize] += 1);
    
    o
}

/// Returns a transposed 2D vector referencing the values of the input
pub(crate) fn transpose_table<T>(arr: &Vec<Vec<T>>) -> Vec<Vec<&T>> {
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

/// Returns a vector of the distincts element in a table (2D vector)
#[allow(dead_code)] // may or may not be used later. Useful either way
pub(crate) fn distinct_in_table<T: Eq + std::hash::Hash + Copy>(arr: &Vec<Vec<T>>) -> Vec<T> {
    arr.iter()
    .flat_map(|i| 
        i.iter()
        .map(|j| *j))
    .collect::<HashSet<T>>()
    .into_iter()
    .collect()
}

/// Returns a vector of the distincts element in a table (2D vector) for non-hashable number types, removing any NaN values.
pub(crate) fn distinct_in_table_non_nan<T: PartialOrd + Copy + Pod>(arr: &Vec<Vec<T>>) -> Vec<NonNanWrapper<T>> {
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

pub(crate) fn pad_vec_to<T: Clone>(v: &mut Vec<T>, n: usize, el: T) {
    v.extend(vec![el; n - v.len()]);
}

pub(crate) fn padded_vec_to<T: Clone>(v: Vec<T>, n: usize, el: T) -> Vec<T> {
    let mut u = v;
    pad_vec_to(&mut u, n, el);
    u
}

pub(crate) fn pad_table<T: Clone>(tab: &Vec<Vec<T>>, el: T, padding: ((i32, i32), (i32, i32))) -> Vec<Vec<T>> {
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