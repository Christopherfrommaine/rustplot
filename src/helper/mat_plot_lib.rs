use std::process::Command;
use log::warn;

/// Creates and runs a python file to use matplotlib to display a plot.
/// 
/// Plot-specific implementations are avalible in each builder type.
/// 
/// # Arguments
/// 
/// * `plt_command` - Will be run as "plt.{plt_command}". Includes all the data needed for a plot.
/// * `title` - Optional title for the plot.
/// * `axes` - Optional bool for whether or not to show axes.
/// * `rge` - Range (if applicable) over which the plot should be displayed.
/// * `path` - If path is Some, then instead of being displayed, the pyplot will be saved to an image file at path.
/// 
/// # Example
/// ```
/// use cgrustplot::helper::{mat_plot_lib::pyplot, file::get_current_dir};
/// 
/// pyplot("scatter([0, 1, 2], [3, 4, 5])", Some("This is a title"), None, Some(((-0.1, 2.1), (-0.1, 5.1))), Some(&(get_current_dir() + "testoutput/doctest_pyplot.png")));
/// 
/// // Pyplot image file is now saved at "./testoutput/doctest_pyplot.png"
/// ```
pub fn pyplot(plt_command: &str, title: Option<&str>, axes: Option<bool>, rge: Option<((f64, f64), (f64, f64))>, path: Option<&str>) {
    let title_str = match title {Some(s) => &vec!["\"",  s, "\""].join(""), None => "None" };
    let axes_str = match axes {Some(b) => if b {"True"} else {"False"}, None => "None"};
    let rge_str = match rge {Some(r) => &vec!["((", &r.0.0.to_string(), ", ", &r.0.1.to_string(), "), (", &r.1.0.to_string(), ", ", &r.1.1.to_string(), "))"].join(""), None => "None"};
    let path_str = match path {Some(p) => &vec!["\"", p, "\""].join(""), None => "None"};

    let script = format!(
"from matplotlib import pyplot as plt

# Data replaced in file
title = {title}
axes = {axes}
rge = {rge}
path = {path}

# Plot, as determined by plot type in the rust file
plt.{plot_command}

# Automatic
if title is not None:
    plt.suptitle(title)

if axes is not None:
    if axes:
        pass
    else:
        plt.cla()

if rge is not None:
    plt.xlim(rge[0])
    plt.ylim(rge[1])

if path is not None:
    plt.savefig(path)
else:
    plt.show()
",  title=title_str, axes=axes_str, rge=rge_str, path=path_str, plot_command=plt_command);

    if let Err(e) = Command::new("python3")
        .arg("-c")
        .arg(script)
        .output() {
            warn!("Failed to run matplotlib script: {e}");
        }
    
}
