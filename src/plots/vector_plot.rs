use std::{collections::HashMap, fmt::Debug};

use crate::helper::{
    arrays::{bin_arr_bounded, distinct_in_table_non_nan},
    charset::{gradient_chars::*, NULL_STR},
    axes::add_opt_axes_and_opt_titles,
    mat_plot_lib::pyplot,
    rendering::RenderableTextBuilder,
    file::save_to_file,
};


/// Builder for an Array Plot
/// Set various options for plotting the data.
/// 
/// # Options
/// 
/// * `data` - Input data representing the array.
/// * `title` - Optional title for the plot. Default is None.
/// * `axes` - Whether or not to display axes and axes labels. Default is true.
/// * `chars` - Charset to be used for plotting. Any set in `cgrustplot::helper::charset::gradient_chars` works. Default is computed.
/// 
#[derive(Clone)]
pub struct ArrayPlotBuilder<'a, T: PartialOrd + Copy> {
    data: &'a Vec<Vec<T>>,
    title: Option<&'a str>,
    axes: Option<bool>,
    chars: Option<Vec<String>>,
}

/// Internal struct representing built values.
struct ArrayPlot<'a, T: PartialOrd + Copy> {
    data: &'a Vec<Vec<T>>,
    title: Option<&'a str>,
    axes: bool,
    chars: Vec<String>,
}

impl<'a, T: PartialOrd + Copy + Debug> ArrayPlotBuilder<'a, T> {
    /// Create an array plot from a table of data.
    fn from(data: &Vec<Vec<T>>) -> ArrayPlotBuilder<T> {
        ArrayPlotBuilder {
            data: data,
            title: None,
            axes: None,
            chars: None,
        }
    }

    pub fn set_title<'b: 'a>(&mut self, title: &'b str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut Self {
        self.axes = Some(do_axes);
        self
    }

    pub fn set_chars(&mut self, chars: Vec<String>) -> &mut Self {
        self.chars = Some(chars);
        self
    }

    fn build(&self) -> ArrayPlot<T> {
        // chars could be a reference, but in case of default, self would need to be mutated

        ArrayPlot {
            data: self.data,
            title: self.title,
            axes: self.axes.unwrap_or(true),
            chars: self.chars.clone().unwrap_or_else(|| choose_character_set(distinct_in_table_non_nan(&self.data).len() as u32)),
        }
    }

    /// Returns the plotted data as a string
    pub fn as_string(&self) -> String {
        self.build().as_string()
    }

    /// Displays the plotted data with println
    pub fn print(&self) {
        self.build().print();
    }

    /// Saves the text content of a plot to a file
    pub fn save(&self, path: &str) {
        save_to_file(&self.build().as_string(), path);
    }

    /// Returns a rendered text builder to render a string
    pub fn as_image(&self) -> RenderableTextBuilder {
        RenderableTextBuilder::from(self.build().as_string())
    }

    /// Displays the plot's data using pyplot
    pub fn pyplot(&self) {
        self.build().pyplot(None);
    }

    /// Saves the plot's data using pyplot
    pub fn save_pyplot(&self, path: &str) {
        self.build().pyplot(Some(path));
    }

    /// Returns the unformatted text content of a plot
    #[allow(dead_code)]
    pub(crate) fn plot(&self) -> String {
        self.build().plot()
    }
}

impl<'a, T: PartialOrd + Copy + Debug> ArrayPlot<'a, T> {
    fn plot(&self) -> String {
        // di is distinct non-NaN integers in the table
        let mut di = distinct_in_table_non_nan(self.data);
        di.sort_unstable();
        
        // Select di.len() unique (usually) characters
        let ref_chars: Vec<&str> = subdivide_round(0, self.chars.len() as i32 - 1, di.len() as u32)
            .into_iter()
            .map(|i| self.chars[i as usize].as_str())
            .collect::<Vec<&str>>();

        // Map from every integer to a corresponding char
        let charmap: HashMap<NonNanWrapper<T>, &str> = di.into_iter().zip(ref_chars.into_iter()).collect();

        // Map each in table to corresponding char
        self.data.iter().map(|i| {
            i.iter().map(|j| {
                // If non-nan, get from charmap, else null character
                if j == j {
                    charmap.get(&NonNanWrapper::from(*j)).unwrap()
                } else {NULL_STR} // Only for NaN
            }).collect::<String>()
        }).collect::<Vec<String>>()
        .join("\n")
    }

    fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), ((0., self.data[0].len() as f64), (0., self.data.len() as f64)), self.axes, self.title)
    }

    fn print(&self) {
        println!("{}", self.as_string());
    }

    fn pyplot(&self, path: Option<&str>) {
        let command = format!("imshow({:?})", self.data);
        pyplot(&command, self.title, Some(self.axes), None, path);
    }
}

/// Displays a table of values with different shades.
/// 
/// # Examples
/// 
/// ## Example 1
/// 
/// ```
/// use cgrustplot::plots::array_plot::array_plot;
/// 
/// let data = vec![vec![0, 1, 2, 1, 0, 1, 2], vec![1, 2, 1, 0, 1, 2, 1], vec![2, 1, 0, 1, 2, 1, 0]];
/// array_plot(&data).print();
/// // Standard Output:
/// // 
/// // 2.500 ┼ ▒█▒ ▒█
/// //       │▒█▒ ▒█▒
/// // 0.500 ┼█▒ ▒█▒ 
/// //       └┼──┼───
/// //        0. 4   
/// ```
/// 
/// # Example 2
/// 
/// ```
/// use cgrustplot::plots::array_plot::*;
/// 
/// // Table of x.sin() * y.sin()
/// let data = bin_arr(&((0..20).map(|r| (0..30).map(|c| (0.5 * r as f64).sin() * (0.333 * c as f64).sin()).collect()).collect()), 8);
/// 
/// array_plot(&data).print();
/// 
/// // Standard Output:
/// //       │++++++++++++++++++++++++++++++
/// // 18.50 ┼++******++==------=++******++=
/// //       │+*%%@@@%*+=-.   .-=+*%%@@@%*+=
/// // 16.50 ┼+*%@@@@%*+=-     .-+*%@@@@%*+=
/// //       │+*%@@@@%*+=-.   ..-+*%@@@@%*+=
/// // 14.50 ┼++*%%%%**+=--...--=++*%%%%**+=
/// //       │++++++++++=========++++++++++=
/// // 12.50 ┼+==-----==++*****++===----===+
/// //       │+=-.. ..-=+*%%@%%*+=--.. ..-=+
/// // 10.50 ┼+-.    .-=+*%@@@@%*=-.    .-=+
/// //       │+-.    .-=+*%@@@@%*=-.    .-=+
/// // 8.500 ┼+=-.....-=+*%%%%%*+=--....--=+
/// //       │+===---===+++***+++====--====+
/// // 6.500 ┼++++++++++=========++++++++++=
/// //       │++*%%%%**+=--....-=++*%%%%**+=
/// // 4.500 ┼+*%@@@@%*+=-.    .-+*%@@@@%*+=
/// //       │+*%@@@@%*+=-.    .-+*%@@@@%*+=
/// // 2.500 ┼+**%@@%%*+=-..  .-=+*%%@@%%*+=
/// //       │++******++==-----==++******++=
/// // 0.500 ┼+=========+++++++++==========+
/// //       └┼──────┼──────┼──────┼────────
/// //        0.5000 7.5000 14.500 21.500   
/// 
/// ```
/// 
/// # Options
/// 
/// * `data` - Input data representing the array.
/// * `title` - Optional title for the plot. Default is None.
/// * `axes` - Whether or not to display axes and axes labels. Default is true.
/// * `chars` - Charset to be used for plotting. Any set in `cgrustplot::helper::charset::gradient_chars` works. Default is computed.
/// 
pub fn array_plot<T: PartialOrd + Copy + Debug>(data: &Vec<Vec<T>>) -> ArrayPlotBuilder<T> {
    ArrayPlotBuilder::from(&data)
}
