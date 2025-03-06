use crate::helper::{
    axes::add_opt_axes_and_opt_titles,
    charset::subdiv_chars::blocks_two_by_two,
    math::{bin_to_u8, pad_range, subdivide},
    mat_plot_lib::pyplot,
    file::save_to_file,
    rendering::RenderableTextBuilder,
};

pub struct RegionPlotBuilder<'a> {
    pred: Box<&'a dyn Fn(f64, f64) -> bool>,
    domain_and_range: Option<((f64, f64), (f64, f64))>,
    padding: Option<f64>,
    size: Option<(u32, u32)>,
    title: Option<&'a str>,
    axes: Option<bool>,
}

/// Internal struct representing built values.
struct RegionPlot<'a> {
    pred: Box<&'a dyn Fn(f64, f64) -> bool>,
    domain_and_range: ((f64, f64), (f64, f64)),
    size: (u32, u32),
    title: Option<&'a str>,
    axes: bool,
}

impl<'a> RegionPlotBuilder<'a> {
    /// Create an array plot from a table of data.
    fn from(pred: &'a impl Fn(f64, f64) -> bool) -> Self {
        RegionPlotBuilder {
            pred: Box::new(pred),
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

    pub fn set_title<'b: 'a>(&mut self, title: &'b str) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn set_axes(&mut self, do_axes: bool) -> &mut Self {
        self.axes = Some(do_axes);
        self
    }

    fn build(&self) -> RegionPlot<'a> {
        // Padding must go before range, as default arg for range is based on padding
        let size = self.size.unwrap_or((60, 30));
        let padding = self.padding.unwrap_or(0.1);
        let domain_and_range = self.domain_and_range.unwrap_or_else(|| ((0., 0.,), (size.0 as f64, size.1 as f64)));

        // With Padding
        let domain_and_range = (pad_range(domain_and_range.0, padding), pad_range(domain_and_range.1, padding));
        
        RegionPlot {
            pred: self.pred.clone(),
            domain_and_range: domain_and_range,
            size: size,
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

impl<'a> RegionPlot<'a> {
   fn plot(&self) -> String {
        let y_values = subdivide(self.domain_and_range.1.0, self.domain_and_range.1.1, self.size.1);
        let x_values = subdivide(self.domain_and_range.0.0, self.domain_and_range.0.1, self.size.0);

        let y_diff = 0.5 * (self.domain_and_range.1.1 - self.domain_and_range.1.0) / (self.size.1 - 1) as f64;
        let x_diff = 0.5 * (self.domain_and_range.0.1 - self.domain_and_range.0.0) / (self.size.0 - 1) as f64;
        
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
        add_opt_axes_and_opt_titles(&self.plot(), self.domain_and_range, self.axes, self.title)
    }

    pub fn print(&self) {
        println!("{}", self.as_string());
    }

    pub fn pyplot(&self, path: Option<&str>) {
        let y_values = subdivide(self.domain_and_range.1.0, self.domain_and_range.1.1, 2 * self.size.1);
        let x_values = subdivide(self.domain_and_range.0.0, self.domain_and_range.0.1, 2 * self.size.0);
        
        let tab: Vec<Vec<u8>> = y_values
        .into_iter()
        .map(|y|
            x_values.clone()
            .into_iter()
            .map(|x|
                (self.pred)(x, y) as u8
            ).collect()
        )
        .rev()
        .collect();
        
        let command = format!("imshow({:?})", tab);

        pyplot(&command, self.title, Some(self.axes), None, path);
    }
}


pub fn region_plot<'a>(pred: &'a impl Fn(f64, f64) -> bool) -> RegionPlotBuilder<'a> {
    RegionPlotBuilder::from(pred)
}