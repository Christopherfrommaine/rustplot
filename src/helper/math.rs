const D: f64 = 1e-6;
const HALF_INV_D: f64 = 0.5 / D;  // Statically precomputed for der function

/// Finds the minimum value of a vector (or default if length is zero).
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::math::min_always;
/// let result = min_always(&vec![1., 2., 3., -4., f64::NAN], 0.);
/// assert_eq!(result, -4.);
/// ```
/// 
/// # Notes
/// 
/// Nan-valued elements are ignored.
pub fn min_always<T: PartialOrd + Copy>(v: &Vec<T>, default: T) -> T {
    match v.iter()
    .filter(|i| i == i) // Filter out NaN Values
    .min_by(|x, y| {
        // None case unused for floats and ints 
        match x.partial_cmp(y) {Some(ord) => ord, None => std::cmp::Ordering::Equal}
    }) {Some(val) => *val, None => default} // for empty iterator
}

/// Finds the maximum value of a vector (or default if length is zero).
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::math::max_always;
/// let result = max_always(&vec![1., 2., 3., -4., f64::NAN], 0.);
/// assert_eq!(result, 3.);
/// ```
/// 
/// # Notes
/// 
/// Nan-valued elements are ignored.
pub fn max_always<T: PartialOrd + Copy>(v: &Vec<T>, default: T) -> T {
    match v.iter()
    .filter(|i| i == i) // Filter out NaN Values
    .max_by(|x, y| {
        // None case unused for floats and ints 
        match x.partial_cmp(y) {Some(ord) => ord, None => std::cmp::Ordering::Equal}
    }) {Some(val) => *val, None => default} // for empty iterator
}

/// Subdivides an inclusive interval into n equally-spaced points.
/// 
/// # Arguments
/// 
/// * `low` - Minimum value of the interval.
/// * `high` - Maximum value of the interval.
/// * `n` - Number of output points
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::math::subdivide;
/// let result = subdivide(0., 5., 6);
/// assert_eq!(result, vec![0., 1., 2., 3., 4., 5.]);
/// ```
/// 
/// # Notes
/// 
/// Returns an empty vector if n is zero.
/// 
/// Returns a constant vector if low == high.
/// 
pub fn subdivide(low: f64, high: f64, n: u32) -> Vec<f64> {
    if n == 0 {return Vec::new()}

    let diff = (high - low) / (n - 1) as f64;

    (0..n).map(|i| low + (i as f64) * diff).collect()
}

/// Subdivides an inclusive interval into n equally-spaced integers.
/// 
/// Just a rounded version of subdivide().
/// 
/// # Arguments
/// 
/// * `low` - Minimum value of the interval.
/// * `high` - Maximum value of the interval.
/// * `n` - Number of output points
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::math::subdivide_round;
/// let result = subdivide_round(0, 100, 6);
/// assert_eq!(result, vec![0, 20, 40, 60, 80, 100]);
/// ```
/// 
/// # Notes
/// 
/// Returns an empty vector if n is zero.
/// Returns a constant vector if low == high
/// 
pub fn subdivide_round(low: i32, high: i32, n: u32) -> Vec<i32> {
    subdivide(low as f64, high as f64, n).into_iter().map(|i| i.round() as i32).collect()
}

/// Pads an interval by a proportion of it's width.
/// 
/// # Examples
/// ```
/// use cgrustplot::helper::math::pad_range;
/// let result = pad_range((0., 1.), 0.1);
/// assert_eq!(result, (-0.1, 1.1));
/// ```
/// 
/// ```
/// use cgrustplot::helper::math::pad_range;
/// let result = pad_range((-1., 1.), 0.1);
/// assert_eq!(result, (-1.2, 1.2));
/// ```
pub fn pad_range(bounds: (f64, f64), padding: f64) -> (f64, f64) {
    let dif = bounds.1 - bounds.0;

    (bounds.0 - padding * dif, bounds.1 + padding * dif)
}

/// Converts a vector of bits into a u8.
/// Vector's length must not exceed 8.
pub(crate) fn bin_to_u8(bin: Vec<bool>) -> u8 {
    assert!(bin.len() <= 8);
    bin.iter().enumerate().fold(0u8, |acc, (i, &b)| acc | ((b as u8) << i))
}

/// Optimized integer cieling division
/// 
/// Equivalent to `(a as f64 / b as f64).ciel() as T` but
/// by only using integer arithemtic.
pub(crate) fn ciel_div<T: num::Integer + Copy>(a: T, b: T) -> T {
    (a + b.clone() - T::one()) / b
}

/// Generates the derivative of a function with the centered finite difference method.
/// 
/// der(f)(x) = (f(x + D) - f(x - D)) / (2 * D)
/// 
/// # Example
/// ```
/// use cgrustplot::helper::math::der;
/// let f = |x: f64| x * x;  // f(x) = x^2
/// let d = der(f);          // f'(x) = 2 * x
/// let result = d(2.);      // f'(2) = 4
/// assert!((result - 4.).abs() < 1e-6);
/// ```
pub fn der<F: Fn(f64) -> f64>(f: F) -> impl Fn(f64) -> f64 {
    move |x: f64| (f(x + D) - f(x - D)) * HALF_INV_D
}

/// Generates the derivative of a function with the centered finite difference method.
/// 
/// der(f, x) = (f(x + D) - f(x - D)) / (2 * D)
/// 
/// # Example
/// ```
/// use cgrustplot::helper::math::der_p;
/// let f = |x: f64| x * x; // f(x) = x^2
/// let result = der_p(f, 2.); // f'(2) = 4
/// assert!((result - 4.).abs() < 1e-6);
/// ```
pub fn der_p<F: Fn(f64) -> f64>(f: F, x: f64) -> f64 {
    (f(x + D) - f(x - D)) * HALF_INV_D
}

/// Non-NAN numerical wrapper type
pub(crate) mod non_nan_type {
    use std::hash::{Hash, Hasher};

    #[derive(Debug)]
    pub(crate) struct NonNanWrapper<T: PartialOrd + Copy> {
        num: T,
    }

    impl<T: PartialOrd + Copy> NonNanWrapper<T> {
        #[allow(dead_code)] // It is used within unit tests
        pub(crate) fn value(&self) -> T {self.num}
    }

    impl<T: PartialOrd + Copy> From<T> for NonNanWrapper<T> {
        fn from(value: T) -> Self {
            assert!(value == value, "Value for NonNanWrapper is NaN");
            NonNanWrapper {num: value}
        }
    }

    impl<T: PartialOrd  + Copy> Clone for NonNanWrapper<T> {
        fn clone(&self) -> Self {*self}
        fn clone_from(&mut self, source: &Self) {*self = *source;}
    }
    impl<T: PartialOrd + Copy> Copy for NonNanWrapper<T> {}

    impl<T: PartialOrd + Copy> PartialEq for NonNanWrapper<T> {
        fn eq(&self, other: &Self) -> bool {
            self.num == other.num
        }
    }
    impl<T: PartialOrd + Copy> Eq for NonNanWrapper<T> {}

    impl<T: PartialOrd + Copy> PartialOrd for NonNanWrapper<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<T: PartialOrd + Copy> Ord for NonNanWrapper<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.num.partial_cmp(&other.num).unwrap()
        }
    }
    impl<T: PartialOrd + Copy> Hash for NonNanWrapper<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Get the underlying bytes of self and use those for hashing.
            // Because T: Copy, a hash can be generated from the byte value,
            // so it doesn't cause issues with references.
            unsafe { 
                std::slice::from_raw_parts(
                    &self.num as *const T as *const u8, 
                    std::mem::size_of::<T>()
                )
            }.hash(state);
        }
    }
}

