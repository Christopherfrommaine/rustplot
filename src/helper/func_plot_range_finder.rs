use crate::helper::math::*;

// pad_range included in math

// der included in math

/// Performs iterative gradient descent on a function
fn grad_desc<F: Fn(f64) -> f64>(f: F, start: f64, steps: u32) -> f64 {
    let mut temp = 1e-8;
    
    let mut curr_x = start;
    let mut curr_f = f(start);

    let mut prev_x;
    let mut prev_f;
    
    for _i in 0..steps {
        let dfx = der_p(&f, curr_x);

        if dfx == 0. {break;}

        prev_x = curr_x;
        prev_f = curr_f;

        curr_x -= temp * dfx;
        curr_f = f(curr_x);

        if prev_f >= curr_f {
            temp *= 0.5;
            curr_x = prev_x;
        } else {
            temp *= 2.;
        }
    }

    curr_x
}

/// Finds local zeros of a function
fn grad_desc_to_zero<F: Fn(f64) -> f64>(f: F, start: f64, steps: u32) -> f64 {
    grad_desc(|x| f(x).powi(2), start, steps)
}

/// Finds local zeros of a fucntion's derivative
fn grad_desc_to_stat<F: Fn(f64) -> f64>(f: F, start: f64, steps: u32) -> f64 {
    return grad_desc_to_zero(|x| der_p(&f, x).powi(2), start, steps)
}

// Subdivide included in math

/// Removes duplicates, within some error value. List must be non-nan
fn distinct_floats(list: &Vec<f64>, epsilon: f64) -> Vec<f64> {
    if list.len() <= 1 || epsilon == 0. {return list.clone()}
    
    let mut prev = f64::INFINITY;
    let mut o: Vec<f64> = Vec::new();

    let mut sorted_list = list.clone();

    sorted_list.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater));

    sorted_list.iter().for_each(
        |i| {
            if (i - prev).abs() > epsilon {
                o.push(*i);
            }
            prev = *i;
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

/// Internal function for use of finding stationary points of a function with extensions and recursive accuracy improvement
fn stationary_points<F: Fn(f64) -> f64>(f: F, low: f64, high: f64, cuts: u32, extend_below: bool, extend_above: bool, depth: u32, max_point_count: usize, cuts_divisor: u32) -> (bool, Vec<f64>) {
    // Recursive base case
    if cuts <= 1 {return (false, vec![(low + high) * 0.5])}
    
    // Range of x and y values to search
    let x: Vec<f64> = subdivide(low, high, cuts + 1);
    let y: Vec<f64> = x.iter().map(|x| f(*x)).collect();

    // Slopes of y at each x
    let dy: Vec<f64> = (0..(x.len() - 1)).map(
            |i| (y[i + 1] - y[i]) / (x[i + 1] - x[i])
        ).collect();

    // Return values: (
    // are there an infinite number of stationary points?,
    // stationary points or first n stationary points if is_inf)
    let mut is_inf = false;
    let mut o: Vec<f64> = Vec::new();

    // Start at numbers closer to zero. Theres probably a much faster way to do this than sort (i.e. O(n) rather than O(nlog(n)))
    let mut rge: Vec<usize> = (0..(dy.len() - 1)).collect();
    rge.sort_unstable_by(|a, b| 
        x[*a]
        .abs()
        .partial_cmp(
            &x[*b].abs()
        )
        .unwrap_or(std::cmp::Ordering::Greater)
    );

    for i in rge {
        // If the derivative changes sign (e.g., goes from + to -. There must be a f'(x) == 0 in between, or a cusp)
        if dy[i] * dy[i + 1] <= 0. {
            // Recursively search the range at a higher detail, to improve prescision and find distinct close-together stationary points
            let mut sp = stationary_points(&f, x[i], x[i + 2], cuts / cuts_divisor, false, false, depth - 1, max_point_count, cuts_divisor);
            
            // Add points to output and check for infinity
            o.append(&mut sp.1);
            if sp.0 || o.len() > max_point_count {
                is_inf = true;
                break
            }
        }
    }

    // Choose distinct points at second-to-last depth
    if cuts / cuts_divisor <= 1 {  // Only at second-to-last depth
        let epsilon = (high - low) / (cuts - 1) as f64;  // Slightly larger than the inter-point interval
        o = distinct_floats(&o, epsilon);
    }
    
    // Conditions indicating infinite stationary points
    if depth <= 0 || o.len() >= max_point_count as usize || is_inf {
        return (true,
            sorted_least(o, true)
            .into_iter().map(
                |x| grad_desc_to_stat(&f, x, 100)
            ).collect()
        )
    }

    // Extend above if still finding new points
    if o.len() > 0 && extend_below {
        let mut sp = stationary_points(&f, low - (high - low), low, cuts / cuts_divisor, true, false, depth - 1, max_point_count, cuts_divisor);
        is_inf = is_inf || sp.0;
        o.append(&mut sp.1)
    }
    
    // Extend below if still finding new points
    if o.len() > 0 && extend_above {
        let mut sp = stationary_points(&f, high, high + (high - low), cuts / cuts_divisor, false, true, depth - 1, max_point_count, cuts_divisor);
        is_inf = is_inf || sp.0;
        o.append(&mut sp.1);
    }
    
    // return a grad desc the points (or a small number of points if there are an infinite number)
    (
        is_inf,
        sorted_least(o, is_inf)
        .into_iter()
        .map(
            |x| grad_desc_to_stat(&f, x, 100)
        )
        .collect()
    )
}

// Mostly a wrapper for stationary_points() with a couple improvements to it's output
fn stat_points<F: Fn(f64) -> f64>(f: F) -> (bool, Vec<f64>) {
    let sp = stationary_points(f, -100., 100., 1000, true, true, 20, 50, 5);

    // Group together nearby (likely equal) points
    let epsilon = if sp.1.len() > 0 {(max_always(&sp.1, 0.) - min_always(&sp.1, 0.)) / sp.1.len() as f64} else {0.};
    let points = distinct_floats(&sp.1, epsilon);

    // Bounds check (within 10^18)
    (sp.0, points.into_iter().filter(|p| p.abs() < 1e18).collect())
}

// Given a list of nums, return the range they occupy, plus some pading. ntor = nums to range
fn ntor(nums: &Vec<f64>, pad: f64) -> (f64, f64) {
    pad_range((min_always(nums, 0.), max_always(nums, 0.)), pad)
}

// Finds zeros of a function using gradient descent
fn zeros<F: Fn(f64) -> f64>(f: F) -> Vec<f64> {
    // Much coarser than stat_points(). Just looks for approximate solutions
    let zero_points: Vec<f64> = sorted_least(
        subdivide(-100., 100., 201),true
    )[..10]
    .into_iter().map(
        |x| grad_desc_to_zero(&f, *x, 100)
    ).collect();
    
    let threshold = 10. * min_always(&zero_points.iter().map(|x| f(*x)).collect(), 0.);
    zero_points.into_iter().filter(|x| f(*x) <= threshold).collect()
}

// Checks if a function f(x) == 0 for all x
fn is_only_zero<F: Fn(f64) -> f64>(f: F) -> bool {
    subdivide(-100., 100., 1001).into_iter().all(|x| f(x).abs() <= 0.001)
}

// Main function to determine the plot range of a given function
pub(crate) fn determine_range<F: Fn(f64) -> f64>(f: F) -> (f64, f64) {
    if is_only_zero(der(&f)) {return ntor(&vec![-1., 1.], 0.5)}  // constant

    if is_only_zero(der(der(&f))){return ntor(&[vec![0., 1.], zeros(f)].concat(), 0.5)} // affine

    // Finds a bunch of 'important' points (p) to a function. Makes a range which displays them all.
    let mut p: Vec<f64> = Vec::new();
    p.append(&mut stat_points(&f).1);  // Stationary Points
    p.append(&mut stat_points(der(&f)).1);  // Change-of-curvature points

    // If large points can be eliminated while still retainining a range
    let pn: Vec<f64> = p.iter().filter(|x| x.abs() < 1e8 as f64).map(|x| *x).collect();
    if pn.len() > 1 {p = pn;}

    // If there may be a too-small range over which points are checked
    if distinct_floats(&p, 0.1).len() <= 1 {
        p.append(&mut stat_points(|x| (der_p(&f, x).abs() - 1.).powi(2)).1);  // f'(x) == 1 points
    }

    p = distinct_floats(&p, 1e-4);  // Consolidate points

    if p.len() <= 1 {
        // If still not enough points, 
        p.append(&mut zeros(f))  // f(x) == 0 points
    }

    // Backup cases, if everything else doens't work
    if p.len() == 1 {return (p[0] - 10., p[0] + 10.)}
    if p.len() == 0 {return (-10., 10.)}

    // Usual return case, generating a range from the important numbers p
    return ntor(&p, 0.5)
}
