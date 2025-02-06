use crate::helper::{
    math::{pad_range, max_always, min_always},
    axes_original::add_opt_axes_and_opt_titles,
};
use crate::plots::func_plot::function_plot;

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

    fn build(&mut self) -> LinePlot {
        // Padding must go before range, as default arg for range is based on padding
        if self.domain_padding.is_none() {
            self.set_domain_padding(0.1);
        }

        if self.range_padding.is_none() {
            self.set_range_padding(0.1);
        }

        self.set_domain(
            pad_range(
                self.domain.unwrap_or((
                    min_always(&self.data.iter().map(|p| p.0).collect(), 0.),
                    max_always(&self.data.iter().map(|p| p.0).collect(), 0.),
                )),
                self.domain_padding.unwrap(),
            )
        );

        self.set_range(
            pad_range(
                self.range.unwrap_or((
                    min_always(&self.data.iter().map(|p| p.1).collect(), 0.),
                    max_always(&self.data.iter().map(|p| p.1).collect(), 0.),
                )),
                self.range_padding.unwrap(),
            )
        );
        
        LinePlot {
            data: self.data,
            domain_and_range: (self.domain.unwrap(), self.range.unwrap()),
            size: self.size.unwrap_or((60, 30)),
            title: self.title,
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

    pub fn plot(&mut self) {
        self.build().plot();
    }
}

impl<'a> LinePlot<'a> {
    pub fn plot(&self) -> String {
        let mut d = self.data.clone();
        d.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Greater));

        let f = move |x: f64| {
            if d[0].0 > x || x > d[d.len() - 1].0 {return f64::NAN;}

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

        plot
            .plot()
    }

    pub fn as_string(&self) -> String {
        add_opt_axes_and_opt_titles(&self.plot(), self.domain_and_range, self.axes, self.title)
    }

    pub fn print(&self) {
        println!("{}", self.as_string());
    }
}

pub fn line_plot<'a>(data: &'a Vec<(f64, f64)>) -> LinePlotBuilder<'a> {
    LinePlotBuilder::from(data)
}