use num::{FromPrimitive, ToPrimitive};
use std::sync::Arc;

use crate::helper::{
    axes::add_opt_axes_and_opt_titles,
    charset::{self, line_chars, NULL_CHR},
    func_plot_domain::determine_plot_domain,
    math::{der_p, max_always, min_always, pad_range, subdivide},
    mat_plot_lib::python_plot,
};


/// Builds elements of a function plot.
/// 
/// This struct allows the user to set various values of the plot, such as
/// title, axes, custom charachter sets, etc.
/// 
/// Internally then uses .build() to convert it's values from Option<T> to T,
/// and finally plots with .as_string() or .print() from those values.
pub struct FuncPlotBuilder {
    func: Arc<dyn Fn(f64) -> f64 + 'static>,
    domain: Option<(f64, f64)>,
    range: Option<(f64, f64)>,
    domain_padding: Option<f64>,
    range_padding: Option<f64>,
    size: Option<(u32, u32)>,
    title: Option<String>,
    axes: Option<bool>,
}

/// Internal struct representing built values.
pub(crate) struct FuncPlot {
    pub(crate) func: Arc<dyn Fn(f64) -> f64 + 'static>,
    pub(crate) domain_and_range: ((f64, f64), (f64, f64)),
    pub(crate) size: (u32, u32),
    pub(crate) title: Option<String>,
    pub(crate) axes: bool,
}

impl FuncPlotBuilder {
    /// Create an array plot from a table of data.
    fn from<F>(func: F) -> FuncPlotBuilder
    where
        F: Fn(f64) -> f64 + 'static
    {
        FuncPlotBuilder {
            func: Arc::new(func),
            domain: None,
            range: None,
            domain_padding: None,
            range_padding: None,
            size: None,
            title: None,
            axes: None,
        }
    }

    pub fn set_domain(&mut self, domain: (f64, f64)) -> &mut FuncPlotBuilder {
        self.domain = Some(domain);
        self
    }

    pub fn set_range(&mut self, range: (f64, f64)) -> &mut FuncPlotBuilder {
        self.range = Some(range);
        self
    }

    pub fn set_domain_padding(&mut self, padding: f64) -> &mut FuncPlotBuilder {
        self.domain_padding = Some(padding);
        self
    }
    
    pub fn set_range_padding(&mut self, padding: f64) -> &mut FuncPlotBuilder {
        self.range_padding = Some(padding);
        self
    }

    pub fn set_size(&mut self, size: (u32, u32)) -> &mut FuncPlotBuilder {
        self.size = Some(size);
        self
    }

    pub fn set_title(&mut self, title: String) -> &mut FuncPlotBuilder {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut FuncPlotBuilder {
        self.axes = Some(do_axes);
        self
    }

    fn function_range_precomputed(&self, domain: (f64, f64), resolution: u32) -> (f64, f64) {
        let x_vals = subdivide(domain.0, domain.1, resolution);
        let y_vals = x_vals.into_iter().map(|i| (self.func)(i)).collect();

        (min_always(&y_vals,0.), max_always(&y_vals,0.))
    }

    fn build(&mut self) -> FuncPlot {
        // Padding must go before range, as default arg for range is based on padding
        self.set_size(
            self.size.unwrap_or((60, 10))
        );
        
        self.set_domain_padding(
            self.domain_padding.unwrap_or(0.1)
        );
        self.set_range_padding(
            self.range_padding.unwrap_or(0.1)
        );

        self.set_domain(
            pad_range(
                self.domain.unwrap_or(
                    determine_plot_domain(&*self.func)
                ),
                self.domain_padding.unwrap(),
            )
        );

        let rge = self.range.clone();
        self.set_range(
            pad_range(
                rge.unwrap_or((&self).function_range_precomputed(self.domain.unwrap(), self.size.unwrap().0)),
                self.range_padding.unwrap(),
            )
        );
        
        FuncPlot {
            func: Arc::clone(&self.func),
            domain_and_range: (self.domain.unwrap(), self.range.unwrap()),
            size: self.size.unwrap(),
            title: self.title.clone(),
            axes: self.axes.unwrap_or(true),
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
}

impl FuncPlot {
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

    pub(crate) fn plot(&self) -> String {
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

    pub fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), self.domain_and_range, self.axes, &self.title)
    }

    pub fn print(&self) {
        println!("{}", self.as_string());
    }
    
}

pub fn function_plot<U, V, G>(func: G) -> FuncPlotBuilder
where
    U: FromPrimitive,
    V: ToPrimitive,
    G: Fn(U) -> V + 'static,
{
    FuncPlotBuilder::from(
        move |x|
        func(
            U::from_f64(x).unwrap()
        )
        .to_f64().unwrap()
    )
}