const D: f64 = 1e-6;
const HALF_INV_D: f64 = 0.5 / D;  // Used for der function. Very slight optimization to have it precomputed

/// Minimum of a Vector with a default value
/// 
/// Returns the minimum value of a vector for all non-NaN values, or default if v.len() == 0.
pub fn min_always<T: PartialOrd + Copy>(v: &Vec<T>, default: T) -> T {
    match v.iter()
    .filter(|i| i == i) // Filter out NaN Values
    .min_by(|x, y| {
        // None case unused for floats and ints 
        match x.partial_cmp(y) {Some(ord) => ord, None => std::cmp::Ordering::Equal}
    }) {Some(val) => *val, None => default} // for empty iterator
}

/// Maximum of a Vector with a default value
/// 
/// Returns the maximum value of a vector for all non-NaN values, or default if v.len() == 0.
pub fn max_always<T: PartialOrd + Copy>(v: &Vec<T>, default: T) -> T {
    match v.iter()
    .filter(|i| i == i) // Filter out NaN Values
    .max_by(|x, y| {
        // None case unused for floats and ints 
        match x.partial_cmp(y) {Some(ord) => ord, None => std::cmp::Ordering::Equal}
    }) {Some(val) => *val, None => default} // for empty iterator
}

/// Subdivides the interval (low, high) inclusive into n equally-spaced points.
pub fn subdivide(low: f64, high: f64, n: u32) -> Vec<f64> {
    if n == 0 {return Vec::new()}

    let diff = (high - low) / (n - 1) as f64;

    (0..n).map(|i| low + (i as f64) * diff).collect()
}

/// Subdivides the interval (low, high) inclusive into n equally-spaced integers.
pub fn subdivide_round(low: i32, high: i32, n: u32) -> Vec<i32> {
    subdivide(low as f64, high as f64, n).into_iter().map(|i| i.round() as i32).collect()
}

/// Pads a range by a ratio of it's width
pub fn pad_range(bounds: (f64, f64), padding: f64) -> (f64, f64) {
    let dif = bounds.1 - bounds.0;

    (bounds.0 - padding * dif, bounds.1 + padding * dif)
}

/// Converts a boolean vector into an integer.
pub(crate) fn bin_to_u8(bin: Vec<bool>) -> u8 {
    assert!(bin.len() <= 8);
    bin.iter().enumerate().fold(0u8, |acc, (i, &b)| acc | ((b as u8) << i))
}

/// Integer cieling division
pub(crate) fn ciel_div<T: num::Integer + Copy>(a: T, b: T) -> T {
    (a + b.clone() - T::one()) / b
}

/// Takes the derivative of a function by centered finite difference method
pub fn der<F: Fn(f64) -> f64>(f: F) -> impl Fn(f64) -> f64 {
    move |x: f64| (f(x + D) - f(x - D)) * HALF_INV_D
}

/// Takes the derivative of a function by centered finite difference method at a single point
pub(crate) fn der_p<F: Fn(f64) -> f64>(f: F, x: f64) -> f64 {
    (f(x + D) - f(x - D)) * HALF_INV_D
}

/// Non-NAN numerical wrapper type
pub(crate) mod non_nan_type {
    use bytemuck::{bytes_of, Pod};
    use std::hash::{Hash, Hasher};

    #[derive(Debug)]
    pub(crate) struct NonNanWrapper<T: PartialOrd + Copy + Pod> {
        num: T,
    }

    impl<T: PartialOrd + Copy + Pod> NonNanWrapper<T> {
        #[allow(dead_code)] // It is used within unit tests
        pub(crate) fn value(&self) -> T {self.num}
    }

    impl<T: PartialOrd + Copy + Pod> From<T> for NonNanWrapper<T> {
        fn from(value: T) -> Self {
            assert!(value == value, "Value for NonNanWrapper is NaN");
            NonNanWrapper {num: value}
        }
    }

    impl<T: PartialOrd  + Copy + Pod> Clone for NonNanWrapper<T> {
        fn clone(&self) -> Self {*self}
        fn clone_from(&mut self, source: &Self) {*self = *source;}
    }
    impl<T: PartialOrd + Copy + Pod> Copy for NonNanWrapper<T> {}

    impl<T: PartialOrd + Copy + Pod> PartialEq for NonNanWrapper<T> {
        fn eq(&self, other: &Self) -> bool {
            self.num == other.num
        }
    }
    impl<T: PartialOrd + Copy + Pod> Eq for NonNanWrapper<T> {}

    impl<T: PartialOrd + Copy + Pod> PartialOrd for NonNanWrapper<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<T: PartialOrd + Copy + Pod> Ord for NonNanWrapper<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.num.partial_cmp(&other.num).unwrap()
        }
    }
    impl<T: PartialOrd + Copy + Pod> Hash for NonNanWrapper<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            bytes_of(&self.num).hash(state);
        }
    }
}

