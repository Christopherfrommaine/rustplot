//! Helper file for automatically determining the domain that a function should be plotted on.
//! 
//! Almost entirely heuristics for what "looks good" and what points may be "interesting".
//! 
//! # Notes
//! 
//! This file is modeled off of a python file which was used for rapid development,
//! including the order of functions.
//! 
//! Random comments referring to "__ included in math" say that python functions
//! in the original file were already implemented in the math module.

use crate::helper::math::*;
use rayon::prelude::*;

// pad_range included in math

// der included in math

/// Performs iterative gradient descent on a function.
/// 
/// # Arguments
/// 
/// * `f` - A function of f64 to be minimized.
/// * `start` - An original guess for the minimum value.
/// * `steps` - The number of steps to be run through.
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::func_plot_domain::grad_desc;
/// let result = grad_desc(|x: f64| (x * x) + 10., 5., 1000);
/// assert!(result <= 0.000001);
/// ```
/// 
/// # Notes
/// 
/// Automatically determines temperature based on if the optimization is working quickly or slowly.
/// A higher `steps` value will hone in to a more specific value, but will not necesarially
/// find a different local minimum.
/// 
/// If descent is easy and a function has many local minima, it is recommended to
/// try using multiple different start points and chose the best.
/// 
/// f is reccomended to be continuous and differentiable, though it may work (barely) even if not.
/// Differentiation is done on a scale of `cgrustplot::helper::math::D`, which is currently 10^-6.
/// If your function relies on x-values with a greater prescision than this, consider re-scaling.
/// 
pub fn grad_desc<F: Fn(f64) -> f64>(f: F, start: f64, steps: u32) -> f64 {
    let mut temp = 1e-8;
    
    let mut curr_x = start;
    let mut curr_f = f(start);

    let mut prev_x;
    let mut prev_f;
    
    for _i in 0..steps {
        // derivative at the point x
        let dfx = der_p(&f, curr_x);

        // Early end in case of flat function
        if dfx == 0. {break;}
        
        // Don't perform grad desc if values explode
        if curr_f.abs() > 1e32 {curr_x = start; break;}

        prev_x = curr_x;
        prev_f = curr_f;

        curr_x -= temp * dfx;
        curr_f = f(curr_x);

        // Update temperature based on progress
        if prev_f <= curr_f {
            temp *= 0.5;
            curr_x = prev_x;
        } else {
            temp *= 2.;
        }
    }

    curr_x
}

/// Finds local zeros of a function using gradient descent
fn grad_desc_to_zero<F: Fn(f64) -> f64>(f: F, start: f64, steps: u32) -> f64 {
    grad_desc(|x| f(x).powi(2), start, steps)
}

/// Finds local zeros of a fucntion's derivative using gradient descent
fn grad_desc_to_stat<F: Fn(f64) -> f64>(f: F, start: f64, steps: u32) -> f64 {
    grad_desc_to_zero(|x| der_p(&f, x).powi(2), start, steps)
}

// Subdivide included in math

/// Finds distinct values in a list, within some minimum distance epsilon.
/// 
/// Returns the sorted list.
/// 
/// # Notes
/// 
/// Clones the list.
/// NaN will be sorted to the end.
/// 
fn distinct_floats(list: &Vec<f64>, epsilon: f64) -> Vec<f64> {
    if list.len() <= 1 || epsilon == 0. {return list.clone()}
    
    let mut prev = f64::INFINITY;
    let mut o: Vec<f64> = Vec::new();

    let mut sorted_list = list.clone();

    sorted_list.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater));

    sorted_list.into_iter().for_each(
        |i| {
            if (i - prev).abs() > epsilon {
                o.push(i);
            }
            prev = i;
        }
    );

    return o
}

/// Returns the n smallest magnitude values of li, or all of them if not flag
fn sorted_least(mut li: Vec<f64>, flag: bool) -> Vec<f64> {
    const N: usize = 5;

    li.sort_unstable_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap_or(std::cmp::Ordering::Greater));

    return li[..if flag {std::cmp::min(li.len(), N)} else {li.len()}].to_vec()
}

/// Finds stationary points of a function with extensions and accuracy improvement.
/// 
/// # Arguments
/// 
/// * `f` - The function to find stationary points of.
/// * `low` - The starting minimum value of the domain to test over.
/// * `high` - The starting maximum value of the domain to test over.
/// * `cuts` - The numebr of evenly-spaced points in the starting domain to test.
/// * `max_depth` - The maximum number of iterations of the function for accuracy and range.
/// * `max_point_count` - The maximum number of stationary points found before returning.
/// * `cuts_divisor` - The ratio of `cuts` between sucuessive iterations.
/// * `include-cusps` - Include points whose derivative don't converge to zero, but "act like" a stationary point.
/// 
/// # Notes
/// 
/// Uses a stack-based approach rather than recursion.
/// 
/// Somewhat paralellized.
/// 
fn stationary_points<F: Fn(f64) -> f64>(f: &F, low: f64, high: f64, cuts: u32, max_depth: u32, max_point_count: usize, cuts_divisor: u32, include_cusps: bool) -> (bool, Vec<f64>) {
    // the following property must hold for this function to work properly:
    // cuts < cuts_divisor.pow(max_depth)
    
    // stack is of the form [(low, high, depth), ... ]
    let mut stack: Vec<(f64, f64, u32)> = vec![(low, high, 0)];

    let mut o: Vec<f64> = Vec::new();
    let mut olen: usize = 0;

    let mut is_inf: bool = false;

    let mut epsilon: f64 = 0.;

    let mut extensions: u32 = 0;
    loop {
        extensions += 1;

        while stack.len() > 0 && !is_inf && o.len() < max_point_count {
            // Safe because of length check in while loop above
            let s = stack.pop().unwrap();

            // Lower, Upper, Depth, and Cuts
            let l = s.0;
            let u = s.1;
            let d = s.2;
            let c = cuts / cuts_divisor.pow(d);

            if c <= 1 {
                // Cusp Check
                if include_cusps | ((der_p(f, l) - der_p(f, u)).abs() < 1000. * (u - l)) {
                    // Update epsilon to smallest difference
                    if epsilon < u - l {epsilon = u - l}
                    
                    // Push final answer
                    o.push(grad_desc_to_stat(&f, (l + u) * 0.5, 100));
                }

                continue;
            }

            // X and Y values to be tested. len == c
            let x: Vec<f64> = subdivide(l, u, c);
            let y: Vec<f64> = x.iter().map(|i| f(*i)).collect();

            // Eliminate Nan
            let x: Vec<f64> = 
                x
                .into_par_iter()
                .enumerate()
                .filter(|(i, _z)| !y[*i].is_nan())
                .map(|(_i, z)| z)
                .collect();
            let y: Vec<f64> = 
                y
                .into_par_iter()
                .filter(|z| !z.is_nan())
                .collect();

            // len(dy) == c - 1 - number_of_nan
            let dy: Vec<f64> = (0..(x.len() - 1)).map(
                |i| (y[i + 1] - y[i]) / (x[i + 1] - x[i])
            ).collect();

            // Search closest-to-zero points first
            let mut rge: Vec<usize> = (0..(x.len() - 2)).collect();
            rge.sort_unstable_by(|a, b|
                (-x[*a].abs()).partial_cmp(&-x[*b].abs()).unwrap());

            rge
            .into_iter()
            .for_each(|i|
                if dy[i] * dy[i + 1] <= 0. {
                    stack.push((x[i], x[i + 2], d + 1));
                }
            );

            is_inf = is_inf || olen > max_point_count;
        }
        
        is_inf = is_inf || extensions > max_depth || o.len() >= max_point_count;
        
        
        if olen == o.len() {
            return (is_inf, sorted_least(distinct_floats(&o, epsilon), is_inf));
        }
        
        olen = o.len();

        // Extend
        stack.push((low - extensions as f64 * (high - low), high - extensions as f64 * (high - low), extensions));
        stack.push((low + extensions as f64 * (high - low), high + extensions as f64 * (high - low), extensions));

    }
}

/// Wrapper for stationary_points() with default values.
/// Also implements a couple cleanups and improvements to stationary_points()'s output
fn stat_points<F: Fn(f64) -> f64>(f: &F, cusps: bool) -> (bool, Vec<f64>) {
    let sp = stationary_points(f, -100., 100., 1001, 5, 50, 5, cusps);
    let is_inf = sp.0;
    let points = sp.1;

    if points.len() == 0 {return (is_inf, points)}

    let epsilon = (max_always(&points, 0.) - min_always(&points, 0.)) / points.len() as f64;
    let points = distinct_floats(&points, epsilon);

    (
        is_inf,
        points
        .into_par_iter()
        .filter(|p| p.abs() < 1e18)
        .collect()
    )
}

/// nums to range
/// Given a list of nums, this returns the range they occupy, plus some pading
fn ntor(nums: &Vec<f64>, pad: f64) -> (f64, f64) {
    pad_range((min_always(nums, 0.), max_always(nums, 0.)), pad)
}

/// Finds zeros of a function using gradient descent
/// 
/// Much coarser than stat_points(). Just looks for approximate solutions
fn zeros<F: Fn(f64) -> f64>(f: F) -> Vec<f64> {
    // Sort domain by closest to zero
    let mut rge = subdivide(-100., 100., 201);
    rge.sort_unstable_by(|a, b| (f(*a).abs()).partial_cmp(&f(*b).abs()).unwrap());
    
    let zero_points: Vec<f64> = rge.into_iter().map(|z| grad_desc_to_zero(&f, z, 100)).collect();

    let threshold: f64 = 10. * min_always(&zero_points.iter().map(|z| f(*z).abs()).collect(), 0.);

    zero_points.into_iter().filter(|z| f(*z).abs() <= threshold).collect()
}

/// Checks if a function f(x) == 0 for all x
/// 
/// # Notes
/// 
/// Checks 1000 points from (-100, 100).
/// 
/// Zero-values need not be exact; heuristics used for approximated zero values.
fn is_only_zero<F: Fn(f64) -> f64>(f: F) -> bool {
    let mut y: Vec<f64> = subdivide(-100., 100., 1001).into_iter().map(|x| f(x).abs()).collect();

    let all_below_threshold: bool = y.par_iter().all(|z| z <= &1e-2);

    y.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater));
    let most_below_strict_threshold: bool = y[0..800usize].into_par_iter().all(|z| z < &1e-6);

    all_below_threshold && most_below_strict_threshold
}

/// Determines the final plot domain of a given function.
/// 
/// Almost entirely based on heuristics for what "looks good"
/// and what values may be "interesting".
/// 
/// This includes:
/// * zero values
/// * stationary points
/// * change-of-curvature points
/// 
/// as well as special cases and handling for
/// * constant functions
/// * linear functions
/// * peicewise (and non-differentiable) function
/// * functions with infinite stationary points (e.g. sin)
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::func_plot_domain::determine_plot_domain;
/// 
/// // Parabola with vertex at x = 5
/// let result = determine_plot_domain(|x: f64| (x - 5.) * (x - 5.) + 8.);
/// 
/// assert!(result.0 < 5. && result.1 > 5.);
/// ```
pub fn determine_plot_domain<F: Fn(f64) -> f64>(f: F) -> (f64, f64) {
    if is_only_zero(der(&f)) {return ntor(&vec![-1., 1.], 0.5)}  // constant

    if is_only_zero(der(der(&f))){return ntor(&[vec![0., 1.], zeros(f)].concat(), 0.5)} // affine

    // Finds a bunch of 'important' points (p) to a function. Makes a range which displays them all.
    let mut p: Vec<f64> = Vec::new();
    p.append(&mut stat_points(&f, false).1);  // Stationary Points
    p.append(&mut stat_points(&der(&f), false).1);  // Change-of-curvature points

    // If large points can be eliminated while still retainining a domain
    let pn: Vec<f64> = p.par_iter().filter(|x| x.abs() < 1e8 as f64).map(|x| *x).collect();
    if pn.len() > 1 {p = pn;}

    // If there may be a too-small domain over which points are checked
    if distinct_floats(&p, 0.1).len() <= 1 {
        p.append(&mut stat_points(&|x| (der_p(&f, x).abs() - 1.).powi(2), false).1);  // f'(x) == 1 points
    }

    p = distinct_floats(&p, 1e-4);  // Consolidate points

    if p.len() <= 1 {
        // If still not enough points, 
        p.append(&mut zeros(&f))  // f(x) == 0 points
    }

    if p.len() <= 1 {
        // If *still* not enough points, allow cusps
        p.append(&mut stat_points(&f, true).1);  // Stationary Points
        p.append(&mut stat_points(&der(&f), true).1);  // Change-of-curvature points
    }

    // Backup cases, if everything else doens't work
    if p.len() == 1 {return (p[0] - 10., p[0] + 10.)}
    if p.len() == 0 {return (-10., 10.)}

    // Usual return case, generating a range from the important numbers p
    return ntor(&p, 0.5)
}
