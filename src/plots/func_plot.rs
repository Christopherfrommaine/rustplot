use num::{FromPrimitive, ToPrimitive};

// TODO: axes and title are not yet implemented
use crate::helper::math::{der, pad_range, subdivide, min_always, max_always};
use crate::helper::func_plot_range_finder::determine_plot_range;



/// Builds elements of a function plot.
/// 
/// This struct allows the user to set various values of the plot, such as
/// title, axes, custom charachter sets, etc.
/// 
/// Internally then uses .build() to convert it's values from Option<T> to T,
/// and finally plots with .as_string() or .print() from those values.
pub struct FuncPlotBuilder {
    func: Box<dyn Fn(f64) -> f64>,
    domain: Option<(f64, f64)>,
    range: Option<(f64, f64)>,
    padding: Option<f64>,
    resolution: Option<u32>,
    size: Option<(u32, u32)>,
    title: Option<String>,
    axes: Option<bool>,
    precomputed: Option<Vec<(f64, f64)>>,
}

/// Internal struct representing built values.
pub(crate) struct FuncPlot<'a> {
    func: &'a Box<dyn Fn(f64) -> f64>,
    full_range: ((f64, f64), (f64, f64)),
    resolution: u32,
    size: (u32, u32),
    title: Option<String>,
    axes: bool,
    precomputed: Vec<(f64, f64)>,
}

impl FuncPlotBuilder {
    /// Create an array plot from a table of data.
    fn from(func: Box<dyn Fn(f64) -> f64>) -> FuncPlotBuilder {
        FuncPlotBuilder {
            func: func,
            domain: None,
            range: None,
            padding: None,
            resolution: None,
            size: None,
            title: None,
            axes: None,
            precomputed: None,
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

    pub fn set_padding(&mut self, padding: f64) -> &mut FuncPlotBuilder {
        self.padding = Some(padding);
        self
    }

    pub fn set_resolution(&mut self, resolution: u32) -> &mut FuncPlotBuilder {
        self.resolution = Some(resolution);
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

    fn set_precomputed(&mut self, domain: (f64, f64), resolution: u32) -> &mut FuncPlotBuilder {
        let x_vals: Vec<f64> = subdivide(domain.0, domain.1, resolution);
        self.precomputed = Some(
            x_vals.into_iter().map(|i| (i, (self.func)(i))).collect()
        );
        self
    }

    fn get_default_range(&mut self, precomputed: &Vec<(f64, f64)>) -> (f64, f64) {
        (
            min_always(
                &precomputed
                .iter()
                .map(|i| i.1)
                .collect::<Vec<f64>>(),0.),
            max_always(
                &precomputed
                .iter()
                .map(|i| i.1)
                .collect::<Vec<f64>>(),0.),
        )
    }

    fn build(&mut self) -> FuncPlot {
        // Padding must go before range, as default arg for range is based on padding
        self.set_size(
            match self.size {
                Some(o) => o,
                None => (30, 50),
            }
        );
        self.set_resolution(
            match self.resolution {
                Some(o) => o,
                None => self.size.unwrap().0,
            }
        );
        self.set_padding(
            match self.padding {
                Some(o) => o,
                None => 0.,
            }
        );
        self.set_domain(
            match self.domain {
                Some(o) => o,
                None => determine_domain(|x| (self.func)(x)),
            }
        );
        self.set_precomputed(
            self.domain.unwrap(),
            self.resolution.unwrap(),
        );
        let range_to_be_set = match &self.range {
            Some(o) => *o,
            None => (self.get_default_range(&self.precomputed.clone().unwrap())).clone(),
        };
        self.set_range(
            pad_range(range_to_be_set, self.padding.unwrap())
        );
        self.set_axes(
            match self.axes {
                Some(o) => o,
                None => true,
            }
        );
        
        FuncPlot {
            func: &(self.func),
            full_range: (self.domain.unwrap(), self.range.unwrap()),
            resolution: self.resolution.unwrap(),
            size: self.size.unwrap(),
            title: self.title.clone(),
            axes: self.axes.unwrap(),
            precomputed: self.precomputed.clone().unwrap(),
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

impl<'a> FuncPlot<'a> {
    fn plot(&self) -> String {
        String::from("")
    }

    fn as_string(&self) -> String {
        match &self.title {
            Some(val) => {
                let mut o = val.clone();
                o.push_str(&self.plot());
                return o
            }
            None => {
                return self.plot()
            }
        }
    }

    fn print(&self) {
        println!("{}", self.as_string());
    }
}

fn float_function_plot< F>(func: F) -> FuncPlotBuilder
where
    F: Fn(f64) -> f64 + 'static,
{
    FuncPlotBuilder::from(Box::new(func))
}

pub fn function_plot<U, V, G>(func: G) -> FuncPlotBuilder
where
    U: FromPrimitive,
    V: ToPrimitive,
    G: Fn(U) -> V + 'static,
{
    float_function_plot(move |x| func(U::from_f64(x).unwrap()).to_f64().unwrap())
}