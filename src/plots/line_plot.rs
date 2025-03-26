//! # Line Plot
//! Displays a line graph of some given points.
//! 
//! # Functions
//! 
//! * `line_plot` - Generates a LinePlotBuilder from some data.
//! 

use crate::{
    helper::{
        math::{pad_range, max_always, min_always},
        axes::add_opt_axes_and_opt_titles,
        mat_plot_lib::pyplot,
        file::save_to_file,
        rendering::RenderableTextBuilder,
    },
    plots::function_plot::function_plot,
};

fn binary_search_closest(v: &Vec<f64>, el: f64) -> (usize, usize) {
    let mut l: usize = 0;
    let mut u: usize = v.len() - 1;
    let mut c: usize = (u + l) / 2;

    loop {
        if u  - l <= 1 {
            return (u, l);
        }

        if v[c] < el {
            l = c;
            c = (u + l) / 2;
        } else if v[c] > el {
            u = c;
            c = (u + l) / 2;
        } else {
            // if v[c] == el
            u = c;
            l = c;
        }
    }
}

/// Builder for an Line Plot
/// Set various options for plotting the data.
/// 
/// # Options
///  
/// * `data` - Input data of a list of points.
/// * `domain` - Specified domain to plot the data over. Default is computed.
/// * `range` - Specified range to display the data over. Default is computed.
/// * `domain_padding` - Proportion of the width of the domain to be padded with. Default is 0.1.
/// * `range_padding` - Proportion of the height of the range to be padded with. Default is 0.1.
/// * `size` - Dimensions (in characters) of the outputted plot. Default is (60, 10).
/// * `title` - Optional title for the plot. Default is None.
/// * `axes` - Whether or not to display axes and axes labels. Default is true.
///  
#[derive(Clone)]
pub struct LinePlotBuilder<'a> {
    data: &'a Vec<(f64, f64)>,
    domain: Option<(f64, f64)>,
    range: Option<(f64, f64)>,
    domain_padding: Option<f64>,
    range_padding: Option<f64>,
    size: Option<(u32, u32)>,
    title: Option<&'a str>,
    axes: Option<bool>,
}

/// Internal struct representing built values.
struct LinePlot<'a> {
    data: &'a Vec<(f64, f64)>,
    domain_and_range: ((f64, f64), (f64, f64)),
    size: (u32, u32),
    title: Option<&'a str>,
    axes: bool,
}

impl<'a> LinePlotBuilder<'a> {
    /// Create an array plot from a table of data.
    fn from<'b: 'a>(data: &'b Vec<(f64, f64)>) -> LinePlotBuilder<'a> {
        LinePlotBuilder {
            data,
            domain: None,
            range: None,
            domain_padding: None,
            range_padding: None,
            size: None,
            title: None,
            axes: None,
        }
    }

    pub fn set_domain(&mut self, domain: (f64, f64)) -> &mut Self {
        self.domain = Some(domain);
        self
    }

    pub fn set_range(&mut self, range: (f64, f64)) -> &mut Self {
        self.range = Some(range);
        self
    }

    pub fn set_domain_padding(&mut self, padding: f64) -> &mut Self {
        self.domain_padding = Some(padding);
        self
    }
    
    pub fn set_range_padding(&mut self, padding: f64) -> &mut Self {
        self.range_padding = Some(padding);
        self
    }

    pub fn set_size(&mut self, size: (u32, u32)) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn set_title<'b: 'a>(&mut self, title: &'b str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut Self {
        self.axes = Some(do_axes);
        self
    }

    fn build(&self) -> LinePlot {
        let domain = self.domain.unwrap_or_else(||
            pad_range(
                self.domain.unwrap_or_else(||(
                    min_always(&self.data.iter().map(|p| p.0).collect(), 0.),
                    max_always(&self.data.iter().map(|p| p.0).collect(), 0.),
                )),
                self.domain_padding.unwrap_or(0.1),
            )
        );

        let range = self.range.unwrap_or_else(||
            pad_range(
                self.range.unwrap_or_else(||(
                    min_always(&self.data.iter().map(|p| p.1).collect(), 0.),
                    max_always(&self.data.iter().map(|p| p.1).collect(), 0.),
                )),
                self.range_padding.unwrap_or(0.1),
            )
        );
        
        LinePlot {
            data: self.data,
            domain_and_range: (domain, range),
            size: self.size.unwrap_or((60, 10)),
            title: self.title,
            axes: self.axes.unwrap_or(true),
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

impl<'a> LinePlot<'a> {
    pub fn plot(&self) -> String {
        let mut d = self.data.clone();
        d.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Greater));

        let f = move |x: f64| {
            if d[0].0 > x {return d[0].1}
            if d[d.len() - 1].0 < x {return d[d.len() - 1].1}
            // if d[0].0 > x || x > d[d.len() - 1].0 {return f64::NAN;}

            let (i0, i1) = binary_search_closest(&d.iter().map(|i| i.0).collect(), x);

            if d[i1].0 == d[i0].0 {
                // If zero dist, return avg
                return (d[i0].1 + d[i1].1) * 0.5
            } else {
                //     (Change in y        / change in x) = slope * (x - x1)      + y1    
                return (d[i1].1 - d[i0].1) / (d[i1].0 - d[i0].0) * (x - d[i0].0) + d[i0].1
            };
        };
        
        let mut plot = function_plot(&f);

        plot
            .set_domain(self.domain_and_range.0)
            .set_range(self.domain_and_range.1)
            .set_domain_padding(0.)
            .set_range_padding(0.)
            .set_axes(self.axes)
            .set_size(self.size);

        if let Some(title) = self.title {
            plot.set_title(title);
        }

        plot.plot()
    }

    pub fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), self.domain_and_range, self.axes, self.title)
    }

    pub fn print(&self) {
        println!("{}", self.as_string());
    }

    fn pyplot(&self, path: Option<&str>) {
        let x_vals: Vec<f64> = self.data.iter().map(|p| p.0).collect();
        let y_vals: Vec<f64> = self.data.iter().map(|p| p.1).collect();

        let command = format!("plot({x_vals:?}, {y_vals:?})");
        pyplot(&command, self.title, Some(self.axes), Some(self.domain_and_range), path);
    }
}

/// Displays a line graph of some given points.
/// 
/// # Example
/// 
/// ```
/// use cgrustplot::plots::line_plot::line_plot;
/// 
/// let points = vec![(0., 0.), (1., 1.), (2., 4.)];
/// line_plot(&points).print();
/// 
/// // Standard Output:
/// //       │                                                    _―――――――
/// // 3.680 ┼                                                _――‾        
/// //       │                                            _――‾            
/// // 2.720 ┼                                        _――‾                
/// //       │                                    _――‾                    
/// // 1.760 ┼                                _――‾                        
/// //       │                         _―――――‾                            
/// // 0.800 ┼             _――――――――――‾                                   
/// //       │――――――――――――‾                                               
/// // -0.16 ┼                                                            
/// //       └┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼──────────
/// //        -0.180 0.1000 0.3800 0.6600 0.9400 1.2200 1.5000 1.7800     
/// ```
/// 
/// # Options
///  
/// * `data` - Input data of a list of points.
/// * `domain` - Specified domain to plot the data over. Default is computed.
/// * `range` - Specified range to display the data over. Default is computed.
/// * `domain_padding` - Proportion of the width of the domain to be padded with. Default is 0.1.
/// * `range_padding` - Proportion of the height of the range to be padded with. Default is 0.1.
/// * `size` - Dimensions (in characters) of the outputted plot. Default is (60, 10).
/// * `title` - Optional title for the plot. Default is None.
/// * `axes` - Whether or not to display axes and axes labels. Default is true.
///  
pub fn line_plot<'a>(data: &'a Vec<(f64, f64)>) -> LinePlotBuilder<'a> {
    LinePlotBuilder::from(data)
}