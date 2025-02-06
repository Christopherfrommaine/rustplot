use num::{FromPrimitive, ToPrimitive};

use crate::helper::{
    axes::add_opt_axes_and_opt_titles,
    charset::{self, line_chars, NULL_CHR},
    func_plot_domain::determine_plot_domain,
    math::{der_p, max_always, min_always, pad_range, subdivide},
    mat_plot_lib::pyplot,
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

    fn determine_range(&self, resolution: u32) -> (f64, f64) {
        let y_vals: Vec<f64>;
        
        match &self.precomputed {
            Some(vals) => {
                y_vals = vals
                    .iter()
                    .map(|p| p.1)
                    .collect();
            }
            None => {
                assert!(self.domain.is_some());
                y_vals = subdivide(self.domain.unwrap().0, self.domain.unwrap().1, resolution)
                    .into_iter()
                    .map(|i| (self.func)(i))
                    .collect();
            }
        }

        (min_always(&y_vals,0.), max_always(&y_vals,0.))
    }

    fn precompute(&mut self, resolution: u32) {
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

    fn build(&mut self) -> FuncPlot {
        if self.size.is_none() {
            self.set_size((60, 10));
        }

        let resolution = self.size.unwrap().0;

        if self.domain_padding.is_none() {
            self.set_domain_padding(0.1);
        }
        if self.range_padding.is_none() {
            self.set_range_padding(0.1);
        }
        
        if self.domain.is_none() {
            self.set_domain(determine_plot_domain(&*self.func));
        }
        if self.range.is_none() {
            self.set_range(self.determine_range(resolution));
        }

        self.set_domain(pad_range(self.domain.unwrap(), self.domain_padding.unwrap()));

        self.precompute(resolution);

        self.set_range(pad_range(self.range.unwrap(), self.range_padding.unwrap()));
        
        FuncPlot {
            func: self.func.clone(),
            domain_and_range: (self.domain.unwrap(), self.range.unwrap()),
            size: self.size.unwrap(),
            title: self.title,
            axes: self.axes.unwrap_or(true),
            precomputed: &self.precomputed,
        }
    }

    /// Returns the plotted data as a string
    pub fn as_string(&mut self) -> String {
        self.build().as_string()
    }

    /// Displays the plotted data with println
    pub fn print(&mut self) {
        self.build().print();
    }

    pub fn plot(&mut self) -> String {
        self.build().plot()
    }

    pub fn pyplot(&mut self) {
        self.build().pyplot(None);
    }

    pub fn save_pyplot(&mut self, path: &str) {
        self.build().pyplot(Some(path));
    }
}

impl<'a> FuncPlot<'a> {
    fn compute_char_at_coords(&self, x_c: usize, prev: (i32, char)) -> (i32, char) {
        // Variables are annotated with units. Either u (the units of the function) or c (units of charachters in the plot)

        let c_per_u_x = self.size.0 as f64 / (self.domain_and_range.0.1 - self.domain_and_range.0.0);
        let c_per_u_y = (self.size.1) as f64 / (self.domain_and_range.1.1 - self.domain_and_range.1.0);

        let x_u = x_c as f64 / c_per_u_x + self.domain_and_range.0.0;
        let y_u = (&self.func)(x_u);
        
        if y_u.is_infinite() {return (0, ' ');}
        if y_u.is_nan() {return (0, ' ');}

        let y_c_f = (y_u - self.domain_and_range.1.0) * c_per_u_y;
        let y_c= (self.size.1 as i32) - (y_c_f as i32) - 1i32;
        let derx_u = der_p(&*self.func, x_u);
        let derx_c = derx_u * c_per_u_y / c_per_u_x;

        let output_char: char;

        if derx_c.is_nan() {
            output_char = charset::NULL_CHR;
        } else if derx_c.is_infinite() {
            output_char = line_chars::VERTICAL;
        } else if prev.0 == y_c {
            if y_c_f - y_c_f.floor() < 1. / 3. {
                output_char = line_chars::FLAT_LOW;
            } else if y_c_f - y_c_f.floor() > 2. / 3. {
                output_char = line_chars::FLAT_HIGH;
            } else {
                output_char = line_chars::FLAT_MED;
            }
        } else if derx_c > 2. {
            output_char = line_chars::UP_TWO;
        } else if derx_c < -2. {
            output_char = line_chars::DOWN_TWO;
        } else if derx_c > 0.5 {
            output_char = line_chars::UP_ONE;
        } else if derx_c < -0.5 {
            output_char = line_chars::DOWN_ONE;
        } else {
            if y_c_f - y_c_f.floor() < 1. / 3. {
                output_char = line_chars::FLAT_LOW;
            } else if y_c_f - y_c_f.floor() > 2. / 3. {
                output_char = line_chars::FLAT_HIGH;
            } else {
                output_char = line_chars::FLAT_MED;
            }
        }

        (y_c, output_char)
    }

    fn plot(&self) -> String {
        let mut o: Vec<Vec<char>> = (0..self.size.1).map(|_i| (0..self.size.0).map(|_j| ' ').collect()).collect();

        let mut prev = (0i32, NULL_CHR);
        for x_c in 0..self.size.0 as usize {
            prev = self.compute_char_at_coords(x_c, prev);
            let (y_c, chr) = prev;
            
            if y_c >= 0 && (y_c as usize) < o.len() {
                o[y_c as usize][x_c] = chr;
            }
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

        let command = format!("plt.plot({x_vals:?}, {y_vals:?})");
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