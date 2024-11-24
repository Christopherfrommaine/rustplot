use std::collections::{HashMap, HashSet};

fn subdivide(low: f64, high: f64, n: u32) -> Vec<f64> {
    let diff: f64 = (high - low) / ((n - 1) as f64);

    (0..n).map(|i| low + (i as f64) * diff).collect()
}

fn subdivide_round(low: i32, high: i32, n: u32) -> Vec<i32> {
    subdivide(low as f64, high as f64, n).into_iter().map(|i| i.round() as i32).collect()
}

fn min_always<T: PartialOrd + Copy>(v: &Vec<T>, default: T) -> T {
    match v.into_iter()
    .min_by(|x, y| {
        match x.partial_cmp(y) {Some(ord) => ord, None => std::cmp::Ordering::Equal}
    }) {Some(val) => *val, None => default}
}

fn max_always<T: PartialOrd + Copy>(v: &Vec<T>, default: T) -> T {
    match v.into_iter()
    .max_by(|x, y| {
        match x.partial_cmp(y) {Some(ord) => ord, None => std::cmp::Ordering::Equal}
    }) {Some(val) => *val, None => default}
}
 
fn distinct_in_table<T: Eq + std::hash::Hash + Copy>(arr: &Vec<Vec<T>>) -> Vec<T> {
    let mut set: HashSet<T> = HashSet::new();

    arr.into_iter().for_each(|i| {
        i.into_iter().for_each(|j| {set.insert(*j);})
    });

    set.into_iter().collect()
}

fn array_plot_string_custom_chars<T>(arr: &Vec<Vec<T>>, chrs: Vec<&str>) -> String 
where
    T: Ord + std::hash::Hash + Copy,
{
    let mut di: Vec<T> = distinct_in_table(&arr); // Distinct Integers
    di.sort();
    
    // Select di.len() unique (usually) charachters
    let chars: Vec<&str> = subdivide_round(0, chrs.len() as i32 - 1, di.len() as u32)
        .into_iter()
        .map(|i| chrs[i as usize])
        .collect::<Vec<&str>>();

    // Map from every integer to a corresponding char
    let charmap: HashMap<T, &str> = di.into_iter().zip(chars.into_iter()).collect();

    // Map each in table to corresponding char
    arr.into_iter().map(|i| {
        i.into_iter().map(|j| {
            if let Some(&res) = charmap.get(&j) {
                res
            } else {chrs[0usize]} // Should not be ever needed
        }).collect::<String>()
    }).collect::<Vec<String>>()
    .join("\n")
}

pub fn array_plot_string<T>(arr: &Vec<Vec<T>>) -> String
where
    T: Ord + std::hash::Hash + Copy,
{
    // Vec<&str> instead of Vec<char> for future ideas
    // e.g. using ANSI escape codes for color / bold if the terminal supports it
    // Charachter sets largely based on https://paulbourke.net/dataformats/asciiart/
    let binary_chars = vec![" ", "█"];
    let ascii_chars = vec![" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"];
    let ascii_chars_large = vec![" ", ".", "'", "`", "^", "\"", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_", "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/", "t", "f", "j", "r", "x", "n", "u", "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p", "d", "b", "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "@", "$"];
    
    // Number of distinct integers (i.e. distinct integers list length. Naming is unrelated to the pickle variety)
    let dill: u32 = distinct_in_table(&arr).len() as u32;

    let chrs: Vec<&str>;
    
    if dill <= binary_chars.len() as u32 {
        chrs = binary_chars;
    } else if dill <= ascii_chars.len() as u32 {
        chrs = ascii_chars;
    } else {
        chrs = ascii_chars_large;
    }

    array_plot_string_custom_chars(arr, chrs)
}

pub fn array_plot<T>(arr: &Vec<Vec<T>>)
where
    T: Ord + std::hash::Hash + Copy,
{
    println!("{}", array_plot_string(arr));
}

pub fn density_plot_string(arr: &Vec<Vec<f64>>, bins: u32) -> String {
    let min: f64 = min_always(&(arr.iter()
        .map(|i| min_always(i, 0.)).collect::<Vec<f64>>()), 0.);
    let max: f64 = max_always(&(arr.iter()
        .map(|i| max_always(i, 0.)).collect::<Vec<f64>>()), 0.);
    
    // Bounds for the bins
    let subdivisions = subdivide(min, max, (bins + 1) as u32);
    
    let binned_arr: Vec<Vec<u32>> = arr.into_iter().map(|i| {
        i.into_iter().map(|j| {
            for i in 0..subdivisions.len() - 1 {
                if subdivisions[i] <= *j && *j <= subdivisions[i + 1] {return i as u32;}
            }
            return 0; // Shouldn't ever happen
        }).collect()
    }).collect();
    
    array_plot_string(&binned_arr)
}

pub fn density_plot(arr: &Vec<Vec<f64>>, bins: u32) {
    println!("{}", density_plot_string(arr, bins));
}
