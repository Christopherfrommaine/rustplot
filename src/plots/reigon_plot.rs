use std::sync::Arc;
use crate::helper::{
    math::{pad_range, subdivide, bin_to_u8},
    axes::add_opt_axes_and_opt_titles,
    charset::subdiv_chars::blocks_two_by_two,
};

pub struct ReigonPlotBuilder {
    pred: Arc<dyn Fn(f64, f64) -> bool + 'static>,
    domain_and_range: Option<((f64, f64), (f64, f64))>,
    padding: Option<f64>,
    size: Option<(u32, u32)>,
    title: Option<String>,
    axes: Option<bool>,
}

/// Internal struct representing built values.
pub(crate) struct ReigonPlot {
    pub(crate) pred: Arc<dyn Fn(f64, f64) -> bool + 'static>,
    pub(crate) domain_and_range: ((f64, f64), (f64, f64)),
    pub(crate) size: (u32, u32),
    pub(crate) title: Option<String>,
    pub(crate) axes: bool,
}

impl ReigonPlotBuilder {
    /// Create an array plot from a table of data.
    fn from(pred: impl Fn(f64, f64) -> bool + 'static) -> ReigonPlotBuilder {
        ReigonPlotBuilder {
            pred: Arc::new(pred),
            domain_and_range: None,
            padding: None,
            size: None,
            title: None,
            axes: None,
        }
    }

    pub fn set_domain_and_range(&mut self, domain_and_range: ((f64, f64), (f64, f64))) -> &mut Self {
        self.domain_and_range = Some(domain_and_range);
        self
    }

    pub fn set_padding(&mut self, padding: f64) -> &mut Self {
        self.padding = Some(padding);
        self
    }

    pub fn set_size(&mut self, size: (u32, u32)) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn set_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut Self {
        self.axes = Some(do_axes);
        self
    }

    fn build(&mut self) -> ReigonPlot {
        // Padding must go before range, as default arg for range is based on padding
        self.set_size(
            self.size.unwrap_or((60, 30))
        );
        
        self.set_padding(
            self.padding.unwrap_or(0.1)
        );

        self.set_domain_and_range(
            self.domain_and_range.unwrap_or(
                ((0., 0.,), (self.size.unwrap().0 as f64, self.size.unwrap().1 as f64))
            )
        );

        self.set_domain_and_range((
            pad_range(self.domain_and_range.unwrap().0, self.padding.unwrap()),
            pad_range(self.domain_and_range.unwrap().1, self.padding.unwrap()),
        ));
        
        ReigonPlot {
            pred: Arc::clone(&self.pred),
            domain_and_range: self.domain_and_range.unwrap(),
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

impl ReigonPlot {
    pub(crate) fn plot(&self) -> String {
        let y_values = subdivide(self.domain_and_range.1.0, self.domain_and_range.1.1, self.size.1);
        let x_values = subdivide(self.domain_and_range.0.0, self.domain_and_range.0.1, self.size.0);

        let y_diff = (self.domain_and_range.1.1 - self.domain_and_range.1.0) / (self.size.1 - 1) as f64;
        let x_diff = (self.domain_and_range.0.1 - self.domain_and_range.0.0) / (self.size.0 - 1) as f64;
        
        y_values
        .into_iter()
        .map(|y|
            x_values.clone()
            .into_iter()
            .map(|x|
                blocks_two_by_two()[
                    bin_to_u8(
                        vec![(self.pred)(x, y + y_diff), (self.pred)(x, y), (self.pred)(x + x_diff, y + y_diff), (self.pred)(x + x_diff, y)]
                    ) as usize
                ]
            ).collect()
        )
        .rev()
        .collect::<Vec<String>>()
        .join("\n")
    }

    pub fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), self.domain_and_range, self.axes, &self.title)
    }

    pub fn print(&self) {
        println!("{}", self.as_string());
    }
}


pub fn reigon_plot(pred: impl Fn(f64, f64) -> bool + 'static) -> ReigonPlotBuilder {
    ReigonPlotBuilder::from(pred)
}