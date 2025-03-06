use num::{FromPrimitive, ToPrimitive};

use crate::helper::{
    axes::add_opt_axes_and_opt_titles,
    charset::{line_chars::*, NULL_CHR},
    func_plot_domain::determine_plot_domain,
    mat_plot_lib::pyplot,
    math::{max_always, min_always, pad_range, subdivide},
    file::save_to_file,
    rendering::RenderableTextBuilder,
};


/// Builds elements of a function plot.
/// 
/// This struct allows the user to set various values of the plot, such as
/// title, axes, custom charachter sets, etc.
/// 
/// Internally then uses .build() to convert it's values from Option<T> to T,
/// and finally plots with .as_string() or .print() from those values.
pub struct FuncPlotBuilder<'a> {
    func: Box<&'a dyn Fn(f64) -> f64>,
    domain: Option<(f64, f64)>,
    range: Option<(f64, f64)>,
    domain_padding: Option<f64>,
    range_padding: Option<f64>,
    size: Option<(u32, u32)>,
    title: Option<&'a str>,
    axes: Option<bool>,
    precomputed: Option<Vec<(f64, f64)>>,
}

/// Internal struct representing built values.
struct FuncPlot<'a> {
    func: Box<&'a dyn Fn(f64) -> f64>,
    domain_and_range: ((f64, f64), (f64, f64)),
    size: (u32, u32),
    title: Option<&'a str>,
    axes: bool,
    precomputed: &'a Option<Vec<(f64, f64)>>
}

impl<'a> FuncPlotBuilder<'a> {
    /// Create an array plot from a table of data.
    fn from<'b: 'a>(func: &'b impl Fn(f64) -> f64) -> Self {
        FuncPlotBuilder {
            func: Box::new(func),
            domain: None,
            range: None,
            domain_padding: None,
            range_padding: None,
            size: None,
            title: None,
            axes: None,
            precomputed: None,
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

    pub fn set_title<'b : 'a>(&mut self, title: &'b str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut Self {
        self.axes = Some(do_axes);
        self
    }

    pub fn enable_precomputation(&mut self) -> &mut Self {
        self.precomputed = Some(vec![]);
        self
    }

    pub fn precompute(&mut self, resolution: u32) {
        if self.precomputed.is_some() {
            assert!(self.domain.is_some());

            self.precomputed = Some(
                subdivide(self.domain.unwrap().0, self.domain.unwrap().1, resolution)
                    .into_iter()
                    .map(|x| (x, (self.func)(x)))
                    .collect::<Vec<(f64, f64)>>()
            );
        }
    }

    fn determine_range(&self, resolution: u32, domain: (f64, f64)) -> (f64, f64) {
        let y_vals: Vec<f64>;
        
        match &self.precomputed {
            Some(vals) => {
                y_vals = vals
                    .iter()
                    .map(|p| p.1)
                    .collect();
            }
            None => {
                y_vals = subdivide(domain.0, domain.1, resolution)
                    .into_iter()
                    .map(|i| (self.func)(i))
                    .collect();
            }
        }

        (min_always(&y_vals,0.), max_always(&y_vals,0.))
    }
    
    // It is reccomended to precompute for expensive functions before building
    fn build(&self) -> FuncPlot {
        let size = self.size.unwrap_or((60, 10));
        let resolution = size.0;

        let domain = self.domain.unwrap_or_else(|| determine_plot_domain(&*self.func));
        let range = self.range.unwrap_or_else(|| self.determine_range(resolution, domain));

        // With padding
        let domain = pad_range(domain, self.domain_padding.unwrap_or(0.1));
        let range = pad_range(range, self.range_padding.unwrap_or(0.1));
        
        FuncPlot {
            func: self.func.clone(),
            domain_and_range: (domain, range),
            size: size,
            title: self.title,
            axes: self.axes.unwrap_or(true),
            precomputed: &self.precomputed,
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

impl<'a> FuncPlot<'a> {
    // Possible better plot version. TODO: update or remove
    fn plot(&self) -> String {
        // charachters per unit
        let cpux = self.size.0 as f64 / (self.domain_and_range.0.1 - self.domain_and_range.0.0);
        let cpuy = self.size.1 as f64 / (self.domain_and_range.1.1 - self.domain_and_range.1.0);
        let ctux = |c: i32| self.domain_and_range.0.0 + (c as f64 + 0.5) / cpux;
        let utcy = |u: f64| ((self.domain_and_range.1.1 - u) * cpuy - 0.5) as i32;

        // xc_vals includes one extra padding value on each side for derivative checks
        let xc_vals: Vec<i32> = (-1..(1 + self.size.0 as i32)).collect();
        let xu_vals: Vec<f64> = xc_vals.iter().map(|xc| ctux(*xc)).collect();
        let yu_vals: Vec<f64> = xu_vals.iter().map(|xu| (self.func)(*xu)).collect();
        let yc_vals: Vec<i32> = yu_vals.iter().map(|yu| utcy(*yu)).collect();

        let mut o = (0..self.size.1).map(|_| (0..self.size.0).map(|_| ' ').collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let mut set_o_char = |x: i32, y: i32, c: char| if 0 <= x && x < self.size.0 as i32 && 0 <= y && y < self.size.1 as i32 {o[y as usize][x as usize] = c};

        for i in 0..self.size.0 as i32 {
            let xc = xc_vals[(i + 1) as usize];
            let (ycl, yc, ycr) = (yc_vals[i as usize], yc_vals[(i + 1) as usize], yc_vals[(i + 2) as usize]);

            let rycl = yc - ycl;
            let rycr = yc - ycr;

            // Vertical Lines
            let lowest_surrounding = std::cmp::min(rycl, rycr);
            if lowest_surrounding < -1 {
                for char_height_diff in (lowest_surrounding + 1)..0 {
                    set_o_char(xc, yc - char_height_diff, VERTICAL);
                }
            }

            // Match for Continuous lines
            let chr =
            match (rycl.clamp(-1, 1), rycr.clamp(-1, 1)) {
                (-1, -1) => FLAT_LOW,
                (0, 0) => FLAT_MED,
                (1, 1) => FLAT_HIGH,

                (-1, 1) => UP_TWO,
                (1, -1) => DOWN_TWO,

                (-1, 0) => FLAT_LOW,
                (0, 1) => FLAT_HIGH,
                (0, -1) => FLAT_LOW,
                (1, 0) => FLAT_HIGH,

                (_, _) => NULL_CHR,
            };

            set_o_char(xc, yc, chr);
            
        }

        o.into_iter().map(|l| l.into_iter().collect::<String>()).collect::<Vec<String>>().join("\n")
    }

    fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), self.domain_and_range, self.axes, self.title)
    }

    fn print(&self) {
        println!("{}", self.as_string());
    }

    fn pyplot(&self, path: Option<&str>) {
        let x_vals: Vec<f64>;
        let y_vals: Vec<f64>;

        match self.precomputed {
            Some(vals) => {
                x_vals = vals.iter().map(|p| p.0).collect();
                y_vals = vals.iter().map(|p| p.1).collect();
            }
            None => {
                x_vals = subdivide(self.domain_and_range.0.0, self.domain_and_range.0.1, 10 * self.size.0);
                y_vals = x_vals.iter().map(|x| (self.func)(*x)).collect();
            }
        }

        let command = format!("plot({x_vals:?}, {y_vals:?})");
        pyplot(&command, self.title, Some(self.axes), Some(self.domain_and_range), path);
    }
}

pub fn function_plot<'a>(func: &'a impl Fn(f64) -> f64) -> FuncPlotBuilder<'a> {
    FuncPlotBuilder::from(func)
}

pub fn as_float_function<'a, U, V>(func: impl Fn(U) -> V) -> impl Fn(f64) -> f64
where
    U: FromPrimitive,
    V: ToPrimitive,
{
    move |x: f64| func(U::from_f64(x).unwrap()).to_f64().unwrap()
}