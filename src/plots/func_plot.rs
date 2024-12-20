use num::{FromPrimitive, ToPrimitive};

// TODO: axes and title are not yet implemented
use crate::helper::{
    math::{pad_range, subdivide, min_always, max_always, der, der_p},
    func_plot_domain::determine_plot_domain,
    axes::add_opt_axes_and_opt_titles,
};


/// Builds elements of a function plot.
/// 
/// This struct allows the user to set various values of the plot, such as
/// title, axes, custom charachter sets, etc.
/// 
/// Internally then uses .build() to convert it's values from Option<T> to T,
/// and finally plots with .as_string() or .print() from those values.
pub struct FuncPlotBuilder<'a> {
    func: Box<dyn Fn(f64) -> f64 + 'a>,
    domain: Option<(f64, f64)>,
    range: Option<(f64, f64)>,
    domain_padding: Option<f64>,
    range_padding: Option<f64>,
    size: Option<(u32, u32)>,
    title: Option<String>,
    axes: Option<bool>,
    precomputed: Option<Vec<(f64, f64)>>,
}

/// Internal struct representing built values.
pub(crate) struct FuncPlot<'a> {
    func: &'a Box<dyn Fn(f64) -> f64 + 'a>,
    domain_and_range: ((f64, f64), (f64, f64)),
    size: (u32, u32),
    title: Option<String>,
    axes: bool,
    precomputed: Vec<(f64, f64)>,
}

impl<'a> FuncPlotBuilder<'a> {
    /// Create an array plot from a table of data.
    fn from<F>(func: F) -> FuncPlotBuilder<'a>
    where
        F: Fn(f64) -> f64 + 'a
    {
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

    pub fn set_domain(&mut self, domain: (f64, f64)) -> &mut FuncPlotBuilder<'a> {
        self.domain = Some(domain);
        self
    }

    pub fn set_range(&mut self, range: (f64, f64)) -> &mut FuncPlotBuilder<'a> {
        self.range = Some(range);
        self
    }

    pub fn set_domain_padding(&mut self, padding: f64) -> &mut FuncPlotBuilder<'a> {
        self.domain_padding = Some(padding);
        self
    }
    
    pub fn set_range_padding(&mut self, padding: f64) -> &mut FuncPlotBuilder<'a> {
        self.range_padding = Some(padding);
        self
    }

    pub fn set_size(&mut self, size: (u32, u32)) -> &mut FuncPlotBuilder<'a> {
        self.size = Some(size);
        self
    }

    pub fn set_title(&mut self, title: String) -> &mut FuncPlotBuilder<'a> {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut FuncPlotBuilder<'a> {
        self.axes = Some(do_axes);
        self
    }

    fn precompute(&mut self, domain: (f64, f64), resolution: u32) -> &mut FuncPlotBuilder<'a> {
        let x_vals = subdivide(domain.0, domain.1, resolution);
        self.precomputed = Some(
            x_vals.into_iter().map(|i| (i, (self.func)(i))).collect()
        );
        self
    }

    fn function_range_precomputed(&self) -> (f64, f64) {
        assert!(self.precomputed.is_some());

        let pc: &Vec<(f64, f64)> = self.precomputed.as_ref().unwrap().as_ref();
        (
            min_always(
                &pc
                .iter()
                .map(|i| i.1)
                .collect::<Vec<f64>>(),0.),
            max_always(
                &pc
                .iter()
                .map(|i| i.1)
                .collect::<Vec<f64>>(),0.),
        )
    }

    fn build(&'a mut self) -> FuncPlot<'a> {
        // Padding must go before range, as default arg for range is based on padding
        self.set_size(
            match self.size {
                Some(o) => o,
                None => (30, 50),
            }
        );
        
        self.set_domain_padding(
            match self.domain_padding {
                Some(o) => o,
                None => 0.1,
            }
        );
        self.set_range_padding(
            match self.range_padding {
                Some(o) => o,
                None => 0.1,
            }
        );

        self.set_domain(
            pad_range(
                match self.domain {
                    Some(o) => o,
                    None => determine_plot_domain(&self.func),
                },
                self.domain_padding.unwrap(),
            )
        );

        self.precompute(self.domain.unwrap(), self.size.unwrap().0);

        let rge = self.range.clone();
        self.set_domain(
            pad_range(
                match rge {
                    Some(o) => o,
                    None => (&self).function_range_precomputed(),
                },
                self.range_padding.unwrap(),
            )
        );

        self.set_axes(
            match self.axes {
                Some(o) => o,
                None => true,
            }
        );
        
        FuncPlot {
            func: &self.func,
            domain_and_range: (self.domain.unwrap(), self.range.unwrap()),
            size: self.size.unwrap(),
            title: self.title.clone(),
            axes: self.axes.unwrap(),
            precomputed: self.precomputed.clone().unwrap(),
        }
    }

    /// Returns the plotted data as a string
    pub fn as_string(&'a mut self) -> String {
        self.build().as_string()
    }

    /// Displays the plotted data with println
    pub fn print(&'a mut self) {
        self.build().print();
    }
}

impl<'a> FuncPlot<'a> {
    fn plot(&self) -> String {
        String::from("")
    }

    fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), self.domain_and_range, self.axes, &self.title)
    }

    fn print(&self) {
        println!("{}", self.as_string());
    }
}

pub fn function_plot<'a, U, V, G>(func: G) -> FuncPlotBuilder<'a>
where
    U: FromPrimitive,
    V: ToPrimitive,
    G: Fn(U) -> V + 'a,
{
    FuncPlotBuilder::from(
        move |x|
        func(
            U::from_f64(x).unwrap()
        )
        .to_f64().unwrap()
    )
}